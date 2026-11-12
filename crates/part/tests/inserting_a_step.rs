//! A step made while the part is read through an earlier one goes in right
//! after it, and the steps that came after replay on top of it.
//!
//! The reading point is the step reader's, in the Part panel; the part is only
//! told where the step goes, as the key step it goes in after — counted from
//! nothing, and the way `PartDocument::shown_through` counts them, so "step 3
//! of 6" is key step 2 here. The operation that opens the step is recorded
//! last in the order things were done, like any other: only its place among
//! the steps of the part is not the end.
//!
//! Closes #737 — this file holds the part's half of each criterion. The
//! reader's half — where the reading point goes, the boxes dashed, the names
//! the panel shows — belongs to `cao_app`, and is transcribed under blocks of
//! its own in `crates/app/tests/the_step_reader.rs` and
//! `crates/app/tests/a_sketch_keeps_its_number.rs`.
//! - The reading point is at step 3 of 6; the user creates a sketch, an
//!   extrusion…: the new step is inserted after step 3 —
//!   `a_sketch_and_its_extrusion_made_while_read_through_step_three_go_in_after_it`
//! - The reading point stays on the new step, the rest still dashed. More steps
//!   can be inserted after it; ▶ (or a click on the last box) plays the rest —
//!   no test: here, the reading point and the dashed boxes are the reader's,
//!   held in `crates/app/tests/the_step_reader.rs` by
//!   `the_reading_point_stays_on_each_step_inserted_and_the_rest_plays_after_them`.
//!   The part's half — a second step put in after the
//!   one just inserted, and the part read past it whole — is asserted by
//!   `a_sketch_and_its_extrusion_made_while_read_through_step_three_go_in_after_it`
//! - The rest replays on top of the new step. A step that loses what it stood
//!   on (a face, an area) gets an alert, as today when an old sketch is edited.
//!   Nothing is deleted on its own —
//!   `the_steps_after_an_insertion_replay_on_top_of_it`,
//!   `a_step_that_loses_its_face_to_an_insertion_is_flagged_and_kept`. An area
//!   cannot be lost that way: a step's areas are its own sketch's, which no
//!   step put in before it draws in
//! - decided with the insertion: a later sketch whose face is still there stays
//!   on that face, the faces made after the insertion numbered further on —
//!   `a_sketch_on_a_face_made_after_an_insertion_stays_on_that_face`
//! - Names: a sketch inserted between « Esquisse 2 » and « Esquisse 3 » takes
//!   the next number, « Esquisse 4 ». The others are not renumbered — the
//!   number, which the part keeps:
//!   `a_sketch_inserted_between_the_second_and_the_third_takes_the_next_number`;
//!   the name every panel writes out of it is in
//!   `crates/app/tests/a_sketch_keeps_its_number.rs`
//! - Undo takes back the last action in time, even one inserted in the middle;
//!   the reading point does not move unless its box disappears. The history
//!   lists actions in the order they were done; the reader shows steps in the
//!   order of the part —
//!   `undo_takes_back_the_last_step_made_even_one_inserted_in_the_middle`,
//!   `the_list_keeps_the_order_things_were_done_and_the_steps_the_order_of_the_part`;
//!   the reading point and what each panel lists are the reader's half
//! - Numbering rule: "the next number" is the highest number ever used plus
//!   one; a name is never reused, even if the sketch that bore it was deleted —
//!   `a_sketch_number_is_never_given_twice_even_once_its_sketch_is_gone`
//! - decided with the numbering rule: the highest number ever given goes with
//!   the part, so that a number dropped before it was saved is not given again
//!   once it is reopened —
//!   `a_sketch_number_dropped_before_the_part_was_saved_is_not_given_again_once_reopened`
//! - decided with the numbering rule: a sketch taken back and redone is the
//!   same sketch, and comes back with its own number rather than the next —
//!   `a_sketch_taken_back_then_redone_gets_its_own_number_back`
//! - Must not break: a part with no insertion opens, replays and exports the
//!   same — no test: every other file of this folder builds its part with no
//!   insertion and replays it, and `replaying.rs` and `reading_a_part.rs` save,
//!   reopen and read it; the export writes the body those tests measure
//! - Must not break: every name an alert or a dimension uses to find its step
//!   still finds it after an insertion —
//!   `a_dimension_and_an_alert_on_a_later_sketch_still_find_it_after_an_insertion`
//! - Must not break: undo and redo across an insertion leave the part as it was
//!   before it — `undoing_and_redoing_an_insertion_gives_back_the_part_as_it_was`
//! - decided with the insertion: « Compacter l'historique » leaves a part with
//!   an insertion as it was — its steps in the order of the part, read through
//!   each the same, its sketches numbered as before —
//!   `compacting_a_part_with_an_insertion_keeps_the_order_of_its_part`
//! - decided with the insertion: a sketch compaction lays down again as it was
//!   drawn, a corner rounded by a variable, is still the sketch that corner
//!   was rounded on — `compacting_keeps_a_size_written_from_a_variable_on_its_own_sketch`

