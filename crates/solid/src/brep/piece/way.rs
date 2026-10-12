//! A piece read the other way or seen in a mirror, and which way a loop of
//! pieces turns: the area it encloses, signed.

use glam::DVec2;

use super::{Named, Piece};

impl Named {
    pub(in crate::brep) fn mirrored(&self) -> Named {
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
    fn mirrored(&self) -> Piece {
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

/// The area a loop of pieces encloses, positive when it turns anticlockwise:
/// the shoelace over the chords, and the segment each arc adds to its chord.
pub(in crate::brep) fn signed_area(pieces: &[Named]) -> f64 {
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
pub(in crate::brep) fn turned(pieces: Vec<Named>, anticlockwise: bool) -> Vec<Named> {
    if (signed_area(&pieces) > 0.0) == anticlockwise {
        pieces
    } else {
        pieces.iter().rev().map(Named::reversed).collect()
    }
}
