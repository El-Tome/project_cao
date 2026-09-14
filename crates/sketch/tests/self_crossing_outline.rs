//! `Sketch::closed_outlines` walks the segment graph by the angular order of
//! the edges around each vertex. Two edges meeting in space with no vertex of
//! their own are invisible to that walk, so a crossing is given one: the
//! bowtie a plain drag produces encloses the two triangles it actually bounds.

use cao_sketch::{Region, Sketch, WorkPlane};
use glam::DVec2;

fn area(region: &Region) -> f64 {
    region
        .triangles
        .iter()
        .map(|[a, b, c]| (*b - *a).perp_dot(*c - *a).abs() * 0.5)
        .sum()
}

fn areas(sketch: &Sketch) -> Vec<f64> {
    let mut found: Vec<f64> = sketch.regions().iter().map(area).collect();
    found.sort_by(f64::total_cmp);
    found
}

const TOLERANCE: f64 = 1e-9;

#[test]
fn a_bowtie_encloses_the_two_triangles_its_crossing_bounds() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let a = sketch.add_point(DVec2::new(20.0, 5.0));
    let b = sketch.add_point(DVec2::new(10.0, 0.0));
    let c = sketch.add_point(DVec2::new(10.0, 10.0));
    let d = sketch.add_point(DVec2::new(0.0, 10.0));
    for (from, to) in [(a, b), (b, c), (c, d), (d, a)] {
        sketch.add_segment(from, to);
    }

    let found = areas(&sketch);
    assert_eq!(found.len(), 2, "the crossing bounds two triangles");
    assert!(
        (found[0] - 12.5).abs() < TOLERANCE && (found[1] - 37.5).abs() < TOLERANCE,
        "the crossing sits at (10, 7.5), leaving 12.5 and 37.5: got {found:?}",
    );
}

#[test]
fn dragging_a_corner_past_the_opposite_side_folds_a_square_into_two_triangles() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let a = sketch.add_point(DVec2::new(0.0, 0.0));
    let b = sketch.add_point(DVec2::new(10.0, 0.0));
    let c = sketch.add_point(DVec2::new(10.0, 10.0));
    let d = sketch.add_point(DVec2::new(0.0, 10.0));
    for (from, to) in [(a, b), (b, c), (c, d), (d, a)] {
        sketch.add_segment(from, to);
    }
    assert_eq!(sketch.regions().len(), 1, "the square starts as one region");

    sketch.settle_around(a, DVec2::new(20.0, 5.0), 1.0);

    let found = areas(&sketch);
    assert_eq!(found.len(), 2, "the folded square encloses two triangles");
    assert!(
        found.iter().sum::<f64>() > 0.0,
        "and both of them have an area: {found:?}",
    );
}

#[test]
fn a_straight_run_cutting_across_a_curve_closes_the_piece_above_it() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let radius = 5.0;
    let centre = sketch.add_point(DVec2::ZERO);
    let right = sketch.add_point(DVec2::new(radius, 0.0));
    let left = sketch.add_point(DVec2::new(-radius, 0.0));
    sketch.add_arc(centre, right, left);
    let from = sketch.add_point(DVec2::new(-8.0, 3.0));
    let to = sketch.add_point(DVec2::new(8.0, 3.0));
    sketch.add_segment(from, to);

    let found = areas(&sketch);
    assert_eq!(found.len(), 1, "the chord shuts the top of the bulge off");
    let expected = radius * radius / 2.0 * (2.0 * (3.0f64 / radius).acos() - 0.96);
    assert!(
        (found[0] - expected).abs() / expected < 0.01,
        "the piece above the chord is about {expected}, got {}",
        found[0],
    );
}

#[test]
fn two_curves_crossing_twice_close_the_lens_between_them() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let radius = 5.0;
    let near = sketch.add_point(DVec2::ZERO);
    let below = sketch.add_point(DVec2::new(0.0, -radius));
    let above = sketch.add_point(DVec2::new(0.0, radius));
    sketch.add_arc(near, below, above);

    let far = sketch.add_point(DVec2::new(6.0, 0.0));
    let over = sketch.add_point(DVec2::new(6.0, radius));
    let under = sketch.add_point(DVec2::new(6.0, -radius));
    sketch.add_arc(far, over, under);

    let found = areas(&sketch);
    assert_eq!(found.len(), 1, "the two bulges overlap in one lens");
    let expected = 2.0 * (radius * radius / 2.0 * (2.0 * (3.0f64 / radius).acos() - 0.96));
    assert!(
        (found[0] - expected).abs() / expected < 0.01,
        "the lens is about {expected}, got {}",
        found[0],
    );
}
