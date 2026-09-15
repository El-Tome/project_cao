use cao_sketch::{DimensionTarget, SegmentId, WorkPlane};
use std::path::Path;

use crate::adapters::InMemoryFiles;
use crate::history::PointRef;
use glam::DVec2;

use super::*;

fn at(text: &str) -> DateTime<Utc> {
    text.parse().expect("a date")
}

fn drawn_part() -> PartDocument {
    let mut document = PartDocument::new("Test", at("2026-01-02T09:00:00Z"));
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
    });
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::ZERO),
        end: PointRef::New(DVec2::new(2.0, 0.0)),
        construction: false,
    });
    document.apply(Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Length(SegmentId(0)),
        value: 100.0,
        placement: None,
    });
    document
}

#[test]
fn a_part_is_stamped_with_the_hour_it_is_handed() {
    let opened = at("2026-01-02T09:00:00Z");
    let document = PartDocument::new("Support", opened);

    assert_eq!(document.metadata.created_at, opened);
    assert_eq!(document.metadata.modified_at, opened);
}

#[test]
fn drawing_in_a_part_does_not_pass_for_writing_it_down() {
    let opened = at("2026-01-02T09:00:00Z");
    let mut document = PartDocument::new("Support", opened);

    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
    });

    assert_eq!(document.metadata.modified_at, opened);
}

#[test]
fn a_saved_part_carries_the_hour_it_was_written_at() {
    let files = InMemoryFiles::default();
    let path = Path::new("/parts/piece.caopart");

    let opened = at("2026-01-02T09:00:00Z");
    let written = at("2026-01-02T17:30:00Z");
    let document = PartDocument::new("Support", opened);
    document.save(&files, path, written).expect("saves");

    let reloaded = PartDocument::load(&files, path).expect("loads");
    assert_eq!(reloaded.metadata.created_at, opened);
    assert_eq!(reloaded.metadata.modified_at, written);
    assert_eq!(
        document.metadata.modified_at, opened,
        "the hour goes into the file, not back onto the part",
    );
}

#[test]
fn a_part_survives_a_save_and_reload() {
    let files = InMemoryFiles::default();
    let path = Path::new("/parts/piece.caopart");

    let document = drawn_part();
    document
        .save(&files, path, at("2026-01-02T09:05:00Z"))
        .expect("saves");
    let reloaded = PartDocument::load(&files, path).expect("loads");

    assert!(files.read(path).expect("bytes").starts_with(b"PK"), "zip");
    assert_eq!(reloaded.name(), "Test");
    assert_eq!(reloaded.history, document.history);
    assert_eq!(reloaded.sketches().len(), 1);
    assert_eq!(reloaded.scale(), 50.0);
}

#[test]
fn the_redo_tail_survives_a_save_and_reload() {
    let files = InMemoryFiles::default();
    let path = Path::new("/parts/piece.caopart");

    let mut document = drawn_part();
    document.undo();
    document
        .save(&files, path, at("2026-01-02T09:05:00Z"))
        .expect("saves");

    let mut reloaded = PartDocument::load(&files, path).expect("loads");
    assert!(reloaded.history.can_redo());
    assert!(!reloaded.has_scale(), "the dimension is undone");
    assert!(reloaded.redo());
    assert_eq!(reloaded.scale(), 50.0);
}

#[test]
fn undo_and_redo_rebuild_the_geometry() {
    let mut document = drawn_part();
    assert_eq!(document.sketches()[0].segments().len(), 1);

    assert!(document.undo());
    assert!(!document.has_scale());
    assert!(document.undo());
    assert_eq!(document.sketches()[0].segments().len(), 0);

    assert!(document.redo());
    assert_eq!(document.sketches()[0].segments().len(), 1);
}

#[test]
fn rewinding_puts_the_part_back_to_a_step() {
    let mut document = drawn_part();
    document.rewind_to(1);
    assert_eq!(document.sketches().len(), 1);
    assert!(document.sketches()[0].segments().is_empty());

    document.rewind_to(3);
    assert_eq!(document.sketches()[0].segments().len(), 1);
    assert!(document.has_scale());
}

