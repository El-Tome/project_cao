//! Which way each loop turns: a face on its left seen from outside the
//! matter.
//!
//! Nothing at a single edge can tell: two faces sharing an edge and run
//! opposite ways along it describe matter on one side of the pair or on the
//! other, and both read the same from the edge alone. What tells is the area a
//! loop sweeps in its surface's own parameters, signed as its surface's normal
//! turns it — positive round the outside of a face, negative round a hole.
//!
//! Every place of a listing may stand `room` off where its geometry says: an
//! edge off its surface, its end off its vertex. An edge off its surface
//! sweeps `room` times its length; an edge ending off the next one's start
//! leaves the area hanging on the place it is read about, by the gap times
//! how far it stands from that place — a place of the face, not the origin.
//! A loop sweeping less than both — a sliver between a side and a circle
//! tangent to it — turns neither way that can be read, and is not judged.

mod sweep;

use std::f64::consts::TAU;

use glam::DVec3;

use super::Mislisted;
use super::lying::point;
use super::nappe::Nappe;
use super::round::Tube;
use crate::brep::{Curve, ListedEdge, Listing, Surface};
use sweep::sweep;

/// How many chords a curve with no short formula is swept along.
const CHORDS: usize = 64;

/// Every face turns its outside loop so that it keeps the face on its left
/// seen from outside the matter, and its holes the other way.
///
/// On a plane, the loop sweeping the most area is the outside one. On a
/// cylinder, a face may instead go all the way round, between two loops that
/// each wind round the axis once, opposite ways: the area between them is
/// what has to come out positive, and every loop that winds round nothing is
/// a hole. On a cone, a face holding its apex within it is bounded by loops
/// winding once round the axis in all, closed by the floor the apex unrolls
/// into; a loop passing through the apex is closed along that floor too,
/// and where the others wind once round it, the rest of the floor closes
/// the face as for an apex held within it.
pub(super) fn turning(listing: &Listing, room: f64) -> Result<(), Mislisted> {
    for (face, listed) in listing.faces.iter().enumerate() {
        let side = if listed.outward { 1.0 } else { -1.0 };
        let (nappe, tube, unrolled) = match &listed.surface {
            Surface::Plane(_) => (None, None, 1.0),
            Surface::Cylinder(cylinder) => (None, None, cylinder.radius),
            Surface::Elliptic(wall) => (None, None, wall.minor),
            Surface::Cone(cone) => {
                let nappe = Nappe::of(cone);
                let widest = widest(listing, &listed.loops, |place| nappe.radius(place));
                (Some(nappe), None, widest)
            }
            Surface::Sphere(sphere) => {
                let tube = Tube::of_sphere(sphere);
                let widest = widest(listing, &listed.loops, |place| tube.radius(place));
                (None, Some(tube), widest)
            }
            Surface::Torus(torus) => {
                let tube = Tube::of_torus(torus);
                let widest = widest(listing, &listed.loops, |place| tube.radius(place));
                (None, Some(tube), widest)
            }
        };
        let about = listed
            .loops
            .iter()
            .flatten()
            .next()
            .map_or(DVec3::ZERO, |&(edge, _)| {
                let first = &listing.edges[edge];
                point(&first.curve, first.from)
            });
        let swept: Vec<(f64, f64)> = listed
            .loops
            .iter()
            .map(|uses| {
                let (mut area, mut turns) = uses
                    .iter()
                    .map(|&(edge, forward)| {
                        let (area, angle) =
                            sweep(&listed.surface, &listing.edges[edge], about, room);
                        if forward {
                            (area, angle)
                        } else {
                            (-area, -angle)
                        }
                    })
                    .fold((0.0, 0.0), |(area, angle), (more, further)| {
                        (area + more, angle + further)
                    });
                if let Some(nappe) = &nappe
                    && passes(listing, uses, nappe.apex(), room)
                {
                    area += (nappe.apex_length() - nappe.length(about)) * turns;
                    turns = 0.0;
                }
                if let Some(tube) = &tube
                    && let Some(pole) = tube
                        .poles()
                        .into_iter()
                        .find(|pole| passes(listing, uses, *pole, room))
                {
                    area += tube.rise(tube.phi(about), tube.phi(pole)) * turns;
                    turns = 0.0;
                }
                (area * side, (turns / TAU).round() * side)
            })
            .collect();
        let blurred: Vec<f64> = listed
            .loops
            .iter()
            .map(|uses| blur(listing, uses, about, room) / unrolled)
            .collect();
        let backwards = |lap: usize| Err(Mislisted::Backwards { face, lap });

        let round: Vec<usize> = (0..swept.len())
            .filter(|&lap| swept[lap].1 != 0.0)
            .collect();
        let winding: f64 = round.iter().map(|&lap| swept[lap].1).sum();
        let floor = match (&nappe, &tube) {
            (Some(nappe), _) => {
                let passed = listed
                    .loops
                    .iter()
                    .any(|uses| passes(listing, uses, nappe.apex(), room));
                let closed = !listed.poles.is_empty() || passed && winding.abs() == 1.0;
                floor(nappe, winding, closed, about).ok_or(Mislisted::Pointed { face })?
            }
            (None, Some(tube)) => {
                let poles: Vec<DVec3> = listed
                    .poles
                    .iter()
                    .map(|&vertex| listing.vertices[vertex])
                    .collect();
                round_floor(tube, &poles, winding, about).ok_or(Mislisted::Pointed { face })?
            }
            (None, None) => (0.0, 0.0),
        };
        let outside = if let Some(&first) = round.first() {
            let (area, turns) = round.iter().fold(floor, |(area, turns), &lap| {
                (area + swept[lap].0, turns + swept[lap].1)
            });
            let blur: f64 = round.iter().map(|&lap| blurred[lap]).sum();
            if turns != 0.0 || area <= -blur {
                return backwards(first);
            }
            None
        } else {
            let widest = (0..swept.len())
                .max_by(|&one, &other| swept[one].0.abs().total_cmp(&swept[other].0.abs()));
            if let Some(widest) = widest
                && swept[widest].0 <= -blurred[widest]
            {
                return backwards(widest);
            }
            widest
        };
        for (lap, &(area, turns)) in swept.iter().enumerate() {
            let slit = is_slit(&listed.loops[lap]);
            if turns == 0.0 && Some(lap) != outside && area >= blurred[lap] && !slit {
                return backwards(lap);
            }
        }
    }
    Ok(())
}

