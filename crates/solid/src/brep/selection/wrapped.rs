//! How many times an operand wraps each side of a region of a surface.

use glam::DVec3;

use crate::brep::Declined;
use crate::brep::combine::Operands;
use crate::brep::domain::Location;
use crate::brep::relation::{Relation, relation};
use crate::brep::surface::Surface;
use crate::brep::topology::SurfaceId;

/// A ray from a region's inside point is cast with this share of the
/// tolerance, where rounding alone could put a point on the wrong side of a
/// face: the arcs have decided where the operand's boundary runs across the
/// region's surface, so a point standing a hair past it, or a hair off one of
/// its faces, is taken where it stands.
const ROUNDING: f64 = 1e-6;

/// An operand's winding just on the side a surface's own normal points to,
/// and just on the other; `covered` when a face of the operand lies there.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::brep) struct Wrapped {
    pub covered: bool,
    pub above: i32,
    pub below: i32,
}

impl Wrapped {
    /// The same, seen from a surface whose normal points the other way.
    pub fn turned(self) -> Wrapped {
        Wrapped {
            above: self.below,
            below: self.above,
            ..self
        }
    }
}

/// Whether a face of the operand on `surface` covers `point`, and which: its
/// matter on the side the surface's normal points to or on the other.
///
/// `point` stands inside a region of `surface`, away from its boundary, so
/// from every face of the operand on that surface too, whose boundary the
/// region's arcs already hold: which faces cover it is asked by parity alone,
/// however thin the region, and a point exactly on the boundary of one is a
/// tie the kernel does not settle.
pub(in crate::brep) fn covering(
    operands: &Operands,
    operand: usize,
    surface: SurfaceId,
    point: DVec3,
) -> Result<Option<Wrapped>, Declined> {
    let located = operands.located(operand, surface, point, 0.0)?;
    if located
        .iter()
        .any(|(_, location)| *location == Location::Boundary)
    {
        return Err(Declined::Tie);
    }
    let mut covering = located
        .iter()
        .filter(|(_, location)| *location == Location::Inside);
    match (covering.next(), covering.next()) {
        (Some(&(face, _)), None) => {
            let flipped = operands.flipped(operand, face);
            Ok(Some(Wrapped {
                covered: true,
                above: i32::from(flipped),
                below: i32::from(!flipped),
            }))
        }
        (None, _) => Ok(None),
        (Some(_), Some(_)) => Err(Declined::Tie),
    }
}

/// How an operand covering two twins wraps both sides of the one piece of
/// surface they are, each covering read in the first twin's frame: its two
/// faces turning their matter towards each other hold a skin thinner than
/// the tolerance, nothing on either side once it is gone; turning it away
/// from each other, a crack as thin, matter on both. Which way is towards
/// is read off the pair's touch: a cylinder touching a plane stands on the
/// side of it its axis is, a plane touching a cylinder outside it, a
/// parallel cylinder outside one it touches outside, and inside one larger
/// it touches inside. Twins of a pair not decided to touch are a tie.
pub(in crate::brep) fn collapsed(
    operands: &Operands,
    [(first, one), (second, other)]: [(SurfaceId, Wrapped); 2],
) -> Result<i32, Declined> {
    let above = lies_above(operands, first, second).ok_or(Declined::Tie)?;
    let (towards, away) = if above {
        ((one.above, other.below), (one.below, other.above))
    } else {
        ((one.below, other.above), (one.above, other.below))
    };
    match (towards, away) {
        ((1, 1), (0, 0)) => Ok(0),
        ((0, 0), (1, 1)) => Ok(1),
        _ => Err(Declined::Tie),
    }
}

/// Whether `other` stands on the side of `one` its own normal points to,
/// along the line the two were decided to touch along; none where they
/// were not.
fn lies_above(operands: &Operands, one: SurfaceId, other: SurfaceId) -> Option<bool> {
    let [first, second] = [one, other].map(|surface| &operands.surfaces.list[surface.0 as usize]);
    if !matches!(
        relation(first, second, operands.scale),
        Relation::Tangent(_)
    ) {
        return None;
    }
    match (first, second) {
        (Surface::Plane(plane), Surface::Cylinder(cylinder)) => {
            Some(plane.distance(cylinder.origin) > 0.0)
        }
        (Surface::Cylinder(_), Surface::Plane(_)) => Some(true),
        (Surface::Cylinder(first), Surface::Cylinder(second)) => {
            let between = second.origin - first.origin;
            let across = (between - first.axis * first.axis.dot(between)).length();
            let outside = (across - (first.radius + second.radius)).abs();
            let inside = (across - (first.radius - second.radius).abs()).abs();
            Some(outside <= inside || second.radius > first.radius)
        }
        (Surface::Plane(_), Surface::Plane(_)) => None,
    }
}

/// How many times an operand no face of which covers `point` wraps it, on
/// both sides alike, as a ray cast through the operand counts. Asked only
/// where the other operand covers the point: a point of a region neither
/// covers may stand on a face of either crossing the surface there, and
/// nothing hangs on it.
pub(in crate::brep) fn wound(
    operands: &Operands,
    operand: usize,
    point: DVec3,
) -> Result<Wrapped, Declined> {
    let winding = operands.bodies[operand].winding(point, operands.eps() * ROUNDING)?;
    Ok(Wrapped {
        covered: false,
        above: winding,
        below: winding,
    })
}
