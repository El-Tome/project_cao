//! #449's maquette: a circle raised into a cylinder and a hole bored through
//! it, computed by the flats written here and by truck behind `cao_solid`,
//! each held to #448's rules and to the arithmetic.
//!
//! On a branch not meant to be merged. What it answers is written on #447.
//!
//! ```text
//! cargo test --release -p cao_solid --test a_bored_cylinder_on_two_kernels \
//!     -- --nocapture
//! ```

use std::f64::consts::PI;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::time::Instant;

use cao_solid::exact::{Body, Contour, Frame};
use cao_solid::soundness::{Flaw, Lines, Silence, Spans, Triangle, closed, enclosed, uncrossed};
use cao_solid::{Loop, Mesh};
use glam::{DVec2, DVec3};

const RADIUS: f64 = 20.0;
const HEIGHT: f64 = 10.0;
const HOLE: f64 = 5.0;

/// A cylinder standing on the XY plane: a centre, a radius, and the heights
/// of its two ends.
#[derive(Clone, Copy)]
struct Round {
    center: DVec2,
    radius: f64,
    from: f64,
    to: f64,
}

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    Add,
    Cut,
}

struct Case {
    name: &'static str,
    tool: Option<(Round, Mode)>,
}

fn stock() -> Round {
    Round {
        center: DVec2::ZERO,
        radius: RADIUS,
        from: 0.0,
        to: HEIGHT,
    }
}

fn hole(center: DVec2, from: f64, to: f64) -> Round {
    Round {
        center,
        radius: HOLE,
        from,
        to,
    }
}

fn cases() -> Vec<Case> {
    let off_axis = DVec2::new(8.0, 0.0);
    let tangent = DVec2::new(RADIUS - HOLE, 0.0);
    let cut = |name, center, from, to| Case {
        name,
        tool: Some((hole(center, from, to), Mode::Cut)),
    };
    vec![
        Case {
            name: "a circle raised",
            tool: None,
        },
        cut("a hole bored flush, off the axis", off_axis, 0.0, HEIGHT),
        cut(
            "a hole bored through, off the axis",
            off_axis,
            -1.0,
            HEIGHT + 1.0,
        ),
        cut(
            "a hole bored flush, tangent to the wall",
            tangent,
            0.0,
            HEIGHT,
        ),
        cut(
            "a hole bored through, tangent to the wall",
            tangent,
            -1.0,
            HEIGHT + 1.0,
        ),
        cut("a hole bored flush, on the axis", DVec2::ZERO, 0.0, HEIGHT),
        cut(
            "a hole bored through, on the axis",
            DVec2::ZERO,
            -1.0,
            HEIGHT + 1.0,
        ),
        cut("a hole bored flush but for a hair", off_axis, 1e-7, HEIGHT),
        Case {
            name: "a boss standing on the top",
            tool: Some((hole(off_axis, HEIGHT, HEIGHT + 5.0), Mode::Add)),
        },
        Case {
            name: "a boss sunk into the top",
            tool: Some((hole(off_axis, HEIGHT - 1.0, HEIGHT + 5.0), Mode::Add)),
        },
    ]
}

/// The volume arithmetic promises: the true circles, not their flats.
fn arithmetic(case: &Case) -> f64 {
    let cylinder = |round: Round| PI * round.radius * round.radius * (round.to - round.from);
    let whole = cylinder(stock());
    match case.tool {
        None => whole,
        Some((tool, Mode::Cut)) => {
            let inside = Round {
                from: tool.from.max(0.0),
                to: tool.to.min(HEIGHT),
                ..tool
            };
            whole - cylinder(inside)
        }
        Some((tool, Mode::Add)) => {
            let outside = Round {
                from: tool.from.max(HEIGHT),
                ..tool
            };
            whole + cylinder(outside)
        }
    }
}

const FLATS: usize = 48;

