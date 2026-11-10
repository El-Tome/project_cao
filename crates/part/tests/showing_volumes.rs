//! The part as it is shown once some of its volumes are made see-through or
//! invisible: the history replayed with each volume's state, each volume being
//! its raw solid — the block its step raised or removed, as extruded, whatever
//! the steps after it did to the part.
//!
//! Closes #734 — the whole transcription is in
//! `crates/app/src/screens/part_tree/state/tests.rs`; these bullets are the
//! ones the replay answers, before any frame draws it.
//! - an opaque step is applied as today, so that with every volume opaque the
//!   part shown is the part — `with_every_volume_opaque_the_part_shown_is_the_part`
//! - an invisible addition brings no matter —
//!   `an_invisible_addition_brings_no_matter`; hiding one of two additions
//!   leaves the other whole, with no hole —
//!   `hiding_one_of_two_overlapping_additions_leaves_the_other_whole`
//! - a see-through addition brings no matter to the opaque part, and its raw
//!   block is a see-through ghost: on a block with a notch, the block
//!   « transparente » leaves the whole raw block as a ghost —
//!   `a_see_through_addition_brings_no_matter_and_its_raw_block_is_a_ghost`
//! - an invisible cut is not applied, and the matter it removed comes back:
//!   the cut invisible leaves the block without its notch —
//!   `an_invisible_cut_is_not_applied_and_the_matter_it_removed_comes_back`
//! - a see-through cut is applied, and the raw block it removes is a ghost:
//!   the cut « transparente » leaves the notched part, the removed block a
//!   ghost in the notch —
//!   `a_see_through_cut_is_applied_and_the_block_it_removes_is_a_ghost`
//! - found while designing the replay: a face of the part shown answers to
//!   the number it has in the part, so that what is picked on the one is
//!   anchored on the other —
//!   `a_face_of_the_part_shown_answers_to_the_number_it_has_in_the_part`

use std::collections::BTreeMap;

use cao_part::{ExtrusionMode, Operation, PartDocument, PointRef, Shown, VolumeState};
use cao_sketch::WorkPlane;
use cao_solid::Body;
use chrono::Utc;
use glam::{DVec2, DVec3};

/// How far from the arithmetic a volume of flat-sided matter may stand.
const VOLUME: f64 = 1e-6;

/// How far a corner of a box may stand from where it was drawn.
const PLACE: f64 = 1e-9;

fn rectangle(document: &mut PartDocument, corner: DVec2, opposite: DVec2) {
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(corner),
        opposite: PointRef::New(opposite),
        construction: false,
    });
}

fn raised(document: &mut PartDocument, inside: &[DVec2], distance: f64, mode: ExtrusionMode) {
    document.apply(Operation::Extrude {
        sketch: 0,
        areas: document.areas_at(0, inside),
        distance: distance.into(),
        mode,
    });
}

/// A part with one sketch on XY, where both fixtures draw.
fn a_part() -> PartDocument {
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document
}

/// The number of the operation at that place in the history.
fn number_of(document: &PartDocument, step: usize) -> u32 {
    let number = document.history.number_at(step).expect("a step there");
    assert!(
        matches!(
            document.history.applied_operations().get(step),
            Some(Operation::Extrude { .. })
        ),
        "the fixture wants a volume at step {step}",
    );
    number
}

/// A block 10 by 10 by 10 at the origin, a notch cut into its corner by a
/// square 10 by 10 from (5, 5), 10 deep: a quarter of the block taken away.
/// Its block and its cut, by the number of their operations.
fn a_block_with_a_notch() -> (PartDocument, u32, u32) {
    let mut document = a_part();
    rectangle(&mut document, DVec2::ZERO, DVec2::splat(10.0));
    rectangle(&mut document, DVec2::splat(5.0), DVec2::splat(15.0));
    raised(
        &mut document,
        &[DVec2::splat(2.0), DVec2::splat(7.0)],
        10.0,
        ExtrusionMode::Add,
    );
    raised(
        &mut document,
        &[DVec2::splat(7.0), DVec2::splat(12.0)],
        10.0,
        ExtrusionMode::Cut,
    );
    assert_volume(document.body(), 750.0, "the notched block");
    let (block, notch) = (number_of(&document, 3), number_of(&document, 4));
    (document, block, notch)
}

