//! Closes #533.
//! - a square turned about one of its sides holds the arithmetic's volume, exactly,
//!   on the exact kernel, and so do a ring, a stepped shaft, a partial turn and a
//!   groove turned into a shaft —
//!   `a_square_turned_about_one_of_its_sides_holds_the_arithmetics_volume` (1000π),
//!   `a_ring_turned_holds_the_arithmetics_volume`,
//!   `a_stepped_shaft_turned_holds_the_arithmetics_volume` (2500π),
//!   `a_partial_turn_either_way_holds_the_arithmetics_volume` ([2,5]×[0,4] at ±90°: 21π),
//!   `a_groove_turned_into_a_shaft_leaves_the_arithmetics_volume` (3820π)
//! - a prism raised after a straight revolution is still computed by the exact kernel —
//!   `a_prism_raised_after_a_straight_turn_is_computed_by_the_exact_kernel`,
//!   `a_shaft_drawn_square_by_rules_stays_on_the_exact_kernel`
//! - a drawing laid on a face of a revolution keeps its face when a size changes —
//!   `a_drawing_on_a_shoulder_of_a_turned_shaft_rides_it_when_the_shoulder_moves`,
//!   `a_drawing_on_the_closing_end_of_a_partial_turn_follows_it_when_the_turn_widens`,
//!   `faces_after_a_full_turn_are_numbered_as_on_main`
//! - a profile with an arc sends the part to the flats from that step on (a slanted
//!   run did too, until #536) —
//!   `a_part_turned_from_an_arc_is_computed_by_the_flats_from_that_step_on`
//! - a revolution the exact kernel declines is a broken step, with its reason kept —
//!   `a_turn_crossing_a_raised_cylinder_at_a_skew_angle_is_a_broken_step_with_its_reason`
//!   (`declined_because(step) == Some(Declined::Unsupported)`, volume unchanged),
//!   `a_step_after_a_declined_turn_numbers_its_faces_as_if_it_had_stood`; a part opened
//!   on its cache keeps it, held beside the cache by
//!   `a_declined_step_keeps_its_reason_once_the_part_opens_on_its_cache`
//! - a profile a hair off the axis is turned as if on it; an area crossing the axis is
//!   turned on both sides and joined —
//!   `a_square_a_hair_across_the_axis_is_turned_as_if_drawn_on_it`,
//!   `a_square_a_hair_off_the_axis_on_its_own_side_is_turned_as_if_drawn_on_it`,
//!   `a_profile_whose_side_was_laid_on_the_axis_turns_into_a_closed_solid` (#488, flats),
//!   `a_cone_whose_leg_was_laid_on_the_axis_comes_out_closed` (#488; exact since #536),
//!   `an_area_across_a_sketch_axis_is_turned_on_both_sides_and_joined` (50π),
//!   `an_area_across_the_axis_turned_part_way_holds_both_sides_volumes` (90°: 20.5π),
//!   `an_area_across_a_construction_line_is_turned_on_both_sides_and_joined`,
//!   `a_circle_across_the_axis_turned_whole_is_turned_from_its_larger_side_alone`
//! - #448's harness draws revolutions, written before the kernel code; a campaign is
//!   run, its failures are named, its fast cases are in the gate, the long ones
//!   behind `--features campaigns` — no test: held in `cao_solid` by
//!   `random_turned_solids.rs` (its draw tests and the kernel gate tests the kernel's
//!   commit un-ignored), its campaigns under `#[ignore]` below `#[cfg(feature =
//!   "campaigns")]`, held by `the_long_campaigns_over_solids_are_compiled_only_when_asked_for`.
//!   The turned campaigns ran four hours each on 6 October, beside the square, profile
//!   and part campaigns run on main and the branch over the same seeds; their failures
//!   are filed by family in `docs/exact-kernel-failures.md` ("#533: turns"), one seed of
//!   each named in `what_the_exact_campaigns_found_in_the_kernel.rs` / `…_triangles.rs`,
//!   and the parts in `an_undo_gives_back_the_part.rs`
//! - #498's eighteen cases still hold — no test: held by
//!   `crates/solid/tests/a_bored_cylinder_on_two_kernels.rs`, which the gate runs
//! - `docs/exact-kernel.md` says what the kernel turns, and what it still hands the
//!   flats — no test: it is prose, held by `language.rs` and by nothing that asserts
//!
//! A part turned from a profile of straight runs, each parallel, square or
//! slanted to its axis, as the application builds it: the exact kernel turns
//! it, and the part stays exact after it. An arc still goes to the flats. The
//! kernel's own tests are `cao_solid`'s, in `the_exact_kernel_turns.rs`.

use std::f64::consts::PI;
use std::sync::mpsc;
use std::time::Duration;

use cao_part::history::{ExtrusionMode, FaceAnchor, Operation, PointRef, RevolutionAxis};
use cao_part::{History, PartDocument, PartState, VariableChange, VariableId};
use cao_sketch::{Area, Constraint, DimensionTarget, PointId, SegmentId, SketchAxis, WorkPlane};
use cao_solid::Declined;
use cao_solid::soundness::closed;
use glam::{DVec2, DVec3};

/// How near the arithmetic an exact volume comes: the kernel's matter is the
/// true cylinder, so what is left is rounding.
const EXACT: f64 = 1e-9;

fn clicked(history: &History, sketch: usize, place: DVec2) -> Vec<Area> {
    PartState::rebuild(history).areas_at(sketch, &[place])
}

fn sketch_on(history: &mut History, plane: WorkPlane) {
    history.push(Operation::CreateSketch { plane, on: None });
}

