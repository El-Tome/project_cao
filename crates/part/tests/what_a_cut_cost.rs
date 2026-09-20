//! What a part hands back when a cut takes rules and values with it.
//!
//! Trimming drops what cannot follow either piece. The drawing used to lose
//! them silently, and the user found out when the shape started moving.

use cao_part::history::{Operation, PointRef};
use cao_part::{Outcome, PartState};
use cao_sketch::{Constraint, DimensionTarget, PointId, SegmentId, WorkPlane};
use glam::DVec2;

const CUT: SegmentId = SegmentId(0);
const ALONGSIDE: SegmentId = SegmentId(1);

fn a_trait_spoken_of_twice() -> PartState {
    let mut state = PartState::default();
    state.apply(&Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    for (start, end) in [((0.0, 1.0), (10.0, 1.0)), ((0.0, 5.0), (10.0, 5.0))] {
        state.apply(&Operation::AddSegment {
            sketch: 0,
            start: PointRef::New(DVec2::new(start.0, start.1)),
            end: PointRef::New(DVec2::new(end.0, end.1)),
            construction: false,
        });
    }
    state.apply(&Operation::Constrain {
        sketch: 0,
        constraint: Constraint::Parallel {
            first: CUT,
            second: ALONGSIDE,
        },
    });
    state.apply(&Operation::Constrain {
        sketch: 0,
        constraint: Constraint::Equal {
            first: CUT,
            second: ALONGSIDE,
        },
    });
    state.apply(&Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Length(CUT),
        value: 10.0,
        placement: None,
    });
    for at in [(4.0, 1.0), (6.0, 1.0)] {
        state.apply(&Operation::AddPoint {
            sketch: 0,
            position: DVec2::new(at.0, at.1),
            on: Vec::new(),
        });
    }
    state
}

#[test]
fn a_cut_says_how_many_rules_and_values_it_could_not_carry_over() {
    let mut state = a_trait_spoken_of_twice();

    let outcome = state.apply(&Operation::Trim {
        sketch: 0,
        segment: CUT,
        from: PointId(5),
        to: PointId(6),
    });

    assert_eq!(
        outcome,
        Some(Outcome::Cut {
            rules: 1,
            values: 1
        }),
        "the equal lengths and the typed length went; the parallel followed",
    );
}

#[test]
fn a_cut_that_cannot_be_made_says_nothing() {
    let mut state = a_trait_spoken_of_twice();

    let outcome = state.apply(&Operation::Trim {
        sketch: 0,
        segment: CUT,
        from: PointId(99),
        to: PointId(6),
    });

    assert_eq!(outcome, None);
}
