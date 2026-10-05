//! Closes #526.
//! - a profile of straight runs and arcs raised, joined and cut goes through
//!   the exact kernel, and a Ø40 raised has the arithmetic's volume —
//!   `a_circle_forty_across_raised_holds_the_arithmetics_volume`,
//!   `a_slot_cut_through_a_raised_rounded_rectangle_leaves_the_arithmetics_volume`,
//!   `a_hole_flush_but_for_a_hair_comes_out_closed`,
//!   `a_disc_joined_across_a_block_s_side_holds_the_arithmetics_volume`,
//!   `a_hole_drilled_right_through_a_hexagonal_bar_is_cut`
//! - once a revolution is in the history, the part is computed by the flats
//!   from that step on —
//!   `a_part_turned_once_is_computed_by_the_flats_from_that_step_on`,
//!   `undoing_the_turn_gives_the_part_back_to_the_exact_kernel`; an ellipse
//!   likewise — `a_part_with_an_ellipse_raised_is_computed_by_the_flats_from_that_step_on`
//! - an operation the exact kernel declines is a broken step —
//!   `a_raise_meeting_a_wall_at_a_slant_is_a_broken_step`,
//!   `a_step_declined_live_is_named_by_its_own_number`,
//!   `a_step_after_a_declined_one_numbers_its_faces_as_if_it_had_stood`,
//!   `a_size_that_would_have_the_kernel_decline_a_step_is_refused_naming_the_step`; a part
//!   opened on its cache still names it, held beside the cache by
//!   `a_declined_step_is_still_named_once_the_part_opens_on_its_cache`
//! - drawing, lighting under the cursor and picking read the exact kernel's
//!   triangles — `a_raised_circle_is_drawn_lit_and_picked_from_its_true_cylinder`
//! - a drawing laid on a face keeps its face when a size of the part changes —
//!   `a_drawing_on_the_top_of_a_disc_rides_it_when_it_grows`,
//!   `a_drawing_on_a_floor_a_cut_left_keeps_it_when_the_cut_deepens`,
//!   `a_drawing_on_a_top_two_blocks_share_keeps_it_when_one_grows`,
//!   `a_hole_drawn_on_a_boss_stays_on_it_when_the_boss_grows`,
//!   `a_hole_drawn_on_a_rounded_plate_moves_no_further_than_its_corners_when_they_change`,
//!   `a_drawing_on_a_top_a_trench_parts_stays_on_its_piece_when_the_trench_moves`
//! - the section view works on an exact body —
//!   `the_section_of_an_exact_part_keeps_only_what_lies_behind_the_plane`
//! - #498's eighteen cases and the harness's fast tests run in the gate, the
//!   long campaigns stay under `--ignored` — no test: held in `cao_solid`, by
//!   `a_bored_cylinder_on_two_kernels` and the fast tests of
//!   `random_exact_solids`, which the gate runs, and by the `#[ignore]` on every
//!   campaign; the campaigns are compiled only with `--features campaigns`,
//!   held by `the_long_campaigns_over_solids_are_compiled_only_when_asked_for`
//!   in `crates/app/tests/architecture.rs`
//! - `cao_solid` takes `glam` and `serde` only, no truck — no test: held by
//!   `the_two_geometry_crates_stay_alone_with_their_maths` in
//!   `crates/app/tests/architecture.rs`
//! - the geometry cache reads an exact body back, and `REBUILT_BY` moves — no
//!   test: held beside the cache, in `document/geometry_cache/tests.rs`, by
//!   `an_exact_body_comes_back_from_the_cache_as_the_replay_leaves_it`,
//!   `a_cache_rebuilt_before_the_exact_kernel_replays_its_design` and
//!   `a_cache_stamped_by_the_flats_rebuild_is_replayed_though_written_as_today`, since only
//!   there can a test tell a part opened on its cache from one replayed
//! - `docs/exact-kernel.md` lists what the kernel can do and what is left — no
//!   test: it is prose, held by `language.rs` and by nothing that asserts
//!
//! The part a history describes, computed by the exact kernel wherever it can
//! and by the flats from the first step it cannot: a revolution, or an area
//! bounded by an ellipse (`an_ellipse_encloses_an_area.rs`), which the exact
//! kernel has no surface for. The screen's side — the notice and the tree row
//! a declined step shows — is `cao_app`'s, tested beside its presenters.

use std::f64::consts::PI;