fn rectangle(history: &mut History, sketch: usize, corner: DVec2, opposite: DVec2) {
    history.push(Operation::AddRectangle {
        sketch,
        corner: PointRef::New(corner),
        opposite: PointRef::New(opposite),
        construction: false,
    });
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

fn raise(history: &mut History, sketch: usize, place: DVec2, distance: f64, mode: ExtrusionMode) {
    history.push(Operation::Extrude {
        sketch,
        areas: clicked(history, sketch, place),
        distance: distance.into(),
        mode,
    });
}

fn turn(
    history: &mut History,
    sketch: usize,
    place: DVec2,
    axis: RevolutionAxis,
    degrees: f64,
    mode: ExtrusionMode,
) {
    history.push(Operation::Revolve {
        sketch,
        areas: clicked(history, sketch, place),
        axis,
        angle: degrees.into(),
        mode,
    });
}

const ABOUT_V: RevolutionAxis = RevolutionAxis::Sketch(SketchAxis::V);

fn assert_near(made: f64, expected: f64, what: &str) {
    assert!(
        (made - expected).abs() <= EXACT * expected,
        "{what}: {made} where the arithmetic gives {expected}",
    );
}

/// The part `history` describes, held to have been computed by the exact
/// kernel throughout.
fn exact(history: &History) -> PartState {
    let state = PartState::rebuild(history);
    assert!(state.body.is_exact(), "the part stays on the exact kernel");
    state
}

/// A profile drawn on XY and turned about V by `degrees`.
fn turned_on_xy(draw: impl FnOnce(&mut History), place: DVec2, degrees: f64) -> History {
    let mut history = History::default();
    sketch_on(&mut history, WorkPlane::XY);
    draw(&mut history);
    turn(&mut history, 0, place, ABOUT_V, degrees, ExtrusionMode::Add);
    history
}

#[test]
fn a_square_turned_about_one_of_its_sides_holds_the_arithmetics_volume() {
    let history = turned_on_xy(
        |history| rectangle(history, 0, DVec2::ZERO, DVec2::splat(10.0)),
        DVec2::splat(5.0),
        360.0,
    );

    assert_near(
        exact(&history).body.volume(),
        1000.0 * PI,
        "a square 10 on a side turned about its side",
    );
}

#[test]
fn a_ring_turned_holds_the_arithmetics_volume() {
    let history = turned_on_xy(
        |history| rectangle(history, 0, DVec2::new(3.0, 0.0), DVec2::new(5.0, 2.0)),
        DVec2::new(4.0, 1.0),
        360.0,
    );

    assert_near(
        exact(&history).body.volume(),
        PI * (25.0 - 9.0) * 2.0,
        "a ring from 3 to 5, 2 high",
    );
}

/// A shaft Ø20 for 20, then Ø10 up to `top`, its shoulder at `shoulder`.
fn a_stepped_shaft(shoulder: f64, top: f64) -> History {
    turned_on_xy(
        |history| {
            polygon(
                history,
                0,
                &[
                    DVec2::new(0.0, 0.0),
                    DVec2::new(10.0, 0.0),
                    DVec2::new(10.0, shoulder),
                    DVec2::new(5.0, shoulder),
                    DVec2::new(5.0, top),
                    DVec2::new(0.0, top),
                ],
            )
        },
        DVec2::new(2.0, 2.0),
        360.0,
    )
}

#[test]
fn a_stepped_shaft_turned_holds_the_arithmetics_volume() {
    assert_near(
        exact(&a_stepped_shaft(20.0, 40.0)).body.volume(),
        2500.0 * PI,
        "Ø20 for 20 and Ø10 for 20",
    );
}

#[test]
fn a_partial_turn_either_way_holds_the_arithmetics_volume() {
    for degrees in [90.0, -90.0] {
        let history = turned_on_xy(
            |history| rectangle(history, 0, DVec2::new(2.0, 0.0), DVec2::new(5.0, 4.0)),
            DVec2::new(3.0, 2.0),
            degrees,
        );

        assert_near(
            exact(&history).body.volume(),
            21.0 * PI,
            &format!("[2,5]×[0,4] turned {degrees}°"),
        );
    }
}

#[test]
fn a_groove_turned_into_a_shaft_leaves_the_arithmetics_volume() {
    let mut history = turned_on_xy(
        |history| rectangle(history, 0, DVec2::ZERO, DVec2::new(10.0, 40.0)),
        DVec2::splat(5.0),
        360.0,
    );
    sketch_on(&mut history, WorkPlane::XY);
    rectangle(
        &mut history,
        1,
        DVec2::new(8.0, 15.0),
        DVec2::new(10.0, 20.0),
    );
    turn(
        &mut history,
        1,
        DVec2::new(9.0, 17.0),
        ABOUT_V,
        360.0,
        ExtrusionMode::Cut,
    );

    assert_near(
        exact(&history).body.volume(),
        3820.0 * PI,
        "a shaft Ø20 by 40 less a groove 2 deep and 5 wide",
    );
}

/// A ring turned about V, far from the origin, then a Ø40 raised 10 at the
/// origin.
fn a_disc_raised_after_a_ring() -> History {
    let mut history = turned_on_xy(
        |history| rectangle(history, 0, DVec2::new(50.0, 0.0), DVec2::new(60.0, 10.0)),
        DVec2::new(55.0, 5.0),
        360.0,
    );
    sketch_on(&mut history, WorkPlane::XY);
    circle(&mut history, 1, DVec2::new(0.0, 100.0), 20.0);
    raise(
        &mut history,
        1,
        DVec2::new(0.0, 100.0),
        10.0,
        ExtrusionMode::Add,
    );
    history
}

#[test]
fn a_prism_raised_after_a_straight_turn_is_computed_by_the_exact_kernel() {
    let history = a_disc_raised_after_a_ring();
    let ring = PI * (3600.0 - 2500.0) * 10.0;

    assert_near(
        exact(&history).body.volume() - ring,
        PI * 400.0 * 10.0,
        "the Ø40 raised after the turn, on its true cylinder",
    );
}

fn segment(history: &mut History, start: PointRef, end: PointRef) {
    history.push(Operation::AddSegment {
        sketch: 0,
        start,
        end,
        construction: false,
    });
}

fn rule(history: &mut History, constraint: Constraint) {
    history.push(Operation::Constrain {
        sketch: 0,
        constraint,
    });
}

#[test]
fn a_shaft_drawn_square_by_rules_stays_on_the_exact_kernel() {
    let mut history = History::default();
    sketch_on(&mut history, WorkPlane::XY);
    segment(
        &mut history,
        PointRef::New(DVec2::new(0.3, 0.2)),
        PointRef::New(DVec2::new(10.0, -0.4)),
    );
    segment(
        &mut history,
        PointRef::Existing(PointId(2)),
        PointRef::New(DVec2::new(10.5, 30.0)),
    );
    segment(
        &mut history,
        PointRef::Existing(PointId(3)),
        PointRef::New(DVec2::new(-0.2, 29.0)),
    );
    segment(
        &mut history,
        PointRef::Existing(PointId(4)),
        PointRef::Existing(PointId(1)),
    );
    rule(
        &mut history,
        Constraint::AxisCollinear {
            segment: SegmentId(3),
            axis: SketchAxis::V,
        },
    );
    rule(
        &mut history,
        Constraint::Perpendicular {
            first: SegmentId(0),
            second: SegmentId(3),
        },
    );
    rule(
        &mut history,
        Constraint::Parallel {
            first: SegmentId(1),
            second: SegmentId(3),
        },
    );
    rule(
        &mut history,
        Constraint::Parallel {
            first: SegmentId(0),
            second: SegmentId(2),
        },
    );
    turn(
        &mut history,
        0,
        DVec2::new(5.0, 15.0),
        ABOUT_V,
        360.0,
        ExtrusionMode::Add,
    );
    assert!(
        PartState::rebuild(&history).body.is_exact(),
        "a quadrilateral given rules is turned exactly",
    );
    sketch_on(&mut history, WorkPlane::XY);
    circle(&mut history, 1, DVec2::new(0.0, 100.0), 20.0);
    raise(
        &mut history,
        1,
        DVec2::new(0.0, 100.0),
        10.0,
        ExtrusionMode::Add,
    );

    exact(&history);
}

/// The plane a drawing laid on face `face` of the part `history` holds
/// stands on once the part is replayed, and whether it lost its face.
fn laid_on(mut history: History, face: usize) -> (WorkPlane, bool) {
    let sketch = PartState::rebuild(&history).sketches.len();
    history.push(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: Some(FaceAnchor { face, up: DVec3::Z }),
    });
    let state = PartState::rebuild(&history);
    (state.sketches[sketch].plane, state.adrift.contains(&sketch))
}

