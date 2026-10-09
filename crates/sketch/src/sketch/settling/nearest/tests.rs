//! What sketch · settling/nearest.rs is held to.
//!
//! Closes #455.
//! - a trait alone given a new length keeps the end nearest the origin —
//!   `a_trait_alone_keeps_the_end_nearest_the_origin`
//! - a distance typed between two free points keeps the one nearest the origin
//!   — `a_distance_between_two_free_points_keeps_the_one_nearest_the_origin`
//! - four traits made square, only their width typed, keep the side nearest
//!   the origin and their height when the width changes —
//!   `a_square_of_four_traits_keeps_its_near_side_and_its_height`
//! - two points as near the origin as each other: the one drawn first stays —
//!   `of_two_points_as_near_the_origin_the_one_drawn_first_stays`
//!
//! The review of the first version found three ways it went wrong, held here:
//! a point held on the trait slid along it, where #446 keeps its place —
//! `a_point_held_on_the_trait_keeps_its_place_along_it`; a distance between
//! two shapes stretched the near one —
//! `a_distance_between_two_shapes_carries_the_far_one_and_leaves_the_near_one`;
//! and an arc hanging off the far end swelled rather than going with it —
//! `an_arc_hanging_off_the_far_end_goes_with_it`.

use glam::DVec2;

use crate::constraints::{Constraint, DimensionTarget};
use crate::laid_from::LaidFrom;
use crate::plane::WorkPlane;
use crate::sketch::{LengthOutcome, PointId, SegmentId, Sketch};

const SCALE: f64 = 1.0;

/// A micrometre: the solver stops at a thousandth of a percent of the
/// drawing's size, far inside that.
const SETTLED: f64 = 1e-3;

fn type_value(sketch: &mut Sketch, target: DimensionTarget, value: f64) {
    sketch.set_dimension(target, value, false);
    assert_eq!(
        sketch.land_value(target, SCALE),
        LengthOutcome::Exact,
        "the value was to land",
    );
}

fn assert_at(sketch: &Sketch, point: PointId, expected: DVec2, what: &str) {
    let at = sketch.point(point);
    assert!(
        at.distance(expected) < SETTLED,
        "{what} was to be at {expected}, it is at {at}",
    );
}

fn a_rectangle(sketch: &mut Sketch, corner: DVec2) -> ([PointId; 4], Vec<SegmentId>) {
    let corners = [
        corner,
        corner + DVec2::new(100.0, 0.0),
        corner + DVec2::new(100.0, 50.0),
        corner + DVec2::new(0.0, 50.0),
    ]
    .map(|corner| sketch.add_point(corner));
    let sides: Vec<SegmentId> = (0..4)
        .map(|index| sketch.add_segment(corners[index], corners[(index + 1) % 4]))
        .collect();
    for pair in 0..4 {
        sketch.add_constraint(Constraint::Perpendicular {
            first: sides[pair],
            second: sides[(pair + 1) % 4],
        });
    }
    (corners, sides)
}

fn a_trait(sketch: &mut Sketch, from: DVec2, to: DVec2) -> (PointId, PointId, SegmentId) {
    let start = sketch.add_point(from);
    let end = sketch.add_point(to);
    (start, end, sketch.add_segment(start, end))
}

#[test]
fn a_trait_alone_keeps_the_end_nearest_the_origin() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let (far, near, segment) = a_trait(
        &mut sketch,
        DVec2::new(150.0, 100.0),
        DVec2::new(100.0, 100.0),
    );
    sketch.set_dimension(DimensionTarget::Length(segment), 50.0, false);

    type_value(&mut sketch, DimensionTarget::Length(segment), 80.0);

    assert_at(&sketch, near, DVec2::new(100.0, 100.0), "the near end");
    assert_at(&sketch, far, DVec2::new(180.0, 100.0), "the far end");
}

#[test]
fn a_distance_between_two_free_points_keeps_the_one_nearest_the_origin() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let far = sketch.add_point(DVec2::new(100.0, 40.0));
    let near = sketch.add_point(DVec2::new(30.0, 40.0));
    let target = DimensionTarget::Distance {
        from: far,
        to: near,
    };
    sketch.set_dimension(target, 70.0, false);

    type_value(&mut sketch, target, 100.0);

    assert_at(&sketch, near, DVec2::new(30.0, 40.0), "the near point");
    assert_at(&sketch, far, DVec2::new(130.0, 40.0), "the far point");
}

