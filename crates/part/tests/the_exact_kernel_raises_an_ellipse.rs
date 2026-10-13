//! Closes #571.
//! - an elliptic boss raised on a plate, and an elliptic pocket and an elliptic
//!   hole cut through one, hold the arithmetic's volume, exactly; a Ø40 raised
//!   afterwards holds its own, not a 48-gon's —
//!   `an_elliptic_boss_raised_on_a_plate_holds_the_arithmetics_volume` (πab·h),
//!   `an_elliptic_pocket_and_an_elliptic_hole_cut_into_a_plate_hold_the_arithmetics_volume`
//!   (the pocket turned 30°, the hole's first half-axis its minor one),
//!   `a_circle_forty_across_raised_after_an_ellipse_holds_its_own_volume`
//! - a profile mixing arcs of ellipse with traits and arcs of circle — an
//!   ellipse cut by a trait, a slot with elliptic ends — is raised exactly;
//!   every face keeps the number the flats give it today, and two arcs of one
//!   ellipse make one wall answering to both —
//!   `an_ellipse_cut_by_a_trait_is_raised_exactly_and_numbers_its_faces_as_the_flats_do`
//!   (cut by one trait and by two, the faces made counted from the floor
//!   against the same step on a part already on the flats),
//!   `a_slot_with_elliptic_ends_is_raised_exactly`,
//!   `the_areas_an_ellipse_and_a_circle_overlapping_part_are_raised_exactly`
//!   (the lens and the crescent, each a circle's segment and an ellipse's),
//!   `two_arcs_of_one_ellipse_make_one_wall_answering_to_both` (the side of
//!   an ellipse beyond a trait, parted at the end of its axis),
//!   `an_ellipse_trimmed_between_two_traits_is_raised_exactly` (two pieces a
//!   cut left of one ellipse, in one outline); every elliptic
//!   leaf of the harness's draws is held to the flats' numbers in `cao_solid`
//!   by `an_elliptic_leaf_names_its_faces_as_the_flats_name_them`
//! - a block's side crossing or touching an elliptic wall, and round and
//!   elliptic walls parallel to it, crossing at up to four rulings or touching
//!   along one, are built exactly and hold their volume —
//!   `a_block_s_side_crossing_or_touching_an_elliptic_boss_holds_the_arithmetics_volume`,
//!   `a_round_hole_through_an_elliptic_boss_crossing_it_at_four_rulings_holds_the_arithmetics_volume`
//!   (4[(r²/2)·φ₀ + (ab/2)·(π/2 − t₀)] removed),
//!   `two_elliptic_walls_crossing_at_four_rulings_hold_the_arithmetics_volume`
//!   (2400·atan(2/3) removed),
//!   `an_elliptic_pocket_touching_an_elliptic_boss_s_wall_inside_holds_the_arithmetics_volume`
//! - a plane at a slant through an elliptic boss, and an elliptic hole drilled
//!   square into a slanted face coming out through the floor, are built
//!   exactly; a lean under decision 11's bound is laid square or along, one in
//!   the warning band is built and noted unstable, as on a circular wall —
//!   `an_elliptic_post_cut_by_a_slanted_plane_holds_the_arithmetics_volume`,
//!   `an_elliptic_hole_drilled_square_into_a_wedge_s_slant_through_its_floor_holds_the_arithmetics_volume`,
//!   `an_elliptic_post_cut_a_ten_millionth_of_a_degree_off_square_is_laid_square_and_not_noted`,
//!   `an_elliptic_post_cut_by_a_slice_turned_a_hundredth_of_a_degree_is_built_and_its_step_noted_unstable`
//! - a round hole drilled from the side through an elliptic boss, and an
//!   elliptic hole drilled from the side through a shaft, are built exactly
//!   along the curve the walls meet —
//!   `a_round_hole_drilled_from_the_side_through_an_elliptic_boss_holds_the_arithmetics_volume`,
//!   `an_elliptic_hole_drilled_from_the_side_through_a_shaft_holds_the_arithmetics_volume`
//!   (equal spans, two ellipses: 16ab²/3; a narrower one integrated)
//! - a whole ellipse whose half-axes are equal within decision 8's hair is
//!   built as a circular cylinder —
//!   `a_whole_ellipse_ten_tolerances_from_round_is_raised_on_a_circular_wall`.
//!   Reworded: the issue says "within the tolerance". Tom widened it on
//!   2026-10-13 to decision 8's hair for a whole ellipse, the hair being below
//!   any step the software shows (#514 holds the user's precision at 0.1 µm,
//!   the hair is 2 nm on a 100 mm part); an arc of ellipse keeps the
//!   tolerance, held in `cao_solid` by
//!   `an_ellipse_whose_half_axes_are_one_within_the_tolerance_is_raised_as_a_circular_cylinder`.
//!   The test is ignored until the lane building the hair lands.
//! - what still declines is an elliptic wall against a cone or a skew wall,
//!   where the faces may cross, with its reason: a broken step, never the
//!   flats —
//!   `an_elliptic_hole_crossing_a_bore_at_a_skew_angle_is_a_broken_step_with_its_reason`,
//!   `an_elliptic_hole_through_a_cone_about_its_axis_is_a_broken_step_with_its_reason`
//! - #448's harness draws ellipses before the kernel code: raised and cut, whole
//!   and in arcs, beside and through blocks, round and elliptic walls parallel
//!   and square to them, slanted planes, weighted towards hairs and tangencies
//!   — no test: held in `cao_solid` by `random_exact_solids.rs`
//!   (`Case::drawn_ellipses`, `…_crossed`, `…_tilted`,
//!   `the_ellipse_generators_draw_every_kind_they_keep`,
//!   `the_draws_before_ellipses_still_give_each_seed_its_case`)
//! - campaigns end with the ellipse draws under one failure in a thousand on
//!   fresh seeds, no seed of the other draws that held before failing; a family
//!   that does not move in two rounds is reported and set aside — no test:
//!   counts and families in `docs/exact-kernel-failures.md` ("#571: elliptic
//!   walls"), the campaigns behind `--features campaigns`; the draws' fast
//!   cases are in the gate, `random_ellipse_cases_keep_every_rule_on_the_exact_kernel`
//!   and `ellipse_cases_the_exact_kernel_keeps_are_kept_through_the_application_s_body_and_stay_exact`
//! - the geometry cache rebuilds a part holding an ellipse (`REBUILT_BY`
//!   moves); the elliptic wall is drawn from its own surface, divided as every
//!   exact wall is, and a click reads its true normal — no test: held beside
//!   the cache by `a_cache_written_before_ellipses_were_exact_is_replayed`,
//!   since only there can a test tell a part opened on its cache from one
//!   replayed; beside the circle's by
//!   `a_raised_ellipse_is_drawn_lit_and_picked_from_its_true_elliptic_wall` in
//!   `the_exact_kernel_computes_the_part.rs`, and in `cao_app` by
//!   `a_click_on_an_exact_elliptic_wall_lands_on_its_one_curved_face`
//! - `docs/exact-kernel.md` and `docs/glossary.md` say what the kernel builds
//!   on an elliptic wall and what it still declines, and no longer send an
//!   ellipse raised to the flats — no test: prose, held by `language.rs` only
//! - a turn of a profile holding an ellipse stays on the flats (what must not
//!   break) — `an_ellipse_turned_is_computed_by_the_flats`; a part already on
//!   the flats raises an ellipse on the flats, held by
//!   `an_ellipse_raised_after_a_turned_arc_is_computed_by_the_flats` in
//!   `the_exact_kernel_computes_the_part.rs`
//!
//! A part holding an ellipse, as the application builds it: the elliptic
//! wall is the exact kernel's, and so is every step after it. The kernel's
//! own tests are `cao_solid`'s, beside the code in `brep/` and in
//! `random_exact_solids.rs`.

