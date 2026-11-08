//! A wall a meet carries a few ulps off the body's own — the copy
//! `Meeting::of` leaves where it snaps a wall onto a touch it stands on but
//! for rounding — is still that wall to every filter of the tessellation
//! (#754). Each test holds a body whose meets carry the copy to what the
//! same body gets carrying the wall itself.
//!
//! Closes #754.
//! - `grazed_near`, and on #706's branch the five sites named above, tell a
//!   wall from its snapped copy by that one rule — `grazed_near` by
//!   `a_meet_ending_at_a_vertex_on_a_wall_it_carries_a_few_ulps_off_keeps_its_samples_near_that_vertex`;
//!   `crossing::is_line` by
//!   `the_line_two_walls_leaning_a_hair_off_parallel_cross_along_is_still_a_line_when_its_meet_carries_one_a_few_ulps_off`;
//!   `Zones::crowded_off_the_meet` by
//!   `the_loop_two_walls_leaning_a_hair_off_parallel_cross_along_is_not_crowded_when_its_meet_carries_one_a_few_ulps_off`;
//!   `twin_of` by
//!   `a_third_wall_carried_a_few_ulps_off_at_one_of_its_meets_still_meets_both_walls_of_the_pair_on_the_same_rays`;
//!   `twin::shared` by
//!   `a_wall_of_the_pair_carried_a_few_ulps_off_at_its_meet_with_a_third_still_meets_it_on_the_pair_s_rays`;
//!   `Zones::sharing`, no test of its own: it compares no wall, and hands
//!   the pair's walls as the body holds them to `is_line` and `shared`,
//!   whose tests hold what it hands; and where the fix reaches beyond the
//!   five, the rays a meet gives the wall it holds by
//!   `the_rings_of_a_wall_its_meets_carry_a_few_ulps_off_are_still_sampled_through_every_sample_of_the_curve`,
//!   and the rays a meet takes from that wall's circles by
//!   `a_meet_carrying_a_wall_a_few_ulps_off_still_takes_the_rays_that_wall_s_circles_take`
//!
//! The issue's first and third criteria are at the head of
//! `crates/solid/tests/random_walls_at_any_angle.rs`.

use super::*;
use crate::brep::tessellation::tests::across;
use crate::brep::topology::FaceId;

fn a_few_ulps_off(wall: Cylinder) -> Cylinder {
    let off = 4.0 * f64::EPSILON * wall.origin.length().max(wall.radius);
    let copy = Cylinder {
        origin: wall.origin + wall.u * off,
        ..wall
    };
    assert_ne!(copy, wall, "the copy stands a few ulps off");
    copy
}

/// `body`, the meets along `edges` carrying `wall` as `copy` instead.
fn carrying(body: &Body, edges: &[EdgeId], wall: Round, copy: Round) -> Body {
    let mut moved = body.clone();
    let mut carried = 0;
    for edge in edges {
        let Curve::Meet(meet) = &mut moved.curves[body.edge(*edge).curve.0 as usize] else {
            panic!("only a meet carries a wall");
        };
        for held in [&mut meet.first, &mut meet.second] {
            if *held == wall {
                *held = copy;
                carried += 1;
            }
        }
    }
    assert_eq!(carried, edges.len(), "every meet carries the wall");
    moved
}

fn points(samples: &Samples, edge: EdgeId) -> Vec<DVec3> {
    samples
        .edge(edge)
        .iter()
        .map(|id| samples.point(*id))
        .collect()
}

#[test]
fn a_meet_ending_at_a_vertex_on_a_wall_it_carries_a_few_ulps_off_keeps_its_samples_near_that_vertex()
 {
    let body = across::stock_bored_flush_across();
    let meets: Vec<EdgeId> = body
        .edge_ids()
        .filter(|edge| matches!(body.curve(body.edge(*edge).curve), Curve::Meet(_)))
        .collect();
    let Some(Round::Cylinder(bore)) = Round::of(body.surface(SurfaceId(1))) else {
        panic!("the bore is the body's second surface");
    };
    let moved = carrying(
        &body,
        &meets,
        Round::Cylinder(bore),
        Round::Cylinder(a_few_ulps_off(bore)),
    );

    for tolerance in [1e-3, 0.02, 0.5] {
        let (wanted, got) = (
            Samples::of(&body, tolerance),
            Samples::of(&moved, tolerance),
        );
        for edge in &meets {
            let (wanted, got) = (points(&wanted, *edge), points(&got, *edge));
            assert!(
                wanted.len() > 2,
                "a half window is sampled between its ends"
            );
            assert!(
                got.len() == wanted.len()
                    && got
                        .iter()
                        .zip(&wanted)
                        .all(|(got, wanted)| got.distance(*wanted) < 1e-9),
                "the half window {edge:?}, carrying the bore a few ulps off, is sampled at \
                 {} places, {} carrying it as the body holds it, within {tolerance}",
                got.len(),
                wanted.len(),
            );
        }
    }
}

