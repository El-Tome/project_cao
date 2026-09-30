//! The arcs leaving each vertex, in turn round it, and the cycles they bound:
//! each keeps what it bounds on its left.

use std::f64::consts::{PI, TAU};

use glam::DVec2;

use super::Arc;
use crate::brep::Declined;
use crate::brep::trace::Trace;

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

pub(super) fn cycles(
    arcs: &[Arc],
    vertices: &[DVec2],
    period: Option<f64>,
) -> Result<Cycles, Declined> {
    let stars = stars(arcs, vertices, period)?;
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
fn stars(
    arcs: &[Arc],
    vertices: &[DVec2],
    period: Option<f64>,
) -> Result<Vec<Vec<usize>>, Declined> {
    let count = arcs
        .iter()
        .filter_map(|arc| arc.ends)
        .flatten()
        .max()
        .map_or(0, |last| last + 1);
    let largest = arcs
        .iter()
        .flat_map(|arc| [arc.trace.start(), arc.trace.end()])
        .fold(1.0_f64, |largest, point| {
            largest.max(point.abs().max_element())
        });
    let rounding = ROUNDING * largest;
    let off = |at: DVec2, vertex: usize| {
        let mut apart = at - vertices[vertex];
        if let Some(period) = period {
            apart.x -= (apart.x / period).round() * period;
        }
        apart.length() + rounding
    };
    let mut stars = vec![Vec::new(); count];
    for (rank, arc) in arcs.iter().enumerate() {
        if let Some([start, end]) = arc.ends {
            let [from, to] = [arc.trace.start(), arc.trace.end()];
            stars[start].push(leaving(arcs, half(rank, true), rounding, off(from, start)));
            stars[end].push(leaving(arcs, half(rank, false), rounding, off(to, end)));
        }
    }
    stars.into_iter().map(ordered).collect()
}

/// How a half leaves its vertex: the angle it sets off at, how far rounding
/// may have turned that angle, how it bends from there, positive to its
/// left, and a bend under which rounding hides it over its lever, where it
/// is read off derivatives rather than given by a formula; how far
/// the vertex stands from the arc's end, rounding included, and how long
/// the arc runs; and how far its chord to a point a tenth of the way along
/// turns from the angle it sets off at, which parts two arcs that set off
/// together and bend alike, touching to a higher order.
#[derive(Clone, Copy)]
struct Leaving {
    half: usize,
    angle: f64,
    blur: f64,
    bend: f64,
    flat: f64,
    off: f64,
    length: f64,
    turn: f64,
}

/// How far along an arc its chord is drawn to, as a share of the arc.
const CHORD: f64 = 0.1;

fn leaving(arcs: &[Arc], half: usize, rounding: f64, off: f64) -> Leaving {
    let (arc, forward) = arc_of(half);
    let trace = &arcs[arc].trace;
    let [start, first, second] = trace.at(if forward { 0.0 } else { 1.0 });
    let direction = if forward { first } else { -first };
    let lever = match *trace {
        Trace::Round { radius, .. } => radius,
        _ => direction.length(),
    };
    let angle = direction.y.atan2(direction.x);
    let chord = trace.at(if forward { CHORD } else { 1.0 - CHORD })[0] - start;
    let turned = chord.y.atan2(chord.x) - angle;
    Leaving {
        half,
        angle,
        blur: rounding / lever,
        bend: direction.perp_dot(second) / direction.length().powi(3),
        flat: match *trace {
            Trace::Graph { .. } => 2.0 * rounding / (lever * lever),
            Trace::Segment { .. } | Trace::Round { .. } => 0.0,
        },
        off,
        length: length(trace),
        turn: (turned + PI).rem_euclid(TAU) - PI,
    }
}

/// How many chords a trace with no short formula for its length is measured
/// along.
const PIECES: usize = 16;

/// How long a trace runs in its surface's parameters, along chords where it
/// has no short formula.
fn length(trace: &Trace) -> f64 {
    match *trace {
        Trace::Segment { from, to } => from.distance(to),
        Trace::Round { radius, sweep, .. } => radius * sweep.abs(),
        Trace::Graph { .. } => (0..PIECES)
            .map(|piece| {
                let at = |piece: usize| trace.at(piece as f64 / PIECES as f64)[0];
                at(piece).distance(at(piece + 1))
            })
            .sum(),
    }
}

/// Whether two halves set off together. Beside rounding, two arcs a vertex
/// stands `h` off, bending `k` apart, stay on one side of each other
/// wherever their directions part by less than `√(2hk)`, whichever way they
/// point: a corner standing short of where a circle touches a side, within
/// the tolerance it was merged within, reads the circle still rising towards
/// the side and off to the wrong side of it. Twice that is taken, for the
/// touch itself sits right on the bound. Two arcs whose directions part by
/// less than that from rounding alone come nearer to touching than any
/// tolerance, and were decided to touch.
///
/// Arcs parting by `a` and bending back towards each other meet again `2a/k`
/// on. Where that is not well within the shorter of them, they touch
/// nowhere: they cross there, at their next corner — a circle crossing a
/// side twice a hair from touching it — and part by where they head.
fn together(one: &Leaving, other: &Leaving) -> bool {
    let apart = (other.angle - one.angle).abs();
    let bent = (one.bend - other.bend).abs();
    let touch = if 4.0 * apart <= bent * one.length.min(other.length) {
        (2.0 * (one.off + other.off) * bent).sqrt()
    } else {
        0.0
    };
    apart <= ANGLE_TIE + one.blur + other.blur + 2.0 * touch
}

/// Two directions closer than this, in radians, set off together, even on
/// long arcs whose ends rounding barely moves.
const ANGLE_TIE: f64 = 1e-10;

/// How far rounding may have moved a position, relative to the largest
/// coordinate drawn. Over a short arc it turns the direction set by its ends
/// by far more than `ANGLE_TIE`: a side a hair long leaving a circle it is
/// tangent to would set off to the wrong side of it.
const ROUNDING: f64 = 1e-14;

/// Two bends closer than this, relative to the larger, cannot be told apart.
const BEND_TIE: f64 = 1e-9;

fn alike(one: &Leaving, other: &Leaving) -> bool {
    (one.bend - other.bend).abs()
        <= BEND_TIE * one.bend.abs().max(other.bend.abs()) + one.flat + other.flat
}

/// Two chords closer than this, in radians, part nothing.
const CHORD_TIE: f64 = 1e-9;

/// The angle of a half's chord, read with its angle as it is unwrapped.
fn chord(leaving: &Leaving) -> f64 {
    leaving.angle + leaving.turn
}

/// Counterclockwise by angle, an arc bending further left after one setting
/// off with it. The turn is cut at the widest gap between directions, so
/// that no two directions setting off together straddle the cut. Arcs
/// setting off together are gathered one after the other: two bending alike
/// may be gathered through a third that sets off with each, and they are
/// ordered by where they head unless they set off together themselves.
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
        while end < count && together(&star[end - 1], &star[end]) {
            end += 1;
        }
        let group = &mut star[start..end];
        group.sort_by(
            |left, right| match (alike(left, right), together(left, right)) {
                (false, _) => left.bend.total_cmp(&right.bend),
                (true, false) => left.angle.total_cmp(&right.angle),
                (true, true) => chord(left).total_cmp(&chord(right)),
            },
        );
        if group.windows(2).any(|pair| {
            alike(&pair[0], &pair[1])
                && together(&pair[0], &pair[1])
                && (chord(&pair[1]) - chord(&pair[0])).abs() <= CHORD_TIE
        }) {
            return Err(Declined::Tie);
        }
        order.extend(group.iter().map(|leaving| leaving.half));
        start = end;
    }
    Ok(order)
}
