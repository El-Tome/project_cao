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

fn put_away(files: &InMemoryFiles, path: &Path) {
    a_part_with_matter()
        .put_away(files, path, at("2026-01-02T10:00:00Z"))
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
    put_away(&files, path);
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
    put_away(&files, path);
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
    put_away(&files, path);
    replacing(&files, path, GEOMETRY_ENTRY, Some(b"half a file"));

    let reopened = PartDocument::load(&files, path).expect("reads");

    assert_eq!(reopened.sketches().len(), 1, "the design is replayed");
}

#[test]
fn a_part_written_before_the_cache_existed_opens_by_replaying_its_design() {
    let files = InMemoryFiles::default();
    let path = Path::new("/parts/piece.caopart");
    put_away(&files, path);
    replacing(&files, path, GEOMETRY_ENTRY, None);

    let reopened = PartDocument::load(&files, path).expect("reads");

    assert_eq!(reopened.sketches().len(), 1, "the design is replayed");
}

/// Every corner of the matter, in the order the faces give them.
fn corners(mesh: &cao_solid::Mesh) -> Vec<glam::DVec3> {
    mesh.triangles().iter().flatten().copied().collect()
}

fn a_part_of_every_kind_of_drawing() -> PartDocument {
    use cao_sketch::{
        Chamfer, ChosenAxis, Constraint, DimensionTarget, Element, PointId, SegmentId, SketchAxis,
    };

    let mut document = PartDocument::new("Test", at("2026-01-02T09:00:00Z"));
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
    });
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::ZERO),
        opposite: PointRef::New(DVec2::new(40.0, 20.0)),
        construction: false,
    });
    document.apply(Operation::AddCircle {
        sketch: 0,
        center: PointRef::New(DVec2::new(10.0, 10.0)),
        radius: 3.0,
        rim: vec![PointRef::New(DVec2::new(13.0, 10.0))],
        construction: false,
    });
    document.apply(Operation::AddArc {
        sketch: 0,
        center: PointRef::New(DVec2::new(30.0, 10.0)),
        start: PointRef::New(DVec2::new(34.0, 10.0)),
        end: PointRef::New(DVec2::new(30.0, 14.0)),
        construction: false,
    });
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::new(0.0, 25.0)),
        end: PointRef::New(DVec2::new(20.0, 25.0)),
        construction: true,
    });
    document.apply(Operation::AddSymmetricSegment {
        sketch: 0,
        middle: PointRef::New(DVec2::new(0.0, 30.0)),
        end: PointRef::New(DVec2::new(10.0, 30.0)),
        construction: false,
    });
    document.apply(Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Length(SegmentId(2)),
        value: 50.0,
        placement: Some(DVec2::new(20.0, -5.0)),
    });
    document.apply(Operation::Constrain {
        sketch: 0,
        constraint: Constraint::Equal {
            first: SegmentId(4),
            second: SegmentId(5),
        },
    });
    document.apply(Operation::Chamfer {
        sketch: 0,
        first: SegmentId(0),
        second: SegmentId(1),
        mode: Chamfer::Equal(2.0),
    });
    document.apply(Operation::Mirror {
        sketch: 0,
        elements: vec![Element::Arc(cao_sketch::ArcId(0))],
        axis: ChosenAxis::Sketch(SketchAxis::V),
    });
    document.apply(Operation::CircularPattern {
        sketch: 0,
        elements: vec![Element::Circle(cao_sketch::CircleId(0))],
        centre: PointId(0),
        degrees: 45.0,
        count: 3,
    });
    document.apply(Operation::EraseMany {
        sketch: 0,
        elements: vec![Element::Segment(SegmentId(5))],
        dimensions: vec![],
        constraints: vec![],
    });
    document.apply(Operation::Extrude {
        sketch: 0,
        picks: vec![DVec2::new(20.0, 5.0)],
        distance: 6.0,
        mode: ExtrusionMode::Add,
    });
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
    });
    document.apply(Operation::AddRectangle {
        sketch: 1,
        corner: PointRef::New(DVec2::new(2.0, 2.0)),
        opposite: PointRef::New(DVec2::new(6.0, 6.0)),
        construction: false,
    });
    document.apply(Operation::Extrude {
        sketch: 1,
        picks: vec![DVec2::new(4.0, 4.0)],
        distance: 10.0,
        mode: ExtrusionMode::Cut,
    });
    document
}