use cao_part::history::{ExtrusionMode, FaceAnchor, Operation, PointRef, RevolutionAxis};
use cao_part::{
    Broken, Formula, History, PartDocument, PartState, Refused, VariableChange, VariableId,
};
use cao_sketch::{Area, Corner, PointId, SegmentId, SketchAxis, WorkPlane};
use cao_solid::soundness;
use glam::{DVec2, DVec3};

/// How near the arithmetic an exact volume comes: the kernel's matter is the
/// true cylinder, so what is left is rounding.
const EXACT: f64 = 1e-9;

/// The areas these places fall in, as the drawing stands — what the
/// interface works out at the moment of the click.
fn clicked(history: &History, sketch: usize, place: DVec2) -> Vec<Area> {
    PartState::rebuild(history).areas_at(sketch, &[place])
}

fn sketch_on(history: &mut History, plane: WorkPlane) {
    history.push(Operation::CreateSketch { plane, on: None });
}

fn circle(history: &mut History, sketch: usize, center: DVec2, radius: f64) {
    history.push(Operation::AddCircle {
        sketch,
        center: PointRef::New(center),
        radius,
        rim: Vec::new(),
        construction: false,
    });
}

fn raise(history: &mut History, sketch: usize, place: DVec2, distance: f64, mode: ExtrusionMode) {
    history.push(Operation::Extrude {
        sketch,
        areas: clicked(history, sketch, place),
        distance: distance.into(),
        mode,
    });
}

fn assert_near(made: f64, expected: f64, what: &str) {
    assert!(
        (made - expected).abs() <= EXACT * expected,
        "{what}: {made} where the arithmetic gives {expected}",
    );
}

#[test]
fn a_circle_forty_across_raised_holds_the_arithmetics_volume() {
    let mut history = History::default();
    sketch_on(&mut history, WorkPlane::XY);
    circle(&mut history, 0, DVec2::ZERO, 20.0);
    raise(&mut history, 0, DVec2::ZERO, 10.0, ExtrusionMode::Add);

    let state = PartState::rebuild(&history);

    assert_near(state.body.volume(), PI * 400.0 * 10.0, "a Ø40 raised 10");
}

fn rectangle(history: &mut History, sketch: usize, corner: DVec2, opposite: DVec2) {
    history.push(Operation::AddRectangle {
        sketch,
        corner: PointRef::New(corner),
        opposite: PointRef::New(opposite),
        construction: false,
    });
}

/// A plane level with XY, `height` above it.
fn level(height: f64) -> WorkPlane {
    WorkPlane {
        origin: DVec3::Z * height,
        ..WorkPlane::XY
    }
}

/// Every triangle the part is drawn with closes against its neighbours and
/// crosses none of them.
fn assert_drawn_closed(state: &PartState) {
    let triangles = state.body.triangles();
    assert!(!triangles.is_empty(), "the part is drawn");
    soundness::closed(&triangles).expect("the part is drawn closed");
    soundness::uncrossed(&triangles).expect("the part is drawn uncrossed");
}

#[test]
fn a_slot_cut_through_a_raised_rounded_rectangle_leaves_the_arithmetics_volume() {
    let mut history = History::default();
    sketch_on(&mut history, WorkPlane::XY);
    rectangle(&mut history, 0, DVec2::ZERO, DVec2::new(40.0, 20.0));
    history.push(Operation::Fillet {
        sketch: 0,
        corners: (0..4)
            .map(|side| Corner::Between(SegmentId(side), SegmentId((side + 1) % 4)))
            .collect(),
        radius: 5.0.into(),
    });
    raise(
        &mut history,
        0,
        DVec2::new(20.0, 10.0),
        10.0,
        ExtrusionMode::Add,
    );
    sketch_on(&mut history, level(10.0));
    rectangle(
        &mut history,
        1,
        DVec2::new(10.0, 7.0),
        DVec2::new(30.0, 13.0),
    );
    raise(
        &mut history,
        1,
        DVec2::new(20.0, 10.0),
        -10.0,
        ExtrusionMode::Cut,
    );

    let state = PartState::rebuild(&history);

    let rounded = 40.0 * 20.0 - (4.0 - PI) * 25.0;
    let slot = 20.0 * 6.0;
    assert_near(
        state.body.volume(),
        (rounded - slot) * 10.0,
        "the slotted plate",
    );
    assert_drawn_closed(&state);
}

