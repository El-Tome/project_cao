use std::io::{Cursor, Read, Write};
use std::path::Path;

use cao_sketch::WorkPlane;
use chrono::{DateTime, Utc};
use glam::DVec2;

use super::*;
use crate::adapters::InMemoryFiles;
use crate::document::PartDocument;
use crate::document::design;
use crate::history::{ExtrusionMode, Operation, PointRef};
use crate::ports::Files;

fn borrowed(design: &[String]) -> Vec<&str> {
    design.iter().map(String::as_str).collect()
}

fn at(text: &str) -> DateTime<Utc> {
    text.parse().expect("a date")
}

fn a_part_with_matter() -> PartDocument {
    let mut document = PartDocument::new("Test", at("2026-01-02T09:00:00Z"));
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
    document.apply(Operation::Extrude {
        sketch: 0,
        areas: document.areas_at(0, &[DVec2::new(5.0, 10.0)]),
        distance: 4.0.into(),
        mode: ExtrusionMode::Add,
    });
    document
}

fn put_away(files: &InMemoryFiles, path: &Path) {
    a_part_with_matter()
        .put_away(files, path, at("2026-01-02T10:00:00Z"))
        .expect("the part is written");
}

/// Every file the design is written as, in the order the print folds them —
/// the index, then one per step.
fn design_of(files: &InMemoryFiles, path: &Path) -> Vec<String> {
    let bytes = files.read(path).expect("the archive is there");
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes)).expect("a zip archive");
    design::read(&mut archive).expect("a design").1
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
fn a_part_opened_on_its_cache_still_knows_the_faces_each_step_made() {
    let files = InMemoryFiles::default();
    let path = Path::new("/parts/piece.caopart");
    put_away(&files, path);

    let reopened = PartDocument::load(&files, path).expect("reads");

    let raised = reopened.faces_made_by(3);
    assert_eq!(raised, a_part_with_matter().faces_made_by(3));
    assert_eq!(raised.len(), 6, "the block's six faces: {raised:?}");
}

#[test]
fn a_part_cached_before_its_faces_were_noted_names_none_rather_than_another_steps() {
    let files = InMemoryFiles::default();
    let path = Path::new("/parts/piece.caopart");
    put_away(&files, path);
    let design = design_of(&files, path);
    let mut noted_nothing = a_part_with_matter().state;
    noted_nothing.made.clear();
    let cached = encoded(&noted_nothing, &borrowed(&design)).expect("a cache");
    replacing(&files, path, GEOMETRY_ENTRY, Some(&cached));
    let mut reopened = PartDocument::load(&files, path).expect("reads");
    reopened.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    reopened.apply(Operation::AddRectangle {
        sketch: 1,
        corner: PointRef::New(DVec2::new(30.0, 0.0)),
        opposite: PointRef::New(DVec2::new(40.0, 10.0)),
        construction: false,
    });
    reopened.apply(Operation::Extrude {
        sketch: 1,
        areas: reopened.areas_at(1, &[DVec2::new(35.0, 5.0)]),
        distance: 4.0.into(),
        mode: ExtrusionMode::Add,
    });

    assert_eq!(
        reopened.faces_made_by(3),
        Vec::<usize>::new(),
        "the first block's faces were never noted, and the second block's \
         are not them"
    );
}

#[test]
fn a_part_opens_on_the_geometry_it_was_put_away_with_rather_than_replaying_its_design() {
    let files = InMemoryFiles::default();
    let path = Path::new("/parts/piece.caopart");
    put_away(&files, path);
    let design = design_of(&files, path);
    let cached = encoded(&a_foreign_geometry(), &borrowed(&design)).expect("a cache");
    replacing(&files, path, GEOMETRY_ENTRY, Some(&cached));

    let reopened = PartDocument::load(&files, path).expect("reads");

    assert!(
        reopened.sketches().is_empty(),
        "the design draws a sketch, so a part still replaying it would hand \
         one back",
    );
}

