//! Where a profile touches itself at one point: a whole circle touching
//! another of its loops, or two corners standing in one place — a loop
//! passing twice through it, or two loops meeting there. Decided once, on the
//! pieces, and laid as one corner.
//!
//! Where a circle touches, the two loops part only as the square of the
//! distance from the touch, so the triangles cannot keep them apart there
//! unless both stop at one point. Where two corners stand together, two
//! vertices of one point would be two walls' edges where there is one. The
//! raise lays that point as a corner every loop through it shares, a line up
//! the walls that meet there, and each wall a circle touches parted there: the
//! body a boolean leaves when it cuts the same holes out of the outline.

use std::f64::consts::TAU;

use glam::DVec2;

use crate::brep::piece::{Named, Piece};

/// A loop of the profile parted at the touches: each piece with the touch
/// its start stands at, if any.
pub(super) struct Parted {
    pub(super) pieces: Vec<Named>,
    pub(super) touches: Vec<Option<usize>>,
}

/// Every loop parted wherever a whole circle of the profile touches another
/// loop, every corner standing where another does made one, and how many
/// touches there are. A circle's touch within `eps` of a corner of the piece
/// it lands on is no touch of this kind and is left alone.
pub(super) fn parted(contours: Vec<Vec<Named>>, eps: f64) -> (Vec<Parted>, usize) {
    let mut points: Vec<DVec2> = Vec::new();
    let met = corners_met(&contours, &mut points, eps);
    let mut stops: Vec<Vec<Vec<(f64, usize)>>> = contours
        .iter()
        .map(|pieces| vec![Vec::new(); pieces.len()])
        .collect();
    for (ring_at, ring) in contours.iter().enumerate() {
        let [whole] = ring.as_slice() else {
            continue;
        };
        let Piece::Arc {
            from,
            center,
            sweep,
            ..
        } = whole.piece
        else {
            continue;
        };
        if !whole.piece.is_ring() {
            continue;
        }
        for (other_at, other) in contours.iter().enumerate() {
            let rings_seen = matches!(other.as_slice(), [one] if one.piece.is_ring());
            if other_at == ring_at || (rings_seen && other_at < ring_at) {
                continue;
            }
            for (piece_at, named) in other.iter().enumerate() {
                let Some(point) = touch(&whole.piece, &named.piece, eps) else {
                    continue;
                };
                let Some(along) = share(&named.piece, point, eps) else {
                    continue;
                };
                let id = points
                    .iter()
                    .position(|known| known.distance(point) <= eps)
                    .unwrap_or_else(|| {
                        points.push(point);
                        points.len() - 1
                    });
                stops[other_at][piece_at].push((along, id));
                stops[ring_at][0].push((turn(from, center, point, sweep), id));
            }
        }
    }
    let parted = contours
        .into_iter()
        .zip(stops)
        .zip(met)
        .map(|((pieces, stops), met)| part(pieces, stops, met, &points))
        .collect();
    (parted, points.len())
}

/// The touch each piece's start stands at when another corner of the profile,
/// of the same loop or another, stands within `eps` of it; each such place
/// added to `points`. A ring has no corner.
fn corners_met(
    contours: &[Vec<Named>],
    points: &mut Vec<DVec2>,
    eps: f64,
) -> Vec<Vec<Option<usize>>> {
    let mut met: Vec<Vec<Option<usize>>> = contours
        .iter()
        .map(|pieces| vec![None; pieces.len()])
        .collect();
    let mut corners: Vec<(DVec2, usize, usize)> = contours
        .iter()
        .enumerate()
        .filter(|(_, pieces)| !matches!(pieces.as_slice(), [ring] if ring.piece.is_ring()))
        .flat_map(|(at, pieces)| {
            pieces
                .iter()
                .enumerate()
                .map(move |(piece_at, named)| (named.piece.from(), at, piece_at))
        })
        .collect();
    corners.sort_by(|one, other| one.0.x.total_cmp(&other.0.x));
    for (rank, &(corner, at, piece_at)) in corners.iter().enumerate() {
        for &(other, other_at, other_piece) in corners[rank + 1..]
            .iter()
            .take_while(|other| other.0.x - corner.x <= eps)
        {
            if corner.distance(other) > eps || met[other_at][other_piece].is_some() {
                continue;
            }
            let id = *met[at][piece_at].get_or_insert_with(|| {
                points.push(corner);
                points.len() - 1
            });
            met[other_at][other_piece] = Some(id);
        }
    }
    met
}

