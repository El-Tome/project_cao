//! Two walls leaning a hair off parallel, face to face, on one set of rays,
//! cut alike beside the line they cross along and met alike by a third wall:
//! the first four steps of the shared order #706 lays them on. The fifth,
//! which would build them by lifting the decline, is set aside; the cases
//! it would build wait under `#[ignore]` in
//! `crates/solid/tests/random_walls_at_any_angle.rs`, with the issue's Done
//! when.
//!
//! Part of #706.
//! - face to face: `Facing` and `Zones` take band pairs of cylinders within
//!   two sags —
//!   `two_walls_leaning_a_hair_off_parallel_are_told_crowded_only_where_they_stand_within_their_room`
//! - each wall reading its heights along its own axis —
//!   `two_walls_leaning_a_hair_off_parallel_each_withhold_their_steps_over_the_heights_of_their_own_rims`
//! - a point on the pair's own meet is never counted as crowded —
//!   `the_loop_two_walls_leaning_a_hair_off_parallel_cross_along_is_sampled_at_every_step_of_their_grids`
//! - one set of rays: a correspondence between the two walls' rays, each
//!   taking the other's grid and own rays —
//!   `two_walls_leaning_a_hair_off_parallel_are_sampled_on_one_set_of_rays_whichever_holds_a_vertex`,
//!   `each_of_two_walls_leaning_a_hair_off_parallel_samples_every_ring_at_its_own_vertex_however_near_the_other_s`;
//!   rays within the pair's drift merged by one rule decided once per pair —
//!   `two_walls_leaning_a_hair_off_parallel_take_no_two_rays_a_hair_apart`;
//!   three such walls on one set too —
//!   `three_walls_leaning_a_hair_off_one_another_are_sampled_on_one_set_of_rays`
//! - the crossing line sampled at its ends, the samples standing in the
//!   wedge just past an end's angle cleared, and each face's triangle on the
//!   line's chord flipped onto the sample of the curve both faces share
//!   beside that end, so that both hold the same chord triangle and cancel —
//!   `beside_the_line_two_walls_leaning_a_hair_off_parallel_cross_along_both_faces_are_cut_alike_and_cancel`,
//!   `beside_a_line_running_far_from_the_node_both_faces_are_still_cut_alike_and_cancel`,
//!   `beside_a_line_leaning_past_a_thousandth_of_a_radian_both_faces_are_still_cut_alike_and_cancel`
//! - a third wall crossing the pair has its two meets sampled on the pair's
//!   shared rays, so that near-duplicates merge —
//!   `a_third_wall_crossing_two_walls_leaning_a_hair_off_parallel_meets_both_on_the_same_rays`,
//!   `a_third_wall_crossing_two_walls_drifting_further_than_the_tolerance_meets_both_on_the_same_rays`

use std::collections::BTreeMap;
use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI, TAU};

use glam::{DQuat, DVec3};

use super::*;
use crate::brep::curve::{Meet, Wall as Round};
use crate::brep::surface::Surface;
use crate::brep::tessellation::contact::walls;
use crate::brep::tessellation::sampling::{Samples, divisions};
use crate::brep::tessellation::tests::fixtures;
use crate::brep::topology::{Coedge, EdgeId, VertexId};

const RADIUS: f64 = 5.0;
const LEAN: f64 = 1e-5;
const FOOT: f64 = 100.0;
const HALF: f64 = 5.0;
const REACH: f64 = 200.0;

