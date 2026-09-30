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
//!   `a_campaign_of_random_parts_undoes_every_gesture`; caught out by a part
//!   broken on purpose — `a_part_whose_undo_leaves_the_matter_standing_is_caught`
//! - a broken part prints as Rust that pastes into a named test —
//!   `a_part_prints_as_the_rust_that_builds_it`
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
    Sketch(Origin),
    /// A drawing on a flat face of the part, the rank-th one counting round.
    SketchOnFace(usize),
    Rectangle {
        low: [f64; 2],
        high: [f64; 2],
    },
    Circle {
        center: [f64; 2],
        radius: f64,
    },
    /// The area under a place, raised by a depth or cut into the part.
    Raise {
        at: [f64; 2],
        depth: f64,
        cut: bool,
    },
    /// The area under a place, turned about one of the drawing's axes.
    Turn {
        at: [f64; 2],
        about: SketchAxis,
        degrees: f64,
        cut: bool,
    },
}

/// The three planes of the origin a drawing can be opened on.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Origin {
    Xy,
    Xz,
    Yz,
}

impl Display for Move {
    fn fmt(&self, out: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Move::Sketch(origin) => write!(out, "Move::Sketch(Origin::{origin:?})"),
            Move::SketchOnFace(rank) => write!(out, "Move::SketchOnFace({rank})"),
            Move::Rectangle { low, high } => {
                write!(out, "Move::Rectangle {{ low: {low:?}, high: {high:?} }}")
            }
            Move::Circle { center, radius } => {
                write!(
                    out,
                    "Move::Circle {{ center: {center:?}, radius: {radius:?} }}"
                )
            }
            Move::Raise { at, depth, cut } => {
                write!(
                    out,
                    "Move::Raise {{ at: {at:?}, depth: {depth:?}, cut: {cut} }}"
                )
            }
            Move::Turn {
                at,
                about,
                degrees,
                cut,
            } => write!(
                out,
                "Move::Turn {{ at: {at:?}, about: SketchAxis::{about:?}, degrees: {degrees:?}, cut: {cut} }}"
            ),
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
        Move::Sketch(origin) => Some(Operation::CreateSketch {
            plane: match origin {
                Origin::Xy => WorkPlane::XY,
                Origin::Xz => WorkPlane::XZ,
                Origin::Yz => WorkPlane::YZ,
            },
            on: None,
        }),
        Move::SketchOnFace(rank) => {
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
        Move::Rectangle { low, high } => Some(Operation::AddRectangle {
            sketch: sketch?,
            corner: PointRef::New(DVec2::from(low)),
            opposite: PointRef::New(DVec2::from(high)),
            construction: false,
        }),
        Move::Circle { center, radius } => Some(Operation::AddCircle {
            sketch: sketch?,
            center: PointRef::New(DVec2::from(center)),
            radius,
            rim: Vec::new(),
            construction: false,
        }),
        Move::Raise { at, depth, cut } => {
            let areas = document.areas_at(sketch?, &[DVec2::from(at)]);
            (!areas.is_empty()).then(|| Operation::Extrude {
                sketch: sketch.unwrap_or_default(),
                areas,
                distance: depth.into(),
                mode: mode(cut),
            })
        }
        Move::Turn {
            at,
            about,
            degrees,
            cut,
        } => {
            let areas = document.areas_at(sketch?, &[DVec2::from(at)]);
            (!areas.is_empty()).then(|| Operation::Revolve {
                sketch: sketch.unwrap_or_default(),
                areas,
                axis: RevolutionAxis::Sketch(about),
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

/// What the undo rule needs of a part: to be shown a gesture, and to take it
/// back and do it again.
trait Undoable {
    fn document(&self) -> &PartDocument;
    fn apply(&mut self, operation: Operation);
    fn undo(&mut self);
    fn redo(&mut self);
}

impl Undoable for PartDocument {
    fn document(&self) -> &PartDocument {
        self
    }

    fn apply(&mut self, operation: Operation) {
        PartDocument::apply(self, operation);
    }

    fn undo(&mut self) {
        PartDocument::undo(self);
    }

    fn redo(&mut self) {
        PartDocument::redo(self);
    }
}

/// Plays the gestures one by one, and after each one takes it back and does
/// it again: the part after the undo must be the part before the gesture, and
/// the part after the redo the part after it. Every part on the way must be
/// sound too — judged after the undo, so that a kernel's own flaw does not
/// hide one — and the whole history, replayed twice, the same part twice.
fn undoes(gestures: &[Move]) -> Result<(), Flaw> {
    undoes_on(fresh(), gestures)
}

fn undoes_on(mut part: impl Undoable, gestures: &[Move]) -> Result<(), Flaw> {
    let mut before = triangles(part.document());
    for gesture in gestures {
        let Some(operation) = operation(part.document(), gesture) else {
            continue;
        };
        let applied = part.document().history.applied();
        part.apply(operation);
        if part.document().history.applied() == applied {
            continue;
        }
        let after = triangles(part.document());
        part.undo();
        if let Some(at) = first_difference(&before, &triangles(part.document())) {
            return Err(Flaw::NotUndone { at });
        }
        part.redo();
        if let Some(at) = first_difference(&after, &triangles(part.document())) {
            return Err(Flaw::NotUndone { at });
        }
        closed(&after)?;
        uncrossed(&after)?;
        before = after;
    }
    let history = &part.document().history;
    let (mut first, mut second) = (fresh(), fresh());
    first.history = history.clone();
    second.history = history.clone();
    first.rewind_to(history.applied());
    second.rewind_to(history.applied());
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
            Move::SketchOnFace(random.below(12))
        } else {
            Move::Sketch(*random.pick(&[Origin::Xy, Origin::Xz, Origin::Yz]))
        });
        let mut shapes: Vec<[f64; 2]> = Vec::new();
        for _ in 0..1 + random.below(3) {
            let low = place(&mut random);
            if random.chance(0.6) {
                let size = [
                    random.on_lattice(1.0, 12.0, 1.0),
                    random.on_lattice(1.0, 12.0, 1.0),
                ];
                gestures.push(Move::Rectangle {
                    low,
                    high: [low[0] + size[0], low[1] + size[1]],
                });
                shapes.push([low[0] + size[0] / 2.0, low[1] + size[1] / 2.0]);
            } else {
                gestures.push(Move::Circle {
                    center: low,
                    radius: random.on_lattice(1.0, 8.0, 0.5),
                });
                shapes.push(low);
            }
        }
        for _ in 0..1 + random.below(2) {
            let at = *random.pick(&shapes);
            let cut = random.chance(0.5);
            if random.chance(0.15) {
                let degrees = *random.pick(&[90.0, 180.0, 360.0, -45.0]);
                let about = *random.pick(&[SketchAxis::U, SketchAxis::V]);
                gestures.push(Move::Turn {
                    at,
                    about,
                    degrees,
                    cut,
                });
            } else {
                let depth =
                    random.on_lattice(1.0, 10.0, 1.0) * if random.chance(0.2) { -1.0 } else { 1.0 };
                gestures.push(Move::Raise { at, depth, cut });
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

fn a_block_raised() -> Vec<Move> {
    vec![
        Move::Sketch(Origin::Xy),
        Move::Rectangle {
            low: [0.0, 0.0],
            high: [10.0, 5.0],
        },
        Move::Raise {
            at: [5.0, 2.0],
            depth: 4.0,
            cut: false,
        },
    ]
}

#[test]
fn a_rectangle_raised_then_undone_gives_back_the_empty_part() {
    assert_eq!(undoes(&a_block_raised()), Ok(()));
}

/// A part whose undo takes the history back and leaves the matter standing.
struct Forgetful(PartDocument);

impl Undoable for Forgetful {
    fn document(&self) -> &PartDocument {
        &self.0
    }

    fn apply(&mut self, operation: Operation) {
        self.0.apply(operation);
    }

    fn undo(&mut self) {
        self.0.history.undo();
    }

    fn redo(&mut self) {
        self.0.history.redo();
    }
}

#[test]
fn a_part_whose_undo_leaves_the_matter_standing_is_caught() {
    assert!(matches!(
        undoes_on(Forgetful(fresh()), &a_block_raised()),
        Err(Flaw::NotUndone { at: 0 })
    ));
}

#[test]
fn a_part_prints_as_the_rust_that_builds_it() {
    let gestures = vec![
        Move::Sketch(Origin::Xz),
        Move::Circle {
            center: [3.0, 4.5],
            radius: 2.5,
        },
        Move::Turn {
            at: [3.0, 4.5],
            about: SketchAxis::V,
            degrees: -45.0,
            cut: false,
        },
        Move::SketchOnFace(3),
        Move::Rectangle {
            low: [0.0, 0.0],
            high: [1e-7, 5.0],
        },
        Move::Raise {
            at: [0.5, 2.0],
            depth: -4.0,
            cut: true,
        },
    ];
    assert_eq!(
        Moves(&gestures).to_string(),
        "&[
    Move::Sketch(Origin::Xz),
    Move::Circle { center: [3.0, 4.5], radius: 2.5 },
    Move::Turn { at: [3.0, 4.5], about: SketchAxis::V, degrees: -45.0, cut: false },
    Move::SketchOnFace(3),
    Move::Rectangle { low: [0.0, 0.0], high: [1e-7, 5.0] },
    Move::Raise { at: [0.5, 2.0], depth: -4.0, cut: true },
]"
    );
    let pasted: &[Move] = &[
        Move::Sketch(Origin::Xz),
        Move::Circle {
            center: [3.0, 4.5],
            radius: 2.5,
        },
        Move::Turn {
            at: [3.0, 4.5],
            about: SketchAxis::V,
            degrees: -45.0,
            cut: false,
        },
        Move::SketchOnFace(3),
        Move::Rectangle {
            low: [0.0, 0.0],
            high: [1e-7, 5.0],
        },
        Move::Raise {
            at: [0.5, 2.0],
            depth: -4.0,
            cut: true,
        },
    ];
    assert_eq!(pasted, gestures.as_slice());
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
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_or(1, |since| since.as_nanos() as u64);
        Random::seeded(now).number() >> 16
    });
    let patience = Duration::from_secs(from_the_environment("CAO_FUZZ_PATIENCE").unwrap_or(60));
    let deadline = Instant::now() + Duration::from_secs(seconds);
    println!("campaign from seed {first}, for {seconds} s");

    let check: Check<Vec<Move>> = Arc::new(|gestures: &Vec<Move>| undoes(gestures));
    let quiet = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let report = campaign(
        first..,
        |seed| {
            eprint!("\rseed {seed} ");
            drawn(seed)
        },
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
