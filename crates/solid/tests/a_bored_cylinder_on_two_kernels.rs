//! #498's table: the sixteen cases of `docs/exact-kernel-journal.md` computed
//! by the flats written here and by the exact kernel of #498, each held to
//! #448's rules and to the arithmetic, beside what OpenCascade answered when
//! #447 measured it.
//!
//! The maquette's branch ran a third column, truck behind `cao_solid` (#449),
//! which answered fourteen of the eighteen rows with nothing. truck did not come
//! to `main` (#526): what it answered is written on #497 and in
//! `docs/exact-kernel-journal.md`.
//!
//! ```text
//! cargo test --release -p cao_solid --test a_bored_cylinder_on_two_kernels \
//!     -- --nocapture
//! ```
//!
//! Closes #498.
//! - a body lists every face, edge and vertex it is made of, each face with
//!   its exact surface and the edges bounding it, each edge with its exact
//!   curve, its two vertices and the faces on either side, each vertex with
//!   its point, and the list is held to the geometry: every edge on both its
//!   faces' surfaces, every vertex on its edges —
//!   `a_bored_cylinder_on_two_kernels`, which holds every body the exact
//!   kernel answers to `soundness::listed`; the check itself is tried on
//!   listings built by hand, sound and broken on purpose, in
//!   `soundness/listed/tests.rs`. An ellipse is declined by the design, so no
//!   edge lists one
//! - the table is filled for every case, on the same axes as #449's: keeps
//!   every rule, gives no answer, or breaks a rule; the volume against the
//!   arithmetic; the time of a boolean — `a_bored_cylinder_on_two_kernels`,
//!   whose arithmetic is held to what the flats enclose by
//!   `the_arithmetic_of_every_case_is_what_the_flats_enclose_but_for_their_sagitta`
//! - a campaign of an hour has been run on it, and every distinct failure it
//!   found is named, fixed or not — no test: a campaign is run by hand, by the
//!   ignored campaign of `random_exact_solids.rs`, and what it finds is named
//!   in `docs/exact-kernel-journal.md`
//! - the two weeks are accounted for, day by day, with whether the failures a
//!   campaign finds go down from one run to the next — no test: it is prose,
//!   in `docs/exact-kernel-journal.md`
//! - the answer is written on #497: whether it holds on the cases, how far two
//!   weeks got, the hardest part, and whether cones, spheres, tori, an exact
//!   STEP and a fillet read as weeks or months — no test: it is written on
//!   GitHub, which nothing in this workspace reads
//!
//! OpenCascade is quoted rather than run: #447 measured 7.8.1 through a C++
//! probe on #449's ten cases, and its comment folds several of them into one
//! row. Each is quoted from the row that covers it; the boss sunk into the
//! top has no row of its own and is quoted from the text, which says it
//! answers every case with exact volumes. The six cases after those ten were
//! never put to it.

use std::f64::consts::{PI, TAU};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::time::Instant;

use cao_solid::brep;
use cao_solid::profile::{Contour, Frame, Run};
use cao_solid::soundness::{
    Flaw, Lines, Silence, Spans, Triangle, closed, enclosed, listed, uncrossed,
};
use cao_solid::{Body, Loop};
use glam::{DVec2, DVec3};

/// A solid a case is made of.
#[derive(Clone, Copy)]
enum Solid {
    /// A cylinder standing on the XY plane between two heights.
    Standing {
        center: DVec2,
        radius: f64,
        from: f64,
        to: f64,
    },
    /// A box between two corners.
    Block { low: DVec3, high: DVec3 },
    /// A cylinder lying along Y between two abscissae, its axis at `center`
    /// in X and Z.
    Lying {
        center: DVec2,
        radius: f64,
        from: f64,
        to: f64,
    },
}

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    Add,
    Cut,
}

struct Case {
    number: &'static str,
    name: &'static str,
    start: Solid,
    steps: Vec<(Mode, Solid)>,
    /// The volume arithmetic promises, in closed form: the true circles, not
    /// their flats.
    arithmetic: f64,
}

const RADIUS: f64 = 20.0;
const HEIGHT: f64 = 10.0;
const HOLE: f64 = 5.0;

