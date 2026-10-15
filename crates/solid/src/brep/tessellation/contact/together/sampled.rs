//! What the outer cylinder's circles and the inner one's take and leave out,
//! the two standing within twice a chord's sag of each other.

use std::f64::consts::{PI, TAU};

use glam::DVec3;

use crate::brep::surface::{Cylinder, Plane};
use crate::brep::tessellation::contact::facing::Facing;
use crate::brep::tessellation::contact::{Contact, Wall};
use crate::brep::tessellation::sampling::{divisions, on_a_plane};

/// What the outer wall's circles and the inner one's take and leave out, and
/// which of the rays each took on its own the two leave out.
pub(in crate::brep::tessellation::contact) struct Together {
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
pub(in crate::brep::tessellation::contact) fn sampled(
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
