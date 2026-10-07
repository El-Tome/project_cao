use std::f64::consts::TAU;

use glam::{DVec2, DVec3};

use super::super::sampling::{Samples, divisions};
use crate::brep::curve::Curve;
use crate::brep::surface::Surface;
use crate::brep::tessellation::Cut;
use crate::brep::topology::Body;
use crate::profile::Frame;
use crate::soundness::{closed, enclosed, uncrossed};
use crate::turning::{Axis, Corner, Straight, Turn};

/// A profile on XY, its corners `(h, r)` read along Y and away from it along
/// X, turned about Y by `degrees`.
fn turned(corners: &[[f64; 2]], degrees: f64) -> Body {
    let count = corners.len() as u32;
    let straight = Straight {
        side: -1.0,
        contours: vec![
            corners
                .iter()
                .zip(0..)
                .map(|(at, run)| Corner {
                    at: DVec2::from(*at),
                    run,
                })
                .collect(),
        ],
        runs: count,
        last_off_the_axis: Some(count - 1),
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
        angle: degrees.to_radians(),
        resolution: 0.0,
        on_the_axis: 0.0,
    };
    Body::turned(&straight, frame, &turn).expect("the profile turns")
}

/// What the profile turned by `degrees` holds, by Pappus.
fn pappus(corners: &[[f64; 2]], degrees: f64) -> f64 {
    let count = corners.len();
    let integral: f64 = (0..count)
        .map(|index| {
            let [one, next] = [corners[index], corners[(index + 1) % count]].map(DVec2::from);
            one.perp_dot(next) * (one.y + next.y)
        })
        .sum();
    degrees.to_radians().abs() * integral.abs() / 6.0
}

/// The triangles of a body, held to the rules: closed, uncrossed, every one
/// on a curved face standing within `tolerance` of its surface, and enclosing
/// `exact` within what the chords take off the curved faces.
fn held(body: &Body, tolerance: f64, exact: f64) -> Cut {
    let cut = body.triangles_by_face(tolerance);
    assert!(cut.uncut.is_empty(), "faces left open: {:?}", cut.uncut);
    closed(&cut.triangles).expect("the triangles close");
    uncrossed(&cut.triangles).expect("no triangle crosses another");
    let mut curved = 0.0;
    for (corners, face) in cut.triangles.iter().zip(&cut.faces) {
        let surface = body.surface(body.face(*face).surface);
        if let Surface::Plane(_) = surface {
            continue;
        }
        curved += (corners[1] - corners[0])
            .cross(corners[2] - corners[0])
            .length()
            / 2.0;
        for one in 0..=8 {
            for other in 0..=8 - one {
                let (a, b) = (one as f64 / 8.0, other as f64 / 8.0);
                let point = corners[0] * (1.0 - a - b) + corners[1] * a + corners[2] * b;
                let off = surface.distance(point).abs();
                assert!(
                    off <= tolerance * (1.0 + 1e-9) + 1e-12,
                    "a triangle of {face:?} stands {off} off its surface, within {tolerance}: {surface:?}"
                );
            }
        }
    }
    let volume = enclosed(&cut.triangles);
    assert!(
        (volume - exact).abs() <= tolerance * curved * 1.01 + 1e-9 * exact,
        "the triangles enclose {volume}, the body {exact}, within {tolerance}"
    );
    cut
}

const STEPPED_SHAFT: [[f64; 2]; 8] = [
    [0.0, 0.0],
    [20.0, 0.0],
    [20.0, 4.0],
    [12.0, 4.0],
    [12.0, 7.0],
    [5.0, 7.0],
    [5.0, 10.0],
    [0.0, 10.0],
];

#[test]
fn a_shaft_with_no_cone_draws_as_on_main() {
    let tolerance = 0.01;
    let drawn: Vec<usize> = [360.0, 90.0, -200.0]
        .into_iter()
        .map(|degrees| {
            let body = turned(&STEPPED_SHAFT, degrees);
            let samples = Samples::of(&body, tolerance);
            for edge in body.edge_ids() {
                let Curve::Circle(circle) = body.curve(body.edge(edge).curve) else {
                    continue;
                };
                let steps = divisions(circle.radius, tolerance) as f64;
                for &id in samples.edge(edge) {
                    if samples.is_vertex(id) {
                        continue;
                    }
                    let from = samples.point(id) - circle.center;
                    let angle = from.dot(circle.v).atan2(from.dot(circle.u));
                    let rank = angle * steps / TAU;
                    assert!(
                        (rank - rank.round()).abs() < 1e-9,
                        "a sample of a circle of radius {} at {angle}, off its grid",
                        circle.radius
                    );
                }
            }
            body.triangles(tolerance).len()
        })
        .collect();
    assert_eq!(drawn, [716, 192, 416]);
}

