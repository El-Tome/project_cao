use std::collections::BTreeMap;
use std::f64::consts::{FRAC_PI_4, PI, TAU};

use glam::{DVec2, DVec3};

use super::super::{Samples, TOLD};
use super::Place;
use crate::brep::Scale;
use crate::brep::surface::{Cone, Surface};
use crate::brep::topology::{Body, EdgeId, FaceId};
use crate::profile::{Contour, Frame};
use crate::turning::{Axis, Corner, Straight, Turn};

/// How far the two planes cutting the band lean off square to its axis.
const SLOPE: f64 = 0.1;

/// A frustum about Y, of radius 10 at its foot and 5 at its top 20 above.
fn frustum() -> Body {
    let corners = [[0.0, 0.0], [20.0, 0.0], [20.0, 5.0], [0.0, 10.0]];
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
                    bend: None,
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
        angle: TAU,
        resolution: 0.0,
        on_the_axis: 0.0,
    };
    Body::turned(&straight, frame, &turn).expect("the frustum turns")
}

/// The frustum between the planes y = 8 + `SLOPE` x and `gap` above it: a
/// band of its cone between two ellipses, each a ring.
fn band(gap: f64) -> (Body, Cone) {
    let v = DVec3::new(1.0, SLOPE, 0.0).normalize();
    let normal = DVec3::Z.cross(v);
    let block = Contour::rectangle(DVec2::splat(-200.0), DVec2::splat(200.0));
    let at = |height: f64, from: f64| Frame {
        origin: DVec3::Y * height - normal * from,
        u: DVec3::Z,
        v,
    };
    let above = Body::raised(&block, &[], at(8.0 + gap, 0.0), normal * 50.0).expect("a block");
    let below = Body::raised(&block, &[], at(8.0, 50.0), normal * 50.0).expect("a block");
    let body = frustum()
        .cut_by(&above)
        .and_then(|body| body.cut_by(&below))
        .expect("the band is cut");
    let Surface::Cone(cone) = *body.surface(body.face(FaceId(0)).surface) else {
        panic!("the band's cone comes first");
    };
    (body, cone)
}

/// Where the ruling of `cone` at `theta` crosses the plane `edge` lies on.
fn on_ruling(body: &Body, cone: &Cone, edge: EdgeId, theta: f64) -> DVec3 {
    let level = |point: DVec3| point.y - SLOPE * point.x;
    let curve = body.curve(body.edge(edge).curve);
    let height = level(curve.point(0.0));
    let [foot, far] = [0.0, 1.0].map(|l| cone.point(DVec2::new(theta, l)));
    let l = (height - level(foot)) / (level(far) - level(foot));
    cone.point(DVec2::new(theta, l))
}

/// The band's two edges, the one nearer the apex first.
fn edges(body: &Body, cone: &Cone) -> [EdgeId; 2] {
    let [one, other] = [EdgeId(0), EdgeId(1)];
    let along = |edge| cone.parameters(on_ruling(body, cone, edge, 0.0)).y;
    if along(one) < along(other) {
        [one, other]
    } else {
        [other, one]
    }
}

/// The band's edges sampled by hand where each ruling at the angles given
/// crosses them, in the order of the angles; no vertex.
fn sampled(body: &Body, cone: &Cone, angles: [Vec<f64>; 2]) -> Samples {
    let mut points = Vec::new();
    let mut runs = vec![Vec::new(), Vec::new()];
    for (edge, mut angles) in edges(body, cone).into_iter().zip(angles) {
        angles.sort_by(f64::total_cmp);
        for theta in angles {
            runs[edge.0 as usize].push(points.len());
            points.push(on_ruling(body, cone, edge, theta));
        }
    }
    Samples {
        points,
        edges: runs,
        vertices: 0,
        grids: BTreeMap::new(),
        lines: Vec::new(),
        span: body.scale().reach(),
    }
}

/// Angles a fiftieth of a radian apart all the way round from `from`, but
/// those strictly within `room` of `from`.
fn round_but_near(from: f64, room: f64) -> Vec<f64> {
    (-157..157)
        .map(|step| step as f64 * 0.02)
        .filter(|off| off.abs() >= room - 1e-12)
        .map(|off| from + off)
        .collect()
}

