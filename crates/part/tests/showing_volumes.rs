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
//! - found in review: a sketch laid on a face of a volume shown see-through
//!   or invisible keeps the plane it has in the part, not the plane it was
//!   recorded on — `a_sketch_on_a_volume_not_shown_opaque_keeps_the_plane_it_has_in_the_part`
//! - found in review: such a sketch takes the plane it has in the part even
//!   where its face is still shown, with another outline than in the part —
//!   `a_sketch_on_a_face_shown_with_another_outline_keeps_the_plane_it_has_in_the_part`;
//!   and so does a sketch on a kept section laid on a face of a volume left
//!   out — `a_sketch_on_a_section_kept_on_a_volume_left_out_keeps_the_plane_it_has_in_the_part`
//! - found in review: a revolution follows the same rules, an invisible
//!   turned cut giving back its matter —
//!   `an_invisible_revolved_cut_gives_back_the_matter_it_turned_away` — and a
//!   see-through turned addition its whole turn as a ghost —
//!   `a_see_through_revolved_addition_is_a_ghost_of_its_whole_turn`
//! - a face of the part shown answering to the number it has in the part —
//!   no test: picking now finds the part's face by where the ray meets it,
//!   held in `crates/app/src/screens/viewport/input/planes/tests.rs`, so the
//!   numbering of what is shown no longer matters

use std::collections::BTreeMap;

use std::f64::consts::PI;

use cao_part::history::{FaceAnchor, SectionAsked, SideDrawing};
use cao_part::{
    ExtrusionMode, Operation, PartDocument, PointRef, RevolutionAxis, Shown, VolumeState,
};
use cao_sketch::{SketchAxis, WorkPlane};
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

/// How tall the block of `a_boss_on_a_block_grown_since` was the day its boss
/// was sketched on its top, and so where the plane recorded with that sketch
/// sits.
const AS_SKETCHED: f64 = 10.0;

/// A block 40 by 20 raised 30 from XY, then a sketch on its top recorded at
/// the height the block had when it was laid, 10, and a boss 10 by 10 raised
/// 5 from it: in the part the sketch follows its face, and the boss stands
/// from 30 to 35. The block's number.
fn a_boss_on_a_block_grown_since() -> (PartDocument, u32) {
    let mut document = a_part();
    rectangle(&mut document, DVec2::ZERO, DVec2::new(40.0, 20.0));
    raised(
        &mut document,
        &[DVec2::new(20.0, 10.0)],
        30.0,
        ExtrusionMode::Add,
    );
    let block = number_of(&document, 2);
    document.apply(Operation::CreateSketch {
        plane: WorkPlane {
            origin: DVec3::new(0.0, 0.0, AS_SKETCHED),
            u: DVec3::X,
            v: DVec3::Y,
        },
        on: Some(FaceAnchor {
            face: 1,
            up: DVec3::Y,
        }),
    });
    document.apply(Operation::AddRectangle {
        sketch: 1,
        corner: PointRef::New(DVec2::new(10.0, 5.0)),
        opposite: PointRef::New(DVec2::new(20.0, 15.0)),
        construction: false,
    });
    document.apply(Operation::Extrude {
        sketch: 1,
        areas: document.areas_at(1, &[DVec2::new(15.0, 10.0)]),
        distance: 5.0.into(),
        mode: ExtrusionMode::Add,
    });
    assert_bounds(
        document.body(),
        DVec3::ZERO,
        DVec3::new(40.0, 20.0, 35.0),
        "the block with its boss on its top",
    );
    (document, block)
}

#[test]
fn a_sketch_on_a_volume_not_shown_opaque_keeps_the_plane_it_has_in_the_part() {
    let (document, block) = a_boss_on_a_block_grown_since();

    for state in [VolumeState::Invisible, VolumeState::SeeThrough] {
        let shown = shown_with(&document, &[(block, state)]);

        assert_bounds(
            &shown.body,
            DVec3::new(10.0, 5.0, 30.0),
            DVec3::new(20.0, 15.0, 35.0),
            &format!(
                "the boss, the block it stands on {state:?}, rather than raised from where \
                 its sketch was recorded,"
            ),
        );
    }
}

