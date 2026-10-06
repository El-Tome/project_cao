//! A contour read into the pieces its walls stand on: one per line and one per
//! circle, so that two runs on one line make one wall and a whole circle made
//! of two halves makes a ring. Each piece carries the numbers of the runs it
//! was read from, which name its wall.

use std::f64::consts::TAU;

use glam::DVec2;

use crate::brep::Declined;
use crate::brep::topology::ascending;
use crate::profile::{Contour, Run};

/// How many roundings of its radius a point of a circle carries once it is
/// computed from a centre that far away: an arc so large that they reach the
/// tolerance cannot keep its own points on its surface.
const ROUNDINGS: f64 = 16.0;

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

/// A piece, and the numbers of the walls of the runs merged into it,
/// ascending.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct Named {
    pub piece: Piece,
    pub numbers: Vec<u32>,
}

impl Named {
    pub(super) fn mirrored(&self) -> Named {
        Named {
            piece: self.piece.mirrored(),
            numbers: self.numbers.clone(),
        }
    }

    fn reversed(&self) -> Named {
        Named {
            piece: self.piece.reversed(),
            numbers: self.numbers.clone(),
        }
    }
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

    /// Whether `corner` lies within `eps` of the line or the circle the piece
    /// runs along.
    fn holds(&self, corner: DVec2, eps: f64) -> bool {
        match *self {
            Piece::Straight { from, to } => {
                let along = to - from;
                along.perp_dot(corner - from).abs() <= eps * along.length()
            }
            Piece::Arc { center, radius, .. } => (corner.distance(center) - radius).abs() <= eps,
        }
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

/// The pieces of a contour, runs on one line or one circle merged; the run
/// of rank `k` numbered `first + k`.
pub(super) fn pieces(contour: &Contour, first: u32, eps: f64) -> Result<Vec<Named>, Declined> {
    let numbers: Vec<u32> = (first..).take(contour.runs.len()).collect();
    pieces_numbered(contour, &numbers, eps)
}

/// The pieces of a contour, runs on one line or one circle merged; the run
/// of rank `k` numbered `numbers[k]`.
pub(super) fn pieces_numbered(
    contour: &Contour,
    numbers: &[u32],
    eps: f64,
) -> Result<Vec<Named>, Declined> {
    let count = contour.corners.len();
    if count == 0 || contour.runs.len() != count || numbers.len() != count {
        return Err(Declined::Profile);
    }
    let mut merged: Vec<Taken> = Vec::new();
    for (index, &number) in numbers.iter().enumerate() {
        let Some(piece) = read(
            contour.corners[index],
            contour.corners[(index + 1) % count],
            contour.runs[index],
            eps,
        )?
        else {
            continue;
        };
        let piece = (piece, Vec::new(), vec![number]);
        let whole = match merged.last() {
            Some(last) => fused(last, &piece, eps)?,
            None => None,
        };
        match (whole, merged.last_mut()) {
            (Some(whole), Some(last)) => *last = whole,
            _ => merged.push(piece),
        }
    }
    while merged.len() > 1 {
        let Some(whole) = fused(&merged[merged.len() - 1], &merged[0], eps)? else {
            break;
        };
        merged[0] = whole;
        merged.pop();
    }
    let named = merged.into_iter().map(|(piece, _, numbers)| Named {
        piece,
        numbers: ascending(numbers),
    });
    closed(named.collect(), eps)
}

/// A piece, the corners of its contour that merging took away from inside
/// it, and the numbers of the runs merged into it.
type Taken = (Piece, Vec<DVec2>, Vec<u32>);

/// Two consecutive pieces as one, when `joined` makes them one and every
/// corner the merge takes away still lies on it: checked against the whole
/// piece each time, so that a long run of corners each a hair off the
/// chord of its neighbours cannot bow away from the wall it becomes.
fn fused(first: &Taken, second: &Taken, eps: f64) -> Result<Option<Taken>, Declined> {
    let Some(whole) = joined(&first.0, &second.0, eps)? else {
        return Ok(None);
    };
    let inner: Vec<DVec2> = first
        .1
        .iter()
        .copied()
        .chain(std::iter::once(first.0.to()))
        .chain(second.1.iter().copied())
        .collect();
    let holds = inner.iter().all(|corner| whole.holds(*corner, eps));
    let numbers = first.2.iter().chain(&second.2).copied().collect();
    Ok(holds.then_some((whole, inner, numbers)))
}

/// Every check below is written as what must hold, so that a number which
/// is not one fails it.
///
/// A straight run whose ends are one corner is read as none, as a turn lays
/// a run to no length: it keeps its number and names no wall, and the piece
/// before it ends where the piece after it starts, within the tolerance.
fn read(from: DVec2, to: DVec2, run: Run, eps: f64) -> Result<Option<Piece>, Declined> {
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
fn closed(mut pieces: Vec<Named>, eps: f64) -> Result<Vec<Named>, Declined> {
    if let [
        Named {
            piece: Piece::Arc { radius, sweep, .. },
            ..
        },
    ] = pieces.as_mut_slice()
    {
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

/// Whether every corner of a profile stands apart from every other: a loop
/// passing twice through one corner, or a hole touching the outline there,
/// would make two vertices of one point. A ring has no corner.
pub(super) fn apart(contours: &[Vec<Named>], eps: f64) -> bool {
    let mut corners: Vec<DVec2> = contours
        .iter()
        .filter(|pieces| !matches!(pieces.as_slice(), [ring] if ring.piece.is_ring()))
        .flatten()
        .map(|named| named.piece.from())
        .collect();
    corners.sort_by(|one, other| one.x.total_cmp(&other.x));
    corners.iter().enumerate().all(|(rank, corner)| {
        corners[rank + 1..]
            .iter()
            .take_while(|other| other.x - corner.x <= eps)
            .all(|other| corner.distance(*other) > eps)
    })
}

/// The area a loop of pieces encloses, positive when it turns anticlockwise:
/// the shoelace over the chords, and the segment each arc adds to its chord.
pub(super) fn signed_area(pieces: &[Named]) -> f64 {
    pieces
        .iter()
        .map(|Named { piece, .. }| {
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
pub(super) fn turned(pieces: Vec<Named>, anticlockwise: bool) -> Vec<Named> {
    if (signed_area(&pieces) > 0.0) == anticlockwise {
        pieces
    } else {
        pieces.iter().rev().map(Named::reversed).collect()
    }
}