use std::f64::consts::{FRAC_PI_2, PI};

use cao_part::history::{ExtrusionMode, FaceAnchor, Operation, PointRef, RevolutionAxis};
use cao_part::{History, PartDocument, PartState};
use cao_sketch::{Area, EllipseId, PointId, SketchAxis, WorkPlane};
use cao_solid::soundness::closed;
use cao_solid::{Declined, Lean};
use glam::{DVec2, DVec3};

/// How near the arithmetic an exact volume comes: the kernel's matter is the
/// true surface, so what is left is rounding.
const EXACT: f64 = 1e-9;

fn clicked(history: &History, sketch: usize, places: &[DVec2]) -> Vec<Area> {
    PartState::rebuild(history).areas_at(sketch, places)
}

fn sketch_on(history: &mut History, plane: WorkPlane) -> usize {
    let sketch = PartState::rebuild(history).sketches.len();
    history.push(Operation::CreateSketch { plane, on: None });
    sketch
}

/// The plane through `origin` spanned by `u` and `v`, a raise going along
/// `u × v`.
fn plane(origin: DVec3, u: DVec3, v: DVec3) -> WorkPlane {
    WorkPlane { origin, u, v }
}

fn level(height: f64) -> WorkPlane {
    plane(DVec3::Z * height, DVec3::X, DVec3::Y)
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

/// The number the next point drawn in `sketch` is given.
fn next_point(history: &History, sketch: usize) -> usize {
    PartState::rebuild(history).sketches[sketch].points().len()
}

/// An ellipse about `center`, its first half-axis `first` and its second
/// `second` long a quarter turn anticlockwise from it. The points it brings
/// follow its centre: the two ends of the first axis, then of the second.
fn ellipse(history: &mut History, sketch: usize, center: DVec2, first: DVec2, second: f64) {
    let across = first.perp().normalize() * second;
    history.push(Operation::AddEllipse {
        sketch,
        center: PointRef::New(center),
        first: [PointRef::New(center - first), PointRef::New(center + first)],
        second: [
            PointRef::New(center - across),
            PointRef::New(center + across),
        ],
        construction: false,
        drawn: None,
    });
}

fn segment(history: &mut History, sketch: usize, start: PointRef, end: PointRef) {
    history.push(Operation::AddSegment {
        sketch,
        start,
        end,
        construction: false,
    });
}

fn raise_areas(
    history: &mut History,
    sketch: usize,
    places: &[DVec2],
    distance: f64,
    mode: ExtrusionMode,
) {
    history.push(Operation::Extrude {
        sketch,
        areas: clicked(history, sketch, places),
        distance: distance.into(),
        mode,
    });
}

fn raise(history: &mut History, sketch: usize, place: DVec2, distance: f64, mode: ExtrusionMode) {
    raise_areas(history, sketch, &[place], distance, mode);
}

/// The number the history gave its last step.
fn last_step(history: &History) -> u32 {
    history.steps().last().expect("a step").operations()[0]
}

/// A part made by applying each operation of `history` as the user would.
fn applied_live(history: &History) -> PartDocument {
    let now = "2026-10-13T09:00:00Z".parse().expect("a date");
    let mut part = PartDocument::new("Elliptic", now);
    for operation in history.operations() {
        part.apply(operation.clone());
    }
    part
}

/// The part `history` describes, applied live, held to have built its last
/// step on the exact kernel, closed, within `room` of the arithmetic's
/// volume.
fn built_within(history: &History, arithmetic: f64, room: f64, what: &str) -> PartDocument {
    let part = applied_live(history);
    assert_eq!(part.declined_because(last_step(history)), None, "{what}");
    assert!(part.body().is_exact(), "{what}: the part stays exact");
    assert_eq!(closed(&part.body().triangles()), Ok(()), "{what}");
    let volume = part.body().volume();
    assert!(
        (volume - arithmetic).abs() <= room * arithmetic,
        "{what}: {volume} where the arithmetic gives {arithmetic}",
    );
    part
}

fn built_exactly(history: &History, arithmetic: f64, what: &str) -> PartDocument {
    built_within(history, arithmetic, EXACT, what)
}

/// The part `history` describes held to have declined its last step with
/// `reason`, standing exact as it stood before it with `before`'s volume.
fn broken_with(history: &History, reason: Declined, before: f64, what: &str) {
    let part = applied_live(history);
    assert_eq!(
        part.declined_because(last_step(history)),
        Some(reason),
        "{what}"
    );
    assert!(part.body().is_exact(), "{what}: never the flats");
    let volume = part.body().volume();
    assert!(
        (volume - before).abs() <= EXACT * before,
        "{what}: the part as it stood before, {volume} against {before}",
    );
}

/// The area of an ellipse of half-axes `a` and `b` beyond a chord `c` from
/// its centre, square to the half-axis `a`.
fn elliptic_segment(a: f64, b: f64, c: f64) -> f64 {
    let along = c / a;
    a * b * (along.acos() - along * (1.0 - along * along).sqrt())
}

/// What `f` adds up to from `from` to `to`, by Simpson's rule: the
/// integrands below are smooth and periodic once their square roots are
/// taken through an angle, so it is exact to rounding.
fn integral(f: impl Fn(f64) -> f64, from: f64, to: f64) -> f64 {
    let steps = 4096;
    let width = (to - from) / steps as f64;
    let inner: f64 = (1..steps)
        .map(|step| {
            let weight = if step % 2 == 1 { 4.0 } else { 2.0 };
            weight * f(from + width * step as f64)
        })
        .sum();
    (f(from) + inner + f(to)) * width / 3.0
}

/// A plate 100 by 80, 10 thick, about the origin on XY.
fn a_plate() -> History {
    let mut history = History::default();
    sketch_on(&mut history, WorkPlane::XY);
    rectangle(
        &mut history,
        0,
        DVec2::new(-50.0, -40.0),
        DVec2::new(50.0, 40.0),
    );
    raise(&mut history, 0, DVec2::ZERO, 10.0, ExtrusionMode::Add);
    history
}

const PLATE: f64 = 80_000.0;

/// An ellipse of half-axes 30 along X and 20 drawn on the plate's top and
/// raised 5 above it.
fn a_plate_with_an_elliptic_boss() -> History {
    let mut history = a_plate();
    let sketch = sketch_on(&mut history, level(10.0));
    ellipse(&mut history, sketch, DVec2::ZERO, DVec2::X * 30.0, 20.0);
    raise(&mut history, sketch, DVec2::ZERO, 5.0, ExtrusionMode::Add);
    history
}

#[test]
fn an_elliptic_boss_raised_on_a_plate_holds_the_arithmetics_volume() {
    let history = a_plate_with_an_elliptic_boss();

    built_exactly(
        &history,
        PLATE + PI * 600.0 * 5.0,
        "an ellipse 60 by 40 raised 5 on the plate",
    );
}

#[test]
fn an_elliptic_pocket_and_an_elliptic_hole_cut_into_a_plate_hold_the_arithmetics_volume() {
    let mut history = a_plate();
    let sketch = sketch_on(&mut history, level(10.0));
    let center = DVec2::new(-20.0, 0.0);
    ellipse(
        &mut history,
        sketch,
        center,
        DVec2::from_angle(PI / 6.0) * 12.0,
        7.0,
    );
    raise(&mut history, sketch, center, -4.0, ExtrusionMode::Cut);
    let pocket = PI * 12.0 * 7.0 * 4.0;
    built_exactly(&history, PLATE - pocket, "a pocket turned 30°, 4 deep");

    let sketch = sketch_on(&mut history, level(10.0));
    let center = DVec2::new(25.0, 0.0);
    ellipse(&mut history, sketch, center, DVec2::X * 10.0, 15.0);
    raise(&mut history, sketch, center, -20.0, ExtrusionMode::Cut);

    built_exactly(
        &history,
        PLATE - pocket - PI * 150.0 * 10.0,
        "the pocket, and a hole 20 by 30 right through",
    );
}

#[test]
fn a_circle_forty_across_raised_after_an_ellipse_holds_its_own_volume() {
    let mut history = a_plate_with_an_elliptic_boss();
    let before = applied_live(&history).body().volume();
    let sketch = sketch_on(&mut history, WorkPlane::XY);
    let center = DVec2::new(0.0, 100.0);
    circle(&mut history, sketch, center, 20.0);
    raise(&mut history, sketch, center, 10.0, ExtrusionMode::Add);

    let part = built_exactly(
        &history,
        before + PI * 400.0 * 10.0,
        "a Ø40 raised 10 after the elliptic boss, a true circle",
    );
    assert!(
        (part.body().volume() - before - PI * 4000.0).abs() <= EXACT * PI * 4000.0,
        "the Ø40 holds its own volume, not a 48-gon's",
    );
}

/// An ellipse 60 by 40 about the origin, the four points its axes give it
/// on it, and a trait standing at each `x` of `at` from below it to above
/// it.
fn an_ellipse_cut_by_traits(history: &mut History, at: &[f64]) -> usize {
    let sketch = sketch_on(history, WorkPlane::XY);
    ellipse(history, sketch, DVec2::ZERO, DVec2::X * 30.0, 20.0);
    for x in at {
        segment(
            history,
            sketch,
            PointRef::New(DVec2::new(*x, -30.0)),
            PointRef::New(DVec2::new(*x, 30.0)),
        );
    }
    sketch
}

/// A round segment, an arc and its chord, turned whole about the sketch's V
/// axis fifty from the origin: a band of a torus, which only the flats make.
fn turn_a_torus(history: &mut History) {
    let sketch = sketch_on(history, WorkPlane::XY);
    let first = next_point(history, sketch);
    history.push(Operation::AddArc {
        sketch,
        center: PointRef::New(DVec2::new(50.0, 5.0)),
        start: PointRef::New(DVec2::new(55.0, 0.0)),
        end: PointRef::New(DVec2::new(55.0, 10.0)),
        construction: false,
    });
    segment(
        history,
        sketch,
        PointRef::Existing(PointId(first + 2)),
        PointRef::Existing(PointId(first + 1)),
    );
    history.push(Operation::Revolve {
        sketch,
        areas: clicked(history, sketch, &[DVec2::new(56.0, 5.0)]),
        axis: RevolutionAxis::Sketch(SketchAxis::V),
        angle: 360.0.into(),
        mode: ExtrusionMode::Add,
    });
}

/// The faces the last step of `history` made, counted from the first: the
/// numbers its raise gave, whatever the steps before it gave.
fn numbered_from_the_floor(history: &History) -> Vec<usize> {
    let made = applied_live(history).faces_made_by(last_step(history));
    made.iter().map(|face| face - made[0]).collect()
}

/// The side beyond one trait, its arc parted at the end of the major axis,
/// and the band between two, each of its arcs parted at an end of the minor
/// axis: the same step on a part the flats compute numbers the same faces.
#[test]
fn an_ellipse_cut_by_a_trait_is_raised_exactly_and_numbers_its_faces_as_the_flats_do() {
    let beyond = elliptic_segment(30.0, 20.0, 10.0);
    for (traits, place, area) in [
        (&[10.0][..], DVec2::new(20.0, 0.0), beyond),
        (&[-10.0, 10.0][..], DVec2::ZERO, PI * 600.0 - 2.0 * beyond),
    ] {
        let mut exact = History::default();
        let sketch = an_ellipse_cut_by_traits(&mut exact, traits);
        raise(&mut exact, sketch, place, 10.0, ExtrusionMode::Add);
        let mut flats = History::default();
        let sketch = an_ellipse_cut_by_traits(&mut flats, traits);
        turn_a_torus(&mut flats);
        let torus = applied_live(&flats).body().volume();
        raise(&mut flats, sketch, place, 10.0, ExtrusionMode::Add);

        built_exactly(&exact, area * 10.0, "the area the traits leave");
        let on_the_flats = applied_live(&flats);
        assert!(!on_the_flats.body().is_exact(), "the torus sent it there");
        assert!(
            on_the_flats.body().volume() > torus,
            "the flats raised the step too"
        );
        assert_eq!(
            numbered_from_the_floor(&exact),
            numbered_from_the_floor(&flats),
            "the faces the flats number, cut by traits at {traits:?}",
        );
    }
}

#[test]
fn a_slot_with_elliptic_ends_is_raised_exactly() {
    let mut history = History::default();
    let sketch = sketch_on(&mut history, WorkPlane::XY);
    let ends: Vec<usize> = [-20.0, 20.0]
        .into_iter()
        .map(|x| {
            let center = next_point(&history, sketch);
            ellipse(
                &mut history,
                sketch,
                DVec2::new(x, 0.0),
                DVec2::X * 10.0,
                8.0,
            );
            center
        })
        .collect();
    let south = |end: usize| PointRef::Existing(PointId(ends[end] + 3));
    let north = |end: usize| PointRef::Existing(PointId(ends[end] + 4));
    segment(&mut history, sketch, north(0), north(1));
    segment(&mut history, sketch, south(0), south(1));
    for (ellipse, between) in [(0, [3, 4]), (1, [4, 3])] {
        history.push(Operation::TrimEllipse {
            sketch,
            ellipse: EllipseId(ellipse),
            between: Some((
                PointId(ends[ellipse] + between[0]),
                PointId(ends[ellipse] + between[1]),
            )),
        });
    }
    raise(&mut history, sketch, DVec2::ZERO, 10.0, ExtrusionMode::Add);

    built_exactly(
        &history,
        (PI * 80.0 + 40.0 * 16.0) * 10.0,
        "two half ellipses 20 by 16 joined by two traits 40 long",
    );
}

/// An ellipse 60 by 40 about the origin, and a circle of radius 12 about
/// the end of its major axis: they cross where `x = 25.1…`, and part a lens
/// and a crescent inside the ellipse.
#[test]
fn the_areas_an_ellipse_and_a_circle_overlapping_part_are_raised_exactly() {
    let crossing = (108.0 - (108.0f64 * 108.0 - 4.0 * 2080.8).sqrt()) / 2.0;
    let disc = |chord: f64| 144.0 * (chord / 12.0).acos() - chord * (144.0 - chord * chord).sqrt();
    let lens = disc(30.0 - crossing) + elliptic_segment(30.0, 20.0, crossing);
    for (place, area, what) in [
        (DVec2::new(28.0, 0.0), lens, "the lens"),
        (DVec2::new(-10.0, 0.0), PI * 600.0 - lens, "the crescent"),
    ] {
        let mut history = History::default();
        let sketch = sketch_on(&mut history, WorkPlane::XY);
        ellipse(&mut history, sketch, DVec2::ZERO, DVec2::X * 30.0, 20.0);
        circle(&mut history, sketch, DVec2::new(30.0, 0.0), 12.0);
        raise(&mut history, sketch, place, 10.0, ExtrusionMode::Add);

        built_exactly(&history, area * 10.0, what);
    }
}

/// An ellipse 60 by 40 whose top and bottom were trimmed off between two
/// traits `10√3` from its centre, where they meet it at `x = ±15`: two
/// pieces the cuts left of the one ellipse, and two traits.
#[test]
fn an_ellipse_trimmed_between_two_traits_is_raised_exactly() {
    let mut history = History::default();
    let sketch = sketch_on(&mut history, WorkPlane::XY);
    ellipse(&mut history, sketch, DVec2::ZERO, DVec2::X * 30.0, 20.0);
    let height = 10.0 * 3f64.sqrt();
    let first = next_point(&history, sketch);
    for y in [height, -height] {
        segment(
            &mut history,
            sketch,
            PointRef::New(DVec2::new(15.0 * y.signum(), y)),
            PointRef::New(DVec2::new(-15.0 * y.signum(), y)),
        );
    }
    for between in [[0, 1], [2, 3]] {
        history.push(Operation::TrimEllipse {
            sketch,
            ellipse: EllipseId(0),
            between: Some((PointId(first + between[0]), PointId(first + between[1]))),
        });
    }
    raise(&mut history, sketch, DVec2::ZERO, 10.0, ExtrusionMode::Add);

    built_exactly(
        &history,
        (400.0 * PI + 300.0 * 3f64.sqrt()) * 10.0,
        "the ellipse less the two caps beyond the traits",
    );
}

/// The side of the ellipse beyond the trait runs through the end of its
/// major axis, a point of the drawing, which parts it into two arcs.
#[test]
fn two_arcs_of_one_ellipse_make_one_wall_answering_to_both() {
    let mut history = History::default();
    let sketch = an_ellipse_cut_by_traits(&mut history, &[10.0]);
    raise(
        &mut history,
        sketch,
        DVec2::new(20.0, 0.0),
        10.0,
        ExtrusionMode::Add,
    );

    let part = applied_live(&history);
    let body = part.body();
    let hit = |y: f64| {
        body.ray_hit(DVec3::new(60.0, y, 5.0), DVec3::NEG_X)
            .expect("the wall is there")
    };
    let (above, below) = (hit(8.0), hit(-8.0));
    assert_eq!(above.face, below.face, "one wall either side of the axis");
    let wall: Vec<[DVec3; 3]> = body.triangles_of(above.face).collect();
    let answering: Vec<usize> = part
        .faces_made_by(last_step(&history))
        .into_iter()
        .filter(|face| body.triangles_of(*face).collect::<Vec<_>>() == wall)
        .collect();
    assert_eq!(answering.len(), 2, "the wall answers to {answering:?}");
}

/// A block 40 by 20, 10 high, and an ellipse 20 by 12 drawn about
/// `(x, 10)`, raised 15 from the block's floor.
fn a_block_and_an_elliptic_boss(x: f64) -> History {
    let mut history = History::default();
    sketch_on(&mut history, WorkPlane::XY);
    rectangle(&mut history, 0, DVec2::ZERO, DVec2::new(40.0, 20.0));
    raise(
        &mut history,
        0,
        DVec2::new(5.0, 5.0),
        10.0,
        ExtrusionMode::Add,
    );
    let sketch = sketch_on(&mut history, WorkPlane::XY);
    let center = DVec2::new(x, 10.0);
    ellipse(&mut history, sketch, center, DVec2::X * 10.0, 6.0);
    raise(&mut history, sketch, center, 15.0, ExtrusionMode::Add);
    history
}

#[test]
fn a_block_s_side_crossing_or_touching_an_elliptic_boss_holds_the_arithmetics_volume() {
    let half = PI * 60.0 / 2.0;
    built_exactly(
        &a_block_and_an_elliptic_boss(40.0),
        8000.0 + half * 5.0 + half * 15.0,
        "the boss standing half in the block across its side",
    );
    built_exactly(
        &a_block_and_an_elliptic_boss(30.0),
        8000.0 + 2.0 * half * 5.0,
        "the boss inside the block, touching its side along a ruling",
    );
}

/// An ellipse 60 by 40 raised 10 on XY, cut through by a tool drawn by
/// `draw` on a sketch five under it and raised 20.
fn an_elliptic_boss_cut_through(draw: impl FnOnce(&mut History, usize)) -> History {
    let mut history = History::default();
    let sketch = sketch_on(&mut history, WorkPlane::XY);
    ellipse(&mut history, sketch, DVec2::ZERO, DVec2::X * 30.0, 20.0);
    raise(&mut history, sketch, DVec2::ZERO, 10.0, ExtrusionMode::Add);
    let sketch = sketch_on(&mut history, level(-5.0));
    draw(&mut history, sketch);
    raise(&mut history, sketch, DVec2::ZERO, 20.0, ExtrusionMode::Cut);
    history
}

#[test]
fn a_round_hole_through_an_elliptic_boss_crossing_it_at_four_rulings_holds_the_arithmetics_volume()
{
    let (a, b, r) = (30.0f64, 20.0f64, 25.0f64);
    let history = an_elliptic_boss_cut_through(|history, sketch| {
        circle(history, sketch, DVec2::ZERO, r);
    });
    let x = (a * a * (r * r - b * b) / (a * a - b * b)).sqrt();
    let y = (b * b * (a * a - r * r) / (a * a - b * b)).sqrt();
    let (polar, eccentric) = (y.atan2(x), (y / b).atan2(x / a));
    let removed = 4.0 * (r * r / 2.0 * polar + a * b / 2.0 * (FRAC_PI_2 - eccentric));

    built_exactly(
        &history,
        (PI * a * b - removed) * 10.0,
        "the boss less a round hole Ø50 about its centre",
    );
}

#[test]
fn two_elliptic_walls_crossing_at_four_rulings_hold_the_arithmetics_volume() {
    let history = an_elliptic_boss_cut_through(|history, sketch| {
        ellipse(history, sketch, DVec2::ZERO, DVec2::X * 20.0, 30.0);
    });

    built_exactly(
        &history,
        (PI * 600.0 - 2400.0 * (2.0f64 / 3.0).atan()) * 10.0,
        "the boss less the same ellipse turned a quarter",
    );
}

/// The pocket 30 by 16 about `(15, 0)` reaches the boss's wall at the end of
/// its major axis, bending tighter than it there.
#[test]
fn an_elliptic_pocket_touching_an_elliptic_boss_s_wall_inside_holds_the_arithmetics_volume() {
    let mut history = History::default();
    let sketch = sketch_on(&mut history, WorkPlane::XY);
    ellipse(&mut history, sketch, DVec2::ZERO, DVec2::X * 30.0, 20.0);
    raise(&mut history, sketch, DVec2::ZERO, 10.0, ExtrusionMode::Add);
    let sketch = sketch_on(&mut history, level(10.0));
    let center = DVec2::new(15.0, 0.0);
    ellipse(&mut history, sketch, center, DVec2::X * 15.0, 8.0);
    raise(&mut history, sketch, center, -4.0, ExtrusionMode::Cut);

    built_exactly(
        &history,
        PI * 600.0 * 10.0 - PI * 120.0 * 4.0,
        "the boss less a pocket 4 deep touching its wall",
    );
}

/// An elliptic post, half-axes 10 along X and 6, raised 30 on XY.
fn an_elliptic_post(history: &mut History) {
    let sketch = sketch_on(history, WorkPlane::XY);
    ellipse(history, sketch, DVec2::ZERO, DVec2::X * 10.0, 6.0);
    raise(history, sketch, DVec2::ZERO, 30.0, ExtrusionMode::Add);
}

/// The post cut above a plane through (0, 0, 20) leaning `degrees` about X
/// off square to it.
fn an_elliptic_post_cut_leaning(degrees: f64) -> History {
    let mut history = History::default();
    an_elliptic_post(&mut history);
    let (sin, cos) = degrees.to_radians().sin_cos();
    let sketch = sketch_on(
        &mut history,
        plane(DVec3::Z * 20.0, DVec3::X, DVec3::new(0.0, cos, sin)),
    );
    rectangle(
        &mut history,
        sketch,
        DVec2::new(-15.0, -20.0),
        DVec2::new(15.0, 20.0),
    );
    raise(&mut history, sketch, DVec2::ZERO, 20.0, ExtrusionMode::Cut);
    history
}

const POST_UNDER_TWENTY: f64 = 20.0 * PI * 60.0;

#[test]
fn an_elliptic_post_cut_by_a_slanted_plane_holds_the_arithmetics_volume() {
    built_exactly(
        &an_elliptic_post_cut_leaning(30.0),
        POST_UNDER_TWENTY,
        "the post cut at 30° through its axis at 20: its mean height",
    );
}

#[test]
fn an_elliptic_post_cut_a_ten_millionth_of_a_degree_off_square_is_laid_square_and_not_noted() {
    let history = an_elliptic_post_cut_leaning(1e-7);

    let part = built_exactly(&history, POST_UNDER_TWENTY, "the post cut square at 20");
    assert_eq!(
        part.unstable_at(last_step(&history)),
        None,
        "a lean decision 11 lays exactly is not unstable",
    );
}

/// A rectangle on XZ, its foot on the sketch's horizontal axis, turned a
/// hundredth of a degree about it: its second side leans that much off
/// along the post's axis, and the slice between them is cut away.
#[test]
fn an_elliptic_post_cut_by_a_slice_turned_a_hundredth_of_a_degree_is_built_and_its_step_noted_unstable()
 {
    let mut history = History::default();
    an_elliptic_post(&mut history);
    let sketch = sketch_on(&mut history, WorkPlane::XZ);
    rectangle(
        &mut history,
        sketch,
        DVec2::new(-20.0, 0.0),
        DVec2::new(20.0, 40.0),
    );
    history.push(Operation::Revolve {
        sketch,
        areas: clicked(&history, sketch, &[DVec2::new(0.0, 20.0)]),
        axis: RevolutionAxis::Sketch(SketchAxis::U),
        angle: 0.01_f64.into(),
        mode: ExtrusionMode::Cut,
    });

    let slice = 20.0 * 30.0 * 30.0 / 2.0 * 0.01_f64.to_radians().tan();
    let part = built_exactly(
        &history,
        30.0 * PI * 60.0 - slice,
        "the post less a slice of 0.01°",
    );
    let Some(unstable) = part.unstable_at(last_step(&history)) else {
        panic!("the step is noted unstable");
    };
    assert_eq!(unstable.lean, Lean::Along);
    assert!(
        (unstable.degrees - 0.01).abs() <= 1e-9,
        "noted at {}°",
        unstable.degrees,
    );
}

/// A wedge: the triangle (0,0), (40,0), (0,20) drawn on XZ and raised 20,
/// down Y. Its slant stands at `x + 2z = 40`.
fn a_wedge() -> History {
    let mut history = History::default();
    let sketch = sketch_on(&mut history, WorkPlane::XZ);
    let corners = [
        DVec2::new(0.0, 0.0),
        DVec2::new(40.0, 0.0),
        DVec2::new(0.0, 20.0),
    ];
    let first = next_point(&history, sketch);
    segment(
        &mut history,
        sketch,
        PointRef::New(corners[0]),
        PointRef::New(corners[1]),
    );
    segment(
        &mut history,
        sketch,
        PointRef::Existing(PointId(first + 1)),
        PointRef::New(corners[2]),
    );
    segment(
        &mut history,
        sketch,
        PointRef::Existing(PointId(first + 2)),
        PointRef::Existing(PointId(first)),
    );
    raise(
        &mut history,
        sketch,
        DVec2::new(10.0, 5.0),
        20.0,
        ExtrusionMode::Add,
    );
    history
}

/// The middle of the wedge's slant, and the slant's outward normal.
const SLANT_MIDDLE: DVec3 = DVec3::new(20.0, -10.0, 10.0);

fn slant_normal() -> DVec3 {
    DVec3::new(1.0, 0.0, 2.0).normalize()
}

/// An ellipse 8 by 4 drawn on the wedge's slant at its middle, laid there as
/// a click lays it, and cut 30 into it, square to it.
fn an_elliptic_hole_drilled_into_the_slant(history: &mut History) {
    let state = PartState::rebuild(history);
    let hit = state
        .body
        .ray_hit(SLANT_MIDDLE + slant_normal() * 10.0, -slant_normal())
        .expect("the slant is there");
    let corners = state.body.plane_of(hit.face).expect("a flat face").corners;
    let sketch = state.sketches.len();
    history.push(Operation::CreateSketch {
        plane: WorkPlane::from_face(&corners, hit.normal, DVec3::Y),
        on: Some(FaceAnchor {
            face: hit.face,
            up: DVec3::Y,
        }),
    });
    let laid = PartState::rebuild(history).sketches[sketch].plane;
    let center = laid.to_local(SLANT_MIDDLE);
    ellipse(history, sketch, center, DVec2::X * 4.0, 2.0);
    raise(history, sketch, center, -30.0, ExtrusionMode::Cut);
}

/// The hole's axis runs `5√5` from the slant's middle down to the floor,
/// which it leaves along an ellipse well inside it.
#[test]
fn an_elliptic_hole_drilled_square_into_a_wedge_s_slant_through_its_floor_holds_the_arithmetics_volume()
 {
    let mut history = a_wedge();
    an_elliptic_hole_drilled_into_the_slant(&mut history);

    built_exactly(
        &history,
        8000.0 - 40.0 * 5f64.sqrt() * PI,
        "an elliptic hole 8 by 4 square to the slant, out through the floor",
    );
}

/// An elliptic boss, half-axes 10 along X and 6, raised 20 on XY, bored
/// along X through its middle by a hole of radius `r`, centred ten up.
fn an_elliptic_boss_bored_from_the_side(r: f64) -> History {
    let mut history = History::default();
    let sketch = sketch_on(&mut history, WorkPlane::XY);
    ellipse(&mut history, sketch, DVec2::ZERO, DVec2::X * 10.0, 6.0);
    raise(&mut history, sketch, DVec2::ZERO, 20.0, ExtrusionMode::Add);
    let sketch = sketch_on(&mut history, plane(DVec3::X * -20.0, DVec3::Y, DVec3::Z));
    let center = DVec2::new(0.0, 10.0);
    circle(&mut history, sketch, center, r);
    raise(&mut history, sketch, center, 40.0, ExtrusionMode::Cut);
    history
}

/// At `y` the bore and the boss share a rectangle `2·10·√(1 − y²/36)` long
/// and `2√(r² − y²)` high; at `r = 6`, the boss's half-width, that adds up
/// to `16ab²/3`.
#[test]
fn a_round_hole_drilled_from_the_side_through_an_elliptic_boss_holds_the_arithmetics_volume() {
    let boss = PI * 60.0 * 20.0;
    built_exactly(
        &an_elliptic_boss_bored_from_the_side(6.0),
        boss - 16.0 * 10.0 * 36.0 / 3.0,
        "a bore of the boss's half-width, meeting it along two ellipses",
    );
    let r = 4.0f64;
    let shared = integral(
        |t: f64| 40.0 * r * r * t.cos().powi(2) * (1.0 - (r * t.sin() / 6.0).powi(2)).sqrt(),
        -FRAC_PI_2,
        FRAC_PI_2,
    );
    built_exactly(
        &an_elliptic_boss_bored_from_the_side(r),
        boss - shared,
        "a narrower bore, meeting it along two loops",
    );
}

/// A shaft of radius 10 raised 40 on XY, bored along X through its middle
/// by an elliptic hole reaching `p` across it and 6 up and down, centred at
/// 20.
fn a_shaft_bored_from_the_side_by_an_ellipse(p: f64) -> History {
    let mut history = History::default();
    let sketch = sketch_on(&mut history, WorkPlane::XY);
    circle(&mut history, sketch, DVec2::ZERO, 10.0);
    raise(&mut history, sketch, DVec2::ZERO, 40.0, ExtrusionMode::Add);
    let sketch = sketch_on(&mut history, plane(DVec3::X * -20.0, DVec3::Y, DVec3::Z));
    let center = DVec2::new(0.0, 20.0);
    ellipse(&mut history, sketch, center, DVec2::X * p, 6.0);
    raise(&mut history, sketch, center, 40.0, ExtrusionMode::Cut);
    history
}

#[test]
fn an_elliptic_hole_drilled_from_the_side_through_a_shaft_holds_the_arithmetics_volume() {
    let shaft = PI * 100.0 * 40.0;
    built_exactly(
        &a_shaft_bored_from_the_side_by_an_ellipse(10.0),
        shaft - 16.0 * 6.0 * 100.0 / 3.0,
        "an elliptic hole as wide as the shaft, meeting it along two ellipses",
    );
    let p = 7.0f64;
    let shared = integral(
        |t: f64| 24.0 * p * t.cos().powi(2) * (100.0 - (p * t.sin()).powi(2)).sqrt(),
        -FRAC_PI_2,
        FRAC_PI_2,
    );
    built_exactly(
        &a_shaft_bored_from_the_side_by_an_ellipse(p),
        shaft - shared,
        "a narrower elliptic hole, meeting it along two loops",
    );
}

/// An ellipse 40 across one way and ten tolerances more the other — the
/// tolerance a billionth of the 20 its raise reaches — raised 10, then a
/// Ø40 about the same centre raised 15 and joined to it. Raised on an
/// elliptic wall, the two walls stand within the hair of each other all
/// round, which the kernel declines; on a circular one they are one wall.
#[test]
#[ignore = "#571: a whole ellipse within the hair of round is raised as a circle once lane c lands"]
fn a_whole_ellipse_ten_tolerances_from_round_is_raised_on_a_circular_wall() {
    let mut history = History::default();
    let sketch = sketch_on(&mut history, WorkPlane::XY);
    ellipse(
        &mut history,
        sketch,
        DVec2::ZERO,
        DVec2::X * 20.0,
        20.0 + 10.0 * 20.0 * 1e-9,
    );
    raise(&mut history, sketch, DVec2::ZERO, 10.0, ExtrusionMode::Add);
    let sketch = sketch_on(&mut history, WorkPlane::XY);
    circle(&mut history, sketch, DVec2::ZERO, 20.0);
    raise(&mut history, sketch, DVec2::ZERO, 15.0, ExtrusionMode::Add);

    built_within(
        &history,
        PI * 400.0 * 15.0,
        1e-7,
        "the near-round boss and a Ø40 of its size, one wall",
    );
}

/// A bore Ø4 through the wedge from its floor up at (15, −10), then the
/// elliptic hole drilled into the slant, whose wall crosses the bore's at a
/// skew angle.
#[test]
fn an_elliptic_hole_crossing_a_bore_at_a_skew_angle_is_a_broken_step_with_its_reason() {
    let mut history = a_wedge();
    let sketch = sketch_on(&mut history, WorkPlane::XY);
    circle(&mut history, sketch, DVec2::new(15.0, -10.0), 2.0);
    raise(
        &mut history,
        sketch,
        DVec2::new(15.0, -10.0),
        30.0,
        ExtrusionMode::Cut,
    );
    an_elliptic_hole_drilled_into_the_slant(&mut history);

    broken_with(
        &history,
        Declined::Unsupported,
        8000.0 - 50.0 * PI,
        "an elliptic wall and a round one at a skew angle",
    );
}

/// A frustum from Ø20 at its foot to Ø10 at 20, turned about Y, and an
/// ellipse 16 by 6 about its axis cut down through it: the elliptic wall
/// crosses the cone's.
#[test]
fn an_elliptic_hole_through_a_cone_about_its_axis_is_a_broken_step_with_its_reason() {
    let mut history = History::default();
    let sketch = sketch_on(&mut history, WorkPlane::XY);
    let first = next_point(&history, sketch);
    let corners = [
        DVec2::new(0.0, 0.0),
        DVec2::new(10.0, 0.0),
        DVec2::new(5.0, 20.0),
        DVec2::new(0.0, 20.0),
    ];
    segment(
        &mut history,
        sketch,
        PointRef::New(corners[0]),
        PointRef::New(corners[1]),
    );
    for corner in &corners[2..] {
        let last = next_point(&history, sketch) - 1;
        segment(
            &mut history,
            sketch,
            PointRef::Existing(PointId(last)),
            PointRef::New(*corner),
        );
    }
    let last = next_point(&history, sketch) - 1;
    segment(
        &mut history,
        sketch,
        PointRef::Existing(PointId(last)),
        PointRef::Existing(PointId(first)),
    );
    history.push(Operation::Revolve {
        sketch,
        areas: clicked(&history, sketch, &[DVec2::new(2.0, 2.0)]),
        axis: RevolutionAxis::Sketch(SketchAxis::V),
        angle: 360.0_f64.into(),
        mode: ExtrusionMode::Add,
    });
    let sketch = sketch_on(&mut history, plane(DVec3::Y * 25.0, DVec3::X, DVec3::Z));
    ellipse(&mut history, sketch, DVec2::ZERO, DVec2::X * 8.0, 3.0);
    raise(&mut history, sketch, DVec2::ZERO, 30.0, ExtrusionMode::Cut);

    broken_with(
        &history,
        Declined::Unsupported,
        3500.0 * PI / 3.0,
        "an elliptic wall about a cone's axis, crossing its slope",
    );
}

/// An ellipse 10 by 6 about (50, 5) on XY, turned a quarter about the
/// sketch's V axis: an elliptic torus, which only the flats make. Pappus
/// gives its volume, the flats a little less.
#[test]
fn an_ellipse_turned_is_computed_by_the_flats() {
    let mut history = History::default();
    let sketch = sketch_on(&mut history, WorkPlane::XY);
    let center = DVec2::new(50.0, 5.0);
    ellipse(&mut history, sketch, center, DVec2::X * 5.0, 3.0);
    history.push(Operation::Revolve {
        sketch,
        areas: clicked(&history, sketch, &[center]),
        axis: RevolutionAxis::Sketch(SketchAxis::V),
        angle: 90.0_f64.into(),
        mode: ExtrusionMode::Add,
    });

    let part = applied_live(&history);

    assert_eq!(part.declined_because(last_step(&history)), None);
    assert!(!part.body().is_exact(), "the elliptic torus is the flats'");
    let pappus = PI * 15.0 * FRAC_PI_2 * 50.0;
    let volume = part.body().volume();
    assert!(
        volume < pappus && volume > 0.98 * pappus,
        "{volume} against Pappus's {pappus}",
    );
}