use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};

use cao_part::history::{FaceAnchor, StepKind};
use cao_part::{
    ExtrusionMode, FileError, Files, Operation, PartDocument, PointRef, VariableChange,
};
use cao_sketch::{Area, Corner, DimensionTarget, Element, SegmentId, WorkPlane};
use chrono::Utc;
use glam::{DVec2, DVec3};

/// How far from the arithmetic a volume of flat-sided matter may stand.
const VOLUME: f64 = 1e-6;
/// How far from where it should be a ray may meet a flat face.
const HEIGHT: f64 = 1e-6;

/// Step 3 of 6, as the part counts its key steps.
const THE_THIRD_STEP: usize = 2;

fn assert_volume(volume: f64, expected: f64, what: &str) {
    assert!(
        (volume - expected).abs() <= VOLUME * expected.max(1.0),
        "{what} holds {volume}, not {expected}",
    );
}

/// Three squares of side 10 drawn on XY, each raised as soon as it is drawn:
/// at x = 0 by 10, at x = 40 by 5, at x = 80 by 2. Six key steps — sketch,
/// extrusion, three times — and 1700 of matter.
fn a_part_of_six_steps() -> PartDocument {
    let mut document = PartDocument::new("part", Utc::now());
    for (sketch, from, depth) in [(0, 0.0, 10.0), (1, 40.0, 5.0), (2, 80.0, 2.0)] {
        document.apply(Operation::CreateSketch {
            plane: WorkPlane::XY,
            on: None,
        });
        document.apply(Operation::AddRectangle {
            sketch,
            corner: PointRef::New(DVec2::new(from, 0.0)),
            opposite: PointRef::New(DVec2::new(from + 10.0, 10.0)),
            construction: false,
        });
        document.apply(Operation::Extrude {
            sketch,
            areas: document.areas_at(sketch, &[DVec2::new(from + 5.0, 5.0)]),
            distance: depth.into(),
            mode: ExtrusionMode::Add,
        });
    }
    assert_volume(
        document.body().volume(),
        1700.0,
        "the fixture, three blocks of 1000, 500 and 200,",
    );
    document
}

/// How high the matter stands over a place of XY, by a ray coming down on it.
fn top_over(document: &PartDocument, place: DVec2) -> Option<f64> {
    let from = 100.0;
    document
        .body()
        .ray_hit(place.extend(from), DVec3::NEG_Z)
        .map(|hit| from - hit.distance)
}

fn assert_top(document: &PartDocument, place: DVec2, expected: f64, what: &str) {
    let top = top_over(document, place);
    assert!(
        top.is_some_and(|top| (top - expected).abs() < HEIGHT),
        "{what}: the matter over {place} stands at {top:?}, not {expected}",
    );
}

/// Read through step 3 of `a_part_of_six_steps`, its second sketch: a sketch
/// put in on XZ, a 10 by 20 rectangle drawn in it at x = 120, and the
/// extrusion of that rectangle by 3 put in after it — three actions, the last
/// three in time. Says which sketch it put in.
fn insert_a_block_after_the_second_sketch(document: &mut PartDocument) -> usize {
    document.insert_after(
        THE_THIRD_STEP,
        Operation::CreateSketch {
            plane: WorkPlane::XZ,
            on: None,
        },
    );
    let inserted = document
        .sketches()
        .iter()
        .position(|sketch| sketch.plane == WorkPlane::XZ)
        .expect("the sketch inserted is one of the part's");
    document.apply(Operation::AddRectangle {
        sketch: inserted,
        corner: PointRef::New(DVec2::new(120.0, 0.0)),
        opposite: PointRef::New(DVec2::new(130.0, 20.0)),
        construction: false,
    });
    document.insert_after(
        THE_THIRD_STEP + 1,
        Operation::Extrude {
            sketch: inserted,
            areas: document.areas_at(inserted, &[DVec2::new(125.0, 10.0)]),
            distance: 3.0.into(),
            mode: ExtrusionMode::Add,
        },
    );
    inserted
}

