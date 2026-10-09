//! Which side of each other a plane at a slant and a wall stand on (#552),
//! read off the relation `relation/sliced.rs` decides: until the kernel
//! hands it those pairs, the relation is given here as it decides it.

use std::f64::consts::PI;

use glam::DVec3;

use super::*;
use crate::brep::conic::{Conic, Round};
use crate::brep::curve::Line;
use crate::brep::relation::Relation;
use crate::brep::surface::{Cone, Cylinder, Plane};

const REACH: f64 = 20.0;

/// Far below the tolerance and far above rounding at this reach.
const ROUNDING: f64 = 1e-12;

/// A point of base radius ten on the XY plane, its apex at (0, 0, 20): its
/// rulings run down and out, against its axis.
fn point() -> Cone {
    Cone::through(
        DVec3::ZERO,
        DVec3::Z,
        [DVec2::new(0.0, 10.0), DVec2::new(REACH, 0.0)],
    )
}

/// A countersink: its apex at the origin, opening up along its axis.
fn countersink() -> Cone {
    Cone::through(
        DVec3::ZERO,
        DVec3::Z,
        [DVec2::new(0.0, 0.0), DVec2::new(10.0, 5.0)],
    )
}

/// The ruling of a cone at the angle `angle` about Z from X, through its
/// apex and off along its slope.
fn ruling(cone: &Cone, angle: f64) -> Line {
    let radial = DVec3::new(angle.cos(), angle.sin(), 0.0);
    Line::through(
        cone.apex(),
        cone.axis * cone.ruling.x + radial * cone.ruling.y,
    )
}

fn above(first: &Surface, second: &Surface, related: &Relation, place: DVec3) -> Option<bool> {
    wrapped::stands_above(first, second, related, place, Scale::of(REACH), ROUNDING)
}

#[test]
fn a_plane_through_a_cone_s_apex_at_a_slant_stands_on_the_side_each_lobe_gives() {
    let cone = point();
    let across = (0.4_f64).acos();
    let rulings = Relation::Rulings {
        lines: [ruling(&cone, -across), ruling(&cone, across)],
        apex: Some(cone.apex()),
    };
    let on_the_cone = |angle: f64| DVec3::new(5.0 * angle.cos(), 5.0 * angle.sin(), 10.0);
    for turned in [1.0, -1.0] {
        let normal = DVec3::new(1.0, 0.0, 0.2).normalize() * turned;
        let (plane, _) = Plane::through(cone.apex(), normal);
        let plane = Plane { normal, ..plane };
        let [conical, flat] = [Surface::Cone(cone), Surface::Plane(plane)];
        for (angle, inside) in [
            (0.0, true),
            (1.0, true),
            (-1.0, true),
            (PI, false),
            (2.0, false),
        ] {
            assert_eq!(
                above(&conical, &flat, &rulings, on_the_cone(angle)),
                Some(inside),
                "the plane runs inside the point on the lobe at {angle}, turned {turned}"
            );
        }
        let facing = turned > 0.0;
        for (place, beyond) in [
            (DVec3::new(2.0, 1.0, 10.0), true),
            (DVec3::new(2.0, -1.0, 10.0), true),
            (DVec3::new(2.0, 6.0, 10.0), false),
            (DVec3::new(1.0, -8.0, 15.0), false),
        ] {
            assert_eq!(
                above(&flat, &conical, &rulings, place),
                Some(beyond == facing),
                "the point beside {place}, the plane turned {turned}"
            );
        }
    }
}

#[test]
fn a_plane_holding_a_cone_s_axis_keeps_it_on_its_axis_side_all_round() {
    let cone = point();
    let (plane, _) = Plane::through(DVec3::ZERO, DVec3::X);
    let rulings = Relation::Rulings {
        lines: [ruling(&cone, -PI / 2.0), ruling(&cone, PI / 2.0)],
        apex: Some(cone.apex()),
    };
    let [conical, flat] = [Surface::Cone(cone), Surface::Plane(plane)];
    for angle in [0.0, 1.0, PI, -2.0] {
        let place = DVec3::new(5.0 * f64::cos(angle), 5.0 * f64::sin(angle), 10.0);
        assert_eq!(above(&conical, &flat, &rulings, place), Some(true));
        assert_eq!(above(&flat, &conical, &rulings, place), None);
    }
}

#[test]
fn a_plane_touching_a_cone_along_a_ruling_stands_outside_it_and_the_cone_on_its_axis_s_side() {
    for (cone, normal) in [
        (point(), DVec3::new(1.0, 0.0, 0.5)),
        (countersink(), DVec3::new(1.0, 0.0, -0.5)),
    ] {
        let touching = Relation::Ruling {
            line: ruling(&cone, 0.0),
            apex: Some(cone.apex()),
        };
        let on_the_axis = cone.origin + cone.axis * 10.0;
        for turned in [1.0, -1.0] {
            let (plane, _) = Plane::through(cone.apex(), normal);
            let plane = Plane {
                normal: normal.normalize() * turned,
                ..plane
            };
            let [conical, flat] = [Surface::Cone(cone), Surface::Plane(plane)];
            let place = cone.apex() + ruling(&cone, 0.3).direction * 5.0;
            assert_eq!(
                above(&conical, &flat, &touching, place),
                Some(cone.ruling.x > 0.0),
                "the plane stands outside the cone, turned {turned}"
            );
            let beside = plane.point(plane.parameters(place));
            assert_eq!(
                above(&flat, &conical, &touching, beside),
                Some(plane.distance(on_the_axis) > 0.0),
                "the cone stands on its axis's side of the plane, turned {turned}"
            );
        }
    }
}

#[test]
fn a_wall_and_a_plane_crossing_along_a_conic_are_read_at_the_place_in_either_order() {
    let wall = Cylinder::about(DVec3::ZERO, DVec3::Z, 5.0);
    let (plane, _) = Plane::through(DVec3::ZERO, DVec3::new(-0.1, 0.0, 1.0));
    let conics = Relation::Conic(Conic::cut(&plane, &Round::Cylinder(wall), Scale::of(REACH)));
    let [round, flat] = [Surface::Cylinder(wall), Surface::Plane(plane)];
    let up = plane.normal.z.signum() > 0.0;
    for (place, outside) in [
        (DVec3::new(5.0, 0.0, 1.0), true),
        (DVec3::new(-5.0, 0.0, 1.0), false),
        (DVec3::new(5.0, 0.0, -1.0), false),
        (DVec3::new(-5.0, 0.0, -1.0), true),
    ] {
        assert_eq!(
            above(&round, &flat, &conics, place),
            Some(outside),
            "the plane beside {place} on the wall"
        );
        assert_eq!(
            above(&round, &flat, &Relation::Unsupported, place),
            None,
            "a wall and a plane not decided along a conic are read as before"
        );
    }
    for (place, upper) in [
        (DVec3::new(-4.0, 0.0, -0.4), true),
        (DVec3::new(-4.0, 1.0, -0.4), true),
        (DVec3::new(4.0, 0.0, 0.4), false),
        (DVec3::new(4.0, -2.0, 0.4), false),
    ] {
        let on_the_plane = plane.point(plane.parameters(place));
        assert_eq!(
            above(&flat, &round, &conics, on_the_plane),
            Some(upper == up),
            "the wall beside {on_the_plane} on the plane"
        );
    }
}