/// A post about Z and a wall of its radius leaning `LEAN` off it towards X,
/// their axes crossing at `FOOT` along X, each a band from `-HALF` to `HALF`
/// high at its axis, and the loop the two cross along there. So far from the
/// origin, a height along one axis stands a thousandth off the same height
/// along the other.
fn leaning_bands() -> (Body, [Wall; 2], [[EdgeId; 2]; 2], EdgeId) {
    let mut build = fixtures::Build::new();
    let crossing = DVec3::X * FOOT;
    let leaning = DVec3::new(LEAN, 0.0, (1.0 - LEAN * LEAN).sqrt());
    let cylinders = [DVec3::Z, leaning].map(|axis| Cylinder::about(crossing, axis, RADIUS));
    let ids = cylinders.map(|wall| build.surface(Surface::Cylinder(wall)));
    let use_of = |edge, forward| Coedge { edge, forward };
    let mut rings = [[EdgeId(0); 2]; 2];
    for side in 0..2 {
        let wall = cylinders[side];
        rings[side] = [-HALF, HALF].map(|z| {
            let on_axis = crossing + wall.axis * (z / wall.axis.z);
            build.circle(ids[side], wall.parameters(on_axis).y, None)
        });
        let [low, high] = rings[side];
        build.face(
            ids[side],
            false,
            vec![vec![use_of(low, true)], vec![use_of(high, false)]],
        );
    }
    let mid_height = Meet {
        first: Round::Cylinder(cylinders[0]),
        second: Round::Cylinder(cylinders[1]),
        component: 1,
    };
    let loop_edge = build.meet_edge(mid_height, None, 0.0, TAU);
    let body = build.finish(REACH);
    let found = walls(&body);
    (body, [found[0], found[1]], rings, loop_edge)
}

fn level_of(body: &Body, ring: EdgeId) -> f64 {
    let Curve::Circle(circle) = body.curve(body.edge(ring).curve) else {
        panic!("a band is bounded by rings");
    };
    circle.center.dot(circle.axis)
}

fn apart_from_the_line(wall: &Cylinder, other: &Cylinder, z: f64, apart: f64) -> DVec3 {
    let on_the_line = wall.parameters(DVec3::new(FOOT, RADIUS, z)).x;
    let at = |angle: f64| {
        let height = wall.parameters(DVec3::new(FOOT, 0.0, z)).y;
        wall.point(DVec2::new(angle, height))
    };
    let (mut near, mut far) = (on_the_line, on_the_line + 0.1);
    assert!(other.distance(at(far)).abs() > apart);
    for _ in 0..200 {
        let middle = (near + far) / 2.0;
        if other.distance(at(middle)).abs() < apart {
            near = middle;
        } else {
            far = middle;
        }
    }
    at(near)
}

#[test]
fn two_walls_leaning_a_hair_off_parallel_are_told_crowded_only_where_they_stand_within_their_room()
{
    let (body, walls, _, _) = leaning_bands();
    let zones = Zones::of(&body, &walls);
    let room = body.scale().eps() * APART;
    let [(_, post), (_, leaning)] = walls;

    for (wall, other) in [(post, leaning), (leaning, post)] {
        let near = apart_from_the_line(&wall, &other, HALF / 2.0, room / 2.0);
        let away = wall.parameters(near).x + FRAC_PI_2;
        let far = wall.point(DVec2::new(away, wall.parameters(near).y));

        assert!(
            zones.crowded(&wall, near),
            "a place of the wall about {} standing {} from the other, within the room \
             of {room}, is not told crowded",
            wall.axis,
            other.distance(near),
        );
        assert!(
            !zones.crowded(&wall, far),
            "a place of the wall about {} standing {} from the other, far past the room \
             of {room}, is told crowded",
            wall.axis,
            other.distance(far),
        );
    }
}

#[test]
fn two_walls_leaning_a_hair_off_parallel_each_withhold_their_steps_over_the_heights_of_their_own_rims()
 {
    let (body, walls, rings, _) = leaning_bands();
    let eps = body.scale().eps();

    for order in [[0, 1], [1, 0]] {
        let facing = Facing::of(&body, &walls[order[0]], &walls[order[1]]);
        for side in 0..2 {
            let ((id, wall), (_, other)) = (walls[side], walls[1 - side]);
            let middle = wall.parameters(DVec3::new(FOOT, 0.0, HALF / 2.0)).y;
            let point = [RADIUS, -RADIUS]
                .map(|x| wall.parameters(DVec3::new(FOOT + x, 0.0, HALF / 2.0)).x)
                .map(|angle| wall.point(DVec2::new(angle, middle)))
                .into_iter()
                .find(|point| other.distance(*point) > 0.0)
                .expect("past the loop, the other stands inside the wall on one side");
            let withheld = facing.withheld(id, point);
            let rims = facing.rims(id);

            assert!(
                !withheld.is_empty()
                    && withheld
                        .iter()
                        .all(|[low, high]| *low > -2.0 * HALF && *high < 2.0 * HALF),
                "the wall about {}, the other on its hollow side, withholds its steps \
                 over {withheld:?}, not over the heights the two face each other",
                wall.axis,
            );
            let levels = rings[side].map(|ring| level_of(&body, ring));
            for level in levels {
                assert!(
                    withheld
                        .iter()
                        .any(|[low, high]| *low - eps <= level && level <= *high + eps),
                    "the wall about {} withholds its steps over {withheld:?}, which miss \
                     its own rim at {level}, walls taken {order:?}",
                    wall.axis,
                );
            }
            for level in levels {
                assert!(
                    rims.iter().any(
                        |[low, high]| (low - level).abs() <= eps && (high - level).abs() <= eps
                    ),
                    "the rims of the wall about {} stand at {rims:?}, not at its own {level}, \
                     walls taken {order:?}",
                    wall.axis,
                );
            }
        }
    }
}

