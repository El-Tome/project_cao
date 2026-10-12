//! A line square to a wall's axis standing inside the wall's section by
//! rounding alone (#552, decision 4).
//!
//! Two planes, one square to a wall's axis and one crossing it steeply,
//! share a line square to the axis; where that line touches the wall's
//! section, the steep plane's conic touches the square plane at its vertex
//! there, and the circle the square plane cuts the wall along touches the
//! line. Found a femtometre inside the section, the line crosses it twice,
//! its roots `√(2rδ)` apart: a few tolerances, with a sliver between them
//! no face agrees on, and the corner the vertex gives stands between them.
//! An elliptic wall's section (#571) is held the same way, the line standing
//! inside it as far as its offset from the axis falls short of the wall's
//! support across it, and its rim, a conic, reaches the touch through the
//! line its plane shares with the steep one: two cuts at 45° and 75° through
//! one point of an elliptic post's rim (5571628398).
//! Inside by no more than rounding, the line only touches the wall where it
//! passes nearest the axis — on a cone's section (5543231406), on a bore cut
//! at 60° through its lowest line at the end of a tube (5544100660). The
//! three curves of the corner each reach it through [`touched`]: the line
//! and the circle as edges lying on the triple's surfaces ([`crossing`]),
//! the conic through the line its plane shares with the square one.
//!
//! Not where the second plane leans along the axis: its conic is a hairpin
//! whose vertex stands where rounding over that lean puts it, a hundred
//! tolerances along the wall, and its arms pass the square plane where the
//! line's two roots do (5549116996); `grazed.rs` decides there. Nor for a
//! line crossed on its own, with no such pair of planes: a line inside a
//! wall, however thin, crosses it twice, and real walls rely on that
//! (5549110872, 5549119078). Nor for a circle no plane of the triple
//! holds: the rim where a wall gives way to a cone, crossed by a plane
//! through the cone's apex a hair off its slope, whose hairpin on the cone
//! crosses the rim where the line's two roots do (5545108465).

use glam::{DVec2, DVec3};

use super::super::planar::planes;
use super::super::{Relation, square};
use super::Crossing;
use super::lying::{Lying, STEEP};
use crate::brep::curve::{Curve, Line};
use crate::brep::scale::Scale;
use crate::brep::surface::{Cylinder, EllipticCylinder, Plane, Surface};

/// How far inside a wall's section, as a share of the tolerance, a line
/// square to the axis stands by rounding alone.
pub(super) const ROUNDING: f64 = 1e-6;

/// Where a line two planes share, square to a wall's axis, only touches the
/// wall's section for standing inside it by rounding alone: its parameter
/// nearest the axis, where both planes cross the axis steeply. None
/// otherwise, the line then crossing the wall as any line does.
pub(super) fn touched(
    line: &Line,
    planes: [&Plane; 2],
    section: &Cylinder,
    scale: Scale,
) -> Option<f64> {
    let axis = section.axis;
    if !steep(line, planes, axis, scale) {
        return None;
    }
    let flat = |v: DVec3| v - axis * axis.dot(v);
    let (from, towards) = (flat(line.origin - section.origin), flat(line.direction));
    let closest = -from.dot(towards) / towards.length_squared();
    let inside = section.radius - (from + towards * closest).length();
    (inside > 0.0 && inside <= ROUNDING * scale.eps()).then_some(closest)
}

/// The same on an elliptic wall's section (#571): inside it by rounding
/// alone where the line's offset from the axis falls short of the wall's
/// reach across the line by no more, its parameter where it passes nearest
/// the axis in the section squashed onto the unit circle, as a line is read
/// against an elliptic wall (`elliptic.rs`).
pub(super) fn touched_on_an_ellipse(
    line: &Line,
    planes: [&Plane; 2],
    wall: &EllipticCylinder,
    scale: Scale,
) -> Option<f64> {
    if !steep(line, planes, wall.axis, scale) {
        return None;
    }
    let flat = |v: DVec3| DVec2::new(v.dot(wall.u), v.dot(wall.v));
    let (from, towards) = (flat(line.origin - wall.origin), flat(line.direction));
    let normal = towards.perp().normalize();
    let inside = (wall.major * normal.x).hypot(wall.minor * normal.y) - normal.dot(from).abs();
    let squashed = |v: DVec2| DVec2::new(v.x / wall.major, v.y / wall.minor);
    let (at, pace) = (squashed(from), squashed(towards));
    (inside > 0.0 && inside <= ROUNDING * scale.eps())
        .then(|| -at.dot(pace) / pace.length_squared())
}

/// Whether both planes cross the axis steeply and the line they share runs
/// square to it.
fn steep(line: &Line, planes: [&Plane; 2], axis: DVec3, scale: Scale) -> bool {
    !planes
        .iter()
        .any(|plane| plane.normal.dot(axis).abs() < STEEP)
        && square(line.direction, axis, scale)
}

/// The touch of a curve of the triple of two planes and a wall, crossing
/// the third of them: the line the planes share crossing the wall, or the
/// circle one plane cuts the wall along crossing the other plane, both read
/// through that line, each lying on the surfaces `lying` names. None for
/// any other curve.
pub(super) fn crossing(
    curve: &Curve,
    lying: &[Lying],
    surface: &Surface,
    scale: Scale,
) -> Option<Crossing> {
    let [one, other] = lying else {
        return None;
    };
    match (curve, one.surface, other.surface, surface) {
        (
            Curve::Line(line),
            Surface::Plane(first),
            Surface::Plane(second),
            Surface::Cylinder(wall),
        ) => {
            let at = touched(line, [first, second], wall, scale)?;
            Some(Crossing {
                parameter: at,
                point: line.point(at),
                tangent: true,
            })
        }
        (
            Curve::Line(line),
            Surface::Plane(first),
            Surface::Plane(second),
            Surface::Elliptic(wall),
        ) => {
            let at = touched_on_an_ellipse(line, [first, second], wall, scale)?;
            Some(Crossing {
                parameter: at,
                point: line.point(at),
                tangent: true,
            })
        }
        (
            Curve::Circle(circle),
            Surface::Plane(square),
            Surface::Cylinder(wall),
            Surface::Plane(steep),
        )
        | (
            Curve::Circle(circle),
            Surface::Cylinder(wall),
            Surface::Plane(square),
            Surface::Plane(steep),
        ) => {
            let Relation::Line(line) = planes(square, steep, scale) else {
                return None;
            };
            let at = touched(&line, [square, steep], wall, scale)?;
            let parameter = circle.parameter(line.point(at));
            Some(Crossing {
                parameter,
                point: circle.point(parameter),
                tangent: true,
            })
        }
        _ => None,
    }
}
