//! What app · wording/export.rs is held to.
//!
//! Closes #580.
//! - a body the triangles cannot cover at the level asked is not written, and
//!   the message says so — the message says nothing was written and names
//!   another level to try —
//!   `an_export_the_triangles_cannot_cover_says_nothing_was_written_and_names_another_level`

use cao_prefs::Command;

use super::*;
use crate::lang::Catalogue;
use crate::wording::command::label;

const LEVELS: [Command; 3] = [
    Command::ExportStlCoarse,
    Command::ExportStlNormal,
    Command::ExportStlFine,
];

#[test]
fn an_export_the_triangles_cannot_cover_says_nothing_was_written_and_names_another_level() {
    let lang = Catalogue::french();

    for asked in LEVELS {
        let said = uncovered(&lang, asked);

        assert!(
            said.contains("n'a pas été écrit"),
            "the reader is told no file was written: {said}",
        );
        let others: Vec<String> = LEVELS
            .into_iter()
            .filter(|level| *level != asked)
            .map(|level| label(&lang, level))
            .collect();
        assert!(
            others.iter().any(|other| said.contains(other.as_str())),
            "asked {asked:?}, the reader is told another level to try, among {others:?}: {said}",
        );
    }
}