/// The number of the face of a body that faces `normal` and stands at
/// `offset` along it.
fn the_face(state: &PartState, normal: DVec3, offset: f64) -> usize {
    (0..state.body.faces_end())
        .find(|&face| {
            state.body.plane_of(face).is_some_and(|plane| {
                plane.normal.distance(normal) < 1e-9
                    && plane
                        .corners
                        .iter()
                        .all(|corner| (corner.dot(normal) - offset).abs() < 1e-9)
            })
        })
        .expect("a face of the body stands there")
}

#[test]
fn a_drawing_on_a_shoulder_of_a_turned_shaft_rides_it_when_the_shoulder_moves() {
    let shoulder = the_face(&exact(&a_stepped_shaft(20.0, 40.0)), DVec3::Y, 20.0);

    for moved in [20.0, 25.0, 12.0] {
        let (plane, adrift) = laid_on(a_stepped_shaft(moved, 40.0), shoulder);

        assert!(!adrift, "the shoulder at {moved} is still there");
        assert!(
            plane.normal().distance(DVec3::Y) < 1e-9 && (plane.origin.y - moved).abs() < 1e-9,
            "the drawing stands on the shoulder at {moved}, not on {plane:?}",
        );
    }
}

#[test]
fn a_drawing_on_the_closing_end_of_a_partial_turn_follows_it_when_the_turn_widens() {
    let closing_end = 4 + 1;

    for degrees in [90.0, 120.0, 200.0, -60.0] {
        let history = turned_on_xy(
            |history| rectangle(history, 0, DVec2::new(2.0, 0.0), DVec2::new(5.0, 4.0)),
            DVec2::new(3.0, 2.0),
            degrees,
        );
        let (plane, adrift) = laid_on(history, closing_end);

        let normal = plane.normal();
        let opening = DVec3::Z.dot(normal);
        assert!(!adrift, "the closing end of a turn of {degrees}° is there");
        assert!(
            normal.y.abs() < 1e-9
                && (opening.abs() - degrees.to_radians().cos().abs()).abs() < 1e-9,
            "the drawing stands on the end turned {degrees}° from the sketch, not on \
             one facing {normal}",
        );
    }
}

/// The number the history gave its last step.
fn last_step(history: &History) -> u32 {
    history.steps().last().expect("a step").operations()[0]
}

/// A part made by applying each operation of `history` as the user would.
fn applied_live(history: &History) -> PartDocument {
    let now = "2026-10-06T09:00:00Z".parse().expect("a date");
    let mut part = PartDocument::new("Turned", now);
    for operation in history.operations() {
        part.apply(operation.clone());
    }
    part
}

