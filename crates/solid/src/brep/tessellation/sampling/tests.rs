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
    let bodies = [1e-6, 7e-5]
        .into_iter()
        .flat_map(|off| {
            [
                fixtures::stock_with_a_hole_tangent_at(grid_angle + off),
                fixtures::block_with_two_holes_touching_at(grid_angle + off),
            ]
        })
        .chain([fixtures::block_with_two_holes_touching_along_a_lean(5e-8)]);
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
fn a_meet_is_sampled_at_the_angle_of_every_ray_the_circles_of_either_cylinder_take() {
    let (body, meets) = loops_across_the_stock(3.0, DVec3::X);
    let eps = body.scale().eps();
    let off_the_grid = [0.123, 1.234, 2.345];
    for tolerance in [1e-3, 0.02, 0.5] {
        for (edge, meet) in body.edge_ids().zip(&meets) {
            for on_first in [true, false] {
                let rays: [&[f64]; 2] = if on_first {
                    [&off_the_grid, &[]]
                } else {
                    [&[], &off_the_grid]
                };
                let points =
                    super::meet::on_meet(meet, body.edge(edge), tolerance, eps, rays, &|_| true);
                for angle in off_the_grid {
                    for t in meet.at_angle(on_first, angle) {
                        let place = meet.point(t);
                        assert!(
                            points.iter().any(|point| (*point - place).length() < 1e-9),
                            "{angle} on the first {on_first} is missed within {tolerance}"
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
fn no_sample_of_an_arc_stands_so_near_its_end_that_it_lies_on_a_plane_touching_the_wall_there() {
    let mut build = fixtures::Build::new();
    let radius = 1.5;
    let wall = build.cylinder(DVec3::ZERO, DVec3::Z, radius);
    let start = build.vertex(DVec3::X * radius);
    let end = build.vertex(DVec3::Y * radius);
    let arc = build.arc(wall, 0.0, start, end);
    let hair: f64 = 3e-8;
    let near = build.vertex(DVec3::new(hair.cos(), hair.sin(), 0.0) * radius + DVec3::Z * 5.0);
    let through = build.circle(wall, 5.0, Some(near));
    let use_of = |edge, forward| crate::brep::topology::Coedge { edge, forward };
    build.face(wall, false, vec![vec![use_of(arc, true)]]);
    build.face(wall, false, vec![vec![use_of(through, true)]]);
    let beside = [
        DVec3::new(radius, -1.0, 0.0),
        DVec3::new(radius, -1.0, 1.0),
        DVec3::new(radius, 0.0, 1.0),
    ]
    .map(|point| build.vertex(point));
    build.polygon(&[start, beside[0], beside[1], beside[2]], DVec3::X);
    let body = build.finish(fixtures::STOCK_RADIUS);
    let eps = body.scale().eps();
    let touching = |point: DVec3| (radius - point.x).abs();
    for tolerance in [1e-3, 0.02, 0.5] {
        let samples = Samples::of(&body, tolerance);
        let ids = samples.edge(arc);
        for id in &ids[1..ids.len() - 1] {
            let point = samples.point(*id);
            assert!(
                touching(point) >= eps * APART,
                "{point} lies on the plane x = {radius} within {tolerance}"
            );
        }
    }
}

/// A ring passing the line a plane touches its wall along, and a vertex of
/// the wall a hair round from that line, further than the kernel's
/// tolerance: the ring would take a ray through it, where it stands on the
/// plane as good as the line does.
#[test]
fn no_sample_of_a_ring_passing_a_plane_touching_its_wall_lies_on_the_plane_but_the_line_itself() {
    let mut build = fixtures::Build::new();
    let radius = fixtures::HOLE_RADIUS;
    let wall = build.cylinder(DVec3::ZERO, DVec3::Z, radius);
    let hair: f64 = 1e-5;
    let off = build.vertex(DVec3::new(hair.cos(), hair.sin(), 0.0) * radius + DVec3::Z * 5.0);
    let ring = build.circle(wall, 0.0, None);
    let through = build.circle(wall, 5.0, Some(off));
    let use_of = |edge, forward| crate::brep::topology::Coedge { edge, forward };
    build.face(
        wall,
        false,
        vec![vec![use_of(ring, true)], vec![use_of(through, false)]],
    );
    let corners = [
        DVec3::new(radius, 0.0, 0.0),
        DVec3::new(radius, -1.0, 0.0),
        DVec3::new(radius, -1.0, 1.0),
        DVec3::new(radius, 0.0, 1.0),
    ]
    .map(|point| build.vertex(point));
    build.polygon(&corners, DVec3::X);
    let body = build.finish(fixtures::STOCK_RADIUS);
    let eps = body.scale().eps();
    for tolerance in [1e-3, 0.02, 0.5] {
        let samples = Samples::of(&body, tolerance);
        for id in samples.edge(ring) {
            let point = samples.point(*id);
            let on_the_line = (point - DVec3::X * radius).length() <= eps;
            assert!(
                on_the_line || (radius - point.x).abs() >= eps * APART,
                "{point} lies on the plane x = {radius} within {tolerance}"
            );
        }
    }
}

#[test]
fn a_ring_is_sampled_at_the_angle_of_a_corner_standing_inside_its_wall_closer_than_a_chord_sags() {
    let mut build = fixtures::Build::new();
    let radius = fixtures::STOCK_RADIUS;
    let wall = build.cylinder(DVec3::ZERO, DVec3::Z, radius);
    let ring = build.circle(wall, 0.0, None);
    let use_of = |edge, forward| crate::brep::topology::Coedge { edge, forward };
    build.face(wall, false, vec![vec![use_of(ring, true)]]);
    let angle: f64 = 0.3;
    let inside = radius - 1e-6;
    let corner = DVec3::new(angle.cos() * inside, angle.sin() * inside, 5.0);
    let corners = [
        corner,
        corner - DVec3::X * 4.0,
        corner - DVec3::X * 4.0 + DVec3::Z,
        corner + DVec3::Z,
    ]
    .map(|point| build.vertex(point));
    build.polygon(&corners, DVec3::Y);
    let body = build.finish(fixtures::STOCK_RADIUS);
    for tolerance in [1e-3, 0.02, 0.5] {
        let samples = Samples::of(&body, tolerance);
        let below = DVec3::new(angle.cos(), angle.sin(), 0.0) * radius;
        assert!(
            sampled(&samples, ring, below),
            "the ring misses {below} within {tolerance}"
        );
    }
}

#[test]
fn a_ring_is_sampled_at_a_vertex_s_angle_where_the_kernel_put_the_vertex_a_hair_off_the_wall() {
    let mut build = fixtures::Build::new();
    let radius = fixtures::HOLE_RADIUS;
    let wall = build.cylinder(DVec3::ZERO, DVec3::Z, radius);
    let off = DVec3::new(radius + 1e-8, 0.0, fixtures::HEIGHT);
    let vertex = build.vertex(off);
    let ring = build.circle(wall, 0.0, None);
    let through = build.circle(wall, fixtures::HEIGHT, Some(vertex));
    let use_of = |edge, forward| crate::brep::topology::Coedge { edge, forward };
    build.face(
        wall,
        false,
        vec![vec![use_of(ring, true)], vec![use_of(through, false)]],
    );
    let body = build.finish(fixtures::STOCK_RADIUS);
    for tolerance in [1e-3, 0.02, 0.5] {
        let samples = Samples::of(&body, tolerance);
        let below = DVec3::new(off.x, off.y, 0.0);
        assert!(
            samples
                .edge(ring)
                .iter()
                .any(|id| (samples.point(*id) - below).length() < 1e-12),
            "the ring misses {below} within {tolerance}"
        );
    }
}

/// A place of the ring at the grid angle nought and another a little more
/// than the kernel's tolerance round from it, both within it of a vertex
/// between them: each is put where the vertex nearest in height stands.
#[test]
fn no_two_samples_of_a_ring_stand_at_one_place_where_two_of_its_angles_are_put_at_one_vertex() {
    let mut build = fixtures::Build::new();
    let radius = fixtures::HOLE_RADIUS;
    let wall = build.cylinder(DVec3::ZERO, DVec3::Z, radius);
    let cylinder = crate::brep::surface::Cylinder::about(DVec3::ZERO, DVec3::Z, radius);
    let gap = crate::brep::scale::Scale::of(fixtures::STOCK_RADIUS).eps() / radius;
    let on_wall = |angle: f64, height: f64| {
        cylinder.origin + cylinder.radial(angle) * radius + DVec3::Z * height
    };
    let near = build.vertex(on_wall(0.9 * gap, 1.0));
    let far = build.vertex(on_wall(1.8 * gap, fixtures::HEIGHT));
    let ring = build.circle(wall, 0.0, None);
    let low = build.circle(wall, 1.0, Some(near));
    let high = build.circle(wall, fixtures::HEIGHT, Some(far));
    let use_of = |edge, forward| crate::brep::topology::Coedge { edge, forward };
    build.face(
        wall,
        false,
        vec![vec![use_of(ring, true)], vec![use_of(low, false)]],
    );
    build.face(
        wall,
        false,
        vec![vec![use_of(low, true)], vec![use_of(high, false)]],
    );
    let body = build.finish(fixtures::STOCK_RADIUS);
    for tolerance in [1e-3, 0.02, 0.5] {
        let samples = Samples::of(&body, tolerance);
        let points: Vec<DVec3> = samples
            .edge(ring)
            .iter()
            .map(|id| samples.point(*id))
            .collect();
        for (at, point) in points.iter().enumerate() {
            assert_ne!(
                *point,
                points[(at + 1) % points.len()],
                "sampled twice in a row within {tolerance}"
            );
        }
    }
}

/// A bore touching the stock inside, and a boss of its radius a hair off its
/// axis, crossing it along two lines: the stock takes the rays of the bore's
/// grid, and would pass them back to it round the lines where it stands
/// closer to the boss than a fifth of the kernel's tolerance.
#[test]
fn a_wall_takes_no_ray_passed_on_by_one_partner_where_it_all_but_lies_on_another() {
    let mut build = fixtures::Build::new();
    let stock = build.cylinder(DVec3::ZERO, DVec3::Z, fixtures::STOCK_RADIUS);
    let center = DVec3::X * (fixtures::STOCK_RADIUS - fixtures::HOLE_RADIUS);
    let off = 1e-7;
    let bore = build.cylinder(center, DVec3::Z, fixtures::HOLE_RADIUS);
    let boss = build.cylinder(center + DVec3::X * off, DVec3::Z, fixtures::HOLE_RADIUS);
    let rings = [stock, bore, boss].map(|wall| build.circle(wall, 0.0, None));
    let use_of = |edge, forward| crate::brep::topology::Coedge { edge, forward };
    for (wall, ring) in [stock, bore, boss].into_iter().zip(rings) {
        build.face(wall, false, vec![vec![use_of(ring, true)]]);
    }
    let body = build.finish(fixtures::STOCK_RADIUS);
    let eps = body.scale().eps();
    let walls = [bore, boss].map(|wall| match body.surface(wall) {
        Surface::Cylinder(cylinder) => *cylinder,
        Surface::Plane(_) => unreachable!("a bore and a boss are cylinders"),
    });
    let across = (fixtures::HOLE_RADIUS.powi(2) - off * off / 4.0).sqrt();
    let lines = [-1.0, 1.0].map(|side| center + DVec3::new(off / 2.0, side * across, 0.0));
    for tolerance in [1e-3, 0.02, 0.5] {
        let samples = Samples::of(&body, tolerance);
        for (ring, other) in [(rings[1], walls[1]), (rings[2], walls[0])] {
            for id in samples.edge(ring) {
                let point = samples.point(*id);
                let on_a_line = lines.iter().any(|line| (point - *line).length() < eps);
                assert!(
                    on_a_line || other.distance(point).abs() >= eps * APART,
                    "{point} all but lies on the other wall within {tolerance}"
                );
            }
        }
    }
}

/// The three walls are close one through the next, and every ray is drawn
/// from the smallest one's axis: the outer and the middle ones stand on the
/// same rays from it.
#[test]
fn a_wall_touching_one_inside_it_takes_the_rays_that_one_takes_from_a_third_inside_it() {
    let mut build = fixtures::Build::new();
    let outer = build.cylinder(DVec3::ZERO, DVec3::Z, fixtures::STOCK_RADIUS);
    let middle = DVec3::X * fixtures::STOCK_RADIUS / 2.0;
    let between = build.cylinder(middle, DVec3::Z, fixtures::STOCK_RADIUS / 2.0);
    let hub = middle + DVec3::Y * fixtures::HOLE_RADIUS;
    let inner = build.cylinder(hub, DVec3::Z, fixtures::HOLE_RADIUS);
    let rings = [outer, between, inner].map(|wall| build.circle(wall, 0.0, None));
    let use_of = |edge, forward| crate::brep::topology::Coedge { edge, forward };
    for (wall, ring) in [outer, between, inner].into_iter().zip(rings) {
        build.face(wall, false, vec![vec![use_of(ring, true)]]);
    }
    let body = build.finish(fixtures::STOCK_RADIUS);
    for tolerance in [0.02, 0.5] {
        let samples = Samples::of(&body, tolerance);
        let [one, other] = [rings[0], rings[1]].map(|ring| angles_round(&samples, ring, hub));
        assert_eq!(
            one.len(),
            other.len(),
            "within {tolerance}: {one:?} against {other:?}"
        );
        for (one, other) in one.iter().zip(&other) {
            assert!(
                (one - other).abs() < 1e-9,
                "{one} and {other} within {tolerance}"
            );
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

/// Two bands of one radius, their axes a hair apart, the second standing
/// above the first: nowhere do the two walls face each other.
fn bands_a_hair_apart_one_above_the_other(offset: f64) -> (Body, [EdgeId; 4]) {
    let mut build = fixtures::Build::new();
    let first = build.cylinder(DVec3::ZERO, DVec3::Z, fixtures::HOLE_RADIUS);
    let second = build.cylinder(DVec3::X * offset, DVec3::Z, fixtures::HOLE_RADIUS);
    let rings = [(first, 0.0), (first, 1.0), (second, 2.0), (second, 3.0)]
        .map(|(wall, height)| build.circle(wall, height, None));
    let use_of = |edge, forward| crate::brep::topology::Coedge { edge, forward };
    for (wall, [low, high]) in [
        (first, [rings[0], rings[1]]),
        (second, [rings[2], rings[3]]),
    ] {
        build.face(
            wall,
            false,
            vec![vec![use_of(low, true)], vec![use_of(high, false)]],
        );
    }
    (build.finish(fixtures::STOCK_RADIUS), rings)
}

#[test]
fn two_walls_a_hair_across_each_other_at_heights_apart_keep_every_step_of_their_grid() {
    let (body, rings) = bands_a_hair_apart_one_above_the_other(1.5 * 2e-8);
    for tolerance in [1e-3, 0.02] {
        let samples = Samples::of(&body, tolerance);
        let steps = divisions(fixtures::HOLE_RADIUS, tolerance);
        for ring in rings {
            let Curve::Circle(circle) = *body.curve(body.edge(ring).curve) else {
                panic!("a band is bounded by rings");
            };
            for step in 0..steps {
                let angle = TAU * step as f64 / steps as f64;
                assert!(
                    sampled(&samples, ring, circle.point(angle)),
                    "the ring at {} misses step {step} within {tolerance}",
                    circle.center.z
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

#[test]
fn a_ring_takes_a_ray_a_rounding_short_of_the_angle_it_starts_at() {
    let body = fixtures::stock();
    let edge = body.edge_ids().next().expect("the stock's first ring");
    let Curve::Circle(circle) = *body.curve(body.edge(edge).curve) else {
        panic!("the stock's edges are its two rings");
    };
    let contact = super::Contact {
        rays: vec![circle.u - circle.v * 1e-17],
        withheld: vec![(0, [f64::NEG_INFINITY, f64::INFINITY])],
        anchors: Vec::new(),
        beside: Vec::new(),
        beneath: Vec::new(),
    };
    let eps = body.scale().eps();
    let points = super::on_circle(
        &circle,
        body.edge(edge),
        0.02,
        eps,
        &contact,
        &super::Ends::of(&body, &circle, body.edge(edge)),
        &[],
    );
    let start = circle.point(0.0);
    assert!(
        points.iter().any(|point| (*point - start).length() < 1e-9),
        "{start} is missed"
    );
}

#[test]
fn a_circle_within_the_tolerance_of_two_walls_belongs_to_the_nearer() {
    use crate::brep::curve::Circle;
    use crate::brep::surface::Cylinder;
    use crate::brep::topology::SurfaceId;

    let eps = 1e-8;
    let first = Cylinder::about(DVec3::ZERO, DVec3::Z, 2.0);
    let second = Cylinder::about(DVec3::X * 0.6 * eps, DVec3::Z, 2.0);
    for walls in [
        [(SurfaceId(0), first), (SurfaceId(1), second)],
        [(SurfaceId(1), second), (SurfaceId(0), first)],
    ] {
        for (id, wall) in [(0, first), (1, second)] {
            let circle = Circle::on(&wall, 1.0);
            let found = super::super::contact::wall_of(&circle, &walls, eps);
            assert_eq!(found.map(|(surface, _)| surface.0), Some(id));
        }
    }
}
