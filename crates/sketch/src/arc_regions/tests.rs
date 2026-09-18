//! What sketch · arc_regions.rs is held to.

use super::*;
use crate::plane::WorkPlane;
use crate::sketch::Sketch;

fn area(triangles: &[[DVec2; 3]]) -> f64 {
    triangles
        .iter()
        .map(|[a, b, c]| (*b - *a).perp_dot(*c - *a).abs() * 0.5)
        .sum()
}

fn a_closed_shape() -> (Sketch, Vec<crate::sketch::PointId>) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corners: Vec<crate::sketch::PointId> = [
        (0.0, 0.0),
        (70.0, 25.0),
        (57.7, 92.7),
        (25.0, 140.0),
        (-95.0, 125.0),
        (10.0, 60.0),
        (8.1, 33.1),
    ]
    .iter()
    .map(|(x, y)| sketch.add_point(DVec2::new(*x, *y)))
    .collect();
    for rank in 0..corners.len() {
        sketch.add_segment(corners[rank], corners[(rank + 1) % corners.len()]);
    }
    (sketch, corners)
}

fn laid(sketch: &mut Sketch, from: DVec2, to: DVec2) {
    let a = sketch.add_point(from);
    let b = sketch.add_point(to);
    sketch.add_segment(a, b);
}

#[test]
fn a_trait_poking_into_a_shape_leaves_its_area_whole() {
    let (mut sketch, _) = a_closed_shape();
    let alone = sketch.regions();
    assert_eq!(alone.len(), 1);
    let whole = area(&alone[0].triangles);

    laid(&mut sketch, DVec2::new(40.0, 90.0), DVec2::new(60.0, 125.0));

    let regions = sketch.regions();
    assert_eq!(
        regions.len(),
        1,
        "a trait that encloses nothing encloses nothing: it has one end \
         inside the shape and one outside, so it cuts no second area out \
         of it",
    );
    assert_eq!(
        regions[0].outline.points.len(),
        8,
        "the shape's seven corners, and the place the trait crosses its side",
    );
    assert!(
        (area(&regions[0].triangles) - whole).abs() < 1e-6,
        "the area was {whole} before the trait was laid and {} after",
        area(&regions[0].triangles),
    );
}

#[test]
fn a_corner_dropped_exactly_on_a_trait_leaves_the_area_it_pinches() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corners: Vec<crate::sketch::PointId> = [
        (25.0, 85.0),
        (37.5, 115.0),
        (20.0, 145.0),
        (-35.0, 130.0),
        (-50.0, 170.0),
        (-60.0, 90.0),
        (-105.0, 50.0),
        (-35.0, -10.0),
        (-40.0, 65.0),
        (-20.0, 40.0),
    ]
    .iter()
    .map(|(x, y)| sketch.add_point(DVec2::new(*x, *y)))
    .collect();
    for rank in 0..corners.len() {
        sketch.add_segment(corners[rank], corners[(rank + 1) % corners.len()]);
    }

    // A whisker off the trait that runs from (-35, 130) to (-50, 170).
    sketch.move_point(corners[2], DVec2::new(-42.499, 150.0));
    let beside = area(&sketch.regions()[0].triangles);

    // And exactly on it, where the face pinches and walks that corner twice.
    sketch.move_point(corners[2], DVec2::new(-42.5, 150.0));

    let regions = sketch.regions();
    assert_eq!(
        regions.len(),
        1,
        "a corner laid on a trait pinches the area at that corner; it does \
         not take it away",
    );
    assert!(
        (area(&regions[0].triangles) - beside).abs() < 1.0,
        "the area was {beside} a thousandth away from the trait and {} on \
         it, where it should barely have moved",
        area(&regions[0].triangles),
    );
}

#[test]
fn a_trait_laid_right_across_a_shape_still_cuts_it_in_two() {
    let (mut sketch, _) = a_closed_shape();
    let whole = area(&sketch.regions()[0].triangles);

    laid(
        &mut sketch,
        DVec2::new(-60.0, 130.0),
        DVec2::new(60.0, 125.0),
    );

    let regions = sketch.regions();
    assert_eq!(
        regions.len(),
        2,
        "both ends outside, so it goes clean through"
    );
    let cut: f64 = regions.iter().map(|region| area(&region.triangles)).sum();
    assert!(
        (cut - whole).abs() < 1e-6,
        "the two areas together are the one they were cut from: {cut} against {whole}",
    );
}

/// The smallest honest test the issue names: one segment for the flat
/// side, one half-arc for the round one.
#[test]
fn a_d_shape_of_one_segment_and_one_half_arc_closes_and_extrudes() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let radius = 5.0;
    let bottom = sketch.add_point(DVec2::new(0.0, -radius));
    let top = sketch.add_point(DVec2::new(0.0, radius));
    let center = sketch.add_point(DVec2::ZERO);
    sketch.add_segment(top, bottom);
    // Counter-clockwise from the bottom, through the right, to the top:
    // the bulge sits on the positive side of the flat edge.
    sketch.add_arc(center, bottom, top);

    let regions = sketch.regions();
    assert_eq!(regions.len(), 1, "one segment and one arc close one area");
    assert!(regions[0].holes.is_empty());

    let found = area(&regions[0].triangles);
    let expected = std::f64::consts::PI * radius * radius * 0.5;
    assert!(
        (found - expected).abs() / expected < 0.01,
        "area {found}, expected ~{expected}"
    );
    assert!(
        regions[0].contains(DVec2::new(2.0, 0.0)),
        "inside the bulge"
    );
    assert!(
        !regions[0].contains(DVec2::new(-2.0, 0.0)),
        "the flat side is not rounded outwards too"
    );
}

/// The outline emits the sampled curve, not the chord between the arc's
/// two ends: something has to sit out at the bulge's own peak, which a
/// shape built from the two ends alone never would.
#[test]
fn the_round_side_of_a_d_shape_bulges_out_to_the_curve_not_the_chord() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let radius = 10.0;
    let bottom = sketch.add_point(DVec2::new(0.0, -radius));
    let top = sketch.add_point(DVec2::new(0.0, radius));
    let center = sketch.add_point(DVec2::ZERO);
    sketch.add_segment(top, bottom);
    sketch.add_arc(center, bottom, top);

    let regions = sketch.regions();
    let peak = DVec2::new(radius, 0.0);
    let nearest = regions[0]
        .outline
        .points
        .iter()
        .map(|point| point.distance(peak))
        .fold(f64::MAX, f64::min);
    assert!(
        nearest < 0.1,
        "no sampled point came near the arc's own peak at {peak}: nearest was {nearest} away",
    );
}

/// An arc joined to nothing is a dangling spur, the same as a lone
/// segment — not an area. Far from the origin and swept the long way
/// round, so the walk samples enough points that a shoelace sum landing
/// on exactly zero cannot be trusted to catch it: the rejection has to be
/// structural, not a floating-point coincidence.
#[test]
fn a_dangling_arc_with_nothing_attached_encloses_nothing() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let middle = DVec2::new(6870.0099, -4722.8225);
    let radius = 3000.964;
    let (from_angle, sweep) = (3.2719, 6.0475);
    let centre = sketch.add_point(middle);
    let start = sketch.add_point(middle + DVec2::from_angle(from_angle) * radius);
    let end = sketch.add_point(middle + DVec2::from_angle(from_angle + sweep) * radius);
    sketch.add_arc(centre, start, end);

    assert!(sketch.regions().is_empty());
}
