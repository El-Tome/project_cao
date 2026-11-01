//! The floors of a sphere's or a torus's chart (#576): its poles as the
//! lines they are read as, which a face holds within it by parity, the
//! stretches of them a loop through a pole is closed along, and the cut of
//! a ring's chart a face winding round the tube is closed along.

use std::f64::consts::TAU;

use glam::{DVec2, DVec3};

mod ring;

use super::super::crossing::heights;
use super::{interleaved, turning, whole};
use crate::brep::surface::Surface;
use crate::brep::trace::Trace;
use ring::{closing, cut};

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

/// A face's loops closed along the floors of a sphere or a torus: at each
/// pole as a cone's are at its apex, and by the whole floor of each pole the
/// face holds within it, which a ray up the meridian from inside the face
/// crosses an odd number of times in all; the loops of a `flipped` face run
/// the other way round in the chart. A face with no loop on a ring, which
/// has no pole, is the whole window of the chart, closed along the cut the
/// chart is centred on `seam` away from, and a face reaching across the cut
/// is closed along what it reaches of it ([`closing`]).
pub(in crate::brep::domain) fn closed_round(
    poles: &[Pole],
    loops: Vec<Vec<Trace>>,
    flipped: bool,
    seam: Option<f64>,
) -> Vec<Vec<Trace>> {
    if loops.is_empty() && poles.is_empty() {
        return seam.map(|seam| cut(seam, TAU, 0.0)).into_iter().collect();
    }
    let held: Vec<bool> = poles
        .iter()
        .map(|pole| holds(&loops, pole, flipped))
        .collect();
    let cuts = if poles.is_empty() {
        closing(&loops, seam, flipped)
    } else {
        Vec::new()
    };
    let mut closed = loops;
    for pole in poles {
        closed = closed
            .into_iter()
            .map(|lap| {
                let joins = joins_round(&lap, pole, flipped);
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
    closed.extend(cuts);
    closed
}

/// The angle a torus's chart is centred on, whose antipode the chart is cut
/// at: none for a sphere, whose chart ends at its poles.
pub(in crate::brep) fn seam(surface: &Surface) -> Option<f64> {
    match surface {
        Surface::Torus(torus) => Some(torus.seam),
        Surface::Sphere(_)
        | Surface::Plane(_)
        | Surface::Cylinder(_)
        | Surface::Cone(_)
        | Surface::Elliptic(_) => None,
    }
}

/// The stretches of a pole's floor the rounds' loops are closed along,
/// each the way the loop runs: a loop keeps its face on its left seen from
/// outside the matter, so on a face whose surface's normal points out a
/// trace arriving at the low pole leaves the face on the side the angle
/// grows towards, and the join runs that way to the trace leaving, however
/// far round; at the high pole the other way, and on a `flipped` face,
/// whose loops run the other way round in the chart, the other way again.
/// Two meridians exactly half a turn apart, a ball's at a half turn, are
/// told apart so, where the nearest way round is a tie.
fn joins_round(lap: &[Trace], pole: &Pole, flipped: bool) -> Vec<Option<Trace>> {
    let count = lap.len();
    let down = pole.high != flipped;
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
            } else if down {
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

/// The stretches of the floors alone that close a face on a sphere or a
/// torus, each laid exactly on its pole's line: what a loop through a pole
/// is closed along, and the whole floor of each pole the face holds, which
/// the face's flux integrates its 1-form over.
pub(in crate::brep) fn floors_round(
    poles: &[Pole],
    loops: &[Vec<Trace>],
    flipped: bool,
    seam: Option<f64>,
) -> Vec<Trace> {
    let held: Vec<bool> = poles
        .iter()
        .map(|pole| holds(loops, pole, flipped))
        .collect();
    let mut floors: Vec<Trace> = if poles.is_empty() {
        closing(loops, seam, flipped)
            .into_iter()
            .flatten()
            .collect()
    } else {
        Vec::new()
    };
    let mut closed = loops.to_vec();
    for pole in poles {
        closed = closed
            .into_iter()
            .map(|lap| {
                let joins = joins_round(&lap, pole, flipped);
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
/// axis, which keeps the face on its left seen from outside the matter: the
/// face lies above it where it runs forwards and its surface's normal points
/// out, below it on a `flipped` face, and the pole is inside where the other
/// traces are crossed an even number of times on the way. A face with no
/// loop holds every pole; one bounded by meridians alone holds none.
pub(in crate::brep) fn holds(loops: &[Vec<Trace>], pole: &Pole, flipped: bool) -> bool {
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
    let above = (pace.x > 0.0) != flipped;
    above == pole.high && crossings % 2 == 0 || above != pole.high && crossings % 2 == 1
}

/// Under this share of its pace, a trace runs along the meridian, and tells
/// nothing of which side the face lies on.
const ACROSS: f64 = 1e-9;
