//! A corner of two planes and a wall, one plane a hair from tangent to the
//! wall at a slant: decided on the hairpin that plane cuts the wall along,
//! which crosses the other plane steeply, rather than where the line the
//! planes share grazes the wall, which rounding moves along that line by a
//! few tolerances — off the hairpin, and the corner off the curve it ends.

use super::*;
use crate::brep::conic::{Conic, Round};

/// The reach of a post's box: its tolerance is the one the corner is held to.
fn tight() -> Scale {
    Scale::of(6.0)
}

/// A post along Z about `(3, 4)`, of radius 1.5.
fn post() -> Surface {
    cylinder(DVec3::new(3.0, 4.0, 0.0), DVec3::Z, 1.5)
}

/// How far, in radians, the block of 5541111069 leans about X: 1e-5°.
const LEAN: f64 = 1e-5 * std::f64::consts::PI / 180.0;

/// The block's top through `(3, 4, height)`, leaning `lean` about X.
fn top(height: f64, lean: f64) -> Plane {
    Plane::through(
        DVec3::new(3.0, 4.0, height),
        DVec3::new(0.0, lean.sin(), lean.cos()),
    )
    .0
}

/// The block's side, square to its top and touching the post at height 1:
/// it cuts the wall along an ellipse whose vertex, in the box, turns on a
/// radius of `1.5 · lean` — fifty tolerances at `LEAN`.
fn side(lean: f64) -> Plane {
    Plane::through(
        DVec3::new(3.0, 4.0 + 1.5 * lean.cos(), 1.0 + 1.5 * lean.sin()),
        DVec3::new(0.0, lean.cos(), -lean.sin()),
    )
    .0
}

fn hairpin(lean: f64) -> Conic {
    let [conic] = Conic::cut(&side(lean), &round(), tight())[..] else {
        panic!("one ellipse expected");
    };
    conic
}

fn round() -> Round {
    let Surface::Cylinder(cylinder) = post() else {
        unreachable!()
    };
    Round::Cylinder(cylinder)
}

/// The corners of the top, the side and the post, as each of the three
/// curves of the triple reaches them: the line the planes share against the
/// post, as an edge of the block lying on both crosses it; the hairpin
/// against the top; the ellipse the top cuts the post along against the
/// side. Each sorted along X.
fn reached(height: f64, lean: f64) -> [Vec<DVec3>; 3] {
    let (top, side) = (
        Surface::Plane(top(height, lean)),
        Surface::Plane(side(lean)),
    );
    let Relation::Line(line) = relation(&top, &side, tight()) else {
        panic!("the top and the side cross");
    };
    let lying = [&top, &side].map(|own| Lying {
        surface: own,
        with: relation(own, &post(), tight()),
    });
    let Relation::Conic(level) = relation(&top, &post(), tight()) else {
        panic!("the top cuts the post at a slant");
    };
    [
        crossings_lying(
            &Curve::Line(line),
            &lying,
            &post(),
            tight(),
            &Touches::default(),
        ),
        crossings(&Curve::Conic(hairpin(lean)), &top, tight()),
        crossings(&Curve::Conic(level[0]), &side, tight()),
    ]
    .map(|found| {
        let Crossings::At(found) = found else {
            panic!("points expected, got {found:?}");
        };
        let mut points: Vec<DVec3> = found.iter().map(|crossing| crossing.point).collect();
        points.sort_by(|one, other| one.x.total_cmp(&other.x));
        points
    })
}

fn bits(points: &[DVec3]) -> Vec<[u64; 3]> {
    points
        .iter()
        .map(|point| point.to_array().map(f64::to_bits))
        .collect()
}

fn assert_on_the_hairpin(corners: &[DVec3], lean: f64) {
    for corner in corners {
        let off = hairpin(lean).distance(*corner);
        assert!(off <= tight().eps() / 2.0, "{corner} {off}");
    }
}