#[test]
fn the_loop_two_walls_leaning_a_hair_off_parallel_cross_along_is_sampled_at_every_step_of_their_grids()
 {
    let (body, walls, _, loop_edge) = leaning_bands();

    for tolerance in [1e-3, 0.02, 0.5] {
        let samples = Samples::of(&body, tolerance);
        let points: Vec<DVec3> = samples
            .edge(loop_edge)
            .iter()
            .map(|id| samples.point(*id))
            .collect();
        for (_, wall) in walls {
            let steps = divisions(wall.radius, tolerance);
            for step in 0..steps {
                let angle = TAU * step as f64 / steps as f64;
                assert!(
                    points.iter().any(|point| {
                        let off = (wall.parameters(*point).x - angle + PI).rem_euclid(TAU) - PI;
                        off.abs() < 1e-9
                    }),
                    "the loop the two walls cross along misses step {step} of the wall about \
                     {} within {tolerance}: {} samples",
                    wall.axis,
                    points.len(),
                );
            }
        }
    }
}

fn leaning_pair_built(
    lean: f64,
    toward: f64,
    radii: [f64; 2],
    vertices: [Option<f64>; 2],
) -> (fixtures::Build, [Wall; 2], [[EdgeId; 2]; 2]) {
    let mut build = fixtures::Build::new();
    let crossing = DVec3::X * FOOT;
    let (sin, cos) = toward.sin_cos();
    let leaning = DVec3::new(lean * cos, lean * sin, (1.0 - lean * lean).sqrt());
    let use_of = |edge, forward| Coedge { edge, forward };
    let mut rings = [[EdgeId(0); 2]; 2];
    let mut walls = [(SurfaceId(0), Cylinder::about(crossing, DVec3::Z, radii[0])); 2];
    for side in 0..2 {
        let wall = Cylinder::about(crossing, [DVec3::Z, leaning][side], radii[side]);
        let id = build.surface(Surface::Cylinder(wall));
        walls[side] = (id, wall);
        let [low, high] =
            [-HALF, HALF].map(|z| wall.parameters(crossing + wall.axis * (z / wall.axis.z)).y);
        let through = vertices[side].map(|angle| build.vertex(wall.point(DVec2::new(angle, low))));
        rings[side] = [build.circle(id, low, through), build.circle(id, high, None)];
        build.face(
            id,
            false,
            vec![
                vec![use_of(rings[side][0], true)],
                vec![use_of(rings[side][1], false)],
            ],
        );
    }
    (build, walls, rings)
}

fn leaning_rings(
    lean: f64,
    toward: f64,
    radii: [f64; 2],
    vertices: [Option<f64>; 2],
) -> (Body, [Wall; 2], [[EdgeId; 2]; 2]) {
    let (build, _, rings) = leaning_pair_built(lean, toward, radii, vertices);
    let body = build.finish(REACH);
    let found = walls(&body);
    (body, [found[0], found[1]], rings)
}