#[test]
fn a_square_of_four_traits_keeps_its_near_side_and_its_height() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let (corners, sides) = a_rectangle(&mut sketch, DVec2::new(100.0, 100.0));
    sketch.set_dimension(DimensionTarget::Length(sides[0]), 100.0, false);
    assert_eq!(sketch.resolve(SCALE), LengthOutcome::Exact);

    type_value(&mut sketch, DimensionTarget::Length(sides[0]), 150.0);

    assert_at(
        &sketch,
        corners[0],
        DVec2::new(100.0, 100.0),
        "the bottom left corner",
    );
    assert_at(
        &sketch,
        corners[3],
        DVec2::new(100.0, 150.0),
        "the top left corner",
    );
    assert_at(
        &sketch,
        corners[1],
        DVec2::new(250.0, 100.0),
        "the bottom right corner",
    );
    assert_at(
        &sketch,
        corners[2],
        DVec2::new(250.0, 150.0),
        "the top right corner",
    );
}

#[test]
fn of_two_points_as_near_the_origin_the_one_drawn_first_stays() {
    for (first, second) in [
        (DVec2::new(-50.0, 100.0), DVec2::new(50.0, 100.0)),
        (DVec2::new(50.0, 100.0), DVec2::new(-50.0, 100.0)),
    ] {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let (drawn_first, drawn_second, segment) = a_trait(&mut sketch, first, second);
        sketch.set_dimension(DimensionTarget::Length(segment), 100.0, false);

        type_value(&mut sketch, DimensionTarget::Length(segment), 200.0);

        assert_at(&sketch, drawn_first, first, "the point drawn first");
        assert_at(
            &sketch,
            drawn_second,
            first + (second - first) * 2.0,
            "the point drawn second",
        );
    }
}

#[test]
fn a_point_held_on_the_trait_keeps_its_place_along_it() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let (_, _, segment) = a_trait(
        &mut sketch,
        DVec2::new(100.0, 100.0),
        DVec2::new(200.0, 100.0),
    );
    let held = sketch.add_point(DVec2::new(150.0, 100.0));
    sketch.add_constraint(Constraint::OnSegment {
        point: held,
        segment,
        from: LaidFrom::Nowhere,
    });
    sketch.set_dimension(DimensionTarget::Length(segment), 100.0, false);

    type_value(&mut sketch, DimensionTarget::Length(segment), 200.0);

    assert_at(
        &sketch,
        held,
        DVec2::new(200.0, 100.0),
        "the point held halfway",
    );
}

#[test]
fn a_distance_between_two_shapes_carries_the_far_one_and_leaves_the_near_one() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let (near, _) = a_rectangle(&mut sketch, DVec2::new(100.0, 100.0));
    let (far, _) = a_rectangle(&mut sketch, DVec2::new(300.0, 100.0));
    let target = DimensionTarget::Distance {
        from: near[0],
        to: far[0],
    };
    sketch.set_dimension(target, 200.0, false);
    assert_eq!(sketch.resolve(SCALE), LengthOutcome::Exact);

    type_value(&mut sketch, target, 250.0);

    for (index, corner) in near.into_iter().enumerate() {
        let drawn = DVec2::new(100.0, 100.0)
            + [
                DVec2::ZERO,
                DVec2::new(100.0, 0.0),
                DVec2::new(100.0, 50.0),
                DVec2::new(0.0, 50.0),
            ][index];
        assert_at(&sketch, corner, drawn, "a corner of the near shape");
        assert_at(
            &sketch,
            far[index],
            drawn + DVec2::new(250.0, 0.0),
            "a corner of the far shape",
        );
    }
}

#[test]
fn an_arc_hanging_off_the_far_end_goes_with_it() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let (_, end, segment) = a_trait(
        &mut sketch,
        DVec2::new(100.0, 100.0),
        DVec2::new(200.0, 100.0),
    );
    let centre = sketch.add_point(DVec2::new(200.0, 150.0));
    let other = sketch.add_point(DVec2::new(250.0, 150.0));
    let arc = sketch.add_arc(centre, end, other);
    sketch.set_dimension(DimensionTarget::Length(segment), 100.0, false);

    type_value(&mut sketch, DimensionTarget::Length(segment), 150.0);

    assert_at(&sketch, end, DVec2::new(250.0, 100.0), "the far end");
    assert_at(
        &sketch,
        centre,
        DVec2::new(250.0, 150.0),
        "the arc's centre",
    );
    assert!(
        (sketch.arc_radius(arc) - 50.0).abs() < SETTLED,
        "the arc was to keep its radius of 50, it is {}",
        sketch.arc_radius(arc),
    );
}
