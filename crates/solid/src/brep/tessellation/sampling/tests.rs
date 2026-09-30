use std::f64::consts::{PI, TAU};

use glam::DVec3;

use super::super::contact::APART;
use super::super::tests::fixtures;
use super::{Samples, divisions};
use crate::brep::curve::{Curve, Meet};
use crate::brep::surface::Surface;
use crate::brep::topology::{Body, EdgeId};

const CLOSE: f64 = 1e-12;

#[test]
fn a_turn_is_cut_into_a_multiple_of_four_steps_between_sixteen_and_a_thousand_and_twenty_four() {
    for radius in [0.1, 1.0, 5.0, 20.0, 1e4] {
        for tolerance in [1e-9, 1e-6, 1e-3, 0.02, 1.0, 100.0, 0.0, -1.0, f64::NAN] {
            let steps = divisions(radius, tolerance);
            assert!(
                steps.is_multiple_of(4),
                "{steps} for {radius} within {tolerance}"
            );
            assert!((16..=1024).contains(&steps), "{steps} for {radius}");
        }
    }
}

#[test]
fn a_chord_of_one_step_stands_within_the_tolerance_with_no_step_to_spare() {
    let sag = |radius: f64, steps: usize| radius * (1.0 - (PI / steps as f64).cos());
    for radius in [0.5, 5.0, 20.0, 300.0] {
        for tolerance in [1e-4, 1e-3, 0.02, 0.3] {
            let steps = divisions(radius, tolerance);
            if steps < 1024 {
                assert!(sag(radius, steps) <= tolerance * (1.0 + CLOSE));
            }
            if steps > 16 {
                assert!(sag(radius, steps - 4) > tolerance);
            }
        }
    }
}

#[test]
fn a_straight_edge_is_sampled_at_its_two_vertices_and_nothing_else() {
    let body = fixtures::block();
    let samples = Samples::of(&body, 0.02);
    for edge in body.edge_ids() {
        let ends = body.edge(edge).ends.expect("a line has ends");
        assert_eq!(
            samples.edge(edge),
            &[ends[0].0 as usize, ends[1].0 as usize]
        );
        for end in ends {
            assert_eq!(samples.point(end.0 as usize), body.vertex(end).point);
        }
    }
}

#[test]
fn a_circle_through_a_vertex_is_sampled_on_the_grid_from_the_vertex_round_to_it() {
    let body = fixtures::block_with_a_lying_cylinder();
    let tolerance = 0.02;
    let samples = Samples::of(&body, tolerance);
    let mut seen = 0;
    for edge in body.edge_ids() {
        let Curve::Circle(circle) = *body.curve(body.edge(edge).curve) else {
            continue;
        };
        seen += 1;
        let steps = divisions(circle.radius, tolerance);
        let ids = samples.edge(edge);
        let vertex = body.edge(edge).ends.expect("through a vertex")[0].0 as usize;
        assert_eq!((ids[0], ids[ids.len() - 1]), (vertex, vertex));
        assert_eq!(ids.len(), steps + 1);
        for id in &ids[1..ids.len() - 1] {
            let angle = circle.parameter(samples.point(*id)).rem_euclid(TAU);
            let step = angle * steps as f64 / TAU;
            assert!(
                (step - step.round()).abs() < 1e-9,
                "{angle} is off the grid"
            );
            assert!((circle.point(angle) - samples.point(*id)).length() < CLOSE);
        }
    }
    assert_eq!(seen, 2);
}

#[test]
fn a_ring_is_sampled_at_every_angle_of_the_grid_from_its_start() {
    let body = fixtures::stock();
    let tolerance = 0.02;
    let samples = Samples::of(&body, tolerance);
    for edge in body.edge_ids() {
        let Curve::Circle(circle) = *body.curve(body.edge(edge).curve) else {
            panic!("the stock's edges are its two rings");
        };
        let steps = divisions(circle.radius, tolerance);
        let ids = samples.edge(edge);
        assert_eq!(ids.len(), steps);
        for (step, id) in ids.iter().enumerate() {
            let expected = circle.point(TAU * step as f64 / steps as f64);
            assert!((samples.point(*id) - expected).length() < CLOSE);
        }
    }
}