/// The kind of each key step in effect, in the order of the part: what the
/// part is made of once an undo has taken some back, which the steps of the
/// history, keeping what can be redone, still hold.
fn kinds_of_the_steps(document: &PartDocument) -> Vec<StepKind> {
    let history = &document.history;
    history
        .key_steps()
        .map(|(at, _)| history.steps()[at].kind())
        .collect()
}

#[test]
fn a_sketch_and_its_extrusion_made_while_read_through_step_three_go_in_after_it() {
    let mut document = a_part_of_six_steps();

    insert_a_block_after_the_second_sketch(&mut document);

    let kinds = kinds_of_the_steps(&document);
    assert_eq!(
        kinds,
        [
            StepKind::Sketch,
            StepKind::Extrusion,
            StepKind::Sketch,
            StepKind::Sketch,
            StepKind::Extrusion,
            StepKind::Extrusion,
            StepKind::Sketch,
            StepKind::Extrusion,
        ],
        "the sketch inserted and its extrusion stand fourth and fifth in the part, between \
         the second sketch and the extrusion raised from it",
    );

    let all_opaque = BTreeMap::new();
    for (key_step, expected, what) in [
        (
            2,
            1000.0,
            "read through the second sketch, the first block alone",
        ),
        (
            3,
            1000.0,
            "read through the sketch inserted, which raises nothing, still the first block alone",
        ),
        (
            4,
            1600.0,
            "read through the extrusion inserted, the first block and the 10 by 20 raised 3",
        ),
        (
            5,
            2100.0,
            "read through the second sketch's extrusion, now after the steps inserted, \
             the second block on top",
        ),
        (6, 2100.0, "read through the third sketch, nothing more"),
        (
            7,
            2300.0,
            "read through the last step, the whole part with the third block",
        ),
    ] {
        assert_volume(
            document.shown_through(&all_opaque, key_step).body.volume(),
            expected,
            what,
        );
    }

    assert_volume(
        document.body().volume(),
        2300.0,
        "the part, its three blocks and the one inserted,",
    );
    assert_top(
        &document,
        DVec2::new(45.0, 5.0),
        5.0,
        "the second sketch's extrusion, replayed after the steps inserted, still raises \
         the second sketch",
    );
    assert_top(
        &document,
        DVec2::new(85.0, 5.0),
        2.0,
        "the third sketch's extrusion, made on the sketch that came third, still raises \
         that sketch and not the one inserted before it",
    );
}

/// The number each sketch of the part bears, sketch by sketch in the order
/// the operations name them by.
fn numbers_of_the_sketches(document: &PartDocument) -> Vec<Option<u32>> {
    (0..document.sketches().len())
        .map(|sketch| document.sketch_number(sketch))
        .collect()
}

fn a_square_in(document: &mut PartDocument, sketch: usize, from: f64) {
    document.apply(Operation::AddRectangle {
        sketch,
        corner: PointRef::New(DVec2::new(from, 0.0)),
        opposite: PointRef::New(DVec2::new(from + 10.0, 10.0)),
        construction: false,
    });
}

/// Files that never leave the process: `cao_part`'s own are kept behind a
/// feature this crate's tests are not built with.
#[derive(Default)]
struct Memory(RefCell<HashMap<PathBuf, Vec<u8>>>);

impl Files for Memory {
    fn read(&self, path: &Path) -> Result<Vec<u8>, FileError> {
        self.0
            .borrow()
            .get(path)
            .cloned()
            .ok_or_else(|| FileError::Absent(path.to_path_buf()))
    }

    fn write(&self, path: &Path, bytes: &[u8]) -> Result<(), FileError> {
        self.0
            .borrow_mut()
            .insert(path.to_path_buf(), bytes.to_vec());
        Ok(())
    }

    fn exists(&self, path: &Path) -> bool {
        self.0.borrow().contains_key(path)
    }
}

fn planes_of_the_sketches(document: &PartDocument) -> Vec<WorkPlane> {
    document
        .sketches()
        .iter()
        .map(|sketch| sketch.plane)
        .collect()
}

/// Three sketches, on XY, XY and XZ, a square in each; then the third's
/// square and its opening taken back, which leaves them to redo.
fn three_sketches_the_last_taken_back() -> PartDocument {
    let mut document = PartDocument::new("part", Utc::now());
    for (sketch, plane) in [(0, WorkPlane::XY), (1, WorkPlane::XY), (2, WorkPlane::XZ)] {
        document.apply(Operation::CreateSketch { plane, on: None });
        a_square_in(&mut document, sketch, 40.0 * sketch as f64);
    }
    assert_eq!(
        numbers_of_the_sketches(&document),
        [Some(1), Some(2), Some(3)],
        "the fixture wants three sketches numbered one to three in the order they were made",
    );
    document.undo();
    document.undo();
    assert_eq!(
        planes_of_the_sketches(&document),
        [WorkPlane::XY, WorkPlane::XY],
        "the fixture wants the third sketch, its square and its opening, taken back",
    );
    document
}

