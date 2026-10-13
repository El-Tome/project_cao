//! Planes leaning on an elliptic wall (#571): a slanted cut through an
//! elliptic boss and an elliptic hole drilled square into a slanted face are
//! built exactly; a plane a hair off square to the wall's axis, or along it,
//! is laid exactly so (decision 11), and one in the band past it is built
//! as drawn and noted unstable (step 10 of the boolean). Each held to its
//! listing and to the arithmetic's volume.

use std::f64::consts::{PI, TAU};

use glam::{DQuat, DVec2, DVec3};

use super::*;
use crate::brep::conic::Conic;
use crate::brep::surface::EllipticCylinder;
use crate::brep::{Lean, Unstable};
use crate::soundness::{closed, uncrossed};

/// The half-axes of the boss below, its major one turned 0.4 off X unless
/// said otherwise.
const MAJOR: f64 = 7.0;
const MINOR: f64 = 3.0;

fn area() -> f64 {
    PI * MAJOR * MINOR
}

/// The world's ground at `height`, turned by `turn` about `pivot`.
fn turned_ground(height: f64, pivot: DVec3, turn: DQuat) -> Frame {
    Frame {
        origin: pivot + turn * (DVec3::Z * height - pivot),
        u: turn * DVec3::X,
        v: turn * DVec3::Y,
    }
}

/// The ellipse about `center` whose first half-axis is `first`, the other
/// `second` long, raised `height` along the frame's normal.
fn oval_on(frame: Frame, center: [f64; 2], first: DVec2, second: f64, height: f64) -> Body {
    let center = DVec2::from(center);
    let outline = Contour {
        corners: vec![center + first],
        runs: vec![Run::Oval {
            center,
            first,
            second,
            turn: TAU,
        }],
    };
    Body::raised(&outline, &[], frame, frame.normal() * height).expect("an ellipse raises")
}

/// The boss: the ellipse about (3, −1), its major half-axis turned
/// `turned` off X, from 0 to `height`.
fn boss(turned: f64, height: f64) -> Body {
    oval_on(
        ground(0.0),
        [3.0, -1.0],
        DVec2::from_angle(turned) * MAJOR,
        MINOR,
        height,
    )
}

/// A block between two corners turned by `turn` about `pivot`.
fn turned_block(low: [f64; 3], high: [f64; 3], pivot: DVec3, turn: DQuat) -> Body {
    let outline = Contour::rectangle(DVec2::new(low[0], low[1]), DVec2::new(high[0], high[1]));
    let frame = turned_ground(low[2], pivot, turn);
    Body::raised(&outline, &[], frame, frame.normal() * (high[2] - low[2])).expect("a block raises")
}

fn degrees(angle: f64) -> f64 {
    angle.to_radians()
}

/// The body held to its listing, its volume to the arithmetic's within
/// `share` of it, and drawn closed.
fn held(body: &Body, volume: f64, share: f64) {
    assert_eq!(listed(&body.listing(), body.scale().reach()), Ok(()));
    assert!(
        (body.volume() - volume).abs() <= share * volume,
        "the body holds {}, the arithmetic {volume}",
        body.volume()
    );
    let triangles = body.triangles(1e-3 * body.scale().reach());
    closed(&triangles).expect("the triangles close");
    uncrossed(&triangles).expect("no triangle crosses another");
}

fn wall_of(body: &Body) -> EllipticCylinder {
    let walls: Vec<EllipticCylinder> = body
        .surfaces
        .iter()
        .filter_map(|surface| match surface {
            Surface::Elliptic(wall) => Some(*wall),
            _ => None,
        })
        .collect();
    let [wall] = walls[..] else {
        panic!("one elliptic wall: {walls:?}");
    };
    wall
}

fn conics(body: &Body) -> Vec<Conic> {
    body.edges
        .iter()
        .filter_map(|edge| match body.curve(edge.curve) {
            Curve::Conic(conic) => Some(*conic),
            _ => None,
        })
        .collect()
}

