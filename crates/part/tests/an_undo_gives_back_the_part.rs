//! Parts drawn at random, gesture by gesture, each gesture taken back and
//! done again: what an undo leaves must be the part as it was, bit for bit.
//!
//! The document raises matter as it goes, but an undo builds the part again
//! from its history. Two roads to one part: if they ever part ways, an undo
//! shows the user a part they never had.
//!
//! Closes #448.
//! - an operation and its undo give back the part it started from, at the
//!   level of `cao_part` — `a_rectangle_raised_then_undone_gives_back_the_empty_part`,
//!   `a_part_whose_undo_changed_a_bit_is_caught`,
//!   `a_campaign_of_random_parts_undoes_every_gesture`
//!
//! The campaign is run by hand, like the one over solids:
//!
//! ```text
//! CAO_FUZZ_SECONDS=300 cargo test --release -p cao_part \
//!     --test an_undo_gives_back_the_part -- --ignored --nocapture
//! ```

use std::fmt::{self, Display, Formatter};
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};

use cao_part::PartDocument;
use cao_part::history::{ExtrusionMode, FaceAnchor, Operation, PointRef, RevolutionAxis};
use cao_sketch::{SketchAxis, WorkPlane};
use cao_solid::soundness::{
    Check, Flaw, Random, Report, Triangle, campaign, closed, first_difference, repeatable,
    uncrossed,
};
use glam::{DVec2, DVec3};

/// One gesture of a random part, in the words a printed case is written in.
///
/// Every gesture draws on the last drawing opened, and a gesture with nothing
/// to act on is skipped: that keeps every list of gestures a part, however it
/// is shrunk.
#[derive(Clone, Debug, PartialEq)]
enum Move {
    /// A drawing on one of the three planes of the origin: 0 for XY, 1 for
    /// XZ, 2 for YZ.
    Sketch(usize),
    /// A drawing on a flat face of the part, the rank-th one counting round.
    SketchOn(usize),
    Rectangle([f64; 2], [f64; 2]),
    Circle([f64; 2], f64),
    /// The area under a place, raised or cut by a depth.
    Raise([f64; 2], f64, bool),
    /// The area under a place, turned about one of the drawing's axes.
    Turn([f64; 2], bool, f64, bool),
}

impl Display for Move {
    fn fmt(&self, out: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Move::Sketch(plane) => write!(out, "Move::Sketch({plane})"),
            Move::SketchOn(rank) => write!(out, "Move::SketchOn({rank})"),
            Move::Rectangle(low, high) => write!(out, "Move::Rectangle({low:?}, {high:?})"),
            Move::Circle(center, radius) => write!(out, "Move::Circle({center:?}, {radius:?})"),
            Move::Raise(at, depth, cut) => write!(out, "Move::Raise({at:?}, {depth:?}, {cut})"),
            Move::Turn(at, about_v, degrees, cut) => {
                write!(out, "Move::Turn({at:?}, {about_v}, {degrees:?}, {cut})")
            }
        }
    }
}

struct Moves<'a>(&'a [Move]);

impl Display for Moves<'_> {
    fn fmt(&self, out: &mut Formatter<'_>) -> fmt::Result {
        writeln!(out, "&[")?;
        for gesture in self.0 {
            writeln!(out, "    {gesture},")?;
        }
        write!(out, "]")
    }
}

fn mode(cut: bool) -> ExtrusionMode {
    if cut {
        ExtrusionMode::Cut
    } else {
        ExtrusionMode::Add
    }
}

