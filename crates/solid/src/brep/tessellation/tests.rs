pub(crate) mod fixtures;

use glam::DVec3;

use crate::brep::topology::Body;
use crate::soundness::{closed, enclosed, uncrossed};

/// The triangles of a body, held to the rules: closed, uncrossed, and
/// enclosing the exact volume within what the chords take off the curved
/// faces — at most the tolerance over their area.
fn held(body: &Body, tolerance: f64, exact: f64, curved_area: f64) -> Vec<[DVec3; 3]> {
    let triangles = body.triangles(tolerance);
    closed(&triangles).expect("the triangles close");
    uncrossed(&triangles).expect("no triangle crosses another");
    let volume = enclosed(&triangles);
    assert!(
        (volume - exact).abs() <= tolerance * curved_area + 1e-9 * exact,
        "the triangles enclose {volume}, the body {exact}",
    );
    triangles
}

#[test]
fn a_block_is_drawn_closed_by_two_triangles_a_side() {
    let triangles = held(&fixtures::block(), 0.02, fixtures::block_volume(), 0.0);
    assert_eq!(triangles.len(), 12);
}

/// The area of a cylinder's wall of `radius` over the block's height.
fn wall(radius: f64) -> f64 {
    std::f64::consts::TAU * radius * fixtures::HEIGHT
}

/// A thousandth of the reach, the tolerance the application draws at.
const DRAWN: f64 = 1e-3 * fixtures::STOCK_RADIUS;

#[test]
fn the_stock_is_drawn_closed_its_wall_going_round_with_no_seam() {
    let volume = fixtures::disc_volume(fixtures::STOCK_RADIUS, fixtures::HEIGHT);
    let triangles = held(
        &fixtures::stock(),
        DRAWN,
        volume,
        wall(fixtures::STOCK_RADIUS),
    );
    assert!(triangles.len() < 2000, "{} triangles", triangles.len());
}

#[test]
fn a_block_with_a_hole_bored_through_is_drawn_closed_round_the_hole() {
    let volume =
        fixtures::block_volume() - fixtures::disc_volume(fixtures::HOLE_RADIUS, fixtures::HEIGHT);
    held(
        &fixtures::block_with_a_hole(),
        DRAWN,
        volume,
        wall(fixtures::HOLE_RADIUS),
    );
}

#[test]
fn two_holes_touching_along_a_line_are_drawn_closed_and_apart() {
    let hole = fixtures::disc_volume(fixtures::HOLE_RADIUS, fixtures::HEIGHT);
    let body = fixtures::block_with_two_touching_holes();
    held(
        &body,
        DRAWN,
        fixtures::block_volume() - 2.0 * hole,
        2.0 * wall(fixtures::HOLE_RADIUS),
    );
}

#[test]
fn a_cylinder_lying_on_the_block_leaves_a_slit_in_the_top_drawn_on_both_sides() {
    let lying = fixtures::disc_volume(fixtures::LYING_RADIUS, fixtures::LYING_LENGTH);
    let area = std::f64::consts::TAU * fixtures::LYING_RADIUS * fixtures::LYING_LENGTH;
    let body = fixtures::block_with_a_lying_cylinder();
    let triangles = held(&body, DRAWN, fixtures::block_volume() + lying, area);
    let half = fixtures::LYING_LENGTH / 2.0;
    let near = DVec3::new(-half, 0.0, fixtures::HEIGHT);
    let far = DVec3::new(half, 0.0, fixtures::HEIGHT);
    let sides = |from: DVec3, to: DVec3| {
        let runs = |corners: &[DVec3; 3]| {
            (0..3).any(|at| corners[at] == from && corners[(at + 1) % 3] == to)
        };
        triangles.iter().filter(|corners| runs(corners)).count()
    };
    assert_eq!((sides(near, far), sides(far, near)), (2, 2));
}

#[test]
fn a_hole_tangent_inside_the_stock_is_drawn_closed_and_uncrossed() {
    let volume = fixtures::disc_volume(fixtures::STOCK_RADIUS, fixtures::HEIGHT)
        - fixtures::disc_volume(fixtures::HOLE_RADIUS, fixtures::HEIGHT);
    let area = wall(fixtures::STOCK_RADIUS) + wall(fixtures::HOLE_RADIUS);
    held(&fixtures::stock_with_a_tangent_hole(), DRAWN, volume, area);
}
