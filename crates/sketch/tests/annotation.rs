//! Where a dimension's annotation is drawn: which side it stands off on, the
//! leader on the end a foot falls outside of, the floor under an angle's arc
//! radius, and a dragged annotation landing back where it was left.
//!
//! Closes #484.
//! - the arc of an angle to an axis is drawn around the middle of the trait,
//!   between the line along the axis and the half of the trait on the side it
//!   was put down — `an_angle_to_an_axis_is_drawn_in_the_quarter_it_measures`
//! - a trait whose middle lies on the axis draws no line along it: the axis is
//!   its arm — `an_angle_measured_on_the_axis_lets_the_axis_speak_for_itself`

use cao_sketch::{
    AnnotationMetrics, AxisToward, DimensionTarget, Sketch, SketchAxis, Toward, WorkPlane,
};
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

/// A trait 40 long, its middle at `middle`, leaning `degrees` off the
/// horizontal.
fn trait_about(middle: DVec2, degrees: f64) -> (Sketch, DimensionTarget) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let half = DVec2::from_angle(degrees.to_radians()) * 20.0;
    let from = sketch.add_point(middle - half);
    let to = sketch.add_point(middle + half);
    let segment = sketch.add_segment(from, to);
    let target = DimensionTarget::AxisAngle {
        segment,
        segment_toward: Toward::End,
        axis: SketchAxis::U,
        axis_toward: AxisToward::Positive,
    };
    sketch.set_dimension(target, degrees, false);
    (sketch, target)
}

fn arms_from(placement: &cao_sketch::Placement, pivot: DVec2) -> Vec<DVec2> {
    placement
        .shape
        .iter()
        .filter(|(from, _)| from.distance(pivot) <= 1e-9)
        .map(|(from, to)| *to - *from)
        .collect()
}

#[test]
fn an_angle_measured_off_the_axis_draws_the_line_it_opens_from() {
    let middle = DVec2::new(10.0, 10.0);
    let (sketch, target) = trait_about(middle, 30.0);

    let placement = sketch.place(target, METRICS).unwrap();

    let arms = arms_from(&placement, middle);
    assert_eq!(
        arms.len(),
        1,
        "the arc opens from a horizontal the drawing shows nowhere, so the annotation carries it: {:?}",
        placement.shape
    );
    assert!(
        arms[0].perp_dot(DVec2::X).abs() <= 1e-9 && arms[0].dot(DVec2::X) > 0.0,
        "the line runs along the axis the angle is measured against, got {:?}",
        arms[0]
    );
}

#[test]
fn an_angle_measured_on_the_axis_lets_the_axis_speak_for_itself() {
    let middle = DVec2::new(10.0, 0.0);
    let (sketch, target) = trait_about(middle, 30.0);

    let placement = sketch.place(target, METRICS).unwrap();

    let along_the_axis = |(from, to): &&(DVec2, DVec2)| {
        from.y.abs() <= 1e-9 && to.y.abs() <= 1e-9 && from.distance(*to) > 1.0
    };
    assert_eq!(
        placement.shape.iter().filter(along_the_axis).count(),
        0,
        "the axis is drawn right through the vertex, and a second line on top of it says nothing: {:?}",
        placement.shape,
    );
}

/// Whether `place` lies between two arms out from `pivot`, the arms included.
fn between(pivot: DVec2, one: DVec2, other: DVec2, place: DVec2) -> bool {
    const ON_AN_ARM: f64 = 1e-6;
    let (to, opening) = (place - pivot, one.perp_dot(other).signum());
    one.perp_dot(to) * opening >= -ON_AN_ARM && to.perp_dot(other) * opening >= -ON_AN_ARM
}

#[test]
fn an_angle_to_an_axis_is_drawn_in_the_quarter_it_measures() {
    let (left, right) = (DVec2::new(20.0, 30.0), DVec2::new(120.0, 50.0));
    let middle = (left + right) * 0.5;
    for (put_down, along_the_axis, along_the_trait, said) in [
        (
            DVec2::new(40.0, 80.0),
            DVec2::Y,
            left - middle,
            "up and to the left",
        ),
        (
            DVec2::new(100.0, 0.0),
            -DVec2::Y,
            right - middle,
            "down and to the right",
        ),
    ] {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let from = sketch.add_point(left);
        let to = sketch.add_point(right);
        let segment = sketch.add_segment(from, to);
        let target = sketch.oriented(sketch.angle_to_axis(segment, SketchAxis::V), put_down);
        sketch.set_dimension(target, 101.3, false);

        let placement = sketch.place(target, METRICS).unwrap();

        let line = arms_from(&placement, middle);
        assert_eq!(
            line.len(),
            1,
            "{said}: one line along the axis: {:?}",
            placement.shape
        );
        assert!(
            line[0].perp_dot(along_the_axis).abs() <= 1e-9 && line[0].dot(along_the_axis) > 0.0,
            "{said}: it runs from the middle the way the arm does: {:?}",
            line[0],
        );
        let radius = line[0].length();
        let on_the_arc = |place: DVec2| (place.distance(middle) - radius).abs() < 1e-6;
        let arc: Vec<_> = placement
            .shape
            .iter()
            .filter(|(from, to)| on_the_arc(*from) && on_the_arc(*to))
            .collect();
        assert!(
            arc.len() > 10,
            "{said}: an arc about the middle: {:?}",
            placement.shape
        );
        for (from, to) in arc {
            for place in [*from, *to] {
                assert!(
                    between(middle, along_the_axis, along_the_trait, place),
                    "{place:?} strays out of the quarter {said}",
                );
            }
        }
        assert!(
            between(middle, along_the_axis, along_the_trait, placement.text_at),
            "{said}: the value sits in the quarter too: {:?}",
            placement.text_at,
        );
    }
}