/// A square drawn from `(left, 0)`, its last run down the axis or beside it,
/// turned whole; then a block raised beside it, and a drawing laid on face
/// `face`.
fn a_block_after_a_whole_turn(left: f64, face: usize) -> (WorkPlane, bool) {
    let mut history = turned_on_xy(
        |history| rectangle(history, 0, DVec2::new(left, 0.0), DVec2::splat(10.0)),
        DVec2::splat(5.0),
        360.0,
    );
    sketch_on(
        &mut history,
        WorkPlane {
            origin: DVec3::Z * 20.0,
            ..WorkPlane::XY
        },
    );
    rectangle(
        &mut history,
        1,
        DVec2::new(30.0, 0.0),
        DVec2::new(40.0, 10.0),
    );
    raise(
        &mut history,
        1,
        DVec2::new(35.0, 5.0),
        5.0,
        ExtrusionMode::Add,
    );
    laid_on(history, face)
}

#[test]
fn faces_after_a_full_turn_are_numbered_as_on_main() {
    // Measured on `main` at dc5bfe5, where the flats turned the square: the
    // block's top answered to this number.
    let blocks_top_on_main = 4;

    let (plane, adrift) = a_block_after_a_whole_turn(0.0, blocks_top_on_main);

    assert!(!adrift, "the block's top is there");
    assert!(
        (plane.origin.z - 25.0).abs() < 1e-9,
        "the drawing laid by number stands on the block's top as it did on \
         `main`, not on {plane:?}",
    );
}

#[test]
fn faces_after_a_full_turn_whose_side_lies_within_the_band_are_numbered_as_on_main() {
    // Measured on `main` at dc5bfe5, the side 0.005 from the axis.
    let blocks_top_on_main = 5;

    let (plane, adrift) = a_block_after_a_whole_turn(0.005, blocks_top_on_main);

    assert!(!adrift, "the block's top is there");
    assert!(
        (plane.origin.z - 25.0).abs() < 1e-9,
        "the drawing laid by number stands on the block's top as it did on \
         `main`, not on {plane:?}",
    );
}

/// What the flats make of a circle: the polygon of 48 sides inscribed in it.
fn inscribed(radius: f64) -> f64 {
    let sides = 48.0;
    sides / 2.0 * radius * radius * (2.0 * PI / sides).sin()
}

/// How much matter a Ø40 raised 10 far from the rest adds to the part
/// `history` describes, applied as the user would, so that the part is built
/// once rather than at every click; the part held to stay on the flats.
fn a_disc_raised_after(history: &History) -> f64 {
    let (added, exact) = a_disc_raised_on(history);
    assert!(!exact, "the part stays on the flats");
    added
}

/// How much matter a Ø40 raised 10 far from the rest adds to the part
/// `history` describes, and whether the part is still exact after it.
fn a_disc_raised_on(history: &History) -> (f64, bool) {
    let mut part = applied_live(history);
    let before = part.body().volume();
    let sketch = part.sketches().len();
    part.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    part.apply(Operation::AddCircle {
        sketch,
        center: PointRef::New(DVec2::new(0.0, 100.0)),
        radius: 20.0,
        rim: Vec::new(),
        construction: false,
    });
    let areas = part.areas_at(sketch, &[DVec2::new(0.0, 100.0)]);
    part.apply(Operation::Extrude {
        sketch,
        areas,
        distance: 10.0.into(),
        mode: ExtrusionMode::Add,
    });
    (part.body().volume() - before, part.body().is_exact())
}

#[test]
fn a_part_turned_from_an_arc_is_computed_by_the_flats_from_that_step_on() {
    let mut history = History::default();
    sketch_on(&mut history, WorkPlane::XY);
    let first = PartState::rebuild(&history).sketches[0].points().len();
    history.push(Operation::AddArc {
        sketch: 0,
        center: PointRef::New(DVec2::new(70.0, 5.0)),
        start: PointRef::New(DVec2::new(75.0, 0.0)),
        end: PointRef::New(DVec2::new(75.0, 10.0)),
        construction: false,
    });
    history.push(Operation::AddSegment {
        sketch: 0,
        start: PointRef::Existing(PointId(first + 2)),
        end: PointRef::Existing(PointId(first + 1)),
        construction: false,
    });
    turn(
        &mut history,
        0,
        DVec2::new(76.0, 5.0),
        ABOUT_V,
        360.0,
        ExtrusionMode::Add,
    );

    assert!(
        !applied_live(&history).body().is_exact(),
        "an arc turned is a torus, which the exact kernel has no surface for",
    );
    assert_near(
        a_disc_raised_after(&history),
        inscribed(20.0) * 10.0,
        "the Ø40 raised after the torus, by the flats",
    );
}

#[test]
fn a_part_turned_from_a_slanted_run_after_an_exact_one_stays_exact_from_that_step_on() {
    let mut history = turned_on_xy(
        |history| rectangle(history, 0, DVec2::new(5.0, 0.0), DVec2::new(6.0, 1.0)),
        DVec2::new(5.5, 0.5),
        360.0,
    );
    exact(&history);
    sketch_on(&mut history, WorkPlane::XY);
    polygon(
        &mut history,
        1,
        &[
            DVec2::new(70.0, 0.0),
            DVec2::new(80.0, 0.0),
            DVec2::new(75.0, 10.0),
        ],
    );
    turn(
        &mut history,
        1,
        DVec2::new(75.0, 3.0),
        ABOUT_V,
        360.0,
        ExtrusionMode::Add,
    );
    exact(&history);

    let (added, stays_exact) = a_disc_raised_on(&history);
    assert!(stays_exact, "the part stays on the exact kernel");
    assert_near(
        added,
        PI * 400.0 * 10.0,
        "the Ø40 raised after the cone, on its true cylinder",
    );
}