/// The angles round `(center, 0)`, from X, of the samples of every circle of
/// `radius` at the top of the block, sorted.
fn rays_round(body: &Body, samples: &Samples, center: f64, radius: f64) -> Vec<f64> {
    let mut angles = Vec::new();
    for edge in body.edge_ids() {
        let Curve::Circle(circle) = *body.curve(body.edge(edge).curve) else {
            continue;
        };
        if circle.radius != radius || circle.center.z != fixtures::HEIGHT {
            continue;
        }
        let ids = samples.edge(edge);
        for id in &ids[..ids.len() - 1] {
            let point = samples.point(*id);
            angles.push(point.y.atan2(point.x - center).rem_euclid(TAU));
        }
    }
    angles.sort_by(f64::total_cmp);
    angles
}

#[test]
fn the_circles_of_a_hole_tangent_inside_the_stock_are_sampled_on_common_rays_from_its_axis() {
    let body = fixtures::stock_with_a_tangent_hole();
    let center = fixtures::STOCK_RADIUS - fixtures::HOLE_RADIUS;
    for tolerance in [1e-6, 0.02, 10.0] {
        let samples = Samples::of(&body, tolerance);
        let outer = rays_round(&body, &samples, center, fixtures::STOCK_RADIUS);
        let inner = rays_round(&body, &samples, center, fixtures::HOLE_RADIUS);
        assert_eq!(outer.len(), inner.len(), "within {tolerance}");
        for (one, other) in outer.iter().zip(&inner) {
            assert!(
                (one - other).abs() < 1e-9,
                "{one} and {other} within {tolerance}"
            );
        }
    }
}

#[test]
fn no_sample_of_two_walls_touching_stands_closer_to_the_other_than_a_fifth_of_the_kernel_s_tolerance_but_their_vertices()
 {
    let grid_angle = 3.0 * TAU / 16.0;
    let bodies = [1e-6, 7e-5].into_iter().flat_map(|off| {
        [
            fixtures::stock_with_a_hole_tangent_at(grid_angle + off),
            fixtures::block_with_two_holes_touching_at(grid_angle + off),
        ]
    });
    for body in bodies {
        let eps = body.scale().eps();
        let cylinders: Vec<_> = body
            .surfaces
            .iter()
            .filter_map(|surface| match surface {
                Surface::Cylinder(cylinder) => Some(*cylinder),
                Surface::Plane(_) => None,
            })
            .collect();
        for tolerance in [1e-6, 0.02, 10.0] {
            let samples = Samples::of(&body, tolerance);
            for edge in body.edge_ids() {
                let Curve::Circle(circle) = *body.curve(body.edge(edge).curve) else {
                    continue;
                };
                for id in samples.edge(edge) {
                    if samples.is_vertex(*id) {
                        continue;
                    }
                    let point = samples.point(*id);
                    for other in &cylinders {
                        if (other.origin - circle.center).cross(other.axis).length() < eps {
                            continue;
                        }
                        let apart = other.distance(point).abs();
                        assert!(
                            apart >= eps * APART,
                            "{point} is {apart} from a wall within {tolerance}"
                        );
                    }
                }
            }
        }
    }
}

/// A body holding nothing but the whole loops where a cylinder of `radius`
/// lying along `axis` through `(0, 0, 5)` crosses the stock: what sampling
/// reads of it is its edges alone.
fn loops_across_the_stock(radius: f64, axis: DVec3) -> (Body, Vec<Meet>) {
    let mut build = fixtures::Build::new();
    let stock = build.cylinder(DVec3::ZERO, DVec3::Z, fixtures::STOCK_RADIUS);
    let bore = build.cylinder(DVec3::Z * 5.0, axis, radius);
    let meeting = build.meeting(stock, bore, fixtures::STOCK_RADIUS);
    for meet in &meeting.components {
        let period = meet.period().expect("a component closes on itself");
        build.meet_edge(*meet, None, 0.0, period);
    }
    (build.finish(fixtures::STOCK_RADIUS), meeting.components)
}

/// Whether `point` is one of the samples of `edge`, to within rounding.
fn sampled(samples: &Samples, edge: EdgeId, point: DVec3) -> bool {
    samples
        .edge(edge)
        .iter()
        .any(|id| (samples.point(*id) - point).length() < 1e-9)
}

