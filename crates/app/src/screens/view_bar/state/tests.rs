//! What app · screens/view_bar/state.rs is held to.
//!
//! Closes #728 — the whole transcription is in `crates/app/tests/view_bar.rs`;
//! these bullets are the ones the bar's presenter answers, with no window.
//! - in a sketch, the bar's « Matière devant » offers « cachée »,
//!   « transparente » and « opaque », in that order, each the state it names —
//!   `the_matter_in_front_menu_offers_hidden_see_through_and_opaque_in_that_order`
//! - it reads « Matière devant : cachée » after the open sketch's state, and
//!   marks that state among the three —
//!   `the_matter_in_front_menu_reads_and_marks_the_state_the_open_sketch_is_in`;
//!   the ▾ that says it opens is the drawing's, and the driver reads it once
//!   the bar is drawn
//! - outside a sketch it is greyed —
//!   `outside_a_sketch_the_matter_in_front_menu_is_greyed`

use cao_render::MatterInFront;
use cao_sketch::WorkPlane;

use super::*;
use crate::lang::Catalogue;
use crate::screens::sketch::SketchEditor;

fn drawing_on_a_sketch(matter_in_front: MatterInFront) -> SketchEditor {
    let mut editor = SketchEditor::default();
    editor.begin_editing(0, WorkPlane::XY);
    editor.matter_in_front = matter_in_front;
    editor
}

#[test]
fn the_matter_in_front_menu_offers_hidden_see_through_and_opaque_in_that_order() {
    let menu = MatterInFrontMenu::of(
        &drawing_on_a_sketch(MatterInFront::Opaque),
        &Catalogue::french(),
    );

    let offered: Vec<(&str, MatterInFront)> = menu
        .choices
        .iter()
        .map(|choice| (choice.label.as_str(), choice.state))
        .collect();

    assert_eq!(
        offered,
        [
            ("cachée", MatterInFront::Hidden),
            ("transparente", MatterInFront::SeeThrough),
            ("opaque", MatterInFront::Opaque),
        ],
        "the menu does not offer the three states, each by its name, in the design's order",
    );
}

#[test]
fn the_matter_in_front_menu_reads_and_marks_the_state_the_open_sketch_is_in() {
    let lang = Catalogue::french();

    for (state, reads) in [
        (MatterInFront::Hidden, "Matière devant : cachée"),
        (MatterInFront::SeeThrough, "Matière devant : transparente"),
        (MatterInFront::Opaque, "Matière devant : opaque"),
    ] {
        let menu = MatterInFrontMenu::of(&drawing_on_a_sketch(state), &lang);

        assert_eq!(
            menu.label, reads,
            "the sketch's matter in front is {state:?}"
        );
        let marked: Vec<MatterInFront> = menu
            .choices
            .iter()
            .filter(|choice| choice.current)
            .map(|choice| choice.state)
            .collect();
        assert_eq!(
            marked,
            [state],
            "the sketch's matter in front is {state:?}, and the menu marks {marked:?}",
        );
    }
}

#[test]
fn outside_a_sketch_the_matter_in_front_menu_is_greyed() {
    let lang = Catalogue::french();
    let mut finished = drawing_on_a_sketch(MatterInFront::Hidden);
    finished.close();

    assert!(
        MatterInFrontMenu::of(&drawing_on_a_sketch(MatterInFront::Hidden), &lang).enabled,
        "« Matière devant » is greyed while a sketch is open",
    );
    for (editor, when) in [
        (SketchEditor::default(), "before a sketch is opened"),
        (finished, "once the sketch is finished"),
    ] {
        assert!(
            !MatterInFrontMenu::of(&editor, &lang).enabled,
            "« Matière devant » can be picked {when}, with no sketch to show the matter \
             in front of",
        );
    }
}