/// A cylinder Ø4 raised 20 along Y from XZ, centred on X at `at`; then a
/// rectangle drawn square to a segment at 30° in XY, the segment turned
/// about, the rectangle turned whole about it. With the cylinder at the
/// origin the two axes meet at 60°.
fn a_turn_at_thirty_degrees_beside_a_cylinder_at(at: f64) -> History {
    let mut history = History::default();
    sketch_on(
        &mut history,
        WorkPlane {
            origin: DVec3::Y * 10.0,
            ..WorkPlane::XZ
        },
    );
    circle(&mut history, 0, DVec2::new(at, 0.0), 2.0);
    raise(
        &mut history,
        0,
        DVec2::new(at, 0.0),
        20.0,
        ExtrusionMode::Add,
    );
    sketch_on(&mut history, WorkPlane::XY);
    let along = DVec2::from_angle(PI / 6.0);
    let at = |h: f64, s: f64| along * h + along.perp() * s;
    history.push(Operation::AddSegment {
        sketch: 1,
        start: PointRef::New(at(-3.0, 0.0)),
        end: PointRef::New(at(-1.0, 0.0)),
        construction: false,
    });
    polygon(
        &mut history,
        1,
        &[at(1.0, 1.0), at(6.0, 1.0), at(6.0, 3.0), at(1.0, 3.0)],
    );
    turn(
        &mut history,
        1,
        at(3.0, 2.0),
        RevolutionAxis::Segment(SegmentId(0)),
        360.0,
        ExtrusionMode::Add,
    );
    history
}

#[test]
fn a_turn_crossing_a_raised_cylinder_at_a_skew_angle_is_a_broken_step_with_its_reason() {
    let history = a_turn_at_thirty_degrees_beside_a_cylinder_at(0.0);

    let part = applied_live(&history);

    assert_eq!(
        part.declined_because(last_step(&history)),
        Some(Declined::Unsupported),
        "the turn's cylinders cross the raised one at 60°",
    );
    assert_near(
        part.body().volume(),
        PI * 4.0 * 20.0,
        "the cylinder, as it stood before",
    );
    assert!(part.body().is_exact(), "the part stays exact");
}

#[test]
fn a_step_after_a_declined_turn_numbers_its_faces_as_if_it_had_stood() {
    let with_a_plate_after = |at: f64| {
        let mut history = a_turn_at_thirty_degrees_beside_a_cylinder_at(at);
        sketch_on(&mut history, WorkPlane::XY);
        rectangle(
            &mut history,
            2,
            DVec2::new(20.0, 20.0),
            DVec2::new(30.0, 30.0),
        );
        raise(
            &mut history,
            2,
            DVec2::new(25.0, 25.0),
            3.0,
            ExtrusionMode::Add,
        );
        history
    };
    // The cylinder's floor, top and wall; the turn's four runs, none on its
    // axis; then the plate's floor and top.
    let plates_top = 3 + 4 + 1;
    let declined = with_a_plate_after(0.0);
    let stood = with_a_plate_after(50.0);
    assert!(applied_live(&declined).is_declined(last_step(&declined) - 3));
    assert!(!applied_live(&stood).is_declined(last_step(&stood) - 3));

    for history in [declined, stood] {
        let (plane, adrift) = laid_on(history, plates_top);
        assert!(
            !adrift && (plane.origin.z - 3.0).abs() < 1e-9,
            "the drawing stands on the plate's top, not on {plane:?}",
        );
    }
}

/// A square 10 on a side whose corner at the origin was drawn at `corner`.
fn a_square_with_its_corner_at(corner: DVec2) -> History {
    turned_on_xy(
        |history| {
            polygon(
                history,
                0,
                &[
                    corner,
                    DVec2::new(10.0, 0.0),
                    DVec2::new(10.0, 10.0),
                    DVec2::new(0.0, 10.0),
                ],
            )
        },
        DVec2::splat(5.0),
        360.0,
    )
}

#[test]
fn a_square_a_hair_across_the_axis_is_turned_as_if_drawn_on_it() {
    assert_near(
        exact(&a_square_with_its_corner_at(DVec2::new(-1e-5, 0.0)))
            .body
            .volume(),
        1000.0 * PI,
        "a corner 1e-5 across the axis, laid on it",
    );
}

#[test]
fn a_square_a_hair_off_the_axis_on_its_own_side_is_turned_as_if_drawn_on_it() {
    assert_near(
        exact(&a_square_with_its_corner_at(DVec2::new(1e-5, 0.0)))
            .body
            .volume(),
        1000.0 * PI,
        "a corner 1e-5 off the axis, laid on it",
    );
}

fn a_part(name: &str) -> PartDocument {
    let mut part = PartDocument::new(name, "2026-09-30T09:00:00Z".parse().expect("a date"));
    part.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    part
}

fn a_segment(start: PointRef, end: PointRef) -> Operation {
    Operation::AddSegment {
        sketch: 0,
        start,
        end,
        construction: false,
    }
}

fn a_rule(constraint: Constraint) -> Operation {
    Operation::Constrain {
        sketch: 0,
        constraint,
    }
}

fn turned_whole_at(part: &mut PartDocument, place: DVec2) {
    let areas = part.areas_at(0, &[place]);
    part.apply(Operation::Revolve {
        sketch: 0,
        areas,
        axis: ABOUT_V,
        angle: 360.0.into(),
        mode: ExtrusionMode::Add,
    });
}

