//! What app · screens/viewport/state.rs is held to.
//!
//! Closes #519.
//! - the grid step is chosen in f64, so a snapped place is round —
//!   `a_place_a_metre_from_the_origin_snaps_onto_a_round_value`
//!
//! Closes #386.
//! - the canvas keeps what it knows apart from what it draws — no test:
//!   `MODES_WITHOUT_A_PRESENTER` lost `viewport` in `architecture.rs`, which
//!   fails from either side — if the pair of files goes away, or if the entry
//!   comes back while they are there
//! - a gesture the cube took goes no further —
//!   `a_gesture_the_cube_took_goes_no_further`
//! - while an extrusion is being set up, a gesture picks an area —
//!   `while_an_extrusion_is_being_set_up_a_gesture_picks_an_area`
//! - otherwise the tool in hand has it —
//!   `otherwise_the_gesture_goes_to_the_tool_in_hand`
//! - the cube is asked before the extrusion —
//!   `the_cube_is_asked_before_the_extrusion`
//! - the presenter takes no `egui::Ui` — no test:
//!   `a_presenter_never_takes_the_interface` reads every `state.rs` under
//!   `crates/app/src` and refuses the three markers
//!
//! Closes #175.
//! - the dimensions a refused change would break blink for a few seconds —
//!   `what_a_refused_change_would_break_blinks_for_a_few_seconds_then_stops`
//! - and so does the matter a refusal names, when it names a step of matter —
//!   `the_matter_a_refusal_names_blinks_the_same_way`
//!
//! Closes #728 — the whole transcription is in `crates/app/tests/view_bar.rs`;
//! these bullets are the ones the camera answers, which no driver reads.
//! « Recadrer » is run here as its button and F run it, through
//! `commands::run`, from a module of the viewport: `camera` is `pub(super)`,
//! seen by every module under `viewport` and by none outside it, so
//! `commands/tests.rs` could not read where it was left.
//! - « Recadrer » outside a sketch keeps the direction the part is seen from,
//!   aims at the middle of the body's bounds and stands back until they fit
//!   and fill the view —
//!   `reframing_outside_a_sketch_frames_the_whole_part_from_where_it_is_seen`
//! - with no volume, it puts back the camera a new part opens with —
//!   `reframing_a_part_with_no_volume_puts_back_the_camera_a_new_part_opens_with`
//! - in a sketch, as before, it faces the plane and frames the sketch, not the
//!   part —
//!   `reframing_in_a_sketch_faces_its_plane_and_frames_its_drawing_rather_than_the_part`
//! - found in review: while a plane is being chosen for a new sketch, no sketch
//!   is open, and « Recadrer » frames the whole part as in the free view,
//!   rather than turning square onto the plane of the sketch left —
//!   `reframing_while_a_plane_is_being_chosen_frames_the_whole_part`
//! - in the free view the coordinates are those of the first point of the
//!   drawn body the cursor's ray meets — the nearer of the faces it crosses —
//!   `the_point_of_the_part_under_a_ray_is_the_first_the_ray_meets`; nothing
//!   when it meets none — `a_ray_that_meets_nothing_of_the_part_points_at_nothing`;
//!   the body as it is drawn, cut where the section view cuts it —
//!   `with_the_section_view_on_the_point_under_a_ray_is_on_the_cut`
//! - nothing when the body is hidden, as nothing of it is met when a plane is
//!   chosen (`viewport/matter.rs` `drawn_under`) —
//!   `a_hidden_body_has_no_point_under_a_ray`
//!
//! Closes #729. The rest of the issue is in `render/tests.rs`,
//! `render/overlays/tests.rs` and `crates/app/tests/view_bar.rs`, each under
//! its own block.
//! - « Vue d'origine » under the cube: the isometric view the part opens with,
//!   framed on the part — read as: the angles of `OrbitCamera::default`, the
//!   body's bounds fitted as « Recadrer » fits them, and a command as
//!   « Recadrer » is one —
//!   `the_origin_view_takes_the_opening_angles_framed_on_the_part`; it turns
//!   the camera there rather than jumping, as « Recadrer » does —
//!   `the_origin_view_turns_the_camera_there_rather_than_jumping`; with no
//!   volume, the camera a new part opens with —
//!   `the_origin_view_of_a_part_with_no_volume_is_the_camera_a_new_part_opens_with`
//! - The cube's corner, size and margin settings —
//!   `the_cube_keeps_its_corner_size_and_margin_settings`