fn ways_seen_from(samples: &Samples, body: &Body, ring: EdgeId, frame: &Cylinder) -> Vec<f64> {
    let Curve::Circle(circle) = body.curve(body.edge(ring).curve) else {
        panic!("a band is bounded by rings");
    };
    let turn = DQuat::from_rotation_arc(circle.axis, frame.axis);
    let mut ids = samples.edge(ring).to_vec();
    if ids.len() > 1 && ids[0] == ids[ids.len() - 1] {
        ids.pop();
    }
    ids.iter()
        .map(|id| {
            let way = turn * (samples.point(*id) - circle.center);
            way.dot(frame.v).atan2(way.dot(frame.u))
        })
        .collect()
}

fn round_off(one: f64, other: f64) -> f64 {
    ((one - other + PI).rem_euclid(TAU) - PI).abs()
}

fn first_radius_whose_grid_grows(tolerance: f64, below: f64) -> Option<f64> {
    (0..250_000)
        .map(|step| RADIUS + step as f64 * 1e-4)
        .find(|radius| divisions(*radius, tolerance) > divisions(radius - below, tolerance))
}

#[test]
fn two_walls_leaning_a_hair_off_parallel_are_sampled_on_one_set_of_rays_whichever_holds_a_vertex() {
    let below = 5e-5;
    for tolerance in [1e-3, 0.02, 0.5] {
        let mut cases = vec![[RADIUS, RADIUS]];
        cases.extend(
            first_radius_whose_grid_grows(tolerance, below).map(|radius| [radius, radius - below]),
        );
        for radii in cases {
            let (body, walls, rings) = leaning_rings(LEAN, 0.0, radii, [Some(0.3), Some(1.0)]);
            let samples = Samples::of(&body, tolerance);
            let frame = walls[0].1;
            let seen =
                rings.map(|pair| pair.map(|ring| ways_seen_from(&samples, &body, ring, &frame)));
            for one in seen.iter().flatten() {
                for other in seen.iter().flatten() {
                    for way in one {
                        let nearest = other
                            .iter()
                            .map(|seen| round_off(*way, *seen))
                            .fold(f64::INFINITY, f64::min);
                        assert!(
                            nearest < 1e-9,
                            "a ray at {way}, turned onto the post's axis, stands {nearest} \
                             round from the nearest of {} on another ring, radii {radii:?} \
                             within {tolerance}",
                            other.len(),
                        );
                    }
                    assert_eq!(one.len(), other.len(), "radii {radii:?} within {tolerance}");
                }
            }
        }
    }
}

#[test]
fn two_walls_leaning_a_hair_off_parallel_take_no_two_rays_a_hair_apart() {
    let below = 5e-5;
    let mut cases = vec![
        (9e-4, 1.0, [RADIUS; 2], 0.02),
        (9e-4, 1.0, [RADIUS; 2], 0.5),
    ];
    for tolerance in [1e-3, 0.02] {
        let radius = first_radius_whose_grid_grows(tolerance, below).expect("a grid grows past 5");
        cases.push((LEAN, 0.0, [radius, radius - below], tolerance));
    }
    for (lean, toward, radii, tolerance) in cases {
        let (body, walls, rings) = leaning_rings(lean, toward, radii, [None; 2]);
        let samples = Samples::of(&body, tolerance);
        for (side, pair) in rings.into_iter().enumerate() {
            let wall = walls[side].1;
            for ring in pair {
                let mut ways = ways_seen_from(&samples, &body, ring, &wall);
                ways.sort_by(f64::total_cmp);
                for (at, way) in ways.iter().enumerate() {
                    let next = ways[(at + 1) % ways.len()];
                    assert!(
                        round_off(*way, next) * wall.radius > 1e-3,
                        "a ring of the wall about {} takes rays at {way} and {next}, a hair \
                         apart, radii {radii:?} leaning {lean} within {tolerance}",
                        wall.axis,
                    );
                }
            }
        }
    }
}

struct Sliver {
    body: Body,
    lines: [EdgeId; 2],
    ends: [[DVec3; 2]; 2],
}

