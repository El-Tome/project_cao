//! A line square to a wall's axis, shared by a plane square to it and a
//! plane crossing it steeply, standing inside the wall's section by
//! rounding alone: the three curves of the corner reach one touch.

use super::*;
use crate::brep::conic::{Conic, Round};
use crate::brep::surface::EllipticCylinder;

/// The reach of 5544100660's tube, a bore of radius 45 about Z at a few
/// hundred from the origin.
fn wide() -> Scale {
    Scale::of(400.0)
}

/// The bore of 5544100660.
fn bore() -> Cylinder {
    Cylinder::about(DVec3::new(240.0, 180.0, 0.0), DVec3::Z, 45.0)
}

/// The tube's end, square to the bore's axis.
fn end() -> Plane {
    Plane::through(DVec3::new(0.0, 0.0, 75.0), DVec3::Z).0
}

/// A cut at 60° through the bore's lowest line at the end, as 5544100660's
/// block laid it: the line it shares with the end stands 3e-14 inside the
/// bore's section, which it only touches.
fn slant() -> Plane {
    Plane {
        origin: DVec3::new(0.0, 68.77404735808354, -39.706714755449624),
        normal: DVec3::new(0.0, 0.8660254037844387, -0.5000000000000003),
        u: DVec3::X,
        v: DVec3::new(0.0, -0.5000000000000003, -0.8660254037844387),
    }
}

fn points(found: Crossings) -> Vec<(DVec3, bool)> {
    let Crossings::At(found) = found else {
        panic!("points expected, got {found:?}");
    };
    found
        .iter()
        .map(|crossing| (crossing.point, crossing.tangent))
        .collect()
}

/// The line the two planes share against the wall, as an edge lying on both
/// crosses it; the circle the first plane cuts the wall along against the
/// second, as an edge lying on both crosses it; the conic the second cuts
/// it along against the first.
fn reached(planes: [Plane; 2], wall: Cylinder, scale: Scale) -> [Vec<(DVec3, bool)>; 3] {
    let [square, steep] = planes.map(Surface::Plane);
    let round = Surface::Cylinder(wall);
    let Relation::Line(line) = relation(&square, &steep, scale) else {
        panic!("the planes cross");
    };
    let Relation::Circle(circle) = relation(&square, &round, scale) else {
        panic!("the square plane cuts the wall along a circle");
    };
    let conics = Conic::cut(&planes[1], &Round::Cylinder(wall), scale);
    let lying = [&square, &steep].map(|own| Lying {
        surface: own,
        with: relation(own, &round, scale),
    });
    [
        points(crossings_lying(
            &Curve::Line(line),
            &lying,
            &round,
            scale,
            &Touches::default(),
        )),
        points(crossings_lying(
            &Curve::Circle(circle),
            &[&square, &round].map(|own| Lying {
                surface: own,
                with: relation(own, &steep, scale),
            }),
            &steep,
            scale,
            &Touches::default(),
        )),
        conics
            .iter()
            .flat_map(|conic| points(crossings(&Curve::Conic(*conic), &square, scale)))
            .collect(),
    ]
}

#[test]
fn a_slant_through_a_bore_s_lowest_line_at_its_end_touches_its_rim_once_whichever_curve_reaches_it()
{
    let [line, circle, conic] = reached([end(), slant()], bore(), wide());
    for found in [&line, &circle, &conic] {
        assert_eq!(found.len(), 1, "{line:?} {circle:?} {conic:?}");
        assert!(found[0].1, "a touch: {found:?}");
    }
    for other in [&circle, &conic] {
        assert!(
            line[0].0.distance(other[0].0) <= 1e-12 * 400.0,
            "{line:?} {other:?}"
        );
    }
}

#[test]
fn a_line_inside_a_bore_s_section_by_rounding_still_crosses_it_twice_beside_a_plane_along_its_axis()
{
    let lean = 1e-5_f64.to_radians();
    let along = Plane::through(
        DVec3::new(240.0, 135.0000000000001, 75.0),
        DVec3::new(0.0, lean.cos(), lean.sin()),
    )
    .0;
    let [square, along] = [end(), along].map(Surface::Plane);
    let Relation::Line(line) = relation(&square, &along, wide()) else {
        panic!("the planes cross");
    };
    let round = Surface::Cylinder(bore());
    let inside = -round.distance(line.point(line.parameter(DVec3::new(240.0, 135.0, 75.0))));
    assert!(inside > 0.0 && inside < 1e-6 * wide().eps(), "{inside}");
    let lying = [&square, &along].map(|own| Lying {
        surface: own,
        with: relation(own, &round, wide()),
    });
    let found = points(crossings_lying(
        &Curve::Line(line),
        &lying,
        &round,
        wide(),
        &Touches::default(),
    ));
    assert_eq!(found.len(), 2, "{found:?}");
}

/// An elliptic bore about Z, its major half-axis of 45 turned 0.4 off X, its
/// minor of 25, at a few hundred from the origin (#571).
fn elliptic_bore() -> EllipticCylinder {
    let first = DVec3::new(0.4_f64.cos(), 0.4_f64.sin(), 0.0) * 45.0;
    EllipticCylinder::about(DVec3::new(240.0, 180.0, 0.0), DVec3::Z, first, 25.0)
}

#[test]
fn a_slant_through_an_elliptic_bore_s_lowest_line_at_its_end_touches_its_rim_once_whichever_curve_reaches_it()
 {
    let wall = elliptic_bore();
    let down = DVec3::NEG_Y;
    let reach = wall.support(down);
    let lowest = wall.origin
        + (wall.u * (wall.major * wall.major * down.dot(wall.u))
            + wall.v * (wall.minor * wall.minor * down.dot(wall.v)))
            / reach
        + DVec3::Z * 75.0;
    let (sin, cos) = 60_f64.to_radians().sin_cos();
    let slant = Plane::through(lowest + DVec3::Y * 3e-14, DVec3::new(0.0, sin, -cos)).0;
    let [square, steep] = [end(), slant].map(Surface::Plane);
    let round = Surface::Elliptic(wall);
    let Relation::Line(line) = relation(&square, &steep, wide()) else {
        panic!("the planes cross");
    };
    let inside = reach - (line.point(line.parameter(lowest)) - wall.origin).dot(down);
    assert!(inside > 0.0 && inside < 1e-6 * wide().eps(), "{inside}");
    let lying = [&square, &steep].map(|own| Lying {
        surface: own,
        with: relation(own, &round, wide()),
    });
    let through_the_line = points(crossings_lying(
        &Curve::Line(line),
        &lying,
        &round,
        wide(),
        &Touches::default(),
    ));
    let Relation::Conic(rims) = relation(&square, &round, wide()) else {
        panic!("the end cuts the wall along its section");
    };
    let along_the_rim: Vec<(DVec3, bool)> = rims
        .iter()
        .flat_map(|rim| points(crossings(&Curve::Conic(*rim), &steep, wide())))
        .collect();
    let along_the_slant: Vec<(DVec3, bool)> = Conic::cut(&slant, &Round::Elliptic(wall), wide())
        .iter()
        .flat_map(|conic| points(crossings(&Curve::Conic(*conic), &square, wide())))
        .collect();
    for found in [&through_the_line, &along_the_rim, &along_the_slant] {
        assert_eq!(
            found.len(),
            1,
            "{through_the_line:?} {along_the_rim:?} {along_the_slant:?}"
        );
        assert!(found[0].1, "a touch: {found:?}");
        assert!(
            found[0].0.distance(through_the_line[0].0) <= 1e-12 * 400.0,
            "{through_the_line:?} {found:?}"
        );
    }
}
