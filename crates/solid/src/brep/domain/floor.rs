//! The floors of a chart: the lines its poles are read as, every angle of
//! one the same point — a cone's apex `l = l_a` (#536), a sphere's two poles
//! `φ = ∓π/2`, a spindle torus's `φ = ∓φ_max` (#576, in [`round`]). A face
//! reaching a pole is a domain closed along its line, for parity alone: a
//! loop arriving at the pole at one angle and leaving at another is closed
//! along it, and a face holding the pole within it is closed by the whole of
//! the line, as a cylinder's band is by its lower circle. Nothing ever keeps
//! a stretch of a floor as an edge.

mod round;

use std::f64::consts::TAU;

use glam::DVec2;

use crate::brep::overlay::Arc;
use crate::brep::surface::Cone;
use crate::brep::trace::Trace;
pub(super) use round::closed_round;
pub(in crate::brep) use round::{floors_round, holds, poles_of, seam};

/// A face's loops closed along the floor of a cone: where a trace ends at
/// the apex and the next starts there, a stretch of the floor between their
/// angles, unwrapped so that the loop turns about the axis by nought, net —
/// a loop through the apex never goes round it; and where the loops go
/// round the axis net, a whole floor turning back against them.
pub(super) fn closed(cone: &Cone, loops: Vec<Vec<Trace>>, eps: f64) -> Vec<Vec<Trace>> {
    let floor = cone.apex_at();
    let mut closed = joined(loops, |point| point.y <= floor + eps);
    let turns = (closed.iter().map(|lap| turning(lap)).sum::<f64>() / TAU).round();
    if turns != 0.0 {
        closed.push(whole(floor, turns));
    }
    closed
}

/// The loops with a stretch of the floor between each trace ending at the
/// pole and the next starting there, unwrapped so that a loop through the
/// pole turns about the axis by nought, net.
fn joined(loops: Vec<Vec<Trace>>, at_the_pole: impl Fn(DVec2) -> bool) -> Vec<Vec<Trace>> {
    loops
        .into_iter()
        .map(|lap| {
            let joins = joins(&lap, &at_the_pole);
            interleaved(lap, joins)
        })
        .collect()
}

/// The stretch of the floor each trace of a loop is followed by: between
/// its end at the pole and the next trace's start there, none elsewhere.
fn joins(lap: &[Trace], at_the_pole: &dyn Fn(DVec2) -> bool) -> Vec<Option<Trace>> {
    let count = lap.len();
    let joins: Vec<usize> = (0..count)
        .filter(|&rank| {
            at_the_pole(lap[rank].end()) && at_the_pole(lap[(rank + 1) % count].start())
        })
        .collect();
    let mut floors: Vec<Option<Trace>> = vec![None; count];
    let Some(&last) = joins.last() else {
        return floors;
    };
    let unclosed = turning(lap);
    for &rank in &joins {
        let from = lap[rank].end();
        let next = lap[(rank + 1) % count].start();
        let mut to = DVec2::new(nearest(next.x, from.x), next.y);
        if rank == last {
            to.x -= TAU * (unclosed / TAU).round();
        }
        floors[rank] = Some(Trace::Segment { from, to });
    }
    floors
}

/// A loop with each trace followed by its stretch of the floor, if any.
pub(super) fn interleaved(lap: Vec<Trace>, floors: Vec<Option<Trace>>) -> Vec<Trace> {
    lap.into_iter()
        .zip(floors)
        .flat_map(|(trace, floor)| std::iter::once(trace).chain(floor))
        .collect()
}

/// A whole floor at `at`, turning back by `turns` turns against the loops.
pub(super) fn whole(at: f64, turns: f64) -> Vec<Trace> {
    vec![Trace::Segment {
        from: DVec2::new(0.0, at),
        to: DVec2::new(-turns * TAU, at),
    }]
}

/// How far a loop of traces turns about the axis, net: each trace's own
/// turning, and from each to the next the nearest way round.
pub(super) fn turning(lap: &[Trace]) -> f64 {
    let count = lap.len();
    (0..count)
        .map(|rank| {
            let (trace, next) = (&lap[rank], &lap[(rank + 1) % count]);
            let end = trace.end().x;
            end - trace.start().x + nearest(next.start().x, end) - end
        })
        .sum()
}

/// An angle moved by whole turns to stand within half a turn of `near`.
fn nearest(angle: f64, near: f64) -> f64 {
    angle + TAU * ((near - angle) / TAU).round()
}

/// The arcs of the floor at `floor` the overlay of a surface is closed by:
/// between each two `ends` at the pole in turn about the axis, each a
/// vertex by rank and where it stands, no two at the same angle, the last
/// running round to the first; with one end, a whole turn from it back to
/// itself; with none, a whole turn with no vertex where the surface holds
/// its pole within a face, and nothing otherwise.
pub(in crate::brep) fn arcs_at(floor: f64, ends: &[(usize, DVec2)], holds: bool) -> Vec<Arc> {
    let mut ends: Vec<(usize, DVec2)> = ends
        .iter()
        .map(|&(rank, at)| (rank, DVec2::new(nearest(at.x, 0.0), at.y)))
        .collect();
    ends.sort_by(|one, other| one.1.x.total_cmp(&other.1.x));
    match ends[..] {
        [] if holds => vec![Arc {
            trace: Trace::Segment {
                from: DVec2::new(0.0, floor),
                to: DVec2::new(TAU, floor),
            },
            ends: None,
        }],
        [] => Vec::new(),
        _ => (0..ends.len())
            .map(|rank| {
                let (start, from) = ends[rank];
                let (end, mut to) = ends[(rank + 1) % ends.len()];
                if rank + 1 == ends.len() {
                    to.x += TAU;
                }
                Arc {
                    trace: Trace::Segment { from, to },
                    ends: Some([start, end]),
                }
            })
            .collect(),
    }
}
