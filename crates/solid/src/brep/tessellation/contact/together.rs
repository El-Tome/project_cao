//! Two parallel walls standing within twice a chord's sag of each other, one
//! inside the other or crossing it by no more, sampled on common rays.

use std::collections::BTreeMap;
use std::f64::consts::{PI, TAU};

use glam::DVec3;

use super::super::sampling::divisions;
use super::facing::Facing;
use super::{APART, Contact, Wall};
use crate::brep::topology::SurfaceId;

/// How many times rays are passed on between walls close to each other: a
/// ray one wall takes from a second reaches a third close to the first in
/// the next round, down a chain of walls each touching the next inside it.
const ROUNDS: usize = 4;

/// What every wall close to another takes from the others, and the steps of
/// its grid it leaves out, starting from the rays each took on its own: in
/// rounds, each pair of `close` walls sharing what both hold, until no wall
/// holds more. A ray a wall already holds, to within the kernel's tolerance
/// along its circle, is not taken twice; nor one that falls where it stands
/// closer to any of its partners than a fifth of `eps`, as a ray of its own
/// there is left out — else a partner it passed its grid on to would pass a
/// step it withholds back to it.
pub(super) fn shared(
    close: &[(&Wall, &Wall, &Facing)],
    mut taken: BTreeMap<SurfaceId, Vec<DVec3>>,
    tolerance: f64,
    eps: f64,
) -> BTreeMap<SurfaceId, Contact> {
    let mut withheld: BTreeMap<SurfaceId, Vec<usize>> = BTreeMap::new();
    let partners = |(id, _): &Wall| {
        let id = *id;
        close.iter().filter_map(move |&(outer, inner, facing)| {
            if outer.0 == id {
                Some((inner, facing))
            } else if inner.0 == id {
                Some((outer, facing))
            } else {
                None
            }
        })
    };
    for _ in 0..ROUNDS {
        let mut received: BTreeMap<SurfaceId, Vec<DVec3>> = BTreeMap::new();
        let mut dropped: BTreeMap<SurfaceId, Vec<usize>> = BTreeMap::new();
        for &(outer, inner, facing) in close {
            let held = |wall: &Wall| taken.get(&wall.0).map_or(&[][..], Vec::as_slice);
            let Some(together) = sampled(
                (outer, held(outer)),
                (inner, held(inner)),
                facing,
                tolerance,
                eps,
            ) else {
                continue;
            };
            for (wall, contact, off) in [outer, inner]
                .into_iter()
                .zip(together.contacts)
                .zip(together.dropped)
                .map(|((wall, contact), off)| (wall, contact, off))
            {
                received.entry(wall.0).or_default().extend(contact.rays);
                let steps = withheld.entry(wall.0).or_default();
                for step in contact.withheld {
                    if !steps.contains(&step) {
                        steps.push(step);
                    }
                }
                dropped.entry(wall.0).or_default().extend(off);
            }
        }
        let mut grown = false;
        for &wall in close.iter().flat_map(|(outer, inner, _)| [outer, inner]) {
            let off = dropped.remove(&wall.0).unwrap_or_default();
            let Some(fresh) = received.remove(&wall.0) else {
                continue;
            };
            let old = taken.remove(&wall.0).unwrap_or_default();
            let before = old.len();
            let mut rays: Vec<DVec3> = old
                .into_iter()
                .enumerate()
                .filter(|(rank, _)| !off.contains(rank))
                .map(|(_, way)| way)
                .chain(fresh)
                .filter(|way| {
                    partners(wall).all(|(other, facing)| !beside(wall, other, facing, *way, eps))
                })
                .collect();
            once_each(&mut rays, wall, eps);
            grown |= rays.len() != before;
            taken.insert(wall.0, rays);
        }
        if !grown {
            break;
        }
    }
    let mut contacts: BTreeMap<SurfaceId, Contact> = BTreeMap::new();
    for (wall, rays) in taken {
        contacts.entry(wall).or_default().rays = rays;
    }
    for (wall, steps) in withheld {
        contacts.entry(wall).or_default().withheld = steps;
    }
    contacts
}

/// Whether a wall stands along `way` from its axis closer to another than a
/// fifth of `eps`, the two face to face there: no ray it takes there,
/// whichever partner passed it on.
fn beside((_, wall): &Wall, (_, other): &Wall, facing: &Facing, way: DVec3, eps: f64) -> bool {
    let point = wall.origin + way * wall.radius;
    other.distance(point).abs() < eps * APART && facing.at(point)
}

