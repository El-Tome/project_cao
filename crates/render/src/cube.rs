use glam::camera::rh::proj::directx;
use glam::{Mat4, Quat, Vec2, Vec3};

use crate::camera::{CubeFace, CubeZone};
use crate::geometry::{Vertex, srgb};

/// Half-size of the orthographic box the cube is drawn in. Slightly larger
/// than the cube's corner radius (0.866) so a corner-on view still has margin.
pub const ORTHO_HALF_SIZE: f32 = 1.0;

const HALF: f32 = 0.5;

/// The cube's local axes are the world axes, so its view matrix is the
/// camera's rotation alone: no translation, no perspective.
pub fn view_projection(camera_rotation: Quat) -> Mat4 {
    let projection = directx::orthographic(
        -ORTHO_HALF_SIZE,
        ORTHO_HALF_SIZE,
        -ORTHO_HALF_SIZE,
        ORTHO_HALF_SIZE,
        -4.0,
        4.0,
    );
    projection * Mat4::from_quat(camera_rotation).inverse()
}

/// True when the face points towards the camera. On a convex cube this is all
/// the visibility test we need, which is why the cube renders correctly
/// without a depth buffer.
pub fn is_visible(face: CubeFace, camera_forward: Vec3) -> bool {
    face.normal().dot(camera_forward) < -1e-4
}

/// The face's centre in cube space, where its label goes.
pub fn face_center(face: CubeFace) -> Vec3 {
    face.normal() * HALF
}

fn tangents(face: CubeFace) -> (Vec3, Vec3) {
    match face {
        CubeFace::PlusX => (Vec3::Y, Vec3::Z),
        CubeFace::MinusX => (Vec3::Z, Vec3::Y),
        CubeFace::PlusY => (Vec3::Z, Vec3::X),
        CubeFace::MinusY => (Vec3::X, Vec3::Z),
        CubeFace::PlusZ => (Vec3::X, Vec3::Y),
        CubeFace::MinusZ => (Vec3::Y, Vec3::X),
    }
}

fn corners(face: CubeFace) -> [Vec3; 4] {
    let (u, v) = tangents(face);
    let center = face_center(face);
    [
        center - u * HALF - v * HALF,
        center + u * HALF - v * HALF,
        center + u * HALF + v * HALF,
        center - u * HALF + v * HALF,
    ]
}

/// Half-width of the border strips that select an edge or a corner rather than
/// the face itself. The face centre therefore covers the middle ~56% of a side.
const ZONE_BORDER: f32 = 0.22;

/// Faces are shaded by orientation so the cube reads as a solid even before
/// the labels are drawn over it.
fn face_color(face: CubeFace, hovered: bool) -> [f32; 4] {
    if hovered {
        return srgb(0.30, 0.60, 0.95, 1.0);
    }
    match face {
        CubeFace::PlusZ => srgb(0.88, 0.89, 0.91, 1.0),
        CubeFace::MinusZ => srgb(0.66, 0.67, 0.70, 1.0),
        CubeFace::PlusX | CubeFace::MinusX => srgb(0.80, 0.81, 0.84, 1.0),
        CubeFace::PlusY | CubeFace::MinusY => srgb(0.74, 0.75, 0.78, 1.0),
    }
}

/// The face reached by leaving a face across the given tangent: the neighbour
/// that shares that border.
fn neighbour(tangent: Vec3, sign: f32) -> CubeFace {
    let normal = tangent * sign;
    CubeFace::ALL
        .into_iter()
        .find(|candidate| candidate.normal().dot(normal) > 0.5)
        .expect("a cube tangent always points at another face")
}

