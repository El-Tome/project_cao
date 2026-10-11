use std::f64::consts::TAU;

use glam::{DVec2, DVec3};

use super::super::super::tests::{fixtures, slanted};
use super::super::conic::on_conic;
use super::super::{Samples, divisions};
use crate::brep::conic::{Conic, On, Round};
use crate::brep::curve::Curve;
use crate::brep::surface::{Cylinder, Surface};
use crate::brep::topology::{Body, EdgeId, SurfaceId, VertexId};
use crate::profile::{Contour, Frame, Run};

/// How close to a grid angle a sample must stand for the outline to cut a
/// face going round there.
const CLEAR: f64 = 1e-9;

fn conics(body: &Body) -> Vec<(EdgeId, Conic)> {
    body.edge_ids()
        .filter_map(|id| match body.curve(body.edge(id).curve) {
            Curve::Conic(conic) => Some((id, *conic)),
            Curve::Line(_) | Curve::Circle(_) | Curve::Meet(_) => None,
        })
        .collect()
}

fn apart(angle: f64, to: f64) -> f64 {
    ((angle - to + TAU / 2.0).rem_euclid(TAU) - TAU / 2.0).abs()
}

#[test]
fn an_ellipse_is_sampled_at_every_grid_angle_of_its_wall_and_its_plane_face_takes_the_same_samples()
{
    for lean in [0.1, -0.45] {
        let body = fixtures::stock_cut_at_a_slant(lean);
        let [(edge, conic)] = conics(&body)[..] else {
            panic!("one ellipse");
        };
        let Surface::Cylinder(cylinder) = *body.surface(SurfaceId(0)) else {
            panic!("the stock's wall comes first");
        };
        let Surface::Plane(plane) = *body.surface(SurfaceId(1)) else {
            panic!("the slanted top comes second");
        };
        for tolerance in [1e-4, 0.02, 0.5] {
            let samples = Samples::of(&body, tolerance);
            let points: Vec<DVec3> = samples
                .edge(edge)
                .iter()
                .map(|id| samples.point(*id))
                .collect();
            let steps = divisions(cylinder.radius, tolerance);
            for step in 0..steps {
                let angle = TAU * step as f64 / steps as f64;
                assert!(
                    points
                        .iter()
                        .any(|point| apart(cylinder.parameters(*point).x, angle) <= CLEAR),
                    "{lean} within {tolerance}: no sample at {angle}"
                );
            }
            for point in &points {
                assert!(cylinder.distance(*point).abs() < 1e-12 * cylinder.radius);
                assert!(plane.distance(*point).abs() < 1e-12 * cylinder.radius);
                let t = conic.parameter(*point);
                assert!(conic.point(t).distance(*point) < 1e-12 * cylinder.radius);
            }
            let cut = body.triangles_by_face(tolerance);
            for face in body.face_ids() {
                if body.face(face).surface == SurfaceId(2) {
                    continue;
                }
                let corners: Vec<DVec3> = cut
                    .triangles
                    .iter()
                    .zip(&cut.faces)
                    .filter(|(_, on)| **on == face)
                    .flat_map(|(corners, _)| *corners)
                    .collect();
                for point in &points {
                    assert!(
                        corners.contains(point),
                        "{lean} within {tolerance}: {point} is no corner of {face:?}"
                    );
                }
            }
        }
    }
}

/// How far `point` stands from the segment between `start` and `end`.
fn off_the_chord(point: DVec3, start: DVec3, end: DVec3) -> f64 {
    let along = end - start;
    let share = ((point - start).dot(along) / along.length_squared()).clamp(0.0, 1.0);
    (point - (start + along * share)).length()
}

