//! What sketch · corner.rs is held to.

use glam::DVec2;

use super::*;
use crate::CutSide;
use crate::chamfer::Chamfer;
use crate::plane::WorkPlane;
use crate::sketch::SegmentId;

const CORNER: DVec2 = DVec2::new(2.0, 1.0);

/// A right angle, one side running east and the other north.
fn a_right_angle() -> (Sketch, SegmentId, SegmentId, PointId) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let pivot = sketch.add_point(CORNER);
    let east = sketch.add_point(CORNER + DVec2::new(10.0, 0.0));
    let north = sketch.add_point(CORNER + DVec2::new(0.0, 10.0));
    let along = sketch.add_segment(pivot, east);
    let up = sketch.add_segment(pivot, north);
    (sketch, along, up, pivot)
}

#[test]
fn a_point_where_two_traits_meet_is_a_corner_and_names_them_both() {
    let (sketch, along, up, pivot) = a_right_angle();

    assert_eq!(
        sketch.sides_of(Corner::At(pivot)),
        Some((pivot, CutSide::Trait(along), CutSide::Trait(up)))
    );
}

#[test]
fn a_point_where_a_third_trait_runs_in_is_no_corner_to_take() {
    let (mut sketch, _along, _up, pivot) = a_right_angle();
    let away = sketch.add_point(CORNER + DVec2::new(-10.0, -10.0));
    sketch.add_segment(pivot, away);

    assert_eq!(
        sketch.sides_of(Corner::At(pivot)),
        None,
        "three traits leave no saying which two the cut is meant for"
    );
}

#[test]
fn an_end_of_a_single_trait_is_no_corner_to_take() {
    let (sketch, along, _up, _pivot) = a_right_angle();
    let lonely = sketch.segments()[along.0].end;

    assert_eq!(sketch.sides_of(Corner::At(lonely)), None);
}

#[test]
fn a_point_where_a_curve_runs_in_beside_two_traits_is_no_corner_to_take() {
    let (mut sketch, _along, _up, pivot) = a_right_angle();
    let centre = sketch.add_point(CORNER + DVec2::new(-5.0, 0.0));
    let far = sketch.add_point(CORNER + DVec2::new(-10.0, 0.0));
    sketch.add_arc(centre, pivot, far);

    assert_eq!(
        sketch.sides_of(Corner::At(pivot)),
        None,
        "two traits and a curve ending here are three sides, and leave no saying \
         which two the cut is meant for"
    );
}

#[test]
fn a_trait_erased_no_longer_makes_a_corner_of_the_point_it_left() {
    let (mut sketch, _along, up, pivot) = a_right_angle();

    sketch.erase(crate::sketch::Element::Segment(up));

    assert_eq!(sketch.sides_of(Corner::At(pivot)), None);
}

#[test]
fn a_corner_clicked_again_names_the_trait_that_was_not_named_yet() {
    let (sketch, along, up, pivot) = a_right_angle();

    let side = CutSide::Trait;
    assert_eq!(sketch.other_side_at(pivot, side(along)), Some(side(up)));
    assert_eq!(
        sketch.other_side_at(pivot, side(up)),
        Some(side(along)),
        "with one side already named, the other is the only choice left"
    );
}

#[test]
fn a_point_that_makes_no_corner_names_no_other_trait() {
    let (mut sketch, along, _up, pivot) = a_right_angle();
    let away = sketch.add_point(CORNER + DVec2::new(-10.0, -10.0));
    sketch.add_segment(pivot, away);

    assert_eq!(
        sketch.other_side_at(pivot, CutSide::Trait(along)),
        None,
        "three traits leave two candidates, which is a guess"
    );
}

#[test]
fn a_corner_named_either_way_stands_at_the_same_point() {
    let (sketch, along, up, pivot) = a_right_angle();

    assert_eq!(sketch.corner_point(Corner::At(pivot)), Some(pivot));
    assert_eq!(
        sketch.corner_point(Corner::Between(along.into(), up.into())),
        Some(pivot)
    );
}

#[test]
fn a_corner_already_cut_still_names_its_two_traits() {
    let (mut sketch, along, up, pivot) = a_right_angle();

    sketch
        .cut_corner(
            along,
            up,
            crate::corner_cut::CutShape::Chamfer(Chamfer::Equal(3.0)),
            false,
            0,
        )
        .expect("a corner that can be cut");

    assert_eq!(
        sketch.sides_of(Corner::At(pivot)),
        Some((pivot, CutSide::Trait(along), CutSide::Trait(up))),
        "the cut leaves both traits whole and meeting where they met"
    );
}

#[test]
fn a_corner_of_two_construction_traits_is_not_offered_by_its_point() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let pivot = sketch.add_point(CORNER);
    let east = sketch.add_point(CORNER + DVec2::new(10.0, 0.0));
    let north = sketch.add_point(CORNER + DVec2::new(0.0, 10.0));
    sketch.add_construction_segment(pivot, east);
    sketch.add_construction_segment(pivot, north);

    assert_eq!(
        sketch.sides_of(Corner::At(pivot)),
        None,
        "what a point offers is the corner of the shape; construction traits \
         are still named one by one"
    );
}