#[test]
fn a_part_from_an_older_version_is_refused() {
    let files = InMemoryFiles::default();
    let path = Path::new("/parts/piece.caopart");

    PartDocument::new("Older", at("2026-01-02T09:00:00Z"))
        .save(&files, path, at("2026-01-02T09:00:00Z"))
        .expect("saves");
    let mut document = PartDocument::load(&files, path).expect("loads");
    document.metadata.schema_version = 2;
    document
        .save(&files, path, at("2026-01-02T09:00:00Z"))
        .expect("saves again");

    let error = PartDocument::load(&files, path).expect_err("must be refused");
    assert!(matches!(error, PartFileError::UnsupportedVersion(2)));
}

#[test]
fn a_second_part_of_the_same_name_does_not_destroy_the_first() {
    let files = InMemoryFiles::default();
    let directory = Path::new("/parts");

    let (first, first_path) =
        PartDocument::create_in(&files, directory, "Support", at("2026-01-02T09:00:00Z"))
            .expect("creates");
    let (second, second_path) =
        PartDocument::create_in(&files, directory, "Support", at("2026-01-02T09:01:00Z"))
            .expect("creates a second");
    let reopened = PartDocument::load(&files, &first_path).expect("the first one is still there");

    assert_ne!(first_path, second_path);
    assert_eq!(second.name(), "Support 2");
    assert_eq!(reopened.metadata.id, first.metadata.id);
    assert_eq!(reopened.name(), "Support");
}

#[test]
fn a_plain_json_file_is_refused() {
    let files = InMemoryFiles::default();
    let path = Path::new("/parts/piece.caopart");
    files
        .write(path, br#"{"name":"ancienne"}"#)
        .expect("writes");

    let error = PartDocument::load(&files, path).expect_err("must be refused");
    assert!(matches!(error, PartFileError::UnsupportedVersion(1)));
}

fn a_drawn_picture() -> Picture {
    Picture::new(2, 2, (0..16).collect()).expect("the pixels fill the size")
}

#[test]
fn a_part_hands_back_the_picture_it_was_written_with() {
    let files = InMemoryFiles::default();
    let path = Path::new("/parts/piece.caopart");
    let mut document = drawn_part();
    document.set_picture(a_drawn_picture());

    document
        .save(&files, path, at("2026-01-02T10:00:00Z"))
        .expect("the part is written");

    assert_eq!(
        PartDocument::load(&files, path).expect("reads").picture(),
        Some(&a_drawn_picture()),
    );
}

#[test]
fn a_part_nobody_has_taken_the_picture_of_opens_without_one() {
    let files = InMemoryFiles::default();
    let path = Path::new("/parts/piece.caopart");
    drawn_part()
        .save(&files, path, at("2026-01-02T10:00:00Z"))
        .expect("the part is written");

    assert_eq!(
        PartDocument::load(&files, path).expect("reads").picture(),
        None,
        "a part written before pictures existed must open exactly as it did",
    );
}

#[test]
fn a_picture_is_pulled_out_on_its_own_without_the_history_being_replayed() {
    let files = InMemoryFiles::default();
    let path = Path::new("/parts/piece.caopart");
    let mut document = drawn_part();
    document.set_picture(a_drawn_picture());
    document
        .save(&files, path, at("2026-01-02T10:00:00Z"))
        .expect("the part is written");

    assert_eq!(
        PartDocument::picture_in(&files, path),
        Some(a_drawn_picture()),
    );
}

#[test]
fn a_part_with_no_picture_hands_back_none_rather_than_failing_to_be_read() {
    let files = InMemoryFiles::default();
    let path = Path::new("/parts/piece.caopart");
    drawn_part()
        .save(&files, path, at("2026-01-02T10:00:00Z"))
        .expect("the part is written");

    assert_eq!(PartDocument::picture_in(&files, path), None);
    assert_eq!(
        PartDocument::picture_in(&files, Path::new("/parts/nothing.caopart")),
        None,
        "a row the panel is still showing for a part that has just gone",
    );
}

#[test]
fn a_picture_survives_a_save_that_did_not_take_it() {
    let files = InMemoryFiles::default();
    let path = Path::new("/parts/piece.caopart");
    let mut document = drawn_part();
    document.set_picture(a_drawn_picture());
    document
        .save(&files, path, at("2026-01-02T10:00:00Z"))
        .expect("the part is written");

    let reopened = PartDocument::load(&files, path).expect("reads");
    reopened
        .save(&files, path, at("2026-01-02T11:00:00Z"))
        .expect("the part is written again");

    assert_eq!(
        PartDocument::load(&files, path).expect("reads").picture(),
        Some(&a_drawn_picture()),
        "save rebuilds the whole archive, and autosave calls it at the end of \
         every gesture: a picture it did not write is a picture gone",
    );
}

fn entries_of(files: &InMemoryFiles, path: &Path) -> Vec<String> {
    let bytes = files.read(path).expect("the archive is there");
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes)).expect("a zip archive");
    (0..archive.len())
        .map(|index| {
            archive
                .by_index(index)
                .expect("an entry of the archive")
                .name()
                .to_string()
        })
        .collect()
}

