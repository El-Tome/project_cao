//! Where a dimension's annotation is drawn: which side it stands off on, the
//! leader on the end a foot falls outside of, the floor under an angle's arc
//! radius, and a dragged annotation landing back where it was left.

use cao_sketch::{AnnotationMetrics, DimensionTarget, Sketch, WorkPlane};
use glam::DVec2;

const METRICS: AnnotationMetrics = AnnotationMetrics {
    offset_pixels: 22.0,
    arrow_pixels: 8.0,
    arc_pixels: 34.0,
    pixel: 1.0,
    nudge: DVec2::ZERO,
};

#[test]
fn a_dimension_line_stands_off_on_the_side_away_from_the_drawing() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    // A closed square sits entirely on the positive side of this segment, so
    // the offset that stands the dimension line off has to point the other
    // way, or the line would be drawn over the square.
    let a = sketch.add_point(DVec2::new(0.0, 0.0));
    let b = sketch.add_point(DVec2::new(40.0, 0.0));
    let c = sketch.add_point(DVec2::new(40.0, 40.0));
    let d = sketch.add_point(DVec2::new(0.0, 40.0));
    let bottom = sketch.add_segment(a, b);
    sketch.add_segment(b, c);
    sketch.add_segment(c, d);
    sketch.add_segment(d, a);

    let placement = sketch
        .place(DimensionTarget::Length(bottom), METRICS)
        .unwrap();

    assert!(
        placement.offset.y < 0.0,
        "the dimension line should stand off below the segment, away from the square: {:?}",
        placement.offset
    );
}

#[test]
fn only_the_end_the_foot_falls_outside_of_gets_a_leader() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(DVec2::new(0.0, 0.0));
    let end = sketch.add_point(DVec2::new(10.0, 0.0));
    let line = sketch.add_segment(start, end);
    let far = sketch.add_point(DVec2::new(30.0, 5.0));

    let placement = sketch
        .place(
            DimensionTarget::PointToSegment {
                point: far,
                segment: line,
            },
            METRICS,
        )
        .unwrap();

    let corner = DVec2::new(10.0, 0.0);
    let leaders = placement
        .shape
        .iter()
        .filter(|(from, to)| *from == corner || *to == corner)
        .count();
    assert_eq!(
        leaders, 1,
        "the foot falls past one end only, so exactly one leader should reach out to it: {:?}",
        placement.shape
    );
}

#[test]
fn an_angular_dimension_keeps_its_arc_radius_above_the_floor() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let pivot = sketch.add_point(DVec2::new(0.0, 0.0));
    let a = sketch.add_point(DVec2::new(10.0, 0.0));
    let b = sketch.add_point(DVec2::new(0.0, 10.0));
    let first = sketch.add_segment(pivot, a);
    let second = sketch.add_segment(pivot, b);
    let target = DimensionTarget::Angle { first, second };
    sketch.set_dimension(target, 90.0, false);
    // A recorded offset closer to the pivot than the clearance around it asks
    // for a radius under the floor, which the floor has to refuse: an arc
    // that thin is unreadable.
    sketch.offset_dimension(target, DVec2::new(10.0, 0.0));

    let placement = sketch.place(target, METRICS).unwrap();

    // The floor clamps what is drawn, not what is stored: the arc is a fan of
    // short segments out from the pivot, and the first point on it says how
    // far the radius actually landed.
    let (drawn, _) = placement.shape[0];
    assert!(
        drawn.length() >= 6.0 * METRICS.pixel - 1e-9,
        "the arc radius should never fall below its floor: {drawn:?}"
    );
}

#[test]
fn a_dimension_dragged_and_placed_again_lands_where_it_was_left() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(DVec2::new(0.0, 0.0));
    let end = sketch.add_point(DVec2::new(40.0, 0.0));
    let segment = sketch.add_segment(start, end);
    let target = DimensionTarget::Length(segment);
    sketch.set_dimension(target, 40.0, false);

    let first = sketch.place(target, METRICS).unwrap();
    // What a drag records, once the button is let go.
    sketch.offset_dimension(target, first.offset);

    let second = sketch.place(target, METRICS).unwrap();

    assert!(
        (second.text_at - first.text_at).length() < 1e-9,
        "asked again with nothing new dragged, the annotation should land exactly \
         where it was left: {:?} vs {:?}",
        first.text_at,
        second.text_at
    );
}
