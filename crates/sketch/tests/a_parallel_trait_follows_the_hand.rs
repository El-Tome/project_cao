//! A trait whose length is typed and whose direction a rule holds cannot
//! stretch or turn: dragged by an end, it travels whole.
//!
//! Closes #473.
//! - a trait with its length typed, parallel to a side of a rectangle:
//!   dragging an end moves the trait, still parallel and still its length —
//!   `a_trait_made_parallel_to_a_rectangle_travels_with_the_hand`
//! - the rectangle does not move —
//!   `the_rectangle_stays_while_the_parallel_trait_travels`

use cao_sketch::{
    Constraint, DimensionTarget, LengthOutcome, PointId, SegmentId, Sketch, WorkPlane,
};
use glam::DVec2;

/// A rectangle 150 by 200 from the origin, its sizes typed, and a trait 80
/// long above it, laid parallel to its top.
fn the_drawing() -> (Sketch, [SegmentId; 4], SegmentId) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corners = [
        Sketch::ORIGIN,
        sketch.add_point(DVec2::new(150.0, 0.0)),
        sketch.add_point(DVec2::new(150.0, 200.0)),
        sketch.add_point(DVec2::new(0.0, 200.0)),
    ];
    let sides: [SegmentId; 4] =
        std::array::from_fn(|side| sketch.add_segment(corners[side], corners[(side + 1) % 4]));
    for pair in 0..3 {
        sketch.add_constraint(Constraint::Perpendicular {
            first: sides[pair],
            second: sides[pair + 1],
        });
    }
    sketch.set_dimension(DimensionTarget::Length(sides[0]), 150.0, false);
    sketch.set_dimension(DimensionTarget::Length(sides[1]), 200.0, false);
    let start = sketch.add_point(DVec2::new(60.0, 280.0));
    let end = sketch.add_point(DVec2::new(130.0, 250.0));
    let free = sketch.add_segment(start, end);
    sketch.set_dimension(DimensionTarget::Length(free), 80.0, false);
    sketch.resolve(1.0);
    sketch.lay_rule(
        Constraint::Parallel {
            first: free,
            second: sides[2],
        },
        1.0,
    );
    (sketch, sides, free)
}

fn drag(sketch: &mut Sketch, point: PointId, by: DVec2) -> LengthOutcome {
    let pull = sketch.pull(point, 1.0);
    let to = sketch.point(point) + by;
    let landing = sketch.slide(point, to);
    sketch.settle_pulled(&pull, landing, 1.0)
}

#[test]
fn a_trait_made_parallel_to_a_rectangle_travels_with_the_hand() {
    for by in [
        DVec2::new(30.0, 20.0),
        DVec2::new(30.0, 0.0),
        DVec2::new(0.0, -15.0),
    ] {
        let (mut sketch, sides, free) = the_drawing();
        let end = sketch.segments()[free.0].end;
        let to = sketch.point(end) + by;

        let outcome = drag(&mut sketch, end, by);

        assert_eq!(outcome, LengthOutcome::Exact, "dragged by {by}");
        assert!(
            sketch.point(end).distance(to) < 1e-3,
            "dragged by {by}: the end is at {}, not {to}",
            sketch.point(end)
        );
        assert!(
            (sketch.segment_length(free) - 80.0).abs() < 1e-3,
            "dragged by {by}: still 80 long, not {}",
            sketch.segment_length(free)
        );
        let (a, b) = sketch.endpoints(free);
        let (c, d) = sketch.endpoints(sides[2]);
        let across = (b - a).normalize().perp_dot((d - c).normalize());
        assert!(
            across.abs() < 1e-4,
            "dragged by {by}: no longer parallel ({across})"
        );
    }
}

#[test]
fn the_rectangle_stays_while_the_parallel_trait_travels() {
    let (mut sketch, _, free) = the_drawing();
    let before: Vec<DVec2> = sketch.points()[..4].to_vec();
    let end = sketch.segments()[free.0].end;

    drag(&mut sketch, end, DVec2::new(30.0, 20.0));

    assert!(
        sketch.points()[..4]
            .iter()
            .zip(&before)
            .all(|(now, was)| now.distance(*was) < 1e-3),
        "the rectangle moved: {:?}",
        &sketch.points()[..4]
    );
}
