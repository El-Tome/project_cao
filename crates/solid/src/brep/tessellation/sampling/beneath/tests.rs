use std::collections::BTreeMap;
use std::f64::consts::{PI, TAU};

use glam::{DVec2, DVec3};

use super::super::Samples;
use super::walls;
use crate::brep::conic::Round;
use crate::brep::curve::Curve;
use crate::brep::tessellation::contact;
use crate::brep::topology::{Body, EdgeId};
use crate::profile::{Contour, Frame, Run};
use crate::turning::{Axis, Corner, Straight, Turn};

/// Coarse enough that every round here is cut into sixteen steps, the
/// fewest, whatever its radius.
const TOLERANCE: f64 = 0.1;

/// The sag of a chord of one step of sixteen on a circle of `radius`.
fn sag(radius: f64) -> f64 {
    radius * (1.0 - (PI / 16.0).cos())
}

fn round(center: DVec2, radius: f64) -> Contour {
    Contour {
        corners: vec![center + DVec2::X * radius],
        runs: vec![Run::Round { center, turn: TAU }],
    }
}

/// A shaft about Z of radius 5 from z 0 to 10, bored through along X at z 5
/// by a hole of radius 1, whose wall holds a face from one side of the
/// shaft to the other.
fn bored_shaft() -> Body {
    let ground = Frame {
        origin: DVec3::ZERO,
        u: DVec3::X,
        v: DVec3::Y,
    };
    let shaft = Body::raised(&round(DVec2::ZERO, 5.0), &[], ground, DVec3::Z * 10.0)
        .expect("the shaft raises");
    let across = Frame {
        origin: DVec3::new(-20.0, 0.0, 5.0),
        u: DVec3::Y,
        v: DVec3::Z,
    };
    let bore = Body::raised(&round(DVec2::ZERO, 1.0), &[], across, DVec3::X * 40.0)
        .expect("the bore raises");
    shaft.cut_by(&bore).expect("the bore is cut")
}

/// A frustum about Y of radius 1 at y 0 and 2 at y 5, joined by an
/// elliptic shaft along Y of half-axes 2 along X and 1.5, 1e-3 off its
/// axis along X: the cone's rims lie on no cylinder of the body, and the
/// ellipse crosses the cap at y 5.
fn frustum_beside_an_elliptic_shaft() -> Body {
    let corners = [[0.0, 0.0], [5.0, 0.0], [5.0, 2.0], [0.0, 1.0]];
    let straight = Straight {
        side: -1.0,
        contours: vec![
            corners
                .iter()
                .zip(0..)
                .map(|(at, run)| Corner {
                    at: DVec2::from(*at),
                    run,
                    bend: None,
                })
                .collect(),
        ],
        runs: corners.len() as u32,
        last_off_the_axis: Some(corners.len() as u32 - 1),
    };
    let frame = Frame {
        origin: DVec3::ZERO,
        u: DVec3::X,
        v: DVec3::Y,
    };
    let turn = Turn {
        axis: Axis {
            origin: DVec2::ZERO,
            direction: DVec2::Y,
        },
        angle: TAU,
        resolution: 0.0,
        on_the_axis: 0.0,
    };
    let frustum = Body::turned(&straight, frame, &turn).expect("the frustum turns");
    let oval = Contour {
        corners: vec![DVec2::X * 2.0],
        runs: vec![Run::Oval {
            center: DVec2::ZERO,
            first: DVec2::X * 2.0,
            second: 1.5,
            turn: TAU,
        }],
    };
    let ground = Frame {
        origin: DVec3::new(1e-3, -20.0, 0.0),
        u: DVec3::X,
        v: DVec3::NEG_Z,
    };
    let shaft = Body::raised(&oval, &[], ground, DVec3::Y * 45.0).expect("the shaft raises");
    frustum.joined(&shaft).expect("the shaft is joined")
}

/// The samples of a body with `points` as the only samples between the
/// ends of `edge`, and nothing between the ends of any other.
fn with_samples_on(body: &Body, edge: EdgeId, points: &[DVec3]) -> Samples {
    let mut all: Vec<DVec3> = body.vertex_ids().map(|id| body.vertex(id).point).collect();
    let vertices = all.len();
    all.extend_from_slice(points);
    let edges = body
        .edge_ids()
        .map(|id| {
            if id == edge {
                (vertices..vertices + points.len()).collect()
            } else {
                Vec::new()
            }
        })
        .collect();
    Samples {
        points: all,
        edges,
        vertices,
        grids: BTreeMap::new(),
        lines: Vec::new(),
        span: 20.0,
    }
}

