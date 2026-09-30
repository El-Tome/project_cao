//! The arcs leaving each vertex, in turn round it, and the cycles they bound:
//! each keeps what it bounds on its left.

use std::f64::consts::TAU;

use super::Arc;
use crate::brep::Declined;

/// A half of an arc by rank: the arc run along its own way, or against it.
pub(super) fn half(arc: usize, forward: bool) -> usize {
    2 * arc + usize::from(!forward)
}

fn arc_of(half: usize) -> (usize, bool) {
    (half / 2, half.is_multiple_of(2))
}

pub(super) struct Cycles {
    /// Each cycle as its arcs in turn, each run along its own way or not.
    pub list: Vec<Vec<(usize, bool)>>,
    /// The cycle each half of an arc belongs to, by the half's rank.
    pub of_half: Vec<usize>,
}

pub(super) fn cycles(arcs: &[Arc]) -> Result<Cycles, Declined> {
    let stars = stars(arcs)?;
    let halves = 2 * arcs.len();
    let mut place = vec![(usize::MAX, 0); halves];
    for (vertex, star) in stars.iter().enumerate() {
        for (rank, &leaving) in star.iter().enumerate() {
            place[leaving] = (vertex, rank);
        }
    }
    let next = |current: usize| {
        let twin = current ^ 1;
        let (vertex, rank) = place[twin];
        if vertex == usize::MAX {
            return current;
        }
        let star = &stars[vertex];
        star[(rank + star.len() - 1) % star.len()]
    };
    let mut of_half = vec![usize::MAX; halves];
    let mut list = Vec::new();
    for start in 0..halves {
        if of_half[start] != usize::MAX {
            continue;
        }
        let mut cycle = Vec::new();
        let mut current = start;
        loop {
            of_half[current] = list.len();
            cycle.push(arc_of(current));
            current = next(current);
            if current == start {
                break;
            }
        }
        list.push(cycle);
    }
    Ok(Cycles { list, of_half })
}

/// The halves leaving each vertex, counterclockwise.
fn stars(arcs: &[Arc]) -> Result<Vec<Vec<usize>>, Declined> {
    let count = arcs
        .iter()
        .filter_map(|arc| arc.ends)
        .flatten()
        .max()
        .map_or(0, |last| last + 1);
    let mut stars = vec![Vec::new(); count];
    for (rank, arc) in arcs.iter().enumerate() {
        if let Some([start, end]) = arc.ends {
            stars[start].push(leaving(arcs, half(rank, true)));
            stars[end].push(leaving(arcs, half(rank, false)));
        }
    }
    stars.into_iter().map(ordered).collect()
}

/// How a half leaves its vertex: the angle it sets off at, and how it bends
/// from there, positive to its left.
#[derive(Clone, Copy)]
struct Leaving {
    half: usize,
    angle: f64,
    bend: f64,
}

fn leaving(arcs: &[Arc], half: usize) -> Leaving {
    let (arc, forward) = arc_of(half);
    let [_, first, second] = arcs[arc].trace.at(if forward { 0.0 } else { 1.0 });
    let direction = if forward { first } else { -first };
    Leaving {
        half,
        angle: direction.y.atan2(direction.x),
        bend: direction.perp_dot(second) / direction.length().powi(3),
    }
}

/// Two directions closer than this, in radians, set off together: rounding
/// is all that parts arcs tangent at their vertex, and nothing the boolean
/// cut comes this close without touching.
const ANGLE_TIE: f64 = 1e-10;

/// Two bends closer than this, relative to the larger, cannot be told apart.
const BEND_TIE: f64 = 1e-9;

/// Counterclockwise by angle, an arc bending further left after one setting
/// off with it. The turn is cut at the widest gap between directions, so
/// that no two directions setting off together straddle the cut.
fn ordered(mut star: Vec<Leaving>) -> Result<Vec<usize>, Declined> {
    star.sort_by(|left, right| left.angle.total_cmp(&right.angle));
    let count = star.len();
    let gap = |rank: usize| {
        let next = star[(rank + 1) % count].angle;
        let wrap = if rank + 1 == count { TAU } else { 0.0 };
        next + wrap - star[rank].angle
    };
    if let Some(widest) = (0..count).max_by(|&left, &right| gap(left).total_cmp(&gap(right))) {
        star.rotate_left((widest + 1) % count);
    }
    for rank in 1..count {
        if star[rank].angle < star[rank - 1].angle {
            star[rank].angle += TAU;
        }
    }
    let mut order = Vec::with_capacity(count);
    let mut start = 0;
    while start < count {
        let mut end = start + 1;
        while end < count && star[end].angle - star[end - 1].angle <= ANGLE_TIE {
            end += 1;
        }
        let together = &mut star[start..end];
        together.sort_by(|left, right| left.bend.total_cmp(&right.bend));
        if together.windows(2).any(|pair| {
            (pair[1].bend - pair[0].bend).abs()
                <= BEND_TIE * pair[0].bend.abs().max(pair[1].bend.abs())
        }) {
            return Err(Declined::Tie);
        }
        order.extend(together.iter().map(|leaving| leaving.half));
        start = end;
    }
    Ok(order)
}