#[test]
fn a_geometry_cached_before_a_stroke_was_changed_in_its_step_is_left_behind() {
    let files = InMemoryFiles::default();
    let path = Path::new("/parts/piece.caopart");
    put_away(&files, path);
    let mut design = design_of(&files, path);
    let drawing = &mut design[1];
    assert!(
        drawing.contains("20.0"),
        "the rectangle is in there: {drawing}"
    );
    *drawing = drawing.replace("20.0", "30.0");
    let cached = encoded(&a_foreign_geometry(), &borrowed(&design)).expect("a cache");
    replacing(&files, path, GEOMETRY_ENTRY, Some(&cached));

    let reopened = PartDocument::load(&files, path).expect("reads");

    assert_eq!(
        reopened.sketches().len(),
        1,
        "the print covers what every step holds and not the index alone, \
         where none of the drawing is",
    );
}

#[test]
fn a_geometry_cached_from_another_design_is_left_behind() {
    let files = InMemoryFiles::default();
    let path = Path::new("/parts/piece.caopart");
    put_away(&files, path);
    let cached = encoded(&a_foreign_geometry(), &["a design nobody wrote"]).expect("a cache");
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

/// A part's geometry as its cache carried it before #526 said which kernel
/// computed the matter: no drawing, and a tetrahedron of four faces numbered
/// from nought, raised by one step.
const CACHED_BEFORE_THE_EXACT_KERNEL: &str = concat!(
    r#"{"millimeters_per_unit":null,"sketches":[],"adrift":[],"body":{"polygons":["#,
    r#"{"corners":[[0.0,0.0,0.0],[0.0,1.0,0.0],[1.0,0.0,0.0]],"face":0},"#,
    r#"{"corners":[[0.0,0.0,0.0],[1.0,0.0,0.0],[0.0,0.0,1.0]],"face":1},"#,
    r#"{"corners":[[0.0,0.0,0.0],[0.0,0.0,1.0],[0.0,1.0,0.0]],"face":2},"#,
    r#"{"corners":[[1.0,0.0,0.0],[0.0,1.0,0.0],[0.0,0.0,1.0]],"face":3}"#,
    r#"]},"made":[{"start":0,"end":4}]}"#,
);

#[test]
fn a_cache_rebuilt_before_the_exact_kernel_replays_its_design() {
    let files = InMemoryFiles::default();
    let path = Path::new("/parts/piece.caopart");
    put_away(&files, path);
    let design = design_of(&files, path);
    let cached = format!(
        r#"{{"rebuilt_by":3,"design":{},"state":{CACHED_BEFORE_THE_EXACT_KERNEL}}}"#,
        fingerprint(&borrowed(&design)),
    );
    replacing(&files, path, GEOMETRY_ENTRY, Some(cached.as_bytes()));

    let reopened = PartDocument::load(&files, path).expect("reads");

    assert_eq!(
        reopened.sketches().len(),
        1,
        "the design is replayed rather than the old cache read",
    );
}

#[test]
fn a_cache_stamped_by_the_flats_rebuild_is_replayed_though_written_as_today() {
    let files = InMemoryFiles::default();
    let path = Path::new("/parts/piece.caopart");
    put_away(&files, path);
    let design = design_of(&files, path);
    let mut marked = a_part_with_matter().state;
    marked.declined.insert(3, cao_solid::Declined::Unsupported);
    let today = String::from_utf8(encoded(&marked, &borrowed(&design)).expect("a cache"))
        .expect("a cache is text");
    let stamped = today.replacen(
        &format!(r#""rebuilt_by":{REBUILT_BY}"#),
        r#""rebuilt_by":3"#,
        1,
    );
    assert_ne!(
        stamped, today,
        "the exact kernel rebuilds past the flats' 3"
    );
    replacing(&files, path, GEOMETRY_ENTRY, Some(stamped.as_bytes()));

    let reopened = PartDocument::load(&files, path).expect("reads");

    assert!(
        !reopened.is_declined(3),
        "the design is replayed rather than the cache read",
    );
}

/// Every corner of the matter, in the order the faces give them.
fn corners(body: &cao_solid::Body) -> Vec<glam::DVec3> {
    body.triangles().iter().flatten().copied().collect()
}

fn a_part_of_every_kind_of_drawing() -> PartDocument {
    use cao_sketch::{
        Chamfer, ChosenAxis, Constraint, Corner, DimensionTarget, Element, PointId, SegmentId,
        SketchAxis,
    };

    let mut document = PartDocument::new("Test", at("2026-01-02T09:00:00Z"));
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
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
        value: 50.0.into(),
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
        corners: vec![Corner::Between(SegmentId(0), SegmentId(1))],
        mode: Chamfer::Equal(2.0).into(),
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
        degrees: 45.0.into(),
        count: 3.0.into(),
    });
    document.apply(Operation::EraseMany {
        sketch: 0,
        elements: vec![Element::Segment(SegmentId(5))],
        dimensions: vec![],
        constraints: vec![],
    });
    document.apply(Operation::Extrude {
        sketch: 0,
        areas: document.areas_at(0, &[DVec2::new(20.0, 5.0)]),
        distance: 6.0.into(),
        mode: ExtrusionMode::Add,
    });
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 1,
        corner: PointRef::New(DVec2::new(2.0, 2.0)),
        opposite: PointRef::New(DVec2::new(6.0, 6.0)),
        construction: false,
    });
    document.apply(Operation::Extrude {
        sketch: 1,
        areas: document.areas_at(1, &[DVec2::new(4.0, 4.0)]),
        distance: 10.0.into(),
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
    let design = design_of(&files, path);
    let cached = encoded(&a_foreign_geometry(), &borrowed(&design)).expect("a cache");
    replacing(&files, path, GEOMETRY_ENTRY, Some(&cached));

    let reopened = PartDocument::load(&files, path).expect("reads");

    assert!(
        reopened.sketches().is_empty(),
        "the part opens on the geometry it was put away with",
    );
}

#[test]
fn a_part_opened_on_its_cached_geometry_still_knows_its_variables() {
    let files = InMemoryFiles::default();
    let path = Path::new("/parts/piece.caopart");
    let mut document = a_part_with_matter();
    document
        .change_variable(crate::variables::VariableChange::Added {
            name: "width".to_string(),
            formula: crate::formula::Formula::Number(120.0),
        })
        .expect("a variable");
    document
        .put_away(&files, path, at("2026-01-02T10:00:00Z"))
        .expect("the part is put away");

    let reopened = PartDocument::load(&files, path).expect("reads");

    assert_eq!(
        reopened.variables().named("width"),
        Some(crate::variables::VariableId(0)),
        "the cache holds none of the table, which comes back from the design",
    );
}

/// A block 30 by 20 by 10 with a bore through it off every round number, so
/// that no coordinate of the matter is one a short decimal writes exactly.
fn a_bored_block() -> PartDocument {
    let mut document = PartDocument::new("Bored", at("2026-10-05T09:00:00Z"));
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::ZERO),
        opposite: PointRef::New(DVec2::new(30.0, 20.0)),
        construction: false,
    });
    document.apply(Operation::Extrude {
        sketch: 0,
        areas: document.areas_at(0, &[DVec2::new(1.0, 1.0)]),
        distance: 10.0.into(),
        mode: ExtrusionMode::Add,
    });
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    let center = DVec2::new(10.0 / 3.0 + 7.0, 7.1);
    document.apply(Operation::AddCircle {
        sketch: 1,
        center: PointRef::New(center),
        radius: 2.7,
        rim: Vec::new(),
        construction: false,
    });
    document.apply(Operation::Extrude {
        sketch: 1,
        areas: document.areas_at(1, &[center]),
        distance: 10.0.into(),
        mode: ExtrusionMode::Cut,
    });
    document
}

