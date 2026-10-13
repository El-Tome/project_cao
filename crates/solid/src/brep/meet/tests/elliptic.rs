//! The curve two perpendicular walls meet along where one of them, or both,
//! is elliptic (#571): the same contract as two cylinders, read on the
//! ellipse's own eccentric angle.

use std::f64::consts::{PI, TAU};

use glam::DVec3;

use super::super::*;
use super::{REACH, around, samples, scale};
use crate::brep::curve::Wall;
use crate::brep::surface::{Cylinder, EllipticCylinder};

fn round(center: DVec3, axis: DVec3, radius: f64) -> Wall {
    Wall::Cylinder(Cylinder::about(center, axis, radius))
}

fn oval(center: DVec3, axis: DVec3, first: DVec3, second: f64) -> Wall {
    Wall::Elliptic(EllipticCylinder::about(center, axis, first, second))
}

/// The direction at `degrees` from `from` towards `towards`.
fn turned(from: DVec3, towards: DVec3, degrees: f64) -> DVec3 {
    let (sin, cos) = degrees.to_radians().sin_cos();
    from * cos + towards * sin
}

/// An elliptic post on the Z axis, half-axes 5 along X and 3 along Y, or
/// turned by `degrees` about its axis.
fn post(degrees: f64) -> Wall {
    oval(
        DVec3::ZERO,
        DVec3::Z,
        turned(DVec3::X, DVec3::Y, degrees) * 5.0,
        3.0,
    )
}

/// Pairs of every shape with an elliptic wall among them: crossing, pierced
/// by a larger wall, piercing, touching inside at a node, of one half-width
/// with their axes meeting, the second inside the first touching it; round
/// through elliptic, elliptic through round, elliptic through elliptic,
/// their half-axes along the frame or turned off it.
fn every_shape() -> Vec<(Wall, Wall)> {
    let bar = |center: DVec3, degrees: f64, first: f64, second: f64| {
        oval(
            center,
            DVec3::X,
            turned(DVec3::Y, DVec3::Z, degrees) * first,
            second,
        )
    };
    vec![
        (post(0.0), round(DVec3::new(0.0, 1.5, 0.0), DVec3::X, 2.0)),
        (post(30.0), round(DVec3::new(0.0, 0.5, 1.0), DVec3::X, 1.5)),
        (post(30.0), round(DVec3::new(0.0, 1.0, 1.0), DVec3::X, 6.0)),
        (
            round(DVec3::ZERO, DVec3::Z, 5.0),
            bar(DVec3::new(0.0, 1.0, 2.0), 20.0, 3.0, 1.5),
        ),
        (
            round(DVec3::ZERO, DVec3::Z, 2.5),
            bar(DVec3::new(0.0, 0.5, -1.0), 70.0, 4.0, 2.0),
        ),
        (post(0.0), round(DVec3::new(0.0, 1.0, 0.0), DVec3::X, 2.0)),
        (post(0.0), round(DVec3::new(0.0, 0.0, 2.0), DVec3::X, 3.0)),
        (post(40.0), bar(DVec3::new(0.0, 0.7, 0.5), 25.0, 2.5, 1.2)),
        (post(-35.0), bar(DVec3::new(0.0, 2.5, -0.5), 0.0, 1.8, 3.5)),
        (post(0.0), bar(DVec3::new(0.0, 0.0, 1.0), 0.0, 3.0, 1.0)),
    ]
}

