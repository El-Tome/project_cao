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

use std::f64::consts::TAU;

use glam::DVec3;

use super::Mislisted;
use super::lying::point;
use super::nappe::Nappe;
use crate::brep::{Circle, Curve, ListedEdge, Listing, Surface};

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
        let (nappe, unrolled) = match &listed.surface {
            Surface::Plane(_) => (None, 1.0),
            Surface::Cylinder(cylinder) => (None, cylinder.radius),
            Surface::Cone(cone) => {
                let nappe = Nappe::of(cone);
                let widest = widest(listing, &listed.loops, &nappe);
                (Some(nappe), widest)
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
        let floor = match &nappe {
            Some(nappe) => {
                let passed = listed
                    .loops
                    .iter()
                    .any(|uses| passes(listing, uses, nappe.apex(), room));
                let closed = listed.apex.is_some() || passed && winding.abs() == 1.0;
                floor(nappe, winding, closed, about).ok_or(Mislisted::Pointed { face })?
            }
            None => (0.0, 0.0),
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

/// The largest distance from a cone's axis a face's edges reach, which
/// unrolls the room into the cone's parameters no wider than it is anywhere.
fn widest(listing: &Listing, loops: &[Vec<(usize, bool)>], nappe: &Nappe) -> f64 {
    loops
        .iter()
        .flatten()
        .flat_map(|&(edge, _)| {
            let edge = &listing.edges[edge];
            (0..=CHORDS).map(move |piece| {
                let at = edge.from + (edge.to - edge.from) * piece as f64 / CHORDS as f64;
                nappe.radius(point(&edge.curve, at))
            })
        })
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
/// and on a cylinder or a cone the angle it turns round the axis.
///
/// On a plane the area is half the moment of the edge about `about`, a
/// place of the face, along the plane's normal: exact for a line and for a
/// circle. On a cylinder it is what the edge sweeps under it, down to the
/// height of `about`, as the angle turns: nothing for a ruling, which turns
/// no angle. On a cone it is the same in `(θ, l)`, `l` along the ruling:
/// nothing for a ruling, the length from that of `about` times the angle for
/// a cross-section.
fn sweep(surface: &Surface, edge: &ListedEdge, about: DVec3, room: f64) -> (f64, f64) {
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
        (Surface::Cylinder(_) | Surface::Cone(_), Curve::Line(_)) => (0.0, 0.0),
        (Surface::Cylinder(cylinder), Curve::Circle(circle))
            if section(cylinder.origin, cylinder.axis, &circle, room) =>
        {
            let axis = cylinder.axis.normalize();
            let height = (circle.center - about).dot(axis);
            let angle = (edge.to - edge.from) * axis.dot(circle.u.cross(circle.v)).signum();
            (-height * angle, angle)
        }
        (Surface::Cylinder(cylinder), _) => {
            let axis = cylinder.axis.normalize();
            around(edge, |place| {
                let from = place - cylinder.origin;
                (
                    (place - about).dot(axis),
                    from.dot(cylinder.v).atan2(from.dot(cylinder.u)),
                )
            })
        }
        (Surface::Cone(cone), Curve::Circle(circle))
            if section(cone.origin, cone.axis, &circle, room) =>
        {
            let nappe = Nappe::of(cone);
            let length = nappe.length(point(&edge.curve, edge.from)) - nappe.length(about);
            let angle = (edge.to - edge.from)
                * cone.axis.normalize().dot(circle.u.cross(circle.v)).signum();
            (-length * angle, angle)
        }
        (Surface::Cone(cone), _) => {
            let nappe = Nappe::of(cone);
            let from = nappe.length(about);
            around(edge, |place| {
                (nappe.length(place) - from, nappe.angle(place))
            })
        }
    }
}

/// Whether a circle is a cross-section of a surface about the axis through
/// `origin` along `axis`, its angle the surface's own: square to the axis,
/// about a place of it within `room`. A circle about another axis lies on
/// the surface only over a stretch a hair long, an arc of the surface's
/// taken for it — the rim of a wall all but touching this one, whose angle
/// may run the other way round.
fn section(origin: DVec3, axis: DVec3, circle: &Circle, room: f64) -> bool {
    let axis = axis.normalize();
    let from = circle.center - origin;
    axis.cross(circle.axis).length() <= SQUARE && (from - axis * from.dot(axis)).length() <= room
}

/// How far from square to the axis, in the sine of the angle between the
/// two axes, a circle may stand and still be a cross-section.
const SQUARE: f64 = 1e-9;

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

/// What a curve sweeps under a surface about an axis, along chords in the
/// surface's parameters as `chart` reads them at each place — the height
/// from that of `about`, then the angle — the angle followed across the
/// turn rather than read afresh.
fn around(edge: &ListedEdge, chart: impl Fn(DVec3) -> (f64, f64)) -> (f64, f64) {
    let mut last: Option<(f64, f64)> = None;
    let (mut area, mut angle) = (0.0, 0.0);
    for piece in 0..=CHORDS {
        let at = edge.from + (edge.to - edge.from) * piece as f64 / CHORDS as f64;
        let (height, theta) = chart(point(&edge.curve, at));
        if let Some((before_height, before)) = last {
            let step = (theta - before + TAU / 2.0).rem_euclid(TAU) - TAU / 2.0;
            area -= (height + before_height) / 2.0 * step;
            angle += step;
        }
        last = Some((height, theta));
    }
    (area, angle)
}