#[test]
fn a_meet_is_sampled_at_every_grid_angle_of_either_cylinder_and_wherever_it_turns_back() {
    for (radius, axis) in [(3.0, DVec3::X), (5.0, DVec3::Y)] {
        let (body, meets) = loops_across_the_stock(radius, axis);
        assert_eq!(meets.len(), 2);
        for tolerance in [1e-3, 0.02, 0.5] {
            let samples = Samples::of(&body, tolerance);
            for (edge, meet) in body.edge_ids().zip(&meets) {
                for (on_first, cylinder) in [(true, meet.first), (false, meet.second)] {
                    let steps = divisions(cylinder.radius, tolerance);
                    let mut places = meet.turns(on_first);
                    for step in 0..steps {
                        places.extend(meet.at_angle(on_first, TAU * step as f64 / steps as f64));
                    }
                    assert!(!places.is_empty());
                    for t in places {
                        assert!(
                            sampled(&samples, edge, meet.point(t)),
                            "{radius} within {tolerance}: {t} on the first {on_first} is missed"
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn a_meet_between_two_vertices_runs_from_the_first_to_the_last_with_every_sample_between() {
    let mut build = fixtures::Build::new();
    let stock = build.cylinder(DVec3::ZERO, DVec3::Z, fixtures::STOCK_RADIUS);
    let bore = build.cylinder(DVec3::Z * 5.0, DVec3::Y, fixtures::HOLE_RADIUS);
    let meet = build
        .meeting(stock, bore, fixtures::STOCK_RADIUS)
        .components[0];
    let [top, bottom] = [PI / 2.0, 3.0 * PI / 2.0];
    let ends = [
        build.vertex(DVec3::new(0.0, fixtures::STOCK_RADIUS, fixtures::HEIGHT)),
        build.vertex(DVec3::new(0.0, fixtures::STOCK_RADIUS, 0.0)),
    ];
    let halves = [
        build.meet_edge(meet, Some(ends), top, bottom),
        build.meet_edge(meet, Some([ends[1], ends[0]]), bottom, top + TAU),
    ];
    let body = build.finish(fixtures::STOCK_RADIUS);
    for (end, &vertex) in ends.iter().enumerate() {
        assert!((meet.point([top, bottom][end]) - body.vertex(vertex).point).length() < 1e-12);
    }
    let eps = body.scale().eps();
    for tolerance in [1e-3, 0.02, 0.5] {
        let samples = Samples::of(&body, tolerance);
        for (half, [from, to]) in halves.iter().zip([[top, bottom], [bottom, top + TAU]]) {
            let ids = samples.edge(*half);
            let listed = body.edge(*half).ends.expect("between two vertices");
            assert_eq!(ids[0], listed[0].0 as usize);
            assert_eq!(ids[ids.len() - 1], listed[1].0 as usize);
            let mut before = from;
            for id in &ids[1..ids.len() - 1] {
                assert!(!samples.is_vertex(*id));
                let point = samples.point(*id);
                let t = body.parameter_on(*half, point);
                assert!(before < t && t < to, "{t} out of order within {tolerance}");
                assert!((meet.point(from) - point).length() > eps);
                assert!((meet.point(to) - point).length() > eps);
                before = t;
            }
        }
    }
}

#[test]
fn a_meet_shorter_than_the_kernel_s_tolerance_keeps_its_two_ends_and_nothing_between() {
    let mut build = fixtures::Build::new();
    let stock = build.cylinder(DVec3::ZERO, DVec3::Z, fixtures::STOCK_RADIUS);
    let bore = build.cylinder(DVec3::Z * 5.0, DVec3::X, 3.0);
    let meet = build
        .meeting(stock, bore, fixtures::STOCK_RADIUS)
        .components[0];
    let [from, to] = [1.0, 1.0 + 1e-12];
    let ends = [from, to].map(|t| build.vertex(meet.point(t)));
    let edge = build.meet_edge(meet, Some(ends), from, to);
    let body = build.finish(fixtures::STOCK_RADIUS);
    let samples = Samples::of(&body, 0.02);
    assert_eq!(
        samples.edge(edge),
        &[ends[0].0 as usize, ends[1].0 as usize]
    );
}

/// How far `point` stands from the segment between `start` and `end`.
fn off_the_chord(point: DVec3, start: DVec3, end: DVec3) -> f64 {
    let along = end - start;
    let share = ((point - start).dot(along) / along.length_squared()).clamp(0.0, 1.0);
    (point - (start + along * share)).length()
}

#[test]
fn every_chord_of_a_meet_stands_within_the_tolerance_of_the_curve_however_fine() {
    for (radius, axis) in [(3.0, DVec3::X), (5.0, DVec3::Y)] {
        let (body, meets) = loops_across_the_stock(radius, axis);
        for tolerance in [1e-6, 1e-3, 0.02, 0.5] {
            let samples = Samples::of(&body, tolerance);
            for (edge, meet) in body.edge_ids().zip(&meets) {
                let period = meet.period().expect("a component closes on itself");
                let mut places: Vec<f64> = samples
                    .edge(edge)
                    .iter()
                    .map(|id| meet.parameter(samples.point(*id)))
                    .collect();
                assert!(places.windows(2).all(|pair| pair[0] < pair[1]));
                places.push(period);
                for pair in places.windows(2) {
                    let (start, end) = (meet.point(pair[0]), meet.point(pair[1]));
                    for part in 1..32 {
                        let at = pair[0] + (pair[1] - pair[0]) * part as f64 / 32.0;
                        let off = off_the_chord(meet.point(at), start, end);
                        assert!(
                            off <= tolerance * (1.0 + 1e-9),
                            "{radius} within {tolerance}: {off} off between {} and {}",
                            pair[0],
                            pair[1]
                        );
                    }
                }
            }
        }
    }
}

/// The angles round the stock's axis, from X, of every sample of `edge`.
fn angles_of(samples: &Samples, edge: EdgeId) -> Vec<f64> {
    samples
        .edge(edge)
        .iter()
        .map(|id| {
            let point = samples.point(*id);
            point.y.atan2(point.x)
        })
        .collect()
}

#[test]
fn the_rings_of_the_stock_a_bore_crosses_are_sampled_through_every_sample_of_the_curve() {
    use super::super::tests::across;
    for body in [
        across::stock_bored_across(3.0, DVec3::X),
        across::stock_bored_touching_its_wall(),
    ] {
        for tolerance in [1e-3, 0.02, 0.5, 10.0] {
            let samples = Samples::of(&body, tolerance);
            let mut rings = Vec::new();
            let mut curve = Vec::new();
            for edge in body.edge_ids() {
                match body.curve(body.edge(edge).curve) {
                    Curve::Circle(_) => rings.push(angles_of(&samples, edge)),
                    Curve::Meet(_) => curve.extend(angles_of(&samples, edge)),
                    Curve::Line(_) => {}
                }
            }
            assert_eq!(rings.len(), 2);
            let steps = divisions(fixtures::STOCK_RADIUS, tolerance);
            for ring in &rings {
                assert!(ring.len() > steps);
                for angle in &curve {
                    assert!(
                        ring.iter().any(|on| (on - angle).abs() < 1e-9),
                        "{angle} is not a sample of the ring within {tolerance}"
                    );
                }
            }
        }
    }
}

#[test]
fn the_rings_of_two_walls_touching_are_sampled_on_the_line_they_touch_along() {
    let inside = DVec3::X * (fixtures::STOCK_RADIUS - fixtures::HOLE_RADIUS);
    let beside = DVec3::X * fixtures::HOLE_RADIUS;
    let bodies = [
        (
            fixtures::stock_with_a_hole_at(inside),
            fixtures::STOCK_RADIUS,
        ),
        (fixtures::block_with_holes_at(&[-beside, beside]), 0.0),
    ];
    for (body, touching) in &bodies {
        for tolerance in [1e-3, 0.02, 0.5] {
            let samples = Samples::of(body, tolerance);
            let mut rings = 0;
            for edge in body.edge_ids() {
                let Curve::Circle(circle) = *body.curve(body.edge(edge).curve) else {
                    continue;
                };
                let line = DVec3::new(*touching, 0.0, circle.center.z);
                rings += 1;
                assert!(
                    sampled(&samples, edge, line),
                    "the ring of radius {} misses {line} within {tolerance}",
                    circle.radius
                );
            }
            assert_eq!(rings, 4);
        }
    }
}

/// The angles round `center`, from X, of every sample of `edge`, sorted, a
/// circle's vertex counted once.
fn angles_round(samples: &Samples, edge: EdgeId, center: DVec3) -> Vec<f64> {
    let mut ids = samples.edge(edge).to_vec();
    if ids.len() > 1 && ids[0] == ids[ids.len() - 1] {
        ids.pop();
    }
    let mut angles: Vec<f64> = ids
        .iter()
        .map(|id| {
            let from = samples.point(*id) - center;
            from.y.atan2(from.x).rem_euclid(TAU)
        })
        .collect();
    angles.sort_by(f64::total_cmp);
    angles
}

#[test]
fn two_walls_a_hair_across_each_other_are_sampled_on_common_rays_whichever_holds_a_vertex() {
    let radius = fixtures::HOLE_RADIUS;
    for (other_radius, offset) in [(radius, 1e-5), (radius - 1e-3, 1.1e-3)] {
        let mut build = fixtures::Build::new();
        let first = build.cylinder(DVec3::ZERO, DVec3::Z, radius);
        let center = DVec3::X * offset;
        let second = build.cylinder(center, DVec3::Z, other_radius);
        let (sin, cos) = 0.3f64.sin_cos();
        let vertex = build.vertex(DVec3::new(cos, sin, 0.0) * radius);
        let through = build.circle(first, 0.0, Some(vertex));
        let ring = build.circle(second, 0.0, None);
        let use_of = |edge, forward| crate::brep::topology::Coedge { edge, forward };
        build.face(first, false, vec![vec![use_of(through, true)]]);
        build.face(second, false, vec![vec![use_of(ring, true)]]);
        let body = build.finish(radius);
        for tolerance in [1e-3, 0.02, 0.5] {
            let samples = Samples::of(&body, tolerance);
            let one = angles_round(&samples, through, center);
            let other = angles_round(&samples, ring, center);
            assert_eq!(
                one.len(),
                other.len(),
                "{offset} apart within {tolerance}: {one:?} against {other:?}"
            );
            for (one, other) in one.iter().zip(&other) {
                assert!(
                    (one - other).abs() < 1e-9,
                    "{one} and {other}, {offset} apart within {tolerance}"
                );
            }
        }
    }
}

#[test]
fn the_rings_of_two_walls_a_hair_across_each_other_are_sampled_on_the_lines_they_cross_along() {
    let (radius, offset) = (fixtures::HOLE_RADIUS, 1e-5);
    let mut build = fixtures::Build::new();
    let first = build.cylinder(DVec3::ZERO, DVec3::Z, radius);
    let second = build.cylinder(DVec3::X * offset, DVec3::Z, radius);
    let rings = [
        build.circle(first, 0.0, None),
        build.circle(second, fixtures::HEIGHT, None),
    ];
    let use_of = |edge, forward| crate::brep::topology::Coedge { edge, forward };
    for (wall, ring) in [first, second].into_iter().zip(rings) {
        build.face(wall, false, vec![vec![use_of(ring, true)]]);
    }
    let body = build.finish(fixtures::STOCK_RADIUS);
    let across = (radius * radius - offset * offset / 4.0).sqrt();
    for tolerance in [1e-3, 0.02, 0.5] {
        let samples = Samples::of(&body, tolerance);
        for (ring, height) in rings.into_iter().zip([0.0, fixtures::HEIGHT]) {
            for side in [-1.0, 1.0] {
                let line = DVec3::new(offset / 2.0, side * across, height);
                assert!(
                    sampled(&samples, ring, line),
                    "{line} is missed within {tolerance}"
                );
            }
        }
    }
}

#[test]
fn every_ring_of_a_cylinder_is_sampled_at_the_angle_of_every_vertex_on_that_cylinder() {
    let (from, to) = (0.3 + 1e-4, 0.3 + 3e-4);
    let body = fixtures::tube_with_a_window(from, to);
    for tolerance in [1e-3, 0.02, 0.5] {
        let samples = Samples::of(&body, tolerance);
        let mut rings = 0;
        for edge in body.edge_ids() {
            if body.edge(edge).ends.is_some() {
                continue;
            }
            let Curve::Circle(circle) = *body.curve(body.edge(edge).curve) else {
                continue;
            };
            rings += 1;
            for angle in [from, to] {
                assert!(
                    sampled(&samples, edge, circle.point(angle)),
                    "the ring of radius {} misses {angle} within {tolerance}",
                    circle.radius
                );
            }
        }
        assert_eq!(rings, 4);
    }
}
