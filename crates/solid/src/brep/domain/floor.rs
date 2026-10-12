//! The floor of a cone's parameters: the line `l = l_a` the apex is read as,
//! every angle of it the one point. A face reaching the apex is a domain
//! closed along it, for parity alone: a loop arriving at the apex at one
//! angle and leaving at another is closed along it, and a face holding the
//! apex within it is closed below by the whole of it, as a cylinder's band
//! is by its lower circle. Nothing ever keeps a stretch of it as an edge.

use std::f64::consts::TAU;

use glam::DVec2;

use crate::brep::overlay::Arc;
use crate::brep::surface::Cone;
use crate::brep::trace::Trace;

/// A face's loops closed along the floor: where a trace ends at the apex
/// and the next starts there, a stretch of the floor between their angles,
/// unwrapped so that the loop turns about the axis by nought, net — a loop
/// through the apex never goes round it; and where the loops go round the
/// axis net, a whole floor turning back against them.
pub(super) fn closed(cone: &Cone, loops: Vec<Vec<Trace>>, eps: f64) -> Vec<Vec<Trace>> {
    let floor = cone.apex_at();
    let at_the_apex = |point: DVec2| point.y <= floor + eps;
    let mut closed: Vec<Vec<Trace>> = loops
        .into_iter()
        .map(|lap| {
            let count = lap.len();
            let joins: Vec<usize> = (0..count)
                .filter(|&rank| {
                    at_the_apex(lap[rank].end()) && at_the_apex(lap[(rank + 1) % count].start())
                })
                .collect();
            let Some(&last) = joins.last() else {
                return lap;
            };
            let unclosed = turning(&lap);
            let mut floors: Vec<Option<Trace>> = vec![None; count];
            for &rank in &joins {
                let from = lap[rank].end();
                let next = lap[(rank + 1) % count].start();
                let mut to = DVec2::new(nearest(next.x, from.x), next.y);
                if rank == last {
                    to.x -= TAU * (unclosed / TAU).round();
                }
                floors[rank] = Some(Trace::Segment { from, to });
            }
            lap.into_iter()
                .zip(floors)
                .flat_map(|(trace, floor)| std::iter::once(trace).chain(floor))
                .collect()
        })
        .collect();
    let turns = (closed.iter().map(|lap| turning(lap)).sum::<f64>() / TAU).round();
    if turns != 0.0 {
        closed.push(vec![Trace::Segment {
            from: DVec2::new(0.0, floor),
            to: DVec2::new(-turns * TAU, floor),
        }]);
    }
    closed
}

/// How far a loop of traces turns about the axis, net: each trace's own
/// turning, and from each to the next the nearest way round.
fn turning(lap: &[Trace]) -> f64 {
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

/// The arcs of the floor the overlay of a cone is closed by below: between
/// each two `ends` at the apex in turn about the axis, each a vertex by
/// rank and where it stands, no two at the same angle, the last running
/// round to the first; with one end, a whole turn from it back to itself;
/// with none, a whole turn with no vertex where the surface holds its apex
/// within a face, and nothing otherwise.
pub(in crate::brep) fn arcs(cone: &Cone, ends: &[(usize, DVec2)], holds: bool) -> Vec<Arc> {
    let floor = cone.apex_at();
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
