//! A shape pulled by hand is recorded as the drawing the gesture showed, and
//! comes back the same from the history.
//!
//! Closes #422.
//! - 21: a whole shape selected and dragged moves unbent and unturned —
//!   `a_whole_rectangle_dragged_moves_unbent_and_unturned`
//! - 21, the curve carried by its centre — no test: here;
//!   `a_circle_dragged_by_its_centre_is_carried_at_its_size` in
//!   cao_sketch's settling/tests.rs holds it
//! - 22: replaying the history rebuilds the same drawing, how far a side
//!   travelled and the angle a shape turned recorded with the step —
//!   `a_side_moved_and_a_shape_turned_are_rebuilt_as_they_were_left`, a pivot
//!   pulled onto the grid included —
//!   `a_trait_pivoted_onto_a_grid_point_is_rebuilt_on_it`; one undo takes the
//!   whole gesture back — `one_undo_takes_back_a_side_moved_and_a_shape_turned`;
//!   the drawing the drag showed is the one recorded —
//!   `a_corner_dragged_is_rebuilt_where_the_drag_showed_it`; and compacting
//!   the history keeps the shape both left —
//!   `compacting_keeps_the_shape_a_side_moved_and_a_turn_left`

use cao_part::{Operation, PartDocument, PointRef};
use cao_sketch::{Constraint, DimensionTarget, PointId, SegmentId, Sketch, WorkPlane};
use chrono::{DateTime, Utc};
use glam::DVec2;

const SETTLED: f64 = 1e-4;

fn at(text: &str) -> DateTime<Utc> {
    text.parse().expect("a date")
}

/// A rectangle as the tool lays it: four traits and three right angles, 100
/// wide and 50 high. Its corners are points 1 to 4 in the order the tool lays
/// them — the corner pressed at (20, 20), the opposite one at (120, 70), then
/// the two between — and the top is trait 2.
fn a_rectangle() -> PartDocument {
    let mut document = PartDocument::new("Test", at("2026-01-02T09:00:00Z"));
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::new(20.0, 20.0)),
        opposite: PointRef::New(DVec2::new(120.0, 70.0)),
        construction: false,
    });
    for corner in 0..3 {
        document.apply(Operation::Constrain {
            sketch: 0,
            constraint: Constraint::Perpendicular {
                first: SegmentId(corner),
                second: SegmentId(corner + 1),
            },
        });
    }
    document
}

fn corners(document: &PartDocument) -> Vec<DVec2> {
    let sketch = &document.sketches()[0];
    (1..=4).map(|rank| sketch.point(PointId(rank))).collect()
}

fn rebuilt(document: &PartDocument) -> Vec<DVec2> {
    let mut again = document.clone();
    again.undo();
    again.redo();
    corners(&again)
}

fn assert_same(found: &[DVec2], wanted: &[DVec2]) {
    for (rank, (one, other)) in found.iter().zip(wanted).enumerate() {
        assert!(
            one.distance(*other) < 1e-9,
            "corner {rank}: {one} against {other}"
        );
    }
}

#[test]
fn a_whole_rectangle_dragged_moves_unbent_and_unturned() {
    let mut document = a_rectangle();
    let before = corners(&document);

    document.apply(Operation::MoveMany {
        sketch: 0,
        points: (1..=4).map(PointId).collect(),
        by: DVec2::new(-30.0, 12.0),
    });

    for (rank, now) in corners(&document).iter().enumerate() {
        let wanted = before[rank] + DVec2::new(-30.0, 12.0);
        assert!(
            now.distance(wanted) < SETTLED,
            "corner {rank} at {now}, not {wanted}"
        );
    }
}

#[test]
fn a_side_moved_and_a_shape_turned_are_rebuilt_as_they_were_left() {
    let mut document = a_rectangle();

    document.apply(Operation::MoveSegment {
        sketch: 0,
        segment: SegmentId(2),
        by: DVec2::new(0.0, 15.0),
    });
    let moved = corners(&document);
    assert!(
        moved[1].distance(DVec2::new(120.0, 85.0)) < SETTLED,
        "{}",
        moved[1]
    );
    assert!(
        moved[0].distance(DVec2::new(20.0, 20.0)) < SETTLED,
        "{}",
        moved[0]
    );

    document.apply(Operation::TurnShape {
        sketch: 0,
        points: (1..=4).map(PointId).collect(),
        about: DVec2::new(70.0, 52.5),
        angle: 0.3,
    });
    let turned = corners(&document);
    let wanted =
        DVec2::new(70.0, 52.5) + DVec2::from_angle(0.3).rotate(moved[0] - DVec2::new(70.0, 52.5));
    assert!(turned[0].distance(wanted) < SETTLED, "{}", turned[0]);

    assert_same(&rebuilt(&document), &turned);
}

