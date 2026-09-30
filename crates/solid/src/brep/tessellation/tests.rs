pub(crate) mod across;
pub(crate) mod fixtures;

use std::f64::consts::{PI, TAU};

use glam::DVec3;

use crate::brep::surface::{Cylinder, Surface};
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
    let by_hand = [
        fixtures::stock_with_a_tangent_hole(),
        fixtures::block_with_a_lying_cylinder(),
    ];
    for body in by_hand.into_iter().chain(bounded_by_meets()) {
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

#[test]
fn a_face_whose_loops_bound_no_region_is_left_open_rather_than_ending_the_program() {
    let mut build = fixtures::Build::new();
    let wall = build.cylinder(DVec3::ZERO, DVec3::Z, fixtures::STOCK_RADIUS);
    let across = build.cylinder(DVec3::Z * 5.0, DVec3::X, fixtures::HOLE_RADIUS);
    let ring = build.circle(wall, 0.0, None);
    let meet = build.meet(wall, across);
    let use_of = |edge, forward| crate::brep::topology::Coedge { edge, forward };
    build.face(
        wall,
        false,
        vec![vec![use_of(ring, true)], vec![use_of(meet, false)]],
    );
    assert!(
        build
            .finish(fixtures::STOCK_RADIUS)
            .triangles(DRAWN)
            .is_empty()
    );
}

#[test]
fn a_tube_whose_window_leaves_a_strut_narrower_than_a_grid_step_is_drawn_closed() {
    let annulus =
        std::f64::consts::PI * (fixtures::STOCK_RADIUS.powi(2) - fixtures::BORE_RADIUS.powi(2));
    let area = wall(fixtures::STOCK_RADIUS) + wall(fixtures::BORE_RADIUS);
    let whole = std::f64::consts::TAU;
    for (from, to) in [
        (0.3, 1.2),
        (3.0, 3.5),
        (0.3, 0.1 + whole),
        (1.0, 0.99 + whole),
    ] {
        let volume =
            annulus * fixtures::HEIGHT - (to - from) / whole * annulus * fixtures::WINDOW_HEIGHT;
        let body = fixtures::tube_with_a_window(from, to);
        for tolerance in [0.02, 0.5, 10.0] {
            held(&body, tolerance, volume, area);
        }
    }
}

/// Angles a tangency is placed at: a few anywhere, and a few a hair either
/// side of a grid angle every grid shares, from below the kernel's own
/// tolerance to a hundred thousandth of a turn — where the first samples on
/// either side stand closer to the other wall than the rules can tell apart.
fn round_a_grid_angle() -> Vec<f64> {
    let shared = 3.0 * TAU / 16.0;
    let mut angles = vec![0.3, 2.0, PI - 1e-7, -PI + 1e-7];
    for offset in [1.5e-9, 1e-7, 1e-5] {
        angles.extend([shared - offset, shared + offset]);
    }
    angles
}

#[test]
fn a_hole_tangent_inside_the_stock_stays_closed_and_uncrossed_wherever_it_touches() {
    let volume = fixtures::disc_volume(fixtures::STOCK_RADIUS, fixtures::HEIGHT)
        - fixtures::disc_volume(fixtures::HOLE_RADIUS, fixtures::HEIGHT);
    let area = wall(fixtures::STOCK_RADIUS) + wall(fixtures::HOLE_RADIUS);
    for angle in round_a_grid_angle() {
        let body = fixtures::stock_with_a_hole_tangent_at(angle);
        for tolerance in [1e-6, 0.02, 0.1, 10.0] {
            held(&body, tolerance, volume, area);
        }
    }
}

#[test]
fn two_holes_touching_stay_closed_and_uncrossed_wherever_they_touch() {
    let hole = fixtures::disc_volume(fixtures::HOLE_RADIUS, fixtures::HEIGHT);
    for angle in round_a_grid_angle() {
        let body = fixtures::block_with_two_holes_touching_at(angle);
        for tolerance in [1e-6, 0.02, 0.1, 10.0] {
            held(
                &body,
                tolerance,
                fixtures::block_volume() - 2.0 * hole,
                2.0 * wall(fixtures::HOLE_RADIUS),
            );
        }
    }
}

#[test]
fn a_slit_a_hair_from_the_first_grid_angle_of_a_wall_going_round_is_drawn_closed() {
    let depth = fixtures::POCKET_DEPTH;
    let volume = fixtures::disc_volume(fixtures::STOCK_RADIUS, fixtures::HEIGHT)
        - fixtures::disc_volume(fixtures::HOLE_RADIUS, depth);
    let area = wall(fixtures::STOCK_RADIUS) + std::f64::consts::TAU * fixtures::HOLE_RADIUS * depth;
    for angle in [-2e-9, 2e-9, 3e-8] {
        let body = fixtures::stock_with_a_pocket_tangent_at(angle, 1000.0);
        for tolerance in [1e-6, 0.02, 10.0] {
            held(&body, tolerance, volume, area);
        }
    }
}

#[test]
fn a_slot_whose_half_rounds_run_across_the_angle_where_the_parameters_wrap_is_drawn_closed() {
    let straight = 2.0 * fixtures::SLOT_HALF_LENGTH * 2.0 * fixtures::HOLE_RADIUS;
    let volume = straight * fixtures::HEIGHT
        + fixtures::disc_volume(fixtures::HOLE_RADIUS, fixtures::HEIGHT);
    for angle in [0.0, 0.3, PI / 2.0, PI - 1e-12, PI, -2.0] {
        let body = fixtures::slot_turned_by(angle);
        for tolerance in [1e-6, 0.02, 10.0] {
            held(&body, tolerance, volume, wall(fixtures::HOLE_RADIUS));
        }
    }
}

/// Gaps a hair wide: half the kernel's own tolerance, and a ten-millionth.
const HAIRS: [f64; 2] = [5e-10 * fixtures::HALF_SIDE, 1e-7];

#[test]
fn holes_a_hair_apart_or_a_hair_from_the_block_s_side_stay_closed_and_uncrossed() {
    let hole = fixtures::disc_volume(fixtures::HOLE_RADIUS, fixtures::HEIGHT);
    for gap in HAIRS {
        for angle in [0.0, 0.1, 3.0 * TAU / 16.0 + 1e-6] {
            let way =
                DVec3::new(angle.cos(), angle.sin(), 0.0) * (fixtures::HOLE_RADIUS + gap / 2.0);
            let body = fixtures::block_with_holes_at(&[-way, way]);
            for tolerance in [1e-6, 0.02, 10.0] {
                let volume = fixtures::block_volume() - 2.0 * hole;
                held(&body, tolerance, volume, 2.0 * wall(fixtures::HOLE_RADIUS));
            }
        }
        let inside = fixtures::HALF_SIDE - fixtures::HOLE_RADIUS - gap;
        for center in [
            DVec3::new(inside, 0.0, 0.0),
            DVec3::new(inside, inside, 0.0),
        ] {
            let body = fixtures::block_with_holes_at(&[center]);
            for tolerance in [1e-6, 0.02, 10.0] {
                let volume = fixtures::block_volume() - hole;
                held(&body, tolerance, volume, wall(fixtures::HOLE_RADIUS));
            }
        }
    }
}

#[test]
fn a_hole_a_hair_inside_the_stock_s_wall_on_a_grid_angle_or_off_it_stays_closed_and_uncrossed() {
    let volume = fixtures::disc_volume(fixtures::STOCK_RADIUS, fixtures::HEIGHT)
        - fixtures::disc_volume(fixtures::HOLE_RADIUS, fixtures::HEIGHT);
    let area = wall(fixtures::STOCK_RADIUS) + wall(fixtures::HOLE_RADIUS);
    for gap in HAIRS {
        let away = fixtures::STOCK_RADIUS - fixtures::HOLE_RADIUS - gap;
        for angle in [0.0, 0.1, PI] {
            let center = DVec3::new(angle.cos(), angle.sin(), 0.0) * away;
            for tolerance in [1e-6, 0.02, 10.0] {
                held(
                    &fixtures::stock_with_a_hole_at(center),
                    tolerance,
                    volume,
                    area,
                );
            }
        }
    }
}

#[test]
fn two_holes_tangent_inside_the_stock_at_once_part_its_wall_in_two_drawn_closed_and_uncrossed() {
    let volume = fixtures::disc_volume(fixtures::STOCK_RADIUS, fixtures::HEIGHT)
        - 2.0 * fixtures::disc_volume(fixtures::HOLE_RADIUS, fixtures::HEIGHT);
    let area = wall(fixtures::STOCK_RADIUS) + 2.0 * wall(fixtures::HOLE_RADIUS);
    let shared = 3.0 * TAU / 16.0;
    for angles in [[0.0, PI], [0.3, 1.7], [shared + 1e-7, PI + 0.2]] {
        let body = fixtures::stock_with_two_holes_tangent_at(angles);
        for tolerance in [1e-6, 0.02, 10.0] {
            held(&body, tolerance, volume, area);
        }
    }
}

/// A body's account of what it is made of, held by the rules written apart
/// from the kernel.
fn listed(body: &Body) {
    crate::soundness::listed(&body.listing(), body.scale().reach()).expect("the listing holds");
}

/// The exact volume of a body against the one arithmetic promises, to within
/// what the quadrature along its curves and the rule of the reference leave.
fn holds(body: &Body, promised: f64) -> f64 {
    let volume = body.volume();
    assert!(
        (volume - promised).abs() <= 1e-9 * promised,
        "the body holds {volume}, arithmetic promised {promised}"
    );
    volume
}

#[test]
fn the_reference_integral_gives_two_equal_cylinders_crossing_what_steinmetz_did() {
    for radius in [0.5, 5.0, 20.0] {
        let steinmetz = 16.0 * radius * radius * radius / 3.0;
        let found = across::common(radius, radius, 0.0);
        assert!(
            (found - steinmetz).abs() <= 1e-12 * steinmetz,
            "{found} against {steinmetz}"
        );
    }
}

#[test]
fn the_stock_bored_across_clear_of_its_top_is_drawn_closed_round_both_windows() {
    let radius = 3.0;
    let stock = fixtures::disc_volume(fixtures::STOCK_RADIUS, fixtures::HEIGHT);
    let area = wall(fixtures::STOCK_RADIUS) + TAU * radius * 2.0 * fixtures::STOCK_RADIUS;
    for axis in [DVec3::X, DVec3::Y] {
        let body = across::stock_bored_across(radius, axis);
        listed(&body);
        let volume = holds(&body, stock - across::common_across(radius, 0.0));
        for tolerance in [1e-6, 1e-3, DRAWN, 0.5, 10.0] {
            held(&body, tolerance, volume, area);
        }
    }
}

#[test]
fn the_stock_bored_across_flush_with_its_top_and_bottom_is_drawn_closed_on_both_sides_of_the_touch()
{
    let radius = across::MIDDLE;
    let stock = fixtures::disc_volume(fixtures::STOCK_RADIUS, fixtures::HEIGHT);
    let area = wall(fixtures::STOCK_RADIUS) + TAU * radius * 2.0 * fixtures::STOCK_RADIUS;
    let body = across::stock_bored_flush_across();
    listed(&body);
    let volume = holds(&body, stock - across::common_across(radius, 0.0));
    for tolerance in [1e-6, 1e-3, DRAWN, 0.5, 10.0] {
        held(&body, tolerance, volume, area);
    }
}

#[test]
fn the_stock_crossed_by_a_cylinder_lying_through_it_is_drawn_closed_north_and_south_of_the_arms() {
    let radius = across::MIDDLE;
    let stock = fixtures::disc_volume(fixtures::STOCK_RADIUS, fixtures::HEIGHT);
    let arm = fixtures::disc_volume(radius, 2.0 * across::ARM);
    let area = wall(fixtures::STOCK_RADIUS) + TAU * radius * 2.0 * across::ARM;
    let body = across::stock_crossed();
    listed(&body);
    let volume = holds(&body, stock + arm - across::common_across(radius, 0.0));
    for tolerance in [1e-6, 1e-3, DRAWN, 0.5, 10.0] {
        held(&body, tolerance, volume, area);
    }
}

/// How far inside `cylinder` a triangle with every corner on it reaches at its
/// deepest, read at points spread over it. None when a corner is off it, or
/// when every corner stands at one height along its axis: a cap square to the
/// axis is not its wall.
fn depth_in(corners: &[DVec3; 3], cylinder: &Cylinder) -> Option<f64> {
    let height = |point: DVec3| (point - cylinder.origin).dot(cylinder.axis);
    let on = corners
        .iter()
        .all(|corner| cylinder.distance(*corner).abs() < 1e-9);
    if !on
        || corners
            .iter()
            .all(|corner| height(*corner) == height(corners[0]))
    {
        return None;
    }
    let mut deepest = 0.0f64;
    for one in 0..=8 {
        for other in 0..=8 - one {
            let (a, b) = (one as f64 / 8.0, other as f64 / 8.0);
            let point = corners[0] * (1.0 - a - b) + corners[1] * a + corners[2] * b;
            deepest = deepest.max(-cylinder.distance(point));
        }
    }
    Some(deepest)
}

/// How far inside its cylinder a triangle of a wall reaches, at the deepest
/// of them all. A triangle whose corners all lie where two cylinders meet
/// lies on both: it is held to the one it stays closest to.
fn deepest_of_any_wall(body: &Body, triangles: &[[DVec3; 3]]) -> f64 {
    let cylinders: Vec<Cylinder> = body
        .surfaces
        .iter()
        .filter_map(|surface| match surface {
            Surface::Cylinder(cylinder) => Some(*cylinder),
            Surface::Plane(_) => None,
        })
        .collect();
    triangles
        .iter()
        .filter_map(|corners| {
            cylinders
                .iter()
                .filter_map(|cylinder| depth_in(corners, cylinder))
                .reduce(f64::min)
        })
        .fold(0.0, f64::max)
}

#[test]
fn no_triangle_of_a_wall_a_meet_bounds_stands_further_inside_its_cylinder_than_the_tolerance() {
    for body in bounded_by_meets() {
        for tolerance in [3e-3, DRAWN, 0.5] {
            let deepest = deepest_of_any_wall(&body, &body.triangles(tolerance));
            assert!(
                deepest > 0.0 && deepest <= tolerance,
                "{deepest} inside a wall within {tolerance}"
            );
        }
    }
}

/// Every body built by hand whose faces the curve two cylinders meet along
/// bounds, near a touch or far from one.
fn bounded_by_meets() -> Vec<Body> {
    let near = fixtures::STOCK_RADIUS - across::TOUCHING;
    vec![
        across::stock_bored_across(3.0, DVec3::X),
        across::stock_bored_flush_across(),
        across::stock_crossed(),
        across::equal_cylinders_crossed(),
        across::equal_cylinders_bored(),
        across::stock_bored_touching_its_wall(),
        across::stock_bored_across_at(across::TOUCHING, DVec3::X, near - 1e-3),
        across::stock_bored_through_its_wall(near + 1e-3),
    ]
}

/// How many steps of its cylinder's grid the widest triangle of a wall spans
/// round its axis: a triangle whose corners all lie on two cylinders is held
/// to the one it stays closest to.
fn widest_in_steps(body: &Body, triangles: &[[DVec3; 3]], tolerance: f64) -> f64 {
    let cylinders: Vec<Cylinder> = body
        .surfaces
        .iter()
        .filter_map(|surface| match surface {
            Surface::Cylinder(cylinder) => Some(*cylinder),
            Surface::Plane(_) => None,
        })
        .collect();
    let mut widest = 0.0f64;
    for corners in triangles {
        let held_to = cylinders
            .iter()
            .filter_map(|cylinder| Some((depth_in(corners, cylinder)?, cylinder)))
            .min_by(|one, other| one.0.total_cmp(&other.0));
        let Some((_, cylinder)) = held_to else {
            continue;
        };
        let angles = corners.map(|corner| cylinder.parameters(corner).x);
        let apart = |one: f64, other: f64| (one - other + PI).rem_euclid(TAU) - PI;
        let span = (0..3)
            .map(|at| apart(angles[at], angles[(at + 1) % 3]).abs())
            .fold(0.0, f64::max);
        let step = TAU / super::sampling::divisions(cylinder.radius, tolerance) as f64;
        widest = widest.max(span / step);
    }
    widest
}

#[test]
fn no_triangle_of_a_wall_a_meet_bounds_spans_more_than_one_step_of_its_grid() {
    for body in bounded_by_meets() {
        for tolerance in [1e-3, DRAWN, 0.5, 10.0] {
            let widest = widest_in_steps(&body, &body.triangles(tolerance), tolerance);
            assert!(
                widest > 0.0 && widest <= 1.0 + 1e-9,
                "a triangle spans {widest} steps within {tolerance}"
            );
        }
    }
}

#[test]
fn two_equal_cylinders_crossing_are_drawn_closed_and_uncrossed_where_their_walls_touch() {
    let (radius, reach) = (across::EQUAL, across::EQUAL_REACH);
    let each = fixtures::disc_volume(radius, 2.0 * reach);
    let area = 2.0 * TAU * radius * 2.0 * reach;
    let body = across::equal_cylinders_crossed();
    listed(&body);
    let volume = holds(&body, 2.0 * each - across::common(radius, radius, 0.0));
    for tolerance in [1e-6, 1e-3, DRAWN, 0.5, 10.0] {
        held(&body, tolerance, volume, area);
    }
}

#[test]
fn a_cylinder_bored_through_by_an_equal_one_is_drawn_closed_and_uncrossed_where_their_walls_touch()
{
    let (radius, reach) = (across::EQUAL, across::EQUAL_REACH);
    let post = fixtures::disc_volume(radius, 2.0 * reach);
    let area = TAU * radius * 2.0 * reach + TAU * radius * 2.0 * radius;
    let body = across::equal_cylinders_bored();
    listed(&body);
    let volume = holds(&body, post - across::common(radius, radius, 0.0));
    for tolerance in [1e-6, 1e-3, DRAWN, 0.5, 10.0] {
        held(&body, tolerance, volume, area);
    }
}

#[test]
fn a_bore_touching_the_stock_s_wall_from_inside_is_drawn_closed_and_uncrossed_round_the_node() {
    let radius = across::TOUCHING;
    let stock = fixtures::disc_volume(fixtures::STOCK_RADIUS, fixtures::HEIGHT);
    let area = wall(fixtures::STOCK_RADIUS) + TAU * radius * 2.0 * fixtures::STOCK_RADIUS;
    let body = across::stock_bored_touching_its_wall();
    listed(&body);
    let taken = across::common_across(radius, fixtures::STOCK_RADIUS - radius);
    let volume = holds(&body, stock - taken);
    for tolerance in [1e-6, 1e-3, DRAWN, 0.5, 10.0] {
        held(&body, tolerance, volume, area);
    }
}

#[test]
fn a_bore_a_hair_inside_the_stock_s_wall_leaves_two_windows_drawn_closed_and_uncrossed() {
    let radius = across::TOUCHING;
    let stock = fixtures::disc_volume(fixtures::STOCK_RADIUS, fixtures::HEIGHT);
    let area = wall(fixtures::STOCK_RADIUS) + TAU * radius * 2.0 * fixtures::STOCK_RADIUS;
    for hair in HAIRS_FROM_A_TOUCH {
        let across = fixtures::STOCK_RADIUS - radius - hair;
        let body = across::stock_bored_across_at(radius, DVec3::X, across);
        listed(&body);
        let volume = holds(&body, stock - across::common_across(radius, across));
        for tolerance in [1e-6, 1e-3, DRAWN, 0.5, 10.0] {
            held(&body, tolerance, volume, area);
        }
    }
}

/// How far a bore stands from touching a wall, from twice the kernel's
/// tolerance, under which it would be decided touching, to a tenth.
const HAIRS_FROM_A_TOUCH: [f64; 5] = [1e-7, 1e-5, 1e-3, 1e-2, 1e-1];

/// Beside the neck a bore breaking through the stock's wall leaves, the
/// matter between the bore's top and the wall thins to nothing: a hair `ε`
/// through leaves it `((z − 5)² − 6ε)/6` thick at the height `z`. Broken
/// through by the least hair the kernel keeps apart from a touch, it stays
/// thinner than the rules' `NEAR` over a band some millionths wide beside the
/// neck, and drawn within a millionth the triangles resolve that band: the
/// rules cannot tell its two sides from one face laid twice. That hair is
/// drawn no finer than a thousandth.
#[test]
fn a_bore_a_hair_through_the_stock_s_wall_leaves_one_window_with_a_neck_drawn_closed_and_uncrossed()
{
    let radius = across::TOUCHING;
    let stock = fixtures::disc_volume(fixtures::STOCK_RADIUS, fixtures::HEIGHT);
    let area = wall(fixtures::STOCK_RADIUS) + TAU * radius * 2.0 * fixtures::STOCK_RADIUS;
    for hair in HAIRS_FROM_A_TOUCH {
        let across = fixtures::STOCK_RADIUS - radius + hair;
        let body = across::stock_bored_through_its_wall(across);
        listed(&body);
        let volume = holds(&body, stock - across::common_across(radius, across));
        let finest = if hair < 1e-6 { 1e-3 } else { 1e-6 };
        for tolerance in [finest, 1e-3, DRAWN, 0.5, 10.0] {
            held(&body, tolerance, volume, area);
        }
    }
}