/// Where a whole circle touches a piece, on the piece's line or between the
/// two circles: none when they stand further apart than `eps` there, or cross.
fn touch(ring: &Piece, piece: &Piece, eps: f64) -> Option<DVec2> {
    let Piece::Arc { center, radius, .. } = *ring else {
        return None;
    };
    match *piece {
        Piece::Straight { from, to } => {
            let along = to - from;
            let foot = from + along * (center - from).dot(along) / along.length_squared();
            ((foot.distance(center) - radius).abs() <= eps).then_some(foot)
        }
        Piece::Arc {
            center: other,
            radius: other_radius,
            ..
        } => {
            let apart = center.distance(other);
            let inside = (radius - other_radius).abs();
            let touching = (apart - radius - other_radius).abs() <= eps
                || (inside > eps && (apart - inside).abs() <= eps);
            if !touching {
                return None;
            }
            let way = (center - other) / apart;
            let on_other = [other + way * other_radius, other - way * other_radius]
                .into_iter()
                .min_by(|one, two| {
                    let off = |point: &DVec2| (point.distance(center) - radius).abs();
                    off(one).total_cmp(&off(two))
                })?;
            let on_ring = center + (on_other - center).normalize() * radius;
            Some((on_other + on_ring) / 2.0)
        }
    }
}

/// How far along a piece `point` stands, as the share of a straight run or
/// the turn of an arc from its start: none when it stands within `eps` of
/// either end or off the stretch the piece runs over. Any point of a ring.
fn share(piece: &Piece, point: DVec2, eps: f64) -> Option<f64> {
    match *piece {
        Piece::Straight { from, to } => {
            let along = to - from;
            let share = (point - from).dot(along) / along.length_squared();
            let clear = point.distance(from) > eps && point.distance(to) > eps;
            (clear && 0.0 < share && share < 1.0).then_some(share)
        }
        Piece::Arc {
            from,
            to,
            center,
            sweep,
            ..
        } => {
            let turned = turn(from, center, point, sweep);
            if piece.is_ring() {
                return Some(turned);
            }
            let clear = point.distance(from) > eps && point.distance(to) > eps;
            (clear && turned < sweep.abs()).then_some(turned)
        }
    }
}

/// How far `point` stands round `center` from `from`, the way `sweep` turns,
/// in `[0, 2π)`.
fn turn(from: DVec2, center: DVec2, point: DVec2, sweep: f64) -> f64 {
    let angle = (from - center).angle_to(point - center);
    (angle * sweep.signum()).rem_euclid(TAU)
}

/// A loop's pieces cut at their stops, each piece with the touch its start
/// stands at: the corner it met, or the stop it starts from. A ring with
/// stops starts at its first.
fn part(
    pieces: Vec<Named>,
    stops: Vec<Vec<(f64, usize)>>,
    met: Vec<Option<usize>>,
    points: &[DVec2],
) -> Parted {
    let mut parted = Parted {
        pieces: Vec::new(),
        touches: Vec::new(),
    };
    for ((named, mut stops), met) in pieces.into_iter().zip(stops).zip(met) {
        stops.sort_by(|one, other| one.0.total_cmp(&other.0));
        stops.dedup_by_key(|stop| stop.1);
        let piece = named.piece;
        if stops.is_empty() {
            parted.pieces.push(named);
            parted.touches.push(met);
            continue;
        }
        let origin = if piece.is_ring() { stops[0].0 } else { 0.0 };
        let cuts = stops
            .iter()
            .map(|&(share, id)| (share - origin, points[id], Some(id)));
        let nodes: Vec<(f64, DVec2, Option<usize>)> = match piece {
            Piece::Arc { .. } if piece.is_ring() => {
                let first = stops[0].1;
                cuts.chain([(TAU, points[first], Some(first))]).collect()
            }
            Piece::Arc { sweep, .. } => std::iter::once((0.0, piece.from(), met))
                .chain(cuts)
                .chain([(sweep.abs(), piece.to(), None)])
                .collect(),
            Piece::Straight { from, to } => std::iter::once((0.0, from, met))
                .chain(cuts)
                .chain([(1.0, to, None)])
                .collect(),
        };
        for pair in nodes.windows(2) {
            let [(share, start, touch), (next, end, _)] = [pair[0], pair[1]];
            parted.pieces.push(Named {
                piece: between(&piece, start, end, next - share),
                numbers: named.numbers.clone(),
            });
            parted.touches.push(touch);
        }
    }
    parted
}

/// The stretch of a piece from `start` to `end`, both on it, turning by
/// `turned` the way it turns when it is an arc.
fn between(piece: &Piece, start: DVec2, end: DVec2, turned: f64) -> Piece {
    match *piece {
        Piece::Straight { .. } => Piece::Straight {
            from: start,
            to: end,
        },
        Piece::Arc {
            center,
            radius,
            sweep,
            ..
        } => Piece::Arc {
            from: start,
            to: end,
            center,
            radius,
            sweep: turned.copysign(sweep),
        },
    }
}
