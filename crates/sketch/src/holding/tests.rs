use glam::DVec2;

use crate::arc::ArcId;
use crate::constraints::{Constraint, DimensionTarget, SketchAxis};
use crate::holding::Support;
use crate::plane::WorkPlane;
use crate::sketch::{CircleId, SegmentId, Sketch};

#[test]
fn a_point_held_on_an_arc_keeps_its_reach_when_the_arc_grows() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(20.0, 30.0));
    let start = sketch.add_point(DVec2::new(30.0, 30.0));
    let end = sketch.add_point(DVec2::new(20.0, 40.0));
    let arc = sketch.add_arc(centre, start, end);
    let point = sketch.add_point(DVec2::new(20.0, 30.0) + DVec2::splat(10.0 / 2.0_f64.sqrt()));
    sketch.add_constraint(Constraint::OnArc { point, arc });

    sketch.set_dimension(DimensionTarget::ArcRadius(arc), 20.0, false);
    sketch.resolve(1.0);

    let reach = sketch.point(point).distance(sketch.point(centre));
    assert!(
        (reach - 20.0).abs() < 1e-3,
        "the point stands {reach} from the centre, and the curve 20",
    );
}

#[test]
fn a_point_held_on_an_axis_of_the_plane_is_pulled_back_onto_it() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let on_axis = sketch.add_point(DVec2::new(30.0, 0.0));
    let far = sketch.add_point(DVec2::new(30.0, 20.0));
    let side = sketch.add_segment(on_axis, far);
    sketch.add_constraint(Constraint::OnAxis {
        point: on_axis,
        axis: SketchAxis::U,
    });

    sketch.set_dimension(DimensionTarget::Length(side), 40.0, false);
    sketch.resolve(1.0);

    let off = sketch.point(on_axis).y;
    assert!(
        off.abs() < 1e-6,
        "the point stands {off} off the axis it is held on",
    );
}

#[test]
fn a_trait_dragged_carries_the_point_it_holds_without_being_bent_by_it() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(DVec2::new(10.0, 10.0));
    let end = sketch.add_point(DVec2::new(50.0, 10.0));
    let side = sketch.add_segment(start, end);
    let point = sketch.add_point(DVec2::new(30.0, 10.0));
    sketch.add_constraint(Constraint::OnSegment {
        point,
        segment: side,
    });

    sketch.settle_around(start, DVec2::new(10.0, 30.0), 1.0);

    let far = sketch.point(end);
    assert!(
        far.distance(DVec2::new(50.0, 10.0)) < 1e-6,
        "the far end of the trait was dragged along to {far}",
    );
    let (a, b, held) = (sketch.point(start), far, sketch.point(point));
    let off = (b - a).perp_dot(held - a).abs() / (b - a).length();
    assert!(off < 1e-6, "the point stands {off} off the trait it is on");
}

#[test]
fn a_point_held_where_two_traits_cross_can_no_longer_move_on_its_own() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let west = sketch.add_point(DVec2::new(0.0, 20.0));
    let east = sketch.add_point(DVec2::new(40.0, 20.0));
    let across = sketch.add_segment(west, east);
    let south = sketch.add_point(DVec2::new(20.0, 0.0));
    let north = sketch.add_point(DVec2::new(20.0, 40.0));
    let up = sketch.add_segment(south, north);
    let crossing = sketch.add_point(DVec2::new(20.0, 20.0));
    sketch.add_constraint(Constraint::OnSegment {
        point: crossing,
        segment: across,
    });
    sketch.add_constraint(Constraint::OnSegment {
        point: crossing,
        segment: up,
    });

    let settled = sketch.settled_points(1.0);

    assert!(
        settled[crossing.0],
        "a point held on both of the traits that cross there has nowhere left to go",
    );
    assert!(
        !settled[north.0],
        "the traits themselves are still free to move",
    );
}

/// A trait, a circle crossing it, an arc, and the drawing's own axes.
fn a_drawing_to_land_on() -> (Sketch, SegmentId, CircleId, ArcId) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let west = sketch.add_point(DVec2::new(0.0, 20.0));
    let east = sketch.add_point(DVec2::new(40.0, 20.0));
    let side = sketch.add_segment(west, east);

    let middle = sketch.add_point(DVec2::new(10.0, 20.0));
    let round = sketch.add_circle(middle, 5.0);

    let centre = sketch.add_point(DVec2::new(30.0, 40.0));
    let start = sketch.add_point(DVec2::new(40.0, 40.0));
    let end = sketch.add_point(DVec2::new(30.0, 50.0));
    let bend = sketch.add_arc(centre, start, end);

    (sketch, side, round, bend)
}

