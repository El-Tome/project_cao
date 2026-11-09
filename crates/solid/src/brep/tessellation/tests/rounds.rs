//! Faces of a sphere or a torus cut into triangles (#576): closed against
//! their neighbours, every triangle within the tolerance of its surface, the
//! volume within what the chords take off, and the same twice.

use std::f64::consts::{FRAC_PI_2, PI, TAU};

use glam::DVec3;

use super::super::sampling::{Samples, divisions_in};
use crate::brep::Declined;
use crate::brep::curve::Curve;
use crate::brep::surface::Surface;
use crate::brep::tessellation::Cut;
use crate::brep::topology::Body;
use crate::brep::turned::tests::round::{
    apple, ball, ball_at_the_origin, bent, cap, grooved_bore, lemon, ring, rounded_shaft, turned,
    zone,
};
use crate::brep::turned::tests::{about_y, ground};
use crate::soundness::{closed, enclosed, repeatable, uncrossed};
use crate::turning::Straight;

/// A shaft of radius 6 from `h = 0` to `h = 20` with a round-bottomed
/// groove between `h = 8` and `h = 12`, cut along the circle of radius 2
/// about `(10, 6)`: a torus whose matter lies outside its tube.
fn grooved_shaft() -> Straight {
    bent(&[&[
        ([0.0, 0.0], None),
        ([0.0, 6.0], None),
        ([8.0, 6.0], Some(([10.0, 6.0], PI))),
        ([12.0, 6.0], None),
        ([20.0, 6.0], None),
        ([20.0, 0.0], None),
    ]])
}

/// The triangles of a body, held to the rules: every face cut, closed,
/// uncrossed, every triangle on a curved face standing within `tolerance`
/// of its surface, enclosing the body's own volume within what the chords
/// take off the curved faces, and the same when drawn again.
fn held(body: &Body, tolerance: f64) -> Cut {
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
    let exact = body.volume();
    let volume = enclosed(&cut.triangles);
    assert!(
        (volume - exact).abs() <= tolerance * curved * 1.01 + 1e-9 * exact,
        "the triangles enclose {volume}, the body {exact}, within {tolerance}"
    );
    repeatable(&cut.triangles, &body.triangles(tolerance)).expect("drawn the same twice");
    cut
}

/// The triangles of the faces of a body lying on a sphere or a torus.
fn on_rounds(body: &Body, cut: &Cut) -> Vec<[DVec3; 3]> {
    cut.triangles
        .iter()
        .zip(&cut.faces)
        .filter(|(_, face)| {
            matches!(
                body.surface(body.face(**face).surface),
                Surface::Sphere(_) | Surface::Torus(_)
            )
        })
        .map(|(corners, _)| *corners)
        .collect()
}

const TOLERANCES: [f64; 3] = [1e-3, 0.01, 0.5];

#[test]
fn a_ball_turned_whole_is_drawn_closed_from_its_poles_alone_and_encloses_its_volume() {
    let body = turned(&ball(), 360.0);
    let whole = 4.0 / 3.0 * PI * 125.0;
    assert!((body.volume() - whole).abs() <= 1e-9 * whole);
    for tolerance in TOLERANCES {
        let cut = held(&body, tolerance);
        let round = on_rounds(&body, &cut);
        assert_eq!(round.len(), cut.triangles.len(), "one face, on the sphere");
        assert!(round.len() >= 16 * 8, "{} triangles", round.len());
    }
}

#[test]
fn a_ball_turned_whole_with_no_edge_at_all_is_capped_once_its_grid_needs_more_than_the_most_steps()
{
    let body = turned(&ball(), 360.0);
    assert_eq!(body.edge_ids().count(), 0, "a whole ball has no edge");
    let capped = |tolerance: f64| Samples::of(&body, tolerance).capped(&body, tolerance);

    assert!(
        !capped(1e-3),
        "a turn of 5 within 1e-3 takes 236 steps on the sphere"
    );
    assert!(
        capped(2e-5),
        "a turn of 5 within 2e-5 needs 1656 steps on the sphere"
    );
}

#[test]
fn a_lune_and_a_half_ball_are_drawn_closed_with_a_fan_at_each_pole() {
    let poles = [DVec3::new(0.0, -5.0, 0.0), DVec3::new(0.0, 5.0, 0.0)];
    for degrees in [90.0, 180.0, 270.0, -120.0, 45.0] {
        for tolerance in TOLERANCES {
            let body = turned(&ball(), degrees);
            let cut = held(&body, tolerance);
            for pole in poles {
                assert!(
                    on_rounds(&body, &cut)
                        .iter()
                        .any(|corners| corners.contains(&pole)),
                    "no fan at {pole} at {degrees}°"
                );
            }
        }
    }
}

#[test]
fn a_ball_whose_pole_stands_at_the_frame_s_origin_is_drawn_closed() {
    for degrees in [360.0, 225.0, 90.0, -225.0] {
        held(
            &Body::turned(&ball_at_the_origin(), ground(), &about_y(degrees))
                .expect("a ball turns"),
            0.01,
        );
    }
}

#[test]
fn a_zone_and_a_cap_are_drawn_closed_whole_and_turned_part_way() {
    for degrees in [360.0, 90.0, 180.0, 250.0, -60.0] {
        for tolerance in TOLERANCES {
            held(&turned(&zone(), degrees), tolerance);
            held(&turned(&cap(), degrees), tolerance);
        }
    }
}

