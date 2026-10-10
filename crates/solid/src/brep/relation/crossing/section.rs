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
//! on its own, the circle and the conic through the line their planes
//! share.
//!
//! Not where the second plane leans along the axis: its conic is a hairpin
//! whose vertex stands where rounding over that lean puts it, a hundred
//! tolerances along the wall, and its arms pass the square plane where the
//! line's two roots do (5549116996); `grazed.rs` decides there. Nor for a
//! line crossed on its own, with no such pair of planes: a line inside a
//! wall, however thin, crosses it twice, and real walls rely on that
//! (5549110872, 5549119078).

use glam::DVec3;

use super::super::square;
use super::lying::STEEP;
use crate::brep::curve::Line;
use crate::brep::scale::Scale;
use crate::brep::surface::{Cylinder, Plane};

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