/// Another bore, cut into whatever the part holds.
fn bored_again(document: &mut PartDocument) {
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    let sketch = document.sketches().len() - 1;
    let center = DVec2::new(22.2, 12.9);
    document.apply(Operation::AddCircle {
        sketch,
        center: PointRef::New(center),
        radius: 3.1,
        rim: Vec::new(),
        construction: false,
    });
    document.apply(Operation::Extrude {
        sketch,
        areas: document.areas_at(sketch, &[center]),
        distance: 10.0.into(),
        mode: ExtrusionMode::Cut,
    });
}

#[test]
fn an_exact_body_comes_back_from_the_cache_as_the_replay_leaves_it() {
    let files = InMemoryFiles::default();
    let path = Path::new("/parts/piece.caopart");
    let bored = a_bored_block();
    bored
        .put_away(&files, path, at("2026-10-05T10:00:00Z"))
        .expect("the part is put away");
    let mut marked = bored.state.clone();
    marked.declined.insert(99, cao_solid::Declined::Unsupported);
    let design = design_of(&files, path);
    let cache = encoded(&marked, &borrowed(&design)).expect("a cache");
    replacing(&files, path, GEOMETRY_ENTRY, Some(&cache));

    let mut cached = PartDocument::load(&files, path).expect("reads");
    let mut replayed = cached.clone();
    replayed.rewind_to(replayed.history.applied());

    assert!(
        cached.is_declined(99) && !replayed.is_declined(99),
        "the part opened on its cache, which alone knows the mark",
    );

    let bore = std::f64::consts::PI * 2.7 * 2.7 * 10.0;
    assert!(
        (cached.body().volume() - (6000.0 - bore)).abs() < 1e-9 * 6000.0,
        "the exact kernel's matter comes back: {}",
        cached.body().volume(),
    );
    assert_eq!(
        cached.body(),
        replayed.body(),
        "the very body the replay makes"
    );
    bored_again(&mut cached);
    bored_again(&mut replayed);
    assert_eq!(
        cached.body(),
        replayed.body(),
        "a step cut into the body read back gives what a replay gives",
    );
}

