//! What app · screens/viewport/visibility.rs is held to.
//!
//! Closes #734 — the whole transcription is in
//! `crates/app/src/screens/part_tree/state/tests.rs`; these bullets are the
//! ones the canvas's state answers, before any row or frame reads it.
//! - each volume's visibility cycles opaque → « transparente » → invisible on
//!   its own — `a_volume_s_visibility_cycles_opaque_see_through_invisible_and_round_again`;
//!   it belongs to the step's operation, not to its place in the history, as a
//!   sketch's eye does —
//!   `a_volume_s_visibility_belongs_to_its_operation_not_to_its_place_in_the_history`
//! - « TOUT LE CORPS » sets every volume at once —
//!   `choosing_for_the_whole_body_sets_every_volume_at_once`; it is lit on the
//!   state they all share, and on no segment when they differ —
//!   `the_whole_body_reads_the_state_every_volume_shares_and_none_once_they_differ`
//! - a face of the part made by no listed volume takes the last state chosen in
//!   « TOUT LE CORPS » (opaque at first), even when no segment is lit any more
//!   — `the_last_whole_body_choice_stays_for_the_faces_no_volume_made_once_the_volumes_differ`;
//!   asked face by face, a face takes the state of the volume that made it
//!   (`faces_made_by`), and one no volume made the last whole-body choice —
//!   `a_face_takes_its_volume_s_state_and_one_no_volume_made_the_last_whole_body_choice`

use cao_part::{ExtrusionMode, PointRef};
use cao_sketch::WorkPlane;
use chrono::Utc;
use glam::DVec2;

use super::*;

/// Where the block stands in the history of `a_block_with_a_hole`.
const THE_BLOCK: usize = 3;
/// Where the cut that bores the hole stands.
const THE_HOLE: usize = 4;

/// A block 10 by 20 raised 4 from XY, and a cut through it, round, at its
/// middle: two volumes, an add and a cut.
fn a_block_with_a_hole() -> PartDocument {
    let mut document = PartDocument::new("part", Utc::now());
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
    document.apply(Operation::AddCircle {
        sketch: 0,
        center: PointRef::New(DVec2::new(5.0, 10.0)),
        radius: 2.0,
        rim: Vec::new(),
        construction: false,
    });
    document.apply(Operation::Extrude {
        sketch: 0,
        areas: document.areas_at(0, &[DVec2::new(1.0, 1.0), DVec2::new(5.0, 10.0)]),
        distance: 4.0.into(),
        mode: ExtrusionMode::Add,
    });
    document.apply(a_cut_through_the_middle(&document, 4.0));
    let steps = document.history.applied_operations();
    assert!(
        matches!(
            steps.get(THE_BLOCK),
            Some(Operation::Extrude {
                mode: ExtrusionMode::Add,
                ..
            })
        ) && matches!(
            steps.get(THE_HOLE),
            Some(Operation::Extrude {
                mode: ExtrusionMode::Cut,
                ..
            })
        ) && steps.len() == THE_HOLE + 1,
        "the fixture wants the block then the hole, last: {steps:?}",
    );
    assert!(!document.body().is_empty(), "the block is raised");
    document
}

/// The disc at the block's middle taken out, that deep from XY.
fn a_cut_through_the_middle(document: &PartDocument, depth: f64) -> Operation {
    Operation::Extrude {
        sketch: 0,
        areas: document.areas_at(0, &[DVec2::new(5.0, 10.0)]),
        distance: depth.into(),
        mode: ExtrusionMode::Cut,
    }
}

const EVERY_STATE: [MatterInFront; 3] = [
    MatterInFront::Hidden,
    MatterInFront::SeeThrough,
    MatterInFront::Opaque,
];

#[test]
fn a_volume_s_visibility_cycles_opaque_see_through_invisible_and_round_again() {
    let document = a_block_with_a_hole();
    let mut state = ViewportState::default();
    assert_eq!(
        state.volume_mode(&document, THE_HOLE),
        MatterInFront::Opaque,
        "a volume opens opaque",
    );

    let mut seen = Vec::new();
    for _ in 0..3 {
        state.next_volume_mode(&document, THE_HOLE);
        seen.push(state.volume_mode(&document, THE_HOLE));
    }

    assert_eq!(
        seen,
        [
            MatterInFront::SeeThrough,
            MatterInFront::Hidden,
            MatterInFront::Opaque
        ],
        "opaque, then see-through, then invisible, then opaque again",
    );
    state.next_volume_mode(&document, THE_HOLE);
    assert_eq!(
        state.volume_mode(&document, THE_BLOCK),
        MatterInFront::Opaque,
        "the cut made see-through takes the block with it",
    );
}

