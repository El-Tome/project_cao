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

use super::Wall;
use super::facing::{Facing, Zones};
use super::own::flat_from;
use crate::brep::scale::Scale;
use crate::brep::surface::Cylinder;
use crate::brep::tessellation::sampling::divisions_in;
use crate::brep::topology::SurfaceId;

/// What a ray of a group stands for, in the order one gives way to another
/// within the drift: a ray through a vertex on its wall, never given up —
/// the vertex stands on it there, on every rim — and taken by every other
/// wall the vertex lies on too, so that two rims a hair apart keep in step
/// there; but not by one it does not lie on that has one of its own beside
/// it. Read from two axes, a vertex on both walls stands up to the drift
/// round on each, further than two vertices can stand apart: no angle tells
/// the one from the other, and the vertex itself does. Then a ray a wall
/// took otherwise on its own, through the samples of a curve, giving way to
/// one an earlier wall of the group took: handed on along the curves
/// through a vertex, two such rays part a hair further pass after pass
/// (318277455101196). Then a step of a wall's grid, an earlier wall's
/// before a later one's; then a ray through a point standing just inside a
/// wall, which gives way to any ray of any.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    Vertex,
    Own,
    Step(usize),
    Beneath,
}

/// For each wall, points of the body.
type Points = BTreeMap<SurfaceId, Vec<DVec3>>;

/// A ray of a group: the wall it comes from, by rank, what it stands for,
/// the vertex it runs through if any, and the way it runs from every axis.
struct Ray {
    side: usize,
    kind: Kind,
    vertex: Option<DVec3>,
    ways: Vec<DVec3>,
}

/// Gives each wall leaning a hair off another the rays of every wall it is
/// joined to by pairs in `leaning` that share their rays, one through the
/// next: such a group is read once, every wall's rays turned onto the
/// first wall's axis, two rays within the widest drift of its pairs one.
/// Read pair by pair, a wall took the rays of its last partner alone, and
/// two partners of one wall a hair round from each other (three walls each
/// leaning a hair off the others). `own` holds the rays each wall took on
/// its own, `beneath` those of them through a point standing just inside
/// it, `anchors` the vertices lying on it. Returns the steps of each
/// wall's grid that gave way to a ray of another.
pub(super) fn shared(
    leaning: &[&Facing],
    own: &mut BTreeMap<SurfaceId, Vec<DVec3>>,
    (beneath, anchors): (&Points, &Points),
    (tolerance, reach): (f64, f64),
) -> BTreeMap<SurfaceId, Vec<usize>> {
    let mut withheld: BTreeMap<SurfaceId, Vec<usize>> = BTreeMap::new();
    for (walls, drift) in groups(leaning) {
        let turns: Vec<DQuat> = walls
            .iter()
            .map(|(_, wall)| DQuat::from_rotation_arc(wall.axis, walls[0].1.axis))
            .collect();
        let onto = |side: usize, way: DVec3| -> Vec<DVec3> {
            let first = if side == 0 { way } else { turns[side] * way };
            (0..walls.len())
                .map(|wall| match wall {
                    _ if wall == side => way,
                    0 => first,
                    _ => turns[wall].inverse() * first,
                })
                .collect()
        };
        let held = |id: SurfaceId| anchors.get(&id).map_or(&[][..], Vec::as_slice);
        let mut rays = Vec::new();
        for (side, (id, wall)) in walls.iter().copied().enumerate() {
            let steps = divisions_in(wall.radius, tolerance, reach);
            rays.extend((0..steps).map(|step| Ray {
                side,
                kind: Kind::Step(step),
                vertex: None,
                ways: onto(side, wall.radial(TAU * step as f64 / steps as f64)),
            }));
            let through = |way: &DVec3| {
                held(id)
                    .iter()
                    .find(|point| flat_from(&wall, **point).normalize() == *way)
                    .copied()
            };
            let inside = beneath.get(&id).map_or(&[][..], Vec::as_slice);
            rays.extend(own.get(&id).into_iter().flatten().map(|way| {
                let vertex = through(way);
                Ray {
                    side,
                    kind: if inside.contains(way) {
                        Kind::Beneath
                    } else if vertex.is_some() {
                        Kind::Vertex
                    } else {
                        Kind::Own
                    },
                    vertex,
                    ways: onto(side, *way),
                }
            }));
        }
        rays.sort_by_key(|ray| (ray.kind.rank(), ray.side));
        let frame = walls[0].1;
        let angle = |ray: &Ray| ray.ways[0].dot(frame.v).atan2(ray.ways[0].dot(frame.u));
        let round = |angle: f64| (angle + PI).rem_euclid(TAU) - PI;
        let near = |one: &Ray, other: &Ray| round(angle(one) - angle(other)).abs() <= drift;
        let mut kept: Vec<Ray> = Vec::with_capacity(rays.len());
        for ray in rays {
            let gives_way = kept.iter().any(|other| {
                near(&ray, other)
                    && match ray.kind {
                        Kind::Vertex => false,
                        Kind::Own => {
                            other.side != ray.side && matches!(other.kind, Kind::Own | Kind::Vertex)
                        }
                        Kind::Step(_) => other.side != ray.side,
                        Kind::Beneath => true,
                    }
            });
            if !gives_way {
                kept.push(ray);
            } else if let Kind::Step(step) = ray.kind {
                withheld.entry(walls[ray.side].0).or_default().push(step);
            }
        }
        for (side, (id, _)) in walls.iter().enumerate() {
            let mine = |ray: &Ray| ray.side == side && ray.kind == Kind::Vertex;
            let left_out = |ray: &Ray| {
                ray.vertex
                    .is_some_and(|vertex| !held(*id).contains(&vertex))
                    && kept.iter().any(|other| mine(other) && near(ray, other))
            };
            let taken = kept
                .iter()
                .filter(|ray| ray.side == side || !left_out(ray))
                .filter(|ray| !(ray.side == side && matches!(ray.kind, Kind::Step(_))))
                .map(|ray| ray.ways[side])
                .collect();
            own.insert(*id, taken);
        }
    }
    withheld
}

