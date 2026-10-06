//! How many times an operand wraps each side of a region of a surface.

use std::f64::consts::TAU;

use glam::{DVec2, DVec3};

use crate::brep::Declined;
use crate::brep::combine::Operands;
use crate::brep::curve::Line;
use crate::brep::domain::Location;
use crate::brep::relation::{Relation, relation};
use crate::brep::scale::Scale;
use crate::brep::surface::{Cylinder, Surface};
use crate::brep::topology::{FaceId, SurfaceId};

/// A ray from a region's inside point is cast with this share of the
/// tolerance, where rounding alone could put a point on the wrong side of a
/// face: the arcs have decided where the operand's boundary runs across the
/// region's surface, so a point standing a hair past it, or a hair off one of
/// its faces, is taken where it stands.
const ROUNDING: f64 = 1e-6;

/// An operand's winding just on the side a surface's own normal points to,
/// and just on the other; `face`, the face of the operand lying there.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::brep) struct Wrapped {
    pub face: Option<FaceId>,
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
                face: Some(face),
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
/// two parallel walls a hair apart crossing at a grazing angle — the two
/// stand one above the other all along the stretch between the lines they
/// cross along: read at its middle, where they stand furthest apart. Kept
/// two by the operand though one at this tolerance, they are read at the
/// first twin's place, where they stand further apart than rounding; so are
/// two planes, the ends of a turn short of whole near its axis. Other twins
/// are a tie.
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
/// touch along, all along the stretch between the two lines they were
/// decided to cross along that `place` stands on, or at `place` where two
/// neither touching nor crossing along two lines — a plane and a wall, two
/// parallel walls, two planes — stand further apart than rounding; none
/// otherwise.
pub(super) fn lies_above(
    operands: &Operands,
    scale: Scale,
    pair: [SurfaceId; 2],
    place: DVec3,
) -> Option<bool> {
    let [first, second] = pair.map(|surface| &operands.surfaces.list[surface.0 as usize]);
    let decided = relation(first, second, scale);
    if let Relation::Lines(lines) = &decided {
        return across_the_lobe(first, second, lines, place);
    }
    if !matches!(decided, Relation::Tangent(_)) {
        let rounding = operands.eps() * ROUNDING;
        return match (first, second) {
            (Surface::Cylinder(first), Surface::Cylinder(second)) => {
                above_at(first, second, place, rounding)
            }
            (Surface::Plane(plane), Surface::Cylinder(cylinder)) => {
                let gap = plane.distance(beside(cylinder, place));
                (gap.abs() > rounding).then_some(gap > 0.0)
            }
            (Surface::Plane(first), Surface::Plane(second)) => {
                let facing = first.normal.dot(second.normal);
                let gap = -second.distance(place) / facing;
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

/// Whether the second of two surfaces crossing along two lines parallel to a
/// cylinder's axis stands above the first all along the stretch of that
/// cylinder between the two lines that `place` stands on: read at the
/// stretch's middle, where the two stand furthest apart. Within the band
/// about either line they stand within rounding of each other, and the side
/// changes only across a line. Of two walls, a point of the first inside
/// the second has the second's wall beyond it where the two face one way at
/// `place`, a bore crossing the wall of a bore it all but touches inside,
/// and behind it where they face each other, a pin dipping into a stock.
fn across_the_lobe(
    first: &Surface,
    second: &Surface,
    lines: &[Line; 2],
    place: DVec3,
) -> Option<bool> {
    let wall = match (first, second) {
        (Surface::Cylinder(wall), _) | (_, Surface::Cylinder(wall)) => wall,
        _ => return None,
    };
    let at = wall.parameters(place);
    let [one, other] = lines.map(|line| wall.parameters(line.origin).x);
    let span = (other - one).rem_euclid(TAU);
    let middle = if (at.x - one).rem_euclid(TAU) < span {
        one + span / 2.0
    } else {
        other + (TAU - span) / 2.0
    };
    let middle = wall.point(DVec2::new(middle, at.y));
    match (first, second) {
        (Surface::Cylinder(first), Surface::Cylinder(second)) => {
            let facing = |cylinder: &Cylinder| cylinder.radial(cylinder.parameters(place).x);
            let agree = facing(first).dot(facing(second)) > 0.0;
            Some((second.distance(middle) < 0.0) == agree)
        }
        (Surface::Plane(plane), Surface::Cylinder(_)) => Some(plane.distance(middle) > 0.0),
        (Surface::Cylinder(wall), Surface::Plane(plane)) => {
            Some(plane.distance(middle) * plane.distance(wall.origin) > 0.0)
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
        face: None,
        above: winding,
        below: winding,
    })
}
