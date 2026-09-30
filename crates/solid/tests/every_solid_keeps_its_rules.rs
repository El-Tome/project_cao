//! Solids drawn at random and combined at random, held to the rules every
//! result must keep.
//!
//! Closes #448.
//! - a generator draws solids and sequences of add and cut from a seed, and
//!   the same seed gives the same case — `the_same_seed_draws_the_same_case`,
//!   `the_generator_draws_every_kind_of_solid_and_step`,
//!   `every_leaf_drawn_is_a_solid`
//! - a broken case is shrunk to the smallest input that still breaks —
//!   `a_failing_case_shrinks_to_the_step_that_fails`,
//!   `shrinking_any_drawn_case_comes_to_an_end`
//! - and printed as Rust that pastes into a named test —
//!   `a_case_prints_as_the_rust_that_builds_it`,
//!   `the_printed_rust_builds_the_case_it_was_printed_from`
//! - the run sits behind a flag, bounded by a deadline, out of the gate —
//!   `a_campaign_of_random_solids_keeps_every_rule`
//! - the gate does not get slower — no test: measured, the tests this adds
//!   to the gate take about a third of a second of the test run; no campaign
//!   runs there
//! - the existing tests do not move — no test: no existing assertion changed.
//!   One helper under the architecture test did: `declared_dependencies` now
//!   leaves a crate's own name out, held by
//!   `a_crate_naming_itself_for_its_own_tests_reaches_for_nothing_new` there
//! - the volume of a solid raised on its own is the arithmetic: a prism its
//!   area times its height, a revolution Pappus, less what its flats lose —
//!   `a_prism_is_promised_its_area_times_its_height_and_a_turn_pappus`,
//!   caught out by a solid broken on purpose —
//!   `a_solid_raised_short_of_its_promise_is_a_volume_flaw`
//! - a case that keeps every rule is held in the gate by name —
//!   `a_block_bored_through_and_given_a_boss_keeps_every_rule`
//!
//! The campaign is run by hand:
//!
//! ```text
//! CAO_FUZZ_SECONDS=300 cargo test --release -p cao_solid \
//!     --test every_solid_keeps_its_rules -- --ignored --nocapture
//! ```
//!
//! `CAO_FUZZ_SEED` starts it from a given seed rather than from the clock, and
//! `CAO_FUZZ_PATIENCE` is how many seconds one case may take before it counts
//! as no answer.

mod random_solids;

use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};

use cao_solid::soundness::{Check, Flaw, Report, campaign, shrink};
use glam::DVec2;
use random_solids::{Case, Leaf, Mode, Outline, Plane, Step};

#[test]
fn the_same_seed_draws_the_same_case() {
    for seed in 0..200 {
        assert_eq!(Case::drawn(seed), Case::drawn(seed), "seed {seed}");
    }
    let distinct: std::collections::BTreeSet<String> =
        (0..200).map(|seed| Case::drawn(seed).to_string()).collect();
    assert!(distinct.len() > 190, "{} distinct cases", distinct.len());
}

