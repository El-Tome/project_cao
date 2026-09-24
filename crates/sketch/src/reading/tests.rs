//! What sketch · reading.rs is held to.
//!
//! Closes #428.
//! - a surface goes up with the square of the scale while its perimeter does
//!   not — `a_surface_goes_up_with_the_square_of_the_scale_while_its_perimeter_does_not`,
//!   which fails if either factor goes, and which every other test in this
//!   file was blind to: they all run at a scale of one, where the squaring is
//!   the identity

use super::*;

use crate::constraints::SketchAxis;
use crate::plane::WorkPlane;

const TOLERANCE: f64 = 1e-9;

fn gap(reading: Option<Reading>) -> (f64, DVec2) {
    match reading {
        Some(Reading::Gap { span, offsets }) => (span, offsets),
        other => panic!("a straight run reads as a gap, got {other:?}"),
    }
}

fn degrees(reading: Option<Reading>) -> f64 {
    match reading {
        Some(Reading::Opening { degrees }) => degrees,
        other => panic!("two traits read as an opening, got {other:?}"),
    }
}

#[test]
fn two_points_read_how_far_apart_they_are_and_how_far_along_each_axis() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let from = sketch.add_point(DVec2::new(10.0, 20.0));
    let to = sketch.add_point(DVec2::new(50.0, 46.7));

    let (span, offsets) = gap(sketch.read(DimensionTarget::Distance { from, to }));

    assert!(
        (span - 2312.89_f64.sqrt()).abs() < TOLERANCE,
        "the direct run between the two, got {span}",
    );
    assert!(
        (offsets.x - 40.0).abs() < TOLERANCE && (offsets.y - 26.7).abs() < TOLERANCE,
        "the reach along each axis, got {offsets:?}",
    );
}

#[test]
fn a_trait_reads_its_length_with_the_reach_along_each_axis_beside_it() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(DVec2::new(1.0, 1.0));
    let end = sketch.add_point(DVec2::new(4.0, 5.0));
    let segment = sketch.add_segment(start, end);

    let (span, offsets) = gap(sketch.read(DimensionTarget::Length(segment)));

    assert!((span - 5.0).abs() < TOLERANCE, "a 3-4-5 trait, got {span}");
    assert!(
        (offsets.x - 3.0).abs() < TOLERANCE && (offsets.y - 4.0).abs() < TOLERANCE,
        "a length carries its two offsets as a distance does, got {offsets:?}",
    );
}

#[test]
fn a_point_reads_its_square_distance_to_a_trait() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(DVec2::new(0.0, 0.0));
    let end = sketch.add_point(DVec2::new(10.0, 0.0));
    let segment = sketch.add_segment(start, end);
    let point = sketch.add_point(DVec2::new(4.0, 3.0));

    let (span, offsets) = gap(sketch.read(DimensionTarget::PointToSegment { point, segment }));

    assert!(
        (span - 3.0).abs() < TOLERANCE,
        "the distance square onto the line, got {span}",
    );
    assert!(
        offsets.x.abs() < TOLERANCE && (offsets.y - 3.0).abs() < TOLERANCE,
        "square onto a flat trait, the whole reach is vertical, got {offsets:?}",
    );
}

#[test]
fn a_circle_reads_its_radius_and_says_nothing_of_where_it_was_clicked() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(3.0, -4.0));
    let circle = sketch.add_circle(centre, 12.5);

    let Some(Reading::Round { radius }) = sketch.read(DimensionTarget::Diameter(circle)) else {
        panic!("a circle reads as a round");
    };

    assert!(
        (radius - 12.5).abs() < TOLERANCE,
        "the radius, which the label doubles for the diameter, got {radius}",
    );
}

#[test]
fn an_arc_reads_the_radius_of_the_circle_it_was_cut_from() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::ZERO);
    let start = sketch.add_point(DVec2::new(7.0, 0.0));
    let end = sketch.add_point(DVec2::new(0.0, 7.0));
    let arc = sketch.add_arc(centre, start, end);

    let Some(Reading::Round { radius }) = sketch.read(DimensionTarget::ArcRadius(arc)) else {
        panic!("an arc reads as a round");
    };

    assert!(
        (radius - 7.0).abs() < TOLERANCE,
        "an arc is a circle that was cut, and keeps its radius, got {radius}",
    );
}

#[test]
fn two_traits_sharing_an_end_read_the_corner_they_make() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corner = sketch.add_point(DVec2::ZERO);
    let along = sketch.add_point(DVec2::new(5.0, 0.0));
    let up = sketch.add_point(DVec2::new(0.0, 5.0));
    let first = sketch.add_segment(corner, along);
    let second = sketch.add_segment(corner, up);

    let opening = degrees(sketch.read(DimensionTarget::Angle { first, second }));

    assert!(
        (opening - 90.0).abs() < TOLERANCE,
        "a square corner, got {opening}",
    );
}

