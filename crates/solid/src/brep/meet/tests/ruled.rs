//! The curve a cone meets a wall along, read along the cone's rulings (#577),
//! across the seam where a lap's parameter comes back to nought.

use std::f64::consts::{PI, TAU};

use glam::{DVec2, DVec3};

use super::super::*;
use super::scale;
use crate::brep::curve::Wall;
use crate::brep::surface::{Cone, Cylinder};

/// A cone of `degrees` from its apex at `apex` along `axis`.
fn pin(apex: DVec3, axis: DVec3, degrees: f64) -> Cone {
    let slope = degrees.to_radians().tan();
    Cone::through(apex, axis, [DVec2::ZERO, DVec2::new(12.0, 12.0 * slope)])
}

/// A pin of five degrees, its apex four radii above a rod of radius two
/// lying along Y, leaning towards X until its steepest ruling touches the
/// rod's side: every other ruling crosses the rod twice.
fn touching_the_rod() -> (Wall, Wall) {
    let edge = (2.0f64 / 8.0).asin();
    let lean = edge - 5f64.to_radians();
    let pin = pin(
        DVec3::new(0.0, 0.0, 8.0),
        DVec3::new(lean.sin(), 0.0, -lean.cos()),
        5.0,
    );
    let rod = Cylinder::about(DVec3::ZERO, DVec3::Y, 2.0);
    (Wall::Cone(pin), Wall::Cylinder(rod))
}

#[test]
fn a_lap_round_a_cone_through_a_single_node_closes_on_itself() {
    let (pin, rod) = touching_the_rod();
    let read = ruled(&pin, &rod).expect("the pin's rulings are read against the rod");
    assert_eq!(read.configuration, Configuration::TwoEllipses);
    assert_eq!(read.doubles.len(), 1, "{:?}", read.doubles);
    for component in 0..2 {
        let meet = Meet {
            first: pin,
            second: rod,
            component,
        };
        let period = meet.period().expect("each lap closes on itself");
        let [start, end] = [meet.point(0.0), meet.point(period)];
        assert!(
            start.distance(end) <= scale().eps(),
            "lap {component} starts at {start} and ends at {end}"
        );
    }
}

/// A pin of five degrees standing straight over a rod of radius two along
/// its own `v`, the rod turned `degrees` about the pin's axis: the curve on
/// the rod turns back in the rod's angle where the pin's own is nought,
/// along the rod, and at a half-turn.
fn over_the_rod(degrees: f64) -> (Wall, Wall) {
    let pin = pin(DVec3::new(0.0, 0.0, 8.0), -DVec3::Z, 5.0);
    let (sin, cos) = degrees.to_radians().sin_cos();
    let along = pin.v * cos - pin.u * sin;
    let rod = Cylinder::about(DVec3::ZERO, along, 2.0);
    (Wall::Cone(pin), Wall::Cylinder(rod))
}

#[test]
fn a_lap_turning_back_on_the_other_wall_at_its_seam_turns_there() {
    let step = TAU / 256.0;
    for degrees in [0.0, 0.3] {
        let (pin, rod) = over_the_rod(degrees);
        for component in 0..2 {
            let meet = Meet {
                first: pin,
                second: rod,
                component,
            };
            assert!(meet.period().is_some(), "lap {component} at {degrees}°");
            let turns = meet.turns(false);
            for (place, name) in [(0.0, "its seam"), (PI, "a half-turn")] {
                assert!(
                    turns.iter().any(|&t| {
                        let off = (t - place).rem_euclid(TAU);
                        off.min(TAU - off) < step && meet.seen_on(false, t)[1].x.abs() < 1e-9
                    }),
                    "lap {component} at {degrees}° turns back at {name}, found {turns:?}"
                );
            }
        }
    }
}