fn open_a_sketch_on_yz(document: &mut PartDocument) {
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::YZ,
        on: None,
    });
}

#[test]
fn a_sketch_number_is_never_given_twice_even_once_its_sketch_is_gone() {
    let mut document = three_sketches_the_last_taken_back();

    open_a_sketch_on_yz(&mut document);
    assert_eq!(
        planes_of_the_sketches(&document),
        [WorkPlane::XY, WorkPlane::XY, WorkPlane::YZ],
        "the fixture wants the third sketch dropped by the sketch opened on YZ after it",
    );

    assert_eq!(
        numbers_of_the_sketches(&document),
        [Some(1), Some(2), Some(4)],
        "the sketch on YZ takes the highest number ever given plus one, 4, and not the 3 the \
         sketch it dropped bore; the two before it keep theirs",
    );
}

#[test]
fn a_sketch_number_dropped_before_the_part_was_saved_is_not_given_again_once_reopened() {
    let mut document = three_sketches_the_last_taken_back();
    a_square_in(&mut document, 1, 100.0);
    assert!(
        !document.history.can_redo(),
        "the fixture wants the third sketch dropped by a second square in the second sketch, \
         leaving nothing to redo and no sketch numbered 3 in the part",
    );

    let files = Memory::default();
    let (saved, put_away) = (
        Path::new("/parts/saved.caopart"),
        Path::new("/parts/put-away.caopart"),
    );
    document
        .save(&files, saved, Utc::now())
        .expect("the part is saved");
    document
        .put_away(&files, put_away, Utc::now())
        .expect("the part is put away");

    for path in [saved, put_away] {
        let mut reopened = PartDocument::load(&files, path).expect("the part opens again");
        open_a_sketch_on_yz(&mut reopened);

        assert_eq!(
            numbers_of_the_sketches(&reopened),
            [Some(1), Some(2), Some(4)],
            "reopened from {path:?}, the part gives the sketch opened on YZ the 3 a sketch \
             dropped before it was saved bore, or another number than the highest ever given \
             plus one",
        );
    }
}

#[test]
fn a_sketch_taken_back_then_redone_gets_its_own_number_back() {
    let mut document = three_sketches_the_last_taken_back();
    open_a_sketch_on_yz(&mut document);
    assert_eq!(
        numbers_of_the_sketches(&document),
        [Some(1), Some(2), Some(4)],
        "the fixture wants the sketch on YZ numbered 4, the third one's 3 dropped",
    );

    document.undo();
    assert_eq!(
        planes_of_the_sketches(&document),
        [WorkPlane::XY, WorkPlane::XY],
        "the fixture wants the sketch on YZ taken back",
    );
    document.redo();

    assert_eq!(
        planes_of_the_sketches(&document),
        [WorkPlane::XY, WorkPlane::XY, WorkPlane::YZ],
        "redo does not give the sketch on YZ back",
    );
    assert_eq!(
        numbers_of_the_sketches(&document),
        [Some(1), Some(2), Some(4)],
        "redone, the sketch on YZ comes back under another number than the 4 it bore",
    );
}

#[test]
fn a_sketch_inserted_between_the_second_and_the_third_takes_the_next_number() {
    let mut document = a_part_of_six_steps();

    let inserted = insert_a_block_after_the_second_sketch(&mut document);

    assert_eq!(
        (
            planes_of_the_sketches(&document),
            numbers_of_the_sketches(&document)
        ),
        (
            vec![WorkPlane::XY, WorkPlane::XY, WorkPlane::XY, WorkPlane::XZ],
            vec![Some(1), Some(2), Some(3), Some(4)],
        ),
        "the sketch put in on XZ between the second and the third, sketch {inserted}, does \
         not bear 4, the next number, or one of the three before it was renumbered",
    );
}

/// What one operation that opens a step does, as a test tells them apart:
/// the plane a sketch is laid on, the sketch an extrusion raises.
fn opening(operation: &Operation) -> String {
    match operation {
        Operation::CreateSketch { plane, .. } if *plane == WorkPlane::XZ => {
            "a sketch on XZ".to_string()
        }
        Operation::CreateSketch { .. } => "a sketch on XY".to_string(),
        Operation::Extrude { sketch, .. } => format!("the extrusion of sketch {sketch}"),
        other => format!("{other:?}"),
    }
}

