//! A part read through one of its key steps, as the step reader of the Part
//! panel reads it: the design's steps replayed through that one, each whole
//! as it now stands, the part itself left alone.
//!
//! Through a key step, not up to a place in the list: a sketch edited after
//! the steps that follow it keeps that edit when read, because the design
//! replays a step whole. The part as it stood when an operation was typed is
//! the history's own reading, which `rewind_to` gives and undo walks; the
//! reader is not the undo stack. A section raises nothing, so it counts for
//! no key step and changes nothing read.
//!
//! Closes #356 — the whole transcription is in
//! `crates/app/src/screens/part_tree/state/tests/reader.rs`; this bullet is
//! the part's half of a criterion, before any frame draws it.
//! - A click sets the reading point. Clicking a box makes the part computed up
//!   to that step included; the boxes after it are marked as not applied, and
//!   so are the arrows between them. Clicking the last box brings the part
//!   back whole — the part's half: read through a key step, the part is its
//!   design replayed through that step, each step whole as it now stands; read
//!   through the last, or past it, it is the part. Decided when it was taken
//!   up: each volume read keeps the state it is shown in, and a volume past
//!   the reading point is not replayed at all, so that it brings no matter and
//!   leaves no ghost whatever its state —
//!   `a_part_read_through_a_key_step_is_its_design_replayed_through_that_step`

use std::collections::BTreeMap;

use cao_part::history::{SectionAsked, SideDrawing};
use cao_part::{ExtrusionMode, Operation, PartDocument, PointRef, VolumeState};
use cao_sketch::{DimensionTarget, SegmentId, WorkPlane};
use chrono::Utc;
use glam::{DVec2, DVec3};

/// How far from the arithmetic a volume of flat-sided matter may stand.
const VOLUME: f64 = 1e-6;

/// Where the block is raised, and where the slab is, in the history of
/// `a_part_whose_first_sketch_was_edited_last`.
const THE_BLOCK: usize = 3;
const THE_SLAB: usize = 7;

fn length_of_the_first_side(document: &mut PartDocument, millimetres: f64) {
    document.apply(Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Length(SegmentId(0)),
        value: millimetres.into(),
        placement: None,
    });
}

/// A square of side 10 drawn on XY and raised 10 into a block, a section kept
/// across it, a second sketch on XY and a slab 10 by 10 raised 5 from it at
/// x = 40; then the first square's side typed again, 20 rather than 10, which
/// makes the block 20 long. Four key steps: the first sketch, the block, the
/// second sketch, the slab.
fn a_part_whose_first_sketch_was_edited_last() -> PartDocument {
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::ZERO),
        opposite: PointRef::New(DVec2::splat(10.0)),
        construction: false,
    });
    length_of_the_first_side(&mut document, 10.0);
    document.apply(Operation::Extrude {
        sketch: 0,
        areas: document.areas_at(0, &[DVec2::splat(5.0)]),
        distance: 10.0.into(),
        mode: ExtrusionMode::Add,
    });
    document.apply(Operation::SaveSection(SectionAsked {
        plane: WorkPlane::YZ,
        on: None,
        offset: 5.0,
        behind: SideDrawing::SeeThrough,
        in_front: SideDrawing::Opaque,
        tilt_about_u_degrees: 0.0,
        tilt_about_v_degrees: 0.0,
        pivot: DVec3::ZERO,
    }));
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 1,
        corner: PointRef::New(DVec2::new(40.0, 0.0)),
        opposite: PointRef::New(DVec2::new(50.0, 10.0)),
        construction: false,
    });
    document.apply(Operation::Extrude {
        sketch: 1,
        areas: document.areas_at(1, &[DVec2::new(45.0, 5.0)]),
        distance: 5.0.into(),
        mode: ExtrusionMode::Add,
    });
    length_of_the_first_side(&mut document, 20.0);

    let operations = document.history.applied_operations();
    assert!(
        matches!(operations.get(THE_BLOCK), Some(Operation::Extrude { .. }))
            && matches!(operations.get(THE_SLAB), Some(Operation::Extrude { .. })),
        "the fixture wants the block then the slab where it says: {operations:?}",
    );
    assert_volume(
        document.body().volume(),
        2500.0,
        "the part, a block 20 by 10 by 10 and a slab 10 by 10 by 5",
    );
    document
}

fn assert_volume(volume: f64, expected: f64, what: &str) {
    assert!(
        (volume - expected).abs() <= VOLUME * expected.max(1.0),
        "{what} holds {volume}, not {expected}",
    );
}

fn number_of(document: &PartDocument, at: usize) -> u32 {
    document.history.number_at(at).expect("an operation there")
}

#[test]
fn a_part_read_through_a_key_step_is_its_design_replayed_through_that_step() {
    let document = a_part_whose_first_sketch_was_edited_last();
    let all_opaque = BTreeMap::new();

    for (key_step, expected, what) in [
        (0, 0.0, "read through the first sketch, the part"),
        (
            1,
            2000.0,
            "read through the block, the part — the block as its sketch now stands, 20 long, \
             and not as it stood when the slab was raised —",
        ),
        (
            2,
            2000.0,
            "read through the second sketch, which raises nothing, the part",
        ),
        (
            3,
            2500.0,
            "read through the slab, the last key step, the part",
        ),
        (7, 2500.0, "read past the last key step, the part"),
    ] {
        let read = document.shown_through(&all_opaque, key_step);
        assert_volume(read.body.volume(), expected, what);
        assert!(
            read.ghosts.is_empty(),
            "{what} lays a ghost with every volume opaque"
        );
    }

    let (block, slab) = (
        number_of(&document, THE_BLOCK),
        number_of(&document, THE_SLAB),
    );
    let both_see_through = BTreeMap::from([
        (block, VolumeState::SeeThrough),
        (slab, VolumeState::SeeThrough),
    ]);
    let read = document.shown_through(&both_see_through, 1);
    let ghosts: Vec<(u32, f64)> = read
        .ghosts
        .iter()
        .map(|ghost| (ghost.step, ghost.body.volume()))
        .collect();
    assert_volume(
        read.body.volume(),
        0.0,
        "read through the block, both volumes see-through, the opaque part",
    );
    assert_eq!(
        ghosts.iter().map(|(step, _)| *step).collect::<Vec<_>>(),
        [block],
        "read through the block, both volumes see-through: the block read is not the one \
         ghost, or the slab past the reading point lays one",
    );
    assert_volume(ghosts[0].1, 2000.0, "the block's ghost");
    assert_volume(
        document.body().volume(),
        2500.0,
        "the part itself, once read through its block,",
    );
}