#[test]
fn the_generator_draws_every_kind_of_solid_and_step() {
    let cases: Vec<Case> = (0..1000).map(Case::drawn).collect();
    let leaves = || cases.iter().flat_map(Case::leaves);
    let seen = |what: &str, found: bool| assert!(found, "no case drew {what}");

    seen(
        "a rectangle",
        leaves().any(|leaf| {
            matches!(
                leaf,
                Leaf::Prism {
                    outline: Outline::Rectangle { .. },
                    ..
                }
            )
        }),
    );
    seen(
        "a circle",
        leaves().any(|leaf| {
            matches!(
                leaf,
                Leaf::Prism {
                    outline: Outline::Circle { .. },
                    ..
                }
            )
        }),
    );
    seen(
        "a star",
        leaves().any(|leaf| {
            matches!(
                leaf,
                Leaf::Prism {
                    outline: Outline::Star { .. },
                    ..
                }
            )
        }),
    );
    seen(
        "a ring",
        leaves().any(|leaf| {
            matches!(
                leaf,
                Leaf::Prism {
                    outline: Outline::Ring { .. },
                    ..
                }
            )
        }),
    );
    seen(
        "a prism pushed backwards",
        leaves().any(|leaf| matches!(leaf, Leaf::Prism { height, .. } if *height < 0.0)),
    );
    seen(
        "a full turn",
        leaves().any(|leaf| matches!(leaf, Leaf::Revolution { degrees, .. } if *degrees == 360.0)),
    );
    seen(
        "a part turn",
        leaves().any(|leaf| matches!(leaf, Leaf::Revolution { degrees, .. } if *degrees < 360.0)),
    );
    seen(
        "a turn touching its axis",
        leaves().any(|leaf| matches!(leaf, Leaf::Revolution { low, .. } if low.x == 0.0)),
    );
    for (name, kind) in [("XY", 0), ("XZ", 1), ("YZ", 2), ("a tilted plane", 3)] {
        seen(name, leaves().any(|leaf| plane_kind(leaf.plane()) == kind));
    }
    seen(
        "a cut",
        cases
            .iter()
            .flat_map(|case| &case.steps)
            .any(|step| step.mode == Mode::Cut),
    );
    seen(
        "an addition",
        cases
            .iter()
            .flat_map(|case| &case.steps)
            .any(|step| step.mode == Mode::Add),
    );
    for count in 1..=5 {
        seen(
            &format!("{count} steps"),
            cases.iter().any(|case| case.steps.len() == count),
        );
    }
    seen(
        "a tool on the plane of the solid before it",
        cases.iter().any(|case| {
            case.steps
                .iter()
                .any(|step| step.tool.plane() == case.start.plane())
        }),
    );
    seen(
        "a circle not started at nought",
        leaves().any(|leaf| matches!(leaf, Leaf::Prism { outline: Outline::Circle { from, .. }, .. } if *from != 0.0)),
    );
    seen(
        "a turn a hair off its axis",
        leaves().any(|leaf| matches!(leaf, Leaf::Revolution { low, .. } if low.x != 0.0 && low.x.abs() < 1e-3)),
    );
    seen(
        "a plane missed by a hair",
        cases.iter().any(|case| {
            case.steps
                .iter()
                .any(|step| match (case.start.plane(), step.tool.plane()) {
                    (Plane::Xy(first), Plane::Xy(second)) => {
                        first != second && (first - second).abs() < 1e-4
                    }
                    _ => false,
                })
        }),
    );
    seen(
        "a plane tilted by a hair",
        leaves().any(|leaf| matches!(leaf.plane(), Plane::Tilted { turn, .. } if turn.x.abs() < 1e-2 && turn.x != 0.0)),
    );
    seen(
        "a case drawn thirty times larger",
        leaves().any(|leaf| matches!(leaf, Leaf::Prism { height, .. } if height.abs() > 60.0)),
    );
    seen(
        "two circles on one centre",
        cases.iter().any(|case| {
            let centres: Vec<DVec2> = case
                .leaves()
                .filter_map(|leaf| match leaf {
                    Leaf::Prism {
                        outline: Outline::Circle { center, .. },
                        ..
                    } => Some(*center),
                    _ => None,
                })
                .collect();
            (1..centres.len()).any(|index| centres[..index].contains(&centres[index]))
        }),
    );
}

#[test]
fn every_leaf_drawn_is_a_solid() {
    for seed in 0..2000 {
        let case = Case::drawn(seed);
        assert!(case.leaves().all(Leaf::is_solid), "seed {seed}: {case}");
    }
}

fn plane_kind(plane: &Plane) -> u8 {
    match plane {
        Plane::Xy(_) => 0,
        Plane::Xz(_) => 1,
        Plane::Yz(_) => 2,
        Plane::Tilted { .. } => 3,
    }
}

#[test]
fn a_block_bored_through_and_given_a_boss_keeps_every_rule() {
    random_solids::holds(&Case::new(
        Leaf::prism(
            Plane::xy(0.0),
            Outline::rectangle([0.0, 0.0], [20.0, 10.0]),
            10.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(-1.0),
                Outline::circle([6.0, 5.0], 2.5),
                12.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(10.0),
                Outline::rectangle([12.0, 2.0], [18.0, 8.0]),
                4.0,
            )),
        ],
    ));
}

