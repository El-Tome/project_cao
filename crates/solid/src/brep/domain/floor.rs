//! The floors of a chart: the lines its poles are read as, every angle of
//! one the same point — a cone's apex `l = l_a` (#536), a sphere's two poles
//! `φ = ∓π/2`, a spindle torus's `φ = ∓φ_max` (#576). A face reaching a pole
//! is a domain closed along its line, for parity alone: a loop arriving at
//! the pole at one angle and leaving at another is closed along it, and a
//! face holding the pole within it is closed by the whole of the line, as a
//! cylinder's band is by its lower circle. Nothing ever keeps a stretch of
//! a floor as an edge.

use std::f64::consts::{PI, TAU};

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
    let cut = wraps(&loops);
    let mut closed = loops;
    for pole in poles {
        closed = closed
            .into_iter()
            .map(|lap| {
                let joins = joins_round(&lap, pole);
                interleaved(lap, joins)
            })
            .collect();
    }
    let turns = (closed.iter().map(|lap| turning(lap)).sum::<f64>() / TAU).round();
    for (pole, held) in poles.iter().zip(held) {
        if held {
            closed.push(whole_round(pole, turns, false));
        }
    }
    closed.extend(cut.into_iter().map(|floor| vec![floor]));
    closed
}

/// The stretches of a pole's floor the rounds' loops are closed along,
/// each the way the loop runs: a loop keeps its face on its left, so a
/// trace arriving at the low pole leaves the face on the side the angle
/// grows towards, and the join runs that way to the trace leaving, however
/// far round; at the high pole the other way. Two meridians exactly half a
/// turn apart, a ball's at a half turn, are told apart so, where the
/// nearest way round is a tie.
fn joins_round(lap: &[Trace], pole: &Pole) -> Vec<Option<Trace>> {
    let count = lap.len();
    (0..count)
        .map(|rank| {
            let (from, next) = (lap[rank].end(), lap[(rank + 1) % count].start());
            if !(pole.reached(from) && pole.reached(next)) {
                return None;
            }
            let gap = (next.x - from.x).rem_euclid(TAU);
            let gap = if gap >= TAU - pole.room { 0.0 } else { gap };
            let along = if gap == 0.0 {
                0.0
            } else if pole.high {
                gap - TAU
            } else {
                gap
            };
            Some(Trace::Segment {
                from,
                to: DVec2::new(from.x + along, next.y),
            })
        })
        .collect()
}

/// The two stretches of the chart's cut that close a face of a torus whose
/// loops wind round the tube — a ring turned part of the way, bounded by
/// two whole meridians and nothing else: the loop running the angle along
/// the meridian up keeps the face on the side where the angle about the
/// axis falls, the one running it down on the other, and between the two
/// the face reaches the cut at the top and at the bottom of the window.
/// None where no loop winds so, or more than two do.
fn wraps(loops: &[Vec<Trace>]) -> Vec<Trace> {
    let winding: Vec<(f64, f64, bool)> = loops
        .iter()
        .filter_map(|lap| match lap[..] {
            [Trace::Segment { from, to }] if from.x == to.x && (to.y - from.y).abs() > PI => {
                Some((from.x, from.y.min(to.y), to.y > from.y))
            }
            _ => None,
        })
        .collect();
    let [(one, low, one_up), (other, _, other_up)] = winding[..] else {
        return Vec::new();
    };
    if one_up == other_up {
        return Vec::new();
    }
    let (right, left) = if one_up { (one, other) } else { (other, one) };
    let left = right - (right - left).rem_euclid(TAU);
    vec![
        Trace::Segment {
            from: DVec2::new(right, low + TAU),
            to: DVec2::new(left, low + TAU),
        },
        Trace::Segment {
            from: DVec2::new(left, low),
            to: DVec2::new(right, low),
        },
    ]
}

/// The stretches of the floors alone that close a face on a sphere or a
/// torus, each laid exactly on its pole's line: what a loop through a pole
/// is closed along, and the whole floor of each pole the face holds, which
/// the face's flux integrates its 1-form over.
pub(in crate::brep) fn floors_round(
    poles: &[Pole],
    loops: &[Vec<Trace>],
    flipped: bool,
) -> Vec<Trace> {
    let held: Vec<bool> = poles.iter().map(|pole| holds(loops, pole)).collect();
    let mut floors = wraps(loops);
    let mut closed = loops.to_vec();
    for pole in poles {
        closed = closed
            .into_iter()
            .map(|lap| {
                let joins = joins_round(&lap, pole);
                floors.extend(joins.iter().flatten().map(|floor| match *floor {
                    Trace::Segment { from, to } => Trace::Segment {
                        from: DVec2::new(from.x, pole.at),
                        to: DVec2::new(to.x, pole.at),
                    },
                    Trace::Round { .. } | Trace::Graph { .. } | Trace::Conic { .. } => *floor,
                }));
                interleaved(lap, joins)
            })
            .collect();
    }
    let turns = (closed.iter().map(|lap| turning(lap)).sum::<f64>() / TAU).round();
    for (pole, held) in poles.iter().zip(held) {
        if held {
            floors.extend(whole_round(pole, turns, flipped));
        }
    }
    floors
}

/// The whole floor of a pole a face holds within it: turning back against
/// the loops where they wind round the axis, and where they wind round
/// nothing, keeping the face on its left seen from outside the matter —
/// the high pole's floor runs the angle down, the low pole's up, the other
/// way round on a face `flipped`, whose matter lies on the side its
/// surface's normal points to.
fn whole_round(pole: &Pole, turns: f64, flipped: bool) -> Vec<Trace> {
    let against = if pole.high == flipped { -1.0 } else { 1.0 };
    whole(pole.at, if turns == 0.0 { against } else { turns })
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
fn interleaved(lap: Vec<Trace>, floors: Vec<Option<Trace>>) -> Vec<Trace> {
    lap.into_iter()
        .zip(floors)
        .flat_map(|(trace, floor)| std::iter::once(trace).chain(floor))
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
