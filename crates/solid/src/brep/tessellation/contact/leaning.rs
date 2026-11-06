//! Two walls leaning a hair off parallel, face to face, sampled on one set
//! of rays (#706).
//!
//! Each sampled on its own grid, a hair turned from the other's, their
//! triangles cross where the sweep orders their samples differently. Turned
//! onto each other by the rotation taking one axis onto the other, each
//! takes the other's grid and the rays the other took on its own, so that
//! both are cut on the same rays. Read from each wall's own axis, a ray
//! drifts by up to how far apart the axes stand where the walls hold their
//! circles: two rays of the pair within that drift are one, and which of
//! the two stays is decided by one rule for the whole pair.
//!
//! Only for two walls whose axes cross, or all but, where they hold their
//! circles. A small wall touching a large one inside, its axis far off the
//! other's, would be cut along rays from its own axis, as two parallel ones
//! are; but sampled on them alone, without the steps withheld beside the
//! line they touch along nor that line taken, a cap's rim and the conic the
//! small wall leaves on it, each a hair off those rays where the two all but
//! meet, cross there, and the cap is left uncut (352000000014902).

use std::collections::BTreeMap;
use std::f64::consts::{PI, TAU};

use glam::{DQuat, DVec3};

use super::facing::Facing;
use crate::brep::tessellation::sampling::divisions;
use crate::brep::topology::SurfaceId;

/// What a ray of the pair stands for, in the order one gives way to another
/// within the drift: a ray a wall took on its own, which is never given up;
/// a step of a wall's grid, the first wall's before the second's; a ray
/// through a point standing just inside a wall, which gives way to any ray
/// of either.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    Own,
    Step(usize),
    Beneath,
}

/// A ray of the pair: the wall it comes from, what it stands for, and the
/// way it runs from either axis.
struct Ray {
    side: usize,
    kind: Kind,
    ways: [DVec3; 2],
}

/// Gives each wall of every pair in `leaning` the rays of the other, turned
/// onto its own axis, two rays within the pair's drift one; `own` holds the
/// rays each wall took on its own, `beneath` those of them through a point
/// standing just inside it. Returns the steps of each wall's grid that gave
/// way to a ray of the other.
pub(super) fn shared(
    leaning: &[&Facing],
    own: &mut BTreeMap<SurfaceId, Vec<DVec3>>,
    beneath: &BTreeMap<SurfaceId, Vec<DVec3>>,
    tolerance: f64,
) -> BTreeMap<SurfaceId, Vec<usize>> {
    let mut withheld: BTreeMap<SurfaceId, Vec<usize>> = BTreeMap::new();
    for facing in leaning {
        let walls = facing.walls();
        let [nearest, farthest] = facing.apart();
        if nearest > farthest - nearest {
            continue;
        }
        let turn = DQuat::from_rotation_arc(walls[1].1.axis, walls[0].1.axis);
        let onto = |side: usize, way: DVec3| match side {
            0 => [way, turn.inverse() * way],
            _ => [turn * way, way],
        };
        let mut rays = Vec::new();
        for (side, (id, wall)) in walls.into_iter().enumerate() {
            let steps = divisions(wall.radius, tolerance);
            let gone = withheld.get(&id).map_or(&[][..], Vec::as_slice);
            rays.extend(
                (0..steps)
                    .filter(|step| !gone.contains(step))
                    .map(|step| Ray {
                        side,
                        kind: Kind::Step(step),
                        ways: onto(side, wall.radial(TAU * step as f64 / steps as f64)),
                    }),
            );
            let inside = beneath.get(&id).map_or(&[][..], Vec::as_slice);
            rays.extend(own.get(&id).into_iter().flatten().map(|way| Ray {
                side,
                kind: if inside.contains(way) {
                    Kind::Beneath
                } else {
                    Kind::Own
                },
                ways: onto(side, *way),
            }));
        }
        rays.sort_by_key(|ray| (ray.kind.rank(), ray.side));
        let frame = walls[0].1;
        let angle = |ray: &Ray| ray.ways[0].dot(frame.v).atan2(ray.ways[0].dot(frame.u));
        let radius = walls[0].1.radius.min(walls[1].1.radius);
        let sine = walls[0].1.axis.cross(walls[1].1.axis).length();
        let drift = farthest / radius + sine;
        let mut kept: Vec<Ray> = Vec::with_capacity(rays.len());
        for ray in rays {
            let at = angle(&ray);
            let gives_way = ray.kind != Kind::Own
                && kept.iter().any(|other| {
                    (other.side != ray.side || ray.kind == Kind::Beneath)
                        && ((at - angle(other) + PI).rem_euclid(TAU) - PI).abs() <= drift
                });
            if !gives_way {
                kept.push(ray);
            } else if let Kind::Step(step) = ray.kind {
                withheld.entry(walls[ray.side].0).or_default().push(step);
            }
        }
        for (side, (id, _)) in walls.into_iter().enumerate() {
            let taken = kept
                .iter()
                .filter(|ray| !(ray.side == side && matches!(ray.kind, Kind::Step(_))))
                .map(|ray| ray.ways[side])
                .collect();
            own.insert(id, taken);
        }
    }
    withheld
}

impl Kind {
    fn rank(self) -> usize {
        match self {
            Kind::Own => 0,
            Kind::Step(_) => 1,
            Kind::Beneath => 2,
        }
    }
}