/// A rectangle 10 by 10 against the sketch's V axis turned a whole turn into
/// a cylinder of radius 10 and length 10, then a rectangle 4 by 6 inside it,
/// against the axis too, turned away: a bore of radius 4 and length 6. Its
/// turned addition and its turned cut.
fn a_bored_turn() -> (PartDocument, u32, u32) {
    let about_v = RevolutionAxis::Sketch(SketchAxis::V);
    let mut document = a_part();
    rectangle(&mut document, DVec2::ZERO, DVec2::splat(10.0));
    rectangle(&mut document, DVec2::new(0.0, 2.0), DVec2::new(4.0, 8.0));
    for (inside, mode) in [
        (
            vec![DVec2::new(7.0, 5.0), DVec2::new(2.0, 5.0)],
            ExtrusionMode::Add,
        ),
        (vec![DVec2::new(2.0, 5.0)], ExtrusionMode::Cut),
    ] {
        document.apply(Operation::Revolve {
            sketch: 0,
            areas: document.areas_at(0, &inside),
            axis: about_v,
            angle: 360.0.into(),
            mode,
        });
    }
    assert_volume(document.body(), (1000.0 - 96.0) * PI, "the bored turn");
    let turned = |step: usize| {
        assert!(
            matches!(
                document.history.applied_operations().get(step),
                Some(Operation::Revolve { .. })
            ),
            "the fixture wants a revolution at step {step}",
        );
        document.history.number_at(step).expect("a step there")
    };
    let (addition, cut) = (turned(3), turned(4));
    (document, addition, cut)
}

#[test]
fn an_invisible_revolved_cut_gives_back_the_matter_it_turned_away() {
    let (document, _, cut) = a_bored_turn();

    let shown = shown_with(&document, &[(cut, VolumeState::Invisible)]);

    assert_volume(&shown.body, 1000.0 * PI, "the turn, its bore invisible");
    assert!(
        shown.ghosts.is_empty(),
        "an invisible cut is shown as a ghost"
    );
}

#[test]
fn a_see_through_revolved_addition_is_a_ghost_of_its_whole_turn() {
    let (document, addition, _) = a_bored_turn();

    let shown = shown_with(&document, &[(addition, VolumeState::SeeThrough)]);

    assert!(
        shown.body.is_empty(),
        "the turn see-through, the opaque part still holds {}",
        shown.body.volume(),
    );
    let ghosts: Vec<u32> = shown.ghosts.iter().map(|ghost| ghost.step).collect();
    assert_eq!(ghosts, [addition], "the ghosts are those of other steps");
    assert_volume(
        &shown.ghosts[0].body,
        1000.0 * PI,
        "the turn's ghost, the whole cylinder it turned, bore and all",
    );
}

/// The span of what a body holds above `height`: every corner of a triangle
/// standing over it, which for a boss on a top is the boss's own top and
/// walls.
fn above(body: &Body, height: f64) -> Option<(DVec3, DVec3)> {
    let corners: Vec<DVec3> = body
        .triangles()
        .into_iter()
        .flatten()
        .filter(|corner| corner.z > height + 1e-9)
        .collect();
    let first = *corners.first()?;
    Some(corners.iter().fold((first, first), |(low, high), corner| {
        (low.min(*corner), high.max(*corner))
    }))
}