fn points_of(samples: &Samples, edge: EdgeId) -> Vec<DVec3> {
    samples
        .edge(edge)
        .iter()
        .map(|id| samples.point(*id))
        .collect()
}

#[test]
fn a_sample_within_twice_the_tolerance_of_the_other_edge_on_its_ruling_is_answered_and_one_just_past_it_is_not()
 {
    let (body, cone) = band(5e-5);
    let [near, far] = edges(&body, &cone);
    let mut lonely = round_but_near(FRAC_PI_4, 0.3);
    lonely.push(FRAC_PI_4);
    let angles = [round_but_near(FRAC_PI_4, 0.3), lonely];
    let wanted = on_ruling(&body, &cone, near, FRAC_PI_4);
    let apart = on_ruling(&body, &cone, far, FRAC_PI_4).distance(wanted);
    for (share, answered) in [(1.0 + 1e-6, true), (1.0 - 1e-6, false)] {
        let mut samples = sampled(&body, &cone, angles.clone());
        let before = points_of(&samples, near);
        samples.on_common_rulings(&body, apart / 2.0 * share);
        let after = points_of(&samples, near);
        if answered {
            assert_eq!(
                after.len(),
                before.len() + 1,
                "the near edge took no place, or more than one"
            );
            assert!(
                after.iter().any(|point| point.distance(wanted) <= 1e-9),
                "no sample on the ruling the far edge's lonely sample stands on"
            );
        } else {
            assert!(
                after == before,
                "a sample {apart} away, past twice the tolerance, was answered"
            );
        }
        assert_eq!(
            points_of(&samples, far).len(),
            angles[1].len(),
            "the far edge took a place"
        );
    }
}

#[test]
fn the_edges_are_read_again_until_no_place_is_taken() {
    let (body, cone) = band(5e-5);
    let [near, _] = edges(&body, &cone);
    let step = 0.02;
    let between = step / 2.0_f64.sqrt();
    let mut inside = round_but_near(0.0, step);
    inside.extend([0.0, between]);
    let mut samples = sampled(&body, &cone, [round_but_near(0.0, step), inside]);
    samples.on_common_rulings(&body, 3.7e-4);
    let after = points_of(&samples, near);
    for theta in [between, 0.0] {
        let wanted = on_ruling(&body, &cone, near, theta);
        assert!(
            after.iter().any(|point| point.distance(wanted) <= 1e-9),
            "no sample of the near edge on the ruling at {theta}"
        );
    }
    let edges = samples.edges.clone();
    samples.on_common_rulings(&body, 3.7e-4);
    assert!(
        samples.edges == edges,
        "reading the edges again took a place"
    );
}

/// How close two places must be to be one, in the cases taken by hand.
const EPS: f64 = 1e-6;

/// A straight run of three samples along X, its first a vertex of the edge,
/// and one vertex off it, at (0, 1, 0), and one sample of another edge, at
/// (2.5, 1, 0).
fn by_hand() -> Samples {
    Samples {
        points: vec![
            DVec3::ZERO,
            DVec3::Y,
            DVec3::X,
            DVec3::X * 2.0,
            DVec3::new(2.5, 1.0, 0.0),
        ],
        edges: vec![vec![0, 2, 3], vec![4]],
        vertices: 2,
        grids: BTreeMap::new(),
        lines: Vec::new(),
        span: 3.0,
    }
}

/// The run of the edge by hand after it is given `places`, each on the
/// chord of the rank given and taken from the other edge's sample; and
/// whether any was taken.
fn given(places: &[(usize, DVec3)]) -> (Vec<DVec3>, Vec<usize>, bool) {
    let mut samples = by_hand();
    let taken: Vec<Place> = places
        .iter()
        .enumerate()
        .map(|(order, (rank, point))| (*rank, order as f64, *point, 4))
        .collect();
    let took = samples.took(BTreeMap::from([(EdgeId(0), taken)]), EPS);
    let run = samples.edge(EdgeId(0)).to_vec();
    (run.iter().map(|id| samples.point(*id)).collect(), run, took)
}

