//! Where a face on a sphere or a torus bulges past its edges (#576, #625):
//! the points of its surface that go furthest along a direction, where the
//! face holds them. A coordinate is largest over such a face either where it
//! is largest over the whole surface, or on the face's boundary — so the
//! face's own extent is read off those points and its edges, never off the
//! whole surface. Along a torus's own axis the furthest points make a whole
//! circle, the top of the tube; two of them are read, and a face holding
//! neither holds none of that circle, or its boundary crosses the circle at
//! the same height.

use std::f64::consts::{PI, TAU};

use glam::{DVec2, DVec3};

use super::{Body, FaceId};
use crate::brep::domain::Location;
use crate::brep::surface::{Surface, Torus};

impl Body {
    /// The points of a face's sphere or torus that go furthest along
    /// `direction` or against it — a sphere's two, a torus's four, found
    /// where the surface's gradient along the direction vanishes — kept
    /// where they stand inside the face or on its boundary at any of their
    /// [`readings`], and every one where the face's domain cannot be read.
    /// None for any other surface.
    pub(in crate::brep) fn bulges(&self, face: FaceId, direction: DVec3) -> Vec<DVec3> {
        let surface = self.surface(self.face(face).surface);
        let candidates = match surface {
            Surface::Sphere(sphere) => [1.0, -1.0]
                .map(|way| sphere.middle() + direction * (way * sphere.radius))
                .to_vec(),
            Surface::Torus(torus) => bulges_of_a_torus(torus, direction),
            Surface::Plane(_) | Surface::Cylinder(_) | Surface::Cone(_) | Surface::Elliptic(_) => {
                return Vec::new();
            }
        };
        if self.face(face).loops.is_empty() {
            return candidates;
        }
        let eps = self.scale().eps();
        candidates
            .into_iter()
            .filter(|point| {
                readings(surface, *point, eps)
                    .into_iter()
                    .any(|at| !matches!(self.locate(face, at, eps), Ok(Location::Outside)))
            })
            .collect()
    }

    /// The points of a face's sphere or torus where a coordinate is largest
    /// or smallest, kept where the face holds them: with its edges and its
    /// poles, what its own box is read off.
    pub(in crate::brep) fn bulges_held(&self, face: FaceId) -> Vec<DVec3> {
        DVec3::AXES
            .into_iter()
            .flat_map(|axis| self.bulges(face, axis))
            .collect()
    }
}

/// Every reading of a point of a sphere or a torus a face may hold it at: as
/// the chart reads it and, on a torus, where it stands within the tolerance
/// of the cut its chart's window ends at, the tolerance inside each end of
/// the window too. Both ends are one circle of the torus, which the face may
/// hold from either side, and the chart's own reading of a point on it may
/// land a rounding past the window's top, where the cut is not counted (seed
/// 2722). The angle about the axis is read round, and a sphere's chart ends
/// at its poles, which a face holds as vertices: neither needs another
/// reading. A point dropped wrongly boxes the face short of it, one kept
/// wrongly a tolerance too wide.
fn readings(surface: &Surface, point: DVec3, eps: f64) -> Vec<DVec2> {
    let at = surface.parameters(point);
    let mut readings = vec![at];
    if let Surface::Torus(torus) = surface {
        let room = eps / torus.radius;
        let [bottom, top] = [torus.seam - PI, torus.seam + PI];
        if at.y - bottom <= room || top - at.y <= room {
            readings.extend([top - room, bottom + room].map(|phi| DVec2::new(at.x, phi)));
        }
    }
    readings
}

/// Where a torus goes furthest along `direction` or against it: in the
/// meridian half-plane of the direction's part square to the axis and in
/// the one across from it, at the angle along the meridian whose tangent is
/// the direction's part along the axis over its part across, and half a
/// turn from it; on the own lobe only. The angle about the axis is kept in
/// the chart's own range, so that a point on the line `θ = 0` is read there
/// and not a rounding below a whole turn.
fn bulges_of_a_torus(torus: &Torus, direction: DVec3) -> Vec<DVec3> {
    let along = direction.dot(torus.axis);
    let across = direction - torus.axis * along;
    let theta = across.dot(torus.v).atan2(across.dot(torus.u));
    let (big, radius) = (torus.center.y, torus.radius);
    let mut found = Vec::new();
    for turn in [0.0, PI] {
        let phi = along.atan2(across.length() * turn.cos());
        let about = if theta + turn > PI {
            theta + turn - TAU
        } else {
            theta + turn
        };
        for half in [0.0, PI] {
            let phi = phi + half;
            if big + radius * phi.cos() >= 0.0 {
                found.push(torus.point(DVec2::new(about, phi)));
            }
        }
    }
    found
}