/// A point of radius 5, its apex 10 along Y.
const POINT: [[f64; 2]; 3] = [[0.0, 0.0], [10.0, 0.0], [0.0, 5.0]];

/// The triangles of the faces of a body lying on a cone.
fn on_cones(body: &Body, cut: &Cut) -> Vec<[DVec3; 3]> {
    cut.triangles
        .iter()
        .zip(&cut.faces)
        .filter(|(_, face)| matches!(body.surface(body.face(**face).surface), Surface::Cone(_)))
        .map(|(corners, _)| *corners)
        .collect()
}

#[test]
fn a_point_turned_whole_is_drawn_closed_with_a_fan() {
    let apex = DVec3::new(0.0, 10.0, 0.0);
    for tolerance in [1e-3, 0.01, 0.5] {
        let body = turned(&POINT, 360.0);
        let cut = held(&body, tolerance, pappus(&POINT, 360.0));
        let fan = on_cones(&body, &cut);
        assert!(fan.len() >= 16, "{} triangles", fan.len());
        for corners in fan {
            assert!(
                corners.contains(&apex),
                "{corners:?} has no corner at the apex"
            );
        }
    }
}

#[test]
fn a_partial_point_is_drawn_closed_at_its_apex() {
    let apex = DVec3::new(0.0, 10.0, 0.0);
    for degrees in [90.0, 180.0, 270.0, -135.0, 359.0] {
        for tolerance in [1e-3, 0.01, 0.5] {
            let body = turned(&POINT, degrees);
            let cut = held(&body, tolerance, pappus(&POINT, degrees));
            for corners in on_cones(&body, &cut) {
                assert!(
                    corners.contains(&apex),
                    "{corners:?} has no corner at the apex, at {degrees}°"
                );
            }
        }
    }
}

/// A shaft of radius 10 and length 30 along Y, its far end chamfered by 2.
const CHAMFERED_SHAFT: [[f64; 2]; 5] = [
    [0.0, 0.0],
    [30.0, 0.0],
    [30.0, 8.0],
    [28.0, 10.0],
    [0.0, 10.0],
];

#[test]
fn a_chamfered_shaft_is_drawn_closed_and_within_the_tolerance() {
    for degrees in [360.0, 90.0, -200.0] {
        for tolerance in [1e-4, 1e-3, 0.02, 0.5, 5.0] {
            let body = turned(&CHAMFERED_SHAFT, degrees);
            held(&body, tolerance, pappus(&CHAMFERED_SHAFT, degrees));
        }
    }
}

/// A ring whose inside opens from a radius of 1 at its foot to 9 at its top.
const FUNNEL: [[f64; 2]; 4] = [[0.0, 1.0], [0.0, 12.0], [8.0, 12.0], [8.0, 9.0]];

/// A cone from a radius of 1 to one of 9 over a height of 8, standing on a
/// disc.
const FRUSTUM: [[f64; 2]; 4] = [[0.0, 0.0], [8.0, 0.0], [8.0, 1.0], [0.0, 9.0]];

/// A cone all but a disc, from a radius of 1 to one of 20 over a height of a
/// half.
const SHALLOW: [[f64; 2]; 5] = [[0.0, 0.0], [2.0, 0.0], [2.0, 1.0], [1.5, 20.0], [0.0, 20.0]];

#[test]
fn a_cone_between_a_small_and_a_large_rim_stays_within_the_tolerance() {
    for corners in [&SHALLOW[..], &FUNNEL[..], &FRUSTUM[..]] {
        for degrees in [360.0, 45.0, -300.0] {
            for tolerance in [1e-4, 1e-3, 0.02, 0.5, 5.0] {
                let body = turned(corners, degrees);
                held(&body, tolerance, pappus(corners, degrees));
            }
        }
    }
}

