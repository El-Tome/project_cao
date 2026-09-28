//! A rule laid between two free traits leaves the first one clicked where it
//! is (#451). The part is rebuilt from its history every time it is opened, or
//! a step undone: the order of the clicks has to come back with it, or the
//! same rule lands elsewhere the second time.

use cao_part::PartState;
use cao_part::history::{History, Operation, PointRef};
use cao_sketch::{CircleId, Constraint, LaidFrom, SegmentId, WorkPlane};
use glam::DVec2;

/// Two free traits, and a square laid from the one drawn second — the order
/// that putting a pair in the order of its ids used to turn round.
fn squared_from_the_second_trait_drawn() -> History {
    let mut history = History::default();
    history.push(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    for (start, end) in [
        (DVec2::new(10.0, 10.0), DVec2::new(110.0, 30.0)),
        (DVec2::new(10.0, 90.0), DVec2::new(60.0, 180.0)),
    ] {
        history.push(Operation::AddSegment {
            sketch: 0,
            start: PointRef::New(start),
            end: PointRef::New(end),
            construction: false,
        });
    }
    history.push(Operation::Constrain {
        sketch: 0,
        constraint: Constraint::Perpendicular {
            first: SegmentId(1),
            second: SegmentId(0),
        },
    });
    history
}

const CLOSE: f64 = 1e-9;

/// How far a trait's ends are from where they were drawn, the worse of the two.
fn moved(state: &PartState, segment: usize, drawn: [DVec2; 2]) -> f64 {
    let now = ends(state, segment);
    now[0].distance(drawn[0]).max(now[1].distance(drawn[1]))
}

fn ends(state: &PartState, segment: usize) -> [DVec2; 2] {
    let sketch = &state.sketches[0];
    let drawn = sketch.segments()[segment];
    [sketch.point(drawn.start), sketch.point(drawn.end)]
}

#[test]
fn a_square_laid_from_the_second_trait_drawn_turns_the_first() {
    let state = PartState::rebuild(&squared_from_the_second_trait_drawn());

    let first = [DVec2::new(10.0, 90.0), DVec2::new(60.0, 180.0)];
    let second = [DVec2::new(10.0, 10.0), DVec2::new(110.0, 30.0)];
    assert!(
        moved(&state, 1, first) < CLOSE,
        "the trait clicked first moved"
    );
    assert!(
        moved(&state, 0, second) > CLOSE,
        "the trait clicked second did not turn"
    );
}

/// A circle and a trait lying apart, made tangent from the trait: the circle
/// is what comes to touch it. The one rule whose order the pair cannot carry.
fn touched_from_the_trait() -> History {
    let mut history = History::default();
    history.push(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    history.push(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::new(10.0, 10.0)),
        end: PointRef::New(DVec2::new(210.0, 10.0)),
        construction: false,
    });
    history.push(Operation::AddCircle {
        sketch: 0,
        center: PointRef::New(DVec2::new(100.0, 90.0)),
        radius: 30.0,
        rim: Vec::new(),
        construction: false,
    });
    history.push(Operation::Constrain {
        sketch: 0,
        constraint: Constraint::Tangent {
            circle: CircleId(0),
            segment: SegmentId(0),
            at: None,
            from: LaidFrom::Trait,
        },
    });
    history
}

/// What the part file keeps of a history is its operations; read back, they
/// are replayed from scratch.
fn saved_and_read_back(history: &History) -> History {
    let written =
        serde_json::to_string(history.applied_operations()).expect("the operations write out");
    let read: Vec<Operation> = serde_json::from_str(&written).expect("and read back");
    let mut reopened = History::default();
    for operation in read {
        reopened.push(operation);
    }
    reopened
}

#[test]
fn a_part_saved_and_read_back_lands_its_rules_in_the_same_place() {
    for history in [
        squared_from_the_second_trait_drawn(),
        touched_from_the_trait(),
    ] {
        let live = PartState::rebuild(&history);
        let reopened = PartState::rebuild(&saved_and_read_back(&history));

        assert_eq!(live.sketches[0].points(), reopened.sketches[0].points());
        let radii = |state: &PartState| -> Vec<f64> {
            state.sketches[0]
                .circles()
                .iter()
                .map(|circle| circle.radius)
                .collect()
        };
        assert_eq!(radii(&live), radii(&reopened));
    }
}

#[test]
fn a_tangency_laid_from_the_trait_leaves_the_trait_where_it_was() {
    let state = PartState::rebuild(&touched_from_the_trait());

    let drawn = [DVec2::new(10.0, 10.0), DVec2::new(210.0, 10.0)];
    assert!(
        moved(&state, 0, drawn) < CLOSE,
        "the trait clicked first moved"
    );
}