/// Every chord between two samples of `edge` along `conic`, its parameters
/// read off them, held within `tolerance` of the curve.
fn chords_held(conic: &Conic, ends: [f64; 2], parameters: &[f64], tolerance: f64) {
    let mut places = vec![ends[0]];
    places.extend(parameters);
    places.push(ends[1]);
    assert!(
        places.windows(2).all(|pair| pair[0] < pair[1]),
        "the samples run along the curve: {places:?}"
    );
    for pair in places.windows(2) {
        let (start, end) = (conic.point(pair[0]), conic.point(pair[1]));
        for part in 1..32 {
            let at = pair[0] + (pair[1] - pair[0]) * part as f64 / 32.0;
            let off = off_the_chord(conic.point(at), start, end);
            assert!(
                off <= tolerance * (1.0 + 1e-9),
                "within {tolerance}: {off} off between {} and {}",
                pair[0],
                pair[1]
            );
        }
    }
}

#[test]
fn a_steep_ellipse_is_sampled_so_no_chord_leaves_it_by_the_tolerance() {
    for lean in [0.45, 3.0, 40.0, -1e3] {
        let body = fixtures::stock_cut_at_a_slant(lean);
        let [(edge, conic)] = conics(&body)[..] else {
            panic!("one ellipse");
        };
        let period = conic.period().expect("an ellipse closes");
        for tolerance in [1e-6, 1e-3, 0.02, 0.5] {
            let samples = Samples::of(&body, tolerance);
            let mut parameters: Vec<f64> = samples
                .edge(edge)
                .iter()
                .map(|id| conic.parameter(samples.point(*id)))
                .collect();
            let first = parameters[0];
            for t in &mut parameters {
                *t = first + (*t - first).rem_euclid(period);
            }
            let rest = parameters.split_off(1);
            chords_held(&conic, [first, first + period], &rest, tolerance);
        }
    }
}

#[test]
fn a_conic_between_two_vertices_runs_from_the_first_to_the_last_with_every_sample_between() {
    let bodies = [
        slanted::chamfer_with_a_flat(6.0),
        slanted::stock_with_a_sliver_cut_at_a_slant(1e-3),
        slanted::stock_with_a_sliver_cut_at_a_slant(1e-7),
    ];
    for body in bodies {
        let [(edge, conic)] = conics(&body)[..] else {
            panic!("one conic");
        };
        let ends = body.edge(edge).ends.expect("a conic between two vertices");
        for tolerance in [1e-6, 1e-3, 0.02, 0.5] {
            let samples = Samples::of(&body, tolerance);
            let run = samples.edge(edge);
            assert_eq!(run[0], ends[0].0 as usize);
            assert_eq!(run[run.len() - 1], ends[1].0 as usize);
            let inner: Vec<f64> = run[1..run.len() - 1]
                .iter()
                .map(|id| conic.parameter(samples.point(*id)))
                .collect();
            let span = [body.edge(edge).from, body.edge(edge).to];
            chords_held(&conic, span, &inner, tolerance);
        }
    }
}

#[test]
fn a_conic_is_sampled_at_the_angle_of_every_ray_the_circles_of_its_wall_take() {
    let body = fixtures::stock_cut_at_a_slant(0.3);
    let [(edge, conic)] = conics(&body)[..] else {
        panic!("one ellipse");
    };
    let eps = body.scale().eps();
    let off_the_grid = [0.123, 1.234, -2.345];
    for tolerance in [1e-3, 0.02, 0.5] {
        let points = on_conic(
            &conic,
            body.edge(edge),
            tolerance,
            eps,
            [&[], &off_the_grid],
            &|_| true,
        );
        for angle in off_the_grid {
            assert!(
                points.iter().any(|point| {
                    let t = conic.parameter(*point);
                    apart(conic.seen_on(On::Wall, t)[0].x, angle) <= CLEAR
                }),
                "{angle} is missed within {tolerance}"
            );
        }
    }
}