#[test]
fn a_prism_is_promised_its_area_times_its_height_and_a_turn_pappus() {
    let block = Leaf::prism(
        Plane::xz(3.0),
        Outline::rectangle([1.0, 2.0], [5.0, 4.5]),
        -6.0,
    );
    assert_eq!(block.promise(), (4.0 * 2.5 * 6.0, 0.0));

    let tube = Leaf::prism(Plane::xy(0.0), Outline::ring([0.0, 0.0], 3.0, 1.0), 2.0);
    let (promised, _) = tube.promise();
    let flats = |radius: f64| 24.0 * radius * radius * (std::f64::consts::TAU / 48.0).sin();
    assert!(
        (promised - (flats(3.0) - flats(1.0)) * 2.0).abs() < 1e-9,
        "{promised}"
    );

    let turned = Leaf::revolution(Plane::xy(0.0), [2.0, 0.0], [4.0, 3.0], 90.0);
    let (pappus, loss) = turned.promise();
    let expected = std::f64::consts::FRAC_PI_2 / 2.0 * (16.0 - 4.0) * 3.0;
    assert!((pappus - expected).abs() < 1e-9, "{pappus}");
    assert!(loss > 0.0 && loss < 0.01 * pappus, "{loss}");
}

#[test]
fn a_solid_raised_short_of_its_promise_is_a_volume_flaw() {
    let block = Leaf::prism(
        Plane::xy(0.0),
        Outline::rectangle([0.0, 0.0], [4.0, 2.5]),
        6.0,
    );
    let lower = Leaf::prism(
        Plane::xy(0.0),
        Outline::rectangle([0.0, 0.0], [4.0, 2.5]),
        5.0,
    );
    let raised = |leaf: &Leaf| leaf.solid().expect("a solid").triangles();

    assert_eq!(
        random_solids::kept_its_promise(&block, &raised(&block)),
        Ok(())
    );
    assert!(matches!(
        random_solids::kept_its_promise(&block, &raised(&lower)),
        Err(Flaw::Volume { promised: 60.0, .. })
    ));

    let turned = Leaf::revolution(Plane::xy(0.0), [2.0, 0.0], [4.0, 3.0], 360.0);
    let shrunk: Vec<[glam::DVec3; 3]> = raised(&turned)
        .iter()
        .map(|triangle| triangle.map(|corner| corner * 0.998))
        .collect();
    assert_eq!(
        random_solids::kept_its_promise(&turned, &raised(&turned)),
        Ok(())
    );
    assert!(matches!(
        random_solids::kept_its_promise(&turned, &shrunk),
        Err(Flaw::Volume { .. })
    ));
}

fn every_kind() -> Case {
    Case::new(
        Leaf::prism(
            Plane::xy(0.0),
            Outline::rectangle([0.0, 0.0], [10.0, 8.5]),
            10.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xz(-1.0),
                Outline::circle([3.0, 4.0], 2.5),
                -12.0,
            )),
            Step::add(Leaf::prism(
                Plane::tilted([1.0, 2.0, 0.1], [15.0, 0.0, -45.0]),
                Outline::star([5.0, 5.0], &[[6.0, 5.0], [5.0, 7.5], [3.0, 4.0]]),
                1e-3,
            )),
            Step::cut(Leaf::prism(
                Plane::yz(2.0),
                Outline::ring([4.0, 4.0], 3.0, 1.5),
                4.0,
            )),
            Step::add(Leaf::revolution(
                Plane::xy(1.0),
                [0.0, -3.0],
                [2.5, 3.0],
                105.0,
            )),
        ],
    )
}

#[test]
fn a_case_prints_as_the_rust_that_builds_it() {
    let printed = every_kind().to_string();
    assert_eq!(
        printed,
        "Case::new(
    Leaf::prism(Plane::xy(0.0), Outline::rectangle([0.0, 0.0], [10.0, 8.5]), 10.0),
    vec![
        Step::cut(Leaf::prism(Plane::xz(-1.0), Outline::circle([3.0, 4.0], 2.5), -12.0)),
        Step::add(Leaf::prism(Plane::tilted([1.0, 2.0, 0.1], [15.0, 0.0, -45.0]), Outline::star([5.0, 5.0], &[[6.0, 5.0], [5.0, 7.5], [3.0, 4.0]]), 0.001)),
        Step::cut(Leaf::prism(Plane::yz(2.0), Outline::ring([4.0, 4.0], 3.0, 1.5), 4.0)),
        Step::add(Leaf::revolution(Plane::xy(1.0), [0.0, -3.0], [2.5, 3.0], 105.0)),
    ],
)"
    );
}

