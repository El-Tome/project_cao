use std::f64::consts::{PI, TAU};

use glam::DVec3;

use super::*;
use crate::brep::scale::Scale;
use crate::brep::surface::Cylinder;

const REACH: f64 = 40.0;

fn scale() -> Scale {
    Scale::of(REACH)
}

/// A cylinder of radius `a` standing on the Z axis, and one of radius `b`
/// lying along X through `(0, d, e)`: the frame of `meet.rs` is the world's.
fn pair(a: f64, b: f64, d: f64, e: f64) -> (Cylinder, Cylinder) {
    (
        Cylinder::about(DVec3::ZERO, DVec3::Z, a),
        Cylinder::about(DVec3::new(0.0, d, e), DVec3::X, b),
    )
}

/// Round the component, and a hair from each end of the span, where the roots
/// vanish and a node may sit.
fn samples(meet: &Meet) -> Vec<f64> {
    let period = meet.period().expect("a component closes on itself");
    let hair = 1e-9;
    let mut samples: Vec<f64> = (0..97).map(|step| period * step as f64 / 97.0).collect();
    let mut end = 0.0;
    while end < period {
        samples.extend([end + hair, end + PI - hair, end + PI + hair]);
        end += TAU;
    }
    samples
}

#[test]
fn every_point_of_a_meet_lies_on_both_cylinders() {
    for (first, second) in every_shape() {
        for meet in components(first, second) {
            for t in samples(&meet) {
                let point = meet.point(t);
                assert!(
                    first.distance(point).abs() < 1e-12 * REACH,
                    "{t}: {point} is off the first by {}",
                    first.distance(point)
                );
                assert!(
                    second.distance(point).abs() < 1e-12 * REACH,
                    "{t}: {point} is off the second by {}",
                    second.distance(point)
                );
            }
        }
    }
}

/// One pair of each shape, exactly: crossing, pierced by a larger one, pierced
/// by a smaller one, touching inside, equal and meeting, the small one inside
/// touching the large one.
fn every_shape() -> Vec<(Cylinder, Cylinder)> {
    vec![
        pair(5.0, 3.0, 4.0, 2.0),
        pair(3.0, 5.0, 1.0, -1.0),
        pair(5.0, 3.0, 0.5, 0.0),
        pair(5.0, 3.0, 2.0, 1.0),
        pair(4.0, 4.0, 0.0, 3.0),
        pair(3.0, 5.0, 2.0, 0.0),
    ]
}

fn components(first: Cylinder, second: Cylinder) -> Vec<Meet> {
    (0..2)
        .map(|component| Meet {
            first,
            second,
            component,
        })
        .filter(|meet| meet.period().is_some())
        .collect()
}

#[test]
fn a_point_of_a_meet_gives_back_its_own_parameter() {
    for (first, second) in every_shape() {
        for meet in components(first, second) {
            for t in samples(&meet) {
                let back = meet.parameter(meet.point(t));
                assert!(
                    (back - t).abs() < 1e-12 * (1.0 + t),
                    "{meet:?}: {t} came back as {back}"
                );
            }
        }
    }
}

#[test]
fn the_derivative_is_the_speed_of_the_point() {
    let step = 1e-6;
    for (first, second) in every_shape() {
        for meet in components(first, second) {
            for t in samples(&meet) {
                let numeric = (meet.point(t + step) - meet.point(t - step)) / (2.0 * step);
                let exact = meet.derivative(t);
                assert!(
                    (numeric - exact).length() < 1e-6 * (1.0 + exact.length()),
                    "{meet:?} at {t}: {exact} against {numeric}"
                );
            }
        }
    }
}

/// Pairs whose frame is not the world's, the second's axis given backwards.
fn turned() -> Vec<(Cylinder, Cylinder)> {
    let first = Cylinder::about(DVec3::new(3.0, 0.0, -1.0), DVec3::Y, 5.0);
    [3.0, 4.0, 6.0]
        .into_iter()
        .map(|radius| {
            let second = Cylinder::about(DVec3::new(1.0, 2.0, 0.0), -DVec3::Z, radius);
            (first, second)
        })
        .chain([(
            Cylinder::about(DVec3::new(0.0, 7.0, 7.0), DVec3::new(1.0, -1.0, 1.0), 4.0),
            Cylinder::about(DVec3::new(1.0, 2.0, 1.0), DVec3::new(1.0, 1.0, 0.0), 4.0),
        )])
        .collect()
}