/// Pairs whose frame is not the world's: axes leaning, given backwards.
fn leaning() -> Vec<(Wall, Wall)> {
    let axis = DVec3::new(1.0, -1.0, 1.0).normalize();
    let across = DVec3::new(1.0, 1.0, 0.0).normalize();
    let square = axis.cross(across);
    vec![
        (
            oval(DVec3::new(0.0, 7.0, 7.0), axis, across * 5.0, 3.0),
            round(DVec3::new(0.0, 7.0, 7.0) + square, -across, 2.5),
        ),
        (
            round(DVec3::new(0.0, 7.0, 7.0), axis, 4.0),
            oval(
                DVec3::new(0.0, 7.0, 7.0) - square * 2.0 + axis,
                across,
                turned(square, axis, 35.0) * 3.0,
                1.5,
            ),
        ),
        (
            oval(
                DVec3::new(2.0, 1.0, -1.0),
                -axis,
                turned(across, square, 60.0) * 4.0,
                2.5,
            ),
            oval(
                DVec3::new(2.0, 1.0, -1.0) + square * 0.5 - axis,
                across,
                turned(square, axis, 10.0) * 2.0,
                1.2,
            ),
        ),
    ]
}

fn components(first: Wall, second: Wall) -> Vec<Meet> {
    (0..2)
        .map(|component| Meet {
            first,
            second,
            component,
        })
        .filter(|meet| meet.period().is_some())
        .collect()
}

fn every_meet() -> Vec<(Wall, Wall, Meet)> {
    every_shape()
        .into_iter()
        .chain(leaning())
        .chain(noded())
        .flat_map(|(first, second)| {
            components(first, second)
                .into_iter()
                .map(move |meet| (first, second, meet))
        })
        .collect()
}

#[test]
fn every_shape_has_a_curve() {
    for (first, second) in every_shape().into_iter().chain(leaning()) {
        assert!(
            !components(first, second).is_empty(),
            "{first:?} {second:?}"
        );
    }
}

#[test]
fn every_point_of_a_meet_on_an_elliptic_wall_lies_on_both_walls() {
    for (first, second, meet) in every_meet() {
        for t in samples(&meet) {
            let point = meet.point(t);
            for wall in [first, second] {
                assert!(
                    wall.surface().distance(point).abs() < 1e-12 * REACH,
                    "{meet:?} at {t}: {point} is off {wall:?} by {}",
                    wall.surface().distance(point)
                );
            }
        }
    }
}

#[test]
fn a_point_of_a_meet_on_an_elliptic_wall_gives_back_its_own_parameter() {
    for (_, _, meet) in every_meet() {
        let period = meet.period().expect("a component closes on itself");
        for t in samples(&meet) {
            let back = meet.parameter(meet.point(t));
            let gap = (back - t).rem_euclid(period);
            assert!(
                gap.min(period - gap) < 1e-9,
                "{meet:?}: {t} came back as {back}"
            );
        }
    }
}

