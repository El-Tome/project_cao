//! What sketch · precision.rs is held to.
//!
//! Closes #515.
//! - a `precision` module holds the three levels, a decade apart, `USE` the
//!   loosest and `SOLVED` the finest — `the_three_levels_stand_a_decade_apart`
//! - one function says how big a drawing is: the diagonal of the box holding
//!   every live curve by its extent —
//!   `a_circle_counts_by_its_extent_not_its_centre`,
//!   `an_arc_counts_only_the_stretch_that_is_drawn`,
//!   `a_tilted_ellipse_counts_by_its_own_reach_not_its_axis_ends`,
//!   `a_drawing_with_no_curve_measures_one_unit`,
//!   `a_collapsed_drawing_measures_one_unit`
//! - construction curves are included and the origin is left out, so a part
//!   drawn a metre out is held like the same part drawn on the origin —
//!   `a_construction_curve_counts_toward_the_size`,
//!   `a_drawing_far_from_the_origin_measures_as_on_it`
//! - an erased curve leaves the size — `an_erased_curve_leaves_the_size`
//! - the campaign reads the constants instead of copying them — no test:
//!   `random_sketches/drawing.rs` and `random_sketches/enclosing.rs` import
//!   them, which the compiler holds
//! - the comments and documents that misstated a figure are put right — no
//!   test: prose
//! - no answer the drawing gives changes — no test: nothing in the crate reads
//!   the module yet, and the random-drawing campaign gave the same verdict on
//!   the same seeds before and after

use glam::DVec2;

use super::*;
use crate::WorkPlane;
use crate::element::Element;
use crate::sketch::Sketch;

const TOLERANCE: f64 = 1e-9;

fn measures(sketch: &Sketch, expected: f64) {
    let size = drawing_size(sketch);
    assert!(
        (size - expected).abs() < TOLERANCE,
        "the drawing measures {size} where {expected} was expected",
    );
}

#[test]
fn a_drawing_with_no_curve_measures_one_unit() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    measures(&sketch, 1.0);

    sketch.add_point(DVec2::new(500.0, -300.0));
    measures(&sketch, 1.0);
}

fn a_square_drawn_at(corner: DVec2) -> Sketch {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let places = [
        corner,
        corner + DVec2::new(3.0, 0.0),
        corner + DVec2::new(3.0, 3.0),
        corner + DVec2::new(0.0, 3.0),
    ];
    let points = places.map(|place| sketch.add_point(place));
    for side in 0..4 {
        sketch.add_segment(points[side], points[(side + 1) % 4]);
    }
    sketch
}

#[test]
fn a_drawing_far_from_the_origin_measures_as_on_it() {
    let expected = DVec2::splat(3.0).length();
    measures(&a_square_drawn_at(DVec2::ZERO), expected);
    measures(&a_square_drawn_at(DVec2::new(1000.0, -2000.0)), expected);
}

#[test]
fn a_collapsed_drawing_measures_one_unit() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let from = sketch.add_point(DVec2::new(7.0, 7.0));
    let to = sketch.add_point(DVec2::new(7.0, 7.0));
    sketch.add_segment(from, to);
    measures(&sketch, 1.0);
}

#[test]
fn a_circle_counts_by_its_extent_not_its_centre() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::ZERO);
    sketch.add_circle(centre, 5.0);
    measures(&sketch, DVec2::splat(10.0).length());
}

#[test]
fn an_arc_counts_only_the_stretch_that_is_drawn() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::ZERO);
    let start = sketch.add_point(DVec2::new(10.0, 0.0));
    let end = sketch.add_point(DVec2::new(0.0, 10.0));
    sketch.add_arc(centre, start, end);
    measures(&sketch, DVec2::splat(10.0).length());
}

#[test]
fn a_tilted_ellipse_counts_by_its_own_reach_not_its_axis_ends() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::ZERO);
    let along = DVec2::splat(1.0).normalize();
    let across = along.perp();
    let first = [along * -4.0, along * 4.0].map(|place| sketch.add_point(place));
    let second = [across * -2.0, across * 2.0].map(|place| sketch.add_point(place));
    sketch.add_ellipse(centre, first, second);
    measures(&sketch, 2.0 * DVec2::new(4.0, 2.0).length());
}

#[test]
fn a_construction_curve_counts_toward_the_size() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let from = sketch.add_point(DVec2::new(10.0, 0.0));
    let to = sketch.add_point(DVec2::new(13.0, 4.0));
    sketch.add_construction_segment(from, to);
    measures(&sketch, 5.0);
}

#[test]
fn an_erased_curve_leaves_the_size() {
    let mut sketch = a_square_drawn_at(DVec2::ZERO);
    let from = sketch.add_point(DVec2::new(100.0, 0.0));
    let to = sketch.add_point(DVec2::new(200.0, 0.0));
    let far = sketch.add_segment(from, to);
    sketch.erase(Element::Segment(far));
    measures(&sketch, DVec2::splat(3.0).length());
}

#[test]
fn the_three_levels_stand_a_decade_apart() {
    const {
        assert!(SOLVED < SAME && SAME < USE);
        assert!((USE / SAME - 10.0).abs() < 1e-9);
        assert!((SAME / SOLVED - 10.0).abs() < 1e-9);
    }
}
