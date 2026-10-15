//! Walls joined into groups: coaxial and within a sag of one another, or
//! standing within a sag of any parallel wall; and a circle standing within
//! a sag of a cone's face.

use std::collections::{BTreeMap, BTreeSet};
use std::f64::consts::PI;

use glam::DVec3;

use super::super::cone::Grid;
use crate::brep::curve::Circle;
use crate::brep::scale::Scale;
use crate::brep::surface::Cylinder;
use crate::brep::tessellation::contact::Wall;
use crate::brep::tessellation::sampling::divisions;
use crate::brep::topology::SurfaceId;

/// Whether a wall stands about the line through `origin` along `axis`.
pub(super) fn along(origin: DVec3, axis: DVec3, (_, wall): &Wall, eps: f64) -> bool {
    let from = wall.origin - origin;
    axis.cross(wall.axis).length() <= Scale::RELATIVE
        && (from - axis * axis.dot(from)).length() <= eps
}

/// Whether two parallel walls stand within twice a chord's sag of each
/// other, the smaller inside the larger, as [`contact`] tells two cylinders
/// close.
///
/// [`contact`]: crate::brep::tessellation::contact
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

/// Whether a circle of a cone's axis stands within twice the sag of a chord
/// of either's grid of the cone, as [`together`] tells two walls close: read
/// across the axis at the circle's height, the way a chord sags. A cone all
/// but square to its axis stands that close to the circles of a plane all
/// but lying on it only along the axis, and is left as it was.
pub(super) fn beside(circle: &Circle, grid: &Grid, tolerance: f64) -> bool {
    let cone = &grid.cone;
    let sag = |steps: usize| circle.radius * (1.0 - (PI / steps as f64).cos());
    let room = 2.0 * sag(divisions(circle.radius, tolerance)).max(sag(grid.steps));
    let height = (circle.center - cone.origin).dot(cone.axis);
    cone.holds(circle, room) && (cone.section(height) - circle.radius).abs() <= room
}

/// Walls joined into groups, each named by one of its walls.
#[derive(Default)]
pub(super) struct Joined {
    parents: BTreeMap<SurfaceId, SurfaceId>,
}

impl Joined {
    /// The walls of `about` joined to every other of them coaxial and within
    /// a sag, and those of them standing within a sag of any parallel wall.
    pub(super) fn close(
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

    pub(super) fn root(&self, wall: SurfaceId) -> SurfaceId {
        let mut at = wall;
        while let Some(&parent) = self.parents.get(&at) {
            at = parent;
        }
        at
    }

    pub(super) fn join(&mut self, one: SurfaceId, other: SurfaceId) {
        let (one, other) = (self.root(one), self.root(other));
        if one != other {
            self.parents.insert(one.max(other), one.min(other));
        }
    }
}
