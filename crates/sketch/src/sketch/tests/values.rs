//! What a value typed on a drawing pins down.

use super::*;

#[test]
fn a_point_is_pushed_square_to_a_line() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(DVec2::new(0.0, 0.0));
    let end = sketch.add_point(DVec2::new(40.0, 0.0));
    let line = sketch.add_segment(start, end);
    let floating = sketch.add_point(DVec2::new(10.0, 5.0));

    sketch.set_dimension(
        DimensionTarget::PointToSegment {
            point: floating,
            segment: line,
        },
        12.0,
        false,
    );
    assert_eq!(sketch.resolve(1.0), LengthOutcome::Exact);

    let distance = sketch.point_to_segment(floating, line).unwrap();
    assert!((distance - 12.0).abs() < 1e-2, "distance = {distance}");
}

#[test]
fn the_distance_holds_when_the_foot_falls_off_the_segment() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(DVec2::new(0.0, 0.0));
    let end = sketch.add_point(DVec2::new(10.0, 0.0));
    let line = sketch.add_segment(start, end);
    let far = sketch.add_point(DVec2::new(80.0, 3.0));

    sketch.set_dimension(
        DimensionTarget::PointToSegment {
            point: far,
            segment: line,
        },
        20.0,
        false,
    );
    sketch.resolve(1.0);

    let distance = sketch.point_to_segment(far, line).unwrap();
    assert!((distance - 20.0).abs() < 1e-2, "distance = {distance}");
}

#[test]
fn a_width_moves_the_trait_sideways_and_leaves_its_height_alone() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(DVec2::new(0.0, 0.0));
    let end = sketch.add_point(DVec2::new(40.0, 30.0));
    sketch.add_segment(start, end);

    sketch.set_dimension(
        DimensionTarget::Projected {
            from: start,
            to: end,
            axis: SketchAxis::U,
        },
        60.0,
        false,
    );
    assert_eq!(sketch.resolve(1.0), LengthOutcome::Exact);

    let width = sketch.projected_gap(start, end, SketchAxis::U).unwrap();
    let height = sketch.projected_gap(start, end, SketchAxis::V).unwrap();
    assert!((width - 60.0).abs() < 1e-2, "width = {width}");
    assert!((height - 30.0).abs() < 1e-2, "height = {height}");
}

#[test]
fn a_width_and_a_height_together_pin_a_trait_down() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = Sketch::ORIGIN;
    let end = sketch.add_point(DVec2::new(40.0, 30.0));
    sketch.add_segment(start, end);

    for (axis, value) in [(SketchAxis::U, 80.0), (SketchAxis::V, 15.0)] {
        sketch.set_dimension(
            DimensionTarget::Projected {
                from: start,
                to: end,
                axis,
            },
            value,
            false,
        );
    }
    assert_eq!(sketch.resolve(1.0), LengthOutcome::Exact);

    let landed = sketch.point(end);
    assert!(
        (landed - DVec2::new(80.0, 15.0)).length() < 1e-2,
        "landed at {landed:?}"
    );
}

#[test]
fn changing_an_angle_turns_the_far_shape_instead_of_bending_it() {
    // A chain hung off the origin, with a corner dimensioned halfway along
    // and nothing else holding it: exactly the drawing in progress where
    // the far end used to wander.
    let mut sketch = Sketch::new(WorkPlane::XY);
    let o = Sketch::ORIGIN;
    let a = sketch.add_point(DVec2::new(40.0, 0.0));
    let corner = sketch.add_point(DVec2::new(80.0, 0.0));
    let c = sketch.add_point(DVec2::new(80.0, 40.0));
    let d = sketch.add_point(DVec2::new(120.0, 60.0));
    sketch.add_segment(o, a);
    let first = sketch.add_segment(a, corner);
    let second = sketch.add_segment(corner, c);
    sketch.add_segment(c, d);

    sketch.set_dimension(DimensionTarget::Angle { first, second }, 90.0, false);
    sketch.resolve(1.0);

    let shape_of = |sketch: &Sketch| {
        [
            sketch.point(o).distance(sketch.point(a)),
            sketch.point(c).distance(sketch.point(d)),
            (sketch.point(d) - sketch.point(c)).perp_dot(sketch.point(corner) - sketch.point(c)),
        ]
    };
    let (was_anchored, was_far) = (sketch.point(a), shape_of(&sketch));

    sketch.set_dimension(DimensionTarget::Angle { first, second }, 60.0, false);
    assert_eq!(sketch.resolve(1.0), LengthOutcome::Exact);

    // The side hanging off the origin has not budged...
    assert!(
        sketch.point(a).distance(was_anchored) < 1e-6,
        "the anchored side moved: {:?}",
        sketch.point(a)
    );
    // ...and the far side kept its shape, corner included, rather than
    // being bent to absorb the change.
    let now = shape_of(&sketch);
    for (before, after) in was_far.iter().zip(now.iter()) {
        assert!(
            (before - after).abs() < 1e-9 * before.abs().max(1.0),
            "the far shape was deformed: {was_far:?} then {now:?}"
        );
    }
    let angle = sketch.angle_between(first, second).unwrap();
    assert!((angle - 60.0).abs() < 1e-3, "angle came out at {angle}");
}

