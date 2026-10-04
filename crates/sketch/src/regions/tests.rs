//! What the walk of a drawing encloses.
//!
//! Closes #493.
//! - two rectangles whose sides lie along each other over part of their length
//!   are two areas, each of its own size —
//!   `two_rectangles_whose_sides_lie_along_each_other_are_two_areas`
//! - every row of the issue's table gets its count of areas, each tinted as it
//!   encloses — `a_short_side_lying_wholly_along_a_longer_one_leaves_both_areas`,
//!   `a_rectangle_overhanging_another_s_top_is_an_area_of_its_own`,
//!   `two_rectangles_overlapping_on_one_baseline_leave_what_they_share_an_area`,
//!   `a_side_drawn_again_corner_to_corner_leaves_the_rectangle_its_area`,
//!   `a_trait_laid_along_part_of_a_side_leaves_the_rectangle_its_area`,
//!   `an_arc_drawn_along_part_of_its_circle_leaves_the_disc_its_area`,
//!   `a_triangle_whose_tip_touches_a_side_is_an_area_of_its_own`,
//!   `two_rectangles_sharing_a_whole_side_each_with_corners_of_its_own_are_two_areas`;
//!   laid again in another order, moved and turned, in
//!   `crates/sketch/tests/curves_lying_along_each_other.rs`
//! - a circle drawn twice on one centre is one area —
//!   `a_circle_drawn_twice_on_one_centre_encloses_one_disc`
//! - nothing under *What must not break* breaks — no test: it has none of its
//!   own, and is held by the suites already there, this file and
//!   `arc_regions/tests.rs` among them, and the tests of #351, #266, #413 and
//!   #345

use super::*;
use crate::plane::WorkPlane;
use crate::sketch::PointId;

fn rectangle(sketch: &mut Sketch, min: DVec2, max: DVec2) {
    let corners = [
        sketch.add_point(min),
        sketch.add_point(DVec2::new(max.x, min.y)),
        sketch.add_point(max),
        sketch.add_point(DVec2::new(min.x, max.y)),
    ];
    for index in 0..4 {
        sketch.add_segment(corners[index], corners[(index + 1) % 4]);
    }
}

#[test]
fn an_open_shape_encloses_nothing() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let a = sketch.add_point(DVec2::ZERO);
    let b = sketch.add_point(DVec2::new(10.0, 0.0));
    let c = sketch.add_point(DVec2::new(10.0, 10.0));
    sketch.add_segment(a, b);
    sketch.add_segment(b, c);
    assert!(sketch.regions().is_empty());
}

#[test]
fn a_closed_contour_is_one_region() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    rectangle(&mut sketch, DVec2::ZERO, DVec2::new(10.0, 4.0));
    let regions = sketch.regions();
    assert_eq!(regions.len(), 1);
    assert_eq!(regions[0].depth, 0);
    let area: f64 = regions[0]
        .triangles
        .iter()
        .map(|[a, b, c]| (b - a).perp_dot(c - a).abs() * 0.5)
        .sum();
    assert!((area - 40.0).abs() < 1e-3, "area {area}");
}

#[test]
fn a_shape_inside_another_is_one_level_deeper() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    rectangle(&mut sketch, DVec2::ZERO, DVec2::new(20.0, 20.0));
    rectangle(&mut sketch, DVec2::new(5.0, 5.0), DVec2::new(10.0, 10.0));
    let regions = sketch.regions();
    assert_eq!(regions.len(), 2);
    assert_eq!(regions[0].depth, 0);
    assert_eq!(regions[1].depth, 1);
}

#[test]
fn two_shapes_sharing_a_side_are_two_regions() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let a = sketch.add_point(DVec2::ZERO);
    let b = sketch.add_point(DVec2::new(10.0, 0.0));
    let c = sketch.add_point(DVec2::new(10.0, 10.0));
    let d = sketch.add_point(DVec2::new(0.0, 10.0));
    let e = sketch.add_point(DVec2::new(20.0, 0.0));
    let f = sketch.add_point(DVec2::new(20.0, 10.0));
    for (from, to) in [(a, b), (b, c), (c, d), (d, a), (b, e), (e, f), (f, c)] {
        sketch.add_segment(from, to);
    }
    let regions = sketch.regions();
    assert_eq!(regions.len(), 2);
    assert!(regions.iter().all(|region| region.depth == 0));
}

fn area(triangles: &[[DVec2; 3]]) -> f64 {
    triangles
        .iter()
        .map(|[a, b, c]| (b - a).perp_dot(c - a).abs() * 0.5)
        .sum()
}