/// #489: a pocket whose side stands a ten-millionth inside the block's own
/// side. The flats cut the block's faces a hair from their edge and left the
/// strip beyond open.
#[test]
fn a_hole_flush_but_for_a_hair_comes_out_closed() {
    let hair = 1e-7;
    let mut history = History::default();
    sketch_on(&mut history, WorkPlane::XY);
    rectangle(&mut history, 0, DVec2::ZERO, DVec2::new(10.0, 10.0));
    raise(
        &mut history,
        0,
        DVec2::new(5.0, 5.0),
        5.0,
        ExtrusionMode::Add,
    );
    sketch_on(&mut history, level(5.0));
    rectangle(&mut history, 1, DVec2::new(hair, 2.0), DVec2::new(4.0, 8.0));
    raise(
        &mut history,
        1,
        DVec2::new(2.0, 5.0),
        -3.0,
        ExtrusionMode::Cut,
    );

    let state = PartState::rebuild(&history);

    assert_drawn_closed(&state);
    let pocket = (4.0 - hair) * 6.0 * 3.0;
    assert!(
        (state.body.volume() - (500.0 - pocket)).abs() < 1e-6,
        "the block less its pocket: {}",
        state.body.volume(),
    );
}

/// What the flats make of a circle: the polygon of 48 sides inscribed in it.
fn inscribed(radius: f64) -> f64 {
    let sides = 48.0;
    sides / 2.0 * radius * radius * (2.0 * PI / sides).sin()
}

/// A ring turned about the sketch's own V axis, far from anything at the
/// origin: matter only the flats can make.
fn turn_a_ring(history: &mut History, sketch: usize) {
    sketch_on(history, WorkPlane::XY);
    rectangle(
        history,
        sketch,
        DVec2::new(50.0, 0.0),
        DVec2::new(60.0, 10.0),
    );
    history.push(Operation::Revolve {
        sketch,
        areas: clicked(history, sketch, DVec2::new(55.0, 5.0)),
        axis: RevolutionAxis::Sketch(SketchAxis::V),
        angle: 360.0.into(),
        mode: ExtrusionMode::Add,
    });
}

#[test]
fn a_part_turned_once_is_computed_by_the_flats_from_that_step_on() {
    let mut history = History::default();
    sketch_on(&mut history, WorkPlane::XY);
    circle(&mut history, 0, DVec2::ZERO, 20.0);
    raise(&mut history, 0, DVec2::ZERO, 10.0, ExtrusionMode::Add);
    let before_the_turn = PartState::rebuild(&history).body.volume();
    turn_a_ring(&mut history, 1);
    let turned = PartState::rebuild(&history).body.volume();
    sketch_on(&mut history, WorkPlane::XY);
    circle(&mut history, 2, DVec2::new(0.0, 100.0), 20.0);
    raise(
        &mut history,
        2,
        DVec2::new(0.0, 100.0),
        10.0,
        ExtrusionMode::Add,
    );
    let after_the_turn = PartState::rebuild(&history).body.volume();

    assert_near(
        before_the_turn,
        PI * 400.0 * 10.0,
        "the Ø40 raised before the turn",
    );
    assert_near(
        after_the_turn - turned,
        inscribed(20.0) * 10.0,
        "the Ø40 raised after the turn, by the flats",
    );
}

#[test]
fn undoing_the_turn_gives_the_part_back_to_the_exact_kernel() {
    let mut history = History::default();
    sketch_on(&mut history, WorkPlane::XY);
    circle(&mut history, 0, DVec2::ZERO, 20.0);
    raise(&mut history, 0, DVec2::ZERO, 10.0, ExtrusionMode::Add);
    turn_a_ring(&mut history, 1);
    history.undo();
    sketch_on(&mut history, WorkPlane::XY);
    circle(&mut history, 2, DVec2::new(0.0, 100.0), 20.0);
    raise(
        &mut history,
        2,
        DVec2::new(0.0, 100.0),
        10.0,
        ExtrusionMode::Add,
    );

    let state = PartState::rebuild(&history);

    assert_near(
        state.body.volume(),
        2.0 * PI * 400.0 * 10.0,
        "two Ø40 raised, the turn between them undone",
    );
}

