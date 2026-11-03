//! What app · screens/viewport/input/selecting.rs is held to, driven through
//! the input pass a frame at a time, the way the canvas calls it.

use cao_part::PartDocument;
use cao_part::history::PointRef;
use cao_sketch::WorkPlane;
use chrono::Utc;
use glam::DVec2;

use crate::lang::Catalogue;
use crate::screens::extrusion::ExtrusionState;
use crate::screens::sketch::{SketchEditor, SketchPhase, Tool};
use crate::screens::viewport::input::handle_sketch_input;
use crate::screens::viewport::{ViewScale, ViewportState, to_ndc};

use super::*;

const SCREEN: egui::Rect = egui::Rect {
    min: egui::Pos2::ZERO,
    max: egui::pos2(1280.0, 800.0),
};

/// Where a place on screen lands on the drawing's plane.
fn on_the_plane(state: &ViewportState, at: egui::Pos2) -> DVec2 {
    let (origin, direction) = state
        .camera
        .ray(to_ndc(at, SCREEN), SCREEN.width() / SCREEN.height());
    WorkPlane::XY
        .ray_intersection(origin.as_dvec3(), direction.as_dvec3())
        .expect("the place is on the drawing's plane")
}

/// One frame of the canvas, the pointer having done `events` since the last.
fn frame(
    egui: &egui::Context,
    state: &mut ViewportState,
    context: &mut SketchContext<'_>,
    events: Vec<egui::Event>,
) {
    egui.begin_pass(egui::RawInput {
        screen_rect: Some(SCREEN),
        events,
        ..Default::default()
    });
    egui::Area::new(egui::Id::new("the canvas"))
        .fixed_pos(SCREEN.min)
        .show(egui, |ui| {
            ui.set_min_size(SCREEN.size());
            let response = ui.interact(
                SCREEN,
                egui::Id::new("the canvas's surface"),
                egui::Sense::click_and_drag(),
            );
            let scale = ViewScale::of(&state.camera, SCREEN, 1.0, &state.config, 1.0);
            handle_sketch_input(ui, state, &response, SCREEN, scale, context);
        });
    egui.end_pass().textures_delta.clear();
}

fn moved(to: egui::Pos2) -> Vec<egui::Event> {
    vec![egui::Event::PointerMoved(to)]
}

fn button(at: egui::Pos2, pressed: bool) -> Vec<egui::Event> {
    vec![egui::Event::PointerButton {
        pos: at,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    }]
}

#[test]
fn no_magnet_is_marked_while_a_box_is_pulled() {
    let mut state = ViewportState::default();
    let pressed = egui::pos2(640.0, 560.0);
    let beside_a_trait_s_middle = egui::pos2(900.0, 500.0);
    let middle = on_the_plane(&state, beside_a_trait_s_middle + egui::vec2(4.0, 3.0));
    let half = DVec2::X * (middle - on_the_plane(&state, pressed)).length();

    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(middle - half),
        end: PointRef::New(middle + half),
        construction: false,
    });
    let mut editor = SketchEditor {
        phase: SketchPhase::Editing(0),
        tool: Tool::Select,
        tool_state: ToolState::Select(Box::default()),
        ..Default::default()
    };
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let mut context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    let egui = egui::Context::default();

    for events in [
        vec![],
        moved(pressed),
        button(pressed, true),
        moved(pressed + egui::vec2(12.0, -12.0)),
        moved(beside_a_trait_s_middle),
        vec![],
    ] {
        frame(&egui, &mut state, &mut context, events);
    }

    assert!(
        context.editor.band().is_some(),
        "the press on empty space pulls a box"
    );
    assert_eq!(
        context.editor.snap, None,
        "the box's corner stays under the hand, so no magnet is marked where it is not"
    );
}
