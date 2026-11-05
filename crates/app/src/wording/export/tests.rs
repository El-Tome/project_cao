//! What app · wording/export.rs is held to.
//!
//! Closes #580.
//! - a body the triangles cannot cover at the level asked is not written, and
//!   the message says so — the message says nothing was written and names
//!   the level found to fit —
//!   `an_export_the_triangles_cannot_cover_says_nothing_was_written_and_names_the_level_that_fits`;
//!   or that none does —
//!   `an_export_no_level_covers_says_nothing_was_written_and_that_no_level_fits`
//!
//! Closes #581.
//! - a part on the flats is refused: the message says no STEP was written and
//!   that the part's surfaces are not exact —
//!   `a_part_on_the_flats_is_told_no_step_is_written_because_it_is_not_exact`

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
fn an_export_the_triangles_cannot_cover_says_nothing_was_written_and_names_the_level_that_fits() {
    let lang = Catalogue::french();

    for asked in LEVELS {
        for fits in LEVELS.into_iter().filter(|level| *level != asked) {
            let said = uncovered(&lang, asked, Some(fits));

            assert!(
                said.contains("n'a pas été écrit"),
                "the reader is told no file was written: {said}",
            );
            assert!(
                said.contains(&format!("« {} »", label(&lang, fits))),
                "asked {asked:?}, the reader is told {fits:?} fits: {said}",
            );
        }
    }
}

#[test]
fn an_export_no_level_covers_says_nothing_was_written_and_that_no_level_fits() {
    let lang = Catalogue::french();

    for asked in LEVELS {
        let said = uncovered(&lang, asked, None);

        assert!(
            said.contains("n'a pas été écrit"),
            "the reader is told no file was written: {said}",
        );
        for other in LEVELS.into_iter().filter(|level| *level != asked) {
            assert!(
                !said.contains(&format!("« {} »", label(&lang, other))),
                "asked {asked:?}, no level fits and {other:?} is suggested: {said}",
            );
        }
        assert!(
            LEVELS
                .into_iter()
                .filter(|level| *level != asked)
                .all(|fits| uncovered(&lang, asked, Some(fits)) != said),
            "that no level fits reads as a level suggested: {said}",
        );
    }
}

#[test]
fn a_part_on_the_flats_is_told_no_step_is_written_because_it_is_not_exact() {
    let said = faceted(&Catalogue::french());

    assert!(
        said.contains("n'a pas été écrit"),
        "the reader is told no file was written: {said}",
    );
    assert!(
        said.contains("STEP"),
        "the reader is told which export: {said}"
    );
    assert!(
        said.contains("exact"),
        "the reader is told the part is not exact: {said}",
    );
}
