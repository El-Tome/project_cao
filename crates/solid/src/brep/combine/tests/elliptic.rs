//! Joins and cuts on an elliptic wall (#571): the planes of a block square
//! to its axis, along it — crossing it along two rulings, or touching it
//! along one at its support point — and at a slant, and the same ellipse
//! twice; then the cases the stresses of `stress_elliptic.rs` found drawn
//! wrong. Each held to its listing, to the arithmetic's volume where it is
//! simple, and drawn closed and uncrossed from a fine tolerance to a coarse
//! one.

use std::f64::consts::{FRAC_PI_2, PI, TAU};

use glam::{DVec2, DVec3};

use super::*;
use crate::soundness::{closed, uncrossed};

/// The half-axes of every ellipse below, its major one turned 0.4 off X
/// unless said otherwise.
const MAJOR: f64 = 7.0;
const MINOR: f64 = 3.0;

fn first(turned: f64) -> DVec2 {
    DVec2::from_angle(turned) * MAJOR
}

/// The ellipse about `center` whose major half-axis is `first`, raised from
/// `from` to `to`.
fn oval(center: [f64; 2], first: DVec2, from: f64, to: f64) -> Body {
    ellipse(center, first, MINOR, from, to)
}

/// The ellipse about `center` whose major half-axis is `first` and whose
/// minor one is `minor` long, raised from `from` to `to`.
fn ellipse(center: [f64; 2], first: DVec2, minor: f64, from: f64, to: f64) -> Body {
    let center = DVec2::from(center);
    let outline = Contour {
        corners: vec![center + first],
        runs: vec![Run::Oval {
            center,
            first,
            second: minor,
            turn: TAU,
        }],
    };
    Body::raised(&outline, &[], ground(from), DVec3::Z * (to - from)).expect("an ellipse raises")
}

/// The area of the ellipse.
fn area() -> f64 {
    PI * MAJOR * MINOR
}

fn held(body: &Body, volume: f64) {
    assert!(
        (body.volume() - volume).abs() <= 1e-9 * volume,
        "the body holds {}, the arithmetic {volume}",
        body.volume()
    );
    drawn(body);
}

/// The body's listing, and its triangles closed and uncrossed from a fine
/// tolerance to a coarse one.
fn drawn(body: &Body) {
    assert_eq!(listed(&body.listing(), body.scale().reach()), Ok(()));
    for share in [1e-4, 1e-3, 2e-2] {
        let triangles = body.triangles(share * body.scale().reach());
        assert_eq!(closed(&triangles).map_err(|_| share), Ok(()), "open");
        assert_eq!(uncrossed(&triangles).map_err(|_| share), Ok(()), "crossed");
    }
}

#[test]
fn an_elliptic_hole_through_a_block_and_a_pocket_into_it_hold_their_volume() {
    let stock = block([-10.0, -10.0, 0.0], [15.0, 10.0, 4.0]);
    let hole = stock
        .cut_by(&oval([3.0, -1.0], first(0.4), -1.0, 5.0))
        .expect("the hole is cut");
    held(&hole, 500.0 * 4.0 - area() * 4.0);
    let pocket = stock
        .cut_by(&oval([3.0, -1.0], first(0.4), 2.0, 5.0))
        .expect("the pocket is cut");
    held(&pocket, 500.0 * 4.0 - area() * 2.0);
}

#[test]
fn an_elliptic_boss_joined_on_a_plate_holds_its_volume() {
    let plate = block([-10.0, -10.0, 0.0], [15.0, 10.0, 2.0]);
    let boss = plate
        .joined(&oval([3.0, -1.0], first(0.4), 2.0, 8.0))
        .expect("the boss is joined");
    held(&boss, 500.0 * 2.0 + area() * 6.0);
    let walls = boss
        .surfaces
        .iter()
        .filter(|surface| matches!(surface, Surface::Elliptic(_)))
        .count();
    assert_eq!(walls, 1);
}

#[test]
fn a_block_s_side_crossing_an_elliptic_boss_cuts_it_along_two_rulings() {
    let boss = oval([3.0, -1.0], first(0.0), 0.0, 4.0);
    let cut = boss
        .cut_by(&block([6.0, -10.0, -1.0], [20.0, 10.0, 5.0]))
        .expect("the boss is cut");
    let share: f64 = 3.0 / MAJOR;
    let beyond = MAJOR * MINOR * (share.acos() - share * (1.0 - share * share).sqrt());
    held(&cut, (area() - beyond) * 4.0);
    let rulings = cut
        .edges
        .iter()
        .filter(|edge| {
            matches!(cut.curve(edge.curve), Curve::Line(line)
                if line.direction.cross(DVec3::Z).length() < 1e-12 && (line.origin.x - 6.0).abs() < 1e-9)
        })
        .count();
    assert_eq!(rulings, 2);
}

