//! A step of matter changed after it was raised: an extrusion or a revolution
//! given new values long after, the part raised again from them, in the step
//! it already had.
//!
//! Closes #736 — the whole transcription is in `crates/app/tests/the_alerts.rs`;
//! these bullets are the part's half of one decision there, before any panel
//! reopens.
//! - decided by Tom when the work opened: « Modifier » on an extrusion's alert
//!   reopens the extrusion, its panel filled with its values, where no
//!   extrusion could be reopened before — the part's half: an extrusion's
//!   values are changed after it was raised, and the part is raised again from
//!   them, every step after it on top, with no step added and the extrusion
//!   reading its new values —
//!   `an_extrusion_changed_after_it_was_raised_is_raised_again_from_its_new_values_in_its_own_step`
//! - the change is one step of the history, which one undo takes back whole —
//!   `one_undo_takes_back_a_change_of_extrusion`
//! - an extrusion the kernel declined, changed to values it can build, is
//!   built: what makes its alert go —
//!   `a_declined_extrusion_changed_to_values_that_build_is_built`
//! - the change is kept in the part's file, saved or put away, and the part
//!   opens on it — `a_changed_extrusion_comes_back_changed_from_the_part_s_file`
//! - decided when the extrusion's change was taken up: a revolution is changed
//!   the same way, the one road serving both steps of matter —
//!   `a_revolution_changed_after_it_was_turned_is_turned_again_from_its_new_angle`
//! - found when the change was made: compacting the history keeps the change,
//!   the step written down as it now stands, where reading the operations as
//!   first done would raise the block from its old values and drop the change
//!   — `compacting_the_history_keeps_the_change_of_an_extrusion`. Written after
//!   the code it holds, and passing from the first run: the compaction reads
//!   `applied_as_raised`, and reading `applied_operations` there instead is
//!   the one-line change it fails on
//!
//! The change is an operation of its own, `Operation::EditMatter`, rather than
//! the first operation rewritten in place: undo, redo and going back to a step
//! all move the history's cursor over a list that is only ever added to, and a
//! value overwritten in the list could not be taken back. Like a section's
//! change, it is filed under the step it changes, so the design keeps its
//! steps. It carries the whole operation the step is replayed as from then on,
//! which `PartDocument::raised_as` hands back: what the tree reads and the
//! extrusion's panel reopens on.
//!
//! Each part changed is held against the same part raised with those values
//! from the start: the same matter, worked out along the same road.

use std::cell::RefCell;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use cao_part::{
    ExtrusionMode, FileError, Files, Operation, PartDocument, PointRef, RevolutionAxis,
};
use cao_sketch::{PointId, SketchAxis, WorkPlane};
use chrono::Utc;
use glam::{DVec2, DVec3};

/// How far apart, relative to the larger, two volumes raised along the same
/// road may stand: the same matter, worked out twice.
const SAME_VOLUME: f64 = 1e-9;

