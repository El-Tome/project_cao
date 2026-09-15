use std::io::{Cursor, Read, Write};
use std::path::Path;

use cao_sketch::WorkPlane;
use chrono::{DateTime, Utc};
use glam::DVec2;

use super::*;
use crate::adapters::InMemoryFiles;
use crate::document::{HISTORY_ENTRY, PartDocument};
use crate::history::{ExtrusionMode, Operation, PointRef};
use crate::ports::Files;

fn at(text: &str) -> DateTime<Utc> {
    text.parse().expect("a date")
}

fn a_part_with_matter() -> PartDocument {
    let mut document = PartDocument::new("Test", at("2026-01-02T09:00:00Z"));
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
    });
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::ZERO),
        opposite: PointRef::New(DVec2::new(10.0, 20.0)),
        construction: false,
    });
    document.apply(Operation::Extrude {
        sketch: 0,
        picks: vec![DVec2::new(5.0, 10.0)],
        distance: 4.0,
        mode: ExtrusionMode::Add,
    });
    document
}

fn saved(files: &InMemoryFiles, path: &Path) {
    a_part_with_matter()
        .save(files, path, at("2026-01-02T10:00:00Z"))
        .expect("the part is written");
}

fn entry_of(files: &InMemoryFiles, path: &Path, name: &str) -> Vec<u8> {
    let bytes = files.read(path).expect("the archive is there");
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes)).expect("a zip archive");
    let mut entry = Vec::new();
    archive
        .by_name(name)
        .expect("the entry is there")
        .read_to_end(&mut entry)
        .expect("the entry is read");
    entry
}

/// Writes the archive again with one entry replaced, or dropped when there is
/// nothing to put in its place.
fn replacing(files: &InMemoryFiles, path: &Path, name: &str, content: Option<&[u8]>) {
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
    if let Some(content) = content {
        written.start_file(name, options).expect("an entry");
        written.write_all(content).expect("the entry is written");
    }
    let bytes = written
        .finish()
        .expect("the archive is closed")
        .into_inner();
    files.write(path, &bytes).expect("the archive is written");
}

/// A geometry no replay of that part could ever give.
fn a_foreign_geometry() -> PartState {
    PartState {
        millimeters_per_unit: Some(7.0),
        ..PartState::default()
    }
}

#[test]
fn a_part_opens_on_the_geometry_it_was_put_away_with_rather_than_replaying_its_design() {
    let files = InMemoryFiles::default();
    let path = Path::new("/parts/piece.caopart");
    saved(&files, path);
    let design = String::from_utf8(entry_of(&files, path, HISTORY_ENTRY)).expect("the design");
    let cached = encoded(&a_foreign_geometry(), &design).expect("a cache");
    replacing(&files, path, GEOMETRY_ENTRY, Some(&cached));

    let reopened = PartDocument::load(&files, path).expect("reads");

    assert!(
        reopened.sketches().is_empty(),
        "the design draws a sketch, so a part still replaying it would hand \
         one back",
    );
}

#[test]
fn a_geometry_cached_from_another_design_is_left_behind() {
    let files = InMemoryFiles::default();
    let path = Path::new("/parts/piece.caopart");
    saved(&files, path);
    let cached = encoded(&a_foreign_geometry(), "a design nobody wrote").expect("a cache");
    replacing(&files, path, GEOMETRY_ENTRY, Some(&cached));

    let reopened = PartDocument::load(&files, path).expect("reads");

    assert_eq!(
        reopened.sketches().len(),
        1,
        "geometry that answers to another design says nothing about this one, \
         so the design is replayed",
    );
}

#[test]
fn a_damaged_cache_is_a_part_that_replays_its_design_rather_than_one_that_will_not_open() {
    let files = InMemoryFiles::default();
    let path = Path::new("/parts/piece.caopart");
    saved(&files, path);
    replacing(&files, path, GEOMETRY_ENTRY, Some(b"half a file"));

    let reopened = PartDocument::load(&files, path).expect("reads");

    assert_eq!(reopened.sketches().len(), 1, "the design is replayed");
}

#[test]
fn a_part_written_before_the_cache_existed_opens_by_replaying_its_design() {
    let files = InMemoryFiles::default();
    let path = Path::new("/parts/piece.caopart");
    saved(&files, path);
    replacing(&files, path, GEOMETRY_ENTRY, None);

    let reopened = PartDocument::load(&files, path).expect("reads");

    assert_eq!(reopened.sketches().len(), 1, "the design is replayed");
}

/// Every corner of the matter, in the order the faces give them.
fn corners(mesh: &cao_solid::Mesh) -> Vec<glam::DVec3> {
    mesh.triangles().iter().flatten().copied().collect()
}

#[test]
fn the_geometry_read_back_is_the_one_replaying_the_design_would_give() {
    const TOLERANCE: f64 = 1e-12;
    let files = InMemoryFiles::default();
    let path = Path::new("/parts/piece.caopart");
    saved(&files, path);

    let cached = PartDocument::load(&files, path).expect("reads");
    let mut replayed = cached.clone();
    replayed.rewind_to(replayed.history.applied());

    let (read, rebuilt) = (corners(cached.body()), corners(replayed.body()));
    assert_eq!(read.len(), rebuilt.len(), "the same matter, face for face");
    for (read, rebuilt) in read.iter().zip(&rebuilt) {
        assert!(
            read.distance(*rebuilt) < TOLERANCE,
            "a corner read back at {read} where the replay puts it at {rebuilt}",
        );
    }
    let (read, rebuilt) = (
        cached.sketches()[0].points().to_vec(),
        replayed.sketches()[0].points().to_vec(),
    );
    assert_eq!(
        read.len(),
        rebuilt.len(),
        "the same drawing, point for point"
    );
    for (read, rebuilt) in read.iter().zip(&rebuilt) {
        assert!(
            read.distance(*rebuilt) < TOLERANCE,
            "a point read back at {read} where the replay puts it at {rebuilt}",
        );
    }
}
