//! Faces of a sphere or a torus cut into triangles (#576): closed against
//! their neighbours, every triangle within the tolerance of its surface, the
//! volume within what the chords take off, and the same twice. A piece of
//! sphere bounded by great-circle arcs that are no line of the sphere's
//! `(θ, φ)` chart, neither a meridian nor the equator, is held to the same
//! (#800), on a sector of a ball built by hand: one over a narrow triangle,
//! and one over a wide triangle whose corners stand eighty degrees from its
//! middle, as a convex corner of sharp angles leaves its piece of sphere.
//! The ball less such a sector leaves a face of the sphere that lies in
//! front of no one direction, more than a hemisphere: it is left open, never
//! drawn inside out.

use std::f64::consts::{FRAC_PI_2, PI, TAU};

use glam::DVec3;

use super::super::sampling::{Samples, divisions_in};
use super::fixtures::Build;
use crate::brep::Declined;
use crate::brep::curve::{Circle, Curve};
use crate::brep::surface::{Sphere, Surface};
use crate::brep::tessellation::Cut;
use crate::brep::topology::{Body, Coedge, FaceId, VertexId};
use crate::brep::turned::tests::round::{
    apple, ball, ball_at_the_origin, bent, cap, grooved_bore, lemon, ring, rounded_shaft, turned,
    zone,
};
use crate::brep::turned::tests::{about_y, ground};
use crate::soundness::{closed, enclosed, listed, repeatable, uncrossed};
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

/// What of the ball a fixture keeps: the sector over a triangle of great
/// circles, or the ball less that sector. The rest's volume is not read: its
/// face of the sphere holds both poles round a loop that winds round neither,
/// a face no step builds, and the flux leaves out the ball they close.
#[derive(Clone, Copy)]
enum Kept {
    Sector,
    Rest,
}

/// The sector of the ball of `radius` about `centre` over the triangle of
/// great circles whose corners stand the ways `corners` point from it, or
/// the ball less that sector, its sphere read about `axis`: three flat faces
/// meeting at the centre, each between two of those ways and bounded
/// towards the sphere by the arc of the great circle through them, and the
/// face of the sphere the three arcs bound — the triangle, or the rest of
/// the sphere round it, which holds the sphere's two poles. Built by hand:
/// no boolean cuts a sphere along a plane through its centre that is
/// neither square to its axis nor holds it.
fn wedge(centre: DVec3, radius: f64, corners: [DVec3; 3], axis: DVec3, kept: Kept) -> Body {
    let ways = counterclockwise(corners);
    let mut build = Build::new();
    let middle = build.vertex(centre);
    let feet = ways.map(|way| build.vertex(centre + way * radius));
    let arcs = [0, 1, 2].map(|at| {
        let next = (at + 1) % 3;
        let turn = ways[at].cross(ways[next]);
        let circle = Circle::about(centre, turn, radius);
        let [from, to] = if circle.axis.dot(turn) > 0.0 {
            [at, next]
        } else {
            [next, at]
        };
        let start = circle.parameter(centre + ways[from] * radius);
        let end = circle.parameter(centre + ways[to] * radius);
        build.edge(
            Curve::Circle(circle),
            Some([feet[from], feet[to]]),
            start,
            start + (end - start).rem_euclid(TAU),
        )
    });
    let round = Sphere::through(centre, axis, 0.0, radius);
    let poles: Vec<VertexId> = match kept {
        Kept::Sector => Vec::new(),
        Kept::Rest => [-radius, radius]
            .map(|side| build.vertex(round.middle() + round.axis * side))
            .to_vec(),
    };
    let sphere = build.surface(Surface::Sphere(round));
    let lap: Vec<Coedge> = match kept {
        Kept::Sector => [0, 1, 2].map(|at| build.leaving(arcs[at], feet[at])),
        Kept::Rest => [2, 1, 0].map(|at| build.leaving(arcs[at], feet[(at + 1) % 3])),
    }
    .to_vec();
    let piece = build.face(sphere, false, vec![lap]);
    for (at, &arc) in arcs.iter().enumerate() {
        let next = (at + 1) % 3;
        let [first, second] = match kept {
            Kept::Sector => [next, at],
            Kept::Rest => [at, next],
        };
        let (plane, flipped) = build.plane(centre, ways[first].cross(ways[second]));
        let out = build.line(middle, feet[first]);
        let back = build.line(feet[second], middle);
        let lap = vec![
            build.leaving(out, middle),
            build.leaving(arc, feet[first]),
            build.leaving(back, feet[second]),
        ];
        build.face(plane, flipped, vec![lap]);
    }
    let mut body = build.finish((centre.abs() + DVec3::splat(radius)).max_element());
    for pole in &poles {
        body.vertices[pole.0 as usize].on = vec![sphere];
    }
    body.faces[piece.0 as usize].poles = poles;
    body
}

