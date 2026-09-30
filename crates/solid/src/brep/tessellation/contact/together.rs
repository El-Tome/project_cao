//! Two parallel walls standing within twice a chord's sag of each other, one
//! inside the other or crossing it by no more, sampled on common rays.

use std::f64::consts::{PI, TAU};

use glam::DVec3;

use super::super::sampling::divisions;
use super::{APART, Contact, Wall};

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
        if (from_axis.length() - inner.radius).abs() >= eps * APART {
            on_inner.push(from_axis.normalize());
        } else {
            leave(0, from);
        }
    }
    let mut on_outer = Vec::new();
    for (way, from) in from_inner {
        let along = offset.dot(way);
        let reach = -along + (along * along - outside).sqrt();
        if (reach - inner.radius).abs() >= eps * APART {
            on_outer.push((offset + way * reach).normalize());
        } else {
            leave(1, from);
        }
    }
    shared.contacts[0].rays = on_outer;
    shared.contacts[1].rays = on_inner;
    Some(shared)
}