#[test]
fn a_place_along_a_trait_lands_on_that_trait() {
    let (sketch, side, _, _) = a_drawing_to_land_on();

    let landed = sketch.supports_at(DVec2::new(25.0, 20.0));

    assert_eq!(landed, vec![Support::Segment(side)]);
}

#[test]
fn a_place_where_a_trait_and_a_circle_cross_lands_on_both() {
    let (sketch, side, round, _) = a_drawing_to_land_on();

    let landed = sketch.supports_at(DVec2::new(15.0, 20.0));

    assert_eq!(landed, vec![Support::Segment(side), Support::Circle(round)]);
}

#[test]
fn a_place_along_an_arc_lands_on_it_and_a_place_past_its_end_lands_on_nothing() {
    let (sketch, _, _, bend) = a_drawing_to_land_on();
    let quarter = DVec2::new(30.0, 40.0) + DVec2::splat(10.0 / 2.0_f64.sqrt());

    assert_eq!(sketch.supports_at(quarter), vec![Support::Arc(bend)]);
    assert_eq!(
        sketch.supports_at(DVec2::new(30.0, 30.0)),
        Vec::new(),
        "the rest of the circle an arc is a piece of is not drawn",
    );
}

#[test]
fn a_place_on_an_axis_of_the_plane_lands_on_that_axis() {
    let (sketch, _, _, _) = a_drawing_to_land_on();

    assert_eq!(
        sketch.supports_at(DVec2::new(25.0, 0.0)),
        vec![Support::Axis(SketchAxis::U)],
    );
    assert_eq!(
        sketch.supports_at(DVec2::new(0.0, 35.0)),
        vec![Support::Axis(SketchAxis::V)],
    );
}

#[test]
fn a_place_beside_the_drawing_lands_on_nothing() {
    let (sketch, _, _, _) = a_drawing_to_land_on();

    assert_eq!(sketch.supports_at(DVec2::new(25.0, 25.0)), Vec::new());
}

#[test]
fn a_point_held_on_a_trait_slides_along_it_when_it_is_pulled_off() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(DVec2::new(10.0, 10.0));
    let end = sketch.add_point(DVec2::new(50.0, 10.0));
    let side = sketch.add_segment(start, end);
    let point = sketch.add_point(DVec2::new(30.0, 10.0));
    sketch.add_constraint(Constraint::OnSegment {
        point,
        segment: side,
    });

    let slid = sketch.slide(point, DVec2::new(35.0, 25.0));

    assert!(
        slid.distance(DVec2::new(35.0, 10.0)) < 1e-9,
        "pulled off its trait, the point went to {slid}",
    );
}

#[test]
fn a_point_held_on_a_circle_slides_round_its_rim() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(20.0, 20.0));
    let round = sketch.add_circle(centre, 10.0);
    let point = sketch.add_point(DVec2::new(30.0, 20.0));
    sketch.add_constraint(Constraint::OnCircle {
        point,
        circle: round,
    });

    let slid = sketch.slide(point, DVec2::new(20.0, 45.0));

    assert!(
        slid.distance(DVec2::new(20.0, 30.0)) < 1e-9,
        "pulled away from its circle, the point went to {slid}",
    );
}

#[test]
fn a_point_held_at_a_crossing_stays_there_however_it_is_pulled() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let west = sketch.add_point(DVec2::new(0.0, 20.0));
    let east = sketch.add_point(DVec2::new(40.0, 20.0));
    let across = sketch.add_segment(west, east);
    let south = sketch.add_point(DVec2::new(20.0, 0.0));
    let north = sketch.add_point(DVec2::new(20.0, 40.0));
    let up = sketch.add_segment(south, north);
    let point = sketch.add_point(DVec2::new(20.0, 20.0));
    for segment in [across, up] {
        sketch.add_constraint(Constraint::OnSegment { point, segment });
    }

    let slid = sketch.slide(point, DVec2::new(35.0, 35.0));

    assert!(
        slid.distance(DVec2::new(20.0, 20.0)) < 1e-9,
        "held on both traits, the point still went to {slid}",
    );
}

#[test]
fn a_point_nothing_holds_goes_where_it_is_pulled() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let point = sketch.add_point(DVec2::new(10.0, 10.0));

    let slid = sketch.slide(point, DVec2::new(35.0, 35.0));

    assert!(
        slid.distance(DVec2::new(35.0, 35.0)) < 1e-9,
        "went to {slid}"
    );
}