#[test]
fn a_profile_whose_side_was_laid_on_the_axis_turns_into_a_closed_solid() {
    let mut part = a_part("turned");
    part.apply(a_segment(
        PointRef::New(DVec2::new(1.0, 2.0)),
        PointRef::New(DVec2::new(20.0, -1.0)),
    ));
    part.apply(a_segment(
        PointRef::Existing(PointId(2)),
        PointRef::New(DVec2::new(35.0, 30.0)),
    ));
    part.apply(a_segment(
        PointRef::Existing(PointId(3)),
        PointRef::New(DVec2::new(1.0, 35.0)),
    ));
    part.apply(a_segment(
        PointRef::Existing(PointId(4)),
        PointRef::Existing(PointId(1)),
    ));
    part.apply(a_rule(Constraint::AxisCollinear {
        segment: SegmentId(3),
        axis: SketchAxis::V,
    }));
    part.apply(a_rule(Constraint::Parallel {
        first: SegmentId(0),
        second: SegmentId(2),
    }));
    turned_whole_at(&mut part, DVec2::new(10.0, 15.0));

    assert_eq!(closed(&part.body().triangles()), Ok(()));
}

#[test]
fn a_cone_whose_leg_was_laid_on_the_axis_comes_out_closed() {
    let mut part = a_part("cone");
    part.apply(a_segment(
        PointRef::New(DVec2::new(1.0, 0.5)),
        PointRef::New(DVec2::new(20.0, -1.0)),
    ));
    part.apply(a_segment(
        PointRef::Existing(PointId(2)),
        PointRef::New(DVec2::new(1.5, 30.0)),
    ));
    part.apply(a_segment(
        PointRef::Existing(PointId(3)),
        PointRef::Existing(PointId(1)),
    ));
    part.apply(a_rule(Constraint::AxisCollinear {
        segment: SegmentId(2),
        axis: SketchAxis::V,
    }));
    part.apply(a_rule(Constraint::Perpendicular {
        first: SegmentId(0),
        second: SegmentId(2),
    }));
    turned_whole_at(&mut part, DVec2::new(5.0, 5.0));

    assert_eq!(closed(&part.body().triangles()), Ok(()));
    assert!(part.body().is_exact(), "the cone is the exact kernel's");
}

/// A rectangle from (−4, 0) to (5, 2), across the V axis, turned `degrees`
/// about `axis`.
fn across(axis: RevolutionAxis, degrees: f64, draw_the_axis: impl FnOnce(&mut History)) -> History {
    let mut history = History::default();
    sketch_on(&mut history, WorkPlane::XY);
    draw_the_axis(&mut history);
    rectangle(&mut history, 0, DVec2::new(-4.0, 0.0), DVec2::new(5.0, 2.0));
    turn(
        &mut history,
        0,
        DVec2::new(1.0, 1.0),
        axis,
        degrees,
        ExtrusionMode::Add,
    );
    history
}

#[test]
fn an_area_across_a_sketch_axis_is_turned_on_both_sides_and_joined() {
    assert_near(
        exact(&across(ABOUT_V, 360.0, |_| {})).body.volume(),
        50.0 * PI,
        "the side reaching 5 turned whole holds the side reaching 4",
    );
}

#[test]
fn an_area_across_the_axis_turned_part_way_holds_both_sides_volumes() {
    for degrees in [90.0, -90.0] {
        assert_near(
            exact(&across(ABOUT_V, degrees, |_| {})).body.volume(),
            20.5 * PI,
            &format!("a quarter turn {degrees}° of each side, apart"),
        );
    }
}

#[test]
fn an_area_across_a_construction_line_is_turned_on_both_sides_and_joined() {
    let history = across(RevolutionAxis::Segment(SegmentId(0)), 360.0, |history| {
        history.push(Operation::AddSegment {
            sketch: 0,
            start: PointRef::New(DVec2::new(0.0, -10.0)),
            end: PointRef::New(DVec2::new(0.0, 10.0)),
            construction: true,
        });
    });

    assert_near(
        exact(&history).body.volume(),
        50.0 * PI,
        "a construction line splits no area, and the turn cuts it itself",
    );
}

#[test]
fn a_turn_about_a_segment_held_parallel_to_v_by_a_rule_stays_exact_beside_a_raised_bore() {
    let mut history = History::default();
    sketch_on(&mut history, WorkPlane::XZ);
    circle(&mut history, 0, DVec2::new(3.0, 0.0), 4.0);
    raise(
        &mut history,
        0,
        DVec2::new(3.0, 0.0),
        -20.0,
        ExtrusionMode::Add,
    );
    sketch_on(&mut history, WorkPlane::XY);
    history.push(Operation::AddSegment {
        sketch: 1,
        start: PointRef::New(DVec2::new(3.0, -5.0)),
        end: PointRef::New(DVec2::new(3.2, 25.0)),
        construction: true,
    });
    history.push(Operation::Constrain {
        sketch: 1,
        constraint: Constraint::AxisParallel {
            segment: SegmentId(0),
            axis: SketchAxis::V,
        },
    });
    let x = PartState::rebuild(&history).sketches[1]
        .endpoints(SegmentId(0))
        .0
        .x;
    rectangle(
        &mut history,
        1,
        DVec2::new(x + 4.0, 0.0),
        DVec2::new(x + 8.0, 20.0),
    );
    turn(
        &mut history,
        1,
        DVec2::new(x + 6.0, 10.0),
        RevolutionAxis::Segment(SegmentId(0)),
        360.0,
        ExtrusionMode::Add,
    );

    let part = applied_live(&history);
    assert_eq!(part.declined_because(last_step(&history)), None);
    assert!(part.body().is_exact(), "the part stays exact");
    assert_near(
        part.body().volume(),
        PI * 64.0 * 20.0,
        "the boss inside the tube turned about it",
    );
}