/// The rays of a wall in the order of their angle, one of each: two closer
/// along its circle than the kernel's tolerance are one.
fn once_each(rays: &mut Vec<DVec3>, (_, cylinder): &Wall, eps: f64) {
    let angle = |way: &DVec3| way.dot(cylinder.v).atan2(way.dot(cylinder.u));
    rays.sort_by(|one, other| angle(one).total_cmp(&angle(other)));
    let gap = eps / cylinder.radius;
    rays.dedup_by(|later, earlier| angle(later) - angle(earlier) <= gap);
    if rays.len() > 1 && angle(&rays[0]) + TAU - angle(&rays[rays.len() - 1]) <= gap {
        rays.pop();
    }
}

/// What the outer wall's circles and the inner one's take and leave out, and
/// which of the rays each took on its own the two leave out.
pub(super) struct Together {
    pub(super) contacts: [Contact; 2],
    pub(super) dropped: [Vec<usize>; 2],
}

/// Where a place a ray is drawn through comes from: a step of a wall's grid,
/// or a ray the wall took on its own, by rank.
#[derive(Clone, Copy)]
enum From {
    Step(usize),
    Own(usize),
}

/// What the outer cylinder's circles and the inner one's take and leave out,
/// when the inner stands inside the outer within twice the sag of either's
/// chords from its wall, or crosses it by no more: every ray the one takes
/// the other takes too — its grid's, and those it took on its own, through
/// its vertices and the curves it meets another along — and a ray along
/// which the two stand closer than a fifth of `eps` neither takes.
///
/// Crossing by a hair, as two walls of one radius a hair apart do, the two
/// bound a sliver thinner than a chord sags on either side of it, as long as
/// half the turn: sampled on common rays, both walls keep their order on
/// each side of the lines they cross along.
pub(super) fn sampled(
    (outer_wall, own_outer): (&Wall, &[DVec3]),
    (inner_wall, own_inner): (&Wall, &[DVec3]),
    facing: &Facing,
    tolerance: f64,
    eps: f64,
) -> Option<Together> {
    let (outer, inner) = (outer_wall.1, inner_wall.1);
    let axis = outer.axis;
    let flat = |vector: DVec3| vector - axis * axis.dot(vector);
    let offset = flat(inner.origin - outer.origin);
    if offset.length() <= eps && outer.radius - inner.radius <= eps {
        return None;
    }
    let outside = offset.length_squared() - outer.radius * outer.radius;
    if outside >= 0.0 {
        return None;
    }
    let sag = |radius: f64| radius * (1.0 - (PI / divisions(radius, tolerance) as f64).cos());
    let gap = outer.radius - inner.radius - offset.length();
    if gap.abs() > 2.0 * sag(outer.radius).max(sag(inner.radius)) {
        return None;
    }

    let outer_steps = divisions(outer.radius, tolerance);
    let inner_steps = divisions(inner.radius, tolerance);
    let through_outer = (0..outer_steps)
        .map(|step| {
            let angle = TAU * step as f64 / outer_steps as f64;
            (outer.radial(angle) * outer.radius, From::Step(step))
        })
        .chain(
            own_outer
                .iter()
                .enumerate()
                .map(|(rank, way)| (flat(*way) * outer.radius, From::Own(rank))),
        );
    let from_inner = (0..inner_steps)
        .map(|step| {
            let angle = TAU * step as f64 / inner_steps as f64;
            (inner.radial(angle), From::Step(step))
        })
        .chain(
            own_inner
                .iter()
                .enumerate()
                .map(|(rank, way)| (flat(*way).normalize(), From::Own(rank))),
        );

    let mut shared = Together {
        contacts: [Contact::default(), Contact::default()],
        dropped: [Vec::new(), Vec::new()],
    };
    let mut leave = |wall: usize, from: From| match from {
        From::Step(step) => shared.contacts[wall].withheld.push(step),
        From::Own(rank) => shared.dropped[wall].push(rank),
    };
    let mut on_inner = Vec::new();
    for (point, from) in through_outer {
        let from_axis = point - offset;
        if (from_axis.length() - inner.radius).abs() >= eps * APART
            || !facing.at(outer.origin + point)
        {
            on_inner.push(from_axis.normalize());
        } else {
            leave(0, from);
        }
    }
    let mut on_outer = Vec::new();
    for (way, from) in from_inner {
        let along = offset.dot(way);
        let reach = -along + (along * along - outside).sqrt();
        if (reach - inner.radius).abs() >= eps * APART
            || !facing.at(outer.origin + offset + way * reach)
        {
            on_outer.push((offset + way * reach).normalize());
        } else {
            leave(1, from);
        }
    }
    shared.contacts[0].rays = on_outer;
    shared.contacts[1].rays = on_inner;
    Some(shared)
}