#[test]
fn a_place_within_a_hair_of_a_vertex_of_the_edge_is_that_vertex_and_one_further_off_is_taken() {
    let hair = EPS * Scale::HAIR;
    let (points, _, took) = given(&[(0, DVec3::X * hair * 0.9)]);
    assert!(
        !took,
        "a place within a hair of the vertex was taken: {points:?}"
    );
    let off = DVec3::X * hair * 1.1;
    let (points, _, took) = given(&[(0, off)]);
    assert!(took);
    assert_eq!(points, [DVec3::ZERO, off, DVec3::X, DVec3::X * 2.0]);
}

#[test]
fn a_place_within_the_kernel_s_tolerance_of_a_sample_of_the_edge_is_that_sample() {
    let (points, _, took) = given(&[(1, DVec3::X * (1.0 + EPS * 0.9))]);
    assert!(
        !took,
        "a place within the tolerance of a sample was taken: {points:?}"
    );
    let off = DVec3::X * (1.0 + EPS * 1.1);
    let (points, _, took) = given(&[(1, off)]);
    assert!(took);
    assert_eq!(points, [DVec3::ZERO, DVec3::X, off, DVec3::X * 2.0]);
}

#[test]
fn of_two_places_within_the_kernel_s_tolerance_of_each_other_only_the_first_is_taken() {
    let first = DVec3::new(1.5, 0.0, 0.0);
    let (points, _, _) = given(&[(1, first), (1, first + DVec3::Y * EPS * 0.9)]);
    assert_eq!(points, [DVec3::ZERO, DVec3::X, first, DVec3::X * 2.0]);
    let second = first + DVec3::Y * EPS * 1.1;
    let (points, _, _) = given(&[(1, first), (1, second)]);
    assert_eq!(
        points,
        [DVec3::ZERO, DVec3::X, first, second, DVec3::X * 2.0]
    );
}

#[test]
fn a_place_within_rounding_of_a_vertex_off_the_edge_is_that_vertex_and_one_further_off_is_its_own()
{
    let (_, run, _) = given(&[(1, DVec3::Y + DVec3::X * EPS * TOLD * 0.9)]);
    assert_eq!(
        run,
        [0, 2, 1, 3],
        "the place is not the vertex at (0, 1, 0)"
    );
    let off = DVec3::Y + DVec3::X * EPS * 1.1;
    let (points, run, _) = given(&[(1, off)]);
    assert_eq!(
        run[2], 5,
        "a place off the vertex at (0, 1, 0) is not a point of its own"
    );
    assert_eq!(points[2], off);
}

#[test]
fn a_place_within_rounding_of_the_sample_it_was_taken_from_is_that_sample_and_one_further_off_is_its_own()
 {
    let source = DVec3::new(2.5, 1.0, 0.0);
    let (_, run, _) = given(&[(2, source + DVec3::X * EPS * TOLD * 0.9)]);
    assert_eq!(
        run,
        [0, 2, 3, 4],
        "the place is not the sample it was taken from"
    );
    let off = source + DVec3::X * EPS * 1.1;
    let (points, run, _) = given(&[(2, off)]);
    assert_eq!(run, [0, 2, 3, 5]);
    assert_eq!(points[3], off);
}

/// The angle of `point` about the cone's axis, read from `from`, within half
/// a turn of it.
fn round_from(cone: &Cone, point: DVec3, from: f64) -> f64 {
    (cone.parameters(point).x - from + PI).rem_euclid(TAU) - PI
}

#[test]
fn a_ruling_through_a_place_of_the_edge_gives_no_place() {
    let (body, cone) = band(5e-5);
    let [near, _] = edges(&body, &cone);
    let samples = sampled(&body, &cone, [round_but_near(FRAC_PI_4, 0.3), Vec::new()]);
    let to = samples.charted(near, &cone);
    for read in &to.chart {
        let places = samples.beyond(&body, &to, &cone, *read);
        assert!(
            places.is_empty(),
            "the ruling through {read} gave {places:?}"
        );
    }
}

#[test]
fn a_read_beside_the_edge_and_not_beyond_its_chord_gives_no_place() {
    let (body, cone) = band(5e-5);
    let [near, _] = edges(&body, &cone);
    let samples = sampled(&body, &cone, [round_but_near(FRAC_PI_4, 0.3), Vec::new()]);
    let to = samples.charted(near, &cone);
    for (one, other) in to.chart.iter().zip(&to.chart[1..]) {
        for off in [-0.5, 0.5] {
            let read = (*one + *other) / 2.0 + DVec2::Y * off;
            let places = samples.beyond(&body, &to, &cone, read);
            assert!(places.is_empty(), "{read} gave {places:?}");
        }
    }
}

