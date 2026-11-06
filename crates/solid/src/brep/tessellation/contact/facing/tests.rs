//! Two walls leaning a hair off parallel, face to face and on one set of
//! rays: the first steps of the shared order #706 now needs, as its last
//! comment lays it out. Each of the
//! first four steps lands with `closer_than_a_sag` still in place and every
//! campaign identical to `main` seed by seed; the fifth lifts the decline, and
//! is held in `crates/solid/tests/random_walls_at_any_angle.rs` with the
//! issue's Done when.
//!
//! Closes #706.
//! - face to face: `Facing` and `Zones` take band pairs of cylinders within
//!   two sags —
//!   `two_walls_leaning_a_hair_off_parallel_are_told_crowded_only_where_they_stand_within_their_room`
//! - each wall reading its heights along its own axis —
//!   `two_walls_leaning_a_hair_off_parallel_each_withhold_their_steps_over_the_heights_of_their_own_rims`
//! - a point on the pair's own meet is never counted as crowded —
//!   `the_loop_two_walls_leaning_a_hair_off_parallel_cross_along_is_sampled_at_every_step_of_their_grids`
//! - one set of rays: a correspondence between the two walls' rays, each
//!   taking the other's grid and own rays —
//!   `two_walls_leaning_a_hair_off_parallel_are_sampled_on_one_set_of_rays_whichever_holds_a_vertex`;
//!   rays within the pair's drift merged by one rule decided once per pair —
//!   `two_walls_leaning_a_hair_off_parallel_take_no_two_rays_a_hair_apart`
//! - the crossing line sampled at its ends, each wall pinning in its own
//!   chart the angles of both ends and nothing sampled in the wedge between
//!   them, so that both faces cut the cell alike — no test: put off to the
//!   third step, whose red is written in its own turn
//! - a third wall crossing the pair has its two meets sampled on the pair's
//!   shared rays, so that near-duplicates merge — no test: put off to the
//!   fourth step, whose red is written in its own turn

use std::f64::consts::{FRAC_PI_2, PI, TAU};

use glam::{DQuat, DVec3};

use super::*;
use crate::brep::curve::{Meet, Wall as Round};
use crate::brep::surface::Surface;
use crate::brep::tessellation::contact::walls;
use crate::brep::tessellation::sampling::{Samples, divisions};
use crate::brep::tessellation::tests::fixtures;
use crate::brep::topology::{Coedge, EdgeId};

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

fn leaning_rings(
    lean: f64,
    toward: f64,
    radii: [f64; 2],
    vertices: [Option<f64>; 2],
) -> (Body, [Wall; 2], [[EdgeId; 2]; 2]) {
    let mut build = fixtures::Build::new();
    let crossing = DVec3::X * FOOT;
    let (sin, cos) = toward.sin_cos();
    let leaning = DVec3::new(lean * cos, lean * sin, (1.0 - lean * lean).sqrt());
    let use_of = |edge, forward| Coedge { edge, forward };
    let mut rings = [[EdgeId(0); 2]; 2];
    for side in 0..2 {
        let wall = Cylinder::about(crossing, [DVec3::Z, leaning][side], radii[side]);
        let id = build.surface(Surface::Cylinder(wall));
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
