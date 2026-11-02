//! Closes #576.
//! - a sphere, a zone of one, a ring torus and a rounded shaft end, turned
//!   whole and part way, hold the arithmetic's volume (Pappus), exactly —
//!   `a_ball_turned_whole_and_part_way_holds_the_arithmetics_volume` (4πr³/3,
//!   and a lune of it),
//!   `a_zone_of_a_sphere_turned_whole_and_part_way_holds_the_arithmetics_volume`
//!   (π[r²(h_b − h_a) − (h_b³ − h_a³)/3]),
//!   `a_ring_torus_turned_whole_and_part_way_holds_the_arithmetics_volume`
//!   (2π²Rr²),
//!   `a_rounded_shaft_end_turned_whole_and_part_way_holds_the_arithmetics_volume`
//!   (πr²L + 2πr³/3, the cap tangent to the shaft)
//! - a plane square to the axis or holding it, and a coaxial cylinder or
//!   cone, joined to or cut from a sphere or a torus, are built exactly —
//!   `a_ball_cut_by_a_plane_square_to_its_axis_holds_the_arithmetics_volume`
//!   (a cap of height H: πH²(3r − H)/3),
//!   `a_ball_cut_by_a_block_through_its_axis_holds_the_arithmetics_volume`
//!   (the block's side a plane holding the axis, the cut a meridian),
//!   `a_coaxial_bore_through_a_ball_holds_the_arithmetics_volume`
//!   (two circles, the rest a zone less a cylinder),
//!   `a_coaxial_shaft_joined_to_a_ball_at_its_equator_holds_the_arithmetics_volume`
//!   (the shaft's wall touching the ball along one circle, decided once),
//!   `a_coaxial_cone_cut_from_a_ring_torus_holds_the_arithmetics_volume`
//!   (the cone meeting the tube along two circles),
//!   `a_round_bottomed_groove_cut_into_a_shaft_holds_the_arithmetics_volume`
//!   (a torus taking matter away, tangent to the shaft's wall)
//! - a sphere or a torus against anything else is a broken step with its
//!   reason —
//!   `a_hole_drilled_off_axis_through_a_ball_is_a_broken_step_with_its_reason`,
//!   `a_plane_at_a_slant_through_a_ball_is_a_broken_step_with_its_reason`
//!   (`Declined::Unsupported` read through `declined_because`, the part as
//!   before the step, and still exact)
//! - every face keeps the number the flats give it today, so a drawing laid
//!   on a face keeps it —
//!   `a_ball_knob_and_a_rounded_shaft_number_their_faces_as_the_flats_do`
//!   (the faces made counted from the floor against the same step on a part
//!   already on the flats, whole and part way; against the numbers that gave
//!   on `main` since #632, which leaves no step a way to the flats); every
//!   rounded leaf of the
//!   harness's draw is held to the flats' numbers in `cao_solid` by
//!   `a_rounded_leaf_names_its_faces_as_the_flats_name_them`
//! - #448's harness draws arcs turned, before the kernel code: whole and part
//!   way, alone and against coaxial and other walls, weighted towards hairs
//!   and tangencies — no test: held in `cao_solid` by
//!   `random_turned_solids.rs` (`Case::drawn_rounded`,
//!   `Case::drawn_rounded_off_the_lattice`,
//!   `the_rounded_generator_draws_every_kind_it_keeps`,
//!   `the_draws_before_rounds_still_give_each_seed_its_case`)
//! - campaigns end with the new draws under one failure in a thousand on
//!   fresh seeds, no seed of the other draws that held before failing; a
//!   family that does not move in two rounds is reported and set aside — no
//!   test: counts and families in `docs/exact-kernel-failures.md` ("#576:
//!   spheres and tori"), the campaigns behind `--features campaigns`; the
//!   draw's fast cases are in the gate,
//!   `random_rounded_cases_keep_every_rule_on_the_exact_kernel` and
//!   `rounded_cases_the_exact_kernel_keeps_are_kept_through_the_application_s_body_and_stay_exact`
//! - `REBUILT_BY` moves, and `docs/exact-kernel.md` and `docs/glossary.md`
//!   no longer send an arc turned to the flats — no test: held beside the
//!   cache by `a_cache_written_before_arcs_turned_were_exact_is_replayed`,
//!   since only there can a test tell a part opened on its cache from one
//!   replayed; the docs are prose, held by `language.rs` only
//! - a turn of a profile holding an ellipse stays on the flats, and a part
//!   already on the flats turns an arc on the flats (what must not break) —
//!   no test: since #632 a turned ellipse is a broken step, held by
//!   `a_turned_ellipse_is_a_broken_step` in
//!   `a_revolution_the_exact_kernel_cannot_make_is_refused.rs`, and no part
//!   goes to the flats any more
//!
//! A part holding an arc turned, as the application builds it: the sphere or
//! the torus is the exact kernel's, and so is every step after it. The
//! kernel's own tests are `cao_solid`'s, beside the code in `brep/` and in
//! `random_turned_solids.rs`.