#[test]
fn the_line_two_walls_leaning_a_hair_off_parallel_cross_along_is_still_a_line_when_its_meet_carries_one_a_few_ulps_off()
 {
    for (radius, degrees, height) in [(5.0, 1e-4_f64, 1.125), (17.5, 0.01, 6.5), (5.0, 0.03, 12.5)]
    {
        let mut sliver = shaft_bored_above_its_node(radius, degrees.to_radians(), height);
        let Some(Round::Cylinder(bore)) = Round::of(sliver.body.surface(SurfaceId(1))) else {
            panic!("the bore is the body's second surface");
        };
        sliver.body = carrying(
            &sliver.body,
            &sliver.lines,
            Round::Cylinder(bore),
            Round::Cylinder(a_few_ulps_off(bore)),
        );
        sliver_cut_alike_and_cancelled(&sliver, radius, degrees);
    }
}

/// Two bands of `RADIUS` leaning `LEAN` off each other as in
/// `leaning_bands`, the loop they cross along bounding both faces, its meet
/// carrying the second wall as `carried` makes it.
fn bands_bounded_by_their_loop(carried: fn(Cylinder) -> Cylinder) -> (Body, [Cylinder; 2], EdgeId) {
    let mut build = fixtures::Build::new();
    let crossing = DVec3::X * FOOT;
    let leaning = DVec3::new(LEAN, 0.0, (1.0 - LEAN * LEAN).sqrt());
    let cylinders = [DVec3::Z, leaning].map(|axis| Cylinder::about(crossing, axis, RADIUS));
    let ids = cylinders.map(|wall| build.surface(Surface::Cylinder(wall)));
    let loop_edge = build.meet_edge(
        Meet {
            first: Round::Cylinder(cylinders[0]),
            second: Round::Cylinder(carried(cylinders[1])),
            component: 1,
        },
        None,
        0.0,
        TAU,
    );
    let use_of = |edge, forward| Coedge { edge, forward };
    for side in 0..2 {
        let wall = cylinders[side];
        let [low, high] = [-HALF, HALF].map(|z| {
            let on_axis = crossing + wall.axis * (z / wall.axis.z);
            build.circle(ids[side], wall.parameters(on_axis).y, None)
        });
        build.face(
            ids[side],
            false,
            vec![
                vec![use_of(low, true)],
                vec![use_of(high, false)],
                vec![use_of(loop_edge, side == 0)],
            ],
        );
    }
    (build.finish(REACH), cylinders, loop_edge)
}

#[test]
fn the_loop_two_walls_leaning_a_hair_off_parallel_cross_along_is_not_crowded_when_its_meet_carries_one_a_few_ulps_off()
 {
    for (carried, moved) in [
        (
            "as the body holds it",
            (|wall| wall) as fn(Cylinder) -> Cylinder,
        ),
        ("a few ulps off", a_few_ulps_off),
    ] {
        let (body, walls, loop_edge) = bands_bounded_by_their_loop(moved);
        for tolerance in [1e-3, 0.02, 0.5] {
            let samples = Samples::of(&body, tolerance);
            let points = points(&samples, loop_edge);
            for wall in walls {
                let steps = divisions(wall.radius, tolerance);
                let missed = (0..steps)
                    .filter(|step| {
                        let angle = TAU * *step as f64 / steps as f64;
                        !points.iter().any(|point| {
                            let off = (wall.parameters(*point).x - angle + PI).rem_euclid(TAU) - PI;
                            off.abs() < 1e-9
                        })
                    })
                    .count();
                assert_eq!(
                    missed,
                    0,
                    "the loop, its meet carrying the second wall {carried}, misses {missed} of \
                     the {steps} steps of the wall about {} within {tolerance}: {} samples",
                    wall.axis,
                    points.len(),
                );
            }
        }
    }
}