/// A block 10 on a side, then a circle drawn on a plane tilted 30° about X
/// and standing at `at`, raised 10 down its normal. Over the block's top, the
/// cylinder meets that top at a slant, along an ellipse the exact kernel does
/// not draw.
fn a_slanted_raise_beside_a_block(at: DVec3) -> History {
    let mut history = History::default();
    sketch_on(&mut history, WorkPlane::XY);
    rectangle(&mut history, 0, DVec2::ZERO, DVec2::splat(10.0));
    raise(&mut history, 0, DVec2::splat(5.0), 10.0, ExtrusionMode::Add);
    let (sin, cos) = (PI / 6.0).sin_cos();
    let slanted = WorkPlane {
        origin: at,
        u: DVec3::X,
        v: DVec3::new(0.0, cos, sin),
    };
    sketch_on(&mut history, slanted);
    circle(&mut history, 1, DVec2::ZERO, 2.0);
    raise(&mut history, 1, DVec2::ZERO, -10.0, ExtrusionMode::Add);
    history
}

const OVER_THE_BLOCK: DVec3 = DVec3::new(5.0, 5.0, 12.0);

/// The number the history gave its last step.
fn last_step(history: &History) -> u32 {
    history.steps().last().expect("a step").operations()[0]
}

/// A part made by applying each operation of `history` as the user would.
fn applied_live(history: &History) -> PartDocument {
    let now = "2026-10-05T09:00:00Z".parse().expect("a date");
    let mut part = PartDocument::new("Exact", now);
    for operation in history.operations() {
        part.apply(operation.clone());
    }
    part
}

#[test]
fn a_raise_meeting_a_wall_at_a_slant_is_a_broken_step() {
    let history = a_slanted_raise_beside_a_block(OVER_THE_BLOCK);
    let mut part = applied_live(&history);
    part.undo();
    part.redo();

    assert!(
        part.is_declined(last_step(&history)),
        "the slanted raise is declined"
    );
    assert_near(
        part.body().volume(),
        1000.0,
        "the block, as it stood before",
    );
}

#[test]
fn a_step_declined_live_is_named_by_its_own_number() {
    let history = a_slanted_raise_beside_a_block(OVER_THE_BLOCK);

    let part = applied_live(&history);

    assert!(
        part.is_declined(last_step(&history)),
        "a step declined as it is applied is named by the number it was given",
    );
    assert!(!part.is_declined(0), "and not by nought");
}

#[test]
fn a_step_after_a_declined_one_numbers_its_faces_as_if_it_had_stood() {
    let with_a_plate_after = |at: DVec3| {
        let mut history = a_slanted_raise_beside_a_block(at);
        sketch_on(&mut history, WorkPlane::XY);
        rectangle(
            &mut history,
            2,
            DVec2::new(20.0, 0.0),
            DVec2::new(30.0, 10.0),
        );
        raise(
            &mut history,
            2,
            DVec2::new(25.0, 5.0),
            3.0,
            ExtrusionMode::Add,
        );
        history
    };
    // The block's six faces, the disc's floor, top and wall, then the
    // plate's floor and top.
    let plates_top = 6 + 3 + TOP;
    let declined = with_a_plate_after(OVER_THE_BLOCK);
    let raised = with_a_plate_after(DVec3::new(100.0, 5.0, 12.0));

    assert_eq!(laid_on(declined, plates_top, 3.0), (3.0, false));
    assert_eq!(laid_on(raised, plates_top, 3.0), (3.0, false));
}

/// A Ø40 raised 10 from XY.
fn a_disc() -> PartState {
    let mut history = History::default();
    sketch_on(&mut history, WorkPlane::XY);
    circle(&mut history, 0, DVec2::ZERO, 20.0);
    raise(&mut history, 0, DVec2::ZERO, 10.0, ExtrusionMode::Add);
    PartState::rebuild(&history)
}

#[test]
fn a_raised_circle_is_drawn_lit_and_picked_from_its_true_cylinder() {
    let state = a_disc();
    let drawn = state.body.triangles();
    let toward = DVec3::new(0.37f64.cos(), 0.37f64.sin(), 0.0);

    let hit = state
        .body
        .ray_hit(toward * 40.0 + DVec3::Z * 5.0, -toward)
        .expect("the ray meets the wall");
    let lit: Vec<[DVec3; 3]> = state.body.triangles_of(hit.face).collect();
    let by_face: usize = (0..state.body.faces_end())
        .map(|face| state.body.triangles_of(face).count())
        .sum();

    assert!(
        hit.normal.distance(toward) < 1e-9,
        "the wall faces the way the cylinder does where the ray met it, {} \
         rather than the way a flat piece of it would",
        hit.normal,
    );
    assert!(!state.body.is_flat(hit.face), "the wall is one curved face");
    assert!(
        lit.iter().all(|triangle| drawn.contains(triangle)),
        "the face lit under the cursor is lit with the triangles the part is \
         drawn with",
    );
    assert_eq!(
        by_face,
        drawn.len(),
        "every triangle drawn belongs to a face"
    );
}

