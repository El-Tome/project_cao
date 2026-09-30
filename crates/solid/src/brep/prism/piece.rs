//! A contour read into the pieces its walls stand on: one per line and one per
//! circle, so that two runs on one line make one wall and a whole circle made
//! of two halves makes a ring.

use std::f64::consts::TAU;

use glam::DVec2;

use crate::brep::Declined;
use crate::profile::{Contour, Run};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum Piece {
    Straight {
        from: DVec2,
        to: DVec2,
    },
    /// `from` turned about `center` by `sweep` lands on `to`; a ring when
    /// the sweep is a whole turn and the two are one corner.
    Arc {
        from: DVec2,
        to: DVec2,
        center: DVec2,
        radius: f64,
        sweep: f64,
    },
}

impl Piece {
    pub(super) fn from(&self) -> DVec2 {
        match *self {
            Piece::Straight { from, .. } | Piece::Arc { from, .. } => from,
        }
    }

    pub(super) fn to(&self) -> DVec2 {
        match *self {
            Piece::Straight { to, .. } | Piece::Arc { to, .. } => to,
        }
    }

    pub(super) fn is_ring(&self) -> bool {
        matches!(*self, Piece::Arc { sweep, .. } if sweep.abs() == TAU)
    }

    fn reversed(&self) -> Piece {
        match *self {
            Piece::Straight { from, to } => Piece::Straight { from: to, to: from },
            Piece::Arc {
                from,
                to,
                center,
                radius,
                sweep,
            } => Piece::Arc {
                from: to,
                to: from,
                center,
                radius,
                sweep: -sweep,
            },
        }
    }

    /// The same piece seen in a mirror across the first axis.
    pub(super) fn mirrored(&self) -> Piece {
        let flip = |point: DVec2| DVec2::new(point.x, -point.y);
        match *self {
            Piece::Straight { from, to } => Piece::Straight {
                from: flip(from),
                to: flip(to),
            },
            Piece::Arc {
                from,
                to,
                center,
                radius,
                sweep,
            } => Piece::Arc {
                from: flip(from),
                to: flip(to),
                center: flip(center),
                radius,
                sweep: -sweep,
            },
        }
    }
}

/// The pieces of a contour, runs on one line or one circle merged.
pub(super) fn pieces(contour: &Contour, eps: f64) -> Result<Vec<Piece>, Declined> {
    let count = contour.corners.len();
    if count == 0 || contour.runs.len() != count {
        return Err(Declined::Profile);
    }
    let mut merged: Vec<Piece> = Vec::new();
    for index in 0..count {
        let piece = read(
            contour.corners[index],
            contour.corners[(index + 1) % count],
            contour.runs[index],
            eps,
        )?;
        let joined = match merged.last() {
            Some(last) => joined(last, &piece, eps)?,
            None => None,
        };
        match (joined, merged.last_mut()) {
            (Some(joined), Some(last)) => *last = joined,
            _ => merged.push(piece),
        }
    }
    while merged.len() > 1 {
        let last = merged[merged.len() - 1];
        let Some(joined) = joined(&last, &merged[0], eps)? else {
            break;
        };
        merged[0] = joined;
        merged.pop();
    }
    closed(merged, eps)
}

/// Every check below is written as what must hold, so that a number which
/// is not one fails it.
fn read(from: DVec2, to: DVec2, run: Run, eps: f64) -> Result<Piece, Declined> {
    if !(from.is_finite() && to.is_finite() && from.distance(to) > eps) {
        return Err(Declined::Profile);
    }
    match run {
        Run::Straight => Ok(Piece::Straight { from, to }),
        Run::Round { center, turn } => {
            let radius = from.distance(center);
            if !(turn.abs() < TAU && radius > eps && lands(from, center, turn, to, eps)) {
                return Err(Declined::Profile);
            }
            Ok(Piece::Arc {
                from,
                to,
                center,
                radius,
                sweep: turn,
            })
        }
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
fn joined(first: &Piece, second: &Piece, eps: f64) -> Result<Option<Piece>, Declined> {
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
        _ => Ok(None),
    }
}

/// The merged pieces, a lone circle made a ring of exactly one turn.
fn closed(mut pieces: Vec<Piece>, eps: f64) -> Result<Vec<Piece>, Declined> {
    if let [Piece::Arc { radius, sweep, .. }] = pieces.as_mut_slice() {
        if *radius * (sweep.abs() - TAU).abs() > eps {
            return Err(Declined::Profile);
        }
        *sweep = TAU.copysign(*sweep);
        return Ok(pieces);
    }
    if pieces.len() < 2 {
        return Err(Declined::Profile);
    }
    Ok(pieces)
}

/// The area a loop of pieces encloses, positive when it turns anticlockwise:
/// the shoelace over the chords, and the segment each arc adds to its chord.
pub(super) fn signed_area(pieces: &[Piece]) -> f64 {
    pieces
        .iter()
        .map(|piece| {
            let chord = piece.from().perp_dot(piece.to()) / 2.0;
            match *piece {
                Piece::Straight { .. } => chord,
                Piece::Arc { radius, sweep, .. } => {
                    chord + radius * radius * (sweep - sweep.sin()) / 2.0
                }
            }
        })
        .sum()
}

/// The pieces turned the way `anticlockwise` asks.
pub(super) fn turned(pieces: Vec<Piece>, anticlockwise: bool) -> Vec<Piece> {
    if (signed_area(&pieces) > 0.0) == anticlockwise {
        pieces
    } else {
        pieces.iter().rev().map(Piece::reversed).collect()
    }
}