/// What a meet on the wall of a side is carried as.
type Carried<'a> = &'a dyn Fn(usize, Meet) -> Meet;

/// `a_leaning_pair_crossed_by_a_third_wall` of radius 1, its two meets
/// bounding the pair's faces and a face of the third wall between them, the
/// meet on the wall `side` as `carried` makes it.
fn a_leaning_pair_crossed_by_a_third_wall_bounding_faces(
    carried: Carried,
) -> (Body, [Cylinder; 2], [EdgeId; 2]) {
    let (mut build, walls, _) = leaning_pair_built(LEAN, 0.0, [RADIUS; 2], [None; 2]);
    let across = DVec3::new(1.0, 1.0, 0.0).normalize();
    let center = DVec3::new(FOOT, 0.0, HALF / 2.0);
    let bore = Cylinder::about(center, across, 1.0);
    let third = build.surface(Surface::Cylinder(bore));
    let pierced = center + across * RADIUS;
    let use_of = |edge, forward| Coedge { edge, forward };
    let mut meets = [EdgeId(0); 2];
    for (side, (_, wall)) in walls.into_iter().enumerate() {
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
        meets[side] = build.meet_edge(carried(side, meet), None, 0.0, period);
        build.add_loop(FaceId(side as u32), vec![use_of(meets[side], true)]);
    }
    build.face(
        third,
        true,
        vec![vec![use_of(meets[0], false)], vec![use_of(meets[1], true)]],
    );
    (build.finish(REACH), walls.map(|(_, wall)| wall), meets)
}

fn met_on_the_same_rays(carried: Carried, what: &str) {
    let as_held = |_: usize, meet: Meet| meet;
    let carryings: [(&str, Carried); 2] = [("as the body holds them", &as_held), (what, carried)];
    for (label, carried) in carryings {
        let (body, walls, meets) = a_leaning_pair_crossed_by_a_third_wall_bounding_faces(carried);
        for tolerance in [1e-3, 0.02, 0.1] {
            let samples = Samples::of(&body, tolerance);
            let seen = [0, 1]
                .map(|side| seen_round_the_first(&samples, meets[side], &walls[side], &walls[0]));
            for (one, other) in [(0, 1), (1, 0)] {
                let apart = seen[one]
                    .iter()
                    .filter(|way| {
                        seen[other]
                            .iter()
                            .all(|seen| round_off(**way, *seen) >= 1e-9)
                    })
                    .count();
                assert_eq!(
                    apart,
                    0,
                    "the meets carrying {label}: {apart} of the {} places the third wall meets \
                     the wall about {} at stand on no ray it meets the other on, within \
                     {tolerance}",
                    seen[one].len(),
                    walls[one].axis,
                );
            }
        }
    }
}

#[test]
fn a_third_wall_carried_a_few_ulps_off_at_one_of_its_meets_still_meets_both_walls_of_the_pair_on_the_same_rays()
 {
    met_on_the_same_rays(
        &|side, meet| match (side, meet.second) {
            (1, Round::Cylinder(third)) => Meet {
                second: Round::Cylinder(a_few_ulps_off(third)),
                ..meet
            },
            _ => meet,
        },
        "the third wall a few ulps off at the second wall's meet",
    );
}

#[test]
fn a_wall_of_the_pair_carried_a_few_ulps_off_at_its_meet_with_a_third_still_meets_it_on_the_pair_s_rays()
 {
    met_on_the_same_rays(
        &|side, meet| match (side, meet.first) {
            (1, Round::Cylinder(wall)) => Meet {
                first: Round::Cylinder(a_few_ulps_off(wall)),
                ..meet
            },
            _ => meet,
        },
        "the second wall a few ulps off at its meet with the third",
    );
}

