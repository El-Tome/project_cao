//! Two parallel walls standing within twice a chord's sag of each other, one
//! inside the other or crossing it by no more, sampled on common rays.

use std::collections::BTreeMap;
use std::f64::consts::{PI, TAU};

use glam::DVec3;

use super::super::sampling::{divisions, on_a_plane};
use super::facing::Facing;
use super::{Contact, Wall};
use crate::brep::surface::{Cylinder, Plane};
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
/// which the two stand closer than their room neither takes.
///
/// Crossing by a hair, as two walls of one radius a hair apart do, the two
/// bound a sliver thinner than a chord sags on either side of it, as long as
/// half the turn: sampled on common rays, both walls keep their order on
/// each side of the lines they cross along.
///
/// A step of the outer's grid falling, seen from the inner's axis, within
/// the kernel's tolerance along the circle of a step of the inner's — walls
/// of one radius, or all but, a hair off one axis — is one place, and the
/// inner's step is the ray both take there: the outer withholds its own
/// over the heights the two face each other. A circle takes a ray that
/// close to a step of its grid at the step, so each kept on its own grid
/// the two would stand a hair round from each other, off one ray, and the
/// outer's chord from the line they touch along, long where the steps
/// beside it are withheld, would cross the inner's next.
///
/// Nor does either keep a step of its grid at the heights of the other's
/// circles, nor take a ray of its own at all, along which the other stands
/// beside one of its `planes` — a plane
/// touching it along a line — and off that line: the other takes no ray
/// there, lest a strip of its wall lie on the plane's face, and a circle
/// with a place on a ray a circle on the same cap has none on no longer
/// keeps its order with it. A disc a hair inside a slot's end, a hair past
/// where its side runs into its arc, has its rim on the slot's cap a hair
/// from the side's corner, and the arc's first chord from that corner, long,
/// leans in past the rim's place there (8562492). At other heights the ray
/// is kept: left out of a circle with nothing beside it, its chords span two
/// steps and the wall sags past the tolerance (8501966).
pub(super) fn sampled(
    (outer_wall, own_outer): (&Wall, &[DVec3]),
    (inner_wall, own_inner): (&Wall, &[DVec3]),
    facing: &Facing,
    hub: Option<DVec3>,
    planes: [&[Plane]; 2],
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
    let mut leave = |wall: usize, from: From, over: Vec<[f64; 2]>| match from {
        From::Step(step) => shared.contacts[wall]
            .withheld
            .extend(over.into_iter().map(|stretch| (step, stretch))),
        From::Own(rank) => shared.dropped[wall].push(rank),
    };
    let on_a_step = |way: DVec3| {
        let angle = way.dot(inner.v).atan2(way.dot(inner.u));
        let step = TAU / inner_steps as f64;
        let off = angle - (angle / step).round() * step;
        off.abs() <= eps / inner.radius
    };
    let refuses = |side: usize, place: DVec3| {
        let wall = [outer, inner][side];
        planes[side]
            .iter()
            .any(|plane| on_a_plane(plane, wall.origin, wall.axis, place, eps))
    };
    let hub = hub.filter(|hub| *hub != inner.origin);
    let meeting = |wall: &Cylinder, point: DVec3| -> DVec3 {
        let Some(hub) = hub else {
            return point;
        };
        let way = flat(point - hub).normalize();
        let from = flat(hub - wall.origin);
        let along = from.dot(way);
        let reach =
            -along + (along * along - from.length_squared() + wall.radius * wall.radius).sqrt();
        from + way * reach
    };
    let mut on_inner = Vec::new();
    for (point, from) in through_outer {
        let from_axis = match hub {
            Some(_) => meeting(&inner, outer.origin + point),
            None => point - offset,
        };
        let over = if facing.crowds(&outer, outer.origin + point) {
            facing.withheld(outer_wall.0, outer.origin + point)
        } else {
            Vec::new()
        };
        if !over.is_empty() {
            leave(0, from, over);
        } else if matches!(from, From::Step(_)) && on_a_step(from_axis) {
            leave(0, from, facing.over(outer.origin + point));
        } else if refuses(1, inner.origin + from_axis.normalize() * inner.radius) {
            leave(0, from, facing.rims(inner_wall.0));
        } else {
            on_inner.push(from_axis.normalize());
        }
    }
    let mut on_outer = Vec::new();
    for (way, from) in from_inner {
        let along = offset.dot(way);
        let reach = -along + (along * along - outside).sqrt();
        let toward = match hub {
            Some(_) => meeting(&outer, inner.origin + way * inner.radius).normalize(),
            None => (offset + way * reach).normalize(),
        };
        let over = if facing.crowds(&inner, outer.origin + offset + way * inner.radius) {
            facing.withheld(inner_wall.0, outer.origin + offset + way * inner.radius)
        } else {
            Vec::new()
        };
        if !over.is_empty() {
            leave(1, from, over);
        } else if refuses(0, outer.origin + toward * outer.radius) {
            leave(1, from, facing.rims(outer_wall.0));
        } else {
            on_outer.push(toward);
        }
    }
    shared.contacts[0].rays = on_outer;
    shared.contacts[1].rays = on_inner;
    Some(shared)
}
