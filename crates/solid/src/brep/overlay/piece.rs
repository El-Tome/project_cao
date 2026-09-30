//! Arcs cut where they turn back in the first parameter, so that each piece
//! is a graph over it: a vertical line meets a piece once at most.

use std::f64::consts::PI;

use glam::DVec2;

use super::Arc;
use crate::brep::trace::Trace;

/// A stretch of an arc over which its first parameter only grows, or only
/// shrinks.
#[derive(Clone, Copy, Debug)]
pub(super) struct Piece {
    pub arc: usize,
    /// The arc's own parameter at the piece's end of least first
    /// parameter, and at its end of most.
    pub from_left: f64,
    pub from_right: f64,
    pub left: DVec2,
    pub right: DVec2,
    /// Whether the arc, run along its own way, runs towards growing first
    /// parameter here: the half of the arc with the region above on its left.
    pub rightward: bool,
}

pub(super) fn pieces(arcs: &[Arc]) -> Vec<Piece> {
    let mut pieces = Vec::new();
    for (rank, arc) in arcs.iter().enumerate() {
        let mut cuts = vec![0.0];
        cuts.extend(turns(&arc.trace));
        cuts.push(1.0);
        for pair in cuts.windows(2) {
            let [start, end] = [pair[0], pair[1]].map(|along| arc.trace.at(along)[0]);
            let rightward = end.x > start.x;
            let (from_left, from_right, left, right) = if rightward {
                (pair[0], pair[1], start, end)
            } else {
                (pair[1], pair[0], end, start)
            };
            pieces.push(Piece {
                arc: rank,
                from_left,
                from_right,
                left,
                right,
                rightward,
            });
        }
    }
    pieces
}

/// Where a trace turns back in its first parameter, strictly between its
/// ends, in order along it.
pub(super) fn turns(trace: &Trace) -> Vec<f64> {
    match *trace {
        Trace::Segment { .. } => Vec::new(),
        Trace::Round { start, sweep, .. } => {
            let end = start + sweep;
            let first = (start.min(end) / PI).ceil() as i64;
            let last = (start.max(end) / PI).floor() as i64;
            let mut turns: Vec<f64> = (first..=last)
                .map(|multiple| (multiple as f64 * PI - start) / sweep)
                .filter(|&along| within(along))
                .collect();
            turns.sort_by(f64::total_cmp);
            turns
        }
        Trace::Graph { .. } => sampled_turns(trace),
    }
}

/// A turn closer than this to an end of its trace, in the trace's own
/// parameter, is that end: a turn standing on an end is one rounding moves
/// to either side of it.
const AT_AN_END: f64 = 1e-9;

fn within(along: f64) -> bool {
    along > AT_AN_END && along < 1.0 - AT_AN_END
}

/// How finely a trace with no closed form for its turns is searched for
/// them: a turn is a change of sign of the first parameter's derivative
/// between two samples.
const SEARCH: usize = 128;

/// The turns of any trace, from its derivatives alone.
pub(super) fn sampled_turns(trace: &Trace) -> Vec<f64> {
    let slope = |along: f64| trace.at(along)[1].x;
    let mut turns = Vec::new();
    let mut last: Option<(f64, f64)> = None;
    for step in 0..=SEARCH {
        let along = step as f64 / SEARCH as f64;
        let here = slope(along);
        if here == 0.0 {
            continue;
        }
        if let Some((before, sign)) = last
            && sign * here < 0.0
        {
            turns.push(root(&slope, before, along, sign));
        }
        last = Some((along, here.signum()));
    }
    turns.retain(|&along| within(along));
    turns
}

/// How many times a stretch of a parameter is halved: past this, the halves
/// are finer than the floats can tell over the whole arc.
const HALVINGS: usize = 64;

/// Halves `[low, high]` round a change of sign of `slope`, whose sign at
/// `low` is `sign`.
fn root(slope: &impl Fn(f64) -> f64, mut low: f64, mut high: f64, sign: f64) -> f64 {
    for _ in 0..HALVINGS {
        let middle = 0.5 * (low + high);
        if slope(middle) * sign > 0.0 {
            low = middle;
        } else {
            high = middle;
        }
    }
    0.5 * (low + high)
}

impl Piece {
    /// The height of the piece where its first parameter is `x`, held within
    /// its ends.
    pub fn height(&self, trace: &Trace, x: f64) -> f64 {
        if x <= self.left.x {
            return self.left.y;
        }
        if x >= self.right.x {
            return self.right.y;
        }
        if let Trace::Segment { .. } = trace {
            let along = (x - self.left.x) / (self.right.x - self.left.x);
            return self.left.y + (self.right.y - self.left.y) * along;
        }
        let (mut left, mut right) = (self.from_left, self.from_right);
        for _ in 0..HALVINGS {
            let middle = 0.5 * (left + right);
            if trace.at(middle)[0].x < x {
                left = middle;
            } else {
                right = middle;
            }
        }
        trace.at(0.5 * (left + right))[0].y
    }
}