#[test]
fn the_printed_rust_builds_the_case_it_was_printed_from() {
    let pasted = Case::new(
        Leaf::prism(
            Plane::xy(0.0),
            Outline::rectangle([0.0, 0.0], [10.0, 8.5]),
            10.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xz(-1.0),
                Outline::circle([3.0, 4.0], 2.5),
                -12.0,
            )),
            Step::add(Leaf::prism(
                Plane::tilted([1.0, 2.0, 0.1], [15.0, 0.0, -45.0]),
                Outline::star([5.0, 5.0], &[[6.0, 5.0], [5.0, 7.5], [3.0, 4.0]]),
                0.001,
            )),
            Step::cut(Leaf::prism(
                Plane::yz(2.0),
                Outline::ring([4.0, 4.0], 3.0, 1.5),
                4.0,
            )),
            Step::add(Leaf::revolution(
                Plane::xy(1.0),
                [0.0, -3.0],
                [2.5, 3.0],
                105.0,
            )),
        ],
    );
    assert_eq!(pasted, every_kind());

    for seed in 0..200 {
        let drawn = Case::drawn(seed);
        let printed = drawn.to_string();
        assert!(
            !printed.contains("NaN") && !printed.contains("inf"),
            "seed {seed} prints a number Rust cannot read back: {printed}"
        );
    }
}

#[test]
fn a_failing_case_shrinks_to_the_step_that_fails() {
    let cuts_a_circle = |case: &Case| {
        case.steps.iter().any(|step| {
            step.mode == Mode::Cut
                && matches!(
                    step.tool,
                    Leaf::Prism {
                        outline: Outline::Circle { .. },
                        ..
                    }
                )
        })
    };
    let drawn = (0..1000)
        .map(Case::drawn)
        .find(|case| case.steps.len() == 3 && cuts_a_circle(case))
        .expect("a drawn case with three steps, one of them a circle cut");

    let shrunk = shrink(drawn, Case::smaller, cuts_a_circle, || true);

    assert_eq!(shrunk.steps.len(), 1, "{shrunk}");
    assert!(cuts_a_circle(&shrunk), "{shrunk}");
    assert_eq!(
        *shrunk.steps[0].tool.plane(),
        Plane::xy(shrunk.steps[0].tool.plane().frame().0.z),
        "{shrunk}"
    );
}

#[test]
fn shrinking_any_drawn_case_comes_to_an_end() {
    for seed in 0..200 {
        let mut rounds = 0;
        let shrunk = shrink(
            Case::drawn(seed),
            Case::smaller,
            |_| true,
            || {
                rounds += 1;
                rounds < 100_000
            },
        );
        assert!(rounds < 100_000, "seed {seed} was still shrinking");
        assert!(
            shrunk.smaller().is_empty(),
            "seed {seed} stopped short: {shrunk}"
        );
    }
}

fn from_the_environment(name: &str) -> Option<u64> {
    std::env::var(name).ok()?.parse().ok()
}

#[test]
#[ignore = "a campaign, run by hand: see the head of this file"]
fn a_campaign_of_random_solids_keeps_every_rule() {
    let seconds = from_the_environment("CAO_FUZZ_SECONDS").unwrap_or(60);
    let first = from_the_environment("CAO_FUZZ_SEED").unwrap_or_else(|| {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_or(1, |since| since.as_secs())
    });
    let patience = Duration::from_secs(from_the_environment("CAO_FUZZ_PATIENCE").unwrap_or(30));
    let deadline = Instant::now() + Duration::from_secs(seconds);
    println!("campaign from seed {first}, for {seconds} s");

    let check: Check<Case> = Arc::new(random_solids::check);
    let quiet = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let report = campaign(first.., Case::drawn, check, Case::smaller, patience, || {
        Instant::now() < deadline
    });
    std::panic::set_hook(quiet);

    print_report(&report);
    assert!(
        report.findings.is_empty(),
        "{} cases of {} broke a rule",
        report.broken.iter().map(|(_, count)| count).sum::<usize>(),
        report.tried
    );
}

fn print_report(report: &Report<Case>) {
    println!("{} cases tried", report.tried);
    for (rule, count) in &report.broken {
        println!("  {rule:?}: broken by {count}");
    }
    for finding in &report.findings {
        println!(
            "\n── {:?}, seed {} ──\nas drawn: {}\nshrunk:   {}\n\n#[test]\nfn seed_{}_keeps_every_rule() {{\n    random_solids::holds(&{});\n}}",
            finding.flaw.rule(),
            finding.seed,
            describe(&finding.flaw),
            describe(&finding.shrunk_flaw),
            finding.seed,
            finding.shrunk.to_string().replace('\n', "\n    "),
        );
    }
}

fn describe(flaw: &Flaw) -> String {
    format!("{flaw:?}")
}