#[test]
fn every_major_step_of_a_design_is_written_in_a_folder_of_its_own() {
    let files = InMemoryFiles::default();
    let path = Path::new("/parts/piece.caopart");
    let mut document = drawn_part();
    document.apply(Operation::Extrude {
        sketch: 0,
        picks: Vec::new(),
        distance: 4.0,
        mode: crate::history::ExtrusionMode::Add,
    });
    document.set_picture(a_drawn_picture());
    document
        .put_away(&files, path, at("2026-01-02T10:00:00Z"))
        .expect("the part is written");

    assert_eq!(
        entries_of(&files, path),
        [
            "part.json",
            "design/history.json",
            "design/sketch-1/steps.json",
            "design/extrusion-2/steps.json",
            "geometry.json",
            "picture.json",
            "picture.rgba",
        ],
        "the index names the steps in order and each of them holds its own \
         operations, while what describes the part as a whole — its rebuilt \
         geometry, its picture — stays beside them at the root",
    );
}

#[test]
fn a_part_whose_index_names_a_step_no_folder_holds_does_not_open() {
    let files = InMemoryFiles::default();
    let path = Path::new("/parts/piece.caopart");
    drawn_part()
        .save(&files, path, at("2026-01-02T10:00:00Z"))
        .expect("the part is written");
    without(&files, path, "design/sketch-1/steps.json");

    let error = PartDocument::load(&files, path).expect_err("a step with no folder");

    assert!(
        matches!(&error, PartFileError::MissingEntry(entry) if entry.contains("sketch-1")),
        "the reader is told which step of the design is missing: {error:?}",
    );
}

/// Writes the archive again with one entry left out.
fn without(files: &InMemoryFiles, path: &Path, name: &str) {
    let bytes = files.read(path).expect("the archive is there");
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes)).expect("a zip archive");
    let mut written = zip::ZipWriter::new(Cursor::new(Vec::new()));
    let options: zip::write::SimpleFileOptions = zip::write::SimpleFileOptions::default();
    for index in 0..archive.len() {
        let mut entry = archive.by_index(index).expect("an entry");
        let entry_name = entry.name().to_string();
        let mut held = Vec::new();
        entry.read_to_end(&mut held).expect("the entry is read");
        if entry_name != name {
            written.start_file(entry_name, options).expect("an entry");
            written.write_all(&held).expect("the entry is written");
        }
    }
    let bytes = written.finish().expect("the archive").into_inner();
    files.write(path, &bytes).expect("the archive is written");
}