#[test]
fn the_section_of_an_exact_part_keeps_only_what_lies_behind_the_plane() {
    let state = a_disc();

    let behind = state.body.behind(DVec3::X, 5.0);

    let (low, high) = behind.bounds().expect("something is left");
    assert!(high.x <= 5.0 + 1e-9, "nothing past the plane: {high}");
    assert!(
        (low.x + 20.0).abs() < 1e-9,
        "the far side of the disc is left: {low}"
    );
    assert!(
        behind
            .ray_hit(DVec3::new(-10.0, 0.0, 5.0), DVec3::X)
            .is_none(),
        "the section is open where the plane went through",
    );
}

/// A sketch laid on face `face` of the part `history` holds, its plane read
/// back once the part is replayed: the height it stands at, and whether it
/// lost its face.
fn laid_on(mut history: History, face: usize, as_drawn: f64) -> (f64, bool) {
    let sketch = PartState::rebuild(&history).sketches.len();
    history.push(Operation::CreateSketch {
        plane: level(as_drawn),
        on: Some(FaceAnchor { face, up: DVec3::Y }),
    });
    let state = PartState::rebuild(&history);
    (
        state.sketches[sketch].plane.origin.z,
        state.adrift.contains(&sketch),
    )
}

/// The faces of the first raise of a part: its floor, its top, then its walls.
const TOP: usize = 1;

#[test]
fn a_drawing_on_the_top_of_a_disc_rides_it_when_it_grows() {
    let disc = |height: f64| {
        let mut history = History::default();
        sketch_on(&mut history, WorkPlane::XY);
        circle(&mut history, 0, DVec2::ZERO, 20.0);
        raise(&mut history, 0, DVec2::ZERO, height, ExtrusionMode::Add);
        history
    };

    assert_eq!(laid_on(disc(10.0), TOP, 10.0), (10.0, false));
    assert_eq!(laid_on(disc(30.0), TOP, 10.0), (30.0, false));
}

#[test]
fn a_drawing_on_a_floor_a_cut_left_keeps_it_when_the_cut_deepens() {
    let pocket = |depth: f64| {
        let mut history = History::default();
        sketch_on(&mut history, WorkPlane::XY);
        rectangle(&mut history, 0, DVec2::ZERO, DVec2::new(40.0, 20.0));
        raise(
            &mut history,
            0,
            DVec2::new(20.0, 10.0),
            20.0,
            ExtrusionMode::Add,
        );
        sketch_on(&mut history, level(20.0));
        rectangle(
            &mut history,
            1,
            DVec2::new(10.0, 5.0),
            DVec2::new(30.0, 15.0),
        );
        raise(
            &mut history,
            1,
            DVec2::new(20.0, 10.0),
            -depth,
            ExtrusionMode::Cut,
        );
        history
    };
    // The block's floor, top and four walls, then the tool's: its first cap
    // on the sketch plane, the second where it ends, the pocket's floor.
    let floor = 6 + TOP;

    assert_eq!(laid_on(pocket(5.0), floor, 15.0), (15.0, false));
    assert_eq!(laid_on(pocket(12.0), floor, 15.0), (8.0, false));
}

#[test]
fn a_drawing_on_a_top_two_blocks_share_keeps_it_when_one_grows() {
    let blocks = |first: f64, second: f64| {
        let mut history = History::default();
        sketch_on(&mut history, WorkPlane::XY);
        rectangle(&mut history, 0, DVec2::ZERO, DVec2::new(20.0, 20.0));
        raise(
            &mut history,
            0,
            DVec2::splat(10.0),
            first,
            ExtrusionMode::Add,
        );
        sketch_on(&mut history, WorkPlane::XY);
        rectangle(
            &mut history,
            1,
            DVec2::new(20.0, 0.0),
            DVec2::new(40.0, 20.0),
        );
        raise(
            &mut history,
            1,
            DVec2::new(30.0, 10.0),
            second,
            ExtrusionMode::Add,
        );
        history
    };
    let second_top = 6 + TOP;

    assert_eq!(laid_on(blocks(10.0, 10.0), second_top, 10.0), (10.0, false));
    assert_eq!(laid_on(blocks(10.0, 15.0), second_top, 10.0), (15.0, false));
    assert_eq!(laid_on(blocks(15.0, 10.0), second_top, 10.0), (10.0, false));
}

