//! Where a sketch is drawn, and which way round it reads.

use glam::{DVec2, DVec3};
use serde::{Deserialize, Serialize};

/// The plane a sketch is drawn on: an origin and two unit tangents. Storing the
/// tangents rather than just a normal fixes which way "right" and "up" point in
/// the sketch, so 2D coordinates keep their meaning across sessions.
///
/// Any plane works, including one lying diagonally on a face of a part — the
/// three origin planes are just the ones that exist today.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkPlane {
    pub origin: DVec3,
    pub u: DVec3,
    pub v: DVec3,
}

/// Which plane a sketch is drawn on, as far as anyone naming it needs to know.
///
/// A reading of the geometry, not a name: the interface decides how each case
/// is said.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlaneKind {
    /// Anywhere but through the world origin. Which is all the geometry
    /// establishes: a face of the part, a plane offset by hand and a face of a
    /// second body all answer this.
    OffOrigin,
    OriginXY,
    OriginXZ,
    OriginYZ,
    /// Through the origin, but square to none of the three axes.
    OriginSlanted,
}

impl WorkPlane {
    pub const XY: Self = Self {
        origin: DVec3::ZERO,
        u: DVec3::X,
        v: DVec3::Y,
    };
    pub const XZ: Self = Self {
        origin: DVec3::ZERO,
        u: DVec3::X,
        v: DVec3::Z,
    };
    pub const YZ: Self = Self {
        origin: DVec3::ZERO,
        u: DVec3::Y,
        v: DVec3::Z,
    };

    pub const ORIGIN_PLANES: [Self; 3] = [Self::XY, Self::XZ, Self::YZ];

    /// A plane through `origin` facing `normal`, with tangents chosen to stay
    /// as close as possible to the world axes so the sketch does not come out
    /// arbitrarily rotated.
    pub fn from_normal(origin: DVec3, normal: DVec3) -> Self {
        let normal = normal.normalize_or(DVec3::Z);
        // Any axis not parallel to the normal works as a seed; picking the
        // least aligned one keeps the result stable.
        let seed = if normal.z.abs() < 0.9 {
            DVec3::Z
        } else {
            DVec3::X
        };
        let u = seed.cross(normal).normalize();
        let v = normal.cross(u).normalize();
        Self { origin, u, v }
    }

    /// The plane a face of the part offers a sketch: its origin on the corner
    /// the face is read from, and its axes squared with the screen.
    ///
    /// `up` is which way is up on screen once the view has swung round to face
    /// the plane. `v` follows it and `u` runs to the right, so the face lands
    /// the way up it was just being looked at, and the corner chosen is the
    /// one that leaves the rest of the face up and to the right of it.
    pub fn from_face(corners: &[DVec3], normal: DVec3, up: DVec3) -> Self {
        let normal = normal.normalize_or(DVec3::Z);
        let seed = Self::from_normal(DVec3::ZERO, normal);
        let v = (up - normal * up.dot(normal)).normalize_or(seed.v);
        let u = v.cross(normal);
        Self {
            origin: read_from(corners, u, v).unwrap_or(DVec3::ZERO),
            u,
            v,
        }
    }

    pub fn normal(&self) -> DVec3 {
        self.u.cross(self.v).normalize_or(DVec3::Z)
    }

    /// Which way the near side of the plane lies, seen from `eye`, and how far
    /// along that direction the plane itself sits.
    ///
    /// The plane is the sketch's, never the camera's, but which of its two
    /// halves stands in the way is a question only the camera can answer: the
    /// same matter is in front from one side and behind from the other.
    pub fn near_side(&self, eye: DVec3) -> (DVec3, f64) {
        let normal = self.normal();
        let towards = match normal.dot(eye - self.origin) < 0.0 {
            true => -normal,
            false => normal,
        };
        (towards, towards.dot(self.origin))
    }

    pub fn to_world(&self, point: DVec2) -> DVec3 {
        self.origin + self.u * point.x + self.v * point.y
    }

    pub fn to_local(&self, point: DVec3) -> DVec2 {
        let offset = point - self.origin;
        DVec2::new(offset.dot(self.u), offset.dot(self.v))
    }

    /// Where a ray meets the plane, in sketch coordinates. `None` when the ray
    /// runs parallel to the plane or points away from it.
    pub fn ray_intersection(&self, origin: DVec3, direction: DVec3) -> Option<DVec2> {
        let normal = self.normal();
        let slope = direction.dot(normal);
        if slope.abs() < 1e-6 {
            return None;
        }
        let distance = (self.origin - origin).dot(normal) / slope;
        if distance < 0.0 {
            return None;
        }
        Some(self.to_local(origin + direction * distance))
    }

    /// Which of the planes the interface knows how to name this one is.
    ///
    /// Only a plane through the origin is recognised by its axes: a face of the
    /// part can be parallel to one without being it, and answering `OriginXZ`
    /// would say the drawing sits somewhere it does not.
    pub fn kind(&self) -> PlaneKind {
        if self.origin.length_squared() > 1e-9 {
            return PlaneKind::OffOrigin;
        }
        let normal = self.normal().abs();
        if normal.z > 0.999 {
            PlaneKind::OriginXY
        } else if normal.y > 0.999 {
            PlaneKind::OriginXZ
        } else if normal.x > 0.999 {
            PlaneKind::OriginYZ
        } else {
            PlaneKind::OriginSlanted
        }
    }
}

/// The corner a face is read from: the one nearest the bottom-left of what the
/// face spans, so every size taken from it comes out positive.
///
/// The bottom-left of the span is a point of no face in general — an L-shaped
/// face has nothing there — so the nearest real corner is what answers, and
/// that is the one the user can put a finger on.
fn read_from(corners: &[DVec3], u: DVec3, v: DVec3) -> Option<DVec3> {
    let least = |axis: DVec3| {
        corners
            .iter()
            .map(|corner| corner.dot(axis))
            .fold(f64::INFINITY, f64::min)
    };
    let (least_u, least_v) = (least(u), least(v));
    corners.iter().copied().min_by(|left, right| {
        let reach = |corner: &DVec3| (corner.dot(u) - least_u).hypot(corner.dot(v) - least_v);
        reach(left).total_cmp(&reach(right))
    })
}

#[cfg(test)]
mod tests;
