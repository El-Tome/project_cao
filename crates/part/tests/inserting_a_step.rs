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
//! the panel shows — belongs to `cao_app`, and is transcribed under a block of
//! its own in `crates/app/tests/the_step_reader.rs`.
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
//!   `a_step_that_loses_its_face_to_an_insertion_is_flagged_and_kept`
//! - Names: a sketch inserted between « Esquisse 2 » and « Esquisse 3 » takes
//!   the next number, « Esquisse 4 ». The others are not renumbered — the
//!   number, which the part keeps:
//!   `a_sketch_inserted_between_the_second_and_the_third_takes_the_next_number`;
//!   the name the panel writes out of it is the reader's half
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

use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};

use cao_part::history::{Step, StepKind};
use cao_part::{ExtrusionMode, FileError, Files, Operation, PartDocument, PointRef};
use cao_sketch::WorkPlane;
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

#[test]
fn a_sketch_and_its_extrusion_made_while_read_through_step_three_go_in_after_it() {
    let mut document = a_part_of_six_steps();

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

    let kinds: Vec<StepKind> = document.history.steps().iter().map(Step::kind).collect();
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