/// What the floor the apex unrolls into sweeps, read from `about`, closing a
/// cone face whose round loops wind `winding` times round the axis, its
/// side taken: once round, turned back the other way, where the face holds
/// its apex; nothing where it winds round nothing and holds none; and no
/// reading at all for a face winding once that holds no apex, or holding
/// one it does not wind round.
fn floor(nappe: &Nappe, winding: f64, holds: bool, about: DVec3) -> Option<(f64, f64)> {
    let pointed = winding.abs() == 1.0;
    if pointed != holds {
        return None;
    }
    if !pointed {
        return Some((0.0, 0.0));
    }
    let below = nappe.apex_length() - nappe.length(about);
    Some((below * TAU * winding, -winding))
}

/// What the floor of a pole a sphere or a torus face holds sweeps, read from
/// `about`, closing the face whose round loops wind `winding` times round
/// the axis: once round, turned back the other way at the one pole the
/// face holds; nothing where it winds round nothing and holds none, or
/// both, as a ball does; and no reading at all for a face winding once that
/// holds no pole or two, or holding one while winding round nothing.
fn round_floor(tube: &Tube, poles: &[DVec3], winding: f64, about: DVec3) -> Option<(f64, f64)> {
    match (winding.abs() == 1.0, poles) {
        (true, [pole]) => {
            let below = tube.rise(tube.phi(about), tube.phi(*pole));
            Some((below * TAU * winding, -winding))
        }
        (false, [] | [_, _]) => Some((0.0, 0.0)),
        _ => None,
    }
}

/// The largest distance from the axis, as `radius` reads it, a face's edges
/// reach, which unrolls the room into the surface's parameters no wider
/// than it is anywhere.
fn widest(listing: &Listing, loops: &[Vec<(usize, bool)>], radius: impl Fn(DVec3) -> f64) -> f64 {
    loops
        .iter()
        .flatten()
        .flat_map(|&(edge, _)| {
            let edge = &listing.edges[edge];
            (0..=CHORDS).map(|piece| {
                let at = edge.from + (edge.to - edge.from) * piece as f64 / CHORDS as f64;
                point(&edge.curve, at)
            })
        })
        .map(radius)
        .fold(0.0, f64::max)
}

/// Whether a loop passes through a place: an edge of it ending within
/// `room` of it.
fn passes(listing: &Listing, uses: &[(usize, bool)], place: DVec3, room: f64) -> bool {
    uses.iter().any(|&(edge, _)| {
        let edge = &listing.edges[edge];
        [edge.from, edge.to]
            .into_iter()
            .any(|at| point(&edge.curve, at).distance(place) <= room)
    })
}

/// What a loop sweeps unread: `room` along its length, and at each gap
/// between the end of an edge and the start of the next, the gap times how
/// far it stands from `about`.
fn blur(listing: &Listing, uses: &[(usize, bool)], about: DVec3, room: f64) -> f64 {
    let ends = |&(edge, forward): &(usize, bool)| {
        let listed = &listing.edges[edge];
        let [start, end] = [listed.from, listed.to].map(|at| point(&listed.curve, at));
        if forward { [start, end] } else { [end, start] }
    };
    let long: f64 = uses
        .iter()
        .map(|&(edge, _)| length(&listing.edges[edge]))
        .sum();
    let gaps: f64 = uses
        .iter()
        .zip(uses.iter().cycle().skip(1))
        .map(|(one, next)| {
            let ([_, end], [start, _]) = (ends(one), ends(next));
            let gap = end.distance(start);
            gap * (end.distance(about) + gap)
        })
        .sum();
    room * long + gaps
}

/// How long an edge runs, along chords where it has no short formula.
fn length(edge: &ListedEdge) -> f64 {
    match edge.curve {
        Curve::Line(_) => (edge.to - edge.from).abs(),
        Curve::Circle(circle) => circle.radius * (edge.to - edge.from).abs(),
        Curve::Meet(_) | Curve::Conic(_) => (0..CHORDS)
            .map(|piece| {
                let at = |piece: usize| {
                    point(
                        &edge.curve,
                        edge.from + (edge.to - edge.from) * piece as f64 / CHORDS as f64,
                    )
                };
                at(piece).distance(at(piece + 1))
            })
            .sum(),
    }
}

/// Whether a loop runs each of its edges once each way: a slit, where a face
/// is touched along a line inside it, which bounds no area and turns neither
/// way.
fn is_slit(uses: &[(usize, bool)]) -> bool {
    uses.iter().all(|&(edge, forward)| {
        uses.iter()
            .filter(|&&(other, _)| other == edge)
            .map(|&(_, way)| if way == forward { 1 } else { -1 })
            .sum::<i32>()
            == 0
    })
}
