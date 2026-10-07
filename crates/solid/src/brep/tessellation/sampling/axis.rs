//! The rays a circle of a cone takes besides its grid: its cone's grid, and,
//! where the cone stands close to another wall, the rays of every wall
//! standing with it.
//!
//! A circle lying on a cone takes the cone's grid and the angles of its
//! vertices ([`cone`]). That leaves a cone's narrow rim with angles of its
//! own grid the wide one has not, and the strip between them, at two
//! consecutive angles, a pentagon rather than a flat trapezoid: which way
//! it is cut into triangles is the sweep's choice. Alone, that costs nothing.
//! A second wall a hair inside the cone — a coaxial cone or cylinder cut from
//! it — has its own pentagons cut the other way, and the skin between the two
//! is thinner than the gap between those choices: the triangles cross. A ring
//! a hair wide between two rims, each sampled on angles the other has not,
//! has the outer one's chords sag inside the inner one's samples, and is left
//! open along it.
//!
//! So a cone with a rim within twice a chord's sag of a parallel wall — as
//! [`contact`] tells two cylinders close — stands in a group with every other
//! rim of it and every coaxial wall within that sag of one of them, joined
//! one through the next. Every circle of the group is sampled on every ray
//! any of them takes: its cones' grids and vertices, every wall's own grid,
//! and what the walls take in contact. Every strip of those walls is then
//! flat between the same two rays, and walls a hair apart keep their order:
//! what [`contact`] gives cylinders close to each other, given to the walls
//! of a cone, decided once for them. A cone standing close to nothing keeps
//! its rims as they were: two grids merged leave samples a fraction of a step
//! apart, and a plane all but lying on a cone's face, a slope a hair from
//! square, cut into a sliver between two of them lies on the cone's
//! triangles.
//!
//! A group's rays are handed to [`contact`] as each wall's own before it
//! shares them: a wall a hair off the axis, close to one of the group, takes
//! them too through its own axis, and the two keep their order. What the
//! walls of the group then take in contact is gathered back onto all of
//! them.
//!
//! [`cone`]: super::cone
//! [`contact`]: super::super::contact

use std::collections::{BTreeMap, BTreeSet};
use std::f64::consts::{PI, TAU};

use glam::DVec3;

use super::super::contact::{self, Contact, Wall};
use super::cone::Grid;
use super::divisions;
use crate::brep::curve::{Circle, Curve};
use crate::brep::scale::Scale;
use crate::brep::surface::{Cone, Cylinder};
use crate::brep::topology::{Body, SurfaceId};

/// Walls of one axis standing together, and the directions from it, square
/// to it, every circle of them is sampled along.
struct Group {
    axis: DVec3,
    u: DVec3,
    v: DVec3,
    rays: Vec<DVec3>,
}

impl Group {
    fn angle(&self, way: DVec3) -> f64 {
        way.dot(self.v).atan2(way.dot(self.u))
    }

    /// The rays taken, each square to the axis and of unit length, in the
    /// order of their angle and one of each.
    fn settle(&mut self) {
        let flat = |way: DVec3| (way - self.axis * self.axis.dot(way)).normalize_or_zero();
        let mut rays: Vec<DVec3> = self.rays.iter().map(|way| flat(*way)).collect();
        rays.retain(|way| way.length_squared() > 0.0);
        rays.sort_by(|one, other| self.angle(*one).total_cmp(&self.angle(*other)));
        rays.dedup_by(|later, earlier| self.angle(*later) - self.angle(*earlier) <= ROUNDING);
        self.rays = rays;
    }
}

/// The grid of every cone, and the walls standing together with a cone, each
/// in its group, with every group's rays: the grid of each of its cones and
/// the angles of their vertices, the grid of each of its walls, and, once
/// [`Axes::gathered`], the rays its walls take in contact.
pub(super) struct Axes {
    cones: Vec<(Cone, Vec<DVec3>)>,
    groups: Vec<Group>,
    of_wall: BTreeMap<SurfaceId, usize>,
}