/// A closed outline through `corners`, each segment starting where the last
/// one ended.
fn polygon(history: &mut History, sketch: usize, corners: &[DVec2]) {
    let points = |history: &History| PartState::rebuild(history).sketches[sketch].points().len();
    let first = points(history);
    let mut from = PointRef::New(corners[0]);
    for &corner in &corners[1..] {
        history.push(Operation::AddSegment {
            sketch,
            start: from,
            end: PointRef::New(corner),
            construction: false,
        });
        from = PointRef::Existing(PointId(points(history) - 1));
    }
    history.push(Operation::AddSegment {
        sketch,
        start: from,
        end: PointRef::Existing(PointId(first)),
        construction: false,
    });
}

/// A hexagonal bar, corners 20 from its axis, raised 10, and a hole Ø4 drilled
/// `depth` deep square into the side between its corners at 0° and 60°,
/// half way up.
fn a_hexagonal_bar_drilled(depth: f64) -> History {
    let mut history = History::default();
    sketch_on(&mut history, WorkPlane::XY);
    let corners: Vec<DVec2> = (0..6)
        .map(|k| DVec2::from_angle(k as f64 * PI / 3.0) * 20.0)
        .collect();
    polygon(&mut history, 0, &corners);
    raise(&mut history, 0, DVec2::ZERO, 10.0, ExtrusionMode::Add);
    let middle = DVec2::from_angle(PI / 6.0) * 20.0 * (PI / 6.0).cos();
    let along = DVec2::from_angle(PI / 6.0).perp();
    sketch_on(
        &mut history,
        WorkPlane {
            origin: middle.extend(0.0),
            u: along.extend(0.0),
            v: DVec3::Z,
        },
    );
    circle(&mut history, 1, DVec2::new(0.0, 5.0), 2.0);
    raise(
        &mut history,
        1,
        DVec2::new(0.0, 5.0),
        -depth,
        ExtrusionMode::Cut,
    );
    history
}

#[test]
fn a_hole_drilled_right_through_a_hexagonal_bar_is_cut() {
    let across_the_flats = 2.0 * 20.0 * (PI / 6.0).cos();
    let hexagon = 3.0 * 3f64.sqrt() / 2.0 * 400.0 * 10.0;
    for depth in [30.0, 45.0, 50.0, 60.0] {
        let history = a_hexagonal_bar_drilled(depth);

        let part = applied_live(&history);

        assert!(
            !part.is_declined(last_step(&history)),
            "a hole {depth} deep touches no slanted side, and is cut",
        );
        let bored = depth.min(across_the_flats);
        assert_near(
            part.body().volume(),
            hexagon - PI * 4.0 * bored,
            &format!("the bar drilled {depth} deep"),
        );
    }
}

/// A sketch laid on face `face`, whose plane stands at `height`, with a circle
/// of radius 1 drawn at `place` of it — or, when `place` is `None`, at the
/// point of it above `over` — cut 3 deep. Gives back where the circle was
/// drawn in the sketch and where its centre stands once the part is replayed.
fn a_hole_drawn_on(
    mut history: History,
    face: usize,
    height: f64,
    over: DVec2,
    place: Option<DVec2>,
) -> (History, DVec2, DVec2) {
    let sketch = PartState::rebuild(&history).sketches.len();
    history.push(Operation::CreateSketch {
        plane: level(height),
        on: Some(FaceAnchor { face, up: DVec3::Y }),
    });
    let laid = PartState::rebuild(&history).sketches[sketch].plane;
    let place = place.unwrap_or_else(|| laid.to_local(over.extend(height)));
    circle(&mut history, sketch, place, 1.0);
    raise(&mut history, sketch, place, -3.0, ExtrusionMode::Cut);
    let at = PartState::rebuild(&history).sketches[sketch]
        .plane
        .to_world(place)
        .truncate();
    (history, place, at)
}

/// A block 40 by 30 by 10, and on its top a boss of radius `radius` about
/// `BOSS`, raised 5.
fn a_block_with_a_boss(radius: f64) -> History {
    let mut history = History::default();
    sketch_on(&mut history, WorkPlane::XY);
    rectangle(&mut history, 0, DVec2::ZERO, DVec2::new(40.0, 30.0));
    raise(&mut history, 0, DVec2::ONE, 10.0, ExtrusionMode::Add);
    sketch_on(&mut history, level(10.0));
    circle(&mut history, 1, BOSS, radius);
    raise(&mut history, 1, BOSS, 5.0, ExtrusionMode::Add);
    history
}

const BOSS: DVec2 = DVec2::new(20.3, 13.7);

