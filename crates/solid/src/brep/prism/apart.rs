//! Whether an ellipse of a profile comes within the tolerance of another of
//! its loops: a touch the raise would have to lay as one corner both loops
//! share, as `touch.rs` lays a whole circle's, which it does not do on an
//! ellipse yet (#571). Such a profile is declined.
//!
//! The nearest the two come is found along the ellipse: read at places
//! spread along its eccentric angle, the distance to the other piece in
//! closed form at each, then closed in on about the nearest of them. Two
//! pieces whose boxes stand further apart than the tolerance are not read.

use std::f64::consts::TAU;

use glam::DVec2;

use crate::brep::piece::{Named, Piece, oval};

/// How many places a whole turn of an ellipse is read at before closing in.
const PLACES: f64 = 512.0;

/// Halvings of a third closing in on the nearest place.
const CLOSINGS: usize = 120;

/// Whether an ellipse of one loop comes within `eps` of a piece of another.
pub(super) fn touches_another_loop(contours: &[Vec<Named>], eps: f64) -> bool {
    contours.iter().enumerate().any(|(at, pieces)| {
        pieces.iter().any(|named| {
            matches!(named.piece, Piece::Oval { .. })
                && contours
                    .iter()
                    .enumerate()
                    .filter(|(other, _)| *other != at)
                    .flat_map(|(_, others)| others)
                    .any(|other| nearest(&named.piece, &other.piece, eps) <= eps)
        })
    })
}

/// How near an oval piece comes to another piece, or more than `eps` where
/// their boxes stand further apart.
fn nearest(piece: &Piece, other: &Piece, eps: f64) -> f64 {
    let Piece::Oval {
        from,
        center,
        first,
        second,
        sweep,
        ..
    } = *piece
    else {
        return f64::INFINITY;
    };
    let axes = (first, second);
    let reach = first.length().max(second);
    let [low, high] = boxed(other);
    let grown = DVec2::splat(reach + eps);
    if (center - grown).cmpgt(high).any() || (center + grown).cmplt(low).any() {
        return f64::INFINITY;
    }
    let start = oval::angle(center, axes, from);
    let away = |share: f64| distance(other, oval::at(center, axes, start + sweep * share));
    let count = ((sweep.abs() / TAU * PLACES).ceil() as usize).max(16);
    let step = 1.0 / count as f64;
    let best = (0..=count)
        .min_by(|one, two| away(*one as f64 * step).total_cmp(&away(*two as f64 * step)))
        .unwrap_or(0);
    let (mut low, mut high) = (
        (best as f64 - 1.0).max(0.0) * step,
        (best as f64 + 1.0).min(count as f64) * step,
    );
    for _ in 0..CLOSINGS {
        let (left, right) = (low + (high - low) / 3.0, high - (high - low) / 3.0);
        if away(left) < away(right) {
            high = right;
        } else {
            low = left;
        }
    }
    away((low + high) / 2.0).min(away(best as f64 * step))
}

/// How far `point` stands from a piece, its ends included.
fn distance(piece: &Piece, point: DVec2) -> f64 {
    let ends = point.distance(piece.from()).min(point.distance(piece.to()));
    match *piece {
        Piece::Straight { from, to } => {
            let along = to - from;
            let share = ((point - from).dot(along) / along.length_squared()).clamp(0.0, 1.0);
            point.distance(from + along * share)
        }
        Piece::Arc {
            from,
            center,
            radius,
            sweep,
            ..
        } => {
            let turned =
                ((from - center).angle_to(point - center) * sweep.signum()).rem_euclid(TAU);
            if turned <= sweep.abs() {
                (point.distance(center) - radius).abs()
            } else {
                ends
            }
        }
        Piece::Oval {
            from,
            center,
            first,
            second,
            sweep,
            ..
        } => {
            let wall = oval::wall(center, (first, second));
            let foot = wall.nearest(point.extend(0.0)).truncate();
            let start = oval::angle(center, (first, second), from);
            let at = oval::angle(center, (first, second), foot) - start;
            let turned = (at * sweep.signum()).rem_euclid(TAU);
            if turned <= sweep.abs() {
                point.distance(foot)
            } else {
                ends
            }
        }
    }
}

/// The box round a piece: its ends, and round an arc or an ellipse the box
/// of its whole curve.
fn boxed(piece: &Piece) -> [DVec2; 2] {
    let (from, to) = (piece.from(), piece.to());
    let (center, reach) = match *piece {
        Piece::Straight { .. } => return [from.min(to), from.max(to)],
        Piece::Arc { center, radius, .. } => (center, radius),
        Piece::Oval {
            center,
            first,
            second,
            ..
        } => (center, first.length().max(second)),
    };
    [center - DVec2::splat(reach), center + DVec2::splat(reach)]
}
