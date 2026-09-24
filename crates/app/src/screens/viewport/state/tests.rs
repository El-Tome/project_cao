//! What app · screens/viewport/state.rs is held to.
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

use cao_part::{ExtrusionMode, PartDocument};
use chrono::Utc;

use super::*;
use crate::lang::Catalogue;
use crate::screens::extrusion::ExtrusionState;
use crate::screens::sketch::SketchEditor;

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

    state.blink(vec![(0, side)], 10.0);

    assert_eq!(state.blinking_on(0, 10.1), Some((vec![side], true)));
    assert_eq!(state.blinking_on(0, 10.3), Some((vec![side], false)));
    assert_eq!(
        state.blinking_on(1, 10.1),
        None,
        "only on the sketch it is on"
    );
    assert_eq!(state.blinking_on(0, 14.0), None, "and not for ever");
}