fn stretch(
    period: f64,
    parameter: impl Fn(DVec3) -> f64,
    point: impl Fn(f64) -> DVec3,
    [from, to]: [DVec3; 2],
    through: DVec3,
) -> (bool, f64, f64) {
    let [start, end] = [from, to].map(&parameter);
    let ahead = start + (end - start).rem_euclid(period);
    let back = end + (start - end).rem_euclid(period);
    let off = |low: f64, high: f64| point((low + high) / 2.0).distance(through);
    if off(start, ahead) <= off(end, back) {
        (true, start, ahead)
    } else {
        (false, end, back)
    }
}

fn shaft_bored_above_its_node(radius: f64, lean: f64, height: f64) -> Sliver {
    let mut build = fixtures::Build::new();
    let away = DVec3::new(1.0, 1.0, 0.0).normalize();
    let shaft = Cylinder::about(DVec3::ZERO, DVec3::Z, radius);
    let bore = Cylinder::about(
        DVec3::ZERO,
        (-away * lean.sin() + DVec3::Z * lean.cos()).normalize(),
        radius,
    );
    let ids = [shaft, bore].map(|wall| build.surface(Surface::Cylinder(wall)));
    let side = shaft.axis.cross(bore.axis).normalize() * radius;
    let nodes = [side, -side];
    let rise =
        (height * (1.0 - bore.axis.z) / (radius * bore.axis.cross(DVec3::Z).length())).asin();
    let tops = [-FRAC_PI_4 - rise, 3.0 * FRAC_PI_4 + rise]
        .map(|angle| shaft.point(DVec2::new(angle, height)));
    let [n1, n2, t1, t2] = [nodes[0], nodes[1], tops[0], tops[1]].map(|point| build.vertex(point));
    let meet_between =
        |build: &mut fixtures::Build, ends: [DVec3; 2], ids: [VertexId; 2], through: DVec3| {
            let meet = (0..2)
                .map(|component| Meet {
                    first: Round::Cylinder(shaft),
                    second: Round::Cylinder(bore),
                    component,
                })
                .filter(|meet| {
                    meet.period().is_some()
                        && ends
                            .iter()
                            .all(|end| meet.point(meet.parameter(*end)).distance(*end) < 1e-9)
                })
                .min_by(|one, other| {
                    let off = |meet: &Meet| meet.point(meet.parameter(through)).distance(through);
                    off(one).total_cmp(&off(other))
                })
                .expect("both ends and the way through lie on one component");
            let (forward, from, to) = stretch(
                meet.period().unwrap(),
                |p| meet.parameter(p),
                |t| meet.point(t),
                ends,
                through,
            );
            let order = if forward { ids } else { [ids[1], ids[0]] };
            build.meet_edge(meet, Some(order), from, to)
        };
    let lines = [
        meet_between(
            &mut build,
            [nodes[0], tops[0]],
            [n1, t1],
            (nodes[0] + tops[0]) / 2.0,
        ),
        meet_between(
            &mut build,
            [nodes[1], tops[1]],
            [n2, t2],
            (nodes[1] + tops[1]) / 2.0,
        ),
    ];
    let half_loop = meet_between(&mut build, nodes, [n1, n2], away * radius);
    let rim = build.arc(ids[0], height, t1, t2);
    let (cap, cap_flipped) = build.plane(DVec3::Z * height, DVec3::Z);
    let conic = build.conic(cap, ids[1], 2.0 * radius)[0];
    let (forward, from, to) = stretch(
        conic.period().unwrap(),
        |p| conic.parameter(p),
        |t| conic.point(t),
        tops,
        away * radius + DVec3::Z * height,
    );
    let mouth = build.conic_edge(
        conic,
        Some(if forward { [t1, t2] } else { [t2, t1] }),
        from,
        to,
    );
    let lap = |build: &fixtures::Build, steps: [(EdgeId, VertexId); 4]| {
        steps.map(|(edge, from)| build.leaving(edge, from)).to_vec()
    };
    let outer = lap(
        &build,
        [(half_loop, n1), (lines[1], n2), (rim, t2), (lines[0], t1)],
    );
    build.face(ids[0], false, vec![outer]);
    let inner = lap(
        &build,
        [(lines[0], n1), (mouth, t1), (lines[1], t2), (half_loop, n2)],
    );
    build.face(ids[1], true, vec![inner]);
    let top = vec![build.leaving(rim, t1), build.leaving(mouth, t2)];
    build.face(cap, cap_flipped, vec![top]);
    Sliver {
        body: build.finish(2.0 * radius + height),
        lines,
        ends: [[nodes[0], tops[0]], [nodes[1], tops[1]]],
    }
}

