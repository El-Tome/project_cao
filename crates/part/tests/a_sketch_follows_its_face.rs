//! A drawing laid on a face of the part travels with that face when the part
//! changes shape.
//!
//! Closes #359.
//! - changing a size under an extrusion carries a sketch drawn on a face of it
//!   along, keeping its place on the face —
//!   `a_sketch_on_a_face_rides_the_face_when_the_part_grows`
//! - a sketch on one of the three base planes never moves —
//!   `a_sketch_on_a_base_plane_stays_where_it_was_put`
//! - a face that no longer exists marks the sketch rather than the drawing
//!   catching another — `a_sketch_whose_face_is_gone_says_so_and_stays_put`,
//!   and the tree says which, held beside the presenter by
//!   `a_sketch_that_lost_its_face_is_marked_in_the_tree`
//! - a point dropped on a corner is held there, and a sketch its own rules
//!   already determine lets go — no test: not in this half of #359. The issue
//!   stays open for it, as #355 did for #348.

use cao_part::history::{ExtrusionMode, FaceAnchor, Operation, PointRef};
use cao_part::{History, PartState};
use cao_sketch::{Area, WorkPlane};
use glam::{DVec2, DVec3};

/// The areas these places fall in, as the drawing stands — what the
/// interface works out at the moment of the click, for a test that has no
/// interface to click in.
fn clicked(history: &History, sketch: usize, place: DVec2) -> Vec<Area> {
    PartState::rebuild(history).areas_at(sketch, &[place])
}

/// How tall the block was the day the drawing was laid on its top, and so
/// where the plane recorded with the step sits. Only the anchor can lift it.
const AS_DRAWN: f64 = 10.0;

/// A block raised from XY, then a sketch started on the face that is its top.
fn block_with_a_sketch_on_top(height: f64, on: Option<FaceAnchor>) -> PartState {
    let mut history = History::default();
    history.push(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    history.push(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::ZERO),
        opposite: PointRef::New(DVec2::new(40.0, 20.0)),
        construction: false,
    });
    history.push(Operation::Extrude {
        sketch: 0,
        areas: clicked(&history, 0, DVec2::new(20.0, 10.0)),
        distance: height.into(),
        mode: ExtrusionMode::Add,
    });
    history.push(Operation::CreateSketch {
        plane: WorkPlane {
            origin: DVec3::new(0.0, 0.0, AS_DRAWN),
            u: DVec3::X,
            v: DVec3::Y,
        },
        on,
    });
    PartState::rebuild(&history)
}

/// The face the top of the block is, as the replay numbers it.
const TOP: usize = 1;

#[test]
fn a_sketch_on_a_face_rides_the_face_when_the_part_grows() {
    let anchor = Some(FaceAnchor {
        face: TOP,
        up: DVec3::Y,
    });

    let low = block_with_a_sketch_on_top(10.0, anchor);
    let high = block_with_a_sketch_on_top(30.0, anchor);

    assert!(
        (low.sketches[1].plane.origin.z - 10.0).abs() < 1e-9,
        "the drawing sat at {:?} on a block 10 tall",
        low.sketches[1].plane.origin,
    );
    assert!(
        (high.sketches[1].plane.origin.z - 30.0).abs() < 1e-9,
        "the block grew to 30 and the drawing stayed at {:?}",
        high.sketches[1].plane.origin,
    );
}

#[test]
fn a_sketch_on_a_base_plane_stays_where_it_was_put() {
    let low = block_with_a_sketch_on_top(10.0, None);
    let high = block_with_a_sketch_on_top(30.0, None);

    assert_eq!(low.sketches[0].plane, WorkPlane::XY);
    assert_eq!(high.sketches[0].plane, WorkPlane::XY);
}

#[test]
fn a_sketch_whose_face_is_gone_says_so_and_stays_put() {
    let anchor = Some(FaceAnchor {
        face: 404,
        up: DVec3::Y,
    });

    let state = block_with_a_sketch_on_top(10.0, anchor);

    assert!(
        (state.sketches[1].plane.origin.z - 10.0).abs() < 1e-9,
        "a drawing whose face is gone keeps the plane it last had",
    );
    assert!(
        state.adrift.contains(&1),
        "a drawing whose face is gone has to be marked, not quietly re-anchored",
    );
}
