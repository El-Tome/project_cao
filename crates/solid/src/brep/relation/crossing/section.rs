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

use glam::DVec3;

use super::super::planar::planes;
use super::super::{Relation, square};
use super::Crossing;
use super::lying::{Lying, STEEP};
use crate::brep::curve::{Curve, Line};
use crate::brep::scale::Scale;
use crate::brep::surface::{Cylinder, Plane, Surface};

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
    if planes
        .iter()
        .any(|plane| plane.normal.dot(axis).abs() < STEEP)
        || !square(line.direction, axis, scale)
    {
        return None;
    }
    let flat = |v: DVec3| v - axis * axis.dot(v);
    let (from, towards) = (flat(line.origin - section.origin), flat(line.direction));
    let closest = -from.dot(towards) / towards.length_squared();
    let inside = section.radius - (from + towards * closest).length();
    (inside > 0.0 && inside <= ROUNDING * scale.eps()).then_some(closest)
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