/// Two blocks 10 by 10 and 4 high raised one after the other, the second
/// from x = 5, over half the first. Their numbers.
fn two_overlapping_additions() -> (PartDocument, u32, u32) {
    let mut document = a_part();
    rectangle(&mut document, DVec2::ZERO, DVec2::splat(10.0));
    rectangle(&mut document, DVec2::new(5.0, 0.0), DVec2::new(15.0, 10.0));
    raised(
        &mut document,
        &[DVec2::new(2.0, 5.0), DVec2::new(7.0, 5.0)],
        4.0,
        ExtrusionMode::Add,
    );
    raised(
        &mut document,
        &[DVec2::new(7.0, 5.0), DVec2::new(12.0, 5.0)],
        4.0,
        ExtrusionMode::Add,
    );
    assert_volume(document.body(), 600.0, "the two blocks joined");
    let (first, second) = (number_of(&document, 3), number_of(&document, 4));
    (document, first, second)
}

fn assert_volume(body: &Body, expected: f64, what: &str) {
    let volume = body.volume();
    assert!(
        (volume - expected).abs() <= VOLUME * expected.max(1.0),
        "{what} holds {volume}, not {expected}",
    );
}

fn assert_bounds(body: &Body, low: DVec3, high: DVec3, what: &str) {
    let (from, to) = body
        .bounds()
        .unwrap_or_else(|| panic!("{what} holds no matter"));
    assert!(
        from.distance(low) < PLACE && to.distance(high) < PLACE,
        "{what} spans {from} to {to}, not {low} to {high}",
    );
}

fn shown_with(document: &PartDocument, states: &[(u32, VolumeState)]) -> Shown {
    document.shown(&states.iter().copied().collect::<BTreeMap<_, _>>())
}

#[test]
fn with_every_volume_opaque_the_part_shown_is_the_part() {
    for (what, (document, first, second)) in [
        ("a notched block", a_block_with_a_notch()),
        ("two overlapping blocks", two_overlapping_additions()),
    ] {
        for states in [
            vec![],
            vec![(first, VolumeState::Opaque), (second, VolumeState::Opaque)],
        ] {
            let shown = shown_with(&document, &states);

            assert!(
                shown.body == *document.body(),
                "{what}, every volume opaque as {states:?}, is not shown as the part is",
            );
            assert!(
                shown.ghosts.is_empty(),
                "{what}, every volume opaque, is shown with ghosts",
            );
        }
    }
}

#[test]
fn an_invisible_addition_brings_no_matter() {
    let (document, block, _) = a_block_with_a_notch();

    let shown = shown_with(&document, &[(block, VolumeState::Invisible)]);

    assert!(
        shown.body.is_empty(),
        "the block invisible, the part still holds {}",
        shown.body.volume(),
    );
    assert!(
        shown.ghosts.is_empty(),
        "an invisible block is shown as a ghost"
    );
}

#[test]
fn hiding_one_of_two_overlapping_additions_leaves_the_other_whole() {
    let (document, first, second) = two_overlapping_additions();

    for (hidden, kept_from, what) in [
        (first, 5.0, "the first block invisible"),
        (second, 0.0, "the second block invisible"),
    ] {
        let shown = shown_with(&document, &[(hidden, VolumeState::Invisible)]);

        assert_volume(&shown.body, 400.0, what);
        assert_bounds(
            &shown.body,
            DVec3::new(kept_from, 0.0, 0.0),
            DVec3::new(kept_from + 10.0, 10.0, 4.0),
            what,
        );
        let over_the_overlap = shown
            .body
            .ray_hit(DVec3::new(7.5, 5.0, 50.0), DVec3::NEG_Z)
            .map(|hit| hit.distance);
        assert!(
            over_the_overlap.is_some_and(|distance| (distance - 46.0).abs() < 1e-6),
            "{what}, a ray down over the half both blocks stand on meets the other's top \
             at {over_the_overlap:?} along, not 46",
        );
    }
}

