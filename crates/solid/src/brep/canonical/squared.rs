//! Decision 11 (#552): a plane of one operand leaning a hair off square to,
//! or along, the axis of a cylinder, a cone or an elliptic wall (#571) of
//! the other, their faces in boxes that meet, is laid exactly so. The conic
//! such a plane cuts divides by the lean, and at a ten-millionth of a
//! degree it stands within the tolerance of the rim or the lines beside
//! it: the kernel answered it both ways, in different places. So is a plane
//! leaning a hair off parallel to a plane of the other (#644), whose line
//! the two fix only to rounding over the lean, and whose corners stand on
//! either side of the tolerance.
//!
//! The second operand is turned rigidly, with its surfaces, curves and
//! corners, so that its own decisions hold as it took them; decided once,
//! before decision 8 reads it. An elliptic wall turns its axis and its
//! half-axes together, and one turned within rounding of an elliptic wall
//! of the first is laid on its major half-axis too, so that decision 1
//! compares the two exactly. It is turned about the middle of where the
//! two operands' boxes overlap, and set down where it lays exactly what it
//! held against the first (`placed.rs`); about the middle of its own box
//! where, turned about where they meet, it would move past the hair: a cut
//! reaching far past a turn, drawn on its slit's face, was left as drawn
//! and refused (5547227649). The turn is made only where it
//! moves no point of the operand further than `Scale::HAIR`
//! tolerances — the furthest the kernel already moves an operand, and what
//! it already takes for one place for two walls of one radius — where one
//! turn lays every such pair exactly, keeps every pair of directions that
//! was exact, and changes the kind of no exact relation the operand had
//! with the first — a face lying on the first's, a wall touching a plane, a
//! plane holding a cone's axis — nor lays a plane a hair beside a cone's
//! apex. Otherwise nothing turns.

mod kept;
mod solved;

use glam::{DQuat, DVec3};

use super::placed::{Laying, placed};
use crate::brep::scale::Scale;
use crate::brep::surface::Surface;
use crate::brep::topology::{Body, SurfaceId};
pub(super) use kept::kept;
use solved::{angle, solved};

/// Under this share of the tolerance, a pair stands exactly as it is.
const ROUNDING: f64 = 1e-6;

/// A direction of the first operand, one of the second's, how the second
/// leans off it and by what sine: `Square` off along the first's, either
/// way, `Along` off square to it — for a plane's normal and a wall's axis,
/// the plane nearly square to the axis, or nearly along it.
#[derive(Clone, Copy, Debug)]
struct Pair {
    first: DVec3,
    second: DVec3,
    lean: Off,
    sine: f64,
}

/// Which way one direction leans off another: nearly parallel to it, or
/// nearly square to it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::brep) enum Off {
    Parallel,
    Square,
}

/// How one direction leans off another, and by what sine.
pub(in crate::brep) fn leaning(first: DVec3, second: DVec3) -> (Off, f64) {
    let parallel = first.cross(second).length();
    let square = first.dot(second).abs();
    if parallel <= square {
        (Off::Parallel, parallel)
    } else {
        (Off::Square, square)
    }
}

/// Whether a lean of `sine` moves no point of the second operand further
/// than rounding: what this decision leaves exact, held as it was drawn or
/// laid by the turn. `None` for an operand with no box.
pub(in crate::brep) fn laid_exactly(second: &Body, scale: Scale) -> Option<impl Fn(f64) -> bool> {
    let (low, high) = second.bounds()?;
    let exact = exact(scale, lever(low, high));
    Some(move |sine: f64| sine <= LAID * exact)
}

/// Half the diagonal of a box: the furthest a turn about its middle moves
/// a point of it, per radian.
fn lever(low: DVec3, high: DVec3) -> f64 {
    ((high - low).length() / 2.0).max(f64::MIN_POSITIVE)
}

/// The sine under which a pair stands exactly as it is, for an operand of
/// that lever.
fn exact(scale: Scale, lever: f64) -> f64 {
    ROUNDING * scale.eps() / lever
}

/// How many times `exact` a pair the turn laid may still lean by.
const LAID: f64 = 10.0;

impl Pair {
    fn of(first: DVec3, second: DVec3) -> Pair {
        let (lean, sine) = leaning(first, second);
        Pair {
            first,
            second,
            lean,
            sine,
        }
    }

    /// The sine of the lean once the second's direction is turned.
    fn off(&self, turn: DQuat) -> f64 {
        let second = turn * self.second;
        match self.lean {
            Off::Parallel => self.first.cross(second).length(),
            Off::Square => self.first.dot(second).abs(),
        }
    }
}

/// The second operand turned so that every plane and wall a hair off square
/// or along stands exactly so; none when no such pair is found, or no turn
/// within the hair lays them all and keeps what was exact.
pub(in crate::brep) fn squared(first: &Body, second: &Body, scale: Scale) -> Option<Body> {
    turned(first, second, scale, true).or_else(|| turned(first, second, scale, false))
}