use cao_part::{ExtrusionMode, Operation, PartDocument, PointRef};
use cao_prefs::Command;
use cao_render::MatterInFront;
use chrono::Utc;
use glam::{DVec2, Vec3};

use super::*;
use crate::lang::Catalogue;
use crate::screens::extrusion::ExtrusionState;
use crate::screens::ribbon::Ribbon;
use crate::screens::sketch::SketchEditor;
use crate::screens::viewport::matter::{drawn_under, shown};

/// Who the canvas hands the gesture to, on an empty part.
fn asked(cube_took_it: bool, setting_up_an_extrusion: bool) -> GestureGoesTo {
    let mut document = PartDocument::new("part", Utc::now());
    let mut editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    if setting_up_an_extrusion {
        extrusion.mode = Some(ExtrusionMode::Add);
        extrusion.sketch = Some(0);
    }
    let lang = Catalogue::french();
    let sketch = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };

    gesture_goes_to(cube_took_it, &sketch)
}

#[test]
fn a_gesture_the_cube_took_goes_no_further() {
    assert_eq!(asked(true, false), GestureGoesTo::TheCube);
}

#[test]
fn while_an_extrusion_is_being_set_up_a_gesture_picks_an_area() {
    assert_eq!(asked(false, true), GestureGoesTo::PickingAnArea);
}

#[test]
fn otherwise_the_gesture_goes_to_the_tool_in_hand() {
    assert_eq!(asked(false, false), GestureGoesTo::TheToolInHand);
}

#[test]
fn the_cube_is_asked_before_the_extrusion() {
    assert_eq!(asked(true, true), GestureGoesTo::TheCube);
}

#[test]
fn what_a_refused_change_would_break_blinks_for_a_few_seconds_then_stops() {
    let mut state = ViewportState::default();
    let side = cao_sketch::DimensionTarget::Length(cao_sketch::SegmentId(1));

    state.blink(vec![(0, side)], Vec::new(), 10.0);

    assert_eq!(state.blinking_on(0, 10.1), Some((vec![side], true)));
    assert_eq!(state.blinking_on(0, 10.3), Some((vec![side], false)));
    assert_eq!(
        state.blinking_on(1, 10.1),
        None,
        "only on the sketch it is on"
    );
    assert_eq!(state.blinking_on(0, 14.0), None, "and not for ever");
    assert_eq!(state.faces_blinking(10.1), None, "no face was named");
}

#[test]
fn the_matter_a_refusal_names_blinks_the_same_way() {
    let mut state = ViewportState::default();

    state.blink(Vec::new(), vec![4, 5], 10.0);

    assert_eq!(state.faces_blinking(10.1), Some((&[4, 5][..], true)));
    assert_eq!(state.faces_blinking(10.3), Some((&[4, 5][..], false)));
    assert_eq!(state.faces_blinking(14.0), None, "and not for ever");
}

#[test]
fn a_place_a_metre_from_the_origin_snaps_onto_a_round_value() {
    let config = ViewportConfig::default();
    let mut camera = OrbitCamera::default();
    camera.zoom_by_factor(200.0);
    let screen = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1280.0, 800.0));
    let scale = ViewScale::of(&camera, screen, 1.0, &config, 1.0);
    assert!(
        (scale.step_millimeters - 0.1).abs() < 1e-3,
        "the view is meant to be zoomed onto a grid of a tenth of a millimetre, \
         and its step is {}",
        scale.step_millimeters,
    );

    let (snapped, _) = cao_sketch::Sketch::new(WorkPlane::XY).magnetise(
        glam::DVec2::new(1_000.001, 1_000.001),
        &scale.snapping(&config),
    );

    assert!(
        (snapped - glam::DVec2::splat(1_000.0)).length() < 1e-9,
        "a place a metre out snaps onto the grid at {snapped:?}, off the round \
         thousand it was pulled to",
    );
}

