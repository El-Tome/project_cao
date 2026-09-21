//! What app · screens/viewport/render.rs is held to.

use cao_part::PartDocument;
use cao_prefs::config::ViewportConfig;
use cao_render::camera::{OrbitCamera, ViewTransition};
use cao_sketch::WorkPlane;
use chrono::Utc;
use glam::Vec3;

use super::*;
use crate::lang::Catalogue;
use crate::screens::extrusion::ExtrusionState;
use crate::screens::sketch::SketchEditor;

const SCREEN: egui::Rect = egui::Rect {
    min: egui::Pos2::ZERO,
    max: egui::pos2(1280.0, 800.0),
};

/// One frame of an empty part, seen the way `state` says it is seen.
fn a_frame(prepare: impl FnOnce(&mut ViewportState)) -> SceneFrame {
    let mut state = ViewportState::default();
    prepare(&mut state);

    let mut document = PartDocument::new("part", Utc::now());
    let mut editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let millimetres = document.scale();
    let context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    let scale = ViewScale::of(
        &OrbitCamera::default(),
        SCREEN,
        1.0,
        &ViewportConfig::default(),
        millimetres,
    );

    build_frame(&state, SCREEN, SCREEN, scale, &context, None)
}

/// How many of a frame's world lines lie in `plane`. The grid is the only thing
/// drawn wholly inside a work plane, so this counts it and nothing else.
fn lines_in(frame: &SceneFrame, plane: WorkPlane) -> usize {
    let (origin, normal) = (plane.origin.as_vec3(), plane.normal().as_vec3());
    frame
        .scene_world_lines
        .iter()
        .filter(|vertex| (Vec3::from(vertex.position) - origin).dot(normal).abs() < 1e-4)
        .count()
}

#[test]
fn no_grid_is_drawn_until_the_view_has_landed_on_the_plane() {
    let landed = a_frame(|state| state.mode = ViewMode::Plane(WorkPlane::XY));
    let swinging = a_frame(|state| {
        state.mode = ViewMode::Plane(WorkPlane::XY);
        state.transition = Some(ViewTransition::to_angles(&OrbitCamera::default(), 0.5, 0.5));
    });

    assert!(
        lines_in(&landed, WorkPlane::XY) > 100,
        "a view sitting on a plane is drawn no grid on it",
    );
    assert!(
        lines_in(&swinging, WorkPlane::XY) < lines_in(&landed, WorkPlane::XY),
        "the grid is drawn while the view is still swinging onto the plane, where \
         it reads as a disc floating in the middle of the screen",
    );
}

#[test]
fn the_world_axes_stand_in_until_the_drawing_has_its_own() {
    let free = a_frame(|state| state.mode = ViewMode::Free);
    let landed = a_frame(|state| state.mode = ViewMode::Plane(WorkPlane::XY));

    assert!(
        !free.scene_world_lines.is_empty(),
        "a free view is drawn neither a grid nor an axis, so it shows nothing at all",
    );
    assert_eq!(
        lines_in(&free, WorkPlane::XY),
        4,
        "the world axes lying in XY are drawn as something other than two lines",
    );
    assert!(
        lines_in(&landed, WorkPlane::XY) > lines_in(&free, WorkPlane::XY),
        "landing on a plane adds nothing to what is drawn on it",
    );
}

#[test]
fn a_frame_carries_the_orientation_cube_wherever_the_view_is() {
    for frame in [
        a_frame(|state| state.mode = ViewMode::Free),
        a_frame(|state| state.mode = ViewMode::Plane(WorkPlane::XY)),
    ] {
        assert!(
            !frame.cube_triangles.is_empty() && !frame.cube_edges.is_empty(),
            "the cube is the one way back to a known view, and it is not drawn",
        );
        assert!(
            !frame.scene_background.is_empty(),
            "the frame carries no background at all",
        );
    }
}