impl Axes {
    pub(super) fn of(
        body: &Body,
        grids: &BTreeMap<SurfaceId, Grid>,
        walls: &[Wall],
        tolerance: f64,
    ) -> Axes {
        let eps = body.scale().eps();
        let circles: Vec<&Circle> = body
            .edge_ids()
            .filter_map(|id| match body.curve(body.edge(id).curve) {
                Curve::Circle(circle) => Some(circle),
                Curve::Line(_) | Curve::Meet(_) => None,
            })
            .collect();
        let mut cones = Vec::new();
        let mut rims: Vec<(Vec<SurfaceId>, &Grid)> = Vec::new();
        for grid in grids.values() {
            cones.push((grid.cone, grid.rays().collect()));
            let held: Vec<SurfaceId> = circles
                .iter()
                .filter(|circle| grid.cone.holds(circle, eps))
                .filter_map(|circle| contact::wall_of(circle, walls, eps))
                .map(|(id, _)| *id)
                .collect();
            if !held.is_empty() {
                rims.push((held, grid));
            }
        }
        let about: Vec<&Wall> = walls
            .iter()
            .filter(|wall| {
                rims.iter()
                    .any(|(_, grid)| along(grid.cone.origin, grid.cone.axis, wall, eps))
            })
            .collect();
        let (mut joined, close) = Joined::close(&about, walls, eps, tolerance);
        rims.retain(|(held, _)| held.iter().any(|id| close.contains(id)));
        for (held, _) in &rims {
            for id in held {
                joined.join(held[0], *id);
            }
        }
        let mut axes = Axes {
            cones,
            groups: Vec::new(),
            of_wall: BTreeMap::new(),
        };
        for (held, grid) in rims {
            let group = axes.group(joined.root(held[0]), grid);
            axes.groups[group].rays.extend(grid.rays());
        }
        for (id, wall) in about {
            let Some(&group) = axes.of_wall.get(&joined.root(*id)) else {
                continue;
            };
            axes.of_wall.insert(*id, group);
            let steps = divisions(wall.radius, tolerance);
            axes.groups[group]
                .rays
                .extend((0..steps).map(|step| wall.radial(TAU * step as f64 / steps as f64)));
        }
        for group in &mut axes.groups {
            group.settle();
        }
        axes
    }

    /// The group whose walls `root` stands for, made for `grid`'s axis when
    /// it is the first.
    fn group(&mut self, root: SurfaceId, grid: &Grid) -> usize {
        if let Some(&group) = self.of_wall.get(&root) {
            return group;
        }
        self.groups.push(Group {
            axis: grid.cone.axis,
            u: grid.cone.u,
            v: grid.cone.v,
            rays: Vec::new(),
        });
        self.of_wall.insert(root, self.groups.len() - 1);
        self.groups.len() - 1
    }

    /// For each wall of a group, the group's rays: what it takes on its own
    /// in contact, besides its vertices.
    pub(super) fn given(&self) -> BTreeMap<SurfaceId, Vec<DVec3>> {
        self.of_wall
            .iter()
            .map(|(id, group)| (*id, self.groups[*group].rays.clone()))
            .collect()
    }

    /// Every group's rays and every ray one of its walls takes in contact.
    pub(super) fn gathered(mut self, contacts: &BTreeMap<SurfaceId, Contact>) -> Axes {
        for (id, group) in &self.of_wall {
            if let Some(contact) = contacts.get(id) {
                self.groups[*group]
                    .rays
                    .extend(contact.rays.iter().copied());
            }
        }
        for group in &mut self.groups {
            group.settle();
        }
        self
    }