/// The canvas as the part screen lays it out, wider than high.
const CANVAS: egui::Rect = egui::Rect {
    min: egui::Pos2::ZERO,
    max: egui::pos2(1280.0, 522.0),
};

/// How much of the canvas a framed part covers at the least, across or up:
/// a sphere framed around a long flat drawing, as a sketch has always been
/// framed, leaves it at a little more.
const FILLS_THE_CANVAS: f32 = 1.0 / 3.0;

/// How close two directions of the eye are taken to be the same one: the dot
/// product of the two, in `f32`, short of one by no more than this.
const SAME_DIRECTION: f32 = 1e-6;

/// A block 400 by 800 by 200 standing far out from the origin, so that a view
/// that does not move to it leaves it out of sight.
fn a_block_far_from_the_origin() -> PartDocument {
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::new(300.0, 400.0)),
        opposite: PointRef::New(DVec2::new(700.0, 1200.0)),
        construction: false,
    });
    document.apply(Operation::Extrude {
        sketch: 0,
        areas: document.areas_at(0, &[DVec2::new(500.0, 800.0)]),
        distance: 200.0.into(),
        mode: ExtrusionMode::Add,
    });
    assert!(!document.body().is_empty(), "the block is raised");
    document
}

/// A view turned from below the part, away from both the corner a new part is
/// seen from and the corner a fresh volume is shown from, framing `radius`
/// around the origin: close up for a small one, far back for a large one.
fn a_view_turned_away(radius: f32) -> ViewportState {
    let mut state = ViewportState {
        canvas: Some(CANVAS),
        ..ViewportState::default()
    };
    state.camera.set_view_angles(0.3, -0.5);
    state
        .camera
        .focus_on(Vec3::ZERO, radius, CANVAS.aspect_ratio());
    state
}

/// « Recadrer » run as its button and F run it, the camera then left where its
/// turn ends.
fn reframed(document: &mut PartDocument, editor: &mut SketchEditor, state: &mut ViewportState) {
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let mut part = SketchContext {
        document,
        editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    crate::commands::run(Command::Reframe, &mut part, &mut Ribbon::default(), state);
    if let Some(mut turn) = state.transition.take() {
        while turn.advance(&mut state.camera, 1.0) {}
    }
}

fn corners_of((low, high): (DVec3, DVec3)) -> Vec<DVec3> {
    let mut corners = Vec::new();
    for x in [low.x, high.x] {
        for y in [low.y, high.y] {
            for z in [low.z, high.z] {
                corners.push(DVec3::new(x, y, z));
            }
        }
    }
    corners
}

/// How much of the canvas these points cover, across or up, whichever is more
/// — one from edge to edge — once each is checked to stand inside it.
fn covered_by(state: &ViewportState, points: &[DVec3]) -> f32 {
    let view_projection = state.camera.view_projection(CANVAS.aspect_ratio());
    let mut low = glam::Vec2::splat(f32::MAX);
    let mut high = glam::Vec2::splat(f32::MIN);
    for point in points {
        let clip = view_projection * point.as_vec3().extend(1.0);
        assert!(clip.w > 0.0, "{point:?} stands behind the eye");
        let at = glam::Vec2::new(clip.x, clip.y) / clip.w;
        assert!(
            at.abs().max_element() <= 1.0,
            "{point:?} lands at {at:?}, outside the canvas, which reaches from -1 to 1",
        );
        low = low.min(at);
        high = high.max(at);
    }
    ((high - low) * 0.5).max_element()
}

#[test]
fn reframing_outside_a_sketch_frames_the_whole_part_from_where_it_is_seen() {
    let mut document = a_block_far_from_the_origin();
    let bounds = document.body().bounds().expect("the block has bounds");
    let middle = ((bounds.0 + bounds.1) * 0.5).as_vec3();

    for (radius, from) in [(1.0, "close up"), (1e5, "far back")] {
        let mut state = a_view_turned_away(radius);
        let seen_along = state.camera.forward();

        reframed(&mut document, &mut SketchEditor::default(), &mut state);

        assert!(
            state.camera.forward().dot(seen_along) >= 1.0 - SAME_DIRECTION,
            "from a view {from}, « Recadrer » turned the part: it is seen along {:?}, \
             it was seen along {seen_along:?}",
            state.camera.forward(),
        );
        assert!(
            (state.camera.target() - middle).length() < 1e-2,
            "from a view {from}, « Recadrer » aims at {:?}, and the middle of the part \
             is {middle:?}",
            state.camera.target(),
        );
        let covered = covered_by(&state, &corners_of(bounds));
        assert!(
            covered >= FILLS_THE_CANVAS,
            "from a view {from}, the part covers {covered} of the canvas once reframed, \
             where the whole part should fill it",
        );
    }
}

/// A part whose one sketch holds a rectangle nothing was raised from.
fn a_drawing_never_raised() -> PartDocument {
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::new(300.0, 400.0)),
        opposite: PointRef::New(DVec2::new(700.0, 1200.0)),
        construction: false,
    });
    document
}