/// The walls joined by pairs of `leaning` that share their rays, one
/// through the next, each group in the order its walls were first met, with
/// the widest drift of its pairs.
fn groups(leaning: &[&Facing]) -> Vec<(Vec<Wall>, f64)> {
    let mut groups: Vec<(Vec<Wall>, f64)> = Vec::new();
    for facing in leaning {
        let Some(drift) = drift(facing) else {
            continue;
        };
        let pair = facing.walls();
        let holds = |walls: &Vec<Wall>| walls.iter().any(|(id, _)| pair.iter().any(|w| w.0 == *id));
        let mut joined: (Vec<Wall>, f64) = (Vec::new(), drift);
        groups.retain(|group| {
            if holds(&group.0) {
                joined.0.extend(group.0.iter().copied());
                joined.1 = joined.1.max(group.1);
                false
            } else {
                true
            }
        });
        for wall in pair {
            if !joined.0.iter().any(|(id, _)| *id == wall.0) {
                joined.0.push(wall);
            }
        }
        groups.push(joined);
    }
    groups
}

impl Kind {
    fn rank(self) -> usize {
        match self {
            Kind::Vertex => 0,
            Kind::Own => 0,
            Kind::Step(_) => 1,
            Kind::Beneath => 2,
        }
    }
}

/// The widest drift, in radians, two walls whose axes do not cross share
/// their rays within: far under a step of the finest grid, `2π/1024`, so
/// that no two rays a grid tells apart are made one. Their axes a hair off
/// each other over the whole part, each wall's rays taken through the
/// other's samples come back to it a hair round from its own, pass after
/// pass, and a third wall's face cut among them crosses (318277455101196).
const DRIFT: f64 = 1e-3;

/// How far round, in radians, a ray of a pair drifts from one wall's axis to
/// the other's, when the two share their rays: their axes cross, or all
/// but, where the walls hold their circles, standing nearest each other no
/// further apart than that changes; or they stand so near each other there
/// that a ray drifts by less than [`DRIFT`].
fn drift(facing: &Facing) -> Option<f64> {
    let [(_, one), (_, other)] = facing.walls();
    let [nearest, farthest] = facing.apart();
    let sine = one.axis.cross(other.axis).length();
    let drift = farthest / one.radius.min(other.radius) + sine;
    (nearest <= farthest - nearest || drift <= DRIFT).then_some(drift)
}

impl Zones<'_> {
    /// The pairs of walls leaning a hair off parallel that share their rays,
    /// in the order they were taken, each with its drift.
    pub(in crate::brep::tessellation) fn sharing(
        &self,
    ) -> impl Iterator<Item = ([Cylinder; 2], f64)> {
        self.pairs.iter().filter_map(|facing| {
            let walls = facing.walls().map(|(_, wall)| wall);
            let leaning = walls[0].axis.cross(walls[1].axis).length() > Scale::RELATIVE;
            leaning
                .then(|| drift(facing))
                .flatten()
                .map(|drift| (walls, drift))
        })
    }
}