use std::f64::consts::PI;

use cao_part::history::{ExtrusionMode, Operation, PointRef, RevolutionAxis};
use cao_part::{History, PartDocument, PartState};
use cao_sketch::{Area, PointId, SketchAxis, WorkPlane};
use cao_solid::Declined;
use glam::{DVec2, DVec3};

/// How near the arithmetic an exact volume comes: the kernel's matter is the
/// true sphere or torus, so what is left is rounding.
const EXACT: f64 = 1e-9;

const ABOUT_V: RevolutionAxis = RevolutionAxis::Sketch(SketchAxis::V);

fn clicked(history: &History, sketch: usize, places: &[DVec2]) -> Vec<Area> {
    PartState::rebuild(history).areas_at(sketch, places)
}

fn sketch_on(history: &mut History, plane: WorkPlane) -> usize {
    let sketch = PartState::rebuild(history).sketches.len();
    history.push(Operation::CreateSketch { plane, on: None });
    sketch
}

/// The number the next point drawn in `sketch` is given.
fn next_point(history: &History, sketch: usize) -> usize {
    PartState::rebuild(history).sketches[sketch].points().len()
}

fn segment(history: &mut History, sketch: usize, start: PointRef, end: PointRef) {
    history.push(Operation::AddSegment {
        sketch,
        start,
        end,
        construction: false,
    });
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
        areas: clicked(history, sketch, &[place]),
        axis,
        angle: degrees.into(),
        mode,
    });
}