#[test]
fn a_block_s_side_touching_an_elliptic_boss_at_its_support_point_is_joined_along_one_ruling() {
    let boss = oval([3.0, -1.0], first(0.4), 0.0, 4.0);
    let wall = match boss.surfaces.iter().find_map(|surface| match surface {
        Surface::Elliptic(wall) => Some(*wall),
        _ => None,
    }) {
        Some(wall) => wall,
        None => panic!("the boss stands on an elliptic wall"),
    };
    let reach = wall.support(DVec3::X);
    let side = 3.0 + reach;
    let support = wall.origin
        + (wall.u * (MAJOR * MAJOR * wall.u.x) + wall.v * (MINOR * MINOR * wall.v.x)) / reach;
    let joined = boss
        .joined(&block([side, -6.0, 0.0], [side + 10.0, 4.0, 4.0]))
        .expect("the block is joined along the ruling it touches");
    held(&joined, area() * 4.0 + 100.0 * 4.0);
    let touching = joined
        .edges
        .iter()
        .filter(|edge| {
            matches!(joined.curve(edge.curve), Curve::Line(line)
                if line.direction.cross(DVec3::Z).length() < 1e-12
                    && line.origin.truncate().distance(support.truncate()) < 1e-9)
        })
        .count();
    assert_eq!(touching, 1, "one ruling of touch");
}

#[test]
fn a_block_s_side_a_hair_off_an_elliptic_boss_s_support_point_is_moved_onto_the_touch() {
    let boss = oval([3.0, -1.0], first(0.4), 0.0, 4.0);
    let Some(Surface::Elliptic(wall)) = boss
        .surfaces
        .iter()
        .find(|surface| matches!(surface, Surface::Elliptic(_)))
    else {
        panic!("the boss stands on an elliptic wall");
    };
    let eps = boss.scale().joined(Scale::of(20.0)).eps();
    for hair in [eps / 3.0, -eps / 3.0] {
        let side = 3.0 + wall.support(DVec3::X) + hair;
        let joined = boss
            .joined(&block([side, -6.0, 0.0], [side + 10.0, 4.0, 4.0]))
            .expect("the block is joined along the ruling it touches");
        held(&joined, area() * 4.0 + (10.0 - hair) * 10.0 * 4.0);
        let sides: Vec<f64> = joined
            .surfaces
            .iter()
            .filter_map(|surface| match surface {
                Surface::Plane(plane) if plane.normal == DVec3::X => Some(plane.offset()),
                _ => None,
            })
            .collect();
        let touch = 3.0 + wall.support(DVec3::X);
        assert!(
            sides
                .iter()
                .any(|offset| (offset - touch).abs() <= 1e-12 * 20.0),
            "{sides:?} for {touch}"
        );
    }
}

#[test]
fn an_elliptic_hole_touching_a_block_s_side_from_inside_is_cut() {
    let probe = oval([0.0, 0.0], first(0.4), 0.0, 1.0);
    let Some(Surface::Elliptic(wall)) = probe
        .surfaces
        .iter()
        .find(|surface| matches!(surface, Surface::Elliptic(_)))
    else {
        panic!("the probe stands on an elliptic wall");
    };
    let center = [3.0, 10.0 - wall.support(DVec3::Y)];
    let stock = block([-10.0, -10.0, 0.0], [15.0, 10.0, 4.0]);
    let hole = stock
        .cut_by(&oval(center, first(0.4), -1.0, 5.0))
        .expect("the hole is cut along the side it touches");
    held(&hole, 500.0 * 4.0 - area() * 4.0);
    let notch = stock
        .cut_by(&oval(center, first(0.4), 2.0, 5.0))
        .expect("the pocket is cut along the side it touches");
    held(&notch, 500.0 * 4.0 - area() * 2.0);
}

#[test]
fn a_round_post_beside_an_elliptic_boss_is_joined_where_their_faces_stand_clear_and_declined_where_they_cross()
 {
    let boss = oval([3.0, -1.0], first(0.4), 0.0, 4.0);
    let clear = boss
        .joined(&standing([9.3, 2.7], 0.1, 0.0, 4.0))
        .expect("a post standing clear of the boss is joined");
    held(&clear, area() * 4.0 + PI * 0.01 * 4.0);
    assert_eq!(
        boss.joined(&standing([9.3, -1.0], 1.0, 0.0, 4.0))
            .map(|_| ()),
        Err(Declined::Unsupported)
    );
}

#[test]
fn the_same_ellipse_cut_twice_is_one_surface() {
    let stock = block([-10.0, -10.0, 0.0], [15.0, 10.0, 4.0]);
    let tool = oval([3.0, -1.0], first(0.4), -1.0, 5.0);
    let once = stock.cut_by(&tool).expect("the hole is cut");
    let again = oval([3.0, -1.0], -first(0.4), -2.0, 6.0);
    let twice = once.cut_by(&again).expect("the same hole is cut again");
    held(&twice, 500.0 * 4.0 - area() * 4.0);
    let walls = twice
        .surfaces
        .iter()
        .filter(|surface| matches!(surface, Surface::Elliptic(_)))
        .count();
    assert_eq!(walls, 1);
}

