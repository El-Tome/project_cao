//! What a drawing still needs before nothing can move.

use super::*;

/// The case reported from the drawing: with two angles and two sides given,
/// the third side follows and cannot be set independently.
#[test]
fn the_third_side_of_a_settled_triangle_is_redundant() {
    let (mut sketch, [base, side, hypotenuse]) = triangle();
    sketch.set_dimension(DimensionTarget::Length(base), 40.0, false);
    sketch.set_dimension(DimensionTarget::Length(side), 30.0, false);
    sketch.set_dimension(
        DimensionTarget::Angle {
            first: base,
            second: side,
        },
        90.0,
        false,
    );
    sketch.resolve(1.0);

    assert!(
        sketch.would_be_redundant(DimensionTarget::Length(hypotenuse), 1.0),
        "the hypotenuse follows from the two sides and their angle"
    );
}

/// Nothing is redundant while the shape can still change.
#[test]
fn a_side_of_an_open_shape_is_not_redundant() {
    let (sketch, [_, _, hypotenuse]) = triangle();
    assert!(!sketch.would_be_redundant(DimensionTarget::Length(hypotenuse), 1.0));
}

/// Pinning a point takes away the two ways a drawing can slide, never the
/// way it can turn: that last freedom needs an angle to a fixed direction.
#[test]
fn a_drawing_needs_no_angle_to_the_axes_to_be_complete() {
    let (mut sketch, [base, side, _]) = triangle();
    sketch.set_dimension(DimensionTarget::Length(base), 40.0, false);
    sketch.set_dimension(DimensionTarget::Length(side), 30.0, false);
    sketch.set_dimension(
        DimensionTarget::Angle {
            first: base,
            second: side,
        },
        90.0,
        false,
    );
    sketch.resolve(1.0);

    // Which way up the drawing sits is implicit, like its origin point.
    assert_eq!(sketch.freedom(1.0).degrees_of_freedom, 0);
    assert!(sketch.is_fully_constrained(1.0));
}

/// Stating the orientation by hand must not take the same freedom twice,
/// or a drawing free to slide would be reported as pinned.
#[test]
fn an_angle_to_an_axis_replaces_the_implicit_one() {
    let (mut sketch, [base, side, _]) = triangle();
    sketch.set_dimension(DimensionTarget::Length(base), 40.0, false);
    sketch.set_dimension(DimensionTarget::Length(side), 30.0, false);
    sketch.set_dimension(
        DimensionTarget::Angle {
            first: base,
            second: side,
        },
        90.0,
        false,
    );
    sketch.set_dimension(
        DimensionTarget::AxisAngle {
            segment: base,
            axis: SketchAxis::U,
        },
        0.0,
        false,
    );
    sketch.resolve(1.0);

    assert_eq!(sketch.freedom(1.0).degrees_of_freedom, 0);
}

/// A drawing that touches nothing fixed can still slide about, however many
/// values it carries.
#[test]
fn a_drawing_that_is_not_pinned_is_never_complete() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let a = sketch.add_point(DVec2::new(5.0, 5.0));
    let b = sketch.add_point(DVec2::new(15.0, 5.0));
    let segment = sketch.add_segment(a, b);
    sketch.set_dimension(DimensionTarget::Length(segment), 10.0, false);
    sketch.set_dimension(
        DimensionTarget::AxisAngle {
            segment,
            axis: SketchAxis::U,
        },
        0.0,
        false,
    );

    assert!(!sketch.is_fully_constrained(1.0));
    assert_eq!(sketch.freedom(1.0).degrees_of_freedom, 2, "free to slide");
}

/// A readout says nothing about the shape, so it must not remove freedom.
#[test]
fn a_driven_dimension_constrains_nothing() {
    let (mut sketch, [base, _, _]) = triangle();
    let before = sketch.freedom(1.0).degrees_of_freedom;
    sketch.set_dimension(DimensionTarget::Length(base), 100.0, true);
    assert_eq!(sketch.freedom(1.0).degrees_of_freedom, before);
}