/// The three ways, of unit length, in the order that turns counterclockwise
/// seen from outside the cone they span.
fn counterclockwise(corners: [DVec3; 3]) -> [DVec3; 3] {
    let [first, second, third] = corners.map(DVec3::normalize);
    if first.dot(second.cross(third)) > 0.0 {
        [first, second, third]
    } else {
        [first, third, second]
    }
}

/// The solid angle the three ways span, seen from where they start.
fn spread(corners: [DVec3; 3]) -> f64 {
    let [a, b, c] = corners.map(DVec3::normalize);
    2.0 * a
        .dot(b.cross(c))
        .abs()
        .atan2(1.0 + a.dot(b) + b.dot(c) + c.dot(a))
}

/// Whether the way `direction` stands outside the cone the three ways span.
fn outside(corners: [DVec3; 3], direction: DVec3) -> bool {
    let ways = counterclockwise(corners);
    (0..3).any(|at| ways[(at + 1) % 3].cross(ways[at]).dot(direction) > 0.0)
}

/// The face of `body` on the sphere, once the fixture is held to what it is
/// built to be: listed sound, its sphere's poles outside the triangle the
/// ways `corners` bound, and every arc of the face neither a meridian nor a
/// latitude of the sphere, no line of its `(θ, φ)` chart.
fn piece_of(body: &Body, corners: [DVec3; 3], what: &str) -> FaceId {
    let eps = body.scale().eps();
    assert_eq!(
        listed(&body.listing(), body.scale().reach()),
        Ok(()),
        "{what}"
    );
    let [piece] = body
        .face_ids()
        .filter(|&face| matches!(body.surface(body.face(face).surface), Surface::Sphere(_)))
        .collect::<Vec<_>>()[..]
    else {
        panic!("{what}: one face on the sphere");
    };
    let Surface::Sphere(sphere) = body.surface(body.face(piece).surface) else {
        panic!("{what}: the piece lies on a sphere");
    };
    for side in [-1.0, 1.0] {
        assert!(
            outside(corners, sphere.axis * side),
            "{what}: a pole of the sphere stands inside the triangle"
        );
    }
    for coedge in body.face(piece).loops.iter().flatten() {
        let Curve::Circle(circle) = body.curve(body.edge(coedge.edge).curve) else {
            panic!("{what}: the piece is bounded by arcs");
        };
        assert!(
            !sphere.meridian(circle, eps) && !sphere.holds(circle, eps),
            "{what}: an arc of the piece, {circle:?}, is a line of its chart"
        );
    }
    piece
}