/// The stock of #449: a cylinder of radius 20 standing from 0 to 10.
fn stock() -> Solid {
    standing(DVec2::ZERO, RADIUS, 0.0, HEIGHT)
}

/// The block: 40 by 40 by 10, centred on the origin, from 0 to 10.
fn block() -> Solid {
    boxed([-20.0, -20.0, 0.0], [20.0, 20.0, HEIGHT])
}

fn standing(center: DVec2, radius: f64, from: f64, to: f64) -> Solid {
    Solid::Standing {
        center,
        radius,
        from,
        to,
    }
}

fn hole(x: f64, from: f64, to: f64) -> Solid {
    standing(DVec2::new(x, 0.0), HOLE, from, to)
}

fn boxed(low: [f64; 3], high: [f64; 3]) -> Solid {
    Solid::Block {
        low: DVec3::from(low),
        high: DVec3::from(high),
    }
}

fn case(
    number: &'static str,
    name: &'static str,
    start: Solid,
    steps: Vec<(Mode, Solid)>,
    arithmetic: f64,
) -> Case {
    Case {
        number,
        name,
        start,
        steps,
        arithmetic,
    }
}

fn cases() -> Vec<Case> {
    let whole = PI * RADIUS * RADIUS * HEIGHT;
    let bored = PI * HOLE * HOLE * HEIGHT;
    let boss = PI * HOLE * HOLE * 5.0;
    let slab = 40.0 * 40.0 * HEIGHT;
    let (flush, through) = ((0.0, HEIGHT), (-1.0, HEIGHT + 1.0));
    let cut = |x: f64, (from, to): (f64, f64)| vec![(Mode::Cut, hole(x, from, to))];
    vec![
        case("1", "a circle raised", stock(), vec![], whole),
        case(
            "2",
            "a hole bored flush, off the axis",
            stock(),
            cut(8.0, flush),
            whole - bored,
        ),
        case(
            "3",
            "a hole bored through, off the axis",
            stock(),
            cut(8.0, through),
            whole - bored,
        ),
        case(
            "4",
            "a hole bored flush, tangent to the wall",
            stock(),
            cut(RADIUS - HOLE, flush),
            whole - bored,
        ),
        case(
            "5",
            "a hole bored through, tangent to the wall",
            stock(),
            cut(RADIUS - HOLE, through),
            whole - bored,
        ),
        case(
            "6",
            "a hole bored flush, on the axis",
            stock(),
            cut(0.0, flush),
            whole - bored,
        ),
        case(
            "7",
            "a hole bored through, on the axis",
            stock(),
            cut(0.0, through),
            whole - bored,
        ),
        case(
            "8",
            "a hole bored flush but for a hair",
            stock(),
            cut(8.0, (1e-7, HEIGHT)),
            whole - PI * HOLE * HOLE * (HEIGHT - 1e-7),
        ),
        case(
            "9",
            "a boss standing on the top",
            stock(),
            vec![(Mode::Add, hole(8.0, HEIGHT, HEIGHT + 5.0))],
            whole + boss,
        ),
        case(
            "10",
            "a boss sunk into the top",
            stock(),
            vec![(Mode::Add, hole(8.0, HEIGHT - 1.0, HEIGHT + 5.0))],
            whole + boss,
        ),
        case(
            "11",
            "a round boss overhanging the top's edge",
            stock(),
            vec![(Mode::Add, hole(18.0, HEIGHT, HEIGHT + 5.0))],
            whole + boss,
        ),
        case(
            "11",
            "a block boss overhanging the top's edge",
            block(),
            vec![(Mode::Add, boxed([15.0, -5.0, HEIGHT], [25.0, 5.0, 15.0]))],
            slab + 10.0 * 10.0 * 5.0,
        ),
        case(
            "12",
            "two blocks sharing a wall, flush",
            block(),
            vec![(Mode::Add, boxed([20.0, -20.0, 0.0], [60.0, 20.0, HEIGHT]))],
            2.0 * slab,
        ),
        case(
            "12",
            "two blocks sharing a wall, offset",
            block(),
            vec![(Mode::Add, boxed([20.0, -10.0, 0.0], [60.0, 30.0, HEIGHT]))],
            2.0 * slab,
        ),
        case(
            "13",
            "a hole tangent to a side of a block",
            block(),
            cut(15.0, flush),
            slab - bored,
        ),
        case(
            "14",
            "a cylinder resting against a flat face",
            block(),
            vec![(
                Mode::Add,
                Solid::Lying {
                    center: DVec2::new(0.0, HEIGHT + HOLE),
                    radius: HOLE,
                    from: -15.0,
                    to: 15.0,
                },
            )],
            slab + PI * HOLE * HOLE * 30.0,
        ),
        case(
            "15",
            "two holes whose circles touch",
            stock(),
            vec![
                (Mode::Cut, hole(-5.0, 0.0, HEIGHT)),
                (Mode::Cut, hole(5.0, 0.0, HEIGHT)),
            ],
            whole - 2.0 * bored,
        ),
        case(
            "16",
            "a pocket flush with a side and the top",
            block(),
            vec![(Mode::Cut, boxed([10.0, -5.0, 5.0], [20.0, 5.0, HEIGHT]))],
            slab - 10.0 * 10.0 * 5.0,
        ),
    ]
}