#[test]
fn a_volume_s_visibility_belongs_to_its_operation_not_to_its_place_in_the_history() {
    let mut document = a_block_with_a_hole();
    let mut state = ViewportState::default();
    state.next_volume_mode(&document, THE_HOLE);
    assert_eq!(
        state.volume_mode(&document, THE_HOLE),
        MatterInFront::SeeThrough
    );

    document.rewind_to(THE_HOLE);
    document.apply(a_cut_through_the_middle(&document, 2.0));

    assert!(
        matches!(
            document.history.applied_operations().get(THE_HOLE),
            Some(Operation::Extrude {
                mode: ExtrusionMode::Cut,
                ..
            })
        ),
        "the fixture wants a new cut where the old one stood",
    );
    assert_eq!(
        state.volume_mode(&document, THE_HOLE),
        MatterInFront::Opaque,
        "a new cut recorded where an undone one stood inherits its state",
    );
}

#[test]
fn choosing_for_the_whole_body_sets_every_volume_at_once() {
    let document = a_block_with_a_hole();

    for chosen in EVERY_STATE {
        let mut state = ViewportState::default();
        state.next_volume_mode(&document, THE_HOLE);

        state.set_whole_body(chosen);

        assert_eq!(
            [THE_BLOCK, THE_HOLE].map(|step| state.volume_mode(&document, step)),
            [chosen; 2],
            "« TOUT LE CORPS » set to {chosen:?} leaves a volume in another state",
        );
    }
}

#[test]
fn the_whole_body_reads_the_state_every_volume_shares_and_none_once_they_differ() {
    let document = a_block_with_a_hole();
    let mut state = ViewportState::default();
    assert_eq!(
        state.shared_volume_mode(&document),
        Some(MatterInFront::Opaque),
        "every volume opens opaque, and « Opaque » is lit",
    );

    state.next_volume_mode(&document, THE_HOLE);
    assert_eq!(
        state.shared_volume_mode(&document),
        None,
        "the cut see-through and the block opaque light no segment",
    );

    state.next_volume_mode(&document, THE_BLOCK);
    assert_eq!(
        state.shared_volume_mode(&document),
        Some(MatterInFront::SeeThrough),
        "both made see-through one by one light « Transparent »",
    );

    state.set_whole_body(MatterInFront::Hidden);
    assert_eq!(
        state.shared_volume_mode(&document),
        Some(MatterInFront::Hidden),
        "« Invisible » chosen is lit",
    );
}

#[test]
fn the_last_whole_body_choice_stays_for_the_faces_no_volume_made_once_the_volumes_differ() {
    let document = a_block_with_a_hole();
    let mut state = ViewportState::default();
    assert_eq!(
        state.whole_body_choice(),
        MatterInFront::Opaque,
        "before any choice, the faces no volume made are opaque",
    );

    state.set_whole_body(MatterInFront::SeeThrough);
    state.next_volume_mode(&document, THE_HOLE);
    assert_eq!(
        state.shared_volume_mode(&document),
        None,
        "the fixture wants the volumes to differ",
    );

    assert_eq!(
        state.whole_body_choice(),
        MatterInFront::SeeThrough,
        "a volume changed on its own takes the last choice away from the faces no volume made",
    );
}

/// A face no step handed out. No part built today has a face that no
/// extrusion or revolution made, so this number stands in for one.
const A_FACE_NO_VOLUME_MADE: usize = usize::MAX;

#[test]
fn a_face_takes_its_volume_s_state_and_one_no_volume_made_the_last_whole_body_choice() {
    let document = a_block_with_a_hole();
    let made_by = |step: usize| {
        let number = document.history.number_at(step).expect("a numbered step");
        document.faces_made_by(number)
    };
    let (block, wall) = (made_by(THE_BLOCK), made_by(THE_HOLE));
    assert!(
        !block.is_empty() && !wall.is_empty(),
        "the fixture wants faces made by each volume: {block:?} {wall:?}",
    );
    let mut state = ViewportState::default();
    state.set_whole_body(MatterInFront::Hidden);
    state.next_volume_mode(&document, THE_HOLE);
    state.next_volume_mode(&document, THE_BLOCK);
    state.next_volume_mode(&document, THE_BLOCK);

    for face in &block {
        assert_eq!(
            state.face_modes(&document).of(Some(*face)),
            MatterInFront::SeeThrough,
            "face {face}, made by the block, is not drawn as the block is",
        );
    }
    for face in &wall {
        assert_eq!(
            state.face_modes(&document).of(Some(*face)),
            MatterInFront::Opaque,
            "face {face}, a wall of the hole, is not drawn as the cut is",
        );
    }
    assert_eq!(
        state.face_modes(&document).of(Some(A_FACE_NO_VOLUME_MADE)),
        MatterInFront::Hidden,
        "a face no volume made is not drawn as « TOUT LE CORPS » last chose",
    );
}