/// Which zone a point on `face` belongs to, from its coordinates along the
/// face tangents (each in -0.5..0.5).
fn zone_at(face: CubeFace, u: f32, v: f32) -> CubeZone {
    let (u_axis, v_axis) = tangents(face);
    let limit = HALF - ZONE_BORDER;
    let on_u = u.abs() > limit;
    let on_v = v.abs() > limit;

    match (on_u, on_v) {
        (true, true) => CubeZone::corner(
            face,
            neighbour(u_axis, u.signum()),
            neighbour(v_axis, v.signum()),
        ),
        (true, false) => CubeZone::edge(face, neighbour(u_axis, u.signum())),
        (false, true) => CubeZone::edge(face, neighbour(v_axis, v.signum())),
        (false, false) => CubeZone::Face(face),
    }
}

/// The 1D cuts of a face side: border strip, centre, border strip.
fn zone_spans() -> [(f32, f32); 3] {
    let limit = HALF - ZONE_BORDER;
    [(-HALF, -limit), (-limit, limit), (limit, HALF)]
}

/// Triangles for the whole cube. Each face is cut into a 3×3 grid so that a
/// hovered edge or corner can be highlighted on every face it touches, the way
/// a CAD navigation cube does. Winding is counter-clockwise seen from outside,
/// so back-face culling hides the far side.
pub fn push_faces(out: &mut Vec<Vertex>, hovered: Option<CubeZone>) {
    for face in CubeFace::ALL {
        let (u_axis, v_axis) = tangents(face);
        let center = face_center(face);

        for (u_start, u_end) in zone_spans() {
            for (v_start, v_end) in zone_spans() {
                let zone = zone_at(face, midpoint(u_start, u_end), midpoint(v_start, v_end));
                let color = face_color(face, hovered == Some(zone));
                let tile = [
                    center + u_axis * u_start + v_axis * v_start,
                    center + u_axis * u_end + v_axis * v_start,
                    center + u_axis * u_end + v_axis * v_end,
                    center + u_axis * u_start + v_axis * v_end,
                ];
                for point in [tile[0], tile[1], tile[2], tile[0], tile[2], tile[3]] {
                    out.push(Vertex::solid(point, color));
                }
            }
        }
    }
}

fn midpoint(start: f32, end: f32) -> f32 {
    (start + end) * 0.5
}

/// Outlines of the front-facing faces only: drawing every edge without a depth
/// buffer would let the back edges show through the solid.
pub fn push_edges(out: &mut Vec<Vertex>, camera_forward: Vec3, width: f32) {
    let color = srgb(0.35, 0.37, 0.40, 1.0);
    for face in CubeFace::ALL {
        if !is_visible(face, camera_forward) {
            continue;
        }
        let points = corners(face);
        for index in 0..4 {
            out.push(Vertex::line(points[index], color, width));
            out.push(Vertex::line(points[(index + 1) % 4], color, width));
        }
    }
}

