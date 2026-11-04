//! The part written out for another program to read.

use cao_part::{Files, PartFileError};
use cao_prefs::Command;

use crate::adapters::save_dialog;
use crate::commands::ExportRequest;
use crate::screens::SketchContext;
use crate::wording;

/// The part written as an STL where the user picks. A level the triangles
/// cannot hold is said on the status line, and no place is asked for.
pub(crate) fn stl(
    asked: Command,
    request: ExportRequest,
    part: &mut SketchContext<'_>,
    files: &impl Files,
    error: &mut Option<String>,
) {
    let ExportRequest {
        gap,
        millimetres_per_unit,
    } = request;
    let Ok(bytes) = cao_solid::stl::of(part.document.body(), gap, millimetres_per_unit) else {
        part.editor.message = Some(wording::export::uncovered(part.lang, asked));
        return;
    };
    if let Some(path) = save_dialog::stl_destination(part.document.name())
        && let Err(failed) = files.write(&path, &bytes)
    {
        *error = Some(wording::part_file::say(
            part.lang,
            &PartFileError::File(failed),
        ));
    }
}
