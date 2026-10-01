//! How many times an operand wraps each side of a region of a surface.

use glam::DVec3;

use crate::brep::Declined;
use crate::brep::combine::Operands;
use crate::brep::domain::Location;
use crate::brep::relation::{Relation, relation};
use crate::brep::scale::Scale;
use crate::brep::surface::{Cylinder, Surface};
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
/// is read off the pair as the operand decided it, at the scale it decided
/// it at. Decided to touch: a cylinder touching a plane stands on the side
/// of it its axis is, a plane touching a cylinder outside it, a parallel
/// cylinder outside one it touches outside, and inside one larger it
/// touches inside. Decided to cross — a plane slicing a hair off a wall,
/// two parallel walls a hair apart crossing at a grazing angle, or one at
/// this tolerance though the operand kept them two — the two stand one
/// above the other all along the stretch between the lines they cross
/// along: read at the first twin's place, where they stand further apart
/// than rounding. Other twins are a tie.
pub(in crate::brep) fn collapsed(
    operands: &Operands,
    operand: usize,
    [(first, place, one), (second, _, other)]: [(SurfaceId, DVec3, Wrapped); 2],
) -> Result<i32, Declined> {
    let scale = operands.decided(operand, [first, second]);
    let above = lies_above(operands, scale, [first, second], place).ok_or(Declined::Tie)?;
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

/// Whether the second of a pair stands on the side of the first its own
/// normal points to, along the line the two were decided at `scale` to
/// touch along, or at `place` where two not decided to touch, a plane and a
/// wall or two parallel walls, stand further apart than rounding; none
/// otherwise.
fn lies_above(
    operands: &Operands,
    scale: Scale,
    pair: [SurfaceId; 2],
    place: DVec3,
) -> Option<bool> {
    let [first, second] = pair.map(|surface| &operands.surfaces.list[surface.0 as usize]);
    if !matches!(relation(first, second, scale), Relation::Tangent(_)) {
        let rounding = operands.eps() * ROUNDING;
        return match (first, second) {
            (Surface::Cylinder(first), Surface::Cylinder(second)) => {
                above_at(first, second, place, rounding)
            }
            (Surface::Plane(plane), Surface::Cylinder(cylinder)) => {
                let gap = plane.distance(beside(cylinder, place));
                (gap.abs() > rounding).then_some(gap > 0.0)
            }
            _ => None,
        };
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

/// The point of a wall nearest `place`, off its axis.
fn beside(cylinder: &Cylinder, place: DVec3) -> DVec3 {
    let from = place - cylinder.origin;
    let foot = cylinder.origin + cylinder.axis * cylinder.axis.dot(from);
    foot + (place - foot).normalize_or_zero() * cylinder.radius
}

/// Whether `second`, parallel to `first`, stands outside it at the angle of
/// `place`, where the two stand further than `rounding` apart there.
fn above_at(first: &Cylinder, second: &Cylinder, place: DVec3, rounding: f64) -> Option<bool> {
    let square = |v: DVec3| v - first.axis * first.axis.dot(v);
    if first.axis.cross(second.axis).length() > ROUNDING {
        return None;
    }
    let outward = square(place - first.origin).normalize_or_zero();
    let gap = square(second.origin - first.origin).dot(outward) + second.radius - first.radius;
    (gap.abs() > rounding).then_some(gap > 0.0)
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
