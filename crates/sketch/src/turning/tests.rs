//! What turning a shape by hand keeps, and what it refuses.

use glam::DVec2;

use super::*;
use crate::constraints::{Constraint, DimensionTarget};
use crate::element::Element;
use crate::plane::WorkPlane;
use crate::sketch::SegmentId;

fn square() -> (Sketch, [PointId; 4]) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corners = [
        DVec2::new(10.0, 10.0),
        DVec2::new(50.0, 10.0),
        DVec2::new(50.0, 50.0),
        DVec2::new(10.0, 50.0),
    ]
    .map(|place| sketch.add_point(place));
    for rank in 0..4 {
        let side = sketch.add_segment(corners[rank], corners[(rank + 1) % 4]);
        sketch.set_dimension(DimensionTarget::Length(side), 40.0, false);
    }
    (sketch, corners)
}

#[test]
fn a_turn_carries_the_shape_whole_about_its_place() {
    let (mut sketch, corners) = square();
    let about = DVec2::new(30.0, 30.0);

    let outcome = sketch.turn_shape(&corners, about, std::f64::consts::FRAC_PI_2, 1.0);

    assert_eq!(outcome, LengthOutcome::Exact);
    assert!(sketch.point(corners[0]).distance(DVec2::new(50.0, 10.0)) < 1e-6);
    assert!(sketch.point(corners[2]).distance(DVec2::new(10.0, 50.0)) < 1e-6);
}

#[test]
fn a_turn_that_would_move_a_fixed_point_leaves_the_shape_where_it_is() {
    let (mut sketch, corners) = square();
    sketch.add_constraint(Constraint::Fixed {
        element: Element::Point(corners[0]),
    });

    let outcome = sketch.turn_shape(&corners, DVec2::new(30.0, 30.0), 0.5, 1.0);

    assert_eq!(outcome, LengthOutcome::BestEffort);
    assert_eq!(sketch.point(corners[1]), DVec2::new(50.0, 10.0));
}

#[test]
fn a_shape_held_level_by_a_rule_has_no_place_to_turn_about() {
    let (mut sketch, corners) = square();
    let base = SegmentId(0);
    sketch.add_constraint(Constraint::AxisParallel {
        segment: base,
        axis: crate::constraints::SketchAxis::U,
    });

    assert_eq!(sketch.turning_centre(&corners, 1.0), None);
}

#[test]
fn an_end_close_enough_to_a_grid_point_is_turned_onto_it() {
    let about = DVec2::new(0.0, 0.0);
    let end = DVec2::new(30.0, 0.0);
    let grid = SnapSettings {
        point_reach: 1.0,
        curve_reach: 1.0,
        grid_step: Some(10.0),
        grid_reach: 2.0,
    };

    let square = angle_onto_grid(about, end, 1.55, &grid);
    let between = angle_onto_grid(about, end, 0.4, &grid);

    assert!(
        (square - std::f64::consts::FRAC_PI_2).abs() < 1e-12,
        "{square}"
    );
    assert!(
        (between - 0.4).abs() < 1e-12,
        "no grid point near: {between}"
    );
}