#[test]
fn a_rectangle_does_not_turn_when_one_of_its_sides_changes() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corner = Sketch::ORIGIN;
    let right = sketch.add_point(DVec2::new(100.0, 0.0));
    let far = sketch.add_point(DVec2::new(100.0, 50.0));
    let top = sketch.add_point(DVec2::new(0.0, 50.0));
    let sides = [
        sketch.add_segment(corner, right),
        sketch.add_segment(right, far),
        sketch.add_segment(far, top),
        sketch.add_segment(top, corner),
    ];
    for pair in sides.windows(2) {
        sketch.set_dimension(
            DimensionTarget::Angle {
                first: pair[0],
                second: pair[1],
            },
            90.0,
            false,
        );
    }
    sketch.set_dimension(DimensionTarget::Length(sides[0]), 100.0, false);
    sketch.set_dimension(DimensionTarget::Length(sides[1]), 50.0, false);
    sketch.resolve(1.0);
    assert!(sketch.is_fully_constrained(1.0));

    sketch.set_dimension(DimensionTarget::Length(sides[1]), 80.0, false);
    sketch.resolve(1.0);

    // Nothing holds the rectangle upright but the implicit rule that it
    // does not turn on its own, which is exactly what is being checked.
    let base = sketch.point(right) - sketch.point(corner);
    assert!(
        base.y.atan2(base.x).to_degrees().abs() < 0.05,
        "the rectangle leaned over: {base:?}"
    );
    assert!((sketch.point(far) - sketch.point(right)).length() - 80.0 < 0.05);
}

#[test]
fn the_two_ways_of_naming_a_pair_are_one_target() {
    let first = DimensionTarget::Angle {
        first: SegmentId(3),
        second: SegmentId(1),
    };
    let second = DimensionTarget::Angle {
        first: SegmentId(1),
        second: SegmentId(3),
    };
    assert_eq!(first.normalised(), second.normalised());

    let there = DimensionTarget::Distance {
        from: PointId(5),
        to: PointId(2),
    };
    let back = DimensionTarget::Distance {
        from: PointId(2),
        to: PointId(5),
    };
    assert_eq!(there.normalised(), back.normalised());
}

/// The bug this solver exists for: setting a second value must not undo the
/// first. Applying each dimension once and forgetting it could never do
/// this.
#[test]
fn an_angle_still_holds_after_a_length_is_changed() {
    let (mut sketch, [base, side, _]) = triangle();

    sketch.set_dimension(
        DimensionTarget::Angle {
            first: base,
            second: side,
        },
        60.0,
        false,
    );
    sketch.resolve(1.0);
    assert!((sketch.angle_between(base, side).unwrap() - 60.0).abs() < 0.1);

    sketch.set_dimension(DimensionTarget::Length(base), 100.0, false);
    sketch.resolve(1.0);

    let angle = sketch.angle_between(base, side).unwrap();
    let length = sketch.segment_length(base);
    assert!((angle - 60.0).abs() < 0.1, "the angle drifted to {angle}°");
    assert!((length - 100.0).abs() < 0.1, "the length is {length}");
}

#[test]
fn every_value_holds_at_once() {
    let (mut sketch, [base, side, _]) = triangle();
    sketch.set_dimension(DimensionTarget::Length(base), 50.0, false);
    sketch.set_dimension(DimensionTarget::Length(side), 20.0, false);
    sketch.set_dimension(
        DimensionTarget::Angle {
            first: base,
            second: side,
        },
        45.0,
        false,
    );

    assert_eq!(sketch.resolve(1.0), LengthOutcome::Exact);

    assert!((sketch.segment_length(base) - 50.0).abs() < 0.1);
    assert!((sketch.segment_length(side) - 20.0).abs() < 0.1);
    assert!((sketch.angle_between(base, side).unwrap() - 45.0).abs() < 0.1);
}

/// Contradictory values cannot both be met, and the solver has to say so
/// rather than quietly settling on one of them.
#[test]
fn impossible_values_are_reported() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let a = sketch.add_point(DVec2::ZERO);
    let b = sketch.add_point(DVec2::new(10.0, 0.0));
    let first = sketch.add_segment(a, b);
    let second = sketch.add_segment(a, b);

    sketch.set_dimension(DimensionTarget::Length(first), 50.0, false);
    sketch.set_dimension(DimensionTarget::Length(second), 90.0, false);

    assert_eq!(sketch.resolve(1.0), LengthOutcome::BestEffort);
}

#[test]
fn an_angle_needs_two_segments_that_meet() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let a = sketch.add_point(DVec2::ZERO);
    let b = sketch.add_point(DVec2::new(10.0, 0.0));
    let c = sketch.add_point(DVec2::new(0.0, 5.0));
    let d = sketch.add_point(DVec2::new(5.0, 5.0));
    let first = sketch.add_segment(a, b);
    let apart = sketch.add_segment(c, d);

    assert!(sketch.angle_between(first, apart).is_none());
}

#[test]
fn an_axis_angle_measures_from_the_sketch_direction() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let a = sketch.add_point(DVec2::ZERO);
    let b = sketch.add_point(DVec2::new(10.0, 10.0));
    let segment = sketch.add_segment(a, b);

    let angle = sketch.angle_with_axis(segment, SketchAxis::U).unwrap();
    assert!((angle - 45.0).abs() < 1e-3, "got {angle}°");
}

/// A distance can be measured between any two points, joined or not, which
/// is what lets a shape be positioned from the origin.
#[test]
fn a_distance_pins_a_point_against_the_origin() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let free = sketch.add_point(DVec2::new(3.0, 4.0));

    sketch.set_dimension(
        DimensionTarget::Distance {
            from: Sketch::ORIGIN,
            to: free,
        },
        10.0,
        false,
    );
    assert_eq!(sketch.resolve(1.0), LengthOutcome::Exact);

    let distance = sketch.point(free).length();
    assert!((distance - 10.0).abs() < 0.01, "got {distance}");
    assert_eq!(sketch.point(Sketch::ORIGIN), DVec2::ZERO);
}