/// The stock bored across along X twice, by radius 2 above its middle and
/// 1.5 below it, each bore a whole window either side: the windows of either
/// stand round the stock's axis where those of the other do.
fn stock_bored_across_twice() -> Body {
    let mut build = fixtures::Build::new();
    let use_of = |edge, forward| Coedge { edge, forward };
    let stock = build.cylinder(DVec3::ZERO, DVec3::Z, fixtures::STOCK_RADIUS);
    let mut windows = Vec::new();
    for (height, radius) in [(7.0, 2.0), (3.0, 1.5)] {
        let bore = build.cylinder(DVec3::Z * height, DVec3::X, radius);
        let meeting = build.meeting(stock, bore, fixtures::STOCK_RADIUS);
        let [near, far] = [0, 1].map(|component| {
            let meet = meeting.components[component];
            let period = meet.period().expect("a window closes on itself");
            build.meet_edge(meet, None, 0.0, period)
        });
        build.face(
            bore,
            true,
            vec![vec![use_of(near, false)], vec![use_of(far, true)]],
        );
        windows.extend([vec![use_of(near, true)], vec![use_of(far, false)]]);
    }
    let rings = [0.0, fixtures::HEIGHT].map(|height| build.circle(stock, height, None));
    let mut loops = vec![vec![use_of(rings[0], true)], vec![use_of(rings[1], false)]];
    loops.extend(windows);
    build.face(stock, false, loops);
    let (top, top_flipped) = build.plane(DVec3::Z * fixtures::HEIGHT, DVec3::Z);
    build.face(top, top_flipped, vec![vec![use_of(rings[1], true)]]);
    let (bottom, bottom_flipped) = build.plane(DVec3::ZERO, DVec3::NEG_Z);
    build.face(bottom, bottom_flipped, vec![vec![use_of(rings[0], false)]]);
    build.finish(fixtures::STOCK_RADIUS)
}

/// `body` and the same body whose meets carry its stock a few ulps off.
fn with_the_stock_a_few_ulps_off(body: Body) -> [Body; 2] {
    let meets: Vec<EdgeId> = body
        .edge_ids()
        .filter(|edge| matches!(body.curve(body.edge(*edge).curve), Curve::Meet(_)))
        .collect();
    let Some(Round::Cylinder(stock)) = Round::of(body.surface(SurfaceId(0))) else {
        panic!("the stock is the body's first surface");
    };
    let moved = carrying(
        &body,
        &meets,
        Round::Cylinder(stock),
        Round::Cylinder(a_few_ulps_off(stock)),
    );
    [body, moved]
}

/// Whether every edge of `body` along a curve `chosen` picks is sampled at
/// the same places in `moved`, within each tolerance.
fn sampled_alike(bodies: &[Body; 2], chosen: fn(&Curve) -> bool, what: &str) {
    let [body, moved] = bodies;
    let edges: Vec<EdgeId> = body
        .edge_ids()
        .filter(|edge| chosen(body.curve(body.edge(*edge).curve)))
        .collect();
    assert!(!edges.is_empty(), "the body has {what}");
    for tolerance in [1e-3, 0.02, 0.5] {
        let (wanted, got) = (Samples::of(body, tolerance), Samples::of(moved, tolerance));
        for edge in &edges {
            let (wanted, got) = (points(&wanted, *edge), points(&got, *edge));
            assert!(wanted.len() > 2, "{what} are sampled round");
            assert!(
                got.len() == wanted.len()
                    && got
                        .iter()
                        .zip(&wanted)
                        .all(|(got, wanted)| got.distance(*wanted) < 1e-9),
                "{edge:?}, of {what}, is sampled at {} places with the stock a few ulps off in \
                 the meets, {} with the stock as the body holds it, within {tolerance}",
                got.len(),
                wanted.len(),
            );
        }
    }
}

#[test]
fn the_rings_of_a_wall_its_meets_carry_a_few_ulps_off_are_still_sampled_through_every_sample_of_the_curve()
 {
    sampled_alike(
        &with_the_stock_a_few_ulps_off(across::stock_bored_across(3.0, DVec3::X)),
        |curve| matches!(curve, Curve::Circle(_)),
        "the stock's rings",
    );
}

#[test]
fn a_meet_carrying_a_wall_a_few_ulps_off_still_takes_the_rays_that_wall_s_circles_take() {
    sampled_alike(
        &with_the_stock_a_few_ulps_off(stock_bored_across_twice()),
        |curve| matches!(curve, Curve::Meet(_)),
        "the windows",
    );
}
