//! A line square to a wall's axis, shared by a plane square to it and a
//! plane crossing it steeply, standing inside the wall's section by
//! rounding alone: the three curves of the corner reach one touch.

use super::*;
use crate::brep::conic::{Conic, Round};

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
/// second; the conic the second cuts it along against the first.
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
        points(crossings(&Curve::Circle(circle), &steep, scale)),
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