#[test]
fn beside_the_line_two_walls_leaning_a_hair_off_parallel_cross_along_both_faces_are_cut_alike_and_cancel()
 {
    cut_alike_and_cancelled(&[(5.0, 1e-4, 1.125), (17.5, 0.01, 6.5)]);
}

#[test]
fn beside_a_line_running_far_from_the_node_both_faces_are_still_cut_alike_and_cancel() {
    cut_alike_and_cancelled(&[(5.0, 0.03, 12.5), (17.5, 0.03, 43.75)]);
}

fn cut_alike_and_cancelled(shapes: &[(f64, f64, f64)]) {
    let key = |point: DVec3| point.to_array().map(f64::to_bits);
    for &(radius, degrees, height) in shapes {
        let sliver = shaft_bored_above_its_node(radius, degrees.to_radians(), height);
        let body = &sliver.body;
        for share in [1e-3, 2e-4] {
            let tolerance = share * body.scale().reach();
            let samples = Samples::of(body, tolerance);
            let cut = body.triangles_by_face(tolerance);

            assert!(
                cut.uncut.is_empty(),
                "{:?} left uncut within {tolerance}",
                cut.uncut
            );
            for (line, ends) in sliver.lines.into_iter().zip(sliver.ends) {
                let taken: Vec<DVec3> = samples
                    .edge(line)
                    .iter()
                    .map(|id| samples.point(*id))
                    .collect();
                let mut keys: Vec<_> = taken.iter().map(|point| key(*point)).collect();
                keys.sort_unstable();
                let mut wanted = ends.map(key).to_vec();
                wanted.sort_unstable();
                assert_eq!(
                    keys, wanted,
                    "the line from {} to {} is sampled at {taken:?} within {tolerance}",
                    ends[0], ends[1],
                );
                let on_the_chord: Vec<_> = cut
                    .triangles
                    .iter()
                    .zip(&cut.faces)
                    .filter(|(corners, _)| {
                        ends.iter()
                            .all(|end| corners.iter().any(|corner| key(*corner) == key(*end)))
                    })
                    .collect();
                assert!(
                    on_the_chord.is_empty(),
                    "beside the line from {} to {}, radius {radius} leaning {degrees}° within \
                     {tolerance}, the two faces are cut apart and drawn: {on_the_chord:?}",
                    ends[0],
                    ends[1],
                );
            }
            let mut sides: BTreeMap<_, i32> = BTreeMap::new();
            for corners in &cut.triangles {
                for at in 0..3 {
                    *sides
                        .entry((key(corners[at]), key(corners[(at + 1) % 3])))
                        .or_default() += 1;
                }
            }
            let open = sides
                .iter()
                .filter(|((from, to), uses)| sides.get(&(*to, *from)) != Some(*uses))
                .count();
            assert_eq!(
                open, 0,
                "{open} sides met one way only, radius {radius} within {tolerance}"
            );
        }
    }
}

