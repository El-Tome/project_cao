//! What app · screens/viewport/input/selecting.rs is held to, driven through
//! the input pass a frame at a time, the way the canvas calls it.
//!
//! Closes #663.
//! - A box held near a corner, and a box held near a side's middle, light
//!   neither the corner nor the side, read where the lit state is decided —
//!   `a_box_held_beside_a_corner_lights_nothing`,
//!   `a_box_held_beside_a_side_s_middle_lights_nothing`; that pointing lights
//!   things up again once the box is let go is held by
//!   `the_corner_lights_up_again_once_the_box_is_let_go`
//! - On the very frame a box appears, nothing is lit either, whether it was
//!   started on empty space beside a corner or on a rule's mark (surfaced by
//!   review) — `the_frame_a_box_starts_beside_a_corner_lights_nothing`,
//!   `the_frame_a_box_starts_on_a_rule_s_mark_lights_nothing`
//! - A fillet's or a chamfer's own points are not lit while a box is held
//!   beside them (surfaced by review) — no test: held in
//!   `screens/viewport/render/cuts/tests.rs`, where that light is decided,
//!   under its own `Closes #663` block
//! - On a rendered frame, the corner a box is held beside is drawn as at rest
//!   — no test: the pixel renderer is not on main; checked on a headless
//!   rendered frame before the merge, and the pull request says so

use cao_part::PartDocument;
use cao_part::history::PointRef;
use cao_sketch::{Element, PointId, WorkPlane};
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

/// A rectangle on the drawing, with the hand pressed on empty space below it,
/// driven a frame at a time.
struct Bench {
    state: ViewportState,
    document: PartDocument,
    editor: SketchEditor,
    extrusion: ExtrusionState,
    lang: Catalogue,
    egui: egui::Context,
    corner: PointId,
    corner_on_screen: egui::Pos2,
    side_s_middle_on_screen: egui::Pos2,
    centre_on_screen: egui::Pos2,
}

const PRESSED: egui::Pos2 = egui::pos2(640.0, 560.0);

impl Bench {
    fn with_a_rectangle() -> Self {
        let state = ViewportState::default();
        let corner = on_the_plane(&state, egui::pos2(820.0, 260.0));
        let opposite = on_the_plane(&state, egui::pos2(1060.0, 440.0));

        let mut document = PartDocument::new("part", Utc::now());
        document.apply(Operation::CreateSketch {
            plane: WorkPlane::XY,
            on: None,
        });
        document.apply(Operation::AddRectangle {
            sketch: 0,
            corner: PointRef::New(corner),
            opposite: PointRef::New(opposite),
            construction: false,
        });
        let corner_id = document.sketches()[0]
            .nearest_point(corner, 1e-6)
            .expect("the rectangle has a corner where it was laid");

        let on_screen = |place: DVec2| on_the_screen(&state, place);
        let corner_on_screen = on_screen(corner);
        let side_s_middle_on_screen =
            on_screen(DVec2::new((corner.x + opposite.x) / 2.0, corner.y));
        let centre_on_screen = on_screen((corner + opposite) / 2.0);

        Self {
            state,
            document,
            editor: SketchEditor {
                phase: SketchPhase::Editing(0),
                tool: Tool::Select,
                tool_state: ToolState::Select(Box::default()),
                ..Default::default()
            },
            extrusion: ExtrusionState::default(),
            lang: Catalogue::french(),
            egui: egui::Context::default(),
            corner: corner_id,
            corner_on_screen,
            side_s_middle_on_screen,
            centre_on_screen,
        }
    }

    /// A few pixels off `place`, on the side away from the rectangle: well
    /// within reach of what is there, and over nothing.
    fn beside(&self, place: egui::Pos2) -> egui::Pos2 {
        place + (place - self.centre_on_screen).normalized() * 9.0
    }

    fn frames(&mut self, frames: Vec<Vec<egui::Event>>) {
        let mut context = SketchContext {
            document: &mut self.document,
            editor: &mut self.editor,
            extrusion: &mut self.extrusion,
            lang: &self.lang,
        };
        for events in frames {
            frame(&self.egui, &mut self.state, &mut context, events);
        }
    }

    fn hold_a_box_until(&mut self, to: egui::Pos2) {
        self.frames(vec![
            vec![],
            moved(PRESSED),
            button(PRESSED, true),
            moved(PRESSED + egui::vec2(12.0, -12.0)),
            moved(PRESSED + (to - PRESSED) * 0.5),
            moved(to),
            vec![],
        ]);
        assert!(
            self.editor.band().is_some(),
            "the press on empty space pulls a box"
        );
    }

