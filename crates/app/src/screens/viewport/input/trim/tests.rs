//! What app · screens/viewport/input/trim.rs is held to.

use super::*;
use cao_sketch::WorkPlane;

/// A trait laid across the top of a curve that grazes it, so that one click
/// is within reach of both.
fn a_trait_touching_a_curve() -> Sketch {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let left = sketch.add_point(DVec2::ZERO);
    let right = sketch.add_point(DVec2::new(10.0, 0.0));
    sketch.add_segment(left, right);
    let centre = sketch.add_point(DVec2::new(5.0, -5.0));
    let east = sketch.add_point(DVec2::new(10.0, -5.0));
    let west = sketch.add_point(DVec2::new(0.0, -5.0));
    sketch.add_arc(centre, east, west);
    sketch
}

#[test]
fn a_click_within_reach_of_both_a_trait_and_a_curve_cuts_the_trait() {
    let sketch = a_trait_touching_a_curve();

    let cut = cut_under(&sketch, 0, DVec2::new(5.0, 0.0), 0.5);

    assert!(
        matches!(cut, Some(Operation::Trim { .. })),
        "the click cut {cut:?}",
    );
}

#[test]
fn a_click_the_trait_is_out_of_reach_of_cuts_the_curve() {
    let sketch = a_trait_touching_a_curve();

    let cut = cut_under(&sketch, 0, DVec2::new(1.47, -1.47), 0.5);

    assert!(
        matches!(cut, Some(Operation::TrimArc { .. })),
        "the click cut {cut:?}",
    );
}

#[test]
fn a_click_on_a_round_neither_a_trait_nor_a_curve_reaches_cuts_the_circle() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(40.0, 0.0));
    sketch.add_circle(centre, 10.0);

    let cut = cut_under(&sketch, 0, DVec2::new(50.0, 0.0), 0.5);

    assert!(
        matches!(cut, Some(Operation::TrimCircle { between: None, .. })),
        "a round with nothing on it goes whole, and the click says so: {cut:?}",
    );
}
