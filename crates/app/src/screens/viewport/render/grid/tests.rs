//! What app · screens/viewport/render/grid.rs is held to.

use glam::DVec3;

use super::*;

/// Coordinates here stay under a few tens of world units and the vertices are
/// built in `f32`, so this is several orders of magnitude above the rounding
/// and several below one grid step.
const TOLERANCE: f32 = 1e-4;

const STEP: f64 = 0.1;

/// Neither through the world origin nor square to any axis — the shape of
/// plane #318 drew its grid nowhere near.
fn a_face_of_the_part() -> WorkPlane {
    WorkPlane {
        origin: DVec3::new(3.0, 4.0, 5.0),
        u: DVec3::new(1.0, 1.0, 0.0).normalize(),
        v: DVec3::new(-1.0, 1.0, 2.0).normalize(),
    }
}

fn a_view_of(plane: WorkPlane) -> (ViewScale, Vec3) {
    let scale = ViewScale {
        units_per_pixel: 0.01,
        step: STEP,
        step_millimeters: STEP,
        height_px: 800.0,
        diagonal_px: 1000.0,
        aspect: 1.5,
    };
    (scale, plane.origin.as_vec3())
}

fn painted(plane: WorkPlane) -> Vec<cao_render::Vertex> {
    let (scale, target) = a_view_of(plane);
    let mut out = Vec::new();
    push(&mut out, plane, target, 10.0, scale, &Theme::default());
    out
}

#[test]
fn every_vertex_of_the_grid_lies_in_the_sketch_plane() {
    let plane = a_face_of_the_part();
    let normal = plane.normal().as_vec3();
    let origin = plane.origin.as_vec3();

    let painted = painted(plane);
    assert!(!painted.is_empty(), "a plane in view is drawn a grid");

    for vertex in &painted {
        let off_plane = (Vec3::from(vertex.position) - origin).dot(normal);
        assert!(
            off_plane.abs() < TOLERANCE,
            "a vertex at {:?} stands {off_plane} away from the plane it is drawn on",
            vertex.position,
        );
    }
}

#[test]
fn the_grid_is_counted_from_the_sketch_own_origin() {
    let plane = a_face_of_the_part();
    let origin = plane.origin.as_vec3();
    let (u, v) = (plane.u.as_vec3(), plane.v.as_vec3());

    let on_a_crossing = painted(plane).iter().any(|vertex| {
        let offset = Vec3::from(vertex.position) - origin;
        [offset.dot(u), offset.dot(v)]
            .iter()
            .all(|along| (along / STEP as f32).fract().abs() < TOLERANCE)
    });

    assert!(
        on_a_crossing,
        "no vertex lands on a whole number of steps from the plane's own origin, \
         so the lines are counted from somewhere else",
    );
}
