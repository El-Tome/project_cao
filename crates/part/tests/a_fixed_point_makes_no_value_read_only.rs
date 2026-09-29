//! A value that only a fixed point decides is typed and drives: Fixed keeps a
//! point where it is, it does not say the drawing is decided.
//!
//! Closes #468.
//! - the rectangle, one corner on the origin, the height typed, the opposite
//!   corner fixed: the width is typed and drives, and the fixed corner gives —
//!   `a_width_only_a_fixed_corner_decides_is_typed_and_the_corner_gives`
//! - a width the values and rules already decide is still a read-only
//!   reference — `a_width_the_values_already_decide_is_still_read_only`

use cao_part::{DimensionOutcome, Operation, Outcome, PartState, PointRef};
use cao_sketch::{
    Constraint, DimensionTarget, Element, LengthOutcome, PointId, SegmentId, Sketch, WorkPlane,
};
use glam::DVec2;

const BOTTOM: SegmentId = SegmentId(0);
const RIGHT: SegmentId = SegmentId(1);
const TOP: SegmentId = SegmentId(2);
const TOP_RIGHT: PointId = PointId(2);

/// A rectangle 75 wide and 50 high, its bottom left corner on the origin, its
/// corners squared and its height typed.
fn rectangle() -> PartState {
    let mut state = PartState::default();
    state.apply(&Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    let corners = [
        PointRef::New(DVec2::new(75.0, 0.0)),
        PointRef::New(DVec2::new(75.0, 50.0)),
        PointRef::New(DVec2::new(0.0, 50.0)),
    ];
    let mut from = PointRef::Existing(Sketch::ORIGIN);
    for (index, corner) in corners.into_iter().enumerate() {
        state.apply(&Operation::AddSegment {
            sketch: 0,
            start: from,
            end: corner,
            construction: false,
        });
        from = PointRef::Existing(PointId(index + 1));
    }
    state.apply(&Operation::AddSegment {
        sketch: 0,
        start: from,
        end: PointRef::Existing(Sketch::ORIGIN),
        construction: false,
    });
    for pair in 0..3 {
        state.apply(&Operation::Constrain {
            sketch: 0,
            constraint: Constraint::Perpendicular {
                first: SegmentId(pair),
                second: SegmentId(pair + 1),
            },
        });
    }
    type_length(&mut state, RIGHT, 50.0);
    state
}

fn type_length(state: &mut PartState, side: SegmentId, value: f64) -> Option<Outcome> {
    state.apply(&Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Length(side),
        value: value.into(),
        placement: None,
    })
}

fn fix_the_top_right_corner(state: &mut PartState) {
    state.apply(&Operation::Constrain {
        sketch: 0,
        constraint: Constraint::Fixed {
            element: Element::Point(TOP_RIGHT),
        },
    });
}

#[test]
fn a_width_only_a_fixed_corner_decides_is_typed_and_the_corner_gives() {
    let mut state = rectangle();
    fix_the_top_right_corner(&mut state);

    let outcome = type_length(&mut state, TOP, 100.0);

    assert_eq!(
        outcome,
        Some(Outcome::Dimension(DimensionOutcome::Geometry(
            LengthOutcome::Exact
        ))),
        "the width drives, it is not laid read-only"
    );
    let sketch = &state.sketches[0];
    assert!(
        sketch
            .dimension_of(DimensionTarget::Length(TOP))
            .is_some_and(|width| !width.driven),
        "the width is kept as a value the user typed"
    );
    assert!(
        (sketch.segment_length(TOP) - 100.0).abs() < 1e-3,
        "the rectangle is 100 wide, not {}",
        sketch.segment_length(TOP)
    );
    assert!(
        sketch.point(TOP_RIGHT).distance(DVec2::new(100.0, 50.0)) < 1e-3,
        "the fixed corner gave, to {}",
        sketch.point(TOP_RIGHT)
    );
}

#[test]
fn a_width_the_values_already_decide_is_still_read_only() {
    let mut state = rectangle();
    type_length(&mut state, BOTTOM, 75.0);
    fix_the_top_right_corner(&mut state);

    let outcome = type_length(&mut state, TOP, 100.0);

    assert_eq!(
        outcome,
        Some(Outcome::Dimension(DimensionOutcome::Reference)),
        "the bottom's value already decides the top"
    );
}