/// The bored shaft, its bore's wall, the shaft's rim at z 0 — a circle
/// about another axis — and the samples given that rim.
fn on_the_bore(points: &[DVec3]) -> Vec<DVec3> {
    let body = bored_shaft();
    let wall = contact::walls(&body)
        .into_iter()
        .find(|(_, wall)| (wall.radius - 1.0).abs() < 1e-9)
        .expect("the bore is a wall");
    let rim = body
        .edge_ids()
        .find(|id| match body.curve(body.edge(*id).curve) {
            Curve::Circle(circle) => {
                circle.center.z.abs() < 1e-9 && circle.axis.cross(DVec3::Z).length() < 1e-9
            }
            _ => false,
        })
        .expect("the shaft has a rim at z 0");
    let beneath = walls(
        &body,
        &[wall],
        &with_samples_on(&body, rim, points),
        TOLERANCE,
    );
    beneath.get(&wall.0).cloned().unwrap_or_default()
}

/// A point `depth` inside the bore's wall, `x` along it.
fn inside_the_bore(x: f64, depth: f64) -> DVec3 {
    DVec3::new(x, 1.0 - depth, 5.0)
}

#[test]
fn a_wall_holding_a_face_takes_a_ray_from_a_sample_just_inside_it_on_that_face() {
    let near = inside_the_bore(0.0, 0.1 * sag(1.0));
    assert_eq!(on_the_bore(&[near]), vec![near]);
}

#[test]
fn a_wall_takes_no_ray_from_a_sample_inside_it_where_its_face_does_not_reach() {
    let past = [
        inside_the_bore(20.0, 0.1 * sag(1.0)),
        inside_the_bore(-30.0, 0.1 * sag(1.0)),
    ];
    assert_eq!(on_the_bore(&past), Vec::<DVec3>::new());
}

#[test]
fn a_wall_takes_a_ray_from_a_sample_within_twice_a_chord_s_sag_inside_it_and_none_past() {
    let within = inside_the_bore(0.0, 1.9 * sag(1.0));
    let past = inside_the_bore(0.0, 2.1 * sag(1.0));
    let outside = inside_the_bore(0.0, -0.1 * sag(1.0));
    assert_eq!(on_the_bore(&[within, past, outside]), vec![within]);
}

#[test]
fn a_stand_in_wall_under_a_cone_s_rim_takes_a_ray_from_a_sample_on_its_cap_and_none_off_it() {
    let body = frustum_beside_an_elliptic_shaft();
    let (id, wall) = contact::walls(&body)
        .into_iter()
        .find(|(id, wall)| id.0 as usize >= body.surfaces.len() && (wall.radius - 2.0).abs() < 1e-9)
        .expect("the cone's cap rim stands on a stand-in wall");
    let oval = body
        .edge_ids()
        .find(|id| match body.curve(body.edge(*id).curve) {
            Curve::Conic(conic) => {
                matches!(conic.round, Round::Elliptic(_))
                    && (conic.point(body.edge(*id).from).y - 5.0).abs() < 1e-9
            }
            _ => false,
        })
        .expect("the ellipse crosses the cap");
    let depth = 0.5 * sag(2.0);
    let on_the_cap = DVec3::new(0.0, 5.0, -(2.0 - depth));
    let off_the_cap = DVec3::new(0.0, 5.5, -(2.0 - depth));
    let past = DVec3::new(0.0, 5.0, -(2.0 - 2.1 * sag(2.0)));
    let samples = with_samples_on(&body, oval, &[on_the_cap, off_the_cap, past]);
    let beneath = walls(&body, &[(id, wall)], &samples, TOLERANCE);
    assert_eq!(
        beneath.get(&id).cloned().unwrap_or_default(),
        vec![on_the_cap]
    );
}

#[test]
fn a_wall_takes_no_ray_from_a_sample_whose_place_on_it_only_another_surface_s_face_holds() {
    let read_on_the_shaft = inside_the_bore(8.0, 0.1 * sag(1.0));
    assert_eq!(on_the_bore(&[read_on_the_shaft]), Vec::<DVec3>::new());
}