/// Two rectangles drawn on XY, the first 10 by 20 raised `first_height`, then
/// the second, 10 by 10 beside it, raised 2: a step of matter after the one
/// that is changed.
fn two_blocks(first_height: f64) -> PartDocument {
    let mut document = PartDocument::new("Blocs", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    for (corner, opposite) in [
        (DVec2::ZERO, DVec2::new(10.0, 20.0)),
        (DVec2::new(20.0, 0.0), DVec2::new(30.0, 10.0)),
    ] {
        document.apply(Operation::AddRectangle {
            sketch: 0,
            corner: PointRef::New(corner),
            opposite: PointRef::New(opposite),
            construction: false,
        });
    }
    let first = the_first_block_raised(&document, first_height);
    document.apply(first);
    document.apply(Operation::Extrude {
        sketch: 0,
        areas: document.areas_at(0, &[IN_THE_SECOND]),
        distance: 2.0.into(),
        mode: ExtrusionMode::Add,
    });
    document
}

const IN_THE_FIRST: DVec2 = DVec2::new(5.0, 10.0);
const IN_THE_SECOND: DVec2 = DVec2::new(25.0, 5.0);

/// Where the first block is raised in the history of `two_blocks`.
const THE_FIRST_BLOCK: usize = 3;

/// The first block raised `height`, as an extrusion of `two_blocks` asks it.
fn the_first_block_raised(document: &PartDocument, height: f64) -> Operation {
    Operation::Extrude {
        sketch: 0,
        areas: document.areas_at(0, &[IN_THE_FIRST]),
        distance: height.into(),
        mode: ExtrusionMode::Add,
    }
}

/// The number of the operation that raised the step at `step` in the history.
fn number_of(document: &PartDocument, step: usize) -> u32 {
    document
        .history
        .number_at(step)
        .expect("the step is in the history")
}

fn same_volume(one: f64, other: f64) -> bool {
    (one - other).abs() <= SAME_VOLUME * one.abs().max(other.abs())
}

#[test]
fn an_extrusion_changed_after_it_was_raised_is_raised_again_from_its_new_values_in_its_own_step() {
    let mut document = two_blocks(4.0);
    let steps = document.history.steps().len();
    let raised = number_of(&document, THE_FIRST_BLOCK);
    let changed = the_first_block_raised(&document, 10.0);

    document.apply(Operation::EditMatter {
        raised_by: raised,
        to: Box::new(changed.clone()),
    });

    let as_if_raised_so = two_blocks(10.0).body().volume();
    let mut wrong = Vec::new();
    if !same_volume(document.body().volume(), as_if_raised_so) {
        wrong.push(format!(
            "the part holds {} where the two blocks raised 10 and 2 from the start hold {as_if_raised_so}",
            document.body().volume(),
        ));
    }
    if document.history.steps().len() != steps {
        wrong.push(format!(
            "the design has {} steps where it had {steps}",
            document.history.steps().len(),
        ));
    }
    if document.raised_as(raised) != Some(&changed) {
        wrong.push(format!(
            "the first block reads {:?}, not its new values",
            document.raised_as(raised),
        ));
    }

    assert!(
        wrong.is_empty(),
        "the first of two blocks, raised 4 then changed to 10, is not raised again from its new \
         values in its own step:\n  {}",
        wrong.join("\n  "),
    );
}

#[test]
fn one_undo_takes_back_a_change_of_extrusion() {
    let mut document = two_blocks(4.0);
    let raised = number_of(&document, THE_FIRST_BLOCK);
    let as_raised = document.history.applied_operations()[THE_FIRST_BLOCK].clone();
    let before = document.body().volume();
    let changed = the_first_block_raised(&document, 10.0);
    document.apply(Operation::EditMatter {
        raised_by: raised,
        to: Box::new(changed),
    });

    document.undo();

    assert!(
        same_volume(document.body().volume(), before)
            && document.raised_as(raised) == Some(&as_raised),
        "one undo after the first block was changed from 4 to 10 leaves {} where it held {before}, \
         and the block reading {:?}",
        document.body().volume(),
        document.raised_as(raised),
    );
}

/// A ball of radius 4 about (5, 10, 0), turned whole from a half disc, then a
/// circle of radius 2 on a plane tilted 30° about X, through (5, 10, 6),
/// raised `distance` along that plane's normal: raised 10 down, across the
/// ball's sphere off its centre, the kernel declines it; raised 10 up, away
/// from the ball, it builds.
fn a_ball_under_a_slanted_circle_raised(distance: f64) -> PartDocument {
    let mut document = PartDocument::new("Boule", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane {
            origin: DVec3::new(5.0, 10.0, 0.0),
            u: DVec3::X,
            v: DVec3::Z,
        },
        on: None,
    });
    let first = document.sketches()[0].points().len();
    document.apply(Operation::AddArc {
        sketch: 0,
        center: PointRef::New(DVec2::ZERO),
        start: PointRef::New(DVec2::new(0.0, -4.0)),
        end: PointRef::New(DVec2::new(0.0, 4.0)),
        construction: false,
    });
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::Existing(PointId(first + 2)),
        end: PointRef::Existing(PointId(first + 1)),
        construction: false,
    });
    document.apply(Operation::Revolve {
        sketch: 0,
        areas: document.areas_at(0, &[DVec2::new(1.0, 0.0)]),
        axis: RevolutionAxis::Sketch(SketchAxis::V),
        angle: 360.0.into(),
        mode: ExtrusionMode::Add,
    });
    let (sin, cos) = (std::f64::consts::PI / 6.0).sin_cos();
    document.apply(Operation::CreateSketch {
        plane: WorkPlane {
            origin: DVec3::new(5.0, 10.0, 6.0),
            u: DVec3::X,
            v: DVec3::new(0.0, cos, sin),
        },
        on: None,
    });
    document.apply(Operation::AddCircle {
        sketch: 1,
        center: PointRef::New(DVec2::ZERO),
        radius: 2.0,
        rim: Vec::new(),
        construction: false,
    });
    let circle = the_circle_raised(&document, distance);
    document.apply(circle);
    document
}

/// Where the slanted circle is raised in the history of
/// `a_ball_under_a_slanted_circle_raised`.
const THE_SLANTED_CIRCLE: usize = 6;

fn the_circle_raised(document: &PartDocument, distance: f64) -> Operation {
    Operation::Extrude {
        sketch: 1,
        areas: document.areas_at(1, &[DVec2::ZERO]),
        distance: distance.into(),
        mode: ExtrusionMode::Add,
    }
}

#[test]
fn a_declined_extrusion_changed_to_values_that_build_is_built() {
    let mut document = a_ball_under_a_slanted_circle_raised(-10.0);
    let raised = number_of(&document, THE_SLANTED_CIRCLE);
    assert!(
        document.is_declined(raised),
        "the fixture wants the circle raised 10 down across the ball declined",
    );
    let turned_up = the_circle_raised(&document, 10.0);

    document.apply(Operation::EditMatter {
        raised_by: raised,
        to: Box::new(turned_up),
    });

    let as_if_raised_so = a_ball_under_a_slanted_circle_raised(10.0).body().volume();
    assert!(
        !document.is_declined(raised) && same_volume(document.body().volume(), as_if_raised_so),
        "the circle declined raised 10 down, changed to 10 up, is declined still ({}) or holds {} \
         where the ball and the circle raised 10 up from the start hold {as_if_raised_so}",
        document.is_declined(raised),
        document.body().volume(),
    );
}

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