/// Holds that the sector of the ball of `radius` about `centre` over the
/// triangle the ways `corners` bound, its sphere read about each of `axes`,
/// is drawn closed at every tolerance, enclosing its volume, its piece of
/// sphere drawn within the sector.
fn sector_drawn(centre: DVec3, radius: f64, corners: [DVec3; 3], axes: &[DVec3]) {
    let ways = counterclockwise(corners);
    let whole = radius.powi(3) * spread(corners) / 3.0;
    for &axis in axes {
        let body = wedge(centre, radius, corners, axis.normalize(), Kept::Sector);
        let what = format!("the sector over {corners:?} read about {axis}");
        let eps = body.scale().eps();
        piece_of(&body, corners, &what);
        assert!(
            (body.volume() - whole).abs() <= 1e-9 * whole,
            "{what}: holds {}, where its sector holds {whole}",
            body.volume()
        );
        for tolerance in TOLERANCES {
            let cut = held(&body, tolerance);
            let drawn = on_rounds(&body, &cut);
            assert!(
                !drawn.is_empty(),
                "{what}: the piece is drawn at {tolerance}"
            );
            for point in drawn.iter().flatten() {
                for at in 0..3 {
                    let outward = ways[(at + 1) % 3].cross(ways[at]).normalize();
                    assert!(
                        (*point - centre).dot(outward) <= eps,
                        "{what}: the piece is drawn at {tolerance} through {point}, past the \
                         flat face facing {outward}"
                    );
                }
            }
        }
    }
}

/// The middle of the sectors, and the radius of their ball.
const SECTOR_CENTRE: DVec3 = DVec3::new(28.0, 2.0, 13.0);
const SECTOR_RADIUS: f64 = 2.0;

#[test]
fn a_sector_of_a_ball_bounded_by_three_great_circle_arcs_none_a_meridian_nor_the_equator_is_drawn_closed_and_encloses_its_volume()
 {
    let axes = [DVec3::new(1.0, 2.0, -3.0), DVec3::new(-2.0, 1.0, 1.0)];
    for corners in [
        [DVec3::X, DVec3::NEG_Y, DVec3::new(-0.1, -0.2, 1.0)],
        [
            DVec3::new(1.0, 0.1, 0.2),
            DVec3::new(0.2, 1.0, 0.1),
            DVec3::new(0.1, 0.2, 1.0),
        ],
    ] {
        sector_drawn(SECTOR_CENTRE, SECTOR_RADIUS, corners, &axes);
    }
}

/// The ways at `polar` degrees from `middle`, at each of `azimuths` degrees
/// round it.
fn round_about<const N: usize>(middle: DVec3, polar: f64, azimuths: [f64; N]) -> [DVec3; N] {
    let middle = middle.normalize();
    let across = middle.any_orthonormal_vector();
    let beside = middle.cross(across);
    let (sin, cos) = polar.to_radians().sin_cos();
    azimuths.map(|azimuth| {
        let (s, c) = azimuth.to_radians().sin_cos();
        middle * cos + (across * c + beside * s) * sin
    })
}

#[test]
fn a_sector_of_a_ball_over_a_wide_triangle_whose_corners_stand_eighty_degrees_from_its_middle_is_drawn_closed_and_encloses_its_volume()
 {
    let middle = DVec3::new(0.3, -0.4, 1.0);
    let corners = round_about(middle, 80.0, [0.0, 110.0, 230.0]);
    let axes = round_about(middle, 90.0, [40.0, 100.0]);
    sector_drawn(SECTOR_CENTRE, SECTOR_RADIUS, corners, &axes);
}

#[test]
fn the_sphere_face_of_a_ball_less_a_sector_in_front_of_no_one_direction_is_left_open_and_never_drawn_inside_out()
 {
    let corners = [
        DVec3::new(1.0, 0.1, 0.2),
        DVec3::new(0.2, 1.0, 0.1),
        DVec3::new(0.1, 0.2, 1.0),
    ];
    for axis in [DVec3::new(1.0, 2.0, -3.0), DVec3::new(-2.0, 1.0, 1.0)] {
        let body = wedge(
            SECTOR_CENTRE,
            SECTOR_RADIUS,
            corners,
            axis.normalize(),
            Kept::Rest,
        );
        let what = format!("the ball less the sector over {corners:?} read about {axis}");
        let piece = piece_of(&body, corners, &what);
        for tolerance in TOLERANCES {
            let cut = body.triangles_by_face(tolerance);
            assert_eq!(
                cut.uncut,
                vec![piece],
                "{what}: at {tolerance}, the sphere face alone is left open"
            );
            assert!(
                !cut.faces.contains(&piece),
                "{what}: at {tolerance}, triangles came back for the sphere face"
            );
        }
    }
}