    /// What a circle of `wall` takes besides its grid, `contact`: the rays
    /// of every cone it lies on, and those of the wall's group if it stands
    /// in one. None for a circle lying on no cone and standing in no group,
    /// which takes `contact` alone. A ray of the group is left out where
    /// `kept` says no sample stands, and at a step of the circle's grid
    /// `contact` withholds there.
    pub(super) fn widened(
        &self,
        contact: &Contact,
        (circle, wall): (&Circle, SurfaceId),
        tolerance: f64,
        eps: f64,
        kept: impl Fn(DVec3) -> bool,
    ) -> Option<Contact> {
        let on_cones = self
            .cones
            .iter()
            .filter(|(cone, _)| cone.holds(circle, eps))
            .flat_map(|(_, rays)| rays.iter().copied());
        let group = self.of_wall.get(&wall).map(|group| &self.groups[*group]);
        let steps = divisions(circle.radius, tolerance);
        let step = TAU / steps as f64;
        let level = circle.center.dot(circle.axis);
        let withheld = |way: &DVec3| {
            let angle = way.dot(circle.v).atan2(way.dot(circle.u));
            let rank = (angle / step).round();
            (angle - rank * step).abs() <= eps / circle.radius
                && contact.withheld.iter().any(|(at, [low, high])| {
                    *at == (rank as i64).rem_euclid(steps as i64) as usize
                        && *low - eps <= level
                        && level <= *high + eps
                })
        };
        let in_group = group
            .into_iter()
            .flat_map(|group| group.rays.iter())
            .filter(|way| !withheld(way) && kept(circle.center + **way * circle.radius))
            .copied();
        let rays: Vec<DVec3> = contact
            .rays
            .iter()
            .copied()
            .chain(on_cones)
            .chain(in_group)
            .collect();
        if rays.len() == contact.rays.len() {
            return None;
        }
        Some(Contact {
            rays,
            withheld: contact.withheld.clone(),
            anchors: contact.anchors.clone(),
            beside: contact.beside.clone(),
            beneath: contact.beneath.clone(),
        })
    }
}

/// Whether a wall stands about the line through `origin` along `axis`.
fn along(origin: DVec3, axis: DVec3, (_, wall): &Wall, eps: f64) -> bool {
    let from = wall.origin - origin;
    axis.cross(wall.axis).length() <= Scale::RELATIVE
        && (from - axis * axis.dot(from)).length() <= eps
}

/// Whether two parallel walls stand within twice a chord's sag of each
/// other, the smaller inside the larger, as [`contact`] tells two cylinders
/// close.
///
/// [`contact`]: super::super::contact
fn together(one: &Cylinder, other: &Cylinder, tolerance: f64) -> bool {
    let (outer, inner) = if one.radius >= other.radius {
        (one, other)
    } else {
        (other, one)
    };
    let between = inner.origin - outer.origin;
    let offset = (between - outer.axis * outer.axis.dot(between)).length();
    let sag = |radius: f64| radius * (1.0 - (PI / divisions(radius, tolerance) as f64).cos());
    offset < outer.radius
        && (outer.radius - inner.radius - offset).abs()
            <= 2.0 * sag(outer.radius).max(sag(inner.radius))
}

/// Walls joined into groups, each named by one of its walls.
#[derive(Default)]
struct Joined {
    parents: BTreeMap<SurfaceId, SurfaceId>,
}

impl Joined {
    /// The walls of `about` joined to every other of them coaxial and within
    /// a sag, and those of them standing within a sag of any parallel wall.
    fn close(
        about: &[&Wall],
        walls: &[Wall],
        eps: f64,
        tolerance: f64,
    ) -> (Joined, BTreeSet<SurfaceId>) {
        let mut joined = Joined::default();
        let mut close = BTreeSet::new();
        for (at, one) in about.iter().enumerate() {
            for other in &about[at + 1..] {
                if along(one.1.origin, one.1.axis, other, eps)
                    && together(&one.1, &other.1, tolerance)
                {
                    joined.join(one.0, other.0);
                }
            }
            let parallel = |other: &&Wall| {
                other.0 != one.0 && one.1.axis.cross(other.1.axis).length() <= Scale::RELATIVE
            };
            if walls
                .iter()
                .filter(parallel)
                .any(|other| together(&one.1, &other.1, tolerance))
            {
                close.insert(one.0);
            }
        }
        (joined, close)
    }

    fn root(&self, wall: SurfaceId) -> SurfaceId {
        let mut at = wall;
        while let Some(&parent) = self.parents.get(&at) {
            at = parent;
        }
        at
    }

    fn join(&mut self, one: SurfaceId, other: SurfaceId) {
        let (one, other) = (self.root(one), self.root(other));
        if one != other {
            self.parents.insert(one.max(other), one.min(other));
        }
    }
}

/// How close two rays of a group stand, in angle, to be one: far under any
/// gap a circle tells apart, so that two rays a circle would sample apart
/// are both kept.
const ROUNDING: f64 = 1e-14;