fn around(angle: f64) -> f64 {
    (angle + PI).rem_euclid(TAU) - PI
}

#[test]
fn a_meet_seen_on_either_cylinder_is_where_that_cylinder_reads_its_points() {
    for (first, second) in every_shape().into_iter().chain(turned()) {
        for meet in components(first, second) {
            for on_first in [true, false] {
                let cylinder = if on_first { first } else { second };
                for t in samples(&meet) {
                    let [seen, ..] = meet.seen_on(on_first, t);
                    let read = cylinder.parameters(meet.point(t));
                    assert!(
                        around(seen.x - read.x).abs() < 1e-12
                            && (seen.y - read.y).abs() < 1e-12 * REACH,
                        "{meet:?} at {t} on the first {on_first}: {seen} against {read}"
                    );
                }
            }
        }
    }
}

#[test]
fn a_meet_seen_on_a_cylinder_turns_with_no_jump_and_its_derivatives_are_its_own() {
    let step = 1e-4;
    for (first, second) in every_shape().into_iter().chain(turned()) {
        for meet in components(first, second) {
            let period = meet.period().expect("a component closes on itself");
            for on_first in [true, false] {
                let seen = |t: f64| meet.seen_on(on_first, t);
                let turns = (seen(period)[0].x - seen(0.0)[0].x) / TAU;
                assert!(
                    (turns - turns.round()).abs() < 1e-9,
                    "{meet:?} turns {turns} times"
                );
                let mut before = seen(0.0)[0];
                for step_along in 1..=400 {
                    let t = period * step_along as f64 / 400.0;
                    let [here, first, second] = seen(t);
                    assert!((here - before).length() < 0.5, "{meet:?} jumps at {t}");
                    before = here;
                    let [after, behind] = [seen(t + step)[0], seen(t - step)[0]];
                    let slope = (after - behind) / (2.0 * step);
                    let bend = (after - 2.0 * here + behind) / (step * step);
                    assert!(
                        (slope - first).length() < 1e-6 * (1.0 + first.length()),
                        "{meet:?} at {t}: {first} against {slope}"
                    );
                    assert!(
                        (bend - second).length() < 1e-4 * (1.0 + second.length()),
                        "{meet:?} at {t}: {second} against {bend}"
                    );
                }
            }
        }
    }
}