#[test]
fn a_declined_step_is_still_named_once_the_part_opens_on_its_cache() {
    let files = InMemoryFiles::default();
    let path = Path::new("/parts/piece.caopart");
    put_away(&files, path);
    let design = design_of(&files, path);
    let mut declined = a_part_with_matter().state;
    declined
        .declined
        .insert(3, cao_solid::Declined::Unsupported);
    let cached = encoded(&declined, &borrowed(&design)).expect("a cache");
    replacing(&files, path, GEOMETRY_ENTRY, Some(&cached));

    let reopened = PartDocument::load(&files, path).expect("reads");

    assert!(
        reopened.is_declined(3),
        "the decline is only known by computing the matter, which a part \
         opened on its cache does not do",
    );
}

#[test]
fn a_declined_step_keeps_its_reason_once_the_part_opens_on_its_cache() {
    let files = InMemoryFiles::default();
    let path = Path::new("/parts/piece.caopart");
    put_away(&files, path);
    let design = design_of(&files, path);
    let mut declined = a_part_with_matter().state;
    declined
        .declined
        .insert(3, cao_solid::Declined::Unsupported);
    let cached = encoded(&declined, &borrowed(&design)).expect("a cache");
    replacing(&files, path, GEOMETRY_ENTRY, Some(&cached));

    let reopened = PartDocument::load(&files, path).expect("reads");

    assert_eq!(
        reopened.declined_because(3),
        Some(cao_solid::Declined::Unsupported),
        "the reason is known only by computing the matter, which a part \
         opened on its cache does not do",
    );
    assert_eq!(reopened.declined_because(0), None, "and only for that step");
}

#[test]
fn a_cache_written_before_turns_were_exact_is_replayed() {
    let files = InMemoryFiles::default();
    let path = Path::new("/parts/piece.caopart");
    put_away(&files, path);
    let design = design_of(&files, path);
    let mut marked = a_part_with_matter().state;
    marked.declined.insert(3, cao_solid::Declined::Unsupported);
    let today = String::from_utf8(encoded(&marked, &borrowed(&design)).expect("a cache"))
        .expect("a cache is text");
    let stamped = today.replacen(
        &format!(r#""rebuilt_by":{REBUILT_BY}"#),
        r#""rebuilt_by":4"#,
        1,
    );
    assert_ne!(
        stamped, today,
        "turning straight profiles exactly rebuilds past the 4 of #526"
    );
    replacing(&files, path, GEOMETRY_ENTRY, Some(stamped.as_bytes()));

    let reopened = PartDocument::load(&files, path).expect("reads");

    assert!(
        !reopened.is_declined(3),
        "the design is replayed rather than the cache read",
    );
}
