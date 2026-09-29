//! A selection moved where the drawing cannot follow is put back, rather than
//! leaving the drawing's rules untrue.
//!
//! Closes #472.
//! - the drawing above, the right side moved down: the drawing is as it was,
//!   every right angle still true —
//!   `a_right_side_moved_where_the_sizes_forbid_leaves_the_rectangle_as_it_was`
//! - a selection the drawing can follow still moves as a block —
//!   `a_selection_the_drawing_can_follow_still_moves_as_a_block`
//! - after the gesture is refused, the ends of any free trait still follow the
//!   hand — `a_free_trait_still_follows_the_hand_after_a_refused_move`

use cao_part::{Operation, PartState, PointRef};
use cao_sketch::{Constraint, DimensionTarget, Element, PointId, SegmentId, Sketch, WorkPlane};
use glam::DVec2;

const BOTTOM_RIGHT: PointId = PointId(1);
const TOP_RIGHT: PointId = PointId(2);

/// A rectangle 150 wide and 200 high from the origin, three right angles, its
/// sizes typed and its top right corner fixed; and a free trait beside it.
fn the_drawing() -> PartState {
    let mut state = PartState::default();
    state.apply(&Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    let corners = [
        PointRef::New(DVec2::new(150.0, 0.0)),
        PointRef::New(DVec2::new(150.0, 200.0)),
        PointRef::New(DVec2::new(0.0, 200.0)),
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
    for (side, value) in [(SegmentId(1), 200.0), (SegmentId(0), 150.0)] {
        state.apply(&Operation::SetDimension {
            sketch: 0,
            target: DimensionTarget::Length(side),
            value: value.into(),
            placement: None,
        });
    }
    state.apply(&Operation::Constrain {
        sketch: 0,
        constraint: Constraint::Fixed {
            element: Element::Point(TOP_RIGHT),
        },
    });
    state.apply(&Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::new(250.0, 300.0)),
        end: PointRef::New(DVec2::new(330.0, 240.0)),
        construction: true,
    });
    state
}

fn squareness(sketch: &Sketch) -> f64 {
    let direction = |side: usize| {
        let (start, end) = sketch.endpoints(SegmentId(side));
        (end - start).normalize()
    };
    (0..4)
        .map(|side| direction(side).dot(direction((side + 1) % 4)).abs())
        .fold(0.0, f64::max)
}

fn move_the_right_side_down(state: &mut PartState) {
    state.apply(&Operation::MoveMany {
        sketch: 0,
        points: vec![BOTTOM_RIGHT, TOP_RIGHT],
        by: DVec2::new(0.0, -80.0),
    });
}

#[test]
fn a_right_side_moved_where_the_sizes_forbid_leaves_the_rectangle_as_it_was() {
    let mut state = the_drawing();
    let before = state.sketches[0].points().to_vec();

    move_the_right_side_down(&mut state);

    let sketch = &state.sketches[0];
    assert!(
        squareness(sketch) < 1e-6,
        "every right angle still true, the worst cosine being {}",
        squareness(sketch)
    );
    assert!(
        sketch
            .points()
            .iter()
            .zip(&before)
            .all(|(now, was)| now.distance(*was) < 1e-9),
        "the drawing is as it was: {:?}",
        sketch.points()
    );
}

#[test]
fn a_selection_the_drawing_can_follow_still_moves_as_a_block() {
    let mut state = the_drawing();
    let start = PointId(4);
    let end = PointId(5);
    let (was_start, was_end) = (state.sketches[0].point(start), state.sketches[0].point(end));

    state.apply(&Operation::MoveMany {
        sketch: 0,
        points: vec![start, end],
        by: DVec2::new(20.0, 10.0),
    });

    let sketch = &state.sketches[0];
    assert!(
        sketch
            .point(start)
            .distance(was_start + DVec2::new(20.0, 10.0))
            < 1e-9
    );
    assert!(sketch.point(end).distance(was_end + DVec2::new(20.0, 10.0)) < 1e-9);
}

#[test]
fn a_free_trait_still_follows_the_hand_after_a_refused_move() {
    let mut state = the_drawing();
    move_the_right_side_down(&mut state);
    let end = PointId(5);
    let to = state.sketches[0].point(end) + DVec2::new(30.0, 20.0);

    state.apply(&Operation::MovePoint {
        sketch: 0,
        point: end,
        position: to,
        merged_into: None,
        on: Vec::new(),
        let_go: false,
    });

    assert!(
        state.sketches[0].point(end).distance(to) < 1e-6,
        "the end is where the hand put it, not at {}",
        state.sketches[0].point(end)
    );
}
