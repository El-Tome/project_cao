//! Half a turn one way and half a turn the other are one and the same
//! direction to the face walk, whichever sign of zero the tangent carries.
//!
//! Closes #605.
//! - a segment tangent under a circle's lowest point, and the same drawing
//!   mirrored and turned so the tangent's zero changes sign, keep their
//!   faces — `a_circle_resting_on_a_side_keeps_its_faces_mirrored_and_turned`
//! - the grain of +π and -π is one cell, shown by a test on the drawing
//!   rather than on the arithmetic —
//!   `an_ellipse_leaning_a_hair_keeps_its_faces_whichever_way_it_leans`
//! - #516's acceptance matrix still holds at every turn — no test: it is
//!   `a_tangency_is_one_touch.rs`'s own matrix, run in the same gate

use cao_sketch::{Region, Sketch, WorkPlane};
use glam::DVec2;

const FLIPS: [DVec2; 4] = [
    DVec2::new(1.0, 1.0),
    DVec2::new(1.0, -1.0),
    DVec2::new(-1.0, 1.0),
    DVec2::new(-1.0, -1.0),
];

fn segment(sketch: &mut Sketch, flip: DVec2, from: DVec2, to: DVec2) {
    let from = sketch.add_point(flip * from);
    let to = sketch.add_point(flip * to);
    sketch.add_segment(from, to);
}

/// A rectangle whose side y = 0 the curve touches, standing on it or hanging
/// under it, cut by a chord so the curve enters the graph: the touch pinches
/// the band between the curve and that side, and a walk sorting the two sides
/// of the half-turn apart swallows it along with a piece of the curve.
fn boxed(sketch: &mut Sketch, flip: DVec2, side: f64) {
    let corners = [
        DVec2::new(-6.0, 0.0),
        DVec2::new(6.0, 0.0),
        DVec2::new(6.0, side * 5.0),
        DVec2::new(-6.0, side * 5.0),
    ];
    for i in 0..4 {
        segment(sketch, flip, corners[i], corners[(i + 1) % 4]);
    }
    segment(
        sketch,
        flip,
        DVec2::new(-6.0, side * 3.0),
        DVec2::new(6.0, side * 3.0),
    );
}

fn area(region: &Region) -> f64 {
    region
        .triangles
        .iter()
        .map(|[a, b, c]| (*b - *a).perp_dot(*c - *a).abs() * 0.5)
        .sum()
}

fn keeps_its_faces(sketch: &Sketch, label: &str) {
    let regions = sketch.regions();
    let total: f64 = regions.iter().map(area).sum();
    assert_eq!(
        regions.len(),
        5,
        "{label}: both pieces of the curve, the band the touch pinches, and \
         both pieces of what is left of the rectangle",
    );
    assert!(
        (total - 60.0).abs() < 1e-6,
        "{label}: the pieces tile the rectangle, got {total}",
    );
}

#[test]
fn a_circle_resting_on_a_side_keeps_its_faces_mirrored_and_turned() {
    for side in [1.0, -1.0] {
        for flip in FLIPS {
            let mut sketch = Sketch::new(WorkPlane::XY);
            boxed(&mut sketch, flip, side);
            let centre = sketch.add_point(flip * DVec2::new(0.0, side * 2.0));
            sketch.add_circle(centre, 2.0);
            keeps_its_faces(&sketch, &format!("circle, side {side}, flipped {flip}"));
        }
    }
}

#[test]
fn an_ellipse_leaning_a_hair_keeps_its_faces_whichever_way_it_leans() {
    let (a, b) = (4.0, 2.0);
    for lean in [0.0f64, 1e-12, -1e-12, 1e-10, -1e-10] {
        for (side, flip) in [1.0, -1.0].into_iter().flat_map(|s| FLIPS.map(|f| (s, f))) {
            let first = DVec2::new(a * lean.cos(), a * lean.sin());
            let second = DVec2::new(-b * lean.sin(), b * lean.cos());
            let height = (a * lean.sin()).hypot(b * lean.cos());
            let centre = DVec2::new(0.0, side * height);
            let mut sketch = Sketch::new(WorkPlane::XY);
            boxed(&mut sketch, flip, side);
            let middle = sketch.add_point(flip * centre);
            let ends = [
                sketch.add_point(flip * (centre + first)),
                sketch.add_point(flip * (centre - first)),
            ];
            let reach = [
                sketch.add_point(flip * (centre + second)),
                sketch.add_point(flip * (centre - second)),
            ];
            sketch.add_ellipse(middle, ends, reach);
            keeps_its_faces(
                &sketch,
                &format!("ellipse leaning {lean:e}, side {side}, flipped {flip}"),
            );
        }
    }
}
