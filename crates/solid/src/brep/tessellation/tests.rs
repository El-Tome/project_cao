pub(crate) mod fixtures;

use glam::DVec3;

use crate::brep::topology::Body;
use crate::soundness::{closed, enclosed, uncrossed};

/// The triangles of a body, held to the rules: closed, uncrossed, and
/// enclosing the exact volume within what the chords take off the curved
/// faces — at most their sag over their area. The sag is the tolerance, or
/// what a thousand and twenty-four steps a turn leave on the stock when the
/// tolerance asks for finer.
fn held(body: &Body, tolerance: f64, exact: f64, curved_area: f64) -> Vec<[DVec3; 3]> {
    let triangles = body.triangles(tolerance);
    closed(&triangles).expect("the triangles close");
    uncrossed(&triangles).expect("no triangle crosses another");
    let volume = enclosed(&triangles);
    let finest = fixtures::STOCK_RADIUS * (1.0 - (std::f64::consts::PI / 1024.0).cos());
    assert!(
        (volume - exact).abs() <= tolerance.max(finest) * curved_area + 1e-9 * exact,
        "the triangles enclose {volume}, the body {exact}, within {tolerance}",
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

#[test]
fn a_hole_tangent_inside_the_stock_stays_uncrossed_however_fine_or_coarse_the_grids() {
    let volume = fixtures::disc_volume(fixtures::STOCK_RADIUS, fixtures::HEIGHT)
        - fixtures::disc_volume(fixtures::HOLE_RADIUS, fixtures::HEIGHT);
    let area = wall(fixtures::STOCK_RADIUS) + wall(fixtures::HOLE_RADIUS);
    for tolerance in [1e-6, 1e-4, 3e-3, 0.02, 0.1, 0.5, 2.0, 10.0] {
        held(
            &fixtures::stock_with_a_tangent_hole(),
            tolerance,
            volume,
            area,
        );
    }
}

#[test]
fn the_same_body_is_drawn_twice_to_the_same_bits() {
    for body in [
        fixtures::stock_with_a_tangent_hole(),
        fixtures::block_with_a_lying_cylinder(),
    ] {
        let first = body.triangles(DRAWN);
        crate::soundness::repeatable(&first, &body.triangles(DRAWN)).expect("the same bits");
    }
}

/// How far inside a cylinder standing on the XY plane about `(x, y)` the
/// triangles of its wall reach at their deepest: the triangles with every
/// corner on it and not all at one height, read at points spread over each.
fn deepest(triangles: &[[DVec3; 3]], x: f64, y: f64, radius: f64) -> f64 {
    let from_axis = |point: DVec3| (point.truncate() - glam::DVec2::new(x, y)).length();
    let mut deepest = 0.0f64;
    for corners in triangles {
        let on = corners
            .iter()
            .all(|corner| (from_axis(*corner) - radius).abs() < 1e-9);
        if !on || corners.iter().all(|corner| corner.z == corners[0].z) {
            continue;
        }
        for one in 0..=8 {
            for other in 0..=8 - one {
                let (a, b) = (one as f64 / 8.0, other as f64 / 8.0);
                let point = corners[0] * (1.0 - a - b) + corners[1] * a + corners[2] * b;
                deepest = deepest.max(radius - from_axis(point));
            }
        }
    }
    deepest
}

#[test]
fn no_triangle_of_a_wall_stands_further_inside_its_cylinder_than_the_tolerance() {
    let center = fixtures::STOCK_RADIUS - fixtures::HOLE_RADIUS;
    for tolerance in [3e-3, DRAWN, 0.5] {
        let triangles = fixtures::stock_with_a_tangent_hole().triangles(tolerance);
        let outer = deepest(&triangles, 0.0, 0.0, fixtures::STOCK_RADIUS);
        let inner = deepest(&triangles, center, 0.0, fixtures::HOLE_RADIUS);
        assert!(
            outer > 0.0 && outer <= tolerance,
            "{outer} within {tolerance}"
        );
        assert!(
            inner > 0.0 && inner <= tolerance,
            "{inner} within {tolerance}"
        );
    }
}

#[test]
fn a_pocket_tangent_inside_the_stock_hangs_a_slit_in_its_wall_drawn_closed_and_uncrossed() {
    let depth = fixtures::POCKET_DEPTH;
    let volume = fixtures::disc_volume(fixtures::STOCK_RADIUS, fixtures::HEIGHT)
        - fixtures::disc_volume(fixtures::HOLE_RADIUS, depth);
    let area = wall(fixtures::STOCK_RADIUS) + std::f64::consts::TAU * fixtures::HOLE_RADIUS * depth;
    for tolerance in [1e-6, 0.02, 10.0] {
        held(
            &fixtures::stock_with_a_tangent_pocket(),
            tolerance,
            volume,
            area,
        );
    }
}

#[test]
fn a_hole_a_hair_inside_the_stock_s_wall_stays_uncrossed_however_fine_or_coarse_the_grids() {
    let volume = fixtures::disc_volume(fixtures::STOCK_RADIUS, fixtures::HEIGHT)
        - fixtures::disc_volume(fixtures::HOLE_RADIUS, fixtures::HEIGHT);
    let area = wall(fixtures::STOCK_RADIUS) + wall(fixtures::HOLE_RADIUS);
    let away = fixtures::STOCK_RADIUS - fixtures::HOLE_RADIUS - 1e-6;
    let center = DVec3::new(0.1f64.cos(), 0.1f64.sin(), 0.0) * away;
    for tolerance in [1e-6, 1e-4, 3e-3, 0.02, 0.1, 0.5, 2.0, 10.0] {
        held(
            &fixtures::stock_with_a_hole_at(center),
            tolerance,
            volume,
            area,
        );
    }
}
