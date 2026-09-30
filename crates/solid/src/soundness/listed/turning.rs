//! Which way each loop turns: a face on its left seen from outside the
//! matter.
//!
//! Nothing at a single edge can tell: two faces sharing an edge and run
//! opposite ways along it describe matter on one side of the pair or on the
//! other, and both read the same from the edge alone. What tells is the area a
//! loop sweeps in its surface's own parameters, signed as its surface's normal
//! turns it — positive round the outside of a face, negative round a hole.

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
pub(super) fn turning(listing: &Listing) -> Result<(), Mislisted> {
    for (face, listed) in listing.faces.iter().enumerate() {
        let side = if listed.outward { 1.0 } else { -1.0 };
        let swept: Vec<(f64, f64)> = listed
            .loops
            .iter()
            .map(|uses| {
                let (area, turns) = uses
                    .iter()
                    .map(|&(edge, forward)| {
                        let (area, angle) = sweep(&listed.surface, &listing.edges[edge]);
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
        let backwards = |lap: usize| Err(Mislisted::Backwards { face, lap });

        let round: Vec<usize> = (0..swept.len())
            .filter(|&lap| swept[lap].1 != 0.0)
            .collect();
        let outside = if let Some(&first) = round.first() {
            let (area, turns) = round.iter().fold((0.0, 0.0), |(area, turns), &lap| {
                (area + swept[lap].0, turns + swept[lap].1)
            });
            if turns != 0.0 || area <= 0.0 {
                return backwards(first);
            }
            None
        } else {
            let widest = (0..swept.len())
                .max_by(|&one, &other| swept[one].0.abs().total_cmp(&swept[other].0.abs()));
            if let Some(widest) = widest
                && swept[widest].0 <= 0.0
            {
                return backwards(widest);
            }
            widest
        };
        for (lap, &(area, turns)) in swept.iter().enumerate() {
            let slit = is_slit(&listed.loops[lap]);
            if turns == 0.0 && Some(lap) != outside && area >= 0.0 && !slit {
                return backwards(lap);
            }
        }
    }
    Ok(())
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
/// On a plane the area is half the moment of the edge about the plane's
/// origin, along its normal: exact for a line and for a circle. On a
/// cylinder it is what the edge sweeps under it, down to the height nought,
/// as the angle turns: nothing for a ruling, which turns no angle.
fn sweep(surface: &Surface, edge: &ListedEdge) -> (f64, f64) {
    match (surface, edge.curve) {
        (Surface::Plane(plane), Curve::Line(_)) => {
            let [from, to] = [edge.from, edge.to].map(|at| point(&edge.curve, at) - plane.origin);
            (plane.normal.dot(from.cross(to)) / 2.0, 0.0)
        }
        (Surface::Plane(plane), Curve::Circle(circle)) => {
            let chord = point(&edge.curve, edge.to) - point(&edge.curve, edge.from);
            let about = plane
                .normal
                .dot((circle.center - plane.origin).cross(chord));
            let turned = circle.radius
                * circle.radius
                * (edge.to - edge.from)
                * plane.normal.dot(circle.u.cross(circle.v));
            ((about + turned) / 2.0, 0.0)
        }
        (Surface::Plane(plane), _) => chords(edge, |place| {
            let from = place - plane.origin;
            (from.dot(plane.u), from.dot(plane.v))
        }),
        (Surface::Cylinder(_), Curve::Line(_)) => (0.0, 0.0),
        (Surface::Cylinder(cylinder), Curve::Circle(circle)) => {
            let axis = cylinder.axis.normalize();
            let height = (circle.center - cylinder.origin).dot(axis);
            let angle = (edge.to - edge.from) * axis.dot(circle.u.cross(circle.v)).signum();
            (-height * angle, angle)
        }
        (Surface::Cylinder(cylinder), _) => around(cylinder, edge),
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
/// parameters, the angle followed across the turn rather than read afresh.
fn around(cylinder: &Cylinder, edge: &ListedEdge) -> (f64, f64) {
    let axis = cylinder.axis.normalize();
    let mut last: Option<(f64, f64)> = None;
    let (mut area, mut angle) = (0.0, 0.0);
    for piece in 0..=CHORDS {
        let at = edge.from + (edge.to - edge.from) * piece as f64 / CHORDS as f64;
        let from = point(&edge.curve, at) - cylinder.origin;
        let (height, theta) = (
            from.dot(axis),
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