/// Two circles one inside the other are a tube, not a rod: the face keeps
/// the middle hollow.
#[test]
fn a_shape_inside_another_is_a_hole_in_its_face() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    rectangle(&mut sketch, DVec2::ZERO, DVec2::new(20.0, 20.0));
    rectangle(&mut sketch, DVec2::new(5.0, 5.0), DVec2::new(15.0, 15.0));
    let regions = sketch.regions();

    assert_eq!(regions[0].holes.len(), 1, "the outer contour is pierced");
    assert!(regions[1].holes.is_empty());

    let ring = area(&regions[0].face_triangles());
    assert!((ring - 300.0).abs() < 1e-2, "area of the ring: {ring}");
    assert!(
        (area(&regions[0].triangles) - 400.0).abs() < 1e-2,
        "the solid fill ignores the hole"
    );

    assert!(regions[0].contains(DVec2::new(2.0, 2.0)));
    assert!(
        !regions[0].contains(DVec2::new(10.0, 10.0)),
        "the hole is empty"
    );
    assert!(regions[1].contains(DVec2::new(10.0, 10.0)));
}

/// Matter inside a hole is matter again, and belongs to its own face.
#[test]
fn a_shape_inside_a_hole_is_not_a_hole_of_the_outer_one() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    rectangle(&mut sketch, DVec2::ZERO, DVec2::new(30.0, 30.0));
    rectangle(&mut sketch, DVec2::new(5.0, 5.0), DVec2::new(25.0, 25.0));
    rectangle(&mut sketch, DVec2::new(10.0, 10.0), DVec2::new(20.0, 20.0));
    let regions = sketch.regions();

    assert_eq!(regions[0].holes.len(), 1);
    assert_eq!(regions[1].holes.len(), 1);
    assert!(regions[2].holes.is_empty());
    assert!((area(&regions[2].face_triangles()) - 100.0).abs() < 1e-2);
}

/// Le cas cité : deux cercles concentriques font un tube.
#[test]
fn two_circles_make_a_tube() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let center = sketch.add_point(DVec2::new(10.0, 10.0));
    sketch.add_circle(center, 8.0);
    sketch.add_circle(center, 5.0);

    let regions = sketch.regions();
    assert_eq!(regions.len(), 2);
    let ring = area(&regions[0].face_triangles());
    let expected = std::f64::consts::PI * (8.0f64.powi(2) - 5.0f64.powi(2));
    assert!(
        (ring - expected).abs() / expected < 0.02,
        "ring {ring}, expected ~{expected}"
    );
    assert!(!regions[0].contains(DVec2::new(10.0, 10.0)));
}

#[test]
fn a_circle_encloses_its_disc() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let center = sketch.add_point(DVec2::new(3.0, 3.0));
    sketch.add_circle(center, 2.0);
    let regions = sketch.regions();
    assert_eq!(regions.len(), 1);
    assert!(encloses(&regions[0].outline.points, DVec2::new(3.0, 3.0)));
}

/// What each area's triangles cover, smallest first, once each is checked to
/// cover exactly what its outline encloses: a loop running over a stretch
/// more than once is cut into triangles lying over each other, and covers
/// more.
fn tinted(sketch: &Sketch) -> Vec<f64> {
    let mut covered: Vec<f64> = sketch
        .regions()
        .iter()
        .map(|region| {
            let (tint, enclosed) = (
                area(&region.triangles),
                signed_area(&region.outline.points).abs(),
            );
            assert!(
                (tint - enclosed).abs() < 1e-9,
                "tinted {tint} where the outline encloses {enclosed}"
            );
            tint
        })
        .collect();
    covered.sort_by(f64::total_cmp);
    covered
}

fn assert_tinted(sketch: &Sketch, expected: &[f64]) {
    let covered = tinted(sketch);
    assert_eq!(
        covered.len(),
        expected.len(),
        "areas {covered:?}, expected {expected:?}"
    );
    assert!(
        covered
            .iter()
            .zip(expected)
            .all(|(found, wanted)| (found - wanted).abs() < 1e-9),
        "areas {covered:?}, expected {expected:?}"
    );
}

#[test]
fn two_rectangles_whose_sides_lie_along_each_other_are_two_areas() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    rectangle(&mut sketch, DVec2::ZERO, DVec2::new(1.0, 2.0));
    rectangle(&mut sketch, DVec2::new(1.0, 1.0), DVec2::new(2.0, 3.0));
    let regions = sketch.regions();
    assert_eq!(regions.len(), 2);
    assert!(
        regions
            .iter()
            .all(|region| (area(&region.triangles) - 2.0).abs() < 1e-9)
    );
    assert_tinted(&sketch, &[2.0, 2.0]);
}