fn noted(unstable: Option<Unstable>, lean: Lean, degrees: f64) {
    let Some(unstable) = unstable else {
        panic!("a lean of {degrees}° is noted");
    };
    assert_eq!(unstable.lean, lean);
    assert!(
        (unstable.degrees - degrees).abs() <= 1e-6 * degrees,
        "the lean noted is {}°, against {degrees}°",
        unstable.degrees,
    );
}

/// A slab above the plane through the boss's axis at height 6, leaning
/// `degrees` about a direction of the ground off both half-axes.
fn slanted(degrees: f64) -> Body {
    let about = DVec3::new(1.1f64.cos(), 1.1f64.sin(), 0.0);
    turned_block(
        [-20.0, -20.0, 6.0],
        [20.0, 20.0, 30.0],
        DVec3::new(3.0, -1.0, 6.0),
        DQuat::from_axis_angle(about, degrees.to_radians()),
    )
}

#[test]
fn a_plane_at_a_slant_through_an_elliptic_boss_cuts_it_along_a_conic_at_a_large_angle_and_at_a_small_one()
 {
    let boss = boss(0.4, 12.0);
    for degrees in [30.0, 1.0, 0.1] {
        let cut = boss
            .cut_by(&slanted(degrees))
            .unwrap_or_else(|declined| panic!("{degrees}°: {declined:?}"));
        held(&cut, area() * 6.0, 1e-9);
        assert_eq!(cut.unstable(), None, "{degrees}°");
        let axis = wall_of(&cut).axis;
        let leaning = conics(&cut)
            .iter()
            .filter(|conic| conic.plane.normal.cross(axis).length() > 1e-3)
            .count();
        assert_eq!(leaning, 1, "{degrees}°: the cut's conic");
    }
}

#[test]
fn an_elliptic_hole_drilled_square_into_a_wedge_s_slant_through_its_floor_holds_the_arithmetics_volume()
 {
    let wedge = Body::raised(
        &Contour::straight(vec![
            DVec2::ZERO,
            DVec2::new(40.0, 0.0),
            DVec2::new(0.0, 20.0),
        ]),
        &[],
        Frame {
            origin: DVec3::ZERO,
            u: DVec3::X,
            v: DVec3::Z,
        },
        DVec3::NEG_Y * 20.0,
    )
    .expect("a wedge raises");
    let on_the_slant = Frame {
        origin: DVec3::new(20.0, -10.0, 10.0),
        u: DVec3::Y,
        v: DVec3::new(2.0, 0.0, -1.0).normalize(),
    };
    let hole = oval_on(
        on_the_slant,
        [0.0, 0.0],
        DVec2::from_angle(0.3) * 4.0,
        2.0,
        30.0,
    );
    let drilled = wedge.cut_by(&hole).expect("the hole is drilled");
    held(&drilled, 8000.0 - PI * 8.0 * 5.0 * 5f64.sqrt(), 1e-9);
    let axis = wall_of(&drilled).axis;
    let rims = conics(&drilled);
    let through_the_floor = rims
        .iter()
        .filter(|conic| conic.plane.normal == DVec3::Z)
        .count();
    assert_eq!(through_the_floor, 1, "the hole leaves through the floor");
    let into_the_slant = rims
        .iter()
        .filter(|conic| conic.plane.normal.cross(axis).length() <= 1e-12)
        .count();
    assert_eq!(into_the_slant, 1, "the hole enters square to the slant");
}

/// A slab above the plane through the boss's axis at height 6, leaning
/// `degrees` off square to it about X.
fn off_square(degrees: f64) -> Body {
    turned_block(
        [-20.0, -20.0, 6.0],
        [20.0, 20.0, 30.0],
        DVec3::new(3.0, -1.0, 6.0),
        DQuat::from_rotation_x(degrees.to_radians()),
    )
}

#[test]
fn a_block_a_ten_millionth_of_a_degree_off_square_to_an_elliptic_boss_cuts_it_along_its_rim_s_ellipse()
 {
    let cut = boss(0.4, 12.0)
        .cut_by(&off_square(1e-7))
        .expect("the boss is cut");
    held(&cut, area() * 6.0, 1e-8);
    assert_eq!(cut.unstable(), None);
    let axis = wall_of(&cut).axis;
    let rims = conics(&cut);
    assert_eq!(rims.len(), 2, "{rims:?}");
    for rim in rims {
        assert_eq!(rim.plane.normal, axis, "a rim square to the axis");
    }
}