impl Solid {
    /// Where the solid is drawn, what it is drawn from there, and where it is
    /// pushed: the vocabulary every kernel here takes.
    fn profile(self) -> (Profile, Frame, DVec3) {
        match self {
            Solid::Standing {
                center,
                radius,
                from,
                to,
            } => (
                Profile::Circle { center, radius },
                Frame {
                    origin: DVec3::Z * from,
                    u: DVec3::X,
                    v: DVec3::Y,
                },
                DVec3::Z * (to - from),
            ),
            Solid::Block { low, high } => (
                Profile::Rectangle {
                    low: low.truncate(),
                    high: high.truncate(),
                },
                Frame {
                    origin: DVec3::Z * low.z,
                    u: DVec3::X,
                    v: DVec3::Y,
                },
                DVec3::Z * (high.z - low.z),
            ),
            Solid::Lying {
                center,
                radius,
                from,
                to,
            } => (
                Profile::Circle { center, radius },
                Frame {
                    origin: DVec3::Y * from,
                    u: DVec3::X,
                    v: DVec3::Z,
                },
                DVec3::Y * (to - from),
            ),
        }
    }
}

#[derive(Clone, Copy)]
enum Profile {
    Circle { center: DVec2, radius: f64 },
    Rectangle { low: DVec2, high: DVec2 },
}

const FLATS: usize = 48;

fn raised_as_flats(solid: Solid) -> Body {
    let (profile, frame, travel) = solid.profile();
    let (outline, curves, triangles) = match profile {
        Profile::Circle { center, radius } => {
            let outline: Vec<DVec2> = (0..FLATS)
                .map(|step| center + DVec2::from_angle(TAU * step as f64 / FLATS as f64) * radius)
                .collect();
            let triangles: Vec<[DVec2; 3]> = (0..FLATS)
                .map(|index| [center, outline[index], outline[(index + 1) % FLATS]])
                .collect();
            (outline, vec![Some(0); FLATS], triangles)
        }
        Profile::Rectangle { low, high } => {
            let outline = vec![
                low,
                DVec2::new(high.x, low.y),
                high,
                DVec2::new(low.x, high.y),
            ];
            let triangles = vec![
                [outline[0], outline[1], outline[2]],
                [outline[0], outline[2], outline[3]],
            ];
            (outline, vec![None; 4], triangles)
        }
    };
    Body::prism(
        Loop {
            points: &outline,
            curves: &curves,
        },
        &[],
        &triangles,
        |point| frame.at(point),
        travel,
    )
}

/// A whole circle as the exact kernel is handed one: a single run all the
/// way round from one corner, as the campaign hands it.
fn raised_exactly(solid: Solid) -> Result<brep::Body, brep::Declined> {
    let (profile, frame, travel) = solid.profile();
    let contour = match profile {
        Profile::Circle { center, radius } => Contour {
            corners: vec![center + DVec2::X * radius],
            runs: vec![Run::Round { center, turn: TAU }],
        },
        Profile::Rectangle { low, high } => Contour::rectangle(low, high),
    };
    brep::Body::raised(&contour, &[], frame, travel)
}

