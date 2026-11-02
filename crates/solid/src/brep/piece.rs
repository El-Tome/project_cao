//! A contour read into the pieces its walls stand on: one per line, one per
//! circle and one per ellipse, so that two runs on one line make one wall and
//! a whole circle made of two halves makes a ring. Each piece carries the
//! numbers of the runs it was read from, which name its wall.

pub(super) mod oval;
mod read;
mod way;

use std::f64::consts::TAU;

use glam::DVec2;

use crate::brep::Declined;
use crate::brep::topology::ascending;
use crate::profile::Contour;
use read::{joined, read};
pub(super) use way::{signed_area, turned};

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
    /// `from` run round the ellipse about `center` by the eccentric angle
    /// `sweep` lands on `to`: the ellipse whose first half-axis is `first`,
    /// its direction and its length, and whose second reaches `second` a
    /// quarter turn anticlockwise from it. A ring when the sweep is a whole
    /// turn and the two are one corner.
    Oval {
        from: DVec2,
        to: DVec2,
        center: DVec2,
        first: DVec2,
        second: f64,
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

impl Piece {
    pub(super) fn from(&self) -> DVec2 {
        match *self {
            Piece::Straight { from, .. } | Piece::Arc { from, .. } | Piece::Oval { from, .. } => {
                from
            }
        }
    }

    pub(super) fn to(&self) -> DVec2 {
        match *self {
            Piece::Straight { to, .. } | Piece::Arc { to, .. } | Piece::Oval { to, .. } => to,
        }
    }

    pub(super) fn is_ring(&self) -> bool {
        matches!(
            *self,
            Piece::Arc { sweep, .. } | Piece::Oval { sweep, .. } if sweep.abs() == TAU
        )
    }

    /// Whether `corner` lies within `eps` of the line, the circle or the
    /// ellipse the piece runs along.
    fn holds(&self, corner: DVec2, eps: f64) -> bool {
        match *self {
            Piece::Straight { from, to } => {
                let along = to - from;
                along.perp_dot(corner - from).abs() <= eps * along.length()
            }
            Piece::Arc { center, radius, .. } => (corner.distance(center) - radius).abs() <= eps,
            Piece::Oval {
                center,
                first,
                second,
                ..
            } => oval::off(center, first, second, corner) <= eps,
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

/// The merged pieces, a lone circle or ellipse made a ring of exactly one
/// turn, and a lone ellipse a hair from round the ring of a circle.
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
    if let [
        Named {
            piece:
                Piece::Oval {
                    first,
                    second,
                    sweep,
                    ..
                },
            ..
        },
    ] = pieces.as_mut_slice()
    {
        if first.length().max(*second) * (sweep.abs() - TAU).abs() > eps {
            return Err(Declined::Profile);
        }
        *sweep = TAU.copysign(*sweep);
        pieces[0].piece = oval::rounded(pieces[0].piece, eps);
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
