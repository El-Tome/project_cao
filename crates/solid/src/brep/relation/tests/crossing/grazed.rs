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
        assert!(off <= tight().eps() / 10.0, "{corner} {off}");
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
fn a_top_under_a_hairpin_s_vertex_by_less_than_the_line_can_tell_is_crossed_where_the_hairpin_says()
{
    let lean = LEAN / 100.0;
    let below = 20.0 * tight().eps();
    let [by_the_line, by_the_hairpin, _] = under_the_vertex(below, lean);
    assert_eq!(by_the_line.len(), 2, "{by_the_line:?}");
    assert_on_the_hairpin(&by_the_line, lean);
    let apart = 2.0 * (2.0 * 1.5 * lean * below).sqrt();
    assert!(
        ((by_the_line[1].x - by_the_line[0].x) - apart).abs() <= 1e-3 * apart,
        "{by_the_line:?} {apart}"
    );
    assert_eq!(bits(&by_the_line), bits(&by_the_hairpin));
}