fn raised_as_flats(round: Round) -> Mesh {
    let outline: Vec<DVec2> = (0..FLATS)
        .map(|step| {
            round.center + DVec2::from_angle(2.0 * PI * step as f64 / FLATS as f64) * round.radius
        })
        .collect();
    let curves = vec![Some(0); FLATS];
    let triangles: Vec<[DVec2; 3]> = (0..FLATS)
        .map(|index| [round.center, outline[index], outline[(index + 1) % FLATS]])
        .collect();
    cao_solid::prism(
        Loop {
            points: &outline,
            curves: &curves,
        },
        &[],
        &triangles,
        |point| DVec3::new(point.x, point.y, round.from),
        DVec3::Z * (round.to - round.from),
    )
}

fn raised_by_truck(round: Round) -> Option<Body> {
    let frame = Frame {
        origin: DVec3::Z * round.from,
        u: DVec3::X,
        v: DVec3::Y,
    };
    Body::raised(
        &Contour::circle(round.center, round.radius),
        &[],
        frame,
        DVec3::Z * (round.to - round.from),
    )
}

/// What one kernel made of one case: its triangles and those of its inputs,
/// or why it has none.
struct Made {
    inputs: Vec<Vec<Triangle>>,
    result: Result<Vec<Triangle>, Flaw>,
    seconds: f64,
}

fn silence(flaw: Silence) -> Result<Vec<Triangle>, Flaw> {
    Err(Flaw::NoAnswer(flaw))
}

fn by_flats(case: &Case) -> Made {
    let body = raised_as_flats(stock());
    let tool = case
        .tool
        .map(|(round, mode)| (raised_as_flats(round), mode));
    let mut inputs = vec![body.triangles()];
    inputs.extend(tool.iter().map(|(mesh, _)| mesh.triangles()));
    let started = Instant::now();
    let result = catch_unwind(AssertUnwindSafe(|| match &tool {
        None => body.clone(),
        Some((mesh, Mode::Add)) => body.union(mesh),
        Some((mesh, Mode::Cut)) => body.difference(mesh),
    }));
    Made {
        inputs,
        seconds: started.elapsed().as_secs_f64(),
        result: result
            .map(|mesh| mesh.triangles())
            .or_else(|_| silence(Silence::Panicked(None))),
    }
}

fn by_truck(case: &Case) -> Made {
    let (Some(body), tool) = (
        raised_by_truck(stock()),
        case.tool
            .map(|(round, mode)| (raised_by_truck(round), mode)),
    ) else {
        return Made {
            inputs: Vec::new(),
            result: silence(Silence::Refused),
            seconds: 0.0,
        };
    };
    let mut inputs: Vec<Vec<Triangle>> = vec![body.triangles().unwrap_or_default()];
    if let Some((Some(tool), _)) = &tool {
        inputs.push(tool.triangles().unwrap_or_default());
    }
    let started = Instant::now();
    let answered = catch_unwind(AssertUnwindSafe(|| match &tool {
        None => Some(body.clone()),
        Some((None, _)) => None,
        Some((Some(tool), Mode::Add)) => body.joined(tool),
        Some((Some(tool), Mode::Cut)) => body.cut_by(tool),
    }));
    let seconds = started.elapsed().as_secs_f64();
    let result = match answered {
        Err(_) => silence(Silence::Panicked(None)),
        Ok(None) => silence(Silence::Refused),
        Ok(Some(body)) => body.triangles().ok_or(Flaw::NoAnswer(Silence::Refused)),
    };
    Made {
        inputs,
        result,
        seconds,
    }
}

/// How a result stands against the rules and the arithmetic.
struct Verdict {
    rules: Result<(), Flaw>,
    volume: Option<f64>,
    faces: usize,
}

/// How far a kernel meshing true curves may land from the arithmetic: its
/// triangles stand within a hundredth of the surface, which on these parts is
/// under a twentieth of a percent of the volume.
const MESHED: f64 = 1e-3;

/// Whether the result is held line by line against its inputs, or against the
/// arithmetic.
///
/// Line by line holds a kernel whose result is made of its inputs' own flats.
/// truck meshes every curve afresh, input and result apart, so the same wall
/// is laid a different way in each and a line grazing it finds them a mesh's
/// tolerance apart: along the lines it would fail on nothing but that.
#[derive(Clone, Copy, PartialEq)]
enum Measure {
    AlongTheLines,
    AgainstTheArithmetic,
}

