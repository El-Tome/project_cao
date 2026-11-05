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
    /// `direction` or against it, kept where the face holds them
    /// ([`Body::held`]). None for any other surface.
    pub(in crate::brep) fn bulges(&self, face: FaceId, direction: DVec3) -> Vec<DVec3> {
        let surface = self.surface(self.face(face).surface);
        self.held(face, furthest(surface, direction))
    }

    /// The points of a face's sphere or torus where a coordinate is largest
    /// or smallest, kept where the face holds them and where they stand past
    /// the box of its edges' extremes and its poles: with those, what its
    /// own box is read off. A point within that box widens no box and no
    /// reach read off the face, and is never located.
    pub(in crate::brep) fn bulges_held(&self, face: FaceId) -> Vec<DVec3> {
        let surface = self.surface(self.face(face).surface);
        let candidates: Vec<DVec3> = DVec3::AXES
            .into_iter()
            .flat_map(|axis| furthest(surface, axis))
            .collect();
        if candidates.is_empty() {
            return candidates;
        }
        let [low, high] = self.edged(face);
        let past = candidates
            .into_iter()
            .filter(|point| point.cmplt(low).any() || point.cmpgt(high).any())
            .collect();
        self.held(face, past)
    }

    /// The box of a face's edges' extremes and of the poles it holds within
    /// it, empty for a face with neither.
    fn edged(&self, face: FaceId) -> [DVec3; 2] {
        let edges = self
            .face(face)
            .loops
            .iter()
            .flatten()
            .flat_map(|coedge| self.extremes(self.edge(coedge.edge)));
        self.poles_held(face).into_iter().chain(edges).fold(
            [DVec3::INFINITY, DVec3::NEG_INFINITY],
            |[low, high], point| [low.min(point), high.max(point)],
        )
    }

    /// Those of `points`, on a face's sphere or torus, that the face holds —
    /// inside it or on its boundary at any of their [`readings`], located
    /// all at once — and every one where the face has no loop or its domain
    /// cannot be read.
    fn held(&self, face: FaceId, points: Vec<DVec3>) -> Vec<DVec3> {
        if points.is_empty() || self.face(face).loops.is_empty() {
            return points;
        }
        let surface = self.surface(self.face(face).surface);
        let eps = self.scale().eps();
        let read: Vec<Vec<DVec2>> = points
            .iter()
            .map(|point| readings(surface, *point, eps))
            .collect();
        let all: Vec<DVec2> = read.iter().flatten().copied().collect();
        let Ok(found) = self.locate_each(face, &all, eps) else {
            return points;
        };
        let mut found = found.into_iter();
        points
            .into_iter()
            .zip(read)
            .filter(|(_, readings)| {
                let locations: Vec<Location> = found.by_ref().take(readings.len()).collect();
                locations.iter().any(|at| *at != Location::Outside)
            })
            .map(|(point, _)| point)
            .collect()
    }
}

/// The points of a sphere or a torus that go furthest along `direction` or
/// against it — a sphere's two, a torus's four, found where the surface's
/// gradient along the direction vanishes; none on any other surface.
fn furthest(surface: &Surface, direction: DVec3) -> Vec<DVec3> {
    match surface {
        Surface::Sphere(sphere) => [1.0, -1.0]
            .map(|way| sphere.middle() + direction * (way * sphere.radius))
            .to_vec(),
        Surface::Torus(torus) => bulges_of_a_torus(torus, direction),
        Surface::Plane(_) | Surface::Cylinder(_) | Surface::Cone(_) | Surface::Elliptic(_) => {
            Vec::new()
        }
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