/// The operations that open a step, in the order the list keeps.
fn openings_done(document: &PartDocument) -> Vec<String> {
    document
        .history
        .operations()
        .iter()
        .filter(|operation| StepKind::opened_by(operation).is_some())
        .map(opening)
        .collect()
}

/// The operation that opens each step, in the order of the part.
fn openings_of_the_part(document: &PartDocument) -> Vec<String> {
    let history = &document.history;
    history
        .steps()
        .iter()
        .filter_map(|step| step.operations().first())
        .filter_map(|number| history.position_of(*number))
        .filter_map(|at| history.operations().get(at))
        .map(opening)
        .collect()
}

#[test]
fn the_list_keeps_the_order_things_were_done_and_the_steps_the_order_of_the_part() {
    let mut document = a_part_of_six_steps();

    insert_a_block_after_the_second_sketch(&mut document);

    let (xy, xz) = ("a sketch on XY", "a sketch on XZ");
    let raising = |sketch: usize| format!("the extrusion of sketch {sketch}");
    assert_eq!(
        (openings_done(&document), openings_of_the_part(&document)),
        (
            vec![
                xy.to_string(),
                raising(0),
                xy.to_string(),
                raising(1),
                xy.to_string(),
                raising(2),
                xz.to_string(),
                raising(3),
            ],
            vec![
                xy.to_string(),
                raising(0),
                xy.to_string(),
                xz.to_string(),
                raising(3),
                raising(1),
                xy.to_string(),
                raising(2),
            ],
        ),
        "the list does not keep the steps put in last, in the order they were done, or the \
         steps do not hold them after the second sketch, in the order of the part, as (the \
         list, the steps)",
    );
}

#[test]
fn undo_takes_back_the_last_step_made_even_one_inserted_in_the_middle() {
    let mut document = a_part_of_six_steps();
    insert_a_block_after_the_second_sketch(&mut document);

    document.undo();

    assert_eq!(
        kinds_of_the_steps(&document),
        [
            StepKind::Sketch,
            StepKind::Extrusion,
            StepKind::Sketch,
            StepKind::Sketch,
            StepKind::Extrusion,
            StepKind::Sketch,
            StepKind::Extrusion,
        ],
        "undo did not take back the extrusion put in after the second sketch, the last \
         thing done, and that one alone",
    );
    assert_volume(
        document.body().volume(),
        1700.0,
        "undone once, the part — its three blocks, the 10 by 20 put in no longer raised —",
    );
    assert_top(
        &document,
        DVec2::new(85.0, 5.0),
        2.0,
        "undone once, the third block, the last step of the part, stays",
    );
}

/// What the sketch of index 2 is sized and asks of the part: its first side
/// typed 15, its third erased, so that the area its extrusion was raised from
/// is no longer closed — the alert « aire perdue » the tree reads off
/// `areas_standing`.
fn a_dimension_and_a_lost_area_on_the_third_sketch(document: &mut PartDocument) {
    document.apply(Operation::SetDimension {
        sketch: 2,
        target: DimensionTarget::Length(SegmentId(0)),
        value: 15.0.into(),
        placement: None,
    });
    document.apply(Operation::EraseMany {
        sketch: 2,
        elements: vec![Element::Segment(SegmentId(2))],
        dimensions: Vec::new(),
        constraints: Vec::new(),
    });
}

/// The areas the extrusion of that sketch was raised from, as it recorded
/// them.
fn areas_raised_from(document: &PartDocument, sketch: usize) -> Vec<Area> {
    document
        .history
        .operations()
        .iter()
        .find_map(|operation| match operation {
            Operation::Extrude {
                sketch: from,
                areas,
                ..
            } if *from == sketch => Some(areas.clone()),
            _ => None,
        })
        .expect("an extrusion of that sketch")
}

/// What the part says of the third sketch: the length of its first side, and
/// how many of the areas its extrusion was raised from still stand.
fn what_the_third_sketch_says(document: &PartDocument) -> (Option<f64>, usize) {
    let measured = document.measured(2, DimensionTarget::Length(SegmentId(0)));
    let standing = document
        .areas_standing(2, &areas_raised_from(document, 2))
        .len();
    (measured, standing)
}

