//! Decision 11 (#552): a plane of one operand leaning a hair off square to,
//! or along, the axis of a cylinder or a cone of the other, their faces in
//! boxes that meet, is laid exactly so. The conic such a plane cuts divides
//! by the lean, and at a ten-millionth of a degree it stands within the
//! tolerance of the rim or the lines beside it: the kernel answered it both
//! ways, in different places.
//!
//! The second operand is turned about the middle of its box, rigidly, with
//! its surfaces, curves and corners, so that its own decisions hold as it
//! took them; decided once, before decision 8 reads it. The turn is made
//! only where it moves no point of the operand further than `Scale::HAIR`
//! tolerances — the furthest the kernel already moves an operand, and what
//! it already takes for one place for two walls of one radius — where one
//! turn lays every such pair exactly, keeps every pair of directions that
//! was exact, and changes the kind of no exact relation the operand had
//! with the first — a face lying on the first's, a wall touching a plane, a
//! plane holding a cone's axis — nor lays two skew walls square, nor a
//! plane a hair beside a cone's apex. Otherwise nothing turns.

use std::f64::consts::{PI, TAU};

use glam::{DQuat, DVec3};

use crate::brep::relation::{Relation, relation};
use crate::brep::scale::Scale;
use crate::brep::surface::Surface;
use crate::brep::topology::{Body, SurfaceId};

/// Under this share of the tolerance, a pair stands exactly as it is.
const ROUNDING: f64 = 1e-6;

/// The largest single step of a turn, in radians: past it the leans are not
/// a hair.
const STEP: f64 = 1e-3;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Lean {
    /// The second's direction off along the first's, either way.
    Square,
    /// The second's direction off square to the first's.
    Along,
}

/// A direction of the first operand, one of the second's, how the second
/// leans off it and by what sine.
#[derive(Clone, Copy, Debug)]
struct Pair {
    first: DVec3,
    second: DVec3,
    lean: Lean,
    sine: f64,
}

impl Pair {
    fn of(first: DVec3, second: DVec3) -> Pair {
        let square = first.cross(second).length();
        let along = first.dot(second).abs();
        let (lean, sine) = if square <= along {
            (Lean::Square, square)
        } else {
            (Lean::Along, along)
        };
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
            Lean::Square => self.first.cross(second).length(),
            Lean::Along => self.first.dot(second).abs(),
        }
    }
}

/// The second operand turned so that every plane and wall a hair off square
/// or along stands exactly so; none when no such pair is found, or no turn
/// within the hair lays them all and keeps what was exact.
pub(in crate::brep) fn squared(first: &Body, second: &Body, scale: Scale) -> Option<Body> {
    let (low, high) = second.bounds()?;
    let lever = ((high - low).length() / 2.0).max(f64::MIN_POSITIVE);
    let eps = scale.eps();
    let exact = ROUNDING * eps / lever;
    let facing = facing(first, second, eps);
    let (near, held) = paired(first, second, &facing, exact, Scale::HAIR * eps / lever);
    if near.is_empty() {
        return None;
    }
    let turn = solved(&near, &held, exact)?;
    let kept_exact = near
        .iter()
        .chain(&held)
        .all(|pair| pair.off(turn) <= 10.0 * exact);
    if !kept_exact || angle(turn) * lever > Scale::HAIR * eps {
        return None;
    }
    let anchors: Vec<DVec3> = first.surfaces.iter().map(direction).collect();
    let turned = second.rotated((low + high) / 2.0, turn, &anchors, exact)?;
    kept(first, second, &turned, scale).then_some(turned)
}

/// The pairs of a direction of each operand whose faces stand in boxes that
/// meet: those of a plane and a wall leaning by a sine between `exact` and
/// `hair`, and those of any two surfaces standing exactly square or along.
fn paired(
    first: &Body,
    second: &Body,
    facing: &[(SurfaceId, SurfaceId)],
    exact: f64,
    hair: f64,
) -> (Vec<Pair>, Vec<Pair>) {
    let (mut near, mut held) = (Vec::new(), Vec::new());
    for &(one, other) in facing {
        let [surface, tool] = [first.surface(one), second.surface(other)];
        let pair = Pair::of(direction(surface), direction(tool));
        let walled = matches!(surface, Surface::Plane(_)) != matches!(tool, Surface::Plane(_));
        if pair.sine <= exact {
            held.push(pair);
        } else if walled && pair.sine <= hair {
            near.push(pair);
        }
    }
    (near, held)
}

/// The one turn laying every pair near, solved one degree of freedom at a
/// time: a direction held square locks the axis it turns about, the near
/// square pairs come first, the largest lean first, and each next pair is
/// laid by a turn about the locked axis; none where no turn about it can.
fn solved(near: &[Pair], held: &[Pair], exact: f64) -> Option<DQuat> {
    let mut locked: Option<DVec3> = None;
    for pair in held.iter().filter(|pair| pair.lean == Lean::Square) {
        match locked {
            None => locked = Some(pair.first),
            Some(axis) if axis.cross(pair.first).length() <= exact => {}
            Some(_) => return None,
        }
    }
    let mut ordered = near.to_vec();
    ordered.sort_by(|one, other| {
        (one.lean == Lean::Along)
            .cmp(&(other.lean == Lean::Along))
            .then(other.sine.total_cmp(&one.sine))
    });
    let mut turn = DQuat::IDENTITY;
    for pair in &ordered {
        if pair.off(turn) <= exact {
            continue;
        }
        let now = turn * pair.second;
        let step = match (pair.lean, locked) {
            (Lean::Square, None) => {
                locked = Some(pair.first);
                arc(now, pair.first * pair.first.dot(now).signum())
            }
            (Lean::Along, None) => {
                let laid = (now - pair.first * pair.first.dot(now)).normalize();
                locked = Some(laid);
                arc(now, laid)
            }
            (_, Some(axis)) => about(axis, now, pair)?,
        };
        if angle(step) >= STEP {
            return None;
        }
        turn = (step * turn).normalize();
    }
    Some(turn)
}

