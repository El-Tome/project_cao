//! What sketch · regions/swept.rs is held to.

use super::*;

fn covered(triangles: &[[DVec2; 3]]) -> f64 {
    triangles
        .iter()
        .map(|[a, b, c]| (*b - *a).perp_dot(*c - *a).abs() * 0.5)
        .sum()
}

fn holds(point: DVec2, triangles: &[[DVec2; 3]]) -> bool {
    triangles.iter().any(|[a, b, c]| {
        let sides =
            [(a, b), (b, c), (c, a)].map(|(from, to)| (*to - *from).perp_dot(point - *from));
        sides.iter().all(|side| *side >= -1e-12) || sides.iter().all(|side| *side <= 1e-12)
    })
}

const TOLERANCE: f64 = 1e-9;

fn square(low: DVec2, side: f64) -> Vec<DVec2> {
    vec![
        low,
        low + DVec2::new(side, 0.0),
        low + DVec2::splat(side),
        low + DVec2::new(0.0, side),
    ]
}

#[test]
fn a_square_is_covered_whole() {
    let found = swept(&[&square(DVec2::ZERO, 4.0)]);
    assert!((covered(&found) - 16.0).abs() < TOLERANCE, "{found:?}");
}

#[test]
fn a_hole_wound_the_other_way_is_left_uncovered() {
    let shell = square(DVec2::ZERO, 4.0);
    let mut hole = square(DVec2::splat(1.0), 2.0);
    hole.reverse();
    let found = swept(&[&shell, &hole]);
    assert!((covered(&found) - 12.0).abs() < TOLERANCE);
    assert!(!holds(DVec2::splat(2.0), &found), "the middle is hollow");
    assert!(holds(DVec2::new(0.5, 2.0), &found), "the rim is matter");
}

#[test]
fn a_hole_touching_its_outline_still_leaves_the_matter_whole() {
    let shell = square(DVec2::ZERO, 4.0);
    let mut hole = vec![
        DVec2::new(2.0, 0.0),
        DVec2::new(3.0, 1.5),
        DVec2::new(2.0, 3.0),
        DVec2::new(1.0, 1.5),
    ];
    hole.reverse();
    let found = swept(&[&shell, &hole]);
    assert!(
        (covered(&found) - 13.0).abs() < TOLERANCE,
        "sixteen less the kite of three, got {}",
        covered(&found),
    );
    assert!(!holds(DVec2::new(2.0, 1.5), &found), "the kite is hollow");
}

#[test]
fn a_face_pinched_at_a_corner_is_covered_on_both_sides_of_it() {
    let shell = vec![
        DVec2::new(0.0, 0.0),
        DVec2::new(2.0, 0.0),
        DVec2::new(1.0, 1.0),
        DVec2::new(2.0, 2.0),
        DVec2::new(3.0, 1.0),
        DVec2::new(2.0, 0.0),
        DVec2::new(4.0, 0.0),
        DVec2::new(4.0, 4.0),
        DVec2::new(0.0, 4.0),
    ];
    let found = swept(&[&shell]);
    assert!(
        (covered(&found) - 14.0).abs() < TOLERANCE,
        "the square less the diamond carved through the pinch, got {}",
        covered(&found),
    );
    assert!(
        !holds(DVec2::new(2.0, 1.0), &found),
        "the diamond is hollow"
    );
    assert!(holds(DVec2::new(2.0, 3.0), &found), "above it is matter");
}