#[test]
fn a_short_side_lying_wholly_along_a_longer_one_leaves_both_areas() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    rectangle(&mut sketch, DVec2::ZERO, DVec2::new(1.0, 3.0));
    rectangle(&mut sketch, DVec2::new(1.0, 1.0), DVec2::new(2.0, 2.0));
    assert_tinted(&sketch, &[1.0, 3.0]);
}

#[test]
fn a_rectangle_overhanging_another_s_top_is_an_area_of_its_own() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    rectangle(&mut sketch, DVec2::ZERO, DVec2::new(2.0, 2.0));
    rectangle(&mut sketch, DVec2::new(1.0, 2.0), DVec2::new(3.0, 3.0));
    assert_tinted(&sketch, &[2.0, 4.0]);
}

#[test]
fn two_rectangles_overlapping_on_one_baseline_leave_what_they_share_an_area() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    rectangle(&mut sketch, DVec2::ZERO, DVec2::new(2.0, 1.0));
    rectangle(&mut sketch, DVec2::new(1.0, 0.0), DVec2::new(3.0, 2.0));
    assert_tinted(&sketch, &[1.0, 1.0, 3.0]);
}

#[test]
fn a_side_drawn_again_corner_to_corner_leaves_the_rectangle_its_area() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corners = [
        sketch.add_point(DVec2::ZERO),
        sketch.add_point(DVec2::new(2.0, 0.0)),
        sketch.add_point(DVec2::new(2.0, 2.0)),
        sketch.add_point(DVec2::new(0.0, 2.0)),
    ];
    for index in 0..4 {
        sketch.add_segment(corners[index], corners[(index + 1) % 4]);
    }
    sketch.add_segment(corners[2], corners[1]);
    assert_tinted(&sketch, &[4.0]);
}

#[test]
fn a_trait_laid_along_part_of_a_side_leaves_the_rectangle_its_area() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    rectangle(&mut sketch, DVec2::ZERO, DVec2::new(2.0, 2.0));
    let from = sketch.add_point(DVec2::new(2.0, 1.0));
    let to = sketch.add_point(DVec2::new(2.0, 3.0));
    sketch.add_segment(from, to);
    assert_tinted(&sketch, &[4.0]);
}

/// The one area a drawing of curves encloses, held against what the curves
/// themselves enclose: tinted as the straight steps they are sampled into, it
/// falls a little short of it.
fn assert_one_curved_area(sketch: &Sketch, enclosed: f64) {
    let covered = tinted(sketch);
    assert_eq!(covered.len(), 1, "areas {covered:?}");
    assert!(
        (covered[0] - enclosed).abs() / enclosed < 0.02,
        "tinted {}, where the curves enclose {enclosed}",
        covered[0]
    );
}

/// The centre of an ellipse about the origin and the ends of its two axes,
/// west, east, south and north, each a point of its own.
fn axis_ends(sketch: &mut Sketch, along: f64, across: f64) -> [PointId; 5] {
    [
        DVec2::ZERO,
        DVec2::new(-along, 0.0),
        DVec2::new(along, 0.0),
        DVec2::new(0.0, -across),
        DVec2::new(0.0, across),
    ]
    .map(|place| sketch.add_point(place))
}

#[test]
fn an_arc_drawn_along_part_of_its_circle_leaves_the_disc_its_area() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::ZERO);
    sketch.add_circle(centre, 2.0);
    let start = sketch.add_point(DVec2::new(2.0, 0.0));
    let end = sketch.add_point(DVec2::new(0.0, 2.0));
    sketch.add_arc(centre, start, end);
    assert_one_curved_area(&sketch, std::f64::consts::PI * 4.0);
}

/// Nothing cuts either circle, so neither has a vertex: the two never meet as
/// pieces, and are held against each other as whole loops.
#[test]
fn a_circle_drawn_twice_on_one_centre_encloses_one_disc() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    for _ in 0..2 {
        let centre = sketch.add_point(DVec2::new(8.0, 4.0));
        sketch.add_circle(centre, 2.0);
    }
    assert_one_curved_area(&sketch, std::f64::consts::PI * 4.0);
}

