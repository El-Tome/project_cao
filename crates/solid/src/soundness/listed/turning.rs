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
//! edge off its surface, its end off its vertex. Areas are read about a place
//! of the face, so that what the room sweeps is `room` times the loop's
//! length and times how far its corners stand from that place: a loop
//! sweeping less — a sliver between a side and a circle tangent to it —
//! turns neither way that can be read, and is not judged.

use std::f64::consts::TAU;

use glam::DVec3;

use super::Mislisted;
use super::lying::point;
use crate::brep::{Curve, Cylinder, ListedEdge, Listing, Surface};

/// How many chords a curve with no short formula is swept along.
const CHORDS: usize = 64;

/// Every face turns its outside loop so that it keeps the face on its left
/// seen from outside the matter, and its holes the other way.
///
/// On a plane, the loop sweeping the most area is the outside one. On a
/// cylinder, a face may instead go all the way round, between two loops that
/// each wind round the axis once, opposite ways: the area between them is
/// what has to come out positive, and every loop that winds round nothing is
/// a hole.
pub(super) fn turning(listing: &Listing, room: f64) -> Result<(), Mislisted> {
    for (face, listed) in listing.faces.iter().enumerate() {
        let side = if listed.outward { 1.0 } else { -1.0 };
        let unrolled = match listed.surface {
            Surface::Plane(_) => 1.0,
            Surface::Cylinder(cylinder) => cylinder.radius,
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
                let (area, turns) = uses
                    .iter()
                    .map(|&(edge, forward)| {
                        let (area, angle) = sweep(&listed.surface, &listing.edges[edge], about);
                        if forward {
                            (area, angle)
                        } else {
                            (-area, -angle)
                        }
                    })
                    .fold((0.0, 0.0), |(area, angle), (more, further)| {
                        (area + more, angle + further)
                    });
                (area * side, (turns / TAU).round() * side)
            })
            .collect();
        let blurred: Vec<f64> = listed
            .loops
            .iter()
            .map(|uses| {
                let reach: f64 = uses
                    .iter()
                    .map(|&(edge, _)| {
                        let listed = &listing.edges[edge];
                        let corners: f64 = listed
                            .ends
                            .iter()
                            .flatten()
                            .map(|&vertex| listing.vertices[vertex].distance(about))
                            .sum();
                        length(listed) + corners
                    })
                    .sum();
                room * reach / unrolled
            })
            .collect();
        let backwards = |lap: usize| Err(Mislisted::Backwards { face, lap });

        let round: Vec<usize> = (0..swept.len())
            .filter(|&lap| swept[lap].1 != 0.0)
            .collect();
        let outside = if let Some(&first) = round.first() {
            let (area, turns) = round.iter().fold((0.0, 0.0), |(area, turns), &lap| {
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

/// How long an edge runs, along chords where it has no short formula.
fn length(edge: &ListedEdge) -> f64 {
    match edge.curve {
        Curve::Line(_) => (edge.to - edge.from).abs(),
        Curve::Circle(circle) => circle.radius * (edge.to - edge.from).abs(),
        Curve::Meet(_) => (0..CHORDS)
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

/// The area an edge run along its own way sweeps in a surface's parameters,
/// and on a cylinder the angle it turns round the axis.
///
/// On a plane the area is half the moment of the edge about `about`, a
/// place of the face, along the plane's normal: exact for a line and for a
/// circle. On a cylinder it is what the edge sweeps under it, down to the
/// height of `about`, as the angle turns: nothing for a ruling, which turns
/// no angle.
fn sweep(surface: &Surface, edge: &ListedEdge, about: DVec3) -> (f64, f64) {
    match (surface, edge.curve) {
        (Surface::Plane(plane), Curve::Line(_)) => {
            let [from, to] = [edge.from, edge.to].map(|at| point(&edge.curve, at) - about);
            (plane.normal.dot(from.cross(to)) / 2.0, 0.0)
        }
        (Surface::Plane(plane), Curve::Circle(circle)) => {
            let chord = point(&edge.curve, edge.to) - point(&edge.curve, edge.from);
            let moment = plane.normal.dot((circle.center - about).cross(chord));
            let turned = circle.radius
                * circle.radius
                * (edge.to - edge.from)
                * plane.normal.dot(circle.u.cross(circle.v));
            ((moment + turned) / 2.0, 0.0)
        }
        (Surface::Plane(plane), _) => chords(edge, |place| {
            let from = place - about;
            (from.dot(plane.u), from.dot(plane.v))
        }),
        (Surface::Cylinder(_), Curve::Line(_)) => (0.0, 0.0),
        (Surface::Cylinder(cylinder), Curve::Circle(circle)) => {
            let axis = cylinder.axis.normalize();
            let height = (circle.center - about).dot(axis);
            let angle = (edge.to - edge.from) * axis.dot(circle.u.cross(circle.v)).signum();
            (-height * angle, angle)
        }
        (Surface::Cylinder(cylinder), _) => around(cylinder, edge, about),
    }
}

/// What a curve sweeps on a plane, along chords in the plane's parameters.
fn chords(edge: &ListedEdge, flat: impl Fn(DVec3) -> (f64, f64)) -> (f64, f64) {
    let places: Vec<(f64, f64)> = (0..=CHORDS)
        .map(|piece| {
            let at = edge.from + (edge.to - edge.from) * piece as f64 / CHORDS as f64;
            flat(point(&edge.curve, at))
        })
        .collect();
    let area = places
        .windows(2)
        .map(|pair| (pair[0].0 * pair[1].1 - pair[1].0 * pair[0].1) / 2.0)
        .sum();
    (area, 0.0)
}

/// What a curve sweeps on a cylinder, along chords in the cylinder's
/// parameters, the angle followed across the turn rather than read afresh,
/// the height read from that of `about`.
fn around(cylinder: &Cylinder, edge: &ListedEdge, about: DVec3) -> (f64, f64) {
    let axis = cylinder.axis.normalize();
    let mut last: Option<(f64, f64)> = None;
    let (mut area, mut angle) = (0.0, 0.0);
    for piece in 0..=CHORDS {
        let at = edge.from + (edge.to - edge.from) * piece as f64 / CHORDS as f64;
        let place = point(&edge.curve, at);
        let from = place - cylinder.origin;
        let (height, theta) = (
            (place - about).dot(axis),
            from.dot(cylinder.v).atan2(from.dot(cylinder.u)),
        );
        if let Some((before_height, before)) = last {
            let step = (theta - before + TAU / 2.0).rem_euclid(TAU) - TAU / 2.0;
            area -= (height + before_height) / 2.0 * step;
            angle += step;
        }
        last = Some((height, theta));
    }
    (area, angle)
}