#[test]
fn at_the_exact_angle_of_a_node_of_its_table_a_ruled_meet_is_found_only_where_it_stands_there() {
    let shapes = [touching_the_rod(), over_the_rod(0.0), over_the_rod(0.3)];
    for (pin, rod) in shapes {
        for component in 0..2 {
            let meet = Meet {
                first: pin,
                second: rod,
                component,
            };
            let read = meet
                .ruled()
                .expect("the pin's rulings are read against the rod");
            let lap = meet.lap(&read).expect("each pair has two laps");
            for &[node, angle] in &lap.around {
                let found = meet.at_angle(false, angle);
                for &t in &found {
                    let seen = meet.seen_on(false, t)[0].x;
                    assert!(
                        super::around(seen - angle).abs() < 1e-9,
                        "lap {component}, node at {node}: {t} stands at {seen}, not {angle}, in {found:?}"
                    );
                }
                let own = node.rem_euclid(TAU);
                assert!(
                    found.iter().any(|&t| {
                        let off = (t - own).rem_euclid(TAU);
                        off.min(TAU - off) < 1e-9
                    }),
                    "lap {component}: the node at {node} stands at {angle}, found {found:?}"
                );
            }
        }
    }
}

#[test]
fn at_the_angle_it_comes_back_to_at_its_seam_a_ruled_meet_is_found_at_its_seam() {
    let (pin, rod) = over_the_rod(45.0);
    for component in 0..2 {
        let meet = Meet {
            first: pin,
            second: rod,
            component,
        };
        for end in [0.0, TAU] {
            let angle = meet.seen_on(false, end)[0].x;
            let found = meet.at_angle(false, angle);
            assert!(
                found.iter().any(|&t| t.min(TAU - t) < 1e-9),
                "lap {component} stands at {angle}, read at {end}, at its seam; found {found:?}"
            );
        }
    }
}

/// Lap `component` of the pin over the rod turned `degrees`, held to cross
/// `theta` at its seam: standing within rounding of it at both ends of its
/// table, and either side of it a hair before and a hair after.
fn crossing_at_its_seam(degrees: f64, component: u8, theta: f64) -> Meet {
    let (pin, rod) = over_the_rod(degrees);
    let meet = Meet {
        first: pin,
        second: rod,
        component,
    };
    let off = |t: f64| super::around(meet.seen_on(false, t)[0].x - theta);
    for (t, name) in [(0.0, "its start"), (TAU, "its end")] {
        assert!(
            off(t).abs() < 1e-15,
            "lap {component} at {degrees}° is {} off {theta} at {name}",
            off(t)
        );
    }
    let hair = 1e-12;
    let [before, after] = [off(TAU - hair), off(hair)];
    assert!(
        (before < 0.0) != (after < 0.0) && before.abs().max(after.abs()) < 1e-9,
        "lap {component} at {degrees}° is {before} off {theta} a hair before its seam and {after} a hair after"
    );
    meet
}

fn found_at_its_seam(meet: &Meet, theta: f64) -> Vec<f64> {
    let found = meet.at_angle(false, theta);
    assert!(
        found.iter().any(|&t| t.min(TAU - t) < 1e-9),
        "the lap stands at {} at its start and {} at its end, and crosses {theta} there; found {found:?}",
        meet.seen_on(false, 0.0)[0].x,
        meet.seen_on(false, TAU)[0].x,
    );
    found
}

#[test]
fn at_an_angle_between_the_two_ends_of_its_table_a_ruled_meet_is_found_at_its_seam() {
    let theta = 6.221836942020814;
    found_at_its_seam(&crossing_at_its_seam(256.5, 1, theta), theta);
}

#[test]
fn at_an_angle_neither_end_step_of_its_table_holds_a_ruled_meet_is_found_at_its_seam() {
    let theta = 0.24746744108332255;
    found_at_its_seam(&crossing_at_its_seam(22.5, 1, theta), theta);
}

#[test]
fn a_ruled_meet_found_a_hair_before_the_end_of_its_lap_is_kept_at_its_seam() {
    let theta = 2.7058540406527594;
    found_at_its_seam(&crossing_at_its_seam(10.5, 0, theta), theta);
}