/// How a kernel's answer to a case stands against the rules and the
/// arithmetic.
struct Verdict {
    rules: Result<(), Flaw>,
    volume: Option<f64>,
    /// The volume of the exact body itself, for the kernel that keeps one.
    exact: Option<f64>,
    triangles: usize,
    seconds: f64,
}

/// No answer, and how long the kernel took to give none.
fn silent(silence: Silence, seconds: f64) -> Verdict {
    Verdict {
        rules: Err(Flaw::NoAnswer(silence)),
        volume: None,
        exact: None,
        triangles: 0,
        seconds,
    }
}

/// How far a triangle of the exact kernel may stand from the surface it
/// stands for.
const MESH: f64 = 0.01;

/// How far a kernel meshing true curves may land from the arithmetic: its
/// triangles stand within a hundredth of the surface, which on these parts is
/// under a twentieth of a percent of the volume.
const MESHED: f64 = 1e-3;

/// How far the exact kernel's own volume may land from the arithmetic, as a
/// fraction of it: rounding, and nothing else.
const EXACT: f64 = 1e-9;

/// The flats, held line by line against their inputs: their result is made
/// of their inputs' own facets.
fn by_flats(case: &Case) -> Verdict {
    let start = raised_as_flats(case.start);
    let tools: Vec<(Mode, Body)> = case
        .steps
        .iter()
        .map(|(mode, solid)| (*mode, raised_as_flats(*solid)))
        .collect();
    let started = Instant::now();
    let result = catch_unwind(AssertUnwindSafe(|| {
        tools
            .iter()
            .fold(start.clone(), |body, (mode, tool)| match mode {
                Mode::Add => body.union(tool),
                Mode::Cut => body.difference(tool),
            })
    }));
    let seconds = started.elapsed().as_secs_f64();
    let Ok(result) = result else {
        return silent(Silence::Panicked(None), seconds);
    };
    let triangles = result.triangles();
    let (low, high) = std::iter::once(&start)
        .chain(tools.iter().map(|(_, tool)| tool))
        .filter_map(Body::bounds)
        .reduce(|(low, high), (other_low, other_high)| (low.min(other_low), high.max(other_high)))
        .expect("solids with a box");
    let lines = Lines::across(low - 1.0, high + 1.0, 48);
    let promised = tools
        .iter()
        .fold(lines.inside(&start.triangles()), |before, (mode, tool)| {
            let tool = lines.inside(&tool.triangles());
            before
                .iter()
                .zip(&tool)
                .map(|(before, tool)| match mode {
                    Mode::Add => before.union(tool),
                    Mode::Cut => before.without(tool),
                })
                .collect::<Vec<Spans>>()
        });
    Verdict {
        rules: closed(&triangles)
            .and_then(|()| uncrossed(&triangles))
            .and_then(|()| lines.compare(&promised, &lines.inside(&triangles))),
        volume: Some(enclosed(&triangles)),
        exact: None,
        triangles: triangles.len(),
        seconds,
    }
}

/// Whether triangles are closed, uncrossed, and enclose the arithmetic
/// within `MESHED`.
fn meshed(case: &Case, triangles: &[Triangle]) -> Result<(), Flaw> {
    closed(triangles)?;
    uncrossed(triangles)?;
    near_the_arithmetic(case, enclosed(triangles), MESHED)
}

/// Whether a volume lands within `share` of the arithmetic, as a fraction of
/// it: never a volume that is no number, which compares false either way.
fn near_the_arithmetic(case: &Case, volume: f64, share: f64) -> Result<(), Flaw> {
    let off = ((volume - case.arithmetic) / case.arithmetic).abs();
    if off.is_nan() || off > share {
        return Err(Flaw::Volume {
            promised: case.arithmetic,
            enclosed: volume,
            worst: None,
        });
    }
    Ok(())
}

