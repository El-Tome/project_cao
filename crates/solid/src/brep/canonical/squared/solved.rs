//! The one turn laying every pair near, solved one degree of freedom at a
//! time.

use std::f64::consts::{PI, TAU};

use glam::{DQuat, DVec3};

use super::Pair;
use crate::brep::Lean;

/// The largest single step of a turn, in radians: past it the leans are not
/// a hair.
const STEP: f64 = 1e-3;

/// The one turn laying every pair near, solved one degree of freedom at a
/// time: a direction held square locks the axis it turns about, the near
/// square pairs come first, the largest lean first, and each next pair is
/// laid by a turn about the locked axis; none where no turn about it can.
pub(super) fn solved(near: &[Pair], held: &[Pair], exact: f64) -> Option<DQuat> {
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
pub(super) fn angle(turn: DQuat) -> f64 {
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
