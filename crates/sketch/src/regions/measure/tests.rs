//! What sketch · regions/measure.rs is held to.
//!
//! Closes #425.
//! - an area bounded by a curve reports what the curve's own formula says, and
//!   not what the polygon drawn through its steps does —
//!   `a_circle_holds_the_surface_its_own_formula_says`,
//!   `a_circle_is_as_far_round_as_its_own_formula_says`,
//!   `an_ellipse_holds_the_surface_its_own_formula_says`
//! - and that is true of a piece of one as well as a whole one —
//!   `half_a_circle_holds_half_the_surface_and_is_as_far_round_as_its_arc_and_its_chord`
//! - a straight-sided area is still read exactly —
//!   `a_rectangle_holds_what_its_two_sides_multiply_to`
//! - an area with a hole in it reports the surface without the hole, and a
//!   perimeter that is the outer boundary only —
//!   `a_hole_comes_out_of_the_surface_and_is_left_out_of_the_way_round`
//! - the sampling it was drawn with does not change the answer —
//!   `how_finely_a_curve_was_sampled_does_not_move_the_answer`
//!
//! Closes #428.
//! - a run's number is where its bend lands, so the two cannot drift apart —
//!   `every_run_of_an_outline_names_a_bend_the_outline_has`

use std::f64::consts::{PI, TAU};

use super::*;
use crate::ellipsing::EllipseDraft;
use crate::plane::WorkPlane;
use crate::sketch::Sketch;

/// A tenth of a micron on a part measured in millimetres: far under anything
/// the drawing is worth, and far over what the arithmetic loses.
const TOLERANCE: f64 = 1e-7;

/// The one area of a drawing, when it has exactly one.
fn only_area(sketch: &Sketch) -> Region {
    let mut regions = sketch.regions();
    assert_eq!(regions.len(), 1, "the drawing was meant to close one area");
    regions.remove(0)
}

fn a_circle_of(radius: f64) -> Sketch {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(3.0, -4.0));
    sketch.add_circle(centre, radius);
    sketch
}

#[test]
fn a_circle_holds_the_surface_its_own_formula_says() {
    let held = only_area(&a_circle_of(10.0)).area();

    assert!(
        (held - PI * 100.0).abs() < TOLERANCE,
        "a circle of ten holds {}, and its own steps would have said about \
         313.9 — a measure that answers the polygon is one nobody trusts twice",
        held,
    );
}

#[test]
fn a_circle_is_as_far_round_as_its_own_formula_says() {
    let round = only_area(&a_circle_of(10.0)).perimeter();

    assert!(
        (round - TAU * 10.0).abs() < TOLERANCE,
        "a circle of ten is {round} round, where its steps would say about 62.79",
    );
}

#[test]
fn how_finely_a_curve_was_sampled_does_not_move_the_answer() {
    let small = only_area(&a_circle_of(0.5));
    let large = only_area(&a_circle_of(500.0));

    assert!(
        (small.area() / 0.25 - large.area() / 250_000.0).abs() < TOLERANCE,
        "two circles of very different size, and so of very different step \
         counts, hold the same surface for their radius: {} against {}",
        small.area() / 0.25,
        large.area() / 250_000.0,
    );
}

#[test]
fn a_rectangle_holds_what_its_two_sides_multiply_to() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corners: Vec<_> = [(0.0, 0.0), (40.0, 0.0), (40.0, 25.0), (0.0, 25.0)]
        .into_iter()
        .map(|(x, y)| sketch.add_point(DVec2::new(x, y)))
        .collect();
    for pair in [(0, 1), (1, 2), (2, 3), (3, 0)] {
        sketch.add_segment(corners[pair.0], corners[pair.1]);
    }

    let area = only_area(&sketch);

    assert!(
        (area.area() - 1000.0).abs() < TOLERANCE,
        "forty by twenty-five, got {}",
        area.area(),
    );
    assert!(
        (area.perimeter() - 130.0).abs() < TOLERANCE,
        "twice forty and twice twenty-five, got {}",
        area.perimeter(),
    );
}

#[test]
fn a_hole_comes_out_of_the_surface_and_is_left_out_of_the_way_round() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::ZERO);
    sketch.add_circle(centre, 10.0);
    let inner = sketch.add_point(DVec2::ZERO);
    sketch.add_circle(inner, 4.0);

    let regions = sketch.regions();
    let ring = regions
        .iter()
        .find(|region| !region.holes.is_empty())
        .expect("the outer circle is hollow of the inner one");

    assert!(
        (ring.area() - PI * (100.0 - 16.0)).abs() < TOLERANCE,
        "the surface an extrusion would make matter of, got {}",
        ring.area(),
    );
    assert!(
        (ring.perimeter() - TAU * 10.0).abs() < TOLERANCE,
        "the outside only: the hole is an area of its own, and is read by \
         clicking inside it. Got {}",
        ring.perimeter(),
    );
}

