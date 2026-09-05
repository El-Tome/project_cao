use glam::{Vec2, Vec3};
use serde::{Deserialize, Serialize};

/// The plane a sketch is drawn on: an origin and two unit tangents. Storing the
/// tangents rather than just a normal fixes which way "right" and "up" point in
/// the sketch, so 2D coordinates keep their meaning across sessions.
///
/// Any plane works, including one lying diagonally on a face of a part — the
/// three origin planes are just the ones that exist today.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkPlane {
    pub origin: Vec3,
    pub u: Vec3,
    pub v: Vec3,
}

impl WorkPlane {
    pub const XY: Self = Self {
        origin: Vec3::ZERO,
        u: Vec3::X,
        v: Vec3::Y,
    };
    pub const XZ: Self = Self {
        origin: Vec3::ZERO,
        u: Vec3::X,
        v: Vec3::Z,
    };
    pub const YZ: Self = Self {
        origin: Vec3::ZERO,
        u: Vec3::Y,
        v: Vec3::Z,
    };

    pub const ORIGIN_PLANES: [Self; 3] = [Self::XY, Self::XZ, Self::YZ];

    /// A plane through `origin` facing `normal`, with tangents chosen to stay
    /// as close as possible to the world axes so the sketch does not come out
    /// arbitrarily rotated.
    pub fn from_normal(origin: Vec3, normal: Vec3) -> Self {
        let normal = normal.normalize_or(Vec3::Z);
        // Any axis not parallel to the normal works as a seed; picking the
        // least aligned one keeps the result stable.
        let seed = if normal.z.abs() < 0.9 {
            Vec3::Z
        } else {
            Vec3::X
        };
        let u = seed.cross(normal).normalize();
        let v = normal.cross(u).normalize();
        Self { origin, u, v }
    }

    pub fn normal(&self) -> Vec3 {
        self.u.cross(self.v).normalize_or(Vec3::Z)
    }

    pub fn to_world(&self, point: Vec2) -> Vec3 {
        self.origin + self.u * point.x + self.v * point.y
    }

    pub fn to_local(&self, point: Vec3) -> Vec2 {
        let offset = point - self.origin;
        Vec2::new(offset.dot(self.u), offset.dot(self.v))
    }

    /// Where a ray meets the plane, in sketch coordinates. `None` when the ray
    /// runs parallel to the plane or points away from it.
    pub fn ray_intersection(&self, origin: Vec3, direction: Vec3) -> Option<Vec2> {
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

    /// Human-readable name, for the three origin planes and anything else.
    pub fn label(&self) -> &'static str {
        let normal = self.normal().abs();
        if normal.z > 0.999 {
            "Plan XY"
        } else if normal.y > 0.999 {
            "Plan XZ"
        } else if normal.x > 0.999 {
            "Plan YZ"
        } else {
            "Plan d'esquisse"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_and_world_coordinates_round_trip() {
        for plane in WorkPlane::ORIGIN_PLANES {
            for point in [Vec2::ZERO, Vec2::new(12.5, -3.0), Vec2::new(-400.0, 900.0)] {
                let round_trip = plane.to_local(plane.to_world(point));
                assert!((round_trip - point).length() < 1e-3, "{plane:?} {point:?}");
            }
        }
    }

    #[test]
    fn a_plane_built_from_a_normal_keeps_that_normal() {
        for normal in [
            Vec3::Z,
            Vec3::X,
            Vec3::new(1.0, 1.0, 1.0),
            Vec3::new(-2.0, 0.5, 3.0),
        ] {
            let plane = WorkPlane::from_normal(Vec3::ZERO, normal);
            let expected = normal.normalize();
            assert!(
                (plane.normal() - expected).length() < 1e-4,
                "{normal:?} gave {:?}",
                plane.normal()
            );
            assert!((plane.u.length() - 1.0).abs() < 1e-4);
            assert!(plane.u.dot(plane.v).abs() < 1e-4);
        }
    }

    #[test]
    fn a_ray_meets_the_plane_where_it_should() {
        let plane = WorkPlane::XY;
        let hit = plane
            .ray_intersection(Vec3::new(3.0, -2.0, 10.0), Vec3::NEG_Z)
            .expect("ray points at the plane");
        assert!((hit - Vec2::new(3.0, -2.0)).length() < 1e-4);

        assert!(plane.ray_intersection(Vec3::Z * 10.0, Vec3::Z).is_none());
        assert!(plane.ray_intersection(Vec3::Z * 10.0, Vec3::X).is_none());
    }
}