#[test]
fn one_undo_takes_back_a_side_moved_and_a_shape_turned() {
    let mut document = a_rectangle();
    let before = corners(&document);

    document.apply(Operation::MoveSegment {
        sketch: 0,
        segment: SegmentId(2),
        by: DVec2::new(0.0, 15.0),
    });
    let moved = corners(&document);
    document.apply(Operation::TurnShape {
        sketch: 0,
        points: (1..=4).map(PointId).collect(),
        about: DVec2::new(70.0, 52.5),
        angle: 0.3,
    });

    document.undo();
    assert_same(&corners(&document), &moved);
    document.undo();
    assert_same(&corners(&document), &before);
}

#[test]
fn a_trait_pivoted_onto_a_grid_point_is_rebuilt_on_it() {
    let mut document = PartDocument::new("Test", at("2026-01-02T09:00:00Z"));
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::new(10.0, 10.0)),
        end: PointRef::New(DVec2::new(110.0, 10.0)),
        construction: false,
    });
    document.apply(Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Length(SegmentId(0)),
        value: 100.0.into(),
        placement: None,
    });
    let sketch = &document.sketches()[0];
    let pull = sketch.pull(PointId(2), document.scale());
    let grid = cao_sketch::SnapSettings {
        point_reach: 1.0,
        curve_reach: 1.0,
        grid_step: Some(10.0),
        grid_reach: 3.0,
    };
    let landing = pull.onto_grid(sketch, DVec2::new(12.0, 150.0), &grid);

    document.apply(Operation::MovePoint {
        sketch: 0,
        point: PointId(2),
        position: landing,
        merged_into: None,
        on: Vec::new(),
        let_go: false,
    });

    let end = document.sketches()[0].point(PointId(2));
    assert!(end.distance(DVec2::new(10.0, 110.0)) < SETTLED, "{end}");
    let mut again = document.clone();
    again.undo();
    again.redo();
    assert!(again.sketches()[0].point(PointId(2)).distance(end) < 1e-9);
}

#[test]
fn a_corner_dragged_is_rebuilt_where_the_drag_showed_it() {
    let mut document = a_rectangle();
    let mut shown: Sketch = document.sketches()[0].clone();
    let pull = shown.pull(PointId(1), document.scale());
    shown.settle_pulled(&pull, DVec2::new(5.0, 45.0), document.scale());

    document.apply(Operation::MovePoint {
        sketch: 0,
        point: PointId(1),
        position: DVec2::new(5.0, 45.0),
        merged_into: None,
        on: Vec::new(),
        let_go: false,
    });

    let wanted: Vec<DVec2> = (1..=4).map(|rank| shown.point(PointId(rank))).collect();
    assert_same(&corners(&document), &wanted);
    assert!(
        wanted[1].distance(DVec2::new(120.0, 70.0)) < SETTLED,
        "the opposite corner stayed"
    );
}

#[test]
fn compacting_keeps_the_shape_a_side_moved_and_a_turn_left() {
    let mut document = a_rectangle();
    document.apply(Operation::MoveSegment {
        sketch: 0,
        segment: SegmentId(2),
        by: DVec2::new(0.0, 15.0),
    });
    document.apply(Operation::TurnShape {
        sketch: 0,
        points: (1..=4).map(PointId).collect(),
        about: DVec2::new(70.0, 52.5),
        angle: 0.3,
    });
    let left = corners(&document);

    document.compact_history();

    let drawn: Vec<DVec2> = document.sketches()[0]
        .drawn_points()
        .map(|(_, place)| place)
        .collect();
    for was in &left {
        assert!(
            drawn.iter().any(|now| now.distance(*was) < SETTLED),
            "no corner at {was} once compacted: {drawn:?}"
        );
    }
}