/// The second operand turned as [`squared`] says, two planes a hair off
/// parallel laid among the pairs where `abreast` (#644), none otherwise:
/// where no one turn lays them with the planes and walls of the rest, the
/// turn is sought again without them, so that a plane and a wall #552 laid
/// are laid still.
fn turned(first: &Body, second: &Body, scale: Scale, abreast: bool) -> Option<Body> {
    let (low, high) = second.bounds()?;
    let lever = lever(low, high);
    let eps = scale.eps();
    let exact = exact(scale, lever);
    let facing = facing(first, second, eps);
    let hair = Scale::HAIR * eps / lever;
    let (near, held, planes) = paired(first, second, &facing, exact, hair, abreast);
    if near.is_empty() || abreast && planes == 0 {
        return None;
    }
    let turn = solved(&near, &held, exact)?;
    let kept_exact = near
        .iter()
        .chain(&held)
        .all(|pair| pair.off(turn) <= LAID * exact);
    if !kept_exact {
        return None;
    }
    let sweep = |about: DVec3| angle(turn) * (about - low).abs().max((high - about).abs()).length();
    let met = [met(first, low, high), (low + high) / 2.0]
        .into_iter()
        .find(|about| sweep(*about) <= Scale::HAIR * eps)?;
    let swept = sweep(met);
    let laying = Laying {
        turn,
        met,
        within: LAID * exact,
        eps,
        left: Scale::HAIR * eps - swept,
    };
    let shift = placed(first, second, &laying);
    let anchors: Vec<DVec3> = first
        .surfaces
        .iter()
        .map(direction)
        .chain(first.surfaces.iter().filter_map(major_half_axis))
        .collect();
    let turned = second.rotated(met, turn, shift, &anchors, exact)?;
    kept(first, second, &turned, scale).then_some(turned)
}

/// The middle of where the boxes of the first operand and of the second,
/// from `low` to `high`, overlap: where the two meet, and what the turn
/// moves least.
fn met(first: &Body, low: DVec3, high: DVec3) -> DVec3 {
    let (one, other) = first.bounds().unwrap_or((low, high));
    (low.max(one) + high.min(other)) / 2.0
}

/// The pairs of a direction of each operand whose faces stand in boxes that
/// meet: those of a plane and a wall, or of two planes off parallel where
/// `abreast`, leaning by a sine between `exact` and `hair`, and those of
/// any two surfaces standing exactly square or along; and how many of the
/// first are two planes.
fn paired(
    first: &Body,
    second: &Body,
    facing: &[(SurfaceId, SurfaceId)],
    exact: f64,
    hair: f64,
    abreast: bool,
) -> (Vec<Pair>, Vec<Pair>, usize) {
    let (mut near, mut held, mut planes) = (Vec::new(), Vec::new(), 0);
    for &(one, other) in facing {
        let [surface, tool] = [first.surface(one), second.surface(other)];
        let pair = Pair::of(direction(surface), direction(tool));
        let walled = matches!(surface, Surface::Plane(_)) != matches!(tool, Surface::Plane(_))
            || ruled(surface) && ruled(tool);
        let parallel = abreast
            && matches!((surface, tool), (Surface::Plane(_), Surface::Plane(_)))
            && matches!(pair.lean, Off::Parallel);
        if pair.sine <= exact {
            held.push(pair);
        } else if (walled || parallel) && pair.sine <= hair {
            planes += usize::from(parallel);
            near.push(pair);
        }
    }
    (near, held, planes)
}

/// Whether a surface is a wall whose rulings run along or through its axis:
/// two such walls a hair off square or parallel are laid so (#577).
fn ruled(surface: &Surface) -> bool {
    matches!(
        surface,
        Surface::Cylinder(_) | Surface::Elliptic(_) | Surface::Cone(_)
    )
}

/// The direction a surface leans by: a plane's normal, a wall's axis.
fn direction(surface: &Surface) -> DVec3 {
    match surface {
        Surface::Plane(plane) => plane.normal,
        Surface::Cylinder(cylinder) => cylinder.axis,
        Surface::Cone(cone) => cone.axis,
        Surface::Elliptic(wall) => wall.axis,
        Surface::Sphere(sphere) => sphere.axis,
        Surface::Torus(torus) => torus.axis,
    }
}

/// The direction of an elliptic wall's major half-axis, which a direction of
/// the second operand turned within rounding of it is laid on as on any of
/// the first's: an elliptic wall's major half-axis among them, so that the
/// two compare exactly as one ellipse. None for any other surface.
fn major_half_axis(surface: &Surface) -> Option<DVec3> {
    match surface {
        Surface::Elliptic(wall) => Some(wall.u),
        Surface::Plane(_)
        | Surface::Cylinder(_)
        | Surface::Cone(_)
        | Surface::Sphere(_)
        | Surface::Torus(_) => None,
    }
}

/// The pairs of a surface of each operand on which faces stand in boxes
/// that meet, in a fixed order.
fn facing(first: &Body, second: &Body, eps: f64) -> Vec<(SurfaceId, SurfaceId)> {
    let boxes = |body: &Body| {
        let mut boxes = vec![Vec::new(); body.surfaces.len()];
        for face in body.face_ids() {
            boxes[body.face(face).surface.0 as usize].push(body.boxed(face, eps));
        }
        boxes
    };
    let (ones, others) = (boxes(first), boxes(second));
    let meet = |one: &[DVec3; 2], other: &[DVec3; 2]| {
        one[0].cmple(other[1]).all() && other[0].cmple(one[1]).all()
    };
    let mut pairs = Vec::new();
    for (one, one_boxes) in ones.iter().enumerate() {
        for (other, other_boxes) in others.iter().enumerate() {
            if one_boxes
                .iter()
                .any(|a| other_boxes.iter().any(|b| meet(a, b)))
            {
                pairs.push((SurfaceId(one as u32), SurfaceId(other as u32)));
            }
        }
    }
    pairs
}
