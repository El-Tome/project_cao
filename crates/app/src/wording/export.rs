use std::path::Path;

use cao_prefs::Command;

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

#[cfg(test)]
mod tests;
