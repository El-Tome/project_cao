//! The floors of a chart: the lines its poles are read as, every angle of
//! one the same point — a cone's apex `l = l_a` (#536), a sphere's two poles
//! `φ = ∓π/2`, a spindle torus's `φ = ∓φ_max` (#576). A face reaching a pole
//! is a domain closed along its line, for parity alone: a loop arriving at
//! the pole at one angle and leaving at another is closed along it, and a
//! face holding the pole within it is closed by the whole of the line, as a
//! cylinder's band is by its lower circle. Nothing ever keeps a stretch of
//! a floor as an edge.

use std::f64::consts::TAU;

use glam::{DVec2, DVec3};

use super::crossing::heights;
use crate::brep::overlay::Arc;
use crate::brep::surface::{Cone, Surface};
use crate::brep::trace::Trace;

/// A pole of a sphere or a torus: the line of the chart it is read as, which
/// end of the chart it closes, how far from the line a trace may end and
/// still reach it, and where it stands.
#[derive(Clone, Copy, Debug)]
pub(in crate::brep) struct Pole {
    pub at: f64,
    pub high: bool,
    pub room: f64,
    pub point: DVec3,
}

/// The poles of a sphere or a torus, the low one first; none of any other
/// surface, a cone's apex being read as its own floor. `eps` is a length,
/// which the room along the meridian is read from at the tube's radius.
pub(in crate::brep) fn poles_of(surface: &Surface, eps: f64) -> Vec<Pole> {
    let (origin, axis, center, radius, lines): (DVec3, DVec3, f64, f64, Vec<f64>) = match surface {
        Surface::Sphere(sphere) => (
            sphere.origin,
            sphere.axis,
            sphere.center,
            sphere.radius,
            sphere.poles(),
        ),
        Surface::Torus(torus) => (
            torus.origin,
            torus.axis,
            torus.center.x,
            torus.radius,
            torus.poles(),
        ),
        Surface::Plane(_) | Surface::Cylinder(_) | Surface::Cone(_) | Surface::Elliptic(_) => {
            return Vec::new();
        }
    };
    lines
        .into_iter()
        .enumerate()
        .map(|(rank, at)| Pole {
            at,
            high: rank == 1,
            room: eps / radius,
            point: origin + axis * (center + radius * at.sin()),
        })
        .collect()
}

impl Pole {
    /// Whether a point of the chart stands at the pole.
    fn reached(&self, point: DVec2) -> bool {
        if self.high {
            point.y >= self.at - self.room
        } else {
            point.y <= self.at + self.room
        }
    }
}

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

/// A face's loops closed along the floors of a sphere or a torus: at each
/// pole as a cone's are at its apex, and by the whole floor of each pole the
/// face holds within it, which a ray up the meridian from inside the face
/// crosses an odd number of times in all.
pub(super) fn closed_round(poles: &[Pole], loops: Vec<Vec<Trace>>) -> Vec<Vec<Trace>> {
    let held: Vec<bool> = poles.iter().map(|pole| holds(&loops, pole)).collect();
    let mut closed = loops;
    for pole in poles {
        closed = joined(closed, |point| pole.reached(point));
    }
    let turns = (closed.iter().map(|lap| turning(lap)).sum::<f64>() / TAU).round();
    for (pole, held) in poles.iter().zip(held) {
        if held {
            closed.push(whole(pole.at, if turns == 0.0 { -1.0 } else { turns }));
        }
    }
    closed
}

/// Whether a face bounded by `loops` holds a pole within it: not where a
/// trace ends at the pole, which a corner holds already, and otherwise by
/// parity along the meridian from the middle of a trace running round the
/// axis, which keeps the face on its left: the face lies above it where it
/// runs forwards, and the pole is inside where the other traces are crossed
/// an even number of times on the way. A face with no loop holds every
/// pole; one bounded by meridians alone holds none.
pub(in crate::brep) fn holds(loops: &[Vec<Trace>], pole: &Pole) -> bool {
    if loops
        .iter()
        .flatten()
        .any(|trace| pole.reached(trace.start()) || pole.reached(trace.end()))
    {
        return false;
    }
    let Some([place, pace, _]) = loops
        .iter()
        .flatten()
        .map(|trace| trace.at(0.5))
        .find(|[_, pace, _]| pace.x.abs() > ACROSS * pace.length())
    else {
        return loops.is_empty();
    };
    let tiny = pole.room;
    let crossings = heights(loops, place.x, true)
        .into_iter()
        .filter(|height| {
            if pole.high {
                *height > place.y + tiny
            } else {
                *height < place.y - tiny
            }
        })
        .count();
    (pace.x > 0.0) == pole.high && crossings % 2 == 0
        || (pace.x > 0.0) != pole.high && crossings % 2 == 1
}

/// Under this share of its pace, a trace runs along the meridian, and tells
/// nothing of which side the face lies on.
const ACROSS: f64 = 1e-9;

/// The loops with a stretch of the floor between each trace ending at the
/// pole and the next starting there, unwrapped so that a loop through the
/// pole turns about the axis by nought, net.
fn joined(loops: Vec<Vec<Trace>>, at_the_pole: impl Fn(DVec2) -> bool) -> Vec<Vec<Trace>> {
    loops
        .into_iter()
        .map(|lap| {
            let count = lap.len();
            let joins: Vec<usize> = (0..count)
                .filter(|&rank| {
                    at_the_pole(lap[rank].end()) && at_the_pole(lap[(rank + 1) % count].start())
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
        .collect()
}

/// A whole floor at `at`, turning back by `turns` turns against the loops.
fn whole(at: f64, turns: f64) -> Vec<Trace> {
    vec![Trace::Segment {
        from: DVec2::new(0.0, at),
        to: DVec2::new(-turns * TAU, at),
    }]
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
