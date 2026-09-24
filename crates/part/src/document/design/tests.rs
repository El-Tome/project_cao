use std::io::Cursor;

use cao_sketch::{SketchAxis, WorkPlane};
use glam::DVec2;

use super::*;
use crate::adapters::InMemoryFiles;
use crate::document::PartDocument;
use crate::history::{PointRef, RevolutionAxis};
use crate::ports::Files;

fn at(text: &str) -> chrono::DateTime<chrono::Utc> {
    text.parse().expect("a date")
}

/// Every kind of step, each opened by an operation the index has to take
/// apart and put back together.
fn a_part_of_every_kind() -> PartDocument {
    let mut document = PartDocument::new("Test", at("2026-01-02T09:00:00Z"));
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::new(1.0, 1.0)),
        opposite: PointRef::New(DVec2::new(11.0, 21.0)),
        construction: false,
    });
    document.apply(Operation::Extrude {
        sketch: 0,
        areas: document.areas_at(0, &[DVec2::new(6.0, 11.0)]),
        distance: 4.0.into(),
        mode: ExtrusionMode::Cut,
    });
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XZ,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 1,
        corner: PointRef::New(DVec2::new(2.0, 0.0)),
        opposite: PointRef::New(DVec2::new(6.0, 8.0)),
        construction: false,
    });
    document.apply(Operation::Revolve {
        sketch: 1,
        areas: document.areas_at(1, &[DVec2::new(4.0, 4.0)]),
        axis: RevolutionAxis::Sketch(SketchAxis::V),
        angle: 90.0.into(),
        mode: ExtrusionMode::Add,
    });
    document
}

fn written(document: &PartDocument) -> (InMemoryFiles, std::path::PathBuf) {
    let files = InMemoryFiles::default();
    let path = std::path::PathBuf::from("/parts/piece.caopart");
    document
        .save(&files, &path, at("2026-01-02T10:00:00Z"))
        .expect("the part is written");
    (files, path)
}

fn design_in(files: &InMemoryFiles, path: &std::path::Path) -> History {
    let bytes = files.read(path).expect("the archive is there");
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes)).expect("a zip archive");
    read(&mut archive).expect("a design").0
}

#[test]
fn an_operation_taken_apart_into_the_index_and_its_folder_comes_back_whole() {
    let document = a_part_of_every_kind();
    let (files, path) = written(&document);

    let design = design_in(&files, &path);

    assert_eq!(
        design.operations(),
        document.history.operations(),
        "a step's opening operation is split between the index and its \
         folder, and what comes back must be the operation that went in",
    );
    assert_eq!(design.applied(), document.history.applied());
    assert_eq!(design.steps(), document.history.steps());
}

#[test]
fn what_a_step_stands_on_is_in_the_index_and_what_it_sets_is_in_its_folder() {
    let (files, path) = written(&a_part_of_every_kind());
    let bytes = files.read(&path).expect("the archive is there");
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes)).expect("a zip archive");

    let index = crate::document::read_entry(&mut archive, INDEX_ENTRY).expect("the index");
    let folder =
        crate::document::read_entry(&mut archive, "design/extrusion-0/steps.json").expect("a step");

    assert!(index.contains("\"areas\""), "{index}");
    assert!(
        !index.contains("\"distance\""),
        "a distance points at nothing and cannot be lost, so the index has no \
         business holding it: {index}",
    );
    assert!(folder.contains("\"distance\""), "{folder}");
    assert!(
        !folder.contains("\"areas\""),
        "the areas clicked are said once, in the index: {folder}",
    );
}

#[test]
fn a_sketch_leaves_nothing_in_its_folder_but_what_was_drawn_on_it() {
    let (files, path) = written(&a_part_of_every_kind());
    let bytes = files.read(&path).expect("the archive is there");
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes)).expect("a zip archive");

    let folder =
        crate::document::read_entry(&mut archive, "design/sketch-0/steps.json").expect("a step");

    assert!(
        !folder.contains("CreateSketch"),
        "a sketch is its plane, and its plane is in the index: {folder}",
    );
    assert!(folder.contains("AddRectangle"), "{folder}");
    assert_eq!(
        design_in(&files, &path).steps()[0].operations().len(),
        2,
        "the making of the sketch keeps its number even though the folder no \
         longer holds it, or undo could not take it back",
    );
}

#[test]
fn each_kind_of_step_is_numbered_in_the_rank_the_operations_already_speak_in() {
    let (files, path) = written(&a_part_of_every_kind());
    let bytes = files.read(&path).expect("the archive is there");
    let archive = zip::ZipArchive::new(Cursor::new(bytes)).expect("a zip archive");

    let entries: Vec<String> = archive.file_names().map(str::to_string).collect();

    for wanted in [
        "design/sketch-0/steps.json",
        "design/extrusion-0/steps.json",
        "design/sketch-1/steps.json",
        "design/revolution-0/steps.json",
    ] {
        assert!(entries.contains(&wanted.to_string()), "{entries:?}");
    }
}

fn a_variable(name: &str, value: f64) -> Operation {
    Operation::Variable(crate::variables::VariableChange::Added {
        name: name.to_string(),
        formula: crate::formula::Formula::Number(value),
    })
}

#[test]
fn the_variables_are_a_table_of_the_part_kept_beside_the_steps_and_come_back() {
    let mut document = PartDocument::new("Test", at("2026-01-02T09:00:00Z"));
    document.apply(a_variable("width", 120.0));
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(a_variable("height", 40.0));
    let (files, path) = written(&document);
    let bytes = files.read(&path).expect("the archive is there");
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes)).expect("a zip archive");

    let table = crate::document::read_entry(&mut archive, VARIABLES_ENTRY).expect("the variables");
    let design = design_in(&files, &path);

    assert!(
        table.contains("width") && table.contains("height"),
        "{table}"
    );
    assert_eq!(design.operations(), document.history.operations());
    assert_eq!(
        design.variable_changes(),
        document.history.variable_changes()
    );
    assert!(
        design.steps()[0].operations() == [2],
        "the sketch holds none of them"
    );
}

#[test]
fn a_part_with_no_variables_writes_no_table() {
    let (files, path) = written(&a_part_of_every_kind());
    let bytes = files.read(&path).expect("the archive is there");
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes)).expect("a zip archive");

    let index = crate::document::read_entry(&mut archive, INDEX_ENTRY).expect("the index");

    assert!(archive.by_name(VARIABLES_ENTRY).is_err());
    assert!(!index.contains("variables"), "{index}");
}
