//! What app · export.rs is held to.
//!
//! Closes #580.
//! - a body the triangles cannot cover at the level asked is not written, and
//!   the message says so — no place is asked and nothing is written —
//!   `a_declined_export_asks_no_destination_writes_nothing_and_says_so`; the
//!   level suggested is one that is written —
//!   `a_level_refused_suggests_only_a_level_that_is_written`; and when none
//!   is, the message says no level fits —
//!   `a_body_no_level_covers_is_told_that_no_level_fits`

use std::cell::Cell;
use std::path::{Path, PathBuf};

use cao_part::{ExtrusionMode, FileError, Files, InMemoryFiles, Operation, PartDocument, PointRef};
use cao_prefs::Command;
use cao_sketch::WorkPlane;
use chrono::Utc;
use glam::DVec2;

use super::stl;
use crate::export::export_request;
use crate::lang::Catalogue;
use crate::screens::SketchContext;
use crate::screens::extrusion::ExtrusionState;
use crate::screens::sketch::SketchEditor;
use crate::wording;

fn a_block() -> PartDocument {
    let mut document = PartDocument::new("Platine", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::ZERO),
        opposite: PointRef::New(DVec2::new(10.0, 20.0)),
        construction: false,
    });
    raised(document, DVec2::new(5.0, 10.0))
}

/// A disc of `radius` millimetres, 10 high: past a radius of 1062 mm the 1024
/// chords a turn holds leave « Fine » behind, past 21 m « Grossière » too.
fn a_disc(radius: f64) -> PartDocument {
    let mut document = PartDocument::new("Disque", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddCircle {
        sketch: 0,
        center: PointRef::New(DVec2::ZERO),
        radius,
        rim: Vec::new(),
        construction: false,
    });
    raised(document, DVec2::ZERO)
}

fn raised(mut document: PartDocument, inside: DVec2) -> PartDocument {
    document.apply(Operation::Extrude {
        sketch: 0,
        areas: document.areas_at(0, &[inside]),
        distance: 10.0.into(),
        mode: ExtrusionMode::Add,
    });
    assert!(!document.body().is_empty(), "the part holds matter");
    document
}

/// What the status line and the error bar say once the part is exported, an
/// older message standing on the status line before.
struct Said {
    message: Option<String>,
    error: Option<String>,
}

fn exported(
    document: &mut PartDocument,
    level: Command,
    files: &impl Files,
    destination: impl FnOnce(&str) -> Option<PathBuf>,
) -> Said {
    let request = export_request(level, document).expect("a level asks for an export");
    let mut editor = SketchEditor {
        message: Some("left by the last command".into()),
        ..SketchEditor::default()
    };
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let mut error = None;
    let mut part = SketchContext {
        document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };

    stl(level, request, &mut part, files, destination, &mut error);

    Said {
        message: editor.message,
        error,
    }
}

fn never_asked(_: &str) -> Option<PathBuf> {
    panic!("a place to write to is asked for a file that will not be written")
}

struct RefusingFiles;

impl Files for RefusingFiles {
    fn read(&self, path: &Path) -> Result<Vec<u8>, FileError> {
        Err(FileError::Absent(path.to_path_buf()))
    }

    fn write(&self, path: &Path, _: &[u8]) -> Result<(), FileError> {
        Err(FileError::Refused(path.to_path_buf()))
    }

    fn exists(&self, _: &Path) -> bool {
        false
    }
}

#[test]
fn a_declined_export_asks_no_destination_writes_nothing_and_says_so() {
    let files = InMemoryFiles::default();

    let said = exported(
        &mut a_disc(2000.0),
        Command::ExportStlFine,
        &files,
        never_asked,
    );

    let message = said.message.expect("the status line says why");
    assert!(
        message.contains("n'a pas été écrit"),
        "the status line says no file was written: {message}"
    );
    assert_eq!(said.error, None);
}

#[test]
fn a_level_refused_suggests_only_a_level_that_is_written() {
    let lang = Catalogue::french();
    let mut disc = a_disc(2000.0);

    let said = exported(
        &mut disc,
        Command::ExportStlFine,
        &InMemoryFiles::default(),
        never_asked,
    );

    let message = said.message.expect("the status line says why");
    let suggested = [Command::ExportStlNormal, Command::ExportStlCoarse]
        .into_iter()
        .find(|level| {
            message == wording::export::uncovered(&lang, Command::ExportStlFine, Some(*level))
        })
        .unwrap_or_else(|| panic!("no coarser level is suggested: {message}"));
    let files = InMemoryFiles::default();
    let path = PathBuf::from("/exports/Disque.stl");
    let written = exported(&mut disc, suggested, &files, |_| Some(path.clone()));
    assert!(
        files.exists(&path),
        "{suggested:?} was suggested and writes nothing: {:?}",
        written.message
    );
}

#[test]
fn a_body_no_level_covers_is_told_that_no_level_fits() {
    let lang = Catalogue::french();
    let mut disc = a_disc(25_000.0);

    for level in [
        Command::ExportStlCoarse,
        Command::ExportStlNormal,
        Command::ExportStlFine,
    ] {
        let said = exported(&mut disc, level, &InMemoryFiles::default(), never_asked);

        assert_eq!(
            said.message,
            Some(wording::export::uncovered(&lang, level, None)),
            "a disc 50 m across fits no level, asked {level:?}"
        );
    }
}

#[test]
fn an_export_writes_the_bytes_where_the_user_chose_clears_the_last_message_and_says_where() {
    let mut document = a_block();
    let files = InMemoryFiles::default();
    let path = PathBuf::from("/exports/plaque.stl");
    let offered = Cell::new(String::new());

    let said = exported(&mut document, Command::ExportStlNormal, &files, |name| {
        offered.set(name.to_string());
        Some(path.clone())
    });

    let request = export_request(Command::ExportStlNormal, &document).expect("an export");
    let bytes = cao_solid::stl::of(document.body(), request.gap, request.millimetres_per_unit)
        .expect("a block is covered");
    assert_eq!(
        files.read(&path).ok(),
        Some(bytes),
        "the file holds the part"
    );
    assert!(
        offered.take().contains("Platine"),
        "the place asked for is offered under the part's name"
    );
    let message = said.message.expect("the status line says where");
    assert!(
        message.contains(&path.display().to_string()) && !message.contains("left by"),
        "the status line says where the file went, and only that: {message}"
    );
    assert_eq!(said.error, None);
}

#[test]
fn a_save_dialog_closed_writes_nothing_and_leaves_no_message() {
    let files = InMemoryFiles::default();

    let said = exported(&mut a_block(), Command::ExportStlNormal, &files, |_| None);

    assert_eq!(said.message, None, "a cancelled export says nothing");
    assert_eq!(said.error, None);
    assert!(
        !files.exists(Path::new("/")),
        "nothing was written anywhere"
    );
}

#[test]
fn a_file_the_system_refuses_fills_the_error_bar() {
    let path = PathBuf::from("/locked/plaque.stl");

    let said = exported(
        &mut a_block(),
        Command::ExportStlNormal,
        &RefusingFiles,
        |_| Some(path.clone()),
    );

    let error = said.error.expect("the error bar says the write failed");
    assert!(
        error.contains(&path.display().to_string()),
        "the error bar names the file refused: {error}"
    );
}
