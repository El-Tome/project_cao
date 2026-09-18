//! Where a sketch is drawn, and which way round it reads.
//!
//! Closes #358.
//! - a sketch on a face has its origin on a corner of it, the face spanning no
//!   negative size — `a_face_is_read_from_a_corner_and_spans_no_negative_size`
//! - the face reads the way up it was being looked at —
//!   `a_face_reads_the_way_up_it_is_looked_at`
//! - a sketch on one of the three base planes is unchanged —
//!   `the_planes_of_the_origin_come_back_as_they_are`
//! - the drawing's own axes are drawn at that origin — no test: emitted by
//!   `push_plane_axes` in `cao_render`, which holds the assertion for it
//! - a sketch reopened finds the axes it was created with — no test: the plane
//!   is stored on `CreateSketch` and read back, and nothing derives it again

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
mod tests {
    use super::*;

    const TOLERANCE: f64 = 1e-9;

    /// The three planes of the origin are the case everything else is judged
    /// against: their own corners and their own screen up have to give them
    /// back exactly as they are written.
    #[test]
    fn the_planes_of_the_origin_come_back_as_they_are() {
        for (plane, up) in [
            (WorkPlane::XY, DVec3::Y),
            (WorkPlane::XZ, DVec3::Z),
            (WorkPlane::YZ, DVec3::Z),
        ] {
            let corners: Vec<DVec3> = [(0.0, 0.0), (50.0, 0.0), (50.0, 30.0), (0.0, 30.0)]
                .into_iter()
                .map(|(x, y)| plane.to_world(DVec2::new(x, y)))
                .collect();
            let read = WorkPlane::from_face(&corners, plane.normal(), up);

            assert!((read.u - plane.u).length() < TOLERANCE, "{read:?}");
            assert!((read.v - plane.v).length() < TOLERANCE, "{read:?}");
            assert!(
                (read.origin - plane.origin).length() < TOLERANCE,
                "{read:?}"
            );
        }
    }

    #[test]
    fn a_face_is_read_from_a_corner_and_spans_no_negative_size() {
        let height = 40.0;
        let corners: Vec<DVec3> = [(10.0, 20.0), (70.0, 20.0), (70.0, 55.0), (10.0, 55.0)]
            .into_iter()
            .map(|(x, y)| DVec3::new(x, y, height))
            .collect();

        let plane = WorkPlane::from_face(&corners, DVec3::Z, DVec3::Y);

        assert!(
            corners.contains(&plane.origin),
            "the origin has to be a corner of the face, not {:?}",
            plane.origin,
        );
        for corner in &corners {
            let local = plane.to_local(*corner);
            assert!(
                local.x >= -TOLERANCE && local.y >= -TOLERANCE,
                "{corner:?} sits at {local:?}, behind the corner the face is read from",
            );
        }
    }

    #[test]
    fn a_face_reads_the_way_up_it_is_looked_at() {
        let corners = [
            DVec3::new(0.0, 0.0, 5.0),
            DVec3::new(60.0, 0.0, 5.0),
            DVec3::new(60.0, 60.0, 5.0),
            DVec3::new(0.0, 60.0, 5.0),
        ];

        let upright = WorkPlane::from_face(&corners, DVec3::Z, DVec3::Y);
        let turned = WorkPlane::from_face(&corners, DVec3::Z, DVec3::X);

        assert!((upright.v - DVec3::Y).length() < TOLERANCE);
        assert!((turned.v - DVec3::X).length() < TOLERANCE);
        assert!(
            (turned.origin - DVec3::new(0.0, 60.0, 5.0)).length() < TOLERANCE,
            "turning the view a quarter turn moves the corner it is read from",
        );
    }

    /// Looking along the normal leaves nothing of the screen up to square the
    /// axes with, and a face still has to come back with a usable plane.
    #[test]
    fn a_face_looked_at_along_its_own_normal_still_gets_its_axes() {
        let corners = [DVec3::ZERO, DVec3::X * 10.0, DVec3::new(10.0, 10.0, 0.0)];

        let plane = WorkPlane::from_face(&corners, DVec3::Z, DVec3::Z);

        assert!((plane.normal() - DVec3::Z).length() < 1e-9);
        assert!(plane.u.dot(plane.v).abs() < TOLERANCE);
    }

    #[test]
    fn local_and_world_coordinates_round_trip() {
        for plane in WorkPlane::ORIGIN_PLANES {
            for point in [
                DVec2::ZERO,
                DVec2::new(12.5, -3.0),
                DVec2::new(-400.0, 900.0),
            ] {
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

    #[test]
    fn a_plane_is_recognised_by_where_it_sits_and_which_way_it_faces() {
        let through_origin = [
            (WorkPlane::XY, PlaneKind::OriginXY),
            (WorkPlane::XZ, PlaneKind::OriginXZ),
            (WorkPlane::YZ, PlaneKind::OriginYZ),
            (
                WorkPlane::from_normal(DVec3::ZERO, DVec3::new(1.0, 1.0, 0.0)),
                PlaneKind::OriginSlanted,
            ),
        ];

        for (plane, kind) in through_origin {
            assert_eq!(plane.kind(), kind, "{plane:?}");
        }
    }

    #[test]
    fn a_plane_parallel_to_an_axis_plane_but_off_the_origin_is_not_that_plane() {
        let raised = WorkPlane::from_normal(DVec3::Z * 12.0, DVec3::Z);

        assert_eq!(raised.kind(), PlaneKind::OffOrigin);
    }

    #[test]
    fn the_near_side_is_the_one_the_eye_stands_on() {
        let plane = WorkPlane::XY;

        let (above, offset) = plane.near_side(DVec3::new(0.0, 0.0, 12.0));

        assert!(above.distance(DVec3::Z) <= TOLERANCE, "got {above:?}");
        assert!(offset.abs() <= TOLERANCE);
    }

    #[test]
    fn looking_from_under_a_plane_turns_its_near_side_over() {
        let plane = WorkPlane::XY;

        let (below, _) = plane.near_side(DVec3::new(0.0, 0.0, -12.0));

        assert!(below.distance(DVec3::NEG_Z) <= TOLERANCE, "got {below:?}");
    }

    #[test]
    fn a_plane_away_from_the_origin_says_how_far_along_it_sits() {
        let plane = WorkPlane::from_normal(DVec3::new(0.0, 0.0, 4.0), DVec3::Z);

        let (towards, offset) = plane.near_side(DVec3::new(0.0, 0.0, 12.0));

        assert!((towards.dot(DVec3::Z) - 1.0).abs() <= TOLERANCE);
        assert!(
            (offset - 4.0).abs() <= TOLERANCE,
            "the plane stands four units along its own normal, got {offset}"
        );
    }
}