#[test]
fn a_read_standing_on_a_chord_across_where_the_curve_s_parameter_wraps_is_answered_on_its_ruling() {
    let (body, cone) = band(5e-5);
    let [near, _] = edges(&body, &cone);
    let samples = sampled(&body, &cone, [round_but_near(0.0, 0.3), Vec::new()]);
    let to = samples.charted(near, &cone);
    let ends: Vec<DVec2> = to
        .chart
        .iter()
        .copied()
        .filter(|at| (at.x.abs() - 0.3).abs() <= 1e-9)
        .collect();
    let [one, other] = ends[..] else {
        panic!("the chord's two ends: {ends:?}");
    };
    assert_eq!(one.y, other.y, "the chord does not stand level");
    let places = samples.beyond(&body, &to, &cone, DVec2::new(0.0, one.y));
    let [(_, along, point)] = places[..] else {
        panic!("{places:?}");
    };
    assert!(
        point.distance(on_ruling(&body, &cone, near, 0.0)) <= 1e-9,
        "{point} is not where the ruling crosses the edge"
    );
    assert!(
        (along - 0.5).abs() <= 0.05,
        "{along} along a chord the ruling halves"
    );
}

#[test]
fn a_read_beyond_the_chord_half_a_turn_round_gives_no_place() {
    let (body, cone) = band(5e-5);
    let [near, _] = edges(&body, &cone);
    let samples = sampled(&body, &cone, [round_but_near(PI, 0.3), Vec::new()]);
    let to = samples.charted(near, &cone);
    let [beside, opposite] =
        [PI - 0.3, PI].map(|theta| cone.parameters(on_ruling(&body, &cone, near, theta)).y);
    let read = DVec2::new(0.0, (beside + opposite) / 2.0);
    let places = samples.beyond(&body, &to, &cone, read);
    assert!(places.is_empty(), "{read} gave {places:?}");
}

#[test]
fn places_taken_on_one_chord_follow_the_edge_whichever_way_it_runs() {
    let (body, cone) = band(5e-5);
    let [near, far] = edges(&body, &cone);
    for center in [FRAC_PI_4, 0.0] {
        for backwards in [false, true] {
            let mut inside = round_but_near(center, 0.3);
            inside.extend([center - 0.1, center + 0.1]);
            let mut samples = sampled(&body, &cone, [round_but_near(center, 0.3), inside]);
            let turned = if backwards { near } else { far };
            samples.edges[turned.0 as usize].reverse();
            samples.on_common_rulings(&body, 0.2);
            let mut angles: Vec<f64> = points_of(&samples, near)
                .into_iter()
                .map(|point| round_from(&cone, point, center))
                .collect();
            if backwards {
                angles.reverse();
            }
            assert!(
                angles.windows(2).all(|pair| pair[0] < pair[1]),
                "about {center}, backwards {backwards}: {angles:?}"
            );
            for theta in [-0.1, 0.1] {
                assert!(
                    angles.iter().any(|angle| (angle - theta).abs() <= 1e-9),
                    "about {center}, backwards {backwards}: no sample at {theta}"
                );
            }
        }
    }
}

#[test]
fn a_sample_past_the_edge_s_box_by_less_than_three_tolerances_is_still_read() {
    let (body, cone) = band(5e-5);
    let [near, far] = edges(&body, &cone);
    let room = 0.8;
    let mut lonely = round_but_near(PI, room);
    lonely.push(PI);
    let mut samples = sampled(&body, &cone, [lonely, round_but_near(PI, room)]);
    let lowest = points_of(&samples, far)
        .into_iter()
        .map(|point| point.x)
        .fold(f64::INFINITY, f64::min);
    let past = lowest - on_ruling(&body, &cone, near, PI).x;
    assert!(
        past > 0.0,
        "the lonely sample stands within the far edge's box"
    );
    samples.on_common_rulings(&body, past / 2.0);
    let wanted = on_ruling(&body, &cone, far, PI);
    assert!(
        points_of(&samples, far)
            .iter()
            .any(|point| point.distance(wanted) <= 1e-9),
        "a sample {past} past the far edge's box, two tolerances, was not read"
    );
}