#[test]
fn a_dimension_and_an_alert_on_a_later_sketch_still_find_it_after_an_insertion() {
    let mut document = a_part_of_six_steps();
    a_dimension_and_a_lost_area_on_the_third_sketch(&mut document);
    let before = what_the_third_sketch_says(&document);
    assert!(
        before
            .0
            .is_some_and(|length| (length - 15.0).abs() < HEIGHT)
            && before.1 == 0,
        "the fixture wants the third sketch's first side 15 long and its extrusion's area \
         lost, as (its length, areas standing): {before:?}",
    );
    assert_volume(
        document.body().volume(),
        1500.0,
        "the fixture, its third block raised from nothing,",
    );

    let inserted = insert_a_block_after_the_second_sketch(&mut document);

    let after = what_the_third_sketch_says(&document);
    assert!(
        after.0.is_some_and(|length| (length - 15.0).abs() < HEIGHT) && after.1 == 0,
        "once a sketch is put in before it, the value on the third sketch, or the alert of \
         its extrusion's lost area, finds another sketch, as (its length, areas standing): \
         {after:?}",
    );
    assert_eq!(
        document
            .areas_standing(inserted, &areas_raised_from(&document, inserted))
            .len(),
        1,
        "the sketch put in has its own area standing, and takes no alert of the third's",
    );
    assert_volume(
        document.body().volume(),
        2100.0,
        "the part with the 10 by 20 put in, the third block still raised from nothing,",
    );
}

/// The part as a test compares it before and after: the kinds of its steps
/// in the order of the part, the plane and the number of each sketch, and
/// what it holds read through each key step, then whole.
struct Seen {
    kinds: Vec<StepKind>,
    planes: Vec<WorkPlane>,
    numbers: Vec<Option<u32>>,
    volumes: Vec<f64>,
}

fn seen(document: &PartDocument) -> Seen {
    let all_opaque = BTreeMap::new();
    let kinds = kinds_of_the_steps(document);
    let mut volumes: Vec<f64> = (0..kinds.len())
        .map(|key_step| document.shown_through(&all_opaque, key_step).body.volume())
        .collect();
    volumes.push(document.body().volume());
    Seen {
        kinds,
        planes: planes_of_the_sketches(document),
        numbers: numbers_of_the_sketches(document),
        volumes,
    }
}

fn assert_seen_alike(now: &Seen, then: &Seen, what: &str) {
    assert_eq!(
        (&now.kinds, &now.planes, &now.numbers),
        (&then.kinds, &then.planes, &then.numbers),
        "{what}: its steps, or its sketches' planes and numbers, are not as they were",
    );
    assert_eq!(
        now.volumes.len(),
        then.volumes.len(),
        "{what}: it is read through another count of key steps",
    );
    for (key_step, (volume, was)) in now.volumes.iter().zip(&then.volumes).enumerate() {
        assert_volume(
            *volume,
            *was,
            &format!("{what}, read through key step {key_step},"),
        );
    }
}

#[test]
fn undoing_and_redoing_an_insertion_gives_back_the_part_as_it_was() {
    let mut document = a_part_of_six_steps();
    let before = seen(&document);
    insert_a_block_after_the_second_sketch(&mut document);
    let inserted = seen(&document);

    for _ in 0..3 {
        document.undo();
    }
    assert_seen_alike(
        &seen(&document),
        &before,
        "the three actions of the insertion undone, the part",
    );
    for _ in 0..3 {
        document.redo();
    }
    assert_seen_alike(
        &seen(&document),
        &inserted,
        "the three actions of the insertion redone, the part",
    );
}

/// A square of side `side` about `centre`, a place of XY, drawn in a sketch
/// in its own terms, then raised by `depth` — `ExtrusionMode::Cut` takes it
/// away — as a step of its own opened after `key_step`, or at the end.
fn a_square_raised(
    document: &mut PartDocument,
    sketch: usize,
    (centre, side): (DVec2, f64),
    depth: f64,
    (mode, key_step): (ExtrusionMode, Option<usize>),
) {
    let plane = document.sketches()[sketch].plane;
    let local = |place: DVec2| plane.to_local(place.extend(plane.origin.z));
    let half = DVec2::splat(side / 2.0);
    document.apply(Operation::AddRectangle {
        sketch,
        corner: PointRef::New(local(centre - half)),
        opposite: PointRef::New(local(centre + half)),
        construction: false,
    });
    let raised = Operation::Extrude {
        sketch,
        areas: document.areas_at(sketch, &[local(centre)]),
        distance: depth.into(),
        mode,
    };
    match key_step {
        Some(key_step) => document.insert_after(key_step, raised),
        None => document.apply(raised),
    };
}

/// Opens a sketch on the plane `plane`, held by no face, as a step of its own
/// opened after `key_step`, or at the end. Says which sketch it opened.
fn open_a_sketch(document: &mut PartDocument, plane: WorkPlane, key_step: Option<usize>) -> usize {
    let opened = Operation::CreateSketch { plane, on: None };
    match key_step {
        Some(key_step) => document.insert_after(key_step, opened),
        None => document.apply(opened),
    };
    document.sketches().len() - 1
}

