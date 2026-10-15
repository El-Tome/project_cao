//! Two parallel walls standing within twice a chord's sag of each other, one
//! inside the other or crossing it by no more, sampled on common rays.

mod sampled;

use std::collections::BTreeMap;
use std::f64::consts::TAU;

use glam::DVec3;

use super::facing::Facing;
use super::{Contact, Wall};
use crate::brep::surface::Plane;
use crate::brep::topology::SurfaceId;
pub(super) use sampled::sampled;

/// How many times rays are passed on between walls close to each other: a
/// ray one wall takes from a second reaches a third close to the first in
/// the next round, down a chain of walls each touching the next inside it.
const ROUNDS: usize = 4;

/// What every wall close to another takes from the others, and the steps of
/// its grid it leaves out, starting from the rays each took on its own: in
/// rounds, each pair of `close` walls sharing what both hold, until no wall
/// holds more. A ray a wall already holds, to within the kernel's tolerance
/// along its circle, is not taken twice; nor one that falls where it stands
/// closer to any of its partners than their room, as a ray of its own
/// there is left out — else a partner it passed its grid on to would pass a
/// step it withholds back to it. But a ray `pinned` to a wall, through a
/// vertex lying on it, it keeps wherever it falls: a ruling leaves the wall
/// there, and the strip beside it, a skin a hair high under a cap, needs a
/// sample on every rim at that angle or its triangle lies flat on the cap.
/// `planes` holds, for each wall, the planes touching it along a line.
pub(super) fn shared(
    close: &[(&Wall, &Wall, &Facing)],
    mut taken: BTreeMap<SurfaceId, Vec<DVec3>>,
    pinned: &BTreeMap<SurfaceId, Vec<DVec3>>,
    planes: &BTreeMap<SurfaceId, Vec<Plane>>,
    tolerance: f64,
    eps: f64,
) -> BTreeMap<SurfaceId, Contact> {
    let pins = |wall: &Wall| pinned.get(&wall.0).map_or(&[][..], Vec::as_slice);
    let touching = |wall: &Wall| planes.get(&wall.0).map_or(&[][..], Vec::as_slice);
    let mut withheld: BTreeMap<SurfaceId, Vec<(usize, [f64; 2])>> = BTreeMap::new();
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
    let hubs = hubs(close);
    for _ in 0..ROUNDS {
        let mut received: BTreeMap<SurfaceId, Vec<DVec3>> = BTreeMap::new();
        let mut dropped: BTreeMap<SurfaceId, Vec<usize>> = BTreeMap::new();
        for (&(outer, inner, facing), hub) in close.iter().zip(&hubs) {
            let held = |wall: &Wall| taken.get(&wall.0).map_or(&[][..], Vec::as_slice);
            let Some(together) = sampled(
                (outer, held(outer)),
                (inner, held(inner)),
                facing,
                *hub,
                [touching(outer), touching(inner)],
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
                .filter(|(rank, way)| !off.contains(rank) || pins(wall).contains(way))
                .map(|(_, way)| way)
                .chain(fresh)
                .filter(|way| {
                    pins(wall).contains(way)
                        || partners(wall).all(|(_, facing)| !beside(wall, facing, *way))
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

/// For each pair of `close` walls, the axis their common rays are drawn
/// from: the smallest wall's of all those close to one another, one through
/// the next, where it stands inside every one of them; otherwise the inner
/// wall's of the pair.
///
/// Three walls touching along one line are three pairs, and each drawing
/// its rays from its own inner axis, a ray passed round them comes back to
/// the first a hair round from where it left: every round of passing adds
/// rays, and the last round's never reach a wall's other partners — a
/// circle printed round one of the three, touching it inside, is left
/// without the rays its partner took last, and their chords cross. Drawn
/// from one axis, a ray is the same ray on every wall, and the passing
/// ends.
fn hubs(close: &[(&Wall, &Wall, &Facing)]) -> Vec<Option<DVec3>> {
    let mut cluster: BTreeMap<SurfaceId, usize> = BTreeMap::new();
    for (rank, (outer, inner, _)) in close.iter().enumerate() {
        for wall in [outer, inner] {
            cluster.entry(wall.0).or_insert(rank);
        }
    }
    loop {
        let mut joined = false;
        for (outer, inner, _) in close {
            let least = cluster[&outer.0].min(cluster[&inner.0]);
            for wall in [outer, inner] {
                let label = cluster.get_mut(&wall.0).expect("every wall is labelled");
                if *label != least {
                    let old = *label;
                    for other in cluster.values_mut() {
                        if *other == old {
                            *other = least;
                        }
                    }
                    joined = true;
                }
            }
        }
        if !joined {
            break;
        }
    }
    let cluster = &cluster;
    let members = |label: usize| {
        close
            .iter()
            .flat_map(|(outer, inner, _)| [*outer, *inner])
            .filter(move |wall| cluster[&wall.0] == label)
    };
    close
        .iter()
        .map(|(outer, _, _)| {
            let label = cluster[&outer.0];
            let smallest =
                members(label).min_by(|one, other| one.1.radius.total_cmp(&other.1.radius))?;
            let hub = smallest.1.origin;
            members(label)
                .all(|(_, wall)| {
                    let from = hub - wall.origin;
                    (from - wall.axis * wall.axis.dot(from)).length() < wall.radius
                })
                .then_some(hub)
        })
        .collect()
}

/// Whether a wall stands along `way` from its axis closer to another than
/// their room, the two face to face there: no ray it takes there, whichever
/// partner passed it on.
fn beside((_, wall): &Wall, facing: &Facing, way: DVec3) -> bool {
    let point = wall.origin + way * wall.radius;
    facing.crowds(wall, point) && facing.at(point)
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