/// The operation a gesture stands for on this part, or nothing when there is
/// nothing for it to act on.
fn operation(document: &PartDocument, gesture: &Move) -> Option<Operation> {
    let sketch = document.sketches().len().checked_sub(1);
    match *gesture {
        Move::Sketch(plane) => Some(Operation::CreateSketch {
            plane: WorkPlane::ORIGIN_PLANES[plane % 3],
            on: None,
        }),
        Move::SketchOn(rank) => {
            let body = document.body();
            let flat: Vec<usize> = (0..body.faces_end())
                .filter(|face| body.is_flat(*face))
                .collect();
            let face = *flat.get(rank % flat.len().max(1))?;
            let pieces: Vec<_> = body.pieces_of(face).collect();
            let normal = pieces[0].normal();
            let up = if normal.z.abs() > 0.9 {
                DVec3::Y
            } else {
                DVec3::Z
            };
            let corners: Vec<DVec3> = pieces
                .iter()
                .flat_map(|piece| piece.corners.clone())
                .collect();
            Some(Operation::CreateSketch {
                plane: WorkPlane::from_face(&corners, normal, up),
                on: Some(FaceAnchor { face, up }),
            })
        }
        Move::Rectangle(low, high) => Some(Operation::AddRectangle {
            sketch: sketch?,
            corner: PointRef::New(DVec2::from(low)),
            opposite: PointRef::New(DVec2::from(high)),
            construction: false,
        }),
        Move::Circle(center, radius) => Some(Operation::AddCircle {
            sketch: sketch?,
            center: PointRef::New(DVec2::from(center)),
            radius,
            rim: Vec::new(),
            construction: false,
        }),
        Move::Raise(at, depth, cut) => {
            let areas = document.areas_at(sketch?, &[DVec2::from(at)]);
            (!areas.is_empty()).then(|| Operation::Extrude {
                sketch: sketch.unwrap_or_default(),
                areas,
                distance: depth.into(),
                mode: mode(cut),
            })
        }
        Move::Turn(at, about_v, degrees, cut) => {
            let areas = document.areas_at(sketch?, &[DVec2::from(at)]);
            (!areas.is_empty()).then(|| Operation::Revolve {
                sketch: sketch.unwrap_or_default(),
                areas,
                axis: RevolutionAxis::Sketch(if about_v {
                    SketchAxis::V
                } else {
                    SketchAxis::U
                }),
                angle: degrees.into(),
                mode: mode(cut),
            })
        }
    }
}

fn triangles(document: &PartDocument) -> Vec<Triangle> {
    document.body().triangles()
}

fn fresh() -> PartDocument {
    PartDocument::new("random", "2026-09-30T09:00:00Z".parse().expect("a date"))
}

/// Plays the gestures one by one, and after each one takes it back and does
/// it again: the part after the undo must be the part before the gesture, and
/// the part after the redo the part after it. Every part on the way must be
/// sound, and the whole history, replayed twice, the same part twice.
fn undoes(gestures: &[Move]) -> Result<(), Flaw> {
    let mut document = fresh();
    let mut before = triangles(&document);
    for gesture in gestures {
        let Some(operation) = operation(&document, gesture) else {
            continue;
        };
        let applied = document.history.applied();
        document.apply(operation);
        if document.history.applied() == applied {
            continue;
        }
        let after = triangles(&document);
        closed(&after)?;
        uncrossed(&after)?;

        document.undo();
        if let Some(at) = first_difference(&before, &triangles(&document)) {
            return Err(Flaw::NotUndone { at });
        }
        document.redo();
        if let Some(at) = first_difference(&after, &triangles(&document)) {
            return Err(Flaw::NotUndone { at });
        }
        before = after;
    }
    let (mut first, mut second) = (fresh(), fresh());
    first.history = document.history.clone();
    second.history = document.history.clone();
    first.rewind_to(first.history.applied());
    second.rewind_to(second.history.applied());
    repeatable(&triangles(&first), &triangles(&second))
}

/// A part of a few drawings, each with a few shapes raised or cut.
fn drawn(seed: u64) -> Vec<Move> {
    let mut random = Random::seeded(seed);
    let mut gestures = Vec::new();
    let place = |random: &mut Random| {
        [
            random.on_lattice(0.0, 20.0, 1.0),
            random.on_lattice(0.0, 20.0, 1.0),
        ]
    };
    for drawing in 0..1 + random.below(3) {
        gestures.push(if drawing > 0 && random.chance(0.6) {
            Move::SketchOn(random.below(12))
        } else {
            Move::Sketch(random.below(3))
        });
        let mut shapes: Vec<[f64; 2]> = Vec::new();
        for _ in 0..1 + random.below(3) {
            let low = place(&mut random);
            if random.chance(0.6) {
                let size = [
                    random.on_lattice(1.0, 12.0, 1.0),
                    random.on_lattice(1.0, 12.0, 1.0),
                ];
                gestures.push(Move::Rectangle(low, [low[0] + size[0], low[1] + size[1]]));
                shapes.push([low[0] + size[0] / 2.0, low[1] + size[1] / 2.0]);
            } else {
                gestures.push(Move::Circle(low, random.on_lattice(1.0, 8.0, 0.5)));
                shapes.push(low);
            }
        }
        for _ in 0..1 + random.below(2) {
            let at = *random.pick(&shapes);
            let cut = random.chance(0.5);
            if random.chance(0.15) {
                let degrees = *random.pick(&[90.0, 180.0, 360.0, -45.0]);
                gestures.push(Move::Turn(at, random.chance(0.5), degrees, cut));
            } else {
                let depth =
                    random.on_lattice(1.0, 10.0, 1.0) * if random.chance(0.2) { -1.0 } else { 1.0 };
                gestures.push(Move::Raise(at, depth, cut));
            }
        }
    }
    gestures
}