fn judged(case: &Case, made: &Made, measure: Measure) -> Verdict {
    let triangles = match &made.result {
        Ok(triangles) => triangles,
        Err(flaw) => {
            return Verdict {
                rules: Err(flaw.clone()),
                volume: None,
                faces: 0,
            };
        }
    };
    let volume = enclosed(triangles);
    let low = DVec3::new(-RADIUS - 1.0, -RADIUS - 1.0, -2.0);
    let high = DVec3::new(RADIUS + 1.0, RADIUS + 1.0, HEIGHT + 6.0);
    let lines = Lines::across(low, high, 48);
    let spans: Vec<Vec<Spans>> = made
        .inputs
        .iter()
        .map(|input| lines.inside(input))
        .collect();
    let promised: Vec<Spans> = match (case.tool, spans.as_slice()) {
        (Some((_, mode)), [body, tool]) => body
            .iter()
            .zip(tool)
            .map(|(body, tool)| match mode {
                Mode::Add => body.union(tool),
                Mode::Cut => body.without(tool),
            })
            .collect(),
        (_, [body, ..]) => body.clone(),
        _ => Vec::new(),
    };
    let promise = arithmetic(case);
    let rules = closed(triangles)
        .and_then(|()| uncrossed(triangles))
        .and_then(|()| match measure {
            Measure::AlongTheLines => lines.compare(&promised, &lines.inside(triangles)),
            Measure::AgainstTheArithmetic if ((volume - promise) / promise).abs() > MESHED => {
                Err(Flaw::Volume {
                    promised: promise,
                    enclosed: volume,
                    worst: None,
                })
            }
            Measure::AgainstTheArithmetic => Ok(()),
        });
    Verdict {
        rules,
        volume: Some(volume),
        faces: triangles.len(),
    }
}

fn row(kernel: &str, case: &Case, made: &Made, verdict: &Verdict) -> String {
    let promised = arithmetic(case);
    let volume = verdict.volume.map_or("—".to_string(), |volume| {
        format!(
            "{volume:.2} ({:+.3} %)",
            (volume - promised) / promised * 100.0
        )
    });
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
    format!(
        "| {} | {kernel} | {rules} | {volume} | {promised:.2} | {} | {:.0} ms |",
        case.name,
        verdict.faces,
        made.seconds * 1000.0
    )
}

/// What each kernel answered on the day the maquette ran: kept every rule,
/// gave no answer, or broke a rule. A newer truck that moves one of these is
/// news for #447.
const SEEN: [(&str, &str, &str); 10] = [
    ("a circle raised", "keeps", "keeps"),
    ("a hole bored flush, off the axis", "keeps", "silent"),
    ("a hole bored through, off the axis", "keeps", "keeps"),
    ("a hole bored flush, tangent to the wall", "keeps", "silent"),
    (
        "a hole bored through, tangent to the wall",
        "keeps",
        "silent",
    ),
    ("a hole bored flush, on the axis", "keeps", "silent"),
    ("a hole bored through, on the axis", "keeps", "keeps"),
    ("a hole bored flush but for a hair", "breaks", "silent"),
    ("a boss standing on the top", "keeps", "silent"),
    ("a boss sunk into the top", "keeps", "keeps"),
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
        "| case | kernel | rules | volume (against the arithmetic) | arithmetic | triangles | boolean |"
    );
    println!("| --- | --- | --- | --- | --- | --- | --- |");
    for (case, (name, flats, truck)) in cases().iter().zip(SEEN) {
        assert_eq!(case.name, name);
        for (kernel, made, measure, expected) in [
            ("flats", by_flats(case), Measure::AlongTheLines, flats),
            (
                "truck",
                by_truck(case),
                Measure::AgainstTheArithmetic,
                truck,
            ),
        ] {
            let verdict = judged(case, &made, measure);
            println!("{}", row(kernel, case, &made, &verdict));
            assert_eq!(kind(&verdict), expected, "{kernel} on {name}");
        }
    }
}