/// A spindle pointed at both ends, 3 wide at 4 along Y, its tips at 0 and 10.
const SPINDLE: [[f64; 2]; 3] = [[0.0, 0.0], [10.0, 0.0], [4.0, 3.0]];

#[test]
fn a_spindle_pointed_at_both_ends_is_drawn_closed_with_a_fan_at_each_tip() {
    let tips = [DVec3::ZERO, DVec3::new(0.0, 10.0, 0.0)];
    for degrees in [360.0, 100.0, -250.0] {
        for tolerance in [1e-3, 0.05, 2.0] {
            let body = turned(&SPINDLE, degrees);
            let cut = held(&body, tolerance, pappus(&SPINDLE, degrees));
            for corners in on_cones(&body, &cut) {
                assert!(
                    tips.iter().any(|tip| corners.contains(tip)),
                    "{corners:?} has no corner at a tip, at {degrees}°"
                );
            }
        }
    }
}

/// A cone from a radius of 2 to one of 5, then another back to 3, on a
/// bore's floor.
const TWO_CONES: [[f64; 2]; 5] = [[0.0, 0.0], [8.0, 0.0], [8.0, 3.0], [3.0, 5.0], [0.0, 2.0]];

/// A cone a hair from a cylinder: from a radius of 5 to one of 5.001 over
/// 10, its apex fifty thousand away.
const ALL_BUT_A_CYLINDER: [[f64; 2]; 4] = [[0.0, 0.0], [10.0, 0.0], [10.0, 5.001], [0.0, 5.0]];

#[test]
fn cones_meeting_at_a_rim_or_all_but_a_cylinder_stay_within_the_tolerance() {
    for corners in [&TWO_CONES[..], &ALL_BUT_A_CYLINDER[..]] {
        for degrees in [360.0, 30.0, -359.0] {
            for tolerance in [1e-3, 0.05, 2.0] {
                let body = turned(corners, degrees);
                held(&body, tolerance, pappus(corners, degrees));
            }
        }
    }
}

/// A countersink's tool: a bore of radius 2 from -1 to 6 along Y, opening to
/// a radius of 7 at 11.
const COUNTERSINK: [[f64; 2]; 5] = [
    [-1.0, 0.0],
    [-1.0, 2.0],
    [6.0, 2.0],
    [11.0, 7.0],
    [11.0, 0.0],
];

/// Whether every angle of a grid of `steps` steps is among the angles of
/// `points` about Y.
fn every_angle(points: &[DVec3], steps: usize) -> bool {
    let ranks: Vec<f64> = points
        .iter()
        .map(|point| (-point.z).atan2(point.x).rem_euclid(TAU) * steps as f64 / TAU)
        .collect();
    (0..steps).all(|step| {
        ranks.iter().any(|rank| {
            let off = (rank - step as f64).abs();
            off < 1e-9 || (off - steps as f64).abs() < 1e-9
        })
    })
}

#[test]
fn a_countersink_s_narrow_rim_shares_its_bore_s_samples() {
    for degrees in [360.0, 120.0] {
        for tolerance in [1e-3, 0.02, 0.5] {
            let body = turned(&COUNTERSINK, degrees);
            held(&body, tolerance, pappus(&COUNTERSINK, degrees));
            if degrees != 360.0 {
                continue;
            }
            let samples = Samples::of(&body, tolerance);
            let [narrow] = body
                .edge_ids()
                .filter(|&edge| match body.curve(body.edge(edge).curve) {
                    Curve::Circle(circle) => circle.radius == 2.0 && circle.center.y == 6.0,
                    Curve::Line(_) | Curve::Meet(_) => false,
                })
                .collect::<Vec<_>>()[..]
            else {
                panic!("one edge runs round the narrow rim");
            };
            let points: Vec<DVec3> = samples
                .edge(narrow)
                .iter()
                .map(|&id| samples.point(id))
                .collect();
            assert!(
                every_angle(&points, divisions(2.0, tolerance)),
                "the bore's grid"
            );
            assert!(
                every_angle(&points, divisions(7.0, tolerance)),
                "the cone's grid"
            );
        }
    }
}
