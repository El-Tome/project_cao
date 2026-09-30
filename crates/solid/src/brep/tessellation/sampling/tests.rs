use std::f64::consts::{PI, TAU};

use super::super::tests::fixtures;
use super::{Samples, divisions};
use crate::brep::curve::Curve;
use crate::brep::surface::Surface;
use crate::brep::topology::Body;

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
fn no_sample_of_two_walls_touching_stands_within_the_kernel_s_tolerance_of_the_other_but_their_vertices()
 {
    let near_a_grid_angle = 3.0 * TAU / 16.0 + 1e-6;
    for body in [
        fixtures::stock_with_a_hole_tangent_at(near_a_grid_angle),
        fixtures::block_with_two_holes_touching_at(near_a_grid_angle),
    ] {
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
                            apart >= eps,
                            "{point} is {apart} from a wall within {tolerance}"
                        );
                    }
                }
            }
        }
    }
}