#[test]
fn every_parameter_where_a_meet_crosses_an_angle_is_found_and_none_else() {
    let scan = 4000;
    for (first, second) in every_shape().into_iter().chain(turned()) {
        for meet in components(first, second) {
            let period = meet.period().expect("a component closes on itself");
            for on_first in [true, false] {
                for grid in 0..24 {
                    let angle = TAU * grid as f64 / 24.0 - PI;
                    let found = meet.at_angle(on_first, angle);
                    assert!(found.windows(2).all(|pair| pair[0] < pair[1]));
                    for &t in &found {
                        assert!((0.0..period).contains(&t));
                        let seen = meet.seen_on(on_first, t)[0].x;
                        assert!(
                            around(seen - angle).abs() < 1e-9,
                            "{meet:?} at {t} is at {seen}, not {angle}"
                        );
                    }
                    let side = |t: f64| around(meet.seen_on(on_first, t)[0].x - angle);
                    for step in 0..scan {
                        let [from, to] = [step, step + 1].map(|k| period * k as f64 / scan as f64);
                        let [before, after] = [side(from), side(to)];
                        if before * after < 0.0 && (before - after).abs() < 1.0 {
                            assert!(
                                found.iter().any(|&t| (from..=to).contains(&t)
                                    || (t - to + period).abs() < 1e-12),
                                "{meet:?} crosses {angle} between {from} and {to}, missed: {found:?}"
                            );
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn a_meet_turning_back_at_an_angle_is_found_there_once() {
    let (first, second) = pair(5.0, 3.0, -3.0, 1.0);
    let meet = components(first, second)[0];
    for (angle, x) in [(0.0, 5.0), (PI, -5.0)] {
        let found = meet.at_angle(true, angle);
        assert_eq!(found.len(), 1, "{angle}: {found:?}");
        assert!((meet.point(found[0]) - DVec3::new(x, 0.0, 1.0)).length() < 1e-12 * REACH);
    }
    for grid in 0..24 {
        let angle = TAU * grid as f64 / 24.0;
        let reached = 3.0 * angle.cos() > -2.0;
        assert_eq!(
            meet.at_angle(false, angle).len(),
            if reached { 2 } else { 0 },
            "the second is crossed at {angle} on each side of the first's axis, or not at all"
        );
    }
}

#[test]
fn a_meet_turning_back_at_an_angle_its_sine_rounds_is_still_found_there_once() {
    let apart = |one: f64, other: f64, period: f64| {
        let gap = (one - other).rem_euclid(period);
        gap.min(period - gap)
    };
    for (a, b) in [(20.0, 5.0), (20.0, 10.0), (20.0, 20.0)] {
        for d in (-6..=6).map(|step| 5.0 * step as f64) {
            for (one, other) in [
                (DVec3::Z, DVec3::X),
                (DVec3::X, DVec3::Y),
                (DVec3::Y, DVec3::Z),
            ] {
                let first = Cylinder::about(DVec3::ZERO, one, a);
                let second = Cylinder::about(one.cross(other) * d + one, other, b);
                for meet in Meeting::of(&first, &second, scale()).components {
                    let period = meet.period().expect("a component closes on itself");
                    for on_first in [true, false] {
                        for turn in meet.turns(on_first) {
                            let angle = meet.seen_on(on_first, turn)[0].x;
                            let found = meet.at_angle(on_first, angle);
                            let there = found
                                .iter()
                                .filter(|&&t| apart(t, turn, period) < 1e-6)
                                .count();
                            assert_eq!(there, 1, "{a} {b} {d} {one}: {turn} in {found:?}");
                        }
                        for grid in 0..96 {
                            let angle = TAU * grid as f64 / 96.0 - PI;
                            let found = meet.at_angle(on_first, angle);
                            for (index, &t) in found.iter().enumerate() {
                                for &other in &found[index + 1..] {
                                    assert!(
                                        apart(t, other, period) > 1e-6,
                                        "{a} {b} {d} {one} at {angle}: {found:?}"
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn a_meet_turns_back_on_a_cylinder_exactly_where_its_angle_stops_and_nowhere_else() {
    let scan = 4000;
    for (first, second) in every_shape().into_iter().chain(turned()) {
        for meet in components(first, second) {
            let period = meet.period().expect("a component closes on itself");
            for on_first in [true, false] {
                let turns = meet.turns(on_first);
                for &t in &turns {
                    assert!((0.0..period).contains(&t));
                    assert!(
                        meet.seen_on(on_first, t)[1].x.abs() < 1e-12,
                        "{meet:?} at {t}"
                    );
                }
                let turning = |t: f64| meet.seen_on(on_first, t)[1].x;
                for step in 0..scan {
                    let [from, to] = [step, step + 1].map(|k| period * k as f64 / scan as f64);
                    if turning(from) * turning(to) < 0.0 {
                        assert!(
                            turns.iter().any(|&t| (from..=to).contains(&t)),
                            "{meet:?} turns back between {from} and {to}, missed: {turns:?}"
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn a_component_the_pair_does_not_have_crosses_no_angle_and_never_turns() {
    let (first, second) = pair(5.0, 3.0, 4.0, 2.0);
    let missing = Meet {
        first,
        second,
        component: 1,
    };
    assert!(missing.at_angle(true, 0.0).is_empty());
    assert!(missing.turns(true).is_empty());
}

fn configuration(a: f64, b: f64, d: f64) -> Configuration {
    let (first, second) = pair(a, b, d, 1.5);
    Meeting::of(&first, &second, scale()).configuration
}

#[test]
fn each_pair_made_by_hand_meets_in_its_own_configuration() {
    assert_eq!(configuration(5.0, 3.0, 4.0), Configuration::OneLoop);
    assert_eq!(configuration(3.0, 5.0, 1.0), Configuration::TwoLoops);
    assert_eq!(configuration(5.0, 3.0, 0.5), Configuration::TwoLoops);
    assert_eq!(configuration(4.0, 4.0, 0.0), Configuration::TwoEllipses);
    assert_eq!(configuration(5.0, 3.0, 2.0), Configuration::FigureOfEight);
    assert_eq!(configuration(5.0, 3.0, -2.0), Configuration::FigureOfEight);
    assert_eq!(configuration(5.0, 3.0, 8.0), Configuration::Contact);
    assert_eq!(configuration(5.0, 3.0, -8.0), Configuration::Contact);
    assert_eq!(configuration(5.0, 3.0, 20.0), Configuration::Apart);
}

#[test]
fn a_pair_within_the_tolerance_of_a_touch_is_snapped_onto_it() {
    let eps = scale().eps();
    for hair in [-eps / 2.0, eps / 2.0] {
        assert_eq!(
            configuration(5.0, 3.0, 2.0 + hair),
            Configuration::FigureOfEight
        );
        assert_eq!(
            configuration(3.0, 5.0, -2.0 + hair),
            Configuration::FigureOfEight
        );
        assert_eq!(configuration(5.0, 3.0, 8.0 + hair), Configuration::Contact);
        assert_eq!(
            configuration(4.0, 4.0 + hair, 0.0),
            Configuration::TwoEllipses
        );
        assert_eq!(configuration(4.0, 4.0, hair), Configuration::TwoEllipses);
        assert_eq!(
            configuration(4.0, 4.0 + hair, -hair),
            Configuration::TwoEllipses
        );
    }
}

#[test]
fn a_pair_twice_the_tolerance_from_a_touch_is_left_as_it_is() {
    let eps = scale().eps();
    let hair = 2.0 * eps;
    assert_eq!(configuration(5.0, 3.0, 2.0 + hair), Configuration::OneLoop);
    assert_eq!(configuration(5.0, 3.0, 2.0 - hair), Configuration::TwoLoops);
    assert_eq!(configuration(5.0, 3.0, 8.0 - hair), Configuration::OneLoop);
    assert_eq!(configuration(5.0, 3.0, 8.0 + hair), Configuration::Apart);
    assert_eq!(configuration(4.0, 4.0 + hair, 0.0), Configuration::TwoLoops);
    assert_eq!(configuration(4.0, 4.0 - hair, 0.0), Configuration::TwoLoops);
    assert_eq!(configuration(4.0, 4.0, hair), Configuration::OneLoop);
    assert_eq!(configuration(4.0, 4.0, -hair), Configuration::OneLoop);
}

#[test]
fn a_pair_a_hair_wider_than_the_tolerance_from_a_touch_keeps_its_curve_exact_through_the_waist() {
    let hair = 1e-7;
    assert!(hair > 2.0 * scale().eps());
    for (a, b, d) in [
        (5.0, 3.0, 2.0 + hair),
        (5.0, 3.0, 2.0 - hair),
        (5.0, 3.0, -2.0 + hair),
        (4.0, 4.0 - hair, 0.0),
        (4.0, 4.0, hair),
        (5.0, 3.0, 8.0 - hair),
    ] {
        let (first, second) = pair(a, b, d, 1.5);
        let meeting = Meeting::of(&first, &second, scale());
        assert!(!meeting.components.is_empty(), "{a} {b} {d}");
        for meet in meeting.components {
            assert_eq!(meet.second, second, "{a} {b} {d}: moved");
            let period = meet.period().expect("a component closes on itself");
            let mut near_the_ends = samples(&meet);
            for end in [0.0, PI, TAU, 3.0 * PI]
                .into_iter()
                .filter(|&end| end < period)
            {
                for step in [1e-6, 1e-5, 1e-4, 1e-3] {
                    near_the_ends.extend([end + step, (end - step).rem_euclid(period)]);
                }
            }
            for t in near_the_ends {
                let point = meet.point(t);
                for cylinder in [first, second] {
                    assert!(
                        cylinder.distance(point).abs() < 1e-12 * REACH,
                        "{a} {b} {d} at {t}: off by {}",
                        cylinder.distance(point)
                    );
                }
                let back = meet.parameter(point);
                let gap = (back - t).rem_euclid(period);
                assert!(
                    gap.min(period - gap) < 1e-9,
                    "{a} {b} {d}: {t} came back as {back}"
                );
                for on_first in [true, false] {
                    let seen = meet.seen_on(on_first, t)[0];
                    let cylinder = if on_first { first } else { second };
                    let read = cylinder.parameters(point);
                    assert!(
                        around(seen.x - read.x).abs() < 1e-12
                            && (seen.y - read.y).abs() < 1e-12 * REACH,
                        "{a} {b} {d} at {t} on the first {on_first}: {seen} against {read}"
                    );
                }
            }
        }
    }
}

#[test]
fn a_node_is_where_its_components_pass_at_the_parameters_it_names() {
    let eps = scale().eps();
    for ((a, b, d), nodes, passes) in [
        ((4.0, 4.0, 0.0), 2, 2),
        ((4.0, 4.0 + eps / 3.0, eps / 3.0), 2, 2),
        ((5.0, 3.0, 2.0), 1, 2),
        ((3.0, 5.0, 2.0), 1, 2),
        ((5.0, 3.0, -2.0 + eps / 3.0), 1, 2),
    ] {
        let (first, second) = pair(a, b, d, -2.5);
        let meeting = Meeting::of(&first, &second, scale());
        assert_eq!(meeting.nodes.len(), nodes, "{a} {b} {d}");
        for node in &meeting.nodes {
            assert_eq!(node.on.len(), passes);
            assert!(first.distance(node.point).abs() <= eps);
            assert!(second.distance(node.point).abs() <= eps);
            for &(component, t) in &node.on {
                let meet = meeting.components[component as usize];
                assert!(
                    (meet.point(t) - node.point).length() < 1e-12 * REACH,
                    "{a} {b} {d}: {} is not {}",
                    meet.point(t),
                    node.point
                );
            }
        }
    }
}

#[test]
fn two_cylinders_touching_from_outside_meet_at_one_point_on_both() {
    let eps = scale().eps();
    for d in [8.0, -8.0, 8.0 - eps / 2.0, -8.0 - eps / 2.0] {
        let (first, second) = pair(5.0, 3.0, d, 4.0);
        let meeting = Meeting::of(&first, &second, scale());
        let contact = meeting.contact.expect("a point of contact");
        assert!(meeting.components.is_empty());
        assert!(first.distance(contact).abs() <= eps);
        assert!(second.distance(contact).abs() <= eps);
        assert!((contact - DVec3::new(0.0, 5.0 * d.signum(), 4.0)).length() <= eps);
    }
}

#[test]
fn a_meeting_is_the_same_whichever_cylinder_is_named_first() {
    let eps = scale().eps();
    for (a, b, d) in [
        (5.0, 3.0, 4.0),
        (3.0, 5.0, 2.0 + eps / 2.0),
        (4.0, 4.0, eps / 3.0),
        (4.0, 4.0 + eps / 3.0, 0.0),
    ] {
        let (first, second) = pair(a, b, d, 0.5);
        assert_eq!(
            Meeting::of(&first, &second, scale()),
            Meeting::of(&second, &first, scale())
        );
    }
}

#[test]
fn a_snapped_meet_stays_within_the_tolerance_of_both_cylinders() {
    let eps = scale().eps();
    for (a, b, d) in [
        (5.0, 3.0, 2.0 + eps / 2.0),
        (3.0, 5.0, -2.0 - eps / 2.0),
        (4.0, 4.0 + eps / 2.0, -eps / 2.0),
    ] {
        let (first, second) = pair(a, b, d, 1.5);
        let meeting = Meeting::of(&first, &second, scale());
        assert!(!meeting.components.is_empty());
        for meet in meeting.components {
            for t in samples(&meet) {
                let point = meet.point(t);
                assert!(
                    first.distance(point).abs() <= eps && second.distance(point).abs() <= eps,
                    "{point} is off: {} and {}",
                    first.distance(point),
                    second.distance(point)
                );
            }
        }
    }
}