#[test]
fn the_derivative_of_a_meet_on_an_elliptic_wall_is_the_speed_of_its_point() {
    let step = 1e-6;
    for (_, _, meet) in every_meet() {
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

#[test]
fn every_point_of_a_stretch_of_a_meet_on_an_elliptic_wall_lies_in_its_box() {
    for (_, _, meet) in every_meet() {
        let period = meet.period().expect("a component closes on itself");
        for (from, to) in [
            (0.0, period),
            (0.3, 1.1),
            (2.0, 5.5),
            (period - 0.4, period + 0.9),
            (1.0, 1.0 + 1e-6),
        ] {
            let [low, high] = meet.bounds(from, to);
            for step in 0..=2000 {
                let point = meet.point(from + (to - from) * step as f64 / 2000.0);
                assert!(
                    point.cmpge(low).all() && point.cmple(high).all(),
                    "{meet:?} from {from} to {to}: {point} outside {low}..{high}"
                );
            }
        }
    }
}

/// The parameters of a wall, `(θ, h)`, `θ` the eccentric angle on an
/// elliptic one.
fn read(wall: &Wall, point: DVec3) -> glam::DVec2 {
    match wall {
        Wall::Cylinder(cylinder) => cylinder.parameters(point),
        Wall::Elliptic(oval) => oval.parameters(point),
    }
}

#[test]
fn a_meet_seen_on_either_wall_is_where_that_wall_reads_its_points() {
    for (first, second, meet) in every_meet() {
        for on_first in [true, false] {
            let wall = if on_first { first } else { second };
            for t in samples(&meet) {
                let [seen, ..] = meet.seen_on(on_first, t);
                let read = read(&wall, meet.point(t));
                assert!(
                    around(seen.x - read.x).abs() < 1e-11
                        && (seen.y - read.y).abs() < 1e-12 * REACH,
                    "{meet:?} at {t} on the first {on_first}: {seen} against {read}"
                );
            }
        }
    }
}

#[test]
fn a_meet_seen_on_an_elliptic_wall_turns_with_no_jump_and_its_derivatives_are_its_own() {
    let step = 1e-4;
    for (_, _, meet) in every_meet() {
        let period = meet.period().expect("a component closes on itself");
        for on_first in [true, false] {
            let seen = |t: f64| meet.seen_on(on_first, t);
            let turns = (seen(period)[0].x - seen(0.0)[0].x) / TAU;
            assert!(
                (turns - turns.round()).abs() < 1e-9,
                "{meet:?} turns {turns} times"
            );
            let mut before = seen(0.0)[0];
            for along in 1..=400 {
                let t = period * along as f64 / 400.0;
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

#[test]
fn every_parameter_where_a_meet_crosses_an_eccentric_angle_is_found_and_none_else() {
    let scan = 4000;
    for (_, _, meet) in every_meet() {
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
                            found
                                .iter()
                                .any(|&t| (from..=to).contains(&t)
                                    || (t - to + period).abs() < 1e-12),
                            "{meet:?} crosses {angle} between {from} and {to}, missed: {found:?}"
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn a_meet_turns_back_on_an_elliptic_wall_exactly_where_its_angle_stops_and_nowhere_else() {
    let scan = 4000;
    for (_, _, meet) in every_meet() {
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

/// The post of half-axes 5 along X and 3 along Y, crossed by a round bar of
/// `radius` along X through `(0, d, 1.5)`: the post reaches 3 along the
/// height, so the shapes stand where a circular post of radius 3 has them.
fn configuration(radius: f64, d: f64) -> Configuration {
    let bar = round(DVec3::new(0.0, d, 1.5), DVec3::X, radius);
    Meeting::of(&post(0.0), &bar, scale()).configuration
}

#[test]
fn each_pair_with_an_elliptic_wall_meets_in_its_own_configuration() {
    assert_eq!(configuration(2.0, 2.0), Configuration::OneLoop);
    assert_eq!(configuration(5.0, 1.0), Configuration::TwoLoops);
    assert_eq!(configuration(2.0, 0.5), Configuration::TwoLoops);
    assert_eq!(configuration(3.0, 0.0), Configuration::TwoEllipses);
    assert_eq!(configuration(2.0, 1.0), Configuration::FigureOfEight);
    assert_eq!(configuration(2.0, -1.0), Configuration::FigureOfEight);
    assert_eq!(configuration(2.0, 5.0), Configuration::Contact);
    assert_eq!(configuration(2.0, -5.0), Configuration::Contact);
    assert_eq!(configuration(2.0, 9.0), Configuration::Apart);
}

#[test]
fn a_pair_with_an_elliptic_wall_within_the_tolerance_of_a_touch_is_snapped_onto_it() {
    let eps = scale().eps();
    for hair in [-eps / 2.0, eps / 2.0] {
        assert_eq!(configuration(2.0, 1.0 + hair), Configuration::FigureOfEight);
        assert_eq!(configuration(2.0, 5.0 + hair), Configuration::Contact);
        assert_eq!(configuration(3.0 + hair, 0.0), Configuration::TwoEllipses);
        assert_eq!(configuration(3.0, hair), Configuration::TwoEllipses);
        assert_eq!(configuration(3.0 + hair, -hair), Configuration::TwoEllipses);
    }
}

#[test]
fn a_pair_with_an_elliptic_wall_twice_the_tolerance_from_a_touch_is_left_as_it_is() {
    let hair = 2.0 * scale().eps();
    assert_eq!(configuration(2.0, 1.0 + hair), Configuration::OneLoop);
    assert_eq!(configuration(2.0, 1.0 - hair), Configuration::TwoLoops);
    assert_eq!(configuration(2.0, 5.0 - hair), Configuration::OneLoop);
    assert_eq!(configuration(2.0, 5.0 + hair), Configuration::Apart);
    assert_eq!(configuration(3.0 + hair, 0.0), Configuration::TwoLoops);
    assert_eq!(configuration(3.0, hair), Configuration::OneLoop);
}

/// Pairs within half the tolerance of a touch, of every kind: an elliptic
/// wall moved onto a node or a contact, and an elliptic wall of one
/// half-width with a round one but for a hair, at a node of both ends. Two
/// elliptic walls turned off the height stand a third of it off a node of
/// both ends: the half-axis that takes the hair moves the far end of the
/// wall by up to twice as much.
fn a_hair_from_a_touch() -> Vec<(Wall, Wall)> {
    let hair = scale().eps() / 2.0;
    let third = scale().eps() / 3.0;
    let bar = |center: DVec3, degrees: f64, first: f64, second: f64| {
        oval(
            center,
            DVec3::X,
            turned(DVec3::Y, DVec3::Z, degrees) * first,
            second,
        )
    };
    let turned_post = post(30.0);
    let Wall::Elliptic(leaning) = turned_post else {
        unreachable!("the post is elliptic")
    };
    let reach = leaning.support(DVec3::Y);
    let Wall::Elliptic(across) = bar(DVec3::new(0.0, third, 1.0), 40.0, 3.0, 1.5) else {
        unreachable!("the bar is elliptic")
    };
    let wider = (reach + third) / across.support(DVec3::Y);
    vec![
        (
            turned_post,
            bar(DVec3::new(0.0, third, 1.0), 40.0, 3.0 * wider, 1.5 * wider),
        ),
        (
            turned_post,
            round(DVec3::new(0.0, -hair, 0.5), DVec3::X, reach - hair),
        ),
        (
            post(0.0),
            round(DVec3::new(0.0, 1.0 + hair, 0.5), DVec3::X, 2.0),
        ),
        (
            post(0.0),
            round(DVec3::new(0.0, 0.0, 0.5), DVec3::X, 3.0 + hair),
        ),
        (
            post(0.0),
            round(DVec3::new(0.0, -hair, 0.5), DVec3::X, 3.0 - hair),
        ),
        (
            turned_post,
            round(DVec3::new(0.0, reach + 2.0 - hair, 0.5), DVec3::X, 2.0),
        ),
        (
            round(DVec3::ZERO, DVec3::Z, 2.0 + hair),
            bar(DVec3::new(0.0, 0.0, 1.0), 0.0, 2.0, 3.5),
        ),
        (
            round(DVec3::ZERO, DVec3::Z, 6.0),
            bar(DVec3::new(0.0, 3.0 - hair, 1.0), 0.0, 3.0, 1.0),
        ),
        (
            post(0.0),
            bar(DVec3::new(0.0, hair, 1.0), 0.0, 3.0 + hair, 1.5),
        ),
        (
            post(0.0),
            bar(DVec3::new(0.0, hair, 1.0), 90.0, 1.5, 3.0 + hair),
        ),
    ]
}

#[test]
fn a_snapped_meet_on_an_elliptic_wall_stays_within_the_tolerance_of_both_walls() {
    let eps = scale().eps();
    for (first, second) in a_hair_from_a_touch() {
        let meeting = Meeting::of(&first, &second, scale());
        assert!(
            !meeting.nodes.is_empty() || meeting.contact.is_some(),
            "{first:?} {second:?}: {:?}",
            meeting.configuration
        );
        let points: Vec<DVec3> = meeting
            .components
            .iter()
            .flat_map(|meet| samples(meet).into_iter().map(|t| meet.point(t)))
            .chain(meeting.nodes.iter().map(|node| node.point))
            .chain(meeting.contact)
            .collect();
        for point in points {
            for wall in [first, second] {
                assert!(
                    wall.surface().distance(point).abs() <= eps,
                    "{first:?} {second:?}: {point} is off {wall:?} by {}",
                    wall.surface().distance(point)
                );
            }
        }
        for meet in &meeting.components {
            for node in &meeting.nodes {
                for &(component, t) in &node.on {
                    if component == meet.component {
                        assert!((meet.point(t) - node.point).length() < 1e-12 * REACH);
                    }
                }
            }
        }
    }
}

#[test]
fn a_wall_moved_onto_a_touch_with_an_elliptic_one_leaves_nothing_more_to_move() {
    for (first, second) in a_hair_from_a_touch() {
        for (mover, (rank, moved_to)) in [
            ("moved", moved(&first, &second, scale())),
            ("instead", moved_instead(&first, &second, scale())),
        ]
        .into_iter()
        .filter_map(|(name, found)| Some((name, found?)))
        {
            let mut pair = [first, second];
            pair[rank] = moved_to;
            let again = moved(&pair[0], &pair[1], scale());
            let left = again.map_or(0.0, |(rank, to)| match (pair[rank], to) {
                (Wall::Cylinder(from), Wall::Cylinder(to)) => {
                    from.origin.distance(to.origin) + (from.radius - to.radius).abs()
                }
                (Wall::Elliptic(from), Wall::Elliptic(to)) => from.gap(&to),
                _ => f64::INFINITY,
            });
            assert!(
                left <= 1e-12 * REACH,
                "{mover} {first:?} {second:?}: {left} left to move"
            );
        }
    }
}

#[test]
fn an_elliptic_wall_given_the_half_width_of_a_round_one_at_a_node_keeps_its_other_half_axis() {
    let hair = scale().eps() / 2.0;
    let bar = oval(
        DVec3::new(0.0, 0.0, 1.0),
        DVec3::X,
        DVec3::Y * (3.0 + hair),
        1.5,
    );
    let Some((1, Wall::Elliptic(snapped))) = moved(&post(0.0), &bar, scale()) else {
        panic!("the bar moves onto the node")
    };
    assert!((snapped.support(DVec3::Y) - 3.0).abs() <= 1e-15 * REACH);
    assert_eq!(snapped.minor, 1.5);
}

#[test]
fn two_walls_one_elliptic_touching_from_outside_meet_at_one_point_on_both() {
    let eps = scale().eps();
    for d in [5.0, -5.0, 5.0 - eps / 2.0, -5.0 - eps / 2.0] {
        let bar = round(DVec3::new(0.0, d, 4.0), DVec3::X, 2.0);
        let meeting = Meeting::of(&post(0.0), &bar, scale());
        let contact = meeting.contact.expect("a point of contact");
        assert!(meeting.components.is_empty());
        for wall in [post(0.0), bar] {
            assert!(wall.surface().distance(contact).abs() <= eps);
        }
        assert!((contact - DVec3::new(0.0, 3.0 * d.signum(), 4.0)).length() <= eps);
    }
}

#[test]
fn two_walls_one_elliptic_and_turned_touch_where_each_reaches_the_other() {
    let eps = scale().eps();
    let turned_post = post(30.0);
    let Wall::Elliptic(leaning) = turned_post else {
        unreachable!("the post is elliptic")
    };
    let reach = leaning.support(DVec3::Y);
    let bar = oval(
        DVec3::new(0.0, reach + 2.0, 4.0),
        DVec3::X,
        turned(DVec3::Y, DVec3::Z, 40.0) * 2.5,
        1.0,
    );
    let Wall::Elliptic(crossing) = bar else {
        unreachable!("the bar is elliptic")
    };
    let gap = reach + crossing.support(DVec3::Y) - (reach + 2.0);
    let bar = oval(
        DVec3::new(0.0, reach + 2.0 + gap, 4.0),
        DVec3::X,
        turned(DVec3::Y, DVec3::Z, 40.0) * 2.5,
        1.0,
    );
    let meeting = Meeting::of(&turned_post, &bar, scale());
    let contact = meeting.contact.expect("a point of contact");
    for wall in [turned_post, bar] {
        assert!(wall.surface().distance(contact).abs() <= eps, "{wall:?}");
    }
}

#[test]
fn a_meeting_with_an_elliptic_wall_is_the_same_whichever_wall_is_named_first() {
    for (first, second) in every_shape()
        .into_iter()
        .chain(leaning())
        .chain(a_hair_from_a_touch())
    {
        assert_eq!(
            Meeting::of(&first, &second, scale()),
            Meeting::of(&second, &first, scale())
        );
    }
}

/// Pairs whose curve has nodes, an elliptic wall among them, its axes
/// along the height or turned off it: a round bar inside an elliptic post
/// touching it, of its half-width through its axis, and an elliptic bar
/// turned of the post's half-width.
fn noded() -> Vec<(Wall, Wall)> {
    let turned_post = post(25.0);
    let Wall::Elliptic(leaning) = turned_post else {
        unreachable!("the post is elliptic")
    };
    let reach = leaning.support(DVec3::Y);
    let Wall::Elliptic(bar) = oval(
        DVec3::new(0.0, 0.0, -2.5),
        DVec3::X,
        turned(DVec3::Y, DVec3::Z, 30.0) * 3.0,
        1.5,
    ) else {
        unreachable!("the bar is elliptic")
    };
    let across = oval(
        DVec3::new(0.0, 0.0, -2.5),
        DVec3::X,
        turned(DVec3::Y, DVec3::Z, 30.0) * 3.0 * reach / bar.support(DVec3::Y),
        1.5 * reach / bar.support(DVec3::Y),
    );
    vec![
        (post(0.0), round(DVec3::new(0.0, 1.0, -2.5), DVec3::X, 2.0)),
        (post(0.0), round(DVec3::new(0.0, -1.0, -2.5), DVec3::X, 2.0)),
        (post(0.0), round(DVec3::new(0.0, 0.0, -2.5), DVec3::X, 3.0)),
        (
            turned_post,
            round(DVec3::new(0.0, reach - 2.0, -2.5), DVec3::X, 2.0),
        ),
        (
            turned_post,
            round(DVec3::new(0.0, 0.0, -2.5), DVec3::X, reach),
        ),
        (turned_post, across),
    ]
}

#[test]
fn a_point_at_a_node_with_an_elliptic_wall_names_every_pass_through_it_and_elsewhere_none() {
    let eps = scale().eps();
    for pair in noded() {
        let (first, second) = pair;
        let meeting = Meeting::of(&first, &second, scale());
        assert!(!meeting.nodes.is_empty(), "{pair:?}");
        for meet in &meeting.components {
            for node in &meeting.nodes {
                let named: Vec<f64> = node
                    .on
                    .iter()
                    .filter(|(component, _)| *component == meet.component)
                    .map(|(_, t)| *t)
                    .collect();
                let near = node.point + DVec3::splat(eps / 4.0);
                assert_eq!(meet.passes(near, eps), named, "{pair:?}");
            }
            let period = meet.period().expect("a component");
            for t in [period / 8.0, period * 3.0 / 8.0, period * 5.0 / 8.0] {
                assert!(meet.passes(meet.point(t), eps).is_empty());
            }
        }
    }
}

#[test]
fn a_meet_knows_its_own_elliptic_walls_and_no_other() {
    let eps = scale().eps();
    let (first, second) = every_shape()[7];
    let meet = components(first, second)[0];
    assert_eq!(meet.own(&first, eps), Some(true));
    assert_eq!(meet.own(&second, eps), Some(false));
    let Wall::Elliptic(beside) = first else {
        unreachable!("the post is elliptic")
    };
    let off = Wall::Elliptic(beside.moved(DVec3::X * 3.0 * eps));
    assert_eq!(meet.own(&off, eps), None);
    assert_eq!(meet.own(&round(DVec3::ZERO, DVec3::Z, 5.0), eps), None);
}
