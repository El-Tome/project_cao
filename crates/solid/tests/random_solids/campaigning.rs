//! What a long campaign over random cases is made of, whatever it draws and
//! whichever kernel it holds: the clock and the seeds it runs over, the lines
//! it counted, its report, and the seeds it named shrunk one by one.
//!
//! Compiled only when asked for — `--features campaigns` — as the campaigns
//! themselves are.

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant, SystemTime};

use cao_solid::soundness::{Check, Flaw, Random, Report, Rerun, campaign_across, shrunk_across};

use super::Case;

/// How many lines the campaign held its cases along, how many it left out
/// for grazing a curved wall or running through a cone's tip, how many
/// cases asking beyond the conics, or meeting along one, the kernel declined
/// as unsupported, and how many turned walls a hair thin it declined to
/// raise.
pub static HELD: AtomicUsize = AtomicUsize::new(0);
pub static GRAZING: AtomicUsize = AtomicUsize::new(0);
pub static TIPS: AtomicUsize = AtomicUsize::new(0);
pub static DECLINED: AtomicUsize = AtomicUsize::new(0);
pub static THIN: AtomicUsize = AtomicUsize::new(0);

/// How many steps the kernel built though it reads them unstably (#552).
pub static UNSTABLE: AtomicUsize = AtomicUsize::new(0);

/// How many cases met along each kind of conic (#552), and along any.
pub static MET_ALONG_A_CONIC: AtomicUsize = AtomicUsize::new(0);
pub static ELLIPSES: AtomicUsize = AtomicUsize::new(0);
pub static PARABOLAS: AtomicUsize = AtomicUsize::new(0);
pub static HYPERBOLAS: AtomicUsize = AtomicUsize::new(0);
pub static LINES_THROUGH_AN_APEX: AtomicUsize = AtomicUsize::new(0);
pub static APEX_ALONE: AtomicUsize = AtomicUsize::new(0);
pub static TANGENT_RULINGS: AtomicUsize = AtomicUsize::new(0);

/// The conics a case meets along, tallied for the report.
pub fn tally_conics(case: &Case) {
    let conics = case.conics();
    let kinds = [
        (&ELLIPSES, conics.ellipses),
        (&PARABOLAS, conics.parabolas),
        (&HYPERBOLAS, conics.hyperbolas),
        (&LINES_THROUGH_AN_APEX, conics.lines_through_an_apex),
        (&APEX_ALONE, conics.apex_alone),
        (&TANGENT_RULINGS, conics.tangent_rulings),
    ];
    for (tally, count) in kinds {
        if count > 0 {
            tally.fetch_add(1, Ordering::Relaxed);
        }
    }
    if conics.total() > 0 {
        MET_ALONG_A_CONIC.fetch_add(1, Ordering::Relaxed);
    }
}

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

/// How many threads a campaign and its shrinks run on: `CAO_FUZZ_THREADS`,
/// or every core of the machine.
fn threads() -> usize {
    from_the_environment("CAO_FUZZ_THREADS").map_or_else(
        || std::thread::available_parallelism().map_or(1, |cores| cores.get()),
        |threads| threads as usize,
    )
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
    let threads = threads();
    let deadline = Instant::now() + Duration::from_secs(seconds);
    println!(
        "campaign of {what} {} from seed {first}, for {seconds} s, on {threads} threads",
        answering.on()
    );

    let check: Check<Case> = Arc::new(check);
    let quiet = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let report = campaign_across(
        threads,
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
/// printed as a test in seed order: what a campaign's failures are sorted
/// into distinct ones from. The seeds are shrunk each on its own, on as many
/// threads as the campaign ran on.
pub fn shrunk_one_by_one(seeds: Vec<u64>, draw: fn(u64) -> Case, check: Check<Case>) {
    let patience = Duration::from_secs(from_the_environment("CAO_FUZZ_PATIENCE").unwrap_or(30));
    let threads = threads();
    println!("{} seeds shrunk on {threads} threads", seeds.len());
    let quiet = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    shrunk_across(
        threads,
        seeds,
        draw,
        check,
        Case::smaller,
        patience,
        |rerun| match rerun {
            Rerun::Holds(seed) => println!("\n── holds, seed {seed} ──"),
            Rerun::Breaks { finding, hung } => println!(
                "\n── {:?}, seed {} ──\nas drawn: {:?}\nshrunk:   {:?}{}\n{}",
                finding.flaw.rule(),
                finding.seed,
                finding.flaw,
                finding.shrunk_flaw,
                if hung {
                    "\nhung:     a check never came back, the shrink stopped there"
                } else {
                    ""
                },
                finding.shrunk.to_string().replace('\n', "\n    "),
            ),
        },
    );
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
        "{} cases asking beyond the conics declined as unsupported",
        DECLINED.load(Ordering::Relaxed)
    );
    let met = MET_ALONG_A_CONIC.load(Ordering::Relaxed);
    if met > 0 {
        println!(
            "{met} cases met along a conic: {} along an ellipse, {} a parabola, {} a hyperbola, \
             {} two lines through an apex, {} an apex alone, {} a tangent ruling",
            ELLIPSES.load(Ordering::Relaxed),
            PARABOLAS.load(Ordering::Relaxed),
            HYPERBOLAS.load(Ordering::Relaxed),
            LINES_THROUGH_AN_APEX.load(Ordering::Relaxed),
            APEX_ALONE.load(Ordering::Relaxed),
            TANGENT_RULINGS.load(Ordering::Relaxed),
        );
    }
    println!(
        "{} cases with a turned wall a hair thin declined as no profile",
        THIN.load(Ordering::Relaxed)
    );
    println!(
        "{} steps built at an unstable lean",
        UNSTABLE.load(Ordering::Relaxed)
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