/// Opens a sketch on the face a ray coming down on `place` meets first, as a
/// click there does: held by that face, laid on the plane it offers. Says
/// which sketch it opened.
fn open_a_sketch_on_the_face_over(document: &mut PartDocument, place: DVec2) -> usize {
    let face = document
        .body()
        .ray_hit(place.extend(100.0), DVec3::NEG_Z)
        .expect("a face under that place")
        .face;
    let offered = document.body().plane_of(face).expect("a flat face");
    let up = DVec3::Y;
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::from_face(&offered.corners, offered.normal, up),
        on: Some(FaceAnchor { face, up }),
    });
    document.sketches().len() - 1
}

fn at_height(height: f64) -> WorkPlane {
    WorkPlane {
        origin: DVec3::new(0.0, 0.0, height),
        ..WorkPlane::XY
    }
}

/// The issue's « To try it »: a block 10 by 10 raised 10, and a hole 2 by 2
/// through its middle, cut 20 up from XY. Four key steps, 960 of matter.
fn a_block_with_a_hole() -> PartDocument {
    let mut document = PartDocument::new("part", Utc::now());
    let block = open_a_sketch(&mut document, WorkPlane::XY, None);
    a_square_raised(
        &mut document,
        block,
        (DVec2::splat(5.0), 10.0),
        10.0,
        (ExtrusionMode::Add, None),
    );
    let hole = open_a_sketch(&mut document, WorkPlane::XY, None);
    a_square_raised(
        &mut document,
        hole,
        (DVec2::splat(5.0), 2.0),
        20.0,
        (ExtrusionMode::Cut, None),
    );
    assert_volume(
        document.body().volume(),
        960.0,
        "the fixture, a block of 1000 with a hole of 40 through it,",
    );
    document
}

#[test]
fn the_steps_after_an_insertion_replay_on_top_of_it() {
    let mut document = a_block_with_a_hole();

    let boss = open_a_sketch(&mut document, at_height(10.0), Some(1));
    a_square_raised(
        &mut document,
        boss,
        (DVec2::splat(5.0), 6.0),
        5.0,
        (ExtrusionMode::Add, Some(2)),
    );

    assert_eq!(
        kinds_of_the_steps(&document),
        [
            StepKind::Sketch,
            StepKind::Extrusion,
            StepKind::Sketch,
            StepKind::Extrusion,
            StepKind::Sketch,
            StepKind::Extrusion,
        ],
        "a step was dropped, or the boss's two do not stand third and fourth",
    );
    assert_volume(
        document.shown_through(&BTreeMap::new(), 3).body.volume(),
        1180.0,
        "read through the boss, the block and the boss 6 by 6 raised 5, the hole still to come,",
    );
    assert_volume(
        document.body().volume(),
        1120.0,
        "the part, the hole cut through the block and through the boss on top of it,",
    );
    assert_eq!(
        top_over(&document, DVec2::splat(5.0)),
        None,
        "the hole, replayed on top of the boss put in before it, does not go through it",
    );
    assert_top(
        &document,
        DVec2::splat(3.0),
        15.0,
        "beside the hole, the boss's top",
    );
}

/// A block 10 by 10 raised 10 at x = 0, a second raised 5 at x = 40, then a
/// sketch on the face over `on` and a square of side 4 raised 3 there.
fn two_blocks_and_a_sketch_on_the_face_over(on: DVec2) -> (PartDocument, usize) {
    let mut document = PartDocument::new("part", Utc::now());
    for (centre, depth) in [(DVec2::splat(5.0), 10.0), (DVec2::new(45.0, 5.0), 5.0)] {
        let sketch = open_a_sketch(&mut document, WorkPlane::XY, None);
        a_square_raised(
            &mut document,
            sketch,
            (centre, 10.0),
            depth,
            (ExtrusionMode::Add, None),
        );
    }
    let on_the_face = open_a_sketch_on_the_face_over(&mut document, on);
    a_square_raised(
        &mut document,
        on_the_face,
        (on, 4.0),
        3.0,
        (ExtrusionMode::Add, None),
    );
    assert_volume(
        document.body().volume(),
        1548.0,
        "the fixture, two blocks of 1000 and 500 and a square of 48 on one of them,",
    );
    assert!(
        !document.is_adrift(on_the_face),
        "the fixture wants the sketch on the face held there",
    );
    (document, on_the_face)
}

