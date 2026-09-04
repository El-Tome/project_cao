use glam::camera::rh::proj::directx;
use glam::{Mat4, Quat, Vec2, Vec3};

use crate::camera::CubeFace;
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

/// Triangles for the whole cube. Winding is counter-clockwise seen from
/// outside, so back-face culling hides the far side.
pub fn push_faces(out: &mut Vec<Vertex>, hovered: Option<CubeFace>) {
    for face in CubeFace::ALL {
        let color = face_color(face, hovered == Some(face));
        let [a, b, c, d] = corners(face);
        for point in [a, b, c, a, c, d] {
            out.push(Vertex::solid(point, color));
        }
    }
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

/// Which face sits under the cursor, given its position in the cube's own
/// viewport as normalized device coordinates (-1..1, y up).
pub fn pick_face(camera_rotation: Quat, ndc: Vec2) -> Option<CubeFace> {
    let origin = camera_rotation * Vec3::new(ndc.x * ORTHO_HALF_SIZE, ndc.y * ORTHO_HALF_SIZE, 2.0);
    let direction = camera_rotation * Vec3::NEG_Z;

    let mut t_enter = f32::NEG_INFINITY;
    let mut t_exit = f32::INFINITY;
    let mut entered_on = None;

    for axis in 0..3 {
        let (origin, direction) = (origin.to_array()[axis], direction.to_array()[axis]);

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
    Some(match (axis, direction > 0.0) {
        (0, true) => CubeFace::MinusX,
        (0, false) => CubeFace::PlusX,
        (1, true) => CubeFace::MinusY,
        (1, false) => CubeFace::PlusY,
        (_, true) => CubeFace::MinusZ,
        (_, false) => CubeFace::PlusZ,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::camera::OrbitCamera;
    use std::f32::consts::FRAC_PI_2;

    /// Looking straight at a face, the centre of the cube's viewport must pick
    /// that same face.
    #[test]
    fn clicking_the_centre_picks_the_face_we_look_at() {
        for face in CubeFace::ALL {
            let mut camera = OrbitCamera::default();
            let mut transition = crate::ViewTransition::to_face(&camera, face);
            while transition.advance(&mut camera, 0.1) {}

            assert_eq!(
                pick_face(camera.rotation(), Vec2::ZERO),
                Some(face),
                "looking at {face:?}"
            );
        }
    }

    #[test]
    fn clicking_outside_the_cube_picks_nothing() {
        let mut camera = OrbitCamera::default();
        camera.set_view_angles(0.0, FRAC_PI_2);
        assert_eq!(pick_face(camera.rotation(), Vec2::new(0.99, 0.99)), None);
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
