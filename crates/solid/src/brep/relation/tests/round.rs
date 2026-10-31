//! What a sphere or a torus makes with another surface before lane R of
//! #576 decides the coaxial pairs: nothing the kernel builds.

use glam::{DVec2, DVec3};

use super::{REACH, scale};
use crate::brep::curve::{Circle, Curve};
use crate::brep::relation::{Relation, Rim, at_a_slant, relation};
use crate::brep::surface::{Cone, Cylinder, Plane, Sphere, Surface, Torus};

fn ball() -> Surface {
    Surface::Sphere(Sphere::through(DVec3::ZERO, DVec3::Z, 5.0, 3.0))
}

#[test]
fn the_rims_and_the_meridians_of_a_pair_are_its_curves_and_the_poles_its_points() {
    let Surface::Sphere(sphere) = ball() else {
        unreachable!()
    };
    let rims = Relation::Rims(vec![
        Rim {
            circle: sphere.circle(5.0, 3.0),
            touching: true,
        },
        Rim {
            circle: sphere.circle(7.0, 1.0),
            touching: false,
        },
    ]);
    assert_eq!(
        rims.curves(),
        vec![
            Curve::Circle(sphere.circle(5.0, 3.0)),
            Curve::Circle(sphere.circle(7.0, 1.0)),
        ]
    );
    assert!(rims.points().is_empty());

    let great = Circle::about(sphere.middle(), DVec3::X, 3.0);
    let poles = vec![DVec3::new(0.0, 0.0, 2.0), DVec3::new(0.0, 0.0, 8.0)];
    let meridians = Relation::Meridians {
        circles: vec![great],
        poles: poles.clone(),
    };
    assert_eq!(meridians.curves(), vec![Curve::Circle(great)]);
    assert_eq!(meridians.points(), poles);
}

fn ring() -> Surface {
    Surface::Torus(Torus::through(
        DVec3::ZERO,
        DVec3::Z,
        DVec2::new(2.0, 4.0),
        1.0,
        DVec2::new(2.0, 5.0),
    ))
}

#[test]
fn every_pair_with_a_round_is_unsupported_and_none_is_at_a_slant() {
    let others = [
        Surface::Plane(Plane::through(DVec3::new(0.0, 0.0, 5.0), DVec3::Z).0),
        Surface::Plane(Plane::through(DVec3::ZERO, DVec3::X).0),
        Surface::Plane(Plane::through(DVec3::ZERO, DVec3::new(1.0, 0.0, 1.0)).0),
        Surface::Cylinder(Cylinder::about(DVec3::ZERO, DVec3::Z, 2.0)),
        Surface::Cylinder(Cylinder::about(DVec3::ZERO, DVec3::X, 2.0)),
        Surface::Cone(Cone::through(
            DVec3::ZERO,
            DVec3::Z,
            [DVec2::new(0.0, 1.0), DVec2::new(REACH, 3.0)],
        )),
        ball(),
        ring(),
    ];
    for round in [ball(), ring()] {
        for other in &others {
            assert_eq!(relation(&round, other, scale()), Relation::Unsupported);
            assert_eq!(relation(other, &round, scale()), Relation::Unsupported);
            assert!(!at_a_slant(&round, other, scale()));
            assert!(!at_a_slant(other, &round, scale()));
        }
    }
}