#[test]
fn a_hole_drawn_on_a_boss_stays_on_it_when_the_boss_grows() {
    let boss_top = 6 + TOP;
    let (_, drawn, _) = a_hole_drawn_on(a_block_with_a_boss(5.0), boss_top, 15.0, BOSS, None);

    for radius in [5.0, 6.0, 7.0, 8.0, 9.7] {
        let (history, _, at) = a_hole_drawn_on(
            a_block_with_a_boss(radius),
            boss_top,
            15.0,
            BOSS,
            Some(drawn),
        );
        let state = PartState::rebuild(&history);

        assert!(
            at.distance(BOSS) < radius - 1.0,
            "the hole stays on a boss of radius {radius}: drawn at {at}",
        );
        assert_near(
            state.body.volume(),
            40.0 * 30.0 * 10.0 + PI * radius * radius * 5.0 - PI * 3.0,
            &format!("the block, a boss of radius {radius} and its hole"),
        );
    }
}

/// A plate 60 by 30, its four corners rounded by `round`, raised 8.
fn a_rounded_plate(round: f64) -> History {
    let mut history = History::default();
    sketch_on(&mut history, WorkPlane::XY);
    rectangle(
        &mut history,
        0,
        DVec2::new(3.3, 1.7),
        DVec2::new(63.3, 31.7),
    );
    history.push(Operation::Fillet {
        sketch: 0,
        corners: (0..4)
            .map(|side| Corner::Between(SegmentId(side), SegmentId((side + 1) % 4)))
            .collect(),
        radius: round.into(),
    });
    raise(
        &mut history,
        0,
        DVec2::new(30.0, 15.0),
        8.0,
        ExtrusionMode::Add,
    );
    history
}

#[test]
fn a_hole_drawn_on_a_rounded_plate_moves_no_further_than_its_corners_when_they_change() {
    let near_a_corner = DVec2::new(13.3, 11.7);
    let (_, drawn, before) = a_hole_drawn_on(a_rounded_plate(5.0), TOP, 8.0, near_a_corner, None);

    for round in [4.4, 4.0, 6.0] {
        let (_, _, after) =
            a_hole_drawn_on(a_rounded_plate(round), TOP, 8.0, near_a_corner, Some(drawn));

        assert!(
            after.distance(before) <= (round - 5.0f64).abs(),
            "rounded by {round} rather than 5, the hole went from {before} to {after}",
        );
    }
}

/// A block 10 on a side, then a circle on a plane tilted 30° over it, raised
/// down its normal by the variable `reach`, first 1: short of the block.
fn a_slanted_raise_reaching(reach: f64) -> PartDocument {
    let mut part = applied_live(&{
        let mut history = History::default();
        sketch_on(&mut history, WorkPlane::XY);
        rectangle(&mut history, 0, DVec2::ZERO, DVec2::splat(10.0));
        raise(&mut history, 0, DVec2::splat(5.0), 10.0, ExtrusionMode::Add);
        history
    });
    part.change_variable(VariableChange::Added {
        name: "reach".to_string(),
        formula: Formula::Number(reach),
    })
    .expect("a first variable");
    let (sin, cos) = (PI / 6.0).sin_cos();
    part.apply(Operation::CreateSketch {
        plane: WorkPlane {
            origin: OVER_THE_BLOCK,
            u: DVec3::X,
            v: DVec3::new(0.0, cos, sin),
        },
        on: None,
    });
    part.apply(Operation::AddCircle {
        sketch: 1,
        center: PointRef::New(DVec2::ZERO),
        radius: 2.0,
        rim: Vec::new(),
        construction: false,
    });
    let areas = part.areas_at(1, &[DVec2::ZERO]);
    let reach = part
        .variables()
        .read("-reach")
        .expect("a formula that reads");
    part.apply(Operation::Extrude {
        sketch: 1,
        areas,
        distance: reach,
        mode: ExtrusionMode::Add,
    });
    part
}

#[test]
fn a_size_that_would_have_the_kernel_decline_a_step_is_refused_naming_the_step() {
    let mut part = a_slanted_raise_reaching(1.0);
    let slanted = part.history.steps().last().expect("a step").operations()[0];
    assert!(
        !part.is_declined(slanted),
        "short of the block, it is built"
    );
    let before = part.body().volume();

    let refused = part.change_variable(VariableChange::Edited {
        variable: VariableId(0),
        name: "reach".to_string(),
        formula: Formula::Number(10.0),
    });

    assert_eq!(
        refused,
        Err(Refused::Breaks(vec![Broken::Operation(slanted)])),
        "as any operation that cannot be done",
    );
    assert_eq!(part.body().volume(), before, "nothing moved");
}