/// The exact kernel, held to its own listing, its triangles to the rules and
/// the arithmetic within `MESHED`, and its own volume to the arithmetic
/// within rounding.
fn by_exact(case: &Case) -> Verdict {
    let mut seconds = 0.0;
    let answered = catch_unwind(AssertUnwindSafe(|| -> Result<_, brep::Declined> {
        let start = raised_exactly(case.start)?;
        let tools = case
            .steps
            .iter()
            .map(|(mode, solid)| raised_exactly(*solid).map(|tool| (*mode, tool)))
            .collect::<Result<Vec<_>, _>>()?;
        let started = Instant::now();
        let body = tools
            .iter()
            .try_fold(start, |body, (mode, tool)| match mode {
                Mode::Add => body.joined(tool),
                Mode::Cut => body.cut_by(tool),
            });
        seconds = started.elapsed().as_secs_f64();
        body
    }));
    let body = match answered {
        Err(_) => return silent(Silence::Panicked(None), seconds),
        Ok(Err(_declined)) => return silent(Silence::Refused, seconds),
        Ok(Ok(body)) => body,
    };
    let triangles = body.triangles(MESH);
    let volume = body.volume();
    let rules = listed(&body.listing(), body.scale().reach())
        .map_err(Flaw::from)
        .and_then(|()| meshed(case, &triangles))
        .and_then(|()| near_the_arithmetic(case, volume, EXACT));
    Verdict {
        rules,
        volume: Some(enclosed(&triangles)),
        exact: Some(volume),
        triangles: triangles.len(),
        seconds,
    }
}

fn against(volume: f64, promised: f64) -> String {
    format!(
        "{volume:.2} ({:+.3} %)",
        (volume - promised) / promised * 100.0
    )
}

fn row(kernel: &str, case: &Case, verdict: &Verdict) -> String {
    let rules = match &verdict.rules {
        Ok(()) => "keeps every rule".to_string(),
        Err(Flaw::NoAnswer(Silence::Refused)) => "no answer".to_string(),
        Err(Flaw::NoAnswer(Silence::Panicked(_))) => "panicked".to_string(),
        Err(Flaw::Volume {
            worst: Some(worst), ..
        }) => format!(
            "breaks Volume: {:.4} against {:.4} along one line",
            worst.enclosed, worst.promised
        ),
        Err(flaw) => format!("breaks {:?}", flaw.rule()),
    };
    let volume = verdict
        .volume
        .map_or("—".to_string(), |volume| against(volume, case.arithmetic));
    let exact = verdict
        .exact
        .map_or("—".to_string(), |volume| against(volume, case.arithmetic));
    format!(
        "| {} | {} | {kernel} | {rules} | {volume} | {exact} | {:.2} | {} | {:.0} ms |",
        case.number,
        case.name,
        case.arithmetic,
        verdict.triangles,
        verdict.seconds * 1000.0
    )
}

/// What each kernel answered on the day the table ran — kept every rule,
/// gave no answer, or broke a rule — and what #447 quotes OpenCascade 7.8.1
/// as answering. The exact kernel's column moves as it lands.
struct Seen {
    case: &'static str,
    flats: &'static str,
    exact: &'static str,
    opencascade: &'static str,
}

const fn seen(
    case: &'static str,
    flats: &'static str,
    exact: &'static str,
    opencascade: &'static str,
) -> Seen {
    Seen {
        case,
        flats,
        exact,
        opencascade,
    }
}

const EXACT_AS_QUOTED: &str = "exact";
const TANGENT_AS_QUOTED: &str = "valid, one non-manifold edge";
const SUNK_AS_QUOTED: &str = "answers, exact volume: no row of its own";
const NOT_MEASURED: &str = "not measured";