#[test]
fn a_ring_torus_is_drawn_closed_whole_and_turned_a_quarter() {
    for degrees in [360.0, 90.0, 180.0, 300.0, -90.0] {
        for tolerance in TOLERANCES {
            let body = turned(&ring(), degrees);
            let cut = held(&body, tolerance);
            if degrees == 360.0 {
                assert_eq!(on_rounds(&body, &cut).len(), cut.triangles.len());
            }
        }
    }
}

#[test]
fn a_rounded_shaft_end_shares_its_shaft_s_circle_and_is_drawn_watertight() {
    for degrees in [360.0, 90.0, 180.0, 300.0, -90.0] {
        for tolerance in TOLERANCES {
            let body = turned(&rounded_shaft(), degrees);
            held(&body, tolerance);
        }
    }
    let body = turned(&rounded_shaft(), 360.0);
    let tolerance = 0.01;
    let samples = Samples::of(&body, tolerance);
    let [equator] = body
        .edge_ids()
        .filter(|&edge| match body.curve(body.edge(edge).curve) {
            Curve::Circle(circle) => (circle.center.y - 10.0).abs() < 1e-12,
            Curve::Line(_) | Curve::Meet(_) | Curve::Conic(_) => false,
        })
        .collect::<Vec<_>>()[..]
    else {
        panic!("one circle where the ball meets the shaft");
    };
    let points: Vec<DVec3> = samples
        .edge(equator)
        .iter()
        .map(|&id| samples.point(id))
        .collect();
    assert!(
        every_angle(&points, divisions_in(4.0, tolerance, body.scale().reach())),
        "the shaft's grid"
    );
    assert!(
        points.len() > divisions_in(4.0, tolerance, body.scale().reach()),
        "the sphere's grid as well: {} samples",
        points.len()
    );
}

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
fn a_round_bottomed_groove_is_drawn_closed_inside_its_shaft() {
    for degrees in [360.0, 120.0, -200.0] {
        for tolerance in TOLERANCES {
            held(&turned(&grooved_shaft(), degrees), tolerance);
        }
    }
}

#[test]
fn a_round_groove_in_a_bore_is_drawn_closed_with_the_matter_outside_the_tube() {
    for degrees in [360.0, 90.0, -180.0] {
        for tolerance in [0.01, 0.1, 2.0] {
            let body = turned(&grooved_bore(), degrees);
            let cut = held(&body, tolerance);
            assert!(!on_rounds(&body, &cut).is_empty());
        }
    }
}

#[test]
fn an_apple_and_a_lemon_are_drawn_closed_with_a_fan_at_each_pole_on_the_axis() {
    for (straight, tips) in [(apple(), 3f64.sqrt()), (lemon(), 3f64.sqrt())] {
        let poles = [DVec3::new(0.0, -tips, 0.0), DVec3::new(0.0, tips, 0.0)];
        for degrees in [360.0, 90.0, -200.0] {
            for tolerance in TOLERANCES {
                let body = turned(&straight, degrees);
                let cut = held(&body, tolerance);
                for pole in poles {
                    assert!(
                        on_rounds(&body, &cut)
                            .iter()
                            .any(|corners| corners.iter().any(|c| c.distance(pole) < 1e-9)),
                        "no fan at {pole} at {degrees}°"
                    );
                }
            }
        }
    }
}

#[test]
fn a_horn_is_declined_before_any_triangle_is_asked_of_it() {
    let horn = bent(&[&[
        ([0.0, 0.0], None),
        ([0.0, 4.0], None),
        ([10.0, 4.0], Some(([10.0, 2.0], -PI))),
        ([10.0, 0.0], None),
    ]]);
    assert_eq!(
        Body::turned(&horn, ground(), &about_y(360.0)),
        Err(Declined::Profile)
    );
}

#[test]
fn both_meridians_of_a_partial_turn_are_sampled_on_the_same_rows_of_the_sphere() {
    let tolerance = 0.01;
    let body = turned(&zone(), 150.0);
    let samples = Samples::of(&body, tolerance);
    let meridians: Vec<Vec<f64>> = body
        .edge_ids()
        .filter(|&edge| match body.curve(body.edge(edge).curve) {
            Curve::Circle(circle) => circle.center == DVec3::ZERO,
            Curve::Line(_) | Curve::Meet(_) | Curve::Conic(_) => false,
        })
        .map(|edge| {
            let mut latitudes: Vec<f64> = samples
                .edge(edge)
                .iter()
                .filter(|&&id| !samples.is_vertex(id))
                .map(|&id| {
                    let point = samples.point(id);
                    point.y.atan2(point.x.hypot(point.z))
                })
                .collect();
            latitudes.sort_by(f64::total_cmp);
            latitudes
        })
        .collect();
    let [opening, closing] = &meridians[..] else {
        panic!("two meridians, {} found", meridians.len());
    };
    assert!(opening.len() >= 4, "{opening:?}");
    assert_eq!(opening.len(), closing.len());
    for (one, other) in opening.iter().zip(closing) {
        assert!((one - other).abs() < 1e-9, "{one} against {other}");
    }
    let rows = divisions_in(5.0, tolerance * 0.45, body.scale().reach()) as f64;
    for latitude in opening {
        let rank = latitude * rows / TAU;
        assert!(
            (rank - rank.round()).abs() < 1e-9 && latitude.abs() < FRAC_PI_2,
            "{latitude} off the rows of the meridian"
        );
    }
}
