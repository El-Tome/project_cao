use cao_prefs::Command;

use crate::lang::Catalogue;
use crate::wording::command::label;

/// Why nothing was written at the level `asked`, and the level to try instead.
pub fn uncovered(lang: &Catalogue, asked: Command) -> String {
    let other = match asked {
        Command::ExportStlNormal => Command::ExportStlCoarse,
        _ => Command::ExportStlNormal,
    };
    lang.t_with(
        "export.uncovered",
        &[
            ("asked", &label(lang, asked)),
            ("other", &label(lang, other)),
        ],
    )
}

#[cfg(test)]
mod tests;