#[test]
fn reframing_a_part_with_no_volume_puts_back_the_camera_a_new_part_opens_with() {
    let opens_with = ViewportState::default().camera;

    for (mut document, what) in [
        (PartDocument::new("part", Utc::now()), "an empty part"),
        (a_drawing_never_raised(), "a part drawn and never raised"),
    ] {
        let mut state = a_view_turned_away(1e5);
        state.camera.set_target(Vec3::new(50.0, -20.0, 80.0));

        reframed(&mut document, &mut SketchEditor::default(), &mut state);

        assert!(
            state.camera.forward().dot(opens_with.forward()) >= 1.0 - SAME_DIRECTION,
            "on {what}, « Recadrer » leaves the part seen along {:?}, and a new part \
             opens seen along {:?}",
            state.camera.forward(),
            opens_with.forward(),
        );
        assert!(
            (state.camera.target() - opens_with.target()).length() < 1e-4,
            "on {what}, « Recadrer » aims at {:?}, and a new part opens aimed at {:?}",
            state.camera.target(),
            opens_with.target(),
        );
        assert!(
            (state.camera.distance() - opens_with.distance()).abs() < 1e-3,
            "on {what}, « Recadrer » stands {} away, and a new part opens {} away",
            state.camera.distance(),
            opens_with.distance(),
        );
    }
}

#[test]
fn reframing_in_a_sketch_faces_its_plane_and_frames_its_drawing_rather_than_the_part() {
    let mut document = a_block_far_from_the_origin();
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XZ,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 1,
        corner: PointRef::New(DVec2::new(40.0, 30.0)),
        opposite: PointRef::New(DVec2::new(200.0, 150.0)),
        construction: false,
    });
    let (low, high) = document.sketches()[1]
        .bounds()
        .expect("the sketch holds a drawing");
    let mut editor = SketchEditor::default();
    editor.begin_editing(1, WorkPlane::XZ);
    let mut state = a_view_turned_away(1e5);

    reframed(&mut document, &mut editor, &mut state);

    let facing = (-WorkPlane::XZ.normal()).as_vec3();
    assert!(
        state.camera.forward().dot(facing) >= 1.0 - SAME_DIRECTION,
        "in a sketch on XZ, « Recadrer » leaves the eye looking along {:?}, not \
         straight at the plane, along {facing:?}",
        state.camera.forward(),
    );
    assert_eq!(
        state.mode,
        ViewMode::Plane(WorkPlane::XZ),
        "in a sketch, « Recadrer » leaves the view off the sketch's plane and its grid",
    );
    let middle = WorkPlane::XZ.to_world((low + high) * 0.5).as_vec3();
    assert!(
        (state.camera.target() - middle).length() < 1e-3,
        "in a sketch, « Recadrer » aims at {:?}, and the middle of the sketch is \
         {middle:?}",
        state.camera.target(),
    );
    let corners = [
        low,
        DVec2::new(low.x, high.y),
        high,
        DVec2::new(high.x, low.y),
    ]
    .map(|corner| WorkPlane::XZ.to_world(corner));
    let covered = covered_by(&state, &corners);
    assert!(
        covered >= FILLS_THE_CANVAS,
        "in a sketch, its drawing covers {covered} of the canvas once reframed",
    );
}

