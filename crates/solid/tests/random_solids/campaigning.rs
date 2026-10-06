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

/// How many lines the campaign held its cases along, how many it left out
/// for grazing a curved wall or running through a cone's tip, how many
/// cases asking for a conic the kernel declined as unsupported, and how many
/// turned walls a hair thin it declined to raise.
pub static HELD: AtomicUsize = AtomicUsize::new(0);
pub static GRAZING: AtomicUsize = AtomicUsize::new(0);
pub static TIPS: AtomicUsize = AtomicUsize::new(0);
pub static DECLINED: AtomicUsize = AtomicUsize::new(0);
pub static THIN: AtomicUsize = AtomicUsize::new(0);

/// The kernel a campaign holds its cases on, as its report names it and as
/// the tests it prints call it.
#[derive(Clone, Copy)]
pub enum Answering {
    Exactly,
    ThroughTheApplication,
}

impl Answering {
    fn on(self) -> &'static str {
        match self {
            Answering::Exactly => "on the exact kernel",
            Answering::ThroughTheApplication => "through the application's body",
        }
    }

    fn holds(self) -> &'static str {
        match self {
            Answering::Exactly => "holds_exactly",
            Answering::ThroughTheApplication => "holds_through_the_application",
        }
    }

    fn test(self) -> &'static str {
        match self {
            Answering::Exactly => "on_the_exact_kernel",
            Answering::ThroughTheApplication => "through_the_application_s_body",
        }
    }
}

pub fn from_the_environment(name: &str) -> Option<u64> {
    std::env::var(name).ok()?.parse().ok()
}

/// A campaign over the cases `draw` gives, held by `check` on the kernel
/// `answering` names, for as long as the environment says, and its report.
pub fn campaign_over(
    what: &str,
    answering: Answering,
    draw: fn(u64) -> Case,
    check: fn(&Case) -> Result<(), Flaw>,
) {
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
    println!(
        "campaign of {what} {} from seed {first}, for {seconds} s",
        answering.on()
    );

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

    print_report(&report, answering);
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

fn print_report(report: &Report<Case>, answering: Answering) {
    println!("{} cases tried", report.tried);
    println!(
        "{} lines held, {} left out for grazing a curved wall",
        HELD.load(Ordering::Relaxed),
        GRAZING.load(Ordering::Relaxed)
    );
    println!(
        "{} lines left out for running through a cone's tip",
        TIPS.load(Ordering::Relaxed)
    );
    println!(
        "{} cases asking for a conic declined as unsupported",
        DECLINED.load(Ordering::Relaxed)
    );
    println!(
        "{} cases with a turned wall a hair thin declined as no profile",
        THIN.load(Ordering::Relaxed)
    );
    for (rule, count) in &report.broken {
        println!("  {rule:?}: broken by {count}");
    }
    for (seed, rule) in &report.failed {
        println!("failed {seed} {rule:?}");
    }
    for finding in &report.findings {
        println!(
            "\n── {:?}, seed {} ──\nas drawn: {:?}\nshrunk:   {:?}\n\n#[test]\nfn seed_{}_keeps_every_rule_{}() {{\n    random_solids::{}(&{});\n}}",
            finding.flaw.rule(),
            finding.seed,
            finding.flaw,
            finding.shrunk_flaw,
            finding.seed,
            answering.test(),
            answering.holds(),
            finding.shrunk.to_string().replace('\n', "\n    "),
        );
    }
}