/// The same ellipse drawn from its other axis, that axis laid the other way:
/// the numbers describing the two differ, the curve does not.
#[test]
fn an_ellipse_drawn_again_from_its_other_axis_encloses_one_area() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let [centre, west, east, south, north] = axis_ends(&mut sketch, 3.0, 1.0);
    sketch.add_ellipse(centre, [west, east], [south, north]);
    let [centre, west, east, south, north] = axis_ends(&mut sketch, 3.0, 1.0);
    sketch.add_ellipse(centre, [north, south], [east, west]);
    assert_one_curved_area(&sketch, std::f64::consts::PI * 3.0);
}

#[test]
fn half_an_ellipse_laid_along_its_whole_ellipse_encloses_one_area() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let [centre, west, east, south, north] = axis_ends(&mut sketch, 3.0, 1.0);
    sketch.add_ellipse(centre, [west, east], [south, north]);
    let [centre, west, east, south, north] = axis_ends(&mut sketch, 3.0, 1.0);
    let half = sketch.add_ellipse(centre, [west, east], [south, north]);
    sketch.draw_the_stretch(half, east, west);
    assert_one_curved_area(&sketch, std::f64::consts::PI * 3.0);
}

#[test]
fn a_triangle_whose_tip_touches_a_side_is_an_area_of_its_own() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    rectangle(&mut sketch, DVec2::ZERO, DVec2::new(4.0, 2.0));
    let corners = [
        sketch.add_point(DVec2::new(2.0, 2.0)),
        sketch.add_point(DVec2::new(3.0, 4.0)),
        sketch.add_point(DVec2::new(1.0, 4.0)),
    ];
    for index in 0..3 {
        sketch.add_segment(corners[index], corners[(index + 1) % 3]);
    }
    assert_tinted(&sketch, &[2.0, 8.0]);
}

#[test]
fn two_rectangles_sharing_a_whole_side_each_with_corners_of_its_own_are_two_areas() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    rectangle(&mut sketch, DVec2::ZERO, DVec2::new(1.0, 2.0));
    rectangle(&mut sketch, DVec2::new(1.0, 0.0), DVec2::new(2.0, 2.0));
    assert_tinted(&sketch, &[2.0, 2.0]);
}

/// A circle about the origin, and an ellipse whose two axes reach just as far
/// laid on it: one curve drawn by two tools.
fn a_circle_and_a_round_ellipse_on_it(sketch: &mut Sketch) {
    let [centre, west, east, south, north] = axis_ends(sketch, 2.0, 2.0);
    sketch.add_circle(centre, 2.0);
    sketch.add_ellipse(centre, [west, east], [south, north]);
}

#[test]
fn a_round_ellipse_laid_on_a_circle_crosses_it_nowhere() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    a_circle_and_a_round_ellipse_on_it(&mut sketch);
    let crossings = sketch.crossings();
    assert!(
        crossings.is_empty(),
        "one curve drawn by two tools crosses itself nowhere, not {} times",
        crossings.len(),
    );
}

#[test]
fn a_round_ellipse_laid_on_a_circle_encloses_one_disc() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    a_circle_and_a_round_ellipse_on_it(&mut sketch);
    assert_one_curved_area(&sketch, std::f64::consts::PI * 4.0);
}

#[test]
fn a_trait_across_a_round_ellipse_laid_on_a_circle_leaves_two_halves() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    a_circle_and_a_round_ellipse_on_it(&mut sketch);
    let low = sketch.add_point(DVec2::new(0.5, -3.0));
    let high = sketch.add_point(DVec2::new(0.5, 3.0));
    sketch.add_segment(low, high);
    let covered = tinted(&sketch);
    assert_eq!(covered.len(), 2, "areas {covered:?}");
    let disc = std::f64::consts::PI * 4.0;
    assert!(
        (covered.iter().sum::<f64>() - disc).abs() / disc < 0.02,
        "the two halves make the disc between them: {covered:?}"
    );
}

/// The arc runs past two of the ellipse's own handles, and is cut at them
/// where the ellipse, broken by the arc's ends, is cut too.
#[test]
fn an_arc_drawn_along_a_round_ellipse_leaves_it_its_area() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let [centre, west, east, south, north] = axis_ends(&mut sketch, 2.0, 2.0);
    sketch.add_ellipse(centre, [west, east], [south, north]);
    let start = sketch.add_point(DVec2::new(2.0, 0.0).rotate(DVec2::from_angle(-0.5)));
    let end = sketch.add_point(DVec2::new(-2.0, 0.0).rotate(DVec2::from_angle(0.5)));
    sketch.add_arc(centre, start, end);
    assert_eq!(sketch.crossings(), Vec::<DVec2>::new());
    assert_one_curved_area(&sketch, std::f64::consts::PI * 4.0);
}