#[test]
fn a_block_a_hundredth_of_a_degree_off_square_to_an_elliptic_boss_is_built_and_noted_unstable() {
    let cut = boss(0.4, 12.0)
        .cut_by(&off_square(1e-2))
        .expect("the boss is cut");
    held(&cut, area() * 6.0, 1e-9);
    noted(cut.unstable(), Lean::Square, 1e-2);
}

/// A block whose side stands at x = 6 across the boss, its major half-axis
/// along X, leaning `degrees` off along its axis about Y through the side's
/// middle.
fn off_along(degrees: f64) -> Body {
    turned_block(
        [6.0, -10.0, -1.0],
        [20.0, 10.0, 5.0],
        DVec3::new(6.0, -1.0, 2.0),
        DQuat::from_rotation_y(degrees.to_radians()),
    )
}

/// What a side through `(6, y, 2)` leaning `degrees` about Y leaves of the
/// boss of height 4 whose major half-axis runs along X: the ellipse less
/// the segment beyond `x = 6 + (z − 2) tan`, summed in closed form.
fn left_by_a_side(degrees: f64) -> f64 {
    let segment = |s: f64| s * s.acos() - (1.0 - s * s).sqrt() + (1.0 - s * s).powf(1.5) / 3.0;
    let beyond = if degrees == 0.0 {
        let s: f64 = 3.0 / MAJOR;
        MAJOR * MINOR * (s.acos() - s * (1.0 - s * s).sqrt()) * 4.0
    } else {
        let tan = degrees.to_radians().tan();
        let [low, high] = [-2.0, 2.0].map(|z: f64| (3.0 + z * tan) / MAJOR);
        MAJOR * MAJOR * MINOR / tan * (segment(high) - segment(low))
    };
    area() * 4.0 - beyond
}

#[test]
fn a_block_s_side_a_ten_millionth_of_a_degree_off_along_an_elliptic_boss_cuts_it_along_two_rulings()
{
    let cut = boss(0.0, 4.0)
        .cut_by(&off_along(1e-7))
        .expect("the boss is cut");
    held(&cut, left_by_a_side(0.0), 1e-8);
    assert_eq!(cut.unstable(), None);
    let axis = wall_of(&cut).axis;
    let rulings = cut
        .edges
        .iter()
        .filter(|edge| {
            matches!(cut.curve(edge.curve), Curve::Line(line)
                if line.direction == axis && (line.origin.x - 6.0).abs() < 1e-6)
        })
        .count();
    assert_eq!(rulings, 2);
}

#[test]
fn a_block_s_side_a_hundredth_of_a_degree_off_along_an_elliptic_boss_is_built_and_noted_unstable() {
    let cut = boss(0.0, 4.0)
        .cut_by(&off_along(1e-2))
        .expect("the boss is cut");
    held(&cut, left_by_a_side(1e-2), 1e-9);
    noted(cut.unstable(), Lean::Along, 1e-2);
}

#[test]
fn an_elliptic_hole_drilled_into_a_face_turned_a_ten_millionth_of_a_degree_is_merged_onto_a_pocket_of_its_ellipse()
 {
    let first = DVec2::from_angle(0.4) * MAJOR;
    let stock = block([-10.0, -10.0, 0.0], [15.0, 10.0, 10.0]);
    let pocketed = stock
        .cut_by(&oval_on(ground(6.0), [3.0, -1.0], first, MINOR, 5.0))
        .expect("the pocket is cut");
    let face = turned_ground(-1.0, DVec3::ZERO, DQuat::from_rotation_x(degrees(1e-7)));
    let hole = oval_on(face, [3.0, -1.0], first, MINOR, 12.0);
    let drilled = pocketed.cut_by(&hole).expect("the hole is drilled");
    held(&drilled, 25.0 * 20.0 * 10.0 - area() * 10.0, 1e-9);
    assert_eq!(wall_of(&drilled), wall_of(&pocketed));
}