#[test]
fn half_a_circle_holds_half_the_surface_and_is_as_far_round_as_its_arc_and_its_chord() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::ZERO);
    sketch.add_circle(centre, 10.0);
    let left = sketch.add_point(DVec2::new(-10.0, 0.0));
    let right = sketch.add_point(DVec2::new(10.0, 0.0));
    sketch.add_segment(left, right);

    let halves = sketch.regions();
    assert_eq!(halves.len(), 2, "a chord through the middle leaves two");

    for half in &halves {
        assert!(
            (half.area() - PI * 50.0).abs() < 1e-6,
            "half a circle of ten, got {}",
            half.area(),
        );
        assert!(
            (half.perimeter() - (PI * 10.0 + 20.0)).abs() < 1e-6,
            "half the way round, and the chord across, got {}",
            half.perimeter(),
        );
    }
}

#[test]
fn an_ellipse_holds_the_surface_its_own_formula_says() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::ZERO);
    let first = [
        sketch.add_point(DVec2::new(20.0, 0.0)),
        sketch.add_point(DVec2::new(-20.0, 0.0)),
    ];
    let second = [
        sketch.add_point(DVec2::new(0.0, 8.0)),
        sketch.add_point(DVec2::new(0.0, -8.0)),
    ];
    sketch.add_ellipse(centre, first, second);

    let oval = only_area(&sketch);

    assert!(
        (oval.area() - PI * 20.0 * 8.0).abs() < 1e-6,
        "pi times the two half-axes, got {}",
        oval.area(),
    );
}

#[test]
fn an_ellipse_is_as_far_round_as_the_integral_nobody_can_write_in_closed_form() {
    let drawn = EllipseDraft {
        centre: DVec2::ZERO,
        first: DVec2::new(20.0, 0.0),
        second: 8.0,
    };

    let round = drawn.run_of(0.0, TAU);

    // Ramanujan's second approximation, which is good to about one part in a
    // hundred million on an ellipse this round — near enough to catch an
    // integral that has gone wrong, and no use at all as the answer itself.
    let (a, b) = (20.0_f64, 8.0_f64);
    let h = ((a - b) / (a + b)).powi(2);
    let ramanujan = PI * (a + b) * (1.0 + 3.0 * h / (10.0 + (4.0 - 3.0 * h).sqrt()));

    assert!(
        (round - ramanujan).abs() < 1e-4,
        "the integral says {round} and Ramanujan {ramanujan}",
    );
}

#[test]
fn a_circle_drawn_as_an_ellipse_is_as_far_round_as_a_circle() {
    let drawn = EllipseDraft {
        centre: DVec2::new(2.0, 5.0),
        first: DVec2::new(7.0, 0.0),
        second: 7.0,
    };

    assert!(
        (drawn.run_of(0.0, TAU) - TAU * 7.0).abs() < 1e-9,
        "an ellipse with two equal axes is a circle, and the integral has to \
         agree with the one case that does have a closed form: got {}",
        drawn.run_of(0.0, TAU),
    );
}

#[test]
fn an_outline_of_nothing_at_all_measures_nothing_rather_than_falling_over() {
    let empty = Outline::default();

    assert_eq!(empty.area(), 0.0);
    assert_eq!(empty.perimeter(), 0.0);
}

#[test]
fn every_run_of_an_outline_names_a_bend_the_outline_has() {
    // The invariant the measurement reads `bends` by. Held by construction in
    // `arc_regions`, and asserted here across every shape that builds one a
    // different way: a whole curve, a cut one, and two of the same curve in
    // one loop.
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::ZERO);
    sketch.add_circle(centre, 10.0);
    let left = sketch.add_point(DVec2::new(-10.0, 0.0));
    let right = sketch.add_point(DVec2::new(10.0, 0.0));
    sketch.add_segment(left, right);
    let far = sketch.add_point(DVec2::new(40.0, 0.0));
    sketch.add_circle(far, 6.0);

    let regions = sketch.regions();
    assert!(regions.len() >= 3, "several shapes, built several ways");
    for region in &regions {
        for outline in std::iter::once(&region.outline).chain(&region.holes) {
            for run in outline.curves.iter().flatten() {
                assert!(
                    *run < outline.bends.len(),
                    "run {run} names no bend of the {} this outline carries; a \
                     run whose bend went missing would be read as the steps it \
                     was sampled into, which is the one answer this file exists \
                     to improve on",
                    outline.bends.len(),
                );
            }
        }
    }
}