/// A block 10 by 20 raised 4 from XY, its corner on the origin.
fn a_block_on_the_origin() -> PartDocument {
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::ZERO),
        opposite: PointRef::New(DVec2::new(10.0, 20.0)),
        construction: false,
    });
    document.apply(Operation::Extrude {
        sketch: 0,
        areas: document.areas_at(0, &[DVec2::new(5.0, 10.0)]),
        distance: 4.0.into(),
        mode: ExtrusionMode::Add,
    });
    assert!(!document.body().is_empty(), "the block is raised");
    document
}

/// Straight down onto the middle of the block's top, from well above it.
const FROM_ABOVE: (DVec3, DVec3) = (DVec3::new(5.0, 10.0, 100.0), DVec3::NEG_Z);

/// Straight up onto the middle of its bottom, from well below.
const FROM_BELOW: (DVec3, DVec3) = (DVec3::new(5.0, 10.0, -100.0), DVec3::Z);

/// How far from where it should be a point met on the block may stand: the
/// block's faces are flat and lie on round values.
const ON_THE_FACE: f64 = 1e-9;

fn under(
    state: &ViewportState,
    document: &PartDocument,
    (origin, direction): (DVec3, DVec3),
) -> Option<DVec3> {
    let editor = SketchEditor::default();
    let drawn = shown(state, &editor, document.body());
    let faces = state.face_modes(document);
    let (_, hit) = drawn_under(&faces, &drawn, origin, direction)?;
    Some(origin + direction * hit.distance)
}

fn assert_met_at(met: Option<DVec3>, expected: DVec3, what: &str) {
    let Some(met) = met else {
        panic!("{what}: the ray meets nothing of the block");
    };
    assert!(
        (met - expected).length() < ON_THE_FACE,
        "{what}: the ray meets the block at {met:?}, where it first meets it at {expected:?}",
    );
}

#[test]
fn the_point_of_the_part_under_a_ray_is_the_first_the_ray_meets() {
    let document = a_block_on_the_origin();
    let state = ViewportState::default();

    assert_met_at(
        under(&state, &document, FROM_ABOVE),
        DVec3::new(5.0, 10.0, 4.0),
        "from above, the top before the bottom",
    );
    assert_met_at(
        under(&state, &document, FROM_BELOW),
        DVec3::new(5.0, 10.0, 0.0),
        "from below, the bottom before the top",
    );
}

#[test]
fn a_ray_that_meets_nothing_of_the_part_points_at_nothing() {
    let document = a_block_on_the_origin();
    let state = ViewportState::default();
    let beside_the_block = (DVec3::new(50.0, 50.0, 100.0), DVec3::NEG_Z);

    assert_eq!(under(&state, &document, beside_the_block), None);
    assert_eq!(
        under(&state, &PartDocument::new("part", Utc::now()), FROM_ABOVE),
        None,
        "a part with no volume has a point under the ray",
    );
}

#[test]
fn a_hidden_body_has_no_point_under_a_ray() {
    let document = a_block_on_the_origin();
    let mut state = ViewportState::default();
    assert_eq!(
        state.whole_body_choice(),
        MatterInFront::Opaque,
        "the body opens opaque"
    );
    assert!(
        under(&state, &document, FROM_ABOVE).is_some(),
        "an opaque body is met"
    );

    state.set_whole_body(MatterInFront::SeeThrough);
    assert!(
        under(&state, &document, FROM_ABOVE).is_some(),
        "a see-through body is drawn, and is met as a plane is chosen on it",
    );

    state.set_whole_body(MatterInFront::Hidden);
    assert_eq!(
        under(&state, &document, FROM_ABOVE),
        None,
        "the body is hidden, and the ray still meets it",
    );
}

#[test]
fn with_the_section_view_on_the_point_under_a_ray_is_on_the_cut() {
    let document = a_block_on_the_origin();
    let mut state = ViewportState::default();

    state.toggle_section(document.body());

    assert_met_at(
        under(&state, &document, FROM_ABOVE),
        DVec3::new(5.0, 10.0, 2.0),
        "with the section through the middle of the block, its upper half hidden",
    );
}

