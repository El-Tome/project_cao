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

use super::facing::{Facing, Zones};
use super::own::flat_from;
use crate::brep::scale::Scale;
use crate::brep::surface::Cylinder;
use crate::brep::tessellation::sampling::divisions;
use crate::brep::topology::SurfaceId;

/// What a ray of the pair stands for, in the order one gives way to another
/// within the drift: a ray through a vertex on its wall, never given up —
/// the vertex stands on it there, on every rim — and taken by the other
/// wall too where the vertex lies on both, so that two rims a hair apart
/// keep in step there; but not where it lies on its own wall alone and the
/// other has one of its own beside it. Read from two axes, a vertex on both
/// walls stands up to the drift round on each, further than two vertices
/// can stand apart: no angle tells the one from the other, and the vertex
/// itself does. Then a ray a wall took otherwise on its own, through the
/// samples of a curve, the second wall's giving way to one the first took:
/// handed on along the curves through a vertex, two such rays part a hair
/// further pass after pass (318277455101196). Then a step of a wall's grid,
/// the first wall's before the second's; then a ray through a point
/// standing just inside a wall, which gives way to any ray of either.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    /// Whether the vertex lies on both walls.
    Vertex(bool),
    Own,
    Step(usize),
    Beneath,
}

/// For each wall, points of the body.
type Points = BTreeMap<SurfaceId, Vec<DVec3>>;

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
/// standing just inside it, `anchors` the vertices lying on it. Returns the
/// steps of each wall's grid that gave way to a ray of the other.
pub(super) fn shared(
    leaning: &[&Facing],
    own: &mut BTreeMap<SurfaceId, Vec<DVec3>>,
    (beneath, anchors): (&Points, &Points),
    tolerance: f64,
) -> BTreeMap<SurfaceId, Vec<usize>> {
    let mut withheld: BTreeMap<SurfaceId, Vec<usize>> = BTreeMap::new();
    for facing in leaning {
        let walls = facing.walls();
        let Some(drift) = drift(facing) else {
            continue;
        };
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
            let held = |id: SurfaceId| anchors.get(&id).map_or(&[][..], Vec::as_slice);
            let other = walls[1 - side].0;
            let through = |way: &DVec3| {
                held(id)
                    .iter()
                    .find(|point| flat_from(&wall, **point).normalize() == *way)
                    .map(|point| held(other).contains(point))
            };
            let inside = beneath.get(&id).map_or(&[][..], Vec::as_slice);
            rays.extend(own.get(&id).into_iter().flatten().map(|way| Ray {
                side,
                kind: if inside.contains(way) {
                    Kind::Beneath
                } else if let Some(on_both) = through(way) {
                    Kind::Vertex(on_both)
                } else {
                    Kind::Own
                },
                ways: onto(side, *way),
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
                        Kind::Vertex(_) => false,
                        Kind::Own => {
                            other.side != ray.side
                                && matches!(other.kind, Kind::Own | Kind::Vertex(_))
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
        for (side, (id, _)) in walls.into_iter().enumerate() {
            let mine = |ray: &Ray| ray.side == side && matches!(ray.kind, Kind::Vertex(_));
            let beside_mine = |ray: &Ray| kept.iter().any(|other| mine(other) && near(ray, other));
            let taken = kept
                .iter()
                .filter(|ray| ray.side == side || !ray.kind.left_out(beside_mine(ray)))
                .filter(|ray| !(ray.side == side && matches!(ray.kind, Kind::Step(_))))
                .map(|ray| ray.ways[side])
                .collect();
            own.insert(id, taken);
        }
    }
    withheld
}

impl Kind {
    /// Whether a ray of the other wall is left out of this one: through a
    /// vertex on that wall alone, `beside` one through a vertex of this
    /// wall's own.
    fn left_out(self, beside: bool) -> bool {
        match self {
            Kind::Vertex(on_both) => !on_both && beside,
            Kind::Own | Kind::Step(_) | Kind::Beneath => false,
        }
    }

    fn rank(self) -> usize {
        match self {
            Kind::Vertex(_) => 0,
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
