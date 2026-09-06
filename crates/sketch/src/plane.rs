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

    pub fn normal(&self) -> DVec3 {
        self.u.cross(self.v).normalize_or(DVec3::Z)
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

    /// Human-readable name.
    ///
    /// Only a plane through the origin is named after its axes: a face of the
    /// part can be parallel to one without being it, and calling it "Plan XZ"
    /// would say the drawing sits somewhere it does not.
    pub fn label(&self) -> &'static str {
        if self.origin.length_squared() > 1e-9 {
            return "Face de la pièce";
        }
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
            for point in [DVec2::ZERO, DVec2::new(12.5, -3.0), DVec2::new(-400.0, 900.0)] {
                let round_trip = plane.to_local(plane.to_world(point));
                assert!((round_trip - point).length() < 1e-3, "{plane:?} {point:?}");
            }
        }
    }

    #[test]
    fn a_plane_built_from_a_normal_keeps_that_normal() {
        for normal in [
            DVec3::Z,
            DVec3::X,
            DVec3::new(1.0, 1.0, 1.0),
            DVec3::new(-2.0, 0.5, 3.0),
        ] {
            let plane = WorkPlane::from_normal(DVec3::ZERO, normal);
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
            .ray_intersection(DVec3::new(3.0, -2.0, 10.0), DVec3::NEG_Z)
            .expect("ray points at the plane");
        assert!((hit - DVec2::new(3.0, -2.0)).length() < 1e-4);

        assert!(plane.ray_intersection(DVec3::Z * 10.0, DVec3::Z).is_none());
        assert!(plane.ray_intersection(DVec3::Z * 10.0, DVec3::X).is_none());
    }
}