/// The corners of the triple whose top stands `below` under the hairpin's
/// vertex where the two meet, as each curve reaches them.
fn under_the_vertex(below: f64, lean: f64) -> [Vec<DVec3>; 3] {
    let vertex = hairpin(lean).anchor;
    reached(vertex.z - below + 1.5 * lean.tan(), lean)
}

#[test]
fn the_corner_of_a_hairpin_and_a_plane_across_it_stands_on_the_hairpin_whichever_curve_reaches_it()
{
    for tolerances in 5..40 {
        let below = f64::from(tolerances) * tight().eps();
        let [by_the_line, by_the_hairpin, by_the_level] = under_the_vertex(below, LEAN);
        assert_eq!(by_the_line.len(), 2, "{by_the_line:?}");
        assert_on_the_hairpin(&by_the_line, LEAN);
        assert_eq!(bits(&by_the_line), bits(&by_the_hairpin));
        assert_eq!(bits(&by_the_line), bits(&by_the_level));
    }
}

#[test]
fn a_top_a_hair_under_a_hairpin_s_vertex_is_touched_where_the_line_touches_the_post() {
    let lean = LEAN / 100.0;
    let below = 20.0 * tight().eps();
    let vertex = hairpin(lean).anchor;
    let height = vertex.z - below + 1.5 * lean.tan();
    let [by_the_line, ..] = reached(height, lean);
    let Relation::Line(line) = relation(
        &Surface::Plane(top(height, lean)),
        &Surface::Plane(side(lean)),
        tight(),
    ) else {
        panic!("the top and the side cross");
    };
    let Crossings::At(found) = crossings(&Curve::Line(line), &post(), tight()) else {
        panic!("points expected");
    };
    assert!(found.iter().all(|crossing| crossing.tangent), "{found:?}");
    let alone: Vec<DVec3> = found.iter().map(|crossing| crossing.point).collect();
    assert_eq!(bits(&by_the_line), bits(&alone));
}

/// 5542230229: a post along X of radius 5.5, a block's face leaning 4e-8°
/// off along its axis and touching the post's top a hair from the block's
/// side, square to the axis. The face's hairpin turns on a fifth of the
/// tolerance, its vertex within the tolerance of the side, which it crosses
/// at two places closer than the tolerance. The block's edge the side and
/// the face share stands a rounding inside the post and crosses it at two
/// places a dozen tolerances apart: moved onto the hairpin, the two were one
/// corner, and the circle and the conic through it were left without one.
#[test]
fn a_side_a_hair_from_a_sharp_hairpin_s_vertex_keeps_the_edge_s_two_crossings_apart() {
    let scale = Scale::of(17.0);
    let post = cylinder(DVec3::new(0.0, 6.0, 2.0), DVec3::X, 5.5);
    let side = Plane::through(
        DVec3::new(7.999999997905605, -5.585053307258781e-9, 0.0),
        DVec3::new(1.0, -6.98131663590118e-10, 1.5501636942752264e-25),
    );
    let face = Plane::through(
        DVec3::new(
            8.02851413518546e-9,
            11.500000005585052,
            -2.9183005232891327e-15,
        ),
        DVec3::new(6.98131663590118e-10, 1.0, -2.537652627714644e-16),
    );
    let [side, face] = [side, face].map(|(plane, _)| Surface::Plane(plane));
    let edge = Line {
        origin: DVec3::new(
            8.000000005934119,
            11.499999999999998,
            -2.6645352591003757e-15,
        ),
        direction: DVec3::new(0.0, 2.537652627714644e-16, 1.0),
    };
    let lying = [&side, &face].map(|own| Lying {
        surface: own,
        with: relation(own, &post, scale),
    });
    let found = crossings_lying(
        &Curve::Line(edge),
        &lying,
        &post,
        scale,
        &Touches::default(),
    );
    let Crossings::At(found) = found else {
        panic!("points expected, got {found:?}");
    };
    let [one, other] = found[..] else {
        panic!("two crossings expected, got {found:?}");
    };
    assert!(!one.tangent && !other.tangent, "{found:?}");
    assert!(one.point.distance(other.point) > scale.eps(), "{found:?}");
}
