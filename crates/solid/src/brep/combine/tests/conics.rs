//! Which surfaces carry the conic a plane cuts a wall along at a slant
//! (#552) all along it: those decided one with its plane or its wall.

use glam::DVec2;

use super::held::carries;
use super::*;
use crate::brep::conic::{Conic, Round};
use crate::brep::surface::{Cone, Cylinder};

fn scale() -> Scale {
    Scale::of(40.0)
}

fn slant(through: DVec3) -> Plane {
    Plane::through(through, DVec3::new(0.3, 0.1, 1.0)).0
}

#[test]
fn a_plane_carries_its_conic_and_its_wall_carries_it_all_along_but_no_other_surface_does() {
    let eps = scale().eps();
    let middle = DVec3::new(1.0, 2.0, 4.0);
    let shaft = Cylinder::about(DVec3::new(1.0, 2.0, 0.0), DVec3::Z, 5.0);
    let point = Cone::through(
        DVec3::new(1.0, 2.0, 0.0),
        DVec3::Z,
        [DVec2::new(0.0, 8.0), DVec2::new(12.0, 0.0)],
    );
    for (wall, round, beside, off) in [
        (
            Surface::Cylinder(shaft),
            Round::Cylinder(shaft),
            Surface::Cylinder(Cylinder::about(shaft.origin, -DVec3::Z, 5.0 + eps / 2.0)),
            Surface::Cylinder(Cylinder::about(shaft.origin, DVec3::Z, 5.0 + 2.0 * eps)),
        ),
        (
            Surface::Cone(point),
            Round::Cone(point),
            Surface::Cone(Cone::through(
                point.origin + DVec3::X * (eps / 3.0),
                DVec3::Z,
                [DVec2::new(0.0, 8.0), DVec2::new(12.0, 0.0)],
            )),
            Surface::Cone(Cone::through(
                point.origin,
                DVec3::Z,
                [DVec2::new(0.0, 8.0), DVec2::new(12.0, 1e-6)],
            )),
        ),
    ] {
        let [conic] = Conic::cut(&slant(middle), &round, scale())[..] else {
            panic!("one conic of a slant across {round:?}");
        };
        let curve = Curve::Conic(conic);
        let level = |height: f64| Surface::Plane(slant(middle + DVec3::Z * height));
        for (surface, expected) in [
            (level(0.0), true),
            (level(eps / 2.0), true),
            (level(3.0 * eps), false),
            (wall, true),
            (beside, true),
            (off, false),
            (Surface::Plane(Plane::through(middle, DVec3::Z).0), false),
            (Surface::Plane(Plane::through(middle, DVec3::X).0), false),
            (
                Surface::Cylinder(Cylinder::about(middle, DVec3::X, 5.0)),
                false,
            ),
        ] {
            assert_eq!(
                carries(&surface, &curve, scale()),
                expected,
                "{surface:?} carrying {conic:?}"
            );
        }
    }
    let [ellipse] = Conic::cut(&slant(middle), &Round::Cylinder(shaft), scale())[..] else {
        panic!("one ellipse");
    };
    assert!(!carries(
        &Surface::Cone(point),
        &Curve::Conic(ellipse),
        scale()
    ));
}
