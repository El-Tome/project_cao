//! How many times an operand wraps each side of a region of a surface.

mod above;

use glam::DVec3;

use crate::brep::Declined;
use crate::brep::combine::Operands;
use crate::brep::domain::Location;
use crate::brep::relation::relation;
use crate::brep::scale::Scale;
use crate::brep::topology::{FaceId, SurfaceId};
pub(super) use above::stands_above;

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
/// normal points to, read off the pair as it was decided at `scale`.
pub(super) fn lies_above(
    operands: &Operands,
    scale: Scale,
    pair: [SurfaceId; 2],
    place: DVec3,
) -> Option<bool> {
    let [first, second] = pair.map(|surface| &operands.surfaces.list[surface.0 as usize]);
    let rounding = operands.eps() * ROUNDING;
    stands_above(
        first,
        second,
        &relation(first, second, scale),
        place,
        scale,
        rounding,
    )
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