#[test]
fn a_trait_against_an_axis_reads_how_far_it_leans() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(DVec2::ZERO);
    let end = sketch.add_point(DVec2::new(1.0, 1.0));
    let segment = sketch.add_segment(start, end);

    let opening = degrees(sketch.read(DimensionTarget::AxisAngle {
        segment,
        axis: SketchAxis::U,
    }));

    assert!(
        (opening - 45.0).abs() < TOLERANCE,
        "a trait laid corner to corner leans forty-five degrees, got {opening}",
    );
}

#[test]
fn a_reading_of_something_the_drawing_no_longer_has_is_no_reading_at_all() {
    let sketch = Sketch::new(WorkPlane::XY);
    let gone = crate::sketch::SegmentId(7);
    let circle = crate::circle::CircleId(7);
    let arc = crate::arc::ArcId(7);
    let point = crate::sketch::PointId(7);

    // Every arm, not just the easy one: a measure outlives the geometry under
    // it until the next frame clears it, and an arm that indexed instead of
    // asking would take the application down in that window.
    for target in [
        DimensionTarget::Length(gone),
        DimensionTarget::Distance {
            from: point,
            to: point,
        },
        DimensionTarget::PointToSegment {
            point,
            segment: gone,
        },
        DimensionTarget::Projected {
            from: point,
            to: point,
            axis: SketchAxis::U,
        },
        DimensionTarget::Radius(circle),
        DimensionTarget::Diameter(circle),
        DimensionTarget::ArcRadius(arc),
        DimensionTarget::ArcSweep(arc),
        DimensionTarget::Angle {
            first: gone,
            second: gone,
        },
        DimensionTarget::AxisAngle {
            segment: gone,
            axis: SketchAxis::U,
        },
    ] {
        assert_eq!(
            sketch.read(target),
            None,
            "{target:?} names nothing the drawing has, and a value shown for it \
             would not be true",
        );
    }
}

#[test]
fn millimetres_scale_the_lengths_and_leave_the_degrees_alone() {
    let reading = Reading::Gap {
        span: 2.0,
        offsets: DVec2::new(1.0, 1.5),
    };

    assert_eq!(
        reading.scaled(10.0),
        Reading::Gap {
            span: 20.0,
            offsets: DVec2::new(10.0, 15.0),
        },
        "a length and both its offsets go through the same scale",
    );
    assert_eq!(
        Reading::Opening { degrees: 37.5 }.scaled(10.0),
        Reading::Opening { degrees: 37.5 },
        "a degree is a degree whatever the drawing is scaled to",
    );
}

#[test]
fn a_straight_run_says_which_two_places_it_was_read_between() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(DVec2::new(1.0, 1.0));
    let end = sketch.add_point(DVec2::new(4.0, 5.0));
    let segment = sketch.add_segment(start, end);

    let (from, to) = sketch
        .run_of(DimensionTarget::Length(segment))
        .expect("a trait runs between its two ends");

    assert!(
        from.distance(DVec2::new(1.0, 1.0)) < TOLERANCE
            && to.distance(DVec2::new(4.0, 5.0)) < TOLERANCE,
        "the triangle is drawn on these two places, got {from:?} and {to:?}",
    );
}

#[test]
fn a_point_and_a_trait_run_between_the_point_and_its_foot() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(DVec2::new(0.0, 0.0));
    let end = sketch.add_point(DVec2::new(10.0, 0.0));
    let segment = sketch.add_segment(start, end);
    let point = sketch.add_point(DVec2::new(4.0, 3.0));

    let (from, to) = sketch
        .run_of(DimensionTarget::PointToSegment { point, segment })
        .expect("a point stands off a trait along a run");

    assert!(
        from.distance(DVec2::new(4.0, 3.0)) < TOLERANCE
            && to.distance(DVec2::new(4.0, 0.0)) < TOLERANCE,
        "square onto the trait, which is where the foot lands, got {from:?} and {to:?}",
    );
}

#[test]
fn a_round_and_an_opening_have_no_run_to_draw_a_triangle_on() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::ZERO);
    let circle = sketch.add_circle(centre, 5.0);

    assert_eq!(
        sketch.run_of(DimensionTarget::Diameter(circle)),
        None,
        "a circle is read from its centre out, not as two axes of a triangle",
    );
}

#[test]
fn a_surface_goes_up_with_the_square_of_the_scale_while_its_perimeter_does_not() {
    let read = Reading::Surface {
        area: 1000.0,
        perimeter: 130.0,
    };

    assert_eq!(
        read.scaled(10.0),
        Reading::Surface {
            area: 100_000.0,
            perimeter: 1300.0,
        },
        "a drawing ten times bigger holds a hundred times the surface and is \
         ten times round: the one place a measure's two units part company, \
         and the one every other test in this file runs at a scale of one",
    );
}