#[test]
fn a_changed_extrusion_comes_back_changed_from_the_part_s_file() {
    let mut document = two_blocks(4.0);
    let steps = document.history.steps().len();
    let raised = number_of(&document, THE_FIRST_BLOCK);
    let changed = the_first_block_raised(&document, 10.0);
    document.apply(Operation::EditMatter {
        raised_by: raised,
        to: Box::new(changed.clone()),
    });
    let files = Memory::default();
    let saved = Path::new("/parts/saved.caopart");
    let put_away = Path::new("/parts/put-away.caopart");
    document
        .save(&files, saved, Utc::now())
        .expect("the part is saved");
    document
        .put_away(&files, put_away, Utc::now())
        .expect("the part is put away");

    let as_if_raised_so = two_blocks(10.0).body().volume();
    let mut wrong = Vec::new();
    for path in [saved, put_away] {
        let reopened = PartDocument::load(&files, path).expect("the part opens again");
        if !same_volume(reopened.body().volume(), as_if_raised_so)
            || reopened.history.steps().len() != steps
            || reopened.raised_as(raised) != Some(&changed)
        {
            wrong.push(format!(
                "{path:?} opens on {} where {as_if_raised_so} was changed, in {} steps where \
                 there were {steps}, the first block reading {:?}",
                reopened.body().volume(),
                reopened.history.steps().len(),
                reopened.raised_as(raised),
            ));
        }
    }

    assert!(
        wrong.is_empty(),
        "the first block changed from 4 to 10 does not come back changed:\n  {}",
        wrong.join("\n  "),
    );
}

/// A rectangle drawn on XZ beside the X axis, from 5 to 10 off it, turned
/// `angle` degrees about it.
fn a_ring_turned(angle: f64) -> PartDocument {
    let mut document = PartDocument::new("Anneau", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XZ,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::new(0.0, 5.0)),
        opposite: PointRef::New(DVec2::new(10.0, 10.0)),
        construction: false,
    });
    let turned = the_ring_turned(&document, angle);
    document.apply(turned);
    document
}

/// Where the ring is turned in the history of `a_ring_turned`.
const THE_RING: usize = 2;

fn the_ring_turned(document: &PartDocument, angle: f64) -> Operation {
    Operation::Revolve {
        sketch: 0,
        areas: document.areas_at(0, &[DVec2::new(5.0, 7.5)]),
        axis: RevolutionAxis::Sketch(SketchAxis::U),
        angle: angle.into(),
        mode: ExtrusionMode::Add,
    }
}

#[test]
fn a_revolution_changed_after_it_was_turned_is_turned_again_from_its_new_angle() {
    let mut document = a_ring_turned(90.0);
    let raised = number_of(&document, THE_RING);
    let changed = the_ring_turned(&document, 180.0);

    document.apply(Operation::EditMatter {
        raised_by: raised,
        to: Box::new(changed.clone()),
    });

    let as_if_turned_so = a_ring_turned(180.0).body().volume();
    assert!(
        same_volume(document.body().volume(), as_if_turned_so)
            && document.raised_as(raised) == Some(&changed),
        "a ring turned 90° then changed to 180° holds {} where one turned 180° from the start \
         holds {as_if_turned_so}, and reads {:?}",
        document.body().volume(),
        document.raised_as(raised),
    );
}

#[test]
fn compacting_the_history_keeps_the_change_of_an_extrusion() {
    let mut document = two_blocks(4.0);
    let raised = number_of(&document, THE_FIRST_BLOCK);
    let changed = the_first_block_raised(&document, 10.0);
    document.apply(Operation::EditMatter {
        raised_by: raised,
        to: Box::new(changed),
    });

    document.compact_history();

    let as_if_raised_so = two_blocks(10.0).body().volume();
    let first_block_reads: Vec<&Operation> = document
        .history
        .applied_operations()
        .iter()
        .enumerate()
        .filter(|(_, operation)| matches!(operation, Operation::Extrude { .. }))
        .filter_map(|(at, _)| document.raised_as(number_of(&document, at)))
        .take(1)
        .collect();
    let raised_ten = |operation: &&Operation| matches!(operation, Operation::Extrude { distance, .. } if *distance == 10.0.into());
    assert!(
        same_volume(document.body().volume(), as_if_raised_so)
            && first_block_reads.iter().all(raised_ten)
            && !first_block_reads.is_empty(),
        "the first block, changed from 4 to 10 then the history compacted, leaves {} where the \
         blocks raised 10 and 2 hold {as_if_raised_so}, and reads {first_block_reads:?}",
        document.body().volume(),
    );
}