#[test]
fn reframing_while_a_plane_is_being_chosen_frames_the_whole_part() {
    let mut document = a_block_far_from_the_origin();
    let bounds = document.body().bounds().expect("the block has bounds");
    let middle = ((bounds.0 + bounds.1) * 0.5).as_vec3();
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XZ,
        on: None,
    });
    let mut editor = SketchEditor::default();
    editor.begin_editing(1, WorkPlane::XZ);
    let mut state = a_view_turned_away(1e5);
    let lang = Catalogue::french();
    crate::commands::run(
        Command::NewSketch,
        &mut SketchContext {
            document: &mut document,
            editor: &mut editor,
            extrusion: &mut ExtrusionState::default(),
            lang: &lang,
        },
        &mut Ribbon::default(),
        &mut state,
    );
    assert!(
        editor.is_choosing_plane() && editor.plane == Some(WorkPlane::XZ),
        "the fixture has a plane being chosen, the plane of the sketch left still held",
    );
    let seen_along = state.camera.forward();

    reframed(&mut document, &mut editor, &mut state);

    assert!(
        state.camera.forward().dot(seen_along) >= 1.0 - SAME_DIRECTION,
        "while a plane is being chosen, « Recadrer » turned the part: it is seen along \
         {:?}, it was seen along {seen_along:?}",
        state.camera.forward(),
    );
    assert_eq!(
        state.mode,
        ViewMode::Free,
        "while a plane is being chosen, « Recadrer » laid the view on a plane",
    );
    assert!(
        (state.camera.target() - middle).length() < 1e-2,
        "while a plane is being chosen, « Recadrer » aims at {:?}, and the middle of the \
         part is {middle:?}",
        state.camera.target(),
    );
    let covered = covered_by(&state, &corners_of(bounds));
    assert!(
        covered >= FILLS_THE_CANVAS,
        "while a plane is being chosen, the part covers {covered} of the canvas once reframed",
    );
}

/// The one command the user reads as « Vue d'origine », found by that name so
/// that this file says what is asked rather than how it is spelled in the code.
fn the_origin_view() -> Command {
    let lang = Catalogue::french();
    Command::ALL
        .into_iter()
        .find(|command| crate::wording::command::label(&lang, *command) == "Vue d'origine")
        .expect("no command reads « Vue d'origine »: the opening view can be asked for by nobody")
}