fn raise(history: &mut History, sketch: usize, place: DVec2, distance: f64, mode: ExtrusionMode) {
    history.push(Operation::Extrude {
        sketch,
        areas: clicked(history, sketch, &[place]),
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

/// The number the history gave its last step.
fn last_step(history: &History) -> u32 {
    history.steps().last().expect("a step").operations()[0]
}

/// A part made by applying each operation of `history` as the user would.
fn applied_live(history: &History) -> PartDocument {
    let now = "2026-10-31T09:00:00Z".parse().expect("a date");
    let mut part = PartDocument::new("Rounded", now);
    for operation in history.operations() {
        part.apply(operation.clone());
    }
    part
}

/// The part `history` describes, applied live, held to have built its last
/// step on the exact kernel, within the arithmetic's volume.
fn built_exactly(history: &History, arithmetic: f64, what: &str) -> PartDocument {
    let part = applied_live(history);
    assert_eq!(part.declined_because(last_step(history)), None, "{what}");
    assert!(part.body().is_exact(), "{what}: the part stays exact");
    let volume = part.body().volume();
    assert!(
        (volume - arithmetic).abs() <= EXACT * arithmetic,
        "{what}: {volume} where the arithmetic gives {arithmetic}",
    );
    part
}

/// The faces the last step of `history` made, counted from the first: the
/// numbers its raise gave, whatever the steps before it gave.
fn numbered_from_the_floor(history: &History) -> Vec<usize> {
    let made = applied_live(history).faces_made_by(last_step(history));
    made.iter().map(|face| face - made[0]).collect()
}

/// A half disc of `radius` about the origin, from its lower pole round its
/// far side to its upper: turned about V, a ball.
fn a_half_disc(history: &mut History, sketch: usize, radius: f64) {
    let first = next_point(history, sketch);
    history.push(Operation::AddArc {
        sketch,
        center: PointRef::New(DVec2::ZERO),
        start: PointRef::New(DVec2::new(0.0, -radius)),
        end: PointRef::New(DVec2::new(0.0, radius)),
        construction: false,
    });
    segment(
        history,
        sketch,
        PointRef::Existing(PointId(first + 2)),
        PointRef::Existing(PointId(first + 1)),
    );
}

/// The region of a disc of `radius` about the origin between the chords at
/// `low` and at `high`: turned about V, a zone of the ball.
fn a_zone(history: &mut History, sketch: usize, radius: f64, low: f64, high: f64) {
    let across = |height: f64| (radius * radius - height * height).sqrt();
    let first = next_point(history, sketch);
    segment(
        history,
        sketch,
        PointRef::New(DVec2::new(0.0, low)),
        PointRef::New(DVec2::new(across(low), low)),
    );
    segment(
        history,
        sketch,
        PointRef::New(DVec2::new(across(high), high)),
        PointRef::New(DVec2::new(0.0, high)),
    );
    history.push(Operation::AddArc {
        sketch,
        center: PointRef::New(DVec2::ZERO),
        start: PointRef::Existing(PointId(first + 1)),
        end: PointRef::Existing(PointId(first + 2)),
        construction: false,
    });
    segment(
        history,
        sketch,
        PointRef::Existing(PointId(first + 3)),
        PointRef::Existing(PointId(first)),
    );
}

/// A tube of `radius` about `center`, its two halves meeting at its top and
/// its bottom: turned about V, a ring torus.
fn a_ring(history: &mut History, sketch: usize, center: DVec2, radius: f64) {
    let first = next_point(history, sketch);
    let top = center + DVec2::new(0.0, radius);
    let bottom = center - DVec2::new(0.0, radius);
    history.push(Operation::AddArc {
        sketch,
        center: PointRef::New(center),
        start: PointRef::New(top),
        end: PointRef::New(bottom),
        construction: false,
    });
    history.push(Operation::AddArc {
        sketch,
        center: PointRef::New(center),
        start: PointRef::Existing(PointId(first + 2)),
        end: PointRef::Existing(PointId(first + 1)),
        construction: false,
    });
}

/// A shaft of `radius` and `length`, its end rounded by the quarter circle of
/// the same radius centred on the axis: turned about V, πr²L + 2πr³/3.
fn a_rounded_shaft(history: &mut History, sketch: usize, radius: f64, length: f64) {
    let first = next_point(history, sketch);
    history.push(Operation::AddArc {
        sketch,
        center: PointRef::New(DVec2::new(0.0, length)),
        start: PointRef::New(DVec2::new(radius, length)),
        end: PointRef::New(DVec2::new(0.0, length + radius)),
        construction: false,
    });
    segment(
        history,
        sketch,
        PointRef::New(DVec2::ZERO),
        PointRef::New(DVec2::new(radius, 0.0)),
    );
    segment(
        history,
        sketch,
        PointRef::Existing(PointId(first + 4)),
        PointRef::Existing(PointId(first + 1)),
    );
    segment(
        history,
        sketch,
        PointRef::Existing(PointId(first + 2)),
        PointRef::Existing(PointId(first + 3)),
    );
}

/// A half disc of `radius` about `center`, its diameter along the line
/// through `center` parallel to V and the disc reaching towards the axis:
/// turned about V, a torus taking matter away, tangent to the wall it sits on.
fn a_half_disc_towards_the_axis(history: &mut History, sketch: usize, center: DVec2, radius: f64) {
    let first = next_point(history, sketch);
    history.push(Operation::AddArc {
        sketch,
        center: PointRef::New(center),
        start: PointRef::New(center + DVec2::new(0.0, radius)),
        end: PointRef::New(center - DVec2::new(0.0, radius)),
        construction: false,
    });
    segment(
        history,
        sketch,
        PointRef::Existing(PointId(first + 2)),
        PointRef::Existing(PointId(first + 1)),
    );
}

#[test]
fn a_ball_turned_whole_and_part_way_holds_the_arithmetics_volume() {
    for (degrees, share) in [(360.0, 1.0), (90.0, 0.25)] {
        let mut history = History::default();
        let sketch = sketch_on(&mut history, WorkPlane::XY);
        a_half_disc(&mut history, sketch, 5.0);
        turn(
            &mut history,
            sketch,
            DVec2::new(2.0, 0.0),
            ABOUT_V,
            degrees,
            ExtrusionMode::Add,
        );

        built_exactly(
            &history,
            share * 500.0 * PI / 3.0,
            &format!("a half disc of radius 5 turned {degrees}°"),
        );
    }
}

#[test]
fn a_zone_of_a_sphere_turned_whole_and_part_way_holds_the_arithmetics_volume() {
    let (radius, low, high) = (5.0f64, -3.0f64, 3.0f64);
    let zone = PI * (radius * radius * (high - low) - (high.powi(3) - low.powi(3)) / 3.0);

    for (degrees, share) in [(360.0, 1.0), (90.0, 0.25)] {
        let mut history = History::default();
        let sketch = sketch_on(&mut history, WorkPlane::XY);
        a_zone(&mut history, sketch, radius, low, high);
        turn(
            &mut history,
            sketch,
            DVec2::new(2.0, 0.0),
            ABOUT_V,
            degrees,
            ExtrusionMode::Add,
        );

        built_exactly(
            &history,
            share * zone,
            &format!("the zone of a ball of radius 5 between −3 and 3, turned {degrees}°"),
        );
    }
}

#[test]
fn a_ring_torus_turned_whole_and_part_way_holds_the_arithmetics_volume() {
    let (big, tube) = (6.0f64, 2.0f64);
    let ring = 2.0 * PI * PI * big * tube * tube;

    for (degrees, share) in [(360.0, 1.0), (90.0, 0.25)] {
        let mut history = History::default();
        let sketch = sketch_on(&mut history, WorkPlane::XY);
        a_ring(&mut history, sketch, DVec2::new(big, 3.0), tube);
        turn(
            &mut history,
            sketch,
            DVec2::new(big, 3.0),
            ABOUT_V,
            degrees,
            ExtrusionMode::Add,
        );

        built_exactly(
            &history,
            share * ring,
            &format!("a tube of radius 2 about 6 turned {degrees}°"),
        );
    }
}

#[test]
fn a_rounded_shaft_end_turned_whole_and_part_way_holds_the_arithmetics_volume() {
    let (radius, length) = (4.0f64, 10.0f64);
    let shaft = PI * radius * radius * length + 2.0 * PI * radius.powi(3) / 3.0;

    for (degrees, share) in [(360.0, 1.0), (90.0, 0.25)] {
        let mut history = History::default();
        let sketch = sketch_on(&mut history, WorkPlane::XY);
        a_rounded_shaft(&mut history, sketch, radius, length);
        turn(
            &mut history,
            sketch,
            DVec2::new(2.0, 5.0),
            ABOUT_V,
            degrees,
            ExtrusionMode::Add,
        );

        built_exactly(
            &history,
            share * shaft,
            &format!("a Ø8 shaft for 10 with a rounded end, turned {degrees}°"),
        );
    }
}

#[test]
fn a_ball_cut_by_a_plane_square_to_its_axis_holds_the_arithmetics_volume() {
    let (radius, at) = (5.0f64, 3.0f64);
    let height = radius - at;
    let cap = PI * height * height * (3.0 * radius - height) / 3.0;

    let mut history = History::default();
    let sketch = sketch_on(&mut history, WorkPlane::XY);
    a_half_disc(&mut history, sketch, radius);
    turn(
        &mut history,
        sketch,
        DVec2::new(2.0, 0.0),
        ABOUT_V,
        360.0,
        ExtrusionMode::Add,
    );
    let slab = sketch_on(
        &mut history,
        WorkPlane {
            origin: DVec3::Y * at,
            ..WorkPlane::XZ
        },
    );
    rectangle(&mut history, slab, DVec2::splat(-10.0), DVec2::splat(10.0));
    raise(&mut history, slab, DVec2::ZERO, -10.0, ExtrusionMode::Cut);

    built_exactly(
        &history,
        500.0 * PI / 3.0 - cap,
        "a ball of radius 5 less the cap above 3",
    );
}

#[test]
fn a_ball_cut_by_a_block_through_its_axis_holds_the_arithmetics_volume() {
    let mut history = History::default();
    let sketch = sketch_on(&mut history, WorkPlane::XY);
    a_half_disc(&mut history, sketch, 5.0);
    turn(
        &mut history,
        sketch,
        DVec2::new(2.0, 0.0),
        ABOUT_V,
        360.0,
        ExtrusionMode::Add,
    );
    let slab = sketch_on(&mut history, WorkPlane::YZ);
    rectangle(&mut history, slab, DVec2::splat(-10.0), DVec2::splat(10.0));
    raise(&mut history, slab, DVec2::ZERO, 10.0, ExtrusionMode::Cut);

    built_exactly(
        &history,
        250.0 * PI / 3.0,
        "a ball of radius 5 less the half the plane holding its axis keeps",
    );
}

#[test]
fn a_coaxial_bore_through_a_ball_holds_the_arithmetics_volume() {
    let (radius, bore) = (5.0f64, 2.0f64);
    let skin = (radius * radius - bore * bore).sqrt();

    let mut history = History::default();
    let sketch = sketch_on(&mut history, WorkPlane::XY);
    a_half_disc(&mut history, sketch, radius);
    turn(
        &mut history,
        sketch,
        DVec2::new(2.0, 0.0),
        ABOUT_V,
        360.0,
        ExtrusionMode::Add,
    );
    let hole = sketch_on(
        &mut history,
        WorkPlane {
            origin: DVec3::Y * -10.0,
            ..WorkPlane::XZ
        },
    );
    circle(&mut history, hole, DVec2::ZERO, bore);
    raise(&mut history, hole, DVec2::ZERO, -20.0, ExtrusionMode::Cut);

    built_exactly(
        &history,
        4.0 * PI / 3.0 * skin.powi(3),
        "a ball of radius 5 bored Ø4 through: a napkin ring",
    );
}

#[test]
fn a_coaxial_shaft_joined_to_a_ball_at_its_equator_holds_the_arithmetics_volume() {
    let (radius, length) = (5.0f64, 15.0f64);

    let mut history = History::default();
    let sketch = sketch_on(&mut history, WorkPlane::XY);
    a_half_disc(&mut history, sketch, radius);
    turn(
        &mut history,
        sketch,
        DVec2::new(2.0, 0.0),
        ABOUT_V,
        360.0,
        ExtrusionMode::Add,
    );
    let shaft = sketch_on(&mut history, WorkPlane::XZ);
    circle(&mut history, shaft, DVec2::ZERO, radius);
    raise(
        &mut history,
        shaft,
        DVec2::ZERO,
        -length,
        ExtrusionMode::Add,
    );

    built_exactly(
        &history,
        250.0 * PI / 3.0 + PI * radius * radius * length,
        "a ball of radius 5 with a Ø10 shaft raised 15 from its equator",
    );
}

#[test]
fn a_coaxial_cone_cut_from_a_ring_torus_holds_the_arithmetics_volume() {
    let mut history = History::default();
    let sketch = sketch_on(&mut history, WorkPlane::XY);
    a_ring(&mut history, sketch, DVec2::new(6.0, 3.0), 2.0);
    turn(
        &mut history,
        sketch,
        DVec2::new(6.0, 3.0),
        ABOUT_V,
        360.0,
        ExtrusionMode::Add,
    );
    let cone = sketch_on(&mut history, WorkPlane::XY);
    let first = next_point(&history, cone);
    segment(
        &mut history,
        cone,
        PointRef::New(DVec2::new(0.0, -5.0)),
        PointRef::New(DVec2::new(14.0, -5.0)),
    );
    segment(
        &mut history,
        cone,
        PointRef::Existing(PointId(first + 1)),
        PointRef::New(DVec2::new(2.0, 7.0)),
    );
    segment(
        &mut history,
        cone,
        PointRef::Existing(PointId(first + 2)),
        PointRef::New(DVec2::new(0.0, 7.0)),
    );
    segment(
        &mut history,
        cone,
        PointRef::Existing(PointId(first + 3)),
        PointRef::Existing(PointId(first)),
    );
    turn(
        &mut history,
        cone,
        DVec2::new(1.0, 0.0),
        ABOUT_V,
        360.0,
        ExtrusionMode::Cut,
    );

    // The cone's slant, x + y = 9, runs through the tube's centre circle and
    // so takes the half of the tube towards the axis: Pappus on the half disc
    // of radius 2 left about (6, 3), its centroid 4r/(3π) from the tube's
    // centre across the slant, away from the axis.
    built_exactly(
        &history,
        24.0 * PI * PI + 16.0 * 2f64.sqrt() * PI / 3.0,
        "a ring of tube radius 2 about 6 less the cone its slant halves",
    );
}

#[test]
fn a_round_bottomed_groove_cut_into_a_shaft_holds_the_arithmetics_volume() {
    let (radius, length, tube) = (10.0f64, 20.0f64, 2.0f64);
    let removed = 2.0 * PI * (radius - 4.0 * tube / (3.0 * PI)) * (PI * tube * tube / 2.0);

    let mut history = History::default();
    let sketch = sketch_on(&mut history, WorkPlane::XY);
    rectangle(
        &mut history,
        sketch,
        DVec2::ZERO,
        DVec2::new(radius, length),
    );
    turn(
        &mut history,
        sketch,
        DVec2::new(5.0, 5.0),
        ABOUT_V,
        360.0,
        ExtrusionMode::Add,
    );
    let groove = sketch_on(&mut history, WorkPlane::XY);
    a_half_disc_towards_the_axis(&mut history, groove, DVec2::new(radius, length / 2.0), tube);
    turn(
        &mut history,
        groove,
        DVec2::new(radius - 1.0, length / 2.0),
        ABOUT_V,
        360.0,
        ExtrusionMode::Cut,
    );

    built_exactly(
        &history,
        PI * radius * radius * length - removed,
        "a Ø20 shaft 20 long less a round-bottomed groove 2 deep",
    );
}

#[test]
fn a_hole_drilled_off_axis_through_a_ball_is_a_broken_step_with_its_reason() {
    let mut history = History::default();
    let sketch = sketch_on(&mut history, WorkPlane::XY);
    a_half_disc(&mut history, sketch, 5.0);
    turn(
        &mut history,
        sketch,
        DVec2::new(2.0, 0.0),
        ABOUT_V,
        360.0,
        ExtrusionMode::Add,
    );
    let hole = sketch_on(
        &mut history,
        WorkPlane {
            origin: DVec3::Y * -10.0,
            ..WorkPlane::XZ
        },
    );
    circle(&mut history, hole, DVec2::new(2.0, 0.0), 1.0);
    raise(
        &mut history,
        hole,
        DVec2::new(2.0, 0.0),
        -20.0,
        ExtrusionMode::Cut,
    );

    let part = applied_live(&history);
    assert_eq!(
        part.declined_because(last_step(&history)),
        Some(Declined::Unsupported),
        "a bore two off the ball's axis crosses its surface where the kernel cannot follow",
    );
    assert_near(
        part.body().volume(),
        500.0 * PI / 3.0,
        "the ball, as it stood before",
    );
    assert!(part.body().is_exact(), "the part stays exact");
}

#[test]
fn a_plane_at_a_slant_through_a_ball_is_a_broken_step_with_its_reason() {
    let mut history = History::default();
    let sketch = sketch_on(&mut history, WorkPlane::XY);
    a_half_disc(&mut history, sketch, 5.0);
    turn(
        &mut history,
        sketch,
        DVec2::new(2.0, 0.0),
        ABOUT_V,
        360.0,
        ExtrusionMode::Add,
    );
    let normal = DVec3::new(1.0, 1.0, 0.0).normalize();
    let u = DVec3::new(1.0, -1.0, 0.0).normalize();
    let slant = sketch_on(
        &mut history,
        WorkPlane {
            origin: DVec3::ZERO,
            u,
            v: normal.cross(u),
        },
    );
    rectangle(&mut history, slant, DVec2::splat(-20.0), DVec2::splat(20.0));
    raise(&mut history, slant, DVec2::ZERO, 10.0, ExtrusionMode::Cut);

    let part = applied_live(&history);
    assert_eq!(
        part.declined_because(last_step(&history)),
        Some(Declined::Unsupported),
        "a plane at 45° through the ball's centre is not square to its axis",
    );
    assert_near(
        part.body().volume(),
        500.0 * PI / 3.0,
        "the ball, as it stood before",
    );
    assert!(part.body().is_exact(), "the part stays exact");
}

#[test]
fn a_ball_knob_and_a_rounded_shaft_number_their_faces_as_the_flats_do() {
    type Draw = fn(&mut History, usize);
    type Place = fn() -> DVec2;
    // What the same step numbered on a part the flats computed, measured on
    // `main` at c8ac192, the last commit where a turned ellipse sent a part
    // there (#632).
    let shapes: [(&str, Draw, Place, [&[usize]; 2]); 2] = [
        (
            "a ball knob",
            |history, sketch| a_half_disc(history, sketch, 5.0),
            || DVec2::new(2.0, 0.0),
            [&[0], &[0, 1, 2]],
        ),
        (
            "a rounded shaft end",
            |history, sketch| a_rounded_shaft(history, sketch, 4.0, 10.0),
            || DVec2::new(2.0, 5.0),
            [&[0, 1, 2], &[0, 1, 2, 5, 6]],
        ),
    ];

    for (what, draw, place, on_the_flats) in shapes {
        for (degrees, on_the_flats) in [360.0, 90.0].into_iter().zip(on_the_flats) {
            let mut exact = History::default();
            let sketch = sketch_on(&mut exact, WorkPlane::XY);
            draw(&mut exact, sketch);
            turn(
                &mut exact,
                sketch,
                place(),
                ABOUT_V,
                degrees,
                ExtrusionMode::Add,
            );

            assert_eq!(
                numbered_from_the_floor(&exact),
                on_the_flats,
                "{what} turned {degrees}°: the faces number from the floor as the flats' did",
            );
        }
    }
}