fn fewer(gestures: &[Move]) -> Vec<Vec<Move>> {
    (0..gestures.len())
        .rev()
        .map(|index| {
            let mut fewer = gestures.to_vec();
            fewer.remove(index);
            fewer
        })
        .collect()
}

#[test]
fn a_rectangle_raised_then_undone_gives_back_the_empty_part() {
    assert_eq!(
        undoes(&[
            Move::Sketch(0),
            Move::Rectangle([0.0, 0.0], [10.0, 5.0]),
            Move::Raise([5.0, 2.0], 4.0, false),
        ]),
        Ok(())
    );
}

#[test]
fn a_part_whose_undo_changed_a_bit_is_caught() {
    let mut document = fresh();
    for gesture in [
        Move::Sketch(0),
        Move::Rectangle([0.0, 0.0], [10.0, 5.0]),
        Move::Raise([5.0, 2.0], 4.0, false),
    ] {
        let operation = operation(&document, &gesture).expect("something to act on");
        document.apply(operation);
    }
    let mut nudged = triangles(&document);
    nudged[3][2].z = f64::from_bits(nudged[3][2].z.to_bits() ^ 1);

    assert_eq!(first_difference(&triangles(&document), &nudged), Some(3));
}

#[test]
fn the_same_seed_draws_the_same_part() {
    for seed in 0..100 {
        assert_eq!(drawn(seed), drawn(seed));
    }
}

fn from_the_environment(name: &str) -> Option<u64> {
    std::env::var(name).ok()?.parse().ok()
}

#[test]
#[ignore = "a campaign, run by hand: see the head of this file"]
fn a_campaign_of_random_parts_undoes_every_gesture() {
    let seconds = from_the_environment("CAO_FUZZ_SECONDS").unwrap_or(60);
    let first = from_the_environment("CAO_FUZZ_SEED").unwrap_or_else(|| {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_or(1, |since| since.as_secs())
    });
    let patience = Duration::from_secs(from_the_environment("CAO_FUZZ_PATIENCE").unwrap_or(60));
    let deadline = Instant::now() + Duration::from_secs(seconds);
    println!("campaign from seed {first}, for {seconds} s");

    let check: Check<Vec<Move>> = Arc::new(|gestures: &Vec<Move>| undoes(gestures));
    let quiet = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let report = campaign(
        first..,
        drawn,
        check,
        |gestures: &Vec<Move>| fewer(gestures),
        patience,
        || Instant::now() < deadline,
    );
    std::panic::set_hook(quiet);

    print_report(&report);
    assert!(
        report.findings.is_empty(),
        "{} parts of {} broke a rule",
        report.broken.iter().map(|(_, count)| count).sum::<usize>(),
        report.tried
    );
}

fn print_report(report: &Report<Vec<Move>>) {
    println!("{} parts tried", report.tried);
    for (rule, count) in &report.broken {
        println!("  {rule:?}: broken by {count}");
    }
    for finding in &report.findings {
        println!(
            "\n── {:?}, seed {} ──\nas drawn: {:?}\nshrunk:   {:?}\n\n#[test]\nfn seed_{}_undoes_every_gesture() {{\n    assert_eq!(undoes({}), Ok(()));\n}}",
            finding.flaw.rule(),
            finding.seed,
            finding.flaw,
            finding.shrunk_flaw,
            finding.seed,
            Moves(&finding.shrunk).to_string().replace('\n', "\n    "),
        );
    }
}
