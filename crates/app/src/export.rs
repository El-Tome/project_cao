//! The part written out for another program to read.

use std::path::PathBuf;

use cao_part::{Files, PartDocument, PartFileError};
use cao_prefs::Command;

use crate::screens::SketchContext;
use crate::wording;

/// What an export picked asks to be written: within `gap` millimetres of the
/// part, in millimetres.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct ExportRequest {
    pub gap: f64,
    pub millimetres_per_unit: f64,
}

/// The gap a level promises, in millimetres.
pub(crate) fn gap(command: Command) -> Option<f64> {
    match command {
        Command::ExportStlCoarse => Some(0.1),
        Command::ExportStlNormal => Some(0.02),
        Command::ExportStlFine => Some(0.005),
        _ => None,
    }
}

pub(crate) fn export_request(command: Command, document: &PartDocument) -> Option<ExportRequest> {
    Some(ExportRequest {
        gap: gap(command)?,
        millimetres_per_unit: document.scale(),
    })
}

/// The levels tried in place of one the triangles cannot hold, the coarser
/// first.
fn in_place_of(asked: Command) -> [Command; 2] {
    match asked {
        Command::ExportStlFine => [Command::ExportStlNormal, Command::ExportStlCoarse],
        Command::ExportStlNormal => [Command::ExportStlCoarse, Command::ExportStlFine],
        _ => [Command::ExportStlNormal, Command::ExportStlFine],
    }
}

/// The part written as an STL where `destination` says. A level the
/// triangles cannot hold asks for no place: the status line says so, and
/// names another level only if that one is written.
pub(crate) fn stl(
    asked: Command,
    request: ExportRequest,
    part: &mut SketchContext<'_>,
    files: &impl Files,
    destination: impl FnOnce(&str) -> Option<PathBuf>,
    error: &mut Option<String>,
) {
    let body = part.document.body();
    let scale = request.millimetres_per_unit;
    let Ok(bytes) = cao_solid::stl::of(body, request.gap, scale) else {
        let fits = in_place_of(asked).into_iter().find(|level| {
            gap(*level).is_some_and(|gap| cao_solid::stl::of(body, gap, scale).is_ok())
        });
        part.editor.message = Some(wording::export::uncovered(part.lang, asked, fits));
        return;
    };
    part.editor.message = None;
    let Some(path) = destination(part.document.name()) else {
        return;
    };
    match files.write(&path, &bytes) {
        Ok(()) => part.editor.message = Some(wording::export::written(part.lang, &path)),
        Err(failed) => {
            *error = Some(wording::part_file::say(
                part.lang,
                &PartFileError::File(failed),
            ));
        }
    }
}

/// The part's exact solid written as a STEP file where `destination` says,
/// in millimetres. A part on the flats asks for no place: the status line
/// says it is not exact.
pub(crate) fn step(
    part: &mut SketchContext<'_>,
    files: &impl Files,
    destination: impl FnOnce(&str) -> Option<PathBuf>,
    error: &mut Option<String>,
) {
    let Ok(text) = cao_solid::step::of(part.document.body(), part.document.scale()) else {
        part.editor.message = Some(wording::export::faceted(part.lang));
        return;
    };
    part.editor.message = None;
    let Some(path) = destination(part.document.name()) else {
        return;
    };
    match files.write(&path, text.as_bytes()) {
        Ok(()) => part.editor.message = Some(wording::export::step_written(part.lang, &path)),
        Err(failed) => {
            *error = Some(wording::part_file::say(
                part.lang,
                &PartFileError::File(failed),
            ))
        }
    }
}

#[cfg(test)]
mod tests;