/// Which zone sits under the cursor, given its position in the cube's own
/// viewport as normalized device coordinates (-1..1, y up). Returns the face,
/// edge or corner aimed at, or `None` when the cursor misses the cube.
pub fn pick_zone(camera_rotation: Quat, ndc: Vec2) -> Option<CubeZone> {
    let origin = camera_rotation * Vec3::new(ndc.x * ORTHO_HALF_SIZE, ndc.y * ORTHO_HALF_SIZE, 2.0);
    let direction_vector = camera_rotation * Vec3::NEG_Z;

    let mut t_enter = f32::NEG_INFINITY;
    let mut t_exit = f32::INFINITY;
    let mut entered_on = None;

    for axis in 0..3 {
        let (origin, direction) = (origin.to_array()[axis], direction_vector.to_array()[axis]);

        if direction.abs() < 1e-6 {
            if origin.abs() > HALF {
                return None;
            }
            continue;
        }

        let (near, far) = {
            let a = (-HALF - origin) / direction;
            let b = (HALF - origin) / direction;
            if a <= b { (a, b) } else { (b, a) }
        };

        if near > t_enter {
            t_enter = near;
            entered_on = Some((axis, direction));
        }
        t_exit = t_exit.min(far);
    }

    if t_enter > t_exit || t_exit < 0.0 {
        return None;
    }

    let (axis, direction) = entered_on?;
    let face = match (axis, direction > 0.0) {
        (0, true) => CubeFace::MinusX,
        (0, false) => CubeFace::PlusX,
        (1, true) => CubeFace::MinusY,
        (1, false) => CubeFace::PlusY,
        (_, true) => CubeFace::MinusZ,
        (_, false) => CubeFace::PlusZ,
    };

    let hit = origin + direction_vector * t_enter;
    let (u_axis, v_axis) = tangents(face);
    Some(zone_at(face, hit.dot(u_axis), hit.dot(v_axis)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::camera::{CubeZone, OrbitCamera};
    use std::f32::consts::FRAC_PI_2;

    /// Looking straight at a face, the centre of the cube's viewport must pick
    /// that same face.
    #[test]
    fn clicking_the_centre_picks_the_face_we_look_at() {
        for face in CubeFace::ALL {
            let mut camera = OrbitCamera::default();
            let mut transition = crate::ViewTransition::to_zone(&camera, CubeZone::Face(face));
            while transition.advance(&mut camera, 0.1) {}

            assert_eq!(
                pick_zone(camera.rotation(), Vec2::ZERO),
                Some(CubeZone::Face(face)),
                "looking at {face:?}"
            );
        }
    }

    /// Straight-on at a face, the border strips select the neighbouring edges
    /// and the four corner tiles select corners.
    #[test]
    fn borders_of_a_face_pick_edges_and_corners() {
        let mut camera = OrbitCamera::default();
        camera.set_view_angles(0.0, FRAC_PI_2);

        // The cube spans 0.5 of the orthographic half-size, so a face border
        // sits around 0.4 in normalized device coordinates.
        let near_border = 0.45;
        assert!(matches!(
            pick_zone(camera.rotation(), Vec2::new(near_border, 0.0)),
            Some(CubeZone::Edge(_))
        ));
        assert!(matches!(
            pick_zone(camera.rotation(), Vec2::new(0.0, near_border)),
            Some(CubeZone::Edge(_))
        ));
        assert!(matches!(
            pick_zone(camera.rotation(), Vec2::new(near_border, near_border)),
            Some(CubeZone::Corner(_))
        ));
        assert!(matches!(
            pick_zone(camera.rotation(), Vec2::ZERO),
            Some(CubeZone::Face(_))
        ));
    }

    /// An edge picked from one of its two faces is the same zone as when it is
    /// picked from the other, so the highlight covers both.
    #[test]
    fn an_edge_is_the_same_zone_from_either_face() {
        let top = CubeZone::Face(CubeFace::PlusZ);
        let front = CubeZone::Face(CubeFace::MinusY);

        let from_top = zone_at(CubeFace::PlusZ, 0.0, -0.45);
        let from_front = zone_at(CubeFace::MinusY, 0.0, 0.45);

        assert_eq!(from_top, from_front);
        assert_ne!(from_top, top);
        assert_ne!(from_top, front);
    }

    #[test]
    fn clicking_outside_the_cube_picks_nothing() {
        let mut camera = OrbitCamera::default();
        camera.set_view_angles(0.0, FRAC_PI_2);
        assert_eq!(pick_zone(camera.rotation(), Vec2::new(0.99, 0.99)), None);
    }

    /// Exactly three faces are visible from a general viewpoint, and never a
    /// face and its opposite at once.
    #[test]
    fn opposite_faces_are_never_both_visible() {
        let camera = OrbitCamera::default();
        let forward = camera.forward();
        for face in CubeFace::ALL {
            if is_visible(face, forward) {
                let opposite = CubeFace::ALL
                    .into_iter()
                    .find(|other| other.normal() == -face.normal())
                    .expect("every face has an opposite");
                assert!(!is_visible(opposite, forward));
            }
        }
    }
}