#[test]
fn a_conic_on_a_cone_widens_the_cone_s_grid() {
    for lean in [0.3, -0.9, 1.5] {
        let body = slanted::frustum_cut_at_a_slant(lean);
        let [(edge, conic)] = conics(&body)[..] else {
            panic!("one ellipse");
        };
        let span = body.edge(edge);
        let widest = conic.widest(span.from, span.to);
        let rim = slanted::BASE * (1.0 - slanted::SQUARE / slanted::TIP);
        assert!(widest > 2.0 * rim);
        for tolerance in [1e-4, 0.02] {
            let samples = Samples::of(&body, tolerance);
            assert_eq!(
                samples.steps(SurfaceId(0)),
                Some(divisions(widest, tolerance))
            );
            assert!(divisions(widest, tolerance) > divisions(rim, tolerance));
        }
    }
}

/// A shaft of radius `SHAFT` about Z, its top at z = 0, cut below a plane
/// through the line y = `across` of its top leaning `lean` off square to Z:
/// what is left is a wedge whose two rims on the wall, the top's circle and
/// the plane's conic, end together on that line.
fn wedge_of_a_shaft(across: f64, lean: f64) -> Body {
    let shaft = Contour {
        corners: vec![DVec2::X * SHAFT],
        runs: vec![Run::Round {
            center: DVec2::ZERO,
            turn: TAU,
        }],
    };
    let ground = Frame {
        origin: DVec3::NEG_Z * 30.0,
        u: DVec3::X,
        v: DVec3::Y,
    };
    let body = Body::raised(&shaft, &[], ground, DVec3::Z * 30.0).expect("a shaft raises");
    let v = DVec3::new(0.0, lean.cos(), lean.sin());
    let normal = DVec3::X.cross(v);
    let leaning = Frame {
        origin: DVec3::Y * across - normal * 50.0,
        u: DVec3::X,
        v,
    };
    let block = Contour::rectangle(DVec2::splat(-200.0), DVec2::splat(200.0));
    let tool = Body::raised(&block, &[], leaning, normal * 50.0).expect("a block raises");
    body.cut_by(&tool).expect("the wedge is cut")
}

const SHAFT: f64 = 60.0;

/// The circle and the conic of `wall` ending together at `vertex`, if two
/// such edges do.
fn rims_ending_at(body: &Body, vertex: VertexId) -> Option<(EdgeId, EdgeId, Cylinder)> {
    let ending = body.edge_ids().filter(|id| {
        body.edge(*id)
            .ends
            .is_some_and(|ends| ends.contains(&vertex))
    });
    let (mut circle, mut conic) = (None, None);
    for id in ending {
        match body.curve(body.edge(id).curve) {
            Curve::Circle(_) => circle = Some(id),
            Curve::Conic(Conic {
                round: Round::Cylinder(wall),
                ..
            }) => conic = Some((id, *wall)),
            _ => {}
        }
    }
    let (circle, (conic, wall)) = (circle?, conic?);
    Some((circle, conic, wall))
}

#[test]
fn a_circle_and_a_conic_ending_together_on_one_wall_take_the_same_grid_angles_beside_their_end() {
    let tolerance = 0.15;
    let step = TAU / divisions(SHAFT, tolerance) as f64;
    let grid = (TAU / 3.0 / step).round() * step;
    let body = wedge_of_a_shaft(SHAFT * (grid - 0.2_f64.to_radians()).sin(), 1.6e-7);
    let samples = Samples::of(&body, tolerance);
    let mut ends = 0;
    for vertex in body.vertex_ids() {
        let Some((circle, conic, wall)) = rims_ending_at(&body, vertex) else {
            continue;
        };
        ends += 1;
        let at = wall.parameters(body.vertex(vertex).point).x;
        let beside = |edge: EdgeId| -> Vec<i64> {
            samples
                .edge(edge)
                .iter()
                .map(|id| wall.parameters(samples.point(*id)).x)
                .filter(|angle| apart(*angle, at) <= step)
                .filter(|angle| apart(*angle, (angle / step).round() * step) <= CLEAR)
                .map(|angle| (angle / step).round() as i64)
                .collect()
        };
        assert_eq!(
            beside(circle),
            beside(conic),
            "the rims ending at {:?} take other grid angles beside it",
            body.vertex(vertex).point
        );
    }
    assert_eq!(ends, 2, "the circle and the conic end together twice");
}