#[test]
fn a_see_through_addition_brings_no_matter_and_its_raw_block_is_a_ghost() {
    let (document, block, _) = a_block_with_a_notch();

    let shown = shown_with(&document, &[(block, VolumeState::SeeThrough)]);

    assert!(
        shown.body.is_empty(),
        "the block see-through, the opaque part still holds {}",
        shown.body.volume(),
    );
    let ghosts: Vec<u32> = shown.ghosts.iter().map(|ghost| ghost.step).collect();
    assert_eq!(ghosts, [block], "the ghosts are those of other steps");
    let ghost = &shown.ghosts[0].body;
    assert_volume(ghost, 1000.0, "the block's ghost");
    assert_bounds(
        ghost,
        DVec3::ZERO,
        DVec3::splat(10.0),
        "the block's ghost, raw, notch and all",
    );
}

#[test]
fn an_invisible_cut_is_not_applied_and_the_matter_it_removed_comes_back() {
    let (document, _, notch) = a_block_with_a_notch();

    let shown = shown_with(&document, &[(notch, VolumeState::Invisible)]);

    assert_volume(&shown.body, 1000.0, "the block, its cut invisible");
    assert!(
        shown.ghosts.is_empty(),
        "an invisible cut is shown as a ghost"
    );
}

#[test]
fn a_see_through_cut_is_applied_and_the_block_it_removes_is_a_ghost() {
    let (document, _, notch) = a_block_with_a_notch();

    let shown = shown_with(&document, &[(notch, VolumeState::SeeThrough)]);

    assert!(
        shown.body == *document.body(),
        "the cut see-through, the opaque part is not the notched part",
    );
    let ghosts: Vec<u32> = shown.ghosts.iter().map(|ghost| ghost.step).collect();
    assert_eq!(ghosts, [notch], "the ghosts are those of other steps");
    let ghost = &shown.ghosts[0].body;
    assert_volume(
        ghost,
        1000.0,
        "the cut's ghost, the whole raw block it removes",
    );
    assert_bounds(
        ghost,
        DVec3::new(5.0, 5.0, 0.0),
        DVec3::new(15.0, 15.0, 10.0),
        "the cut's ghost",
    );
}

/// The face of `shown` a ray straight down at (x, y) meets, and whether the
/// part's own face of that number is met at the same place: a face of the
/// part may answer to several numbers, and the shown face's need only be one.
fn met_under_the_same_number(
    document: &PartDocument,
    shown: &Body,
    (x, y): (f64, f64),
) -> (usize, Option<f64>, f64) {
    let (origin, down) = (DVec3::new(x, y, 50.0), DVec3::NEG_Z);
    let hit = shown
        .ray_hit(origin, down)
        .unwrap_or_else(|| panic!("nothing of the part shown under ({x}, {y})"));
    let in_the_part = document
        .body()
        .ray_hit_among(origin, down, &|face| face == hit.face)
        .map(|met| met.distance);
    (hit.face, in_the_part, hit.distance)
}

#[test]
fn a_face_of_the_part_shown_answers_to_the_number_it_has_in_the_part() {
    let (document, _, notch) = a_block_with_a_notch();
    let shown = shown_with(&document, &[(notch, VolumeState::Invisible)]);
    let (face, in_the_part, distance) =
        met_under_the_same_number(&document, &shown.body, (2.0, 2.0));
    assert!(
        in_the_part.is_some_and(|met| (met - distance).abs() < 1e-9),
        "the block's top, its cut invisible, answers to {face}, which in the part is met at \
         {in_the_part:?} along rather than {distance}",
    );

    let (document, first, _) = two_overlapping_additions();
    let shown = shown_with(&document, &[(first, VolumeState::Invisible)]);
    let (face, in_the_part, distance) =
        met_under_the_same_number(&document, &shown.body, (12.0, 5.0));
    assert!(
        in_the_part.is_some_and(|met| (met - distance).abs() < 1e-9),
        "the second block's top, the first invisible, answers to {face}, which in the part is \
         met at {in_the_part:?} along rather than {distance}",
    );
}
