use std::path::Path;

use cao_prefs::Command;
use cao_solid::step::Unwritten;

use crate::lang::Catalogue;
use crate::wording::command::label;

/// Why nothing was written at the level `asked`, and the level that `fits`
/// instead, when one does.
pub fn uncovered(lang: &Catalogue, asked: Command, fits: Option<Command>) -> String {
    let asked = label(lang, asked);
    match fits {
        Some(fits) => lang.t_with(
            "export.uncovered",
            &[("asked", &asked), ("fits", &label(lang, fits))],
        ),
        None => lang.t_with("export.uncovered_at_every_level", &[("asked", &asked)]),
    }
}

pub fn written(lang: &Catalogue, path: &Path) -> String {
    lang.t_with("export.written", &[("path", &path.display().to_string())])
}

pub fn step_written(lang: &Catalogue, path: &Path) -> String {
    lang.t_with(
        "export.step_written",
        &[("path", &path.display().to_string())],
    )
}

pub fn faceted(lang: &Catalogue) -> String {
    lang.t("export.faceted")
}

pub fn unwritten(lang: &Catalogue, refused: Unwritten) -> String {
    match refused {
        Unwritten::Faceted => faceted(lang),
        Unwritten::Unfitted => lang.t("export.unfitted"),
    }
}

#[cfg(test)]
mod tests;
