//! An oval run read into its piece (#571): the ellipse about the run's
//! centre, its first half-axis as drawn and the other's length a quarter turn
//! anticlockwise from it, and the eccentric angle the run turns by from its
//! corner. An ellipse whose two half-axes are one within the tolerance is the
//! circle of their mean, decided once here: its wall is a circular cylinder,
//! as an arc's is. So is a whole ellipse whose half-axes are one within
//! decision 8's hair (Tom, 2026-10-13): no corner stands on it for the move
//! to take off its wall, and the hair is under any step the application
//! shows. So is an arc of one between two straight pieces (Tom, 2026-11-03,
//! #574), its corners laid on the circle; beside another curve, an arc keeps
//! the tolerance, its corners on the ellipse.

use std::f64::consts::{PI, TAU};

use glam::{DVec2, DVec3};

use super::{Piece, ROUNDINGS};
use crate::brep::Declined;
use crate::brep::scale::Scale;
use crate::brep::surface::EllipticCylinder;

/// Where the ellipse about `center` stands at the eccentric angle `angle`
/// from its first half-axis.
pub(in crate::brep) fn at(center: DVec2, (first, second): (DVec2, f64), angle: f64) -> DVec2 {
    center + first * angle.cos() + first.perp().normalize() * (second * angle.sin())
}

/// The eccentric angle from its first half-axis at which the ellipse about
/// `center` passes through `point`, or through where `point` is squashed
/// onto it.
pub(in crate::brep) fn angle(center: DVec2, (first, second): (DVec2, f64), point: DVec2) -> f64 {
    let out = point - center;
    let along = out.dot(first) / first.length_squared();
    let across = out.dot(first.perp().normalize()) / second;
    across.atan2(along)
}

/// The ellipse as the wall it is raised into stands on the profile's own
/// plane, its axis square to it.
pub(in crate::brep) fn wall(center: DVec2, (first, second): (DVec2, f64)) -> EllipticCylinder {
    EllipticCylinder::about(center.extend(0.0), DVec3::Z, first.extend(0.0), second)
}

/// How far `point` stands from the ellipse.
pub(super) fn off(center: DVec2, first: DVec2, second: f64, point: DVec2) -> f64 {
    wall(center, (first, second))
        .distance(point.extend(0.0))
        .abs()
}

/// The piece an oval run from `from` to `to` makes: declined where its
/// ellipse is no ellipse, too large for the tolerance to hold its points,
/// where `from` stands off it, or where the turn does not land on `to`.
pub(super) fn read(
    [from, to]: [DVec2; 2],
    center: DVec2,
    (first, second): (DVec2, f64),
    turn: f64,
    eps: f64,
) -> Result<Piece, Declined> {
    let held = center.is_finite()
        && first.is_finite()
        && first.length() > eps
        && second > eps
        && turn.is_finite()
        && first.length().max(second) * ROUNDINGS * f64::EPSILON <= eps;
    if !held {
        return Err(Declined::Profile);
    }
    let size = first.length().max(second);
    let whole = size * (turn.abs() - TAU).abs() <= eps;
    let apart = from.distance(to) > eps && turn.abs() < TAU;
    let start = angle(center, (first, second), from);
    let lands = at(center, (first, second), start + turn).distance(to) <= eps;
    if !((whole || apart) && off(center, first, second, from) <= eps && lands) {
        return Err(Declined::Profile);
    }
    if (first.length() - second).abs() <= eps {
        return Ok(round([from, to], center, (first, second), turn));
    }
    Ok(Piece::Oval {
        from,
        to,
        center,
        first,
        second,
        sweep: turn,
    })
}

/// A run along an ellipse whose half-axes are one within the tolerance, as
/// the arc of the circle of their mean: its turn about the centre is the
/// eccentric one, read again off where its two corners stand about the
/// centre, which part from their eccentric angles by a hair.
fn round([from, to]: [DVec2; 2], center: DVec2, axes: (DVec2, f64), turn: f64) -> Piece {
    let aside = |point: DVec2| (point - center).to_angle() - angle(center, axes, point);
    let shift = aside(to) - aside(from);
    Piece::Arc {
        from,
        to,
        center,
        radius: (axes.0.length() + axes.1) / 2.0,
        sweep: turn + (shift + PI).rem_euclid(TAU) - PI,
    }
}

/// A whole ellipse, a ring, whose half-axes are one within decision 8's
/// hair, as the ring of the circle of their mean; any other piece as it is.
pub(super) fn rounded(piece: Piece, eps: f64) -> Piece {
    match piece {
        Piece::Oval {
            from,
            to,
            center,
            first,
            second,
            sweep,
        } if piece.is_ring() && (first.length() - second).abs() <= Scale::HAIR * eps => {
            Piece::Arc {
                from,
                to,
                center,
                radius: (first.length() + second) / 2.0,
                sweep,
            }
        }
        other => other,
    }
}

/// Each arc of an ellipse whose half-axes are one within decision 8's hair,
/// between two straight pieces, as the arc of the circle of their mean (Tom,
/// 2026-11-03, #574): its corners laid on that circle along their rays from
/// the centre, by half the hair at most, and the straight pieces either side
/// ending where they now stand. An arc beside another curve keeps the
/// tolerance, its corners on the ellipse: moved, the corner would leave that
/// curve.
pub(super) fn laid_round(pieces: &mut [super::Named], eps: f64) {
    let count = pieces.len();
    for index in 0..count {
        let Piece::Oval {
            from,
            to,
            center,
            first,
            second,
            sweep,
        } = pieces[index].piece
        else {
            continue;
        };
        let [before, after] = [(index + count - 1) % count, (index + 1) % count];
        let straight = |rank: usize| matches!(pieces[rank].piece, Piece::Straight { .. });
        if pieces[index].piece.is_ring()
            || (first.length() - second).abs() > Scale::HAIR * eps
            || !(straight(before) && straight(after))
        {
            continue;
        }
        let radius = (first.length() + second) / 2.0;
        let laid = |corner: DVec2| center + (corner - center).normalize() * radius;
        let [from, to] = [laid(from), laid(to)];
        pieces[index].piece = round([from, to], center, (first, second), sweep);
        if let Piece::Straight { to: end, .. } = &mut pieces[before].piece {
            *end = from;
        }
        if let Piece::Straight { from: start, .. } = &mut pieces[after].piece {
            *start = to;
        }
    }
}

/// One piece for two consecutive runs of one ellipse, read in the first's
/// half-axes: none where they run along two ellipses, a refusal where the
/// second turns back over the first or the whole turn misses its end.
pub(super) fn joined(first: &Piece, second: &Piece, eps: f64) -> Result<Option<Piece>, Declined> {
    let (
        Piece::Oval {
            from,
            center,
            first: axis,
            second: reach,
            sweep,
            ..
        },
        Piece::Oval {
            to,
            center: other_center,
            first: other_axis,
            second: other_reach,
            sweep: other_sweep,
            ..
        },
    ) = (*first, *second)
    else {
        return Ok(None);
    };
    let [one, other] = [
        wall(center, (axis, reach)),
        wall(other_center, (other_axis, other_reach)),
    ];
    if !one.within(&other, eps) {
        return Ok(None);
    }
    let total = sweep + other_sweep;
    let start = angle(center, (axis, reach), from);
    if sweep * other_sweep < 0.0
        || one.major * (total.abs() - TAU) > eps
        || at(center, (axis, reach), start + total).distance(to) > eps
    {
        return Err(Declined::Profile);
    }
    Ok(Some(Piece::Oval {
        from,
        to,
        center,
        first: axis,
        second: reach,
        sweep: total,
    }))
}
