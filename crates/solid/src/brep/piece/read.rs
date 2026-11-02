//! A run read into the piece its wall stands on, and two consecutive pieces
//! on one line or one circle read as one.

use std::f64::consts::TAU;

use glam::DVec2;

use super::{Piece, ROUNDINGS, oval};
use crate::brep::Declined;
use crate::profile::Run;

/// Every check below is written as what must hold, so that a number which
/// is not one fails it.
///
/// A straight run whose ends are one corner is read as none, as a turn lays
/// a run to no length: it keeps its number and names no wall, and the piece
/// before it ends where the piece after it starts, within the tolerance.
pub(super) fn read(from: DVec2, to: DVec2, run: Run, eps: f64) -> Result<Option<Piece>, Declined> {
    if !(from.is_finite() && to.is_finite()) {
        return Err(Declined::Profile);
    }
    match run {
        Run::Straight if from.distance(to) > eps => Ok(Some(Piece::Straight { from, to })),
        Run::Straight => Ok(None),
        Run::Round { center, turn } => {
            let radius = from.distance(center);
            let held = radius > eps && radius * ROUNDINGS * f64::EPSILON <= eps;
            let whole = radius * (turn.abs() - TAU).abs() <= eps;
            let apart = from.distance(to) > eps && turn.abs() < TAU;
            if !((whole || apart) && held && lands(from, center, turn, to, eps)) {
                return Err(Declined::Profile);
            }
            Ok(Some(Piece::Arc {
                from,
                to,
                center,
                radius,
                sweep: turn,
            }))
        }
        Run::Oval {
            center,
            first,
            second,
            turn,
        } => oval::read([from, to], center, (first, second), turn, eps).map(Some),
    }
}

/// Whether `from`, turned about `center` by `turn`, lands on `to`.
fn lands(from: DVec2, center: DVec2, turn: f64, to: DVec2, eps: f64) -> bool {
    let landed = center + DVec2::from_angle(turn).rotate(from - center);
    landed.distance(to) <= eps
}

/// One piece for two consecutive ones on the same line or circle, none when
/// they stand on different ones, and a refusal when the second turns back
/// over the first or when the whole arc misses the second's end.
pub(super) fn joined(first: &Piece, second: &Piece, eps: f64) -> Result<Option<Piece>, Declined> {
    match (*first, *second) {
        (Piece::Straight { from, to: corner }, Piece::Straight { to, .. }) => {
            let across = to - from;
            if across.length() <= eps {
                return Err(Declined::Profile);
            }
            let aside = across.perp_dot(corner - from).abs() / across.length();
            if aside > eps {
                return Ok(None);
            }
            if (corner - from).dot(to - corner) <= 0.0 {
                return Err(Declined::Profile);
            }
            Ok(Some(Piece::Straight { from, to }))
        }
        (
            Piece::Arc {
                from,
                center,
                radius,
                sweep,
                ..
            },
            Piece::Arc {
                to,
                center: other_center,
                radius: other_radius,
                sweep: other_sweep,
                ..
            },
        ) => {
            if center.distance(other_center) > eps || (radius - other_radius).abs() > eps {
                return Ok(None);
            }
            let total = sweep + other_sweep;
            if sweep * other_sweep < 0.0
                || radius * (total.abs() - TAU) > eps
                || !lands(from, center, total, to, eps)
            {
                return Err(Declined::Profile);
            }
            Ok(Some(Piece::Arc {
                from,
                to,
                center,
                radius,
                sweep: total,
            }))
        }
        (Piece::Oval { .. }, Piece::Oval { .. }) => oval::joined(first, second, eps),
        _ => Ok(None),
    }
}