#[test]
fn a_disc_joined_across_a_block_s_side_holds_the_arithmetics_volume() {
    let mut history = History::default();
    sketch_on(&mut history, WorkPlane::XY);
    rectangle(&mut history, 0, DVec2::ZERO, DVec2::splat(20.0));
    raise(
        &mut history,
        0,
        DVec2::splat(10.0),
        10.0,
        ExtrusionMode::Add,
    );
    sketch_on(&mut history, WorkPlane::XY);
    circle(&mut history, 1, DVec2::new(20.0, 10.0), 8.0);
    raise(
        &mut history,
        1,
        DVec2::new(25.0, 10.0),
        15.0,
        ExtrusionMode::Add,
    );

    let state = PartState::rebuild(&history);

    let half_disc = PI * 64.0 / 2.0;
    assert_near(
        state.body.volume(),
        400.0 * 10.0 + half_disc * 5.0 + half_disc * 15.0,
        "the block, and the disc standing half in it and above it",
    );
    assert_drawn_closed(&state);
}

#[test]
fn a_part_with_an_ellipse_raised_is_computed_by_the_flats_from_that_step_on() {
    let mut history = History::default();
    sketch_on(&mut history, WorkPlane::XY);
    history.push(Operation::AddEllipse {
        sketch: 0,
        center: PointRef::New(DVec2::new(50.0, 20.0)),
        first: [
            PointRef::New(DVec2::new(20.0, 20.0)),
            PointRef::New(DVec2::new(80.0, 20.0)),
        ],
        second: [
            PointRef::New(DVec2::new(50.0, 0.0)),
            PointRef::New(DVec2::new(50.0, 40.0)),
        ],
        construction: false,
        drawn: None,
    });
    raise(
        &mut history,
        0,
        DVec2::new(50.0, 20.0),
        5.0,
        ExtrusionMode::Add,
    );
    let oval = PartState::rebuild(&history).body.volume();
    sketch_on(&mut history, WorkPlane::XY);
    circle(&mut history, 1, DVec2::new(0.0, 100.0), 20.0);
    raise(
        &mut history,
        1,
        DVec2::new(0.0, 100.0),
        10.0,
        ExtrusionMode::Add,
    );
    let after_the_oval = PartState::rebuild(&history).body.volume();

    assert_near(
        after_the_oval - oval,
        inscribed(20.0) * 10.0,
        "the Ø40 raised after the ellipse, by the flats",
    );
}

/// A block 30 by 10 by 10, a trench `width` wide from `from` cut `depth`
/// right across its top, and a sketch laid on what is left of the top:
/// where its plane stands once the part is replayed, and whether it lost its
/// face.
fn laid_on_a_top_a_trench_parts(from: f64, width: f64, depth: f64) -> (DVec3, bool) {
    let mut history = History::default();
    sketch_on(&mut history, WorkPlane::XY);
    rectangle(&mut history, 0, DVec2::ZERO, DVec2::new(30.0, 10.0));
    raise(&mut history, 0, DVec2::ONE, 10.0, ExtrusionMode::Add);
    sketch_on(&mut history, level(10.0));
    rectangle(
        &mut history,
        1,
        DVec2::new(from, -1.0),
        DVec2::new(from + width, 11.0),
    );
    raise(
        &mut history,
        1,
        DVec2::new(from + width / 2.0, 5.0),
        -depth,
        ExtrusionMode::Cut,
    );
    history.push(Operation::CreateSketch {
        plane: level(10.0),
        on: Some(FaceAnchor {
            face: TOP,
            up: DVec3::Y,
        }),
    });
    let state = PartState::rebuild(&history);
    (state.sketches[2].plane.origin, state.adrift.contains(&2))
}

#[test]
fn a_drawing_on_a_top_a_trench_parts_stays_on_its_piece_when_the_trench_moves() {
    for (from, width, depth) in [
        (10.0, 10.0, 5.0),
        (12.0, 10.0, 5.0),
        (2.0, 3.0, 5.0),
        (20.0, 5.0, 3.0),
    ] {
        assert_eq!(
            laid_on_a_top_a_trench_parts(from, width, depth),
            (DVec3::Z * 10.0, false),
            "a trench {width} wide from {from}: the drawing stays on the piece \
             the top's number stayed with",
        );
    }
}