    /// Presses at `from` and moves a pixel at a time along `step`, stopping on
    /// the first frame a box is there.
    fn start_a_box(&mut self, from: egui::Pos2, step: egui::Vec2) {
        self.frames(vec![vec![], moved(from), button(from, true)]);
        let mut at = from;
        for _ in 0..30 {
            at += step;
            self.frames(vec![moved(at)]);
            if self.editor.band().is_some() {
                return;
            }
        }
        panic!("a press on empty space or on a rule's mark pulls a box");
    }
}

/// Where a place on the drawing's plane lands on screen.
fn on_the_screen(state: &ViewportState, place: DVec2) -> egui::Pos2 {
    let view_projection = state
        .camera
        .view_projection(SCREEN.width() / SCREEN.height());
    let clip = view_projection * WorkPlane::XY.to_world(place).as_vec3().extend(1.0);
    let ndc = clip.truncate() / clip.w;
    egui::pos2(
        SCREEN.center().x + ndc.x * SCREEN.width() * 0.5,
        SCREEN.center().y - ndc.y * SCREEN.height() * 0.5,
    )
}

#[test]
fn a_box_held_beside_a_corner_lights_nothing() {
    let mut bench = Bench::with_a_rectangle();
    let beside_the_corner = bench.beside(bench.corner_on_screen);

    bench.hold_a_box_until(beside_the_corner);

    assert_eq!(
        bench.editor.hovered_point, None,
        "the box will not take the corner it is held beside, so the corner is not lit"
    );
    assert_eq!(
        bench.editor.hovered, None,
        "nothing is lit as pointed at while a box is pulled"
    );
}

#[test]
fn a_box_held_beside_a_side_s_middle_lights_nothing() {
    let mut bench = Bench::with_a_rectangle();
    let beside_the_side = bench.beside(bench.side_s_middle_on_screen);

    bench.hold_a_box_until(beside_the_side);

    assert_eq!(
        bench.editor.hovered, None,
        "the box will not take the side it is held beside, so the side is not lit"
    );
    assert_eq!(
        bench.editor.hovered_point, None,
        "nothing is lit as pointed at while a box is pulled"
    );
}

#[test]
fn the_corner_lights_up_again_once_the_box_is_let_go() {
    let mut bench = Bench::with_a_rectangle();
    let beside_the_corner = bench.beside(bench.corner_on_screen);
    bench.hold_a_box_until(beside_the_corner);

    bench.frames(vec![
        button(beside_the_corner, false),
        vec![],
        moved(bench.corner_on_screen),
        vec![],
    ]);

    assert!(bench.editor.band().is_none(), "the box is let go");
    assert_eq!(
        bench.editor.hovered_point,
        Some(bench.corner),
        "pointing at the corner with no button held lights it"
    );
    assert_eq!(
        bench.editor.hovered,
        Some(Selection::Element(Element::Point(bench.corner))),
        "and a click there would take the corner"
    );
}

#[test]
fn the_frame_a_box_starts_beside_a_corner_lights_nothing() {
    let mut bench = Bench::with_a_rectangle();
    let outwards = (bench.corner_on_screen - bench.centre_on_screen).normalized();
    let out_of_reach = bench.corner_on_screen + outwards * 14.0;

    bench.start_a_box(out_of_reach, -outwards);

    assert_eq!(
        bench.editor.hovered_point, None,
        "the box will not take the corner it starts beside, so the corner is not lit"
    );
    assert_eq!(
        bench.editor.hovered, None,
        "nothing is lit as pointed at from the frame a box appears"
    );
}

#[test]
fn the_frame_a_box_starts_on_a_rule_s_mark_lights_nothing() {
    let mut bench = Bench::with_a_rectangle();
    bench.document.apply(Operation::Constrain {
        sketch: 0,
        constraint: cao_sketch::Constraint::Equal {
            first: cao_sketch::SegmentId(0),
            second: cao_sketch::SegmentId(2),
        },
    });
    let view_projection = bench
        .state
        .camera
        .view_projection(SCREEN.width() / SCREEN.height());
    let (_, mark) = crate::screens::viewport::render::placed_marks(
        &bench.document.sketches()[0],
        view_projection,
        SCREEN,
    )[0];

    bench.start_a_box(mark, egui::vec2(1.0, 0.0));

    assert_eq!(
        bench.editor.hovered, None,
        "a rule not held, pressed on, pulls a box, and the box does not light its mark"
    );
    assert_eq!(
        bench.editor.hovered_point, None,
        "nothing is lit as pointed at from the frame a box appears"
    );
}