#[test]
fn a_step_that_loses_its_face_to_an_insertion_is_flagged_and_kept() {
    let (mut document, on_the_top) = two_blocks_and_a_sketch_on_the_face_over(DVec2::splat(5.0));
    let before = kinds_of_the_steps(&document).len();

    let cut = open_a_sketch(&mut document, WorkPlane::XY, Some(1));
    a_square_raised(
        &mut document,
        cut,
        (DVec2::splat(5.0), 20.0),
        20.0,
        (ExtrusionMode::Cut, Some(2)),
    );

    assert!(
        document.is_adrift(on_the_top),
        "the first block cut away whole by the steps put in after it, the sketch that stood \
         on its top is not marked as having lost its face: it is laid on {:?}",
        document.sketches()[on_the_top].plane,
    );
    assert_eq!(
        (
            kinds_of_the_steps(&document).len(),
            document.sketches()[on_the_top].live_segments().count()
        ),
        (before + 2, 4),
        "a step, or the square drawn on the sketch that lost its face, was deleted on its \
         own, as (key steps, the square's sides)",
    );
}

/// Where a sketch is laid: its origin and which way it faces.
fn laid(document: &PartDocument, sketch: usize) -> (DVec3, DVec3) {
    let plane = document.sketches()[sketch].plane;
    (plane.origin, plane.normal())
}

#[test]
fn a_sketch_on_a_face_made_after_an_insertion_stays_on_that_face() {
    let (mut document, on_the_second) =
        two_blocks_and_a_sketch_on_the_face_over(DVec2::new(45.0, 5.0));
    let before = laid(&document, on_the_second);
    assert_top(
        &document,
        DVec2::new(45.0, 5.0),
        8.0,
        "the fixture, the square of side 4 raised 3 on the second block's top,",
    );

    insert_a_block_after_the_second_sketch(&mut document);

    let after = laid(&document, on_the_second);
    assert!(
        !document.is_adrift(on_the_second)
            && after.0.distance(before.0) < HEIGHT
            && after.1.distance(before.1) < HEIGHT,
        "a block put in before the second block was raised moved the sketch on that block's \
         top off it, from {before:?} to {after:?}, adrift: {}",
        document.is_adrift(on_the_second),
    );
    assert_top(
        &document,
        DVec2::new(45.0, 5.0),
        8.0,
        "the square raised on the second block's top, once a block is put in before it,",
    );
    assert_volume(
        document.body().volume(),
        2148.0,
        "the part, with the 10 by 20 raised 3 put in,",
    );
}

#[test]
fn compacting_a_part_with_an_insertion_keeps_the_order_of_its_part() {
    let mut document = a_part_of_six_steps();
    insert_a_block_after_the_second_sketch(&mut document);
    let inserted = seen(&document);

    document.compact_history();

    assert_seen_alike(&seen(&document), &inserted, "the part compacted");
}

/// What each sketch has drawn, sketch by sketch: its traits, and the corners
/// it has cut.
fn drawings(document: &PartDocument) -> Vec<(usize, usize)> {
    document
        .sketches()
        .iter()
        .map(|sketch| {
            (
                sketch.live_segments().count(),
                sketch.live_corner_cuts().count(),
            )
        })
        .collect()
}

#[test]
fn compacting_keeps_a_size_written_from_a_variable_on_its_own_sketch() {
    let mut document = a_part_of_six_steps();
    let two = document.variables().read("2").expect("it reads");
    document
        .change_variable(VariableChange::Added {
            name: "rounding".to_string(),
            formula: two,
        })
        .expect("nothing to refuse");
    let rounding = document.variables().read("rounding").expect("it reads");
    document.apply(Operation::Fillet {
        sketch: 2,
        corners: vec![Corner::Between(SegmentId(0).into(), SegmentId(1).into())],
        radius: rounding,
        construction: false,
    });
    insert_a_block_after_the_second_sketch(&mut document);
    let (drawn, inserted) = (drawings(&document), seen(&document));
    assert_eq!(
        drawn.iter().map(|(_, cut)| *cut).collect::<Vec<_>>(),
        [0, 0, 1, 0],
        "the fixture wants one corner of the third sketch rounded by `rounding`, and none \
         of the sketch put in",
    );

    document.compact_history();

    assert_eq!(
        drawings(&document),
        drawn,
        "compacted, the third sketch, laid down again as it was drawn for its corner \
         rounded by `rounding`, or the sketch put in, no longer holds what it held, as \
         (traits, corners cut) sketch by sketch",
    );
    assert_seen_alike(&seen(&document), &inserted, "the part compacted");
}