#[test]
fn each_of_two_walls_leaning_a_hair_off_parallel_samples_every_ring_at_its_own_vertex_however_near_the_other_s()
 {
    let post = Cylinder::about(DVec3::X * FOOT, DVec3::Z, RADIUS);
    let leaning = DVec3::new(LEAN, 0.0, (1.0 - LEAN * LEAN).sqrt());
    let leaning = Cylinder::about(DVec3::X * FOOT, leaning, RADIUS);
    let way = DQuat::from_rotation_arc(post.axis, leaning.axis) * post.radial(0.3);
    let facing = way.dot(leaning.v).atan2(way.dot(leaning.u));
    for hair in [1e-7, 1e-6, 5e-6] {
        let vertices = [0.3, facing + hair];
        let (body, walls, rings) = leaning_rings(LEAN, 0.0, [RADIUS; 2], vertices.map(Some));
        for tolerance in [1e-3, 0.02] {
            let samples = Samples::of(&body, tolerance);
            for (side, pair) in rings.into_iter().enumerate() {
                let wall = walls[side].1;
                for ring in pair {
                    let mut ways = ways_seen_from(&samples, &body, ring, &wall);
                    ways.sort_by(f64::total_cmp);
                    assert!(
                        ways.iter()
                            .any(|way| round_off(*way, vertices[side]) < 1e-9),
                        "a ring of the wall about {} misses its own vertex at {}, the other's \
                         {hair} round from it, within {tolerance}",
                        wall.axis,
                        vertices[side],
                    );
                    for (at, way) in ways.iter().enumerate() {
                        let next = ways[(at + 1) % ways.len()];
                        assert!(
                            round_off(*way, next) * wall.radius > 1e-3,
                            "a ring of the wall about {} takes rays at {way} and {next}, a hair \
                             apart, the vertices {hair} apart, within {tolerance}",
                            wall.axis,
                        );
                    }
                }
            }
        }
    }
}

fn a_leaning_pair_crossed_by_a_third_wall(
    lean: f64,
    third: f64,
) -> (Body, [Wall; 2], [EdgeId; 2], [[EdgeId; 2]; 2]) {
    let (mut build, walls, rings) = leaning_pair_built(lean, 0.0, [RADIUS; 2], [None; 2]);
    let across = DVec3::new(1.0, 1.0, 0.0).normalize();
    let center = DVec3::new(FOOT, 0.0, HALF / 2.0);
    let bore = Cylinder::about(center, across, third);
    let pierced = center + across * RADIUS;
    let meets = walls.map(|(_, wall)| {
        let meet = (0..4)
            .map(|component| Meet {
                first: Round::Cylinder(wall),
                second: Round::Cylinder(bore),
                component,
            })
            .filter(|meet| meet.period().is_some())
            .min_by(|one, other| {
                let off = |meet: &Meet| meet.point(meet.parameter(pierced)).distance(pierced);
                off(one).total_cmp(&off(other))
            })
            .expect("the third wall pierces both walls");
        let period = meet.period().expect("a closed loop");
        build.meet_edge(meet, None, 0.0, period)
    });
    let body = build.finish(REACH);
    (body, walls, meets, rings)
}

fn seen_round_the_first(
    samples: &Samples,
    edge: EdgeId,
    wall: &Cylinder,
    frame: &Cylinder,
) -> Vec<f64> {
    let turn = DQuat::from_rotation_arc(wall.axis, frame.axis);
    samples
        .edge(edge)
        .iter()
        .map(|id| {
            let from = samples.point(*id) - wall.origin;
            let way = turn * (from - wall.axis * wall.axis.dot(from));
            way.dot(frame.v).atan2(way.dot(frame.u))
        })
        .collect()
}

#[test]
fn a_third_wall_crossing_two_walls_leaning_a_hair_off_parallel_meets_both_on_the_same_rays() {
    met_alike(LEAN, &[1e-3, 0.02, 0.1]);
}

#[test]
fn a_third_wall_crossing_two_walls_drifting_further_than_the_tolerance_meets_both_on_the_same_rays()
{
    met_alike(5e-4, &[1e-3, 2e-3]);
}

#[test]
fn beside_a_line_leaning_past_a_thousandth_of_a_radian_both_faces_are_still_cut_alike_and_cancel() {
    cut_alike_and_cancelled(&[(5.0, 0.05, 25.0), (17.5, 0.05, 87.5)]);
}

