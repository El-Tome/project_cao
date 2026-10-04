//! Points drawn in one place, each of its own, and the areas the curves ending
//! on them close.
//!
//! The rectangle tool's two generated corners are always new points, so a
//! rectangle whose corner lands where the drawing already has a point leaves
//! two points there. The graph the areas are read from takes them as one
//! vertex, as it does a crossing; the drawing keeps both.
//!
//! Closes #494.
//! - a corner drawn where the drawing already has a point is that point, and
//!   the area it closes is found —
//!   `a_diagonal_whose_ends_stand_on_two_corners_cuts_the_square_in_two`,
//!   `an_arc_whose_ends_stand_on_two_corners_cuts_the_square_in_two`
//! - two rectangles sharing a whole side, each with its own four points, are
//!   still two areas — `two_squares_sharing_a_side_with_points_of_their_own_are_two_areas`
//! - the six drawings the campaigns over random drawings filed under #494
//!   hold, a second rectangle started on the first one's corner among them —
//!   no test: they are in `what_random_sketches_found.rs`, their `ignore`
//!   taken off, and that file answers for them
//! - the drawing keeps what the user drew — no test: settled by the borrow,
//!   since `regions` reads the sketch through `&self` and the weld lives in
//!   the graph it builds

use cao_sketch::{Sketch, WorkPlane};
use glam::DVec2;

fn square(sketch: &mut Sketch, corner: DVec2, side: f64) {
    let corners = [
        corner,
        corner + DVec2::new(side, 0.0),
        corner + DVec2::new(side, side),
        corner + DVec2::new(0.0, side),
    ]
    .map(|place| sketch.add_point(place));
    for index in 0..4 {
        sketch.add_segment(corners[index], corners[(index + 1) % 4]);
    }
}

fn areas(sketch: &Sketch) -> Vec<f64> {
    let mut found: Vec<f64> = sketch
        .regions()
        .iter()
        .map(|region| {
            region
                .triangles
                .iter()
                .map(|[a, b, c]| (*b - *a).perp_dot(*c - *a).abs() * 0.5)
                .sum()
        })
        .collect();
    found.sort_by(f64::total_cmp);
    found
}

#[test]
fn a_diagonal_whose_ends_stand_on_two_corners_cuts_the_square_in_two() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let from = sketch.add_point(DVec2::new(2.0, 0.0));
    let to = sketch.add_point(DVec2::new(0.0, 2.0));
    sketch.add_segment(from, to);
    square(&mut sketch, DVec2::ZERO, 2.0);

    let found = areas(&sketch);
    assert_eq!(found.len(), 2, "the diagonal halves the square: {found:?}");
    assert!(
        found.iter().all(|half| (half - 2.0).abs() < 1e-9),
        "each half is 2, got {found:?}",
    );
}

#[test]
fn an_arc_whose_ends_stand_on_two_corners_cuts_the_square_in_two() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::ZERO);
    let start = sketch.add_point(DVec2::new(2.0, 0.0));
    let end = sketch.add_point(DVec2::new(0.0, 2.0));
    sketch.add_arc(centre, start, end);
    square(&mut sketch, DVec2::ZERO, 2.0);

    let found = areas(&sketch);
    let quarter = std::f64::consts::PI;
    assert_eq!(found.len(), 2, "the arc cuts the square in two: {found:?}");
    assert!(
        (found.iter().sum::<f64>() - 4.0).abs() < 1e-2,
        "the two pieces make the square, got {found:?}",
    );
    assert!(
        (found[1] - quarter).abs() < 1e-2,
        "the quarter disc is about {quarter}, got {found:?}",
    );
}

#[test]
fn two_squares_sharing_a_side_with_points_of_their_own_are_two_areas() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    square(&mut sketch, DVec2::ZERO, 2.0);
    square(&mut sketch, DVec2::new(2.0, 0.0), 2.0);

    let found = areas(&sketch);
    assert_eq!(found.len(), 2, "one area on each side: {found:?}");
    assert!(
        found.iter().all(|each| (each - 4.0).abs() < 1e-9),
        "each square is 4, got {found:?}",
    );
}