/// A block 10 by 10 by 10 notched through at its corner by a square from
/// (-5, -5) to (5, 5): its top an L, the corner (0, 0) gone. Then a sketch on
/// that top, read from one of the L's own corners, and a boss 2 by 2 raised 5
/// from it. The notch's number.
fn a_boss_on_a_corner_notched_top() -> (PartDocument, u32) {
    let mut document = a_part();
    rectangle(&mut document, DVec2::ZERO, DVec2::splat(10.0));
    rectangle(&mut document, DVec2::splat(-5.0), DVec2::splat(5.0));
    raised(
        &mut document,
        &[DVec2::splat(2.0), DVec2::splat(7.0)],
        10.0,
        ExtrusionMode::Add,
    );
    raised(
        &mut document,
        &[DVec2::splat(2.0), DVec2::splat(-2.0)],
        20.0,
        ExtrusionMode::Cut,
    );
    let notch = number_of(&document, 4);
    assert_volume(document.body(), 750.0, "the block notched at its corner");
    document.apply(Operation::CreateSketch {
        plane: WorkPlane {
            origin: DVec3::new(0.0, 0.0, 10.0),
            u: DVec3::X,
            v: DVec3::Y,
        },
        on: Some(FaceAnchor {
            face: 1,
            up: DVec3::Y,
        }),
    });
    document.apply(Operation::AddRectangle {
        sketch: 1,
        corner: PointRef::New(DVec2::splat(2.0)),
        opposite: PointRef::New(DVec2::splat(4.0)),
        construction: false,
    });
    document.apply(Operation::Extrude {
        sketch: 1,
        areas: document.areas_at(1, &[DVec2::splat(3.0)]),
        distance: 5.0.into(),
        mode: ExtrusionMode::Add,
    });
    (document, notch)
}

#[test]
fn a_sketch_on_a_face_shown_with_another_outline_keeps_the_plane_it_has_in_the_part() {
    let (document, notch) = a_boss_on_a_corner_notched_top();
    let in_the_part = above(document.body(), 10.0).expect("the boss stands on the top");

    for state in [VolumeState::Invisible, VolumeState::SeeThrough] {
        let shown = shown_with(&document, &[(notch, state)]);

        let boss = above(&shown.body, 10.0).expect("the boss is shown");
        assert!(
            boss.0.distance(in_the_part.0) < PLACE && boss.1.distance(in_the_part.1) < PLACE,
            "the notch {state:?}, the boss spans {} to {}, not {} to {} as in the part",
            boss.0,
            boss.1,
            in_the_part.0,
            in_the_part.1,
        );
    }
}

#[test]
fn a_sketch_on_a_section_kept_on_a_volume_left_out_keeps_the_plane_it_has_in_the_part() {
    let mut document = a_part();
    rectangle(&mut document, DVec2::ZERO, DVec2::new(40.0, 20.0));
    raised(
        &mut document,
        &[DVec2::new(20.0, 10.0)],
        30.0,
        ExtrusionMode::Add,
    );
    let block = number_of(&document, 2);
    let as_recorded = WorkPlane {
        origin: DVec3::new(0.0, 0.0, AS_SKETCHED),
        u: DVec3::X,
        v: DVec3::Y,
    };
    document.apply(Operation::SaveSection(SectionAsked {
        plane: as_recorded,
        on: Some(FaceAnchor {
            face: 1,
            up: DVec3::Y,
        }),
        offset: 5.0,
        behind: SideDrawing::Opaque,
        in_front: SideDrawing::Hidden,
        tilt_about_u_degrees: 0.0,
        tilt_about_v_degrees: 0.0,
        pivot: DVec3::ZERO,
    }));
    let section = document.sections()[0].id;
    document.apply(Operation::CreateSketchOnSection {
        section,
        plane: WorkPlane {
            origin: DVec3::new(0.0, 0.0, AS_SKETCHED + 5.0),
            ..as_recorded
        },
    });
    document.apply(Operation::AddRectangle {
        sketch: 1,
        corner: PointRef::New(DVec2::new(10.0, 5.0)),
        opposite: PointRef::New(DVec2::new(20.0, 15.0)),
        construction: false,
    });
    document.apply(Operation::Extrude {
        sketch: 1,
        areas: document.areas_at(1, &[DVec2::new(15.0, 10.0)]),
        distance: 5.0.into(),
        mode: ExtrusionMode::Add,
    });
    assert_bounds(
        document.body(),
        DVec3::ZERO,
        DVec3::new(40.0, 20.0, 40.0),
        "the block and the boss raised from the section kept 5 over its top",
    );

    for state in [VolumeState::Invisible, VolumeState::SeeThrough] {
        let shown = shown_with(&document, &[(block, state)]);

        assert_bounds(
            &shown.body,
            DVec3::new(10.0, 5.0, 35.0),
            DVec3::new(20.0, 15.0, 40.0),
            &format!(
                "the boss on the section, the block it was kept on {state:?}, rather than \
                 raised from where the section was recorded,"
            ),
        );
    }
}
