//! What app · screens/sketch.rs is held to.
//!
//! Closes #401.
//! - pointing the trim tool at a curve no longer lights the whole of it —
//!   `only_the_tool_that_takes_the_whole_thing_lights_the_whole_of_it`
//! - pointing Sélectionner at something still lights the whole of it —
//!   `only_the_tool_that_takes_the_whole_thing_lights_the_whole_of_it`
//!
//! Closes #176.
//! - `Échap` clears the measure, and so does taking another tool —
//!   `dropping_what_is_half_done_drops_the_measure_with_it`, which is what
//!   both gestures call
//! - a change to the drawing clears the measure —
//!   `a_measure_goes_when_the_drawing_it_read_moves`
//! - undo and redo clear it too — no test: both go through
//!   `clamp_editor_to_document`, which calls `reset_pending`, and that is the
//!   first test above
//! - a measure is not a step of the history — no test: nothing here writes to
//!   a document; it is asserted in `viewport/input/reading/tests.rs`
//!
//! Closes #728 — the whole transcription is in `crates/app/tests/view_bar.rs`;
//! this is the bullet the editor answers.
//! - picking « cachée », « transparente » or « opaque » sets the open sketch's
//!   matter in front to it directly, from whichever state it was in, rather
//!   than stepping to the next —
//!   `picking_a_state_of_the_matter_in_front_sets_the_open_sketch_to_it_whatever_it_was`

use super::*;

use cao_render::MatterInFront;
use cao_sketch::{DimensionPicks, DimensionTarget, Measured, PointId};

fn measuring() -> SketchEditor {
    SketchEditor {
        tool: Tool::Measure,
        tool_state: ToolState::Measure {
            picks: DimensionPicks::default(),
            showing: Some(Measured::Of(DimensionTarget::Distance {
                from: PointId(1),
                to: PointId(2),
            })),
        },
        ..SketchEditor::default()
    }
}

fn is_showing(editor: &SketchEditor) -> bool {
    matches!(
        editor.tool_state,
        ToolState::Measure {
            showing: Some(_),
            ..
        }
    )
}

#[test]
fn dropping_what_is_half_done_drops_the_measure_with_it() {
    let mut editor = measuring();
    assert!(
        is_showing(&editor),
        "the measure is on screen to begin with"
    );

    editor.reset_pending();

    assert!(
        !is_showing(&editor),
        "Échap and every change of tool come through here, and a measure \
         surviving one of them would outlive the gesture that asked for it",
    );
}

#[test]
fn a_measure_goes_when_the_drawing_it_read_moves() {
    let mut editor = measuring();

    editor.forget_the_measure();

    assert!(
        !is_showing(&editor),
        "a value read off a drawing that has since moved is no longer true",
    );
}

#[test]
fn forgetting_the_measure_leaves_another_tool_s_work_alone() {
    let mut editor = SketchEditor {
        tool: Tool::Select,
        tool_state: ToolState::Select(Box::default()),
        ..SketchEditor::default()
    };

    editor.forget_the_measure();

    assert!(
        matches!(editor.tool_state, ToolState::Select(_)),
        "only the measure is dropped: a selection held while the drawing \
         changes is the selection tool's own business",
    );
}

#[test]
fn only_the_tool_that_takes_the_whole_thing_lights_the_whole_of_it() {
    assert!(
        Tool::Select.lights_the_whole_of_it(),
        "picking takes the whole of what is pointed at",
    );

    for tool in [Tool::Trim, Tool::Split, Tool::Chamfer, Tool::Fillet] {
        assert!(
            !tool.lights_the_whole_of_it(),
            "{tool:?} takes a stretch, and lighting the whole promises otherwise",
        );
    }
}

#[test]
fn picking_a_state_of_the_matter_in_front_sets_the_open_sketch_to_it_whatever_it_was() {
    let the_three = [
        MatterInFront::Hidden,
        MatterInFront::SeeThrough,
        MatterInFront::Opaque,
    ];

    for was in the_three {
        for picked in the_three {
            let mut editor = SketchEditor::default();
            editor.begin_editing(0, WorkPlane::XY);
            editor.matter_in_front = was;

            editor.set_matter_in_front(picked);

            assert_eq!(
                editor.matter_in_front, picked,
                "the sketch's matter in front was {was:?}, {picked:?} was picked",
            );
        }
    }
}