fn ask_for_the_origin_view(document: &mut PartDocument, state: &mut ViewportState) {
    let mut editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let mut part = SketchContext {
        document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    crate::commands::run(the_origin_view(), &mut part, &mut Ribbon::default(), state);
}

fn land(state: &mut ViewportState) {
    if let Some(mut turn) = state.transition.take() {
        while turn.advance(&mut state.camera, 1.0) {}
    }
}

#[test]
fn the_origin_view_takes_the_opening_angles_framed_on_the_part() {
    let opens_with = ViewportState::default().camera;
    let mut document = a_block_far_from_the_origin();

    for (radius, (yaw, pitch), mode) in [
        (1.0, (0.3, -0.5), ViewMode::Free),
        (1e5, (3.0, 1.2), ViewMode::Free),
        (40.0, (-2.0, -1.4), ViewMode::Plane(WorkPlane::XZ)),
    ] {
        let turned_away = || {
            let mut state = a_view_turned_away(radius);
            state.camera.set_view_angles(yaw, pitch);
            state.mode = mode;
            state
        };
        let seen = format!("from {radius} away at yaw {yaw}, pitch {pitch}, in {mode:?}");
        let mut framed_by_reframe = turned_away();
        reframed(
            &mut document,
            &mut SketchEditor::default(),
            &mut framed_by_reframe,
        );
        let mut state = turned_away();

        ask_for_the_origin_view(&mut document, &mut state);
        land(&mut state);

        assert!(
            state.camera.forward().dot(opens_with.forward()) >= 1.0 - SAME_DIRECTION,
            "{seen}, « Vue d'origine » leaves the part seen along {:?}, and a new part \
             opens seen along {:?}",
            state.camera.forward(),
            opens_with.forward(),
        );
        assert!(
            (state.camera.target() - framed_by_reframe.camera.target()).length() < 1e-4,
            "{seen}, « Vue d'origine » aims at {:?}, and « Recadrer » aims at {:?}",
            state.camera.target(),
            framed_by_reframe.camera.target(),
        );
        assert!(
            (state.camera.distance() - framed_by_reframe.camera.distance()).abs() < 1e-3,
            "{seen}, « Vue d'origine » stands {} away, and « Recadrer » stands {} away",
            state.camera.distance(),
            framed_by_reframe.camera.distance(),
        );
        assert_eq!(
            state.mode,
            ViewMode::Free,
            "{seen}, the view ends oblique and still counts as lying on a plane, grid and all",
        );
    }
}

#[test]
fn the_origin_view_turns_the_camera_there_rather_than_jumping() {
    let mut document = a_block_far_from_the_origin();
    let mut state = a_view_turned_away(40.0);
    let seen_along = state.camera.forward();

    ask_for_the_origin_view(&mut document, &mut state);

    assert!(
        state.camera.forward().dot(seen_along) >= 1.0 - SAME_DIRECTION,
        "the frame « Vue d'origine » is asked on, the part is already seen along {:?}, \
         where it was seen along {seen_along:?}: the view jumped",
        state.camera.forward(),
    );
    assert!(
        state.transition.is_some(),
        "« Vue d'origine » asks for no turn: the camera is never going to move",
    );
}

#[test]
fn the_origin_view_of_a_part_with_no_volume_is_the_camera_a_new_part_opens_with() {
    let opens_with = ViewportState::default().camera;

    for (mut document, what) in [
        (PartDocument::new("part", Utc::now()), "an empty part"),
        (a_drawing_never_raised(), "a part drawn and never raised"),
    ] {
        let mut state = a_view_turned_away(1e5);
        state.camera.set_target(Vec3::new(50.0, -20.0, 80.0));
        state.mode = ViewMode::Plane(WorkPlane::XY);

        ask_for_the_origin_view(&mut document, &mut state);
        land(&mut state);

        assert!(
            state.camera.forward().dot(opens_with.forward()) >= 1.0 - SAME_DIRECTION,
            "on {what}, « Vue d'origine » leaves the part seen along {:?}, and a new part \
             opens seen along {:?}",
            state.camera.forward(),
            opens_with.forward(),
        );
        assert!(
            (state.camera.target() - opens_with.target()).length() < 1e-4,
            "on {what}, « Vue d'origine » aims at {:?}, and a new part opens aimed at {:?}",
            state.camera.target(),
            opens_with.target(),
        );
        assert!(
            (state.camera.distance() - opens_with.distance()).abs() < 1e-3,
            "on {what}, « Vue d'origine » stands {} away, and a new part opens {} away",
            state.camera.distance(),
            opens_with.distance(),
        );
        assert_eq!(
            state.mode,
            ViewMode::Free,
            "on {what}, the view ends oblique and still counts as lying on a plane",
        );
    }
}

#[test]
fn the_cube_keeps_its_corner_size_and_margin_settings() {
    let view = egui::Rect::from_min_size(egui::pos2(200.0, 100.0), egui::vec2(1000.0, 600.0));

    for (size, margin) in [(96.0, 12.0), (160.0, 30.0), (80.0, 4.0)] {
        for (cube_corner, wanted) in [
            (
                ViewportCorner::TopLeft,
                egui::pos2(view.min.x + margin, view.min.y + margin),
            ),
            (
                ViewportCorner::TopRight,
                egui::pos2(view.max.x - margin - size, view.min.y + margin),
            ),
            (
                ViewportCorner::BottomLeft,
                egui::pos2(view.min.x + margin, view.max.y - margin - size),
            ),
            (
                ViewportCorner::BottomRight,
                egui::pos2(
                    view.max.x - margin - size,
                    view.max.y - margin - size - BAR_TOP,
                ),
            ),
        ] {
            let config = ViewportConfig {
                cube_corner,
                cube_size: size,
                cube_margin: margin,
                ..ViewportConfig::default()
            };

            let cube = cube_rect(view, &config);

            assert!(
                (cube.min - wanted).length() < 1e-3,
                "a cube {size} wide set {margin} in at {cube_corner:?} stands at {:?}, not \
                 {wanted:?}",
                cube.min,
            );
            assert!(
                (cube.size() - egui::Vec2::splat(size)).length() < 1e-3,
                "a cube set {size} wide at {cube_corner:?} is {:?}",
                cube.size(),
            );
        }
    }
}
