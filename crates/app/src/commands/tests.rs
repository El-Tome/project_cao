//! What app · commands.rs is held to.
//!
//! Closes #210.
//! - `run` takes the part being worked on as one `SketchContext` instead of
//!   four parameters — no test: the signature is what the compiler holds, and
//!   the harness below could not be written against the old one without
//!   threading each of the four by hand
//! - `SketchContext` lives in `screens/`, not in `screens/viewport/` — no test:
//!   the path this file imports it from is the check, and it fails to compile
//!   if the type goes back
//! - a command clears the message the last one left in the title bar —
//!   `a_command_clears_the_message_the_last_one_left`
//! - `app.rs` stays under its line budget — no test: `LINE_BUDGET` in
//!   `architecture.rs` already weighs it, and it is not on the list of files
//!   over it

use cao_part::PartDocument;
use cao_prefs::Command;
use chrono::Utc;

use super::run;
use crate::lang::Catalogue;
use crate::screens::SketchContext;
use crate::screens::extrusion::ExtrusionState;
use crate::screens::ribbon::Ribbon;
use crate::screens::sketch::SketchEditor;
use crate::screens::viewport::ViewportState;

#[test]
fn a_command_clears_the_message_the_last_one_left() {
    let mut document = PartDocument::new("part", Utc::now());
    let mut editor = SketchEditor {
        message: Some("left by the last command".into()),
        ..SketchEditor::default()
    };
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let mut part = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };

    run(
        Command::Undo,
        &mut part,
        &mut Ribbon::default(),
        &mut ViewportState::default(),
    );

    assert_eq!(
        editor.message, None,
        "an undo that had nothing to undo still leaves no old message standing"
    );
}