/// The turn taking `from` to `to`, both of unit length. Not
/// `DQuat::from_rotation_arc`, which answers none for every turn under a
/// few hundredths of a microradian: every turn this decision makes.
fn arc(from: DVec3, to: DVec3) -> DQuat {
    let across = from.cross(to);
    DQuat::from_xyzw(across.x, across.y, across.z, 1.0 + from.dot(to)).normalize()
}

/// The angle a turn turns by. Not `DQuat::to_axis_angle`, which answers
/// none under a few hundredths of a microradian.
fn angle(turn: DQuat) -> f64 {
    2.0 * turn.xyz().length().atan2(turn.w.abs())
}

/// The least turn about `axis` that lays `now` square or along the pair's
/// first direction; none where no turn about it can.
fn about(axis: DVec3, now: DVec3, pair: &Pair) -> Option<DQuat> {
    let along = axis * axis.dot(now);
    let across = now - along;
    if across.length() < 1e-12 {
        return None;
    }
    let side = axis.cross(across);
    let target = pair.first;
    let angle = match pair.lean {
        Lean::Along => {
            let (a, b, c) = (across.dot(target), side.dot(target), along.dot(target));
            let reach = a.hypot(b);
            if reach <= c.abs() {
                return None;
            }
            let base = b.atan2(a);
            let spread = (-c / reach).acos();
            [base + spread, base - spread]
                .map(|angle| (angle + PI).rem_euclid(TAU) - PI)
                .into_iter()
                .min_by(|one, other| one.abs().total_cmp(&other.abs()))?
        }
        Lean::Square => {
            let target = target * target.dot(now).signum();
            if (axis.dot(target) - axis.dot(now)).abs() > 1e-12 {
                return None;
            }
            let to = target - axis * axis.dot(target);
            across.cross(to).dot(axis).atan2(across.dot(to))
        }
    };
    Some(DQuat::from_axis_angle(axis, angle))
}

/// The direction a surface leans by: a plane's normal, a wall's axis.
fn direction(surface: &Surface) -> DVec3 {
    match surface {
        Surface::Plane(plane) => plane.normal,
        Surface::Cylinder(cylinder) => cylinder.axis,
        Surface::Cone(cone) => cone.axis,
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

/// Whether every surface of the second operand stands with each of the
/// first's whose faces it meets as it stood before the turn, where it stood
/// exactly: one with it, touching it, along a ruling or at an apex of it, or
/// holding its cone's axis. A tool face lying on the body's, turned a hair
/// off it, or a plane holding a cone's axis turned into a hyperbola a hair
/// from the apex, would be what the turn is made against. Nor is a plane
/// laid a hair beside a cone's apex, but only exactly through it: the plane
/// through a point's apex a hair off its axis, turned about the middle of
/// its box along the axis, stood a hair beside the apex and was cut along
/// two rulings it does not hold (5543232861). Nor are two walls laid
/// square that stood skew: the rule is a plane's, and a turn about an axis
/// leaning 1e-7°, laid square to another turn's end, met that turn's wall
/// along the curve of two perpendicular cylinders (5543615040). A face laid
/// on the body's, a wall laid touching or parallel to another: these are
/// what the turn is for, and refused, they left a tool at its lean beside
/// another drawn at that lean and turned.
pub(super) fn kept(first: &Body, before: &Body, after: &Body, scale: Scale) -> bool {
    let apex = |relation: &Relation| {
        matches!(
            relation,
            Relation::Ruling { .. } | Relation::Apex(_) | Relation::Rulings { .. }
        )
    };
    facing(first, before, scale.eps())
        .into_iter()
        .all(|(one, other)| {
            let (known, tool) = (first.surface(one), after.surface(other));
            let was = relation(known, before.surface(other), scale);
            let is = relation(known, tool, scale);
            let exact = apex(&was) || matches!(was, Relation::Same { .. } | Relation::Tangent(_));
            if exact {
                std::mem::discriminant(&was) == std::mem::discriminant(&is)
            } else if let Relation::Meet(_) = is {
                matches!(was, Relation::Meet(_))
            } else {
                !apex(&is) || through_the_apex(known, tool, scale)
            }
        })
}

/// Whether a plane passes through a cone's apex but for rounding.
fn through_the_apex(one: &Surface, other: &Surface, scale: Scale) -> bool {
    match (one, other) {
        (Surface::Cone(cone), Surface::Plane(plane))
        | (Surface::Plane(plane), Surface::Cone(cone)) => {
            plane.distance(cone.apex()).abs() <= 10.0 * ROUNDING * scale.eps()
        }
        _ => false,
    }
}