#[test]
fn a_cut_drawn_on_a_face_of_a_turned_part_where_a_click_lays_it_is_computed_by_the_exact_kernel() {
    let mut history = turned_on_xy(
        |history| rectangle(history, 0, DVec2::new(-30.0, -7.5), DVec2::new(0.0, -52.5)),
        DVec2::new(-14.8, -26.5),
        360.0,
    );
    let turned = exact(&history);
    let top = the_face(&turned, DVec3::Y, -7.5);
    let face = turned.body.plane_of(top).expect("the top is flat");
    let plane = WorkPlane::from_face(&face.corners, face.normal, DVec3::Z);
    history.push(Operation::CreateSketch {
        plane,
        on: Some(FaceAnchor {
            face: top,
            up: DVec3::Z,
        }),
    });
    let middle = plane.to_local(DVec3::new(0.0, -7.5, 0.0));
    rectangle(
        &mut history,
        1,
        middle - DVec2::splat(5.0),
        middle + DVec2::splat(5.0),
    );
    raise(&mut history, 1, middle, -10.0, ExtrusionMode::Cut);

    assert_near(
        turned.body.volume() - exact(&history).body.volume(),
        1000.0,
        "a pocket 10 by 10, 10 deep, in the top of a Ø60 cylinder",
    );
}

#[test]
fn an_area_across_the_axis_turned_into_a_shaft_takes_both_sides_away() {
    let mut history = turned_on_xy(
        |history| rectangle(history, 0, DVec2::ZERO, DVec2::new(10.0, 40.0)),
        DVec2::splat(5.0),
        360.0,
    );
    sketch_on(&mut history, WorkPlane::XY);
    rectangle(
        &mut history,
        1,
        DVec2::new(-4.0, 10.0),
        DVec2::new(5.0, 20.0),
    );
    turn(
        &mut history,
        1,
        DVec2::new(1.0, 15.0),
        ABOUT_V,
        360.0,
        ExtrusionMode::Cut,
    );

    assert_near(
        exact(&history).body.volume(),
        4000.0 * PI - 250.0 * PI,
        "a shaft Ø20 by 40 less the Ø10 by 10 the side reaching 5 turns",
    );
}

/// A square from -10 to 10 across V, 10 high, holed by a square 3 wide on
/// its right, turned by `degrees`.
fn a_holed_area_across_the_axis(degrees: f64) -> History {
    turned_on_xy(
        |history| {
            rectangle(history, 0, DVec2::new(-10.0, 0.0), DVec2::new(10.0, 10.0));
            rectangle(history, 0, DVec2::new(3.0, 3.0), DVec2::new(6.0, 6.0));
        },
        DVec2::new(-5.0, 5.0),
        degrees,
    )
}

#[test]
fn a_holed_area_across_the_axis_turned_whole_is_filled_by_its_other_side() {
    assert_near(
        exact(&a_holed_area_across_the_axis(360.0)).body.volume(),
        1000.0 * PI,
        "the left side turned whole fills the ring the hole on the right leaves",
    );
}

#[test]
fn a_holed_area_across_the_axis_turned_part_way_keeps_its_hole() {
    let right = 500.0 - 3.0 * (36.0 - 9.0) / 2.0;
    let left = 500.0;

    assert_near(
        exact(&a_holed_area_across_the_axis(90.0)).body.volume(),
        PI / 2.0 * (right + left),
        "a quarter turn of each side, apart, the right one holed",
    );
}

#[test]
fn a_slanted_area_across_the_axis_turned_whole_is_its_larger_side_s_cone_exactly() {
    let history = turned_on_xy(
        |history| {
            polygon(
                history,
                0,
                &[
                    DVec2::new(-5.0, 0.0),
                    DVec2::new(10.0, 0.0),
                    DVec2::new(0.0, 10.0),
                ],
            )
        },
        DVec2::new(1.0, 1.0),
        360.0,
    );

    assert_near(
        exact(&history).body.volume(),
        1000.0 * PI / 3.0,
        "the side reaching 10 turned whole, holding the side reaching 5",
    );
}

/// A shaft Ø20 for 30 turned whole, given a flat 2 deep across its side,
/// then a block raised at 50 whose top is drawn on by number.
fn a_flat_on_a_shaft_then_a_block(history: &mut History) {
    sketch_on(
        history,
        WorkPlane {
            origin: DVec3::Z * 8.0,
            ..WorkPlane::XY
        },
    );
    rectangle(history, 1, DVec2::new(-15.0, 5.0), DVec2::new(15.0, 15.0));
    raise(history, 1, DVec2::new(0.0, 10.0), 5.0, ExtrusionMode::Cut);
    sketch_on(
        history,
        WorkPlane {
            origin: DVec3::Z * 50.0,
            ..WorkPlane::XY
        },
    );
    rectangle(history, 2, DVec2::new(30.0, 0.0), DVec2::new(40.0, 10.0));
    raise(history, 2, DVec2::new(35.0, 5.0), 5.0, ExtrusionMode::Add);
}

#[test]
#[ignore = "known renumbering, not mended by #533: the flats parted the shaft's \
            cylinder, cut across by the flat, into two numbers, where the exact \
            kernel sees it whole; every face numbered after that step on main is \
            one lower now, and a drawing saved there lands on a wall, without a \
            word. Both kernels would have to part a face alike"]
fn a_drawing_saved_on_main_on_a_block_raised_after_a_flat_on_a_turned_shaft_keeps_its_face() {
    let mut history = turned_on_xy(
        |history| rectangle(history, 0, DVec2::ZERO, DVec2::new(10.0, 40.0)),
        DVec2::splat(5.0),
        360.0,
    );
    a_flat_on_a_shaft_then_a_block(&mut history);
    // Measured on `main` at dc5bfe5.
    let blocks_top_on_main = 11;

    let (plane, adrift) = laid_on(history, blocks_top_on_main);

    assert!(!adrift, "the block's top is there");
    assert!(
        (plane.origin.z - 55.0).abs() < 1e-9 && plane.normal().distance(DVec3::Z) < 1e-9,
        "the drawing stands on the block's top as it did on `main`, not on {plane:?}",
    );
}

