use std::f64::consts::TAU;

use glam::DVec3;

use super::super::super::tests::{fixtures, slanted};
use super::super::conic::on_conic;
use super::super::{Samples, divisions};
use crate::brep::conic::{Conic, On};
use crate::brep::curve::Curve;
use crate::brep::surface::Surface;
use crate::brep::topology::{Body, EdgeId, SurfaceId};

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