const SEEN: [Seen; 18] = [
    seen("a circle raised", "keeps", "keeps", EXACT_AS_QUOTED),
    seen(
        "a hole bored flush, off the axis",
        "keeps",
        "keeps",
        EXACT_AS_QUOTED,
    ),
    seen(
        "a hole bored through, off the axis",
        "keeps",
        "keeps",
        EXACT_AS_QUOTED,
    ),
    seen(
        "a hole bored flush, tangent to the wall",
        "keeps",
        "keeps",
        TANGENT_AS_QUOTED,
    ),
    seen(
        "a hole bored through, tangent to the wall",
        "keeps",
        "keeps",
        TANGENT_AS_QUOTED,
    ),
    seen(
        "a hole bored flush, on the axis",
        "keeps",
        "keeps",
        EXACT_AS_QUOTED,
    ),
    seen(
        "a hole bored through, on the axis",
        "keeps",
        "keeps",
        EXACT_AS_QUOTED,
    ),
    seen(
        "a hole bored flush but for a hair",
        "breaks",
        "keeps",
        EXACT_AS_QUOTED,
    ),
    seen(
        "a boss standing on the top",
        "keeps",
        "keeps",
        EXACT_AS_QUOTED,
    ),
    seen("a boss sunk into the top", "keeps", "keeps", SUNK_AS_QUOTED),
    seen(
        "a round boss overhanging the top's edge",
        "keeps",
        "keeps",
        NOT_MEASURED,
    ),
    seen(
        "a block boss overhanging the top's edge",
        "keeps",
        "keeps",
        NOT_MEASURED,
    ),
    seen(
        "two blocks sharing a wall, flush",
        "keeps",
        "keeps",
        NOT_MEASURED,
    ),
    seen(
        "two blocks sharing a wall, offset",
        "keeps",
        "keeps",
        NOT_MEASURED,
    ),
    seen(
        "a hole tangent to a side of a block",
        "keeps",
        "keeps",
        NOT_MEASURED,
    ),
    seen(
        "a cylinder resting against a flat face",
        "keeps",
        "keeps",
        NOT_MEASURED,
    ),
    seen(
        "two holes whose circles touch",
        "keeps",
        "keeps",
        NOT_MEASURED,
    ),
    seen(
        "a pocket flush with a side and the top",
        "keeps",
        "keeps",
        NOT_MEASURED,
    ),
];

fn kind(verdict: &Verdict) -> &'static str {
    match &verdict.rules {
        Ok(()) => "keeps",
        Err(Flaw::NoAnswer(_)) => "silent",
        Err(_) => "breaks",
    }
}

#[test]
fn a_bored_cylinder_on_two_kernels() {
    println!(
        "| # | case | kernel | rules | triangles' volume (against the arithmetic) | exact volume | arithmetic | triangles | boolean |"
    );
    println!("| --- | --- | --- | --- | --- | --- | --- | --- | --- |");
    let mut differ = Vec::new();
    for (case, expected) in cases().iter().zip(SEEN) {
        assert_eq!(case.name, expected.case);
        for (kernel, verdict, expected) in [
            ("flats", by_flats(case), expected.flats),
            ("exact", by_exact(case), expected.exact),
        ] {
            println!("{}", row(kernel, case, &verdict));
            if kind(&verdict) != expected {
                differ.push(format!(
                    "{kernel} on {}: {} where {expected} was seen",
                    case.name,
                    kind(&verdict)
                ));
            }
        }
        println!(
            "| {} | {} | OpenCascade 7.8.1, quoted from #447 | {} | — | — | {:.2} | — | {} |",
            case.number,
            case.name,
            expected.opencascade,
            case.arithmetic,
            if expected.opencascade == NOT_MEASURED {
                "—"
            } else {
                "1–3 ms"
            }
        );
    }
    assert!(differ.is_empty(), "{}", differ.join("\n"));
}

#[test]
fn the_arithmetic_of_every_case_is_what_the_flats_enclose_but_for_their_sagitta() {
    let flats_of_a_circle = FLATS as f64 / TAU * (TAU / FLATS as f64).sin();
    for case in cases() {
        let solids = std::iter::once(case.start).chain(case.steps.iter().map(|(_, solid)| *solid));
        let round = solids
            .clone()
            .any(|solid| !matches!(solid, Solid::Block { .. }));
        let enclosed = by_flats(&case).volume.expect("the flats answer every case");
        let lost = 1.0 - enclosed / case.arithmetic;
        if round {
            assert!(
                lost.abs() < 1.2 * (1.0 - flats_of_a_circle),
                "{}: the flats enclose {enclosed}, the arithmetic says {}",
                case.name,
                case.arithmetic
            );
        } else {
            assert!(
                lost.abs() < 1e-12,
                "{}: the flats enclose {enclosed}, the arithmetic says {}",
                case.name,
                case.arithmetic
            );
        }
    }
}

#[test]
fn a_volume_that_is_no_number_is_not_the_volume_arithmetic_promised() {
    for case in cases() {
        assert_eq!(
            near_the_arithmetic(&case, case.arithmetic, EXACT),
            Ok(()),
            "{}",
            case.name
        );
        assert!(
            near_the_arithmetic(&case, f64::NAN, MESHED).is_err(),
            "{}",
            case.name
        );
    }
}
