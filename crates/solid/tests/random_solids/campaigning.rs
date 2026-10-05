//! What a long campaign over random cases is made of, whatever it draws and
//! whichever kernel it holds: the clock and the seeds it runs over, the lines
//! it counted, its report, and the seeds it named shrunk one by one.
//!
//! Compiled only when asked for — `--features campaigns` — as the campaigns
//! themselves are.

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant, SystemTime};

use cao_solid::soundness::{Check, Flaw, Random, Report, answer, campaign, shrink};

use super::Case;

/// How many lines the campaign held its cases along, and how many it left
/// out for grazing a curved wall.
pub static HELD: AtomicUsize = AtomicUsize::new(0);
pub static GRAZING: AtomicUsize = AtomicUsize::new(0);

pub fn from_the_environment(name: &str) -> Option<u64> {
    std::env::var(name).ok()?.parse().ok()
}

/// A campaign on the exact kernel over the cases `draw` gives, for as long as
/// the environment says, and its report.
pub fn campaign_over(what: &str, draw: fn(u64) -> Case, check: fn(&Case) -> Result<(), Flaw>) {
    let seconds = from_the_environment("CAO_FUZZ_SECONDS").unwrap_or(60);
    let first = from_the_environment("CAO_FUZZ_SEED").unwrap_or_else(|| {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_or(1, |since| since.as_nanos() as u64);
        Random::seeded(now).number() >> 16
    });
    let cases = from_the_environment("CAO_FUZZ_CASES").unwrap_or(u64::MAX);
    let patience = Duration::from_secs(from_the_environment("CAO_FUZZ_PATIENCE").unwrap_or(30));
    let deadline = Instant::now() + Duration::from_secs(seconds);
    println!("campaign of {what} on the exact kernel from seed {first}, for {seconds} s");

    let check: Check<Case> = Arc::new(check);
    let quiet = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let report = campaign(
        first..first.saturating_add(cases),
        draw,
        check,
        Case::smaller,
        patience,
        || Instant::now() < deadline,
    );
    std::panic::set_hook(quiet);

    print_report(&report);
    assert!(
        report.findings.is_empty(),
        "{} cases of {} broke a rule",
        report.broken.iter().map(|(_, count)| count).sum::<usize>(),
        report.tried
    );
}

/// Each seed run again, shrunk while it still breaks the same rule, and
/// printed as a test: what a campaign's failures are sorted into distinct
/// ones from.
pub fn shrunk_one_by_one(seeds: Vec<u64>, draw: fn(u64) -> Case, check: Check<Case>) {
    let patience = Duration::from_secs(from_the_environment("CAO_FUZZ_PATIENCE").unwrap_or(30));
    let quiet = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    for seed in seeds {
        let drawn = draw(seed);
        let Err(flaw) = answer(drawn.clone(), &check, patience) else {
            println!("\n── holds, seed {seed} ──");
            continue;
        };
        let mut last = flaw.clone();
        let shrunk = shrink(
            drawn.clone(),
            Case::smaller,
            |candidate| match answer(candidate.clone(), &check, patience) {
                Err(broken) if broken.is_like(&flaw) => {
                    last = broken;
                    true
                }
                _ => false,
            },
            || true,
        );
        println!(
            "\n── {:?}, seed {seed} ──\nas drawn: {flaw:?}\nshrunk:   {last:?}\n{}",
            flaw.rule(),
            shrunk.to_string().replace('\n', "\n    "),
        );
    }
    std::panic::set_hook(quiet);
}

fn print_report(report: &Report<Case>) {
    println!("{} cases tried", report.tried);
    println!(
        "{} lines held, {} left out for grazing a curved wall",
        HELD.load(Ordering::Relaxed),
        GRAZING.load(Ordering::Relaxed)
    );
    for (rule, count) in &report.broken {
        println!("  {rule:?}: broken by {count}");
    }
    for (seed, rule) in &report.failed {
        println!("failed {seed} {rule:?}");
    }
    for finding in &report.findings {
        println!(
            "\n── {:?}, seed {} ──\nas drawn: {:?}\nshrunk:   {:?}\n\n#[test]\nfn seed_{}_keeps_every_rule_on_the_exact_kernel() {{\n    random_solids::holds_exactly(&{});\n}}",
            finding.flaw.rule(),
            finding.seed,
            finding.flaw,
            finding.shrunk_flaw,
            finding.seed,
            finding.shrunk.to_string().replace('\n', "\n    "),
        );
    }
}
