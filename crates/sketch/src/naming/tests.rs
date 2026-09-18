//! What naming an area by the curves that bound it is held to.
//!
//! Closes #345.
//! - an area whose drawing is dragged raises the same matter as before —
//!   `a_name_holds_though_the_drawing_is_pulled_about`
//! - an area one of whose bounding traits is rounded raises the same matter as
//!   before — `an_area_is_found_though_rounding_a_corner_gave_it_a_curve`
//! - an area one of whose bounding traits is erased raises nothing —
//!   `a_name_no_area_answers_to_is_lost`
//! - the two halves of a circle a chord cuts are told apart, though the same
//!   two curves bound them both —
//!   `the_two_halves_of_a_cut_circle_are_told_apart`

use super::*;
use crate::plane::WorkPlane;
use crate::sketch::{Element, PointId, Sketch};

fn rectangle(sketch: &mut Sketch, min: DVec2, max: DVec2) -> Vec<SegmentId> {
    let corners = [
        sketch.add_point(min),
        sketch.add_point(DVec2::new(max.x, min.y)),
        sketch.add_point(max),
        sketch.add_point(DVec2::new(min.x, max.y)),
    ];
    (0..4)
        .map(|index| sketch.add_segment(corners[index], corners[(index + 1) % 4]))
        .collect()
}

#[test]
fn an_area_is_named_by_every_trait_that_borders_it() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let drawn = rectangle(&mut sketch, DVec2::ZERO, DVec2::new(10.0, 4.0));

    let regions = sketch.regions();

    let mut named: Vec<CurveId> = drawn.into_iter().map(CurveId::Segment).collect();
    named.sort_unstable();
    assert_eq!(regions[0].bounds(), named);
}

#[test]
fn a_circle_nothing_cuts_is_named_by_itself() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::ZERO);
    let circle = sketch.add_circle(centre, 5.0);

    let regions = sketch.regions();

    assert_eq!(regions[0].bounds(), [CurveId::Circle(circle)]);
}

#[test]
fn a_name_holds_though_the_drawing_is_pulled_about() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    rectangle(&mut sketch, DVec2::ZERO, DVec2::new(10.0, 4.0));
    let regions = sketch.regions();
    let area = Area::of(&regions[0], DVec2::new(5.0, 2.0));

    sketch.move_point(PointId(2), DVec2::new(40.0, 30.0));
    sketch.move_point(PointId(3), DVec2::new(0.0, 30.0));
    let pulled = sketch.regions();

    assert_eq!(
        area.found_in(&pulled),
        Some(0),
        "the place first clicked is nowhere near the shape now, and the \
         curves bounding it never changed",
    );
    assert!(
        !pulled[0].contains(area.inside),
        "a drawing the click still falls inside would prove nothing",
    );
}

#[test]
fn a_name_no_area_answers_to_is_lost() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let drawn = rectangle(&mut sketch, DVec2::ZERO, DVec2::new(10.0, 4.0));
    rectangle(&mut sketch, DVec2::new(20.0, 0.0), DVec2::new(30.0, 4.0));
    let regions = sketch.regions();
    let area = Area::of(&regions[0], DVec2::new(5.0, 2.0));

    sketch.erase(Element::Segment(drawn[0]));
    let left = sketch.regions();

    assert_eq!(left.len(), 1, "the second rectangle is still drawn");
    assert_eq!(
        area.found_in(&left),
        None,
        "an area that is gone answers to nothing, rather than to whatever \
         area is left",
    );
}

#[test]
fn an_area_is_found_though_rounding_a_corner_gave_it_a_curve() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let drawn = rectangle(&mut sketch, DVec2::ZERO, DVec2::new(10.0, 4.0));
    let inside = DVec2::new(5.0, 2.0);

    let rounded = sketch.fillet(drawn[0], drawn[1], 1.0).expect("a corner");
    let regions = sketch.regions();

    // What the descent will hand over: the two sides as they are left, and
    // the two traits the rounding never touched. The curve now standing where
    // the corner was descends from neither side, and is not in the name.
    let mut carried: Vec<CurveId> = rounded.pieces.into_iter().map(CurveId::Segment).collect();
    carried.extend([CurveId::Segment(drawn[2]), CurveId::Segment(drawn[3])]);
    carried.sort_unstable();
    let area = Area {
        bounds: carried,
        inside,
    };

    assert!(
        regions[0].bounds().contains(&CurveId::Arc(rounded.arc)),
        "the rounding really did put a curve in the boundary",
    );
    assert_eq!(area.found_in(&regions), Some(0));
}

#[test]
fn the_two_halves_of_a_cut_circle_are_told_apart() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::ZERO);
    sketch.add_circle(centre, 5.0);
    let left = sketch.add_point(DVec2::new(-5.0, 0.0));
    let right = sketch.add_point(DVec2::new(5.0, 0.0));
    sketch.add_segment(left, right);

    let regions = sketch.regions();
    let above = area_under(&regions, DVec2::new(0.0, 2.0)).expect("the upper half");
    let below = area_under(&regions, DVec2::new(0.0, -2.0)).expect("the lower half");

    assert_eq!(
        regions[above].bounds(),
        regions[below].bounds(),
        "the chord and the circle bound them both, which is why the place \
         clicked has to settle it",
    );
    assert_eq!(
        Area::of(&regions[above], DVec2::new(0.0, 2.0)).found_in(&regions),
        Some(above),
    );
    assert_eq!(
        Area::of(&regions[below], DVec2::new(0.0, -2.0)).found_in(&regions),
        Some(below),
    );
}

#[test]
fn a_name_holding_nothing_answers_to_no_area() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    rectangle(&mut sketch, DVec2::ZERO, DVec2::new(10.0, 4.0));

    let nameless = Area {
        bounds: Vec::new(),
        inside: DVec2::new(5.0, 2.0),
    };

    assert_eq!(
        nameless.found_in(&sketch.regions()),
        None,
        "every area is bounded by all of nothing, so an empty name would \
         otherwise take the first one it met",
    );
}
