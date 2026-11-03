//! What app · screens/viewport/render.rs is held to.
//!
//! Closes #319.
//! - See-through: the matter in front of the plane is drawn translucent, and
//!   the sketch shows through it —
//!   `see_through_draws_the_matter_in_front_translucent`

use cao_part::{ExtrusionMode, Operation, PartDocument, PointRef};
use cao_prefs::config::ViewportConfig;
use cao_render::camera::{OrbitCamera, ViewTransition};
use cao_sketch::WorkPlane;
use chrono::Utc;
use glam::{DVec2, DVec3, Vec3};

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

    build_frame(&state, SCREEN, SCREEN, scale, &context, None, None)
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

const SKETCHED_AT: f32 = 2.0;

/// One frame of a block from 0 to 4 over XY, seen square onto a sketch opened
/// halfway up it, after `clicks` on the button that says how the matter in
/// front of the sketch is drawn.
fn a_frame_through_a_block(clicks: usize) -> SceneFrame {
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::ZERO),
        opposite: PointRef::New(DVec2::new(10.0, 10.0)),
        construction: false,
    });
    document.apply(Operation::Extrude {
        sketch: 0,
        areas: document.areas_at(0, &[DVec2::new(5.0, 5.0)]),
        distance: 4.0.into(),
        mode: ExtrusionMode::Add,
    });
    let halfway = WorkPlane {
        origin: DVec3::Z * f64::from(SKETCHED_AT),
        ..WorkPlane::XY
    };
    document.apply(Operation::CreateSketch {
        plane: halfway,
        on: None,
    });

    let mut editor = SketchEditor::default();
    editor.begin_editing(1, halfway);
    for _ in 0..clicks {
        editor.next_matter_in_front();
    }
    let mut state = ViewportState::default();
    state.mode = ViewMode::Plane(halfway);
    assert!(
        state.camera.eye().z > SKETCHED_AT,
        "the fixture looks at the sketch from under it",
    );

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

    build_frame(&state, SCREEN, SCREEN, scale, &context, None, None)
}

fn in_front(vertices: &[cao_render::Vertex]) -> Vec<&cao_render::Vertex> {
    vertices
        .iter()
        .filter(|vertex| vertex.position[2] > SKETCHED_AT + 1e-3)
        .collect()
}

#[test]
fn see_through_draws_the_matter_in_front_translucent() {
    let frame = a_frame_through_a_block(1);

    assert!(
        in_front(&frame.scene_solids).is_empty(),
        "the matter in front of the sketch is drawn opaque, and hides it",
    );
    assert!(
        frame
            .scene_solids
            .iter()
            .any(|vertex| vertex.position[2].abs() <= 1e-3),
        "the matter behind the sketch is not drawn",
    );
    let see_through = in_front(&frame.scene_surfaces);
    assert!(
        !see_through.is_empty(),
        "the matter in front of the sketch is not drawn at all",
    );
    assert!(
        see_through
            .iter()
            .all(|vertex| vertex.color[3] > 0.0 && vertex.color[3] < 1.0),
        "the matter in front is drawn with an opacity outside (0, 1): {:?}",
        see_through
            .iter()
            .map(|vertex| vertex.color[3])
            .collect::<Vec<_>>(),
    );
}