/// Everything a sketch carries, to the nearest nanometre.
///
/// The solver settles the same drawing a hair differently depending on whether
/// it was walked step by step or replayed in one go — a couple of last bits of
/// an `f64`, far under anything a part is drawn to.
fn rounded(sketch: &cao_sketch::Sketch) -> serde_json::Value {
    fn round(value: &mut serde_json::Value) {
        match value {
            serde_json::Value::Number(number) => {
                if let Some(held) = number.as_f64() {
                    *value = serde_json::json!((held * 1e9).round() / 1e9);
                }
            }
            serde_json::Value::Array(items) => items.iter_mut().for_each(round),
            serde_json::Value::Object(fields) => fields.values_mut().for_each(round),
            _ => {}
        }
    }
    let mut drawn = serde_json::to_value(sketch).expect("a sketch");
    round(&mut drawn);
    drawn
}

#[test]
fn a_drawing_of_every_kind_comes_back_from_the_cache_as_the_replay_leaves_it() {
    const TOLERANCE: f64 = 1e-9;
    let files = InMemoryFiles::default();
    let path = Path::new("/parts/piece.caopart");
    let document = a_part_of_every_kind_of_drawing();
    let drawn = &document.sketches()[0];
    assert!(
        !drawn.circles().is_empty()
            && !drawn.arcs().is_empty()
            && !drawn.dimensions().is_empty()
            && !drawn.constraints().is_empty(),
        "the part is meant to hold one of everything a sketch can carry",
    );
    document
        .save(&files, path, at("2026-01-02T10:00:00Z"))
        .expect("the part is written");

    let cached = PartDocument::load(&files, path).expect("reads");
    let mut replayed = cached.clone();
    replayed.rewind_to(replayed.history.applied());

    for (index, (read, rebuilt)) in cached
        .sketches()
        .iter()
        .zip(replayed.sketches())
        .enumerate()
    {
        assert_eq!(
            rounded(read),
            rounded(rebuilt),
            "sketch {index} came back from the cache other than the replay \
             leaves it",
        );
    }
    let (read, rebuilt) = (corners(cached.body()), corners(replayed.body()));
    assert_eq!(read.len(), rebuilt.len(), "the same matter, face for face");
    for (read, rebuilt) in read.iter().zip(&rebuilt) {
        assert!(
            read.distance(*rebuilt) < TOLERANCE,
            "a corner read back at {read} where the replay puts it at {rebuilt}",
        );
    }
}

#[test]
fn a_gesture_writes_the_design_alone_and_leaves_the_geometry_for_the_way_out() {
    let files = InMemoryFiles::default();
    let path = Path::new("/parts/piece.caopart");
    a_part_with_matter()
        .save(&files, path, at("2026-01-02T10:00:00Z"))
        .expect("the part is written");

    let mut archive =
        zip::ZipArchive::new(Cursor::new(files.read(path).expect("the archive"))).expect("a zip");

    assert!(
        archive.by_name(GEOMETRY_ENTRY).is_err(),
        "the autosave writes at the end of every gesture, and geometry written \
         there is geometry written again at the next one",
    );
}

#[test]
fn a_part_put_away_carries_the_geometry_it_was_showing() {
    let files = InMemoryFiles::default();
    let path = Path::new("/parts/piece.caopart");
    let document = a_part_with_matter();
    document
        .put_away(&files, path, at("2026-01-02T10:00:00Z"))
        .expect("the part is written");
    let design = String::from_utf8(entry_of(&files, path, HISTORY_ENTRY)).expect("the design");
    let cached = encoded(&a_foreign_geometry(), &design).expect("a cache");
    replacing(&files, path, GEOMETRY_ENTRY, Some(&cached));

    let reopened = PartDocument::load(&files, path).expect("reads");

    assert!(
        reopened.sketches().is_empty(),
        "the part opens on the geometry it was put away with",
    );
}