/// The shaft of a part whose shoulder's radius at the top is the variable
/// `w`: a size of 10 draws its runs straight, any other slants the last one.
fn a_shaft_whose_top_radius_is_a_variable(w: f64) -> PartDocument {
    let now = "2026-10-06T09:00:00Z".parse().expect("a date");
    let mut part = PartDocument::new("Turned", now);
    part.change_variable(VariableChange::Added {
        name: "w".into(),
        formula: w.into(),
    })
    .expect("w is a name");
    let mut history = History::default();
    sketch_on(&mut history, WorkPlane::XY);
    polygon(
        &mut history,
        0,
        &[
            DVec2::new(0.0, 0.0),
            DVec2::new(10.0, 0.0),
            DVec2::new(10.0, 20.0),
            DVec2::new(w, 30.0),
            DVec2::new(0.0, 30.0),
        ],
    );
    for operation in history.operations() {
        part.apply(operation.clone());
    }
    for constraint in [
        Constraint::AxisCollinear {
            segment: SegmentId(4),
            axis: SketchAxis::V,
        },
        Constraint::AxisPerpendicular {
            segment: SegmentId(0),
            axis: SketchAxis::V,
        },
        Constraint::AxisParallel {
            segment: SegmentId(1),
            axis: SketchAxis::V,
        },
        Constraint::AxisPerpendicular {
            segment: SegmentId(3),
            axis: SketchAxis::V,
        },
    ] {
        part.apply(Operation::Constrain {
            sketch: 0,
            constraint,
        });
    }
    let w = part.variables().read("w").expect("w is known");
    for (segment, value) in [(0, 10.0.into()), (1, 20.0.into()), (4, 30.0.into()), (3, w)] {
        part.apply(Operation::SetDimension {
            sketch: 0,
            target: DimensionTarget::Length(SegmentId(segment)),
            value,
            placement: None,
        });
    }
    let areas = part.areas_at(0, &[DVec2::splat(2.0)]);
    part.apply(Operation::Revolve {
        sketch: 0,
        areas,
        axis: ABOUT_V,
        angle: 360.0.into(),
        mode: ExtrusionMode::Add,
    });
    let mut rest = History::default();
    sketch_on(&mut rest, WorkPlane::XY);
    a_flat_on_a_shaft_then_a_block(&mut rest);
    for operation in rest.operations().iter().skip(1) {
        part.apply(operation.clone());
    }
    part
}

#[test]
fn a_drawing_after_a_turn_keeps_its_face_when_a_size_slants_a_run_into_a_cone() {
    for (from, to) in [(10.0, 6.0), (6.0, 10.0)] {
        let mut part = a_shaft_whose_top_radius_is_a_variable(from);
        assert!(part.body().is_exact(), "w at {from}: the part is exact");
        let top = (0..part.body().faces_end())
            .find(|&face| {
                part.body().plane_of(face).is_some_and(|plane| {
                    plane.normal.distance(DVec3::Z) < 1e-9
                        && plane
                            .corners
                            .iter()
                            .all(|corner| (corner.z - 55.0).abs() < 1e-9)
                })
            })
            .expect("the block's top");
        let drawing = part.sketches().len();
        part.apply(Operation::CreateSketch {
            plane: WorkPlane::XY,
            on: Some(FaceAnchor {
                face: top,
                up: DVec3::Z,
            }),
        });

        part.change_variable(VariableChange::Edited {
            variable: VariableId(0),
            name: "w".into(),
            formula: to.into(),
        })
        .expect("w takes the size");

        let plane = part.sketches()[drawing].plane;
        assert!(part.body().is_exact(), "w at {to}: the part is exact");
        assert!(
            !part.is_adrift(drawing),
            "w from {from} to {to}: the top is there"
        );
        assert!(
            (plane.origin.z - 55.0).abs() < 1e-9 && plane.normal().distance(DVec3::Z) < 1e-9,
            "w from {from} to {to}: the drawing stays on the block's top, not on {plane:?}",
        );
    }
}

/// How long a whole turn of a round area across its axis may take: its larger
/// side alone, on the flats, takes milliseconds, where both sides turned and
/// joined took most of a minute in release (the night campaign of parts).
const PATIENCE: Duration = Duration::from_secs(10);

#[test]
fn a_circle_across_the_axis_turned_whole_is_turned_from_its_larger_side_alone() {
    let (center, radius) = (DVec2::new(5.0, 16.0), 7.0);
    let history = turned_on_xy(|history| circle(history, 0, center, radius), center, 360.0);
    let (sender, receiver) = mpsc::channel();
    std::thread::spawn(move || {
        let _ = sender.send(PartState::rebuild(&history).body.volume());
    });
    let volume = receiver
        .recv_timeout(PATIENCE)
        .expect("the turn answers within the patience");

    let steps = 100_000;
    let width = (center.x + radius) / steps as f64;
    let pappus: f64 = (0..steps)
        .map(|step| {
            let u = (step as f64 + 0.5) * width;
            let half = (radius * radius - (u - center.x).powi(2)).max(0.0).sqrt();
            2.0 * PI * u * 2.0 * half * width
        })
        .sum();
    assert!(
        (volume - pappus).abs() < 0.02 * pappus,
        "the disc's side reaching 12 turned whole holds the side reaching 2: {volume} where Pappus gives {pappus}",
    );
}
