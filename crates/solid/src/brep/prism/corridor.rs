//! A stretch of a profile run once each way: a corridor of no width, which a
//! drawing leaves where a trait joins a loop to another, and which a raise
//! would stand two walls back to back along. Two loops meeting at a point
//! are no corridor, and are laid as one corner (`touch.rs`) — unless two of
//! the walls leaving that corner nearly lie along each other, which leaves a
//! fin between them.

use std::f64::consts::TAU;

use glam::DVec2;

use crate::brep::piece::{Named, Piece};

/// How far apart, in radians, the walls leaving a corner the profile passes
/// through twice have to stand. Closer, they hold between them a fin nobody
/// drew: a side the solver left leaning on its neighbour by a hair, read as
/// two sides parting at the corner they share (#540).
const NEEDLE: f64 = 1e-3;

/// Whether a corner standing where another stands, within `eps`, has two runs
/// leaving it nearly the same way.
pub(super) fn needle(contours: &[Vec<Named>], eps: f64) -> bool {
    let leaving: Vec<(DVec2, DVec2)> = contours
        .iter()
        .filter(|pieces| pieces.len() > 1)
        .flat_map(|pieces| {
            pieces.iter().enumerate().flat_map(|(at, named)| {
                let before = &pieces[(at + pieces.len() - 1) % pieces.len()].piece;
                let corner = named.piece.from();
                [
                    (corner, way(&named.piece, true)),
                    (corner, way(before, false)),
                ]
            })
        })
        .collect();
    let met = |corner: DVec2| {
        leaving
            .iter()
            .filter(|(other, _)| other.distance(corner) <= eps)
            .count()
            > 2
    };
    leaving.iter().enumerate().any(|(at, (corner, one))| {
        met(*corner)
            && leaving[at + 1..].iter().any(|(other, way)| {
                other.distance(*corner) <= eps && one.angle_to(*way).abs() < NEEDLE
            })
    })
}

/// Which way a piece leaves its start, or, read back from its end, which way
/// it leaves that end.
fn way(piece: &Piece, from_start: bool) -> DVec2 {
    match *piece {
        Piece::Straight { from, to } if from_start => (to - from).normalize_or_zero(),
        Piece::Straight { from, to } => (from - to).normalize_or_zero(),
        Piece::Arc {
            from,
            to,
            center,
            sweep,
            ..
        } => {
            let (corner, sign) = if from_start { (from, 1.0) } else { (to, -1.0) };
            (corner - center).perp().normalize_or_zero() * sweep.signum() * sign
        }
    }
}

/// Whether two pieces of the profile run along one stretch longer than `eps`,
/// one each way.
pub(super) fn run_both_ways(contours: &[Vec<Named>], eps: f64) -> bool {
    let pieces: Vec<Piece> = contours.iter().flatten().map(|named| named.piece).collect();
    pieces.iter().enumerate().any(|(at, one)| {
        pieces[at + 1..]
            .iter()
            .any(|other| back_along(one, other, eps))
    })
}

fn back_along(one: &Piece, other: &Piece, eps: f64) -> bool {
    match (*one, *other) {
        (
            Piece::Straight { from, to },
            Piece::Straight {
                from: start,
                to: end,
            },
        ) => {
            let along = to - from;
            let length = along.length();
            let off = |point: DVec2| along.perp_dot(point - from).abs() / length;
            if off(start) > eps || off(end) > eps || along.dot(end - start) >= 0.0 {
                return false;
            }
            let at = |point: DVec2| along.dot(point - from) / length;
            let (low, high) = (at(end), at(start));
            high.min(length) - low.max(0.0) > eps
        }
        (
            Piece::Arc {
                center,
                radius,
                sweep,
                ..
            },
            Piece::Arc {
                center: other_center,
                radius: other_radius,
                sweep: other_sweep,
                ..
            },
        ) => {
            if center.distance(other_center) > eps
                || (radius - other_radius).abs() > eps
                || sweep * other_sweep >= 0.0
            {
                return false;
            }
            let [first, second] = [one, other].map(|piece| span(piece, center));
            shared(first, second) * radius > eps
        }
        _ => false,
    }
}

/// Where an arc runs round `center`, anticlockwise: the angle it starts at in
/// `[0, 2π)`, and how far round it goes.
fn span(piece: &Piece, center: DVec2) -> (f64, f64) {
    let Piece::Arc { from, sweep, .. } = *piece else {
        return (0.0, 0.0);
    };
    let start = (from - center).to_angle() + sweep.min(0.0);
    (start.rem_euclid(TAU), sweep.abs())
}

/// How far round two stretches of one circle share.
fn shared((first, first_length): (f64, f64), (second, second_length): (f64, f64)) -> f64 {
    [-TAU, 0.0, TAU]
        .into_iter()
        .map(|shift| {
            let start = second + shift;
            ((first + first_length).min(start + second_length) - first.max(start)).max(0.0)
        })
        .sum()
}
