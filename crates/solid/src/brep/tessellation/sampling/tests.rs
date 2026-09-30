use std::f64::consts::{PI, TAU};

use super::super::tests::fixtures;
use super::{Samples, divisions};
use crate::brep::curve::Curve;

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
