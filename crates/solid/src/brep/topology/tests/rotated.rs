//! A body turned rigidly about a point (#552, decision 11): its corners,
//! surfaces and curves turned together, each edge read again between its
//! corners on its turned curve.

use std::f64::consts::{FRAC_PI_4, TAU};

use glam::DQuat;

use super::*;
use crate::turning::{Axis, Corner, Straight, Turn};

/// A slot raised along Z: planes, walls, lines and circles.
fn slot() -> Body {
    let outline = Contour {
        corners: vec![
            DVec2::new(-10.0, -5.0),
            DVec2::new(10.0, -5.0),
            DVec2::new(10.0, 5.0),
            DVec2::new(-10.0, 5.0),
        ],
        runs: vec![
            Run::Straight,
            Run::Round {
                center: DVec2::new(10.0, 0.0),
                turn: PI,
            },
            Run::Straight,
            Run::Round {
                center: DVec2::new(-10.0, 0.0),
                turn: PI,
            },
        ],
    };
    Body::raised(&outline, &[], ground(), DVec3::Z * 10.0).expect("the slot raises")
}

/// A shaft chamfered at its end, turned three quarters of a turn about Y:
/// a cone, a wall, its end planes and the two its turn stops on.
fn chamfered() -> Body {
    let corners = [
        [0.0, 0.0],
        [8.0, 0.0],
        [8.0, 28.0],
        [6.0, 30.0],
        [0.0, 30.0],
    ];
    let straight = Straight {
        side: -1.0,
        contours: vec![
            (0..corners.len())
                .map(|run| Corner {
                    at: DVec2::from(corners[run]),
                    run: run as u32,
                })
                .collect(),
        ],
        runs: corners.len() as u32,
        last_off_the_axis: Some(4),
    };
    let turn = Turn {
        axis: Axis {
            origin: DVec2::ZERO,
            direction: DVec2::Y,
        },
        angle: 0.75 * TAU,
        resolution: 0.0,
        on_the_axis: 0.0,
    };
    Body::turned(&straight, ground(), &turn).expect("the shaft turns")
}

fn middle(body: &Body) -> DVec3 {
    let (low, high) = body.bounds().expect("a body has a box");
    (low + high) / 2.0
}

fn skew_turn(angle: f64) -> DQuat {
    DQuat::from_axis_angle(DVec3::new(1.0, 2.0, 3.0).normalize(), angle)
}

const ROUNDING: f64 = 1e-11;

#[test]
fn a_body_turned_rigidly_keeps_each_corner_on_its_surfaces_and_each_edge_between_its_corners() {
    for body in [slot(), chamfered()] {
        let pivot = middle(&body);
        let turn = skew_turn(1e-7);
        let turned = body
            .rotated(pivot, turn, DVec3::ZERO, &[], 0.0)
            .expect("a small turn is made");
        for (before, after) in body.vertices.iter().zip(&turned.vertices) {
            let placed = pivot + turn * (before.point - pivot);
            assert!(placed.distance(after.point) < ROUNDING);
            for on in &after.on {
                let off = turned.surface(*on).distance(after.point).abs();
                assert!(off < ROUNDING, "a corner stands {off} off its surface");
            }
        }
        for edge in turned.edge_ids() {
            let stretch = turned.edge(edge);
            let Some(ends) = stretch.ends else {
                continue;
            };
            for (end, at) in ends.into_iter().zip([stretch.from, stretch.to]) {
                let off = turned.point_on(edge, at).distance(turned.vertex(end).point);
                assert!(off < ROUNDING, "an edge ends {off} off its corner");
            }
            assert!(stretch.from < stretch.to, "an edge runs its own way");
        }
    }
}

#[test]
fn a_body_turned_keeps_its_volume() {
    for body in [slot(), chamfered()] {
        let turned = body
            .rotated(middle(&body), skew_turn(1e-7), DVec3::ZERO, &[], 0.0)
            .expect("a small turn is made");
        let (before, after) = (body.volume(), turned.volume());
        assert!(
            (after - before).abs() <= 1e-9 * before,
            "the volume goes from {before} to {after}"
        );
    }
}

#[test]
fn a_circle_turned_with_its_wall_reads_its_angle_as_the_wall_does() {
    for body in [slot(), chamfered()] {
        let turned = body
            .rotated(middle(&body), skew_turn(1e-7), DVec3::ZERO, &[], 0.0)
            .expect("a small turn is made");
        let walls: Vec<(DVec3, DVec3)> = turned
            .surfaces
            .iter()
            .filter_map(|surface| match surface {
                Surface::Cylinder(cylinder) => Some((cylinder.axis, cylinder.u)),
                Surface::Cone(cone) => Some((cone.axis, cone.u)),
                Surface::Plane(_) | Surface::Elliptic(_) => None,
            })
            .collect();
        for curve in &turned.curves {
            if let Curve::Circle(circle) = curve {
                assert!(
                    walls.contains(&(circle.axis, circle.u)),
                    "a circle about {} reads its angle from {}, no wall's",
                    circle.axis,
                    circle.u
                );
            }
        }
    }
}

#[test]
fn a_direction_turned_within_rounding_of_an_anchor_is_laid_on_its_bits() {
    let body = slot();
    let turned = body
        .rotated(
            middle(&body),
            skew_turn(1e-15),
            DVec3::ZERO,
            &[DVec3::Z],
            1e-12,
        )
        .expect("a hair of a turn is made");
    let normals: Vec<DVec3> = turned
        .surfaces
        .iter()
        .map(|surface| match surface {
            Surface::Plane(plane) => plane.normal,
            Surface::Cylinder(cylinder) => cylinder.axis,
            Surface::Cone(cone) => cone.axis,
            Surface::Elliptic(wall) => wall.axis,
        })
        .collect();
    assert!(normals.contains(&DVec3::Z));
    assert!(
        normals
            .iter()
            .all(|normal| *normal == DVec3::Z || normal.z.abs() < 1e-12)
    );
}

#[test]
fn a_turn_that_would_turn_a_canonical_direction_round_is_refused() {
    let diamond = Contour::straight(vec![
        DVec2::new(5.0, 0.0),
        DVec2::new(0.0, 5.0),
        DVec2::new(-5.0, 0.0),
        DVec2::new(0.0, -5.0),
    ]);
    let body = Body::raised(&diamond, &[], ground(), DVec3::Z * 10.0).expect("it raises");
    let refused: Vec<bool> = [1e-6, -1e-6]
        .map(|angle| {
            body.rotated(
                middle(&body),
                DQuat::from_rotation_z(angle),
                DVec3::ZERO,
                &[],
                0.0,
            )
            .is_none()
        })
        .to_vec();
    assert_eq!(
        refused.iter().filter(|refused| **refused).count(),
        1,
        "a side at {FRAC_PI_4} turned past it one way and not the other: {refused:?}"
    );
}