fn three_walls_leaning_a_hair_off_one_another(
    vertices: [f64; 3],
) -> (Body, Vec<Wall>, [[EdgeId; 2]; 3]) {
    let mut build = fixtures::Build::new();
    let crossing = DVec3::X * FOOT;
    let leaning = |toward: f64| {
        let (sin, cos) = toward.sin_cos();
        DVec3::new(LEAN * cos, LEAN * sin, (1.0 - LEAN * LEAN).sqrt())
    };
    let axes = [DVec3::Z, leaning(0.0), leaning(FRAC_PI_2)];
    let use_of = |edge, forward| Coedge { edge, forward };
    let mut rings = [[EdgeId(0); 2]; 3];
    for side in 0..3 {
        let wall = Cylinder::about(crossing, axes[side], RADIUS);
        let id = build.surface(Surface::Cylinder(wall));
        let [low, high] =
            [-HALF, HALF].map(|z| wall.parameters(crossing + wall.axis * (z / wall.axis.z)).y);
        let through = build.vertex(wall.point(DVec2::new(vertices[side], low)));
        rings[side] = [
            build.circle(id, low, Some(through)),
            build.circle(id, high, None),
        ];
        build.face(
            id,
            false,
            vec![
                vec![use_of(rings[side][0], true)],
                vec![use_of(rings[side][1], false)],
            ],
        );
    }
    let body = build.finish(REACH);
    let found = walls(&body);
    (body, found, rings)
}

#[test]
fn three_walls_leaning_a_hair_off_one_another_are_sampled_on_one_set_of_rays() {
    let (body, walls, rings) = three_walls_leaning_a_hair_off_one_another([0.3, 1.0, 2.0]);
    for tolerance in [1e-3, 0.02, 0.5] {
        let samples = Samples::of(&body, tolerance);
        let frame = walls[0].1;
        let seen: Vec<Vec<f64>> = rings
            .iter()
            .flatten()
            .map(|ring| ways_seen_from(&samples, &body, *ring, &frame))
            .collect();
        for (one, ring) in seen.iter().zip(rings.iter().flatten()) {
            for (other, other_ring) in seen.iter().zip(rings.iter().flatten()) {
                for way in one {
                    let nearest = other
                        .iter()
                        .map(|seen| round_off(*way, *seen))
                        .fold(f64::INFINITY, f64::min);
                    assert!(
                        nearest < 1e-9,
                        "ring {ring:?} takes a ray at {way}, turned onto the first wall's axis, \
                         {nearest} round from the nearest of the {} of ring {other_ring:?}, within \
                         {tolerance}",
                        other.len(),
                    );
                }
            }
        }
    }
}

fn met_alike(lean: f64, tolerances: &[f64]) {
    for &tolerance in tolerances {
        let (body, walls, meets, rings) = a_leaning_pair_crossed_by_a_third_wall(lean, 1.0);
        let samples = Samples::of(&body, tolerance);
        let frame = walls[0].1;
        let seen =
            [0, 1].map(|side| seen_round_the_first(&samples, meets[side], &walls[side].1, &frame));

        for (one, other) in [(0, 1), (1, 0)] {
            for way in &seen[one] {
                let nearest = seen[other]
                    .iter()
                    .map(|seen| round_off(*way, *seen))
                    .fold(f64::INFINITY, f64::min);
                assert!(
                    nearest < 1e-9,
                    "the third wall meets the wall about {} at {way} round the first's axis, \
                     {nearest} from the nearest of the {} places it meets the other at, \
                     within {tolerance}",
                    walls[one].1.axis,
                    seen[other].len(),
                );
            }
        }
        assert_eq!(seen[0].len(), seen[1].len(), "within {tolerance}");
        for (side, meet) in meets.into_iter().enumerate() {
            let points: Vec<DVec3> = samples
                .edge(meet)
                .iter()
                .map(|id| samples.point(*id))
                .collect();
            for (at, point) in points.iter().enumerate() {
                let next = points[(at + 1) % points.len()];
                assert!(
                    point.distance(next) > 1e-4,
                    "the third wall meets the wall about {} at {point} and {next}, a hair \
                     apart, within {tolerance}",
                    walls[side].1.axis,
                );
            }
        }
        for (side, pair) in rings.into_iter().enumerate() {
            let wall = walls[side].1;
            for ring in pair {
                let mut ways = ways_seen_from(&samples, &body, ring, &wall);
                ways.sort_by(f64::total_cmp);
                for (at, way) in ways.iter().enumerate() {
                    let next = ways[(at + 1) % ways.len()];
                    assert!(
                        round_off(*way, next) * wall.radius > 1e-3,
                        "a ring of the wall about {} takes rays at {way} and {next}, a hair \
                         apart, within {tolerance}",
                        wall.axis,
                    );
                }
            }
        }
    }
}