/// A slab above the plane through `origin` whose normal is `normal`, wide
/// enough to cover every boss below.
fn slab(origin: DVec3, normal: DVec3) -> Body {
    let normal = normal.normalize();
    let u = normal.cross(DVec3::Y).normalize();
    let frame = Frame {
        origin,
        u,
        v: normal.cross(u),
    };
    Body::raised(
        &Contour::rectangle(DVec2::new(-20.0, -20.0), DVec2::new(20.0, 20.0)),
        &[],
        frame,
        normal * 20.0,
    )
    .expect("the slab raises")
}

#[test]
fn a_plane_at_a_slant_through_an_elliptic_boss_cuts_it_along_a_conic() {
    let boss = oval([3.0, -1.0], first(0.4), 0.0, 10.0);
    let tool = slab(DVec3::new(3.0, -1.0, 6.0), DVec3::new(0.3, 0.2, 1.0));
    let cut = boss.cut_by(&tool).expect("the boss is cut at a slant");
    held(&cut, area() * 6.0);
    let conics = cut
        .curves
        .iter()
        .filter(|curve| matches!(curve, Curve::Conic(_)))
        .count();
    assert!(conics >= 2, "{conics} conics");
}

#[test]
fn a_plane_leaning_a_hair_off_square_to_or_along_an_elliptic_wall_s_axis_is_declined() {
    let boss = oval([3.0, -1.0], first(0.4), 0.0, 10.0);
    let lean = 1e-5;
    for normal in [DVec3::new(lean, 0.0, 1.0), DVec3::new(1.0, 0.0, lean)] {
        let tool = slab(DVec3::new(4.0, -1.0, 6.0), normal);
        assert_eq!(
            boss.cut_by(&tool).map(|_| ()),
            Err(Declined::Unsupported),
            "{normal}"
        );
    }
    let level = boss
        .cut_by(&slab(DVec3::new(4.0, -1.0, 6.0), DVec3::Z))
        .expect("a level cut is built");
    held(&level, area() * 6.0);
}

#[test]
fn a_side_a_hair_inside_the_end_of_an_elliptic_boss_s_major_axis_leaves_a_strip_drawn_off_it() {
    let boss = ellipse(
        [3.0, 0.0],
        DVec2::from_angle(FRAC_PI_2) * 2.0,
        1.5,
        -1.0,
        4.0,
    );
    let side = 2.0 - 2e-8 / 3.0;
    let cut = boss
        .cut_by(&block([-1.5, -4.0, 1.0], [4.0, side, 2.5]))
        .expect("the boss is cut");
    drawn(&cut);
}

#[test]
fn a_side_a_hair_inside_an_elliptic_boss_s_support_point_leaves_a_strip_drawn_off_it() {
    let boss = ellipse([-2.0, 2.0], DVec2::from_angle(0.4) * 2.0, 1.6, 0.0, 4.0);
    let Some(Surface::Elliptic(wall)) = boss
        .surfaces
        .iter()
        .find(|surface| matches!(surface, Surface::Elliptic(_)))
    else {
        panic!("the boss stands on an elliptic wall");
    };
    let reach = wall.support(DVec3::X);
    let tool = block(
        [-2.0 - reach + 2e-8 / 3.0, 1.0, 2.0],
        [1.0 + reach, 2.5, 2.5],
    );
    for body in [boss.joined(&tool), boss.cut_by(&tool), tool.cut_by(&boss)] {
        drawn(&body.expect("the boss and the block are joined and cut"));
    }
}

#[test]
fn a_tilted_slot_with_elliptic_ends_cut_by_a_block_is_drawn_closed_and_uncrossed() {
    let center = DVec2::new(-2.0, -1.0);
    let along = DVec2::from_angle(PI / 4.0);
    let across = along.perp() * 1.5;
    let [left, right] = [center - along * 5.0, center + along * 5.0];
    let slot = Contour {
        corners: vec![left - across, right - across, right + across, left + across],
        runs: vec![
            Run::Straight,
            Run::Oval {
                center: right,
                first: -across,
                second: 1.0,
                turn: PI,
            },
            Run::Straight,
            Run::Oval {
                center: left,
                first: across,
                second: 0.7,
                turn: PI,
            },
        ],
    };
    let normal = DVec3::new(0.3_f64.sin(), 0.0, 0.3_f64.cos());
    let u = normal.cross(DVec3::Y).normalize();
    let frame = Frame {
        origin: DVec3::ZERO,
        u,
        v: normal.cross(u),
    };
    let boss = Body::raised(&slot, &[], frame, normal * 3.0).expect("the slot raises");
    let cut = boss
        .cut_by(&block([-2.0, -2.0, 1.0], [4.0, 1.0, 3.0]))
        .expect("the slot is cut");
    drawn(&cut);
}

#[test]
fn a_flat_elliptic_boss_cut_along_its_minor_axis_is_drawn_closed_and_uncrossed() {
    let boss = ellipse([1.5, -1.0], DVec2::new(7.0, 0.0), 1.0, 1.0, 3.0);
    let cut = boss
        .cut_by(&block([1.5, -1.999999, 1.0], [11.5, 0.000001, 3.0]))
        .expect("the boss is cut");
    drawn(&cut);
}
