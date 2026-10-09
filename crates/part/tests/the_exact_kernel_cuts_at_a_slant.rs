//! Closes #552.
//! - a hole drilled square into a slanted face, coming out through the floor at a
//!   slant, holds the arithmetic's volume, exactly —
//!   `a_hole_drilled_square_into_a_wedge_s_slant_through_its_floor_holds_the_arithmetics_volume`
//!   (wedge (0,0),(40,0),(0,20) on XZ raised 20; Ø6 laid on the slant through its
//!   FaceAnchor at its middle, cut 30: 8000 − 45√5π)
//! - a shaft cut by a slanted plane, and a flat milled on a chamfer, hold the
//!   arithmetic's volume, exactly —
//!   `a_shaft_cut_by_a_slanted_plane_holds_the_arithmetics_volume` (Ø20 raised 30,
//!   cut above the plane through (0,0,20) at 30°: 2000π),
//!   `a_flat_milled_on_a_chamfer_holds_the_arithmetics_volume` (frustum
//!   (0,0),(10,0),(5,20),(0,20) about V, z ≥ 6 for y ∈ [2,18] removed: a hyperbola,
//!   3500π/3 − 4[F(9.5) − F(6)])
//! - a plane through a cone's apex at a slant, and a plane parallel to a ruling,
//!   are built exactly —
//!   `a_point_cut_through_its_tip_at_a_slant_holds_the_arithmetics_volume`
//!   (250π/3 − (10/3)(25·acos 0.4 − 2√21)),
//!   `a_point_cut_parallel_to_its_slope_holds_the_arithmetics_volume`
//!   (250π/3 − [10(25·acos 0.7 − 3.5√12.75) + 17√12.75]/3)
//! - the steps #526, #533 and #536 declined because they asked for a conic are built;
//!   what still declines is a skew pair or a non-conic curve, with its reason —
//!   `a_plane_at_a_slant_across_a_cone_holds_the_arithmetics_volume` (slab 2),
//!   `a_slanted_hole_crossing_a_bore_at_a_skew_angle_is_a_broken_step_with_its_reason`
//!   (8000 − 50π, `declined_because == Some(Unsupported)`, still exact); also
//!   `a_raise_meeting_a_block_at_a_slant_is_built_exactly` in
//!   `the_exact_kernel_computes_the_part.rs`
//! - #448's harness draws slanted planes against cylinders and cones before the
//!   kernel code, holes in slanted faces, slanted cuts, flats on chamfers, weighted
//!   towards hairs and tangencies; a plane tangent to a cylinder along a ruling is a
//!   line — no test: held in `cao_solid` by `random_turned_solids.rs`
//!   (`Case::drawn_tilted`, `the_tilted_generator_draws_every_kind_it_keeps`,
//!   `the_draws_before_conics_still_give_each_seed_its_case`,
//!   `a_case_asking_beyond_the_conics_is_told_from_one_that_does_not`), and by
//!   `a_plane_tangent_to_a_cylinder_along_a_ruling_a_hair_off_parallel_is_that_ruling`
//!   in `brep/relation/tests/sliced.rs` and
//!   `a_plane_touching_a_shaft_along_a_ruling_lists_that_line_and_no_conic` in
//!   `the_exact_kernel_joins_and_cuts.rs`
//! - campaigns end with every random-solid draw under one failure in a thousand on
//!   fresh seeds, no seed that held before failing — no test: counts and families in
//!   `docs/exact-kernel-failures.md` ("#552: conics"), one seed per family in
//!   `what_the_exact_campaigns_found_in_the_kernel.rs` / `…_triangles.rs`, campaigns
//!   behind `--features campaigns`, held by
//!   `the_long_campaigns_over_solids_are_compiled_only_when_asked_for`
//! - #498's eighteen cases still hold, and the gate grows by no more than a few
//!   seconds — no test: held by `a_bored_cylinder_on_two_kernels.rs`, which the gate
//!   runs; the gate's time on base and branch is in `docs/exact-kernel-failures.md`
//! - `docs/exact-kernel.md` says which conics the kernel builds and what it still
//!   declines — no test: prose, held by `language.rs` only
//!
//! A part cut at a slant, as the application builds it: a plane meets a
//! cylinder or a cone at any angle along a conic the exact kernel builds — an
//! ellipse, a parabola, a hyperbola, or two lines through a cone's tip — and
//! the part stays exact. The kernel's own tests are `cao_solid`'s, in
//! `the_exact_kernel_joins_and_cuts.rs`, `the_exact_kernel_turns.rs` and
//! `brep/relation/tests/sliced.rs`.

use std::f64::consts::PI;

use cao_part::history::{ExtrusionMode, FaceAnchor, Operation, PointRef, RevolutionAxis};
use cao_part::{History, PartDocument, PartState};
use cao_sketch::{Area, PointId, SketchAxis, WorkPlane};
use cao_solid::Declined;
use cao_solid::soundness::closed;
use glam::{DVec2, DVec3};

/// How near the arithmetic an exact volume comes: the kernel's matter is the
/// true surface, so what is left is rounding.
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

/// The number the history gave its last step.
fn last_step(history: &History) -> u32 {
    history.steps().last().expect("a step").operations()[0]
}

/// A part made by applying each operation of `history` as the user would.
fn applied_live(history: &History) -> PartDocument {
    let now = "2026-10-09T09:00:00Z".parse().expect("a date");
    let mut part = PartDocument::new("Slanted", now);
    for operation in history.operations() {
        part.apply(operation.clone());
    }
    part
}

/// The part `history` describes, applied live, held to have built its last
/// step on the exact kernel, closed, with the arithmetic's volume.
fn built_exactly(history: &History, arithmetic: f64, what: &str) {
    let part = applied_live(history);
    assert_eq!(part.declined_because(last_step(history)), None, "{what}");
    assert!(part.body().is_exact(), "{what}: the part stays exact");
    assert_eq!(closed(&part.body().triangles()), Ok(()), "{what}");
    let volume = part.body().volume();
    assert!(
        (volume - arithmetic).abs() <= EXACT * arithmetic,
        "{what}: {volume} where the arithmetic gives {arithmetic}",
    );
}

/// The plane through `origin` spanned by `u` and `v`, a raise going along
/// `u × v`.
fn plane(origin: DVec3, u: DVec3, v: DVec3) -> WorkPlane {
    WorkPlane { origin, u, v }
}

/// A wedge: the triangle (0,0), (40,0), (0,20) drawn on XZ and raised 20,
/// down Y. Its slant stands at `x + 2z = 40`.
fn a_wedge() -> History {
    let mut history = History::default();
    sketch_on(&mut history, WorkPlane::XZ);
    polygon(
        &mut history,
        0,
        &[
            DVec2::new(0.0, 0.0),
            DVec2::new(40.0, 0.0),
            DVec2::new(0.0, 20.0),
        ],
    );
    raise(
        &mut history,
        0,
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

/// A hole Ø6 drawn on the wedge's slant at its middle, laid there as a click
/// lays it — the face the ray finds, its corners and its normal — and cut 30
/// into it, square to it.
fn a_hole_drilled_into_the_slant(history: &mut History) {
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
    circle(history, sketch, center, 3.0);
    raise(history, sketch, center, -30.0, ExtrusionMode::Cut);
}

/// The hole's axis runs `5√5` from the slant's middle down to the floor,
/// which it leaves along an ellipse well inside it.
#[test]
fn a_hole_drilled_square_into_a_wedge_s_slant_through_its_floor_holds_the_arithmetics_volume() {
    let mut history = a_wedge();
    a_hole_drilled_into_the_slant(&mut history);

    built_exactly(
        &history,
        8000.0 - 45.0 * 5f64.sqrt() * PI,
        "a hole Ø6 square to the slant, out through the floor",
    );
}

/// A bore Ø4 through the wedge from its floor up at (15, −10), then the
/// hole drilled into the slant, whose wall crosses the bore's at a skew
/// angle: the second is declined, the first stands.
#[test]
fn a_slanted_hole_crossing_a_bore_at_a_skew_angle_is_a_broken_step_with_its_reason() {
    let mut history = a_wedge();
    sketch_on(&mut history, WorkPlane::XY);
    circle(&mut history, 1, DVec2::new(15.0, -10.0), 2.0);
    raise(
        &mut history,
        1,
        DVec2::new(15.0, -10.0),
        30.0,
        ExtrusionMode::Cut,
    );
    a_hole_drilled_into_the_slant(&mut history);

    let part = applied_live(&history);

    assert_eq!(
        part.declined_because(last_step(&history)),
        Some(Declined::Unsupported),
        "two cylinders at a skew angle",
    );
    let volume = part.body().volume();
    let arithmetic = 8000.0 - 50.0 * PI;
    assert!(
        (volume - arithmetic).abs() <= EXACT * arithmetic,
        "the wedge and its bore, as they stood before: {volume}",
    );
    assert!(part.body().is_exact(), "the part stays exact");
}

#[test]
fn a_shaft_cut_by_a_slanted_plane_holds_the_arithmetics_volume() {
    let mut history = History::default();
    sketch_on(&mut history, WorkPlane::XY);
    circle(&mut history, 0, DVec2::ZERO, 10.0);
    raise(&mut history, 0, DVec2::ZERO, 30.0, ExtrusionMode::Add);
    let (sin, cos) = (PI / 6.0).sin_cos();
    sketch_on(
        &mut history,
        plane(DVec3::Z * 20.0, DVec3::X, DVec3::new(0.0, cos, sin)),
    );
    rectangle(
        &mut history,
        1,
        DVec2::new(-15.0, -20.0),
        DVec2::new(15.0, 20.0),
    );
    raise(&mut history, 1, DVec2::ZERO, 20.0, ExtrusionMode::Cut);

    built_exactly(
        &history,
        2000.0 * PI,
        "a Ø20 cut at 30° through its axis at 20: its mean height",
    );
}

const ABOUT_V: RevolutionAxis = RevolutionAxis::Sketch(SketchAxis::V);

/// A profile drawn on XY through `corners` and turned whole about V, the
/// world's Y axis.
fn turned_whole(corners: &[DVec2], place: DVec2) -> History {
    let mut history = History::default();
    sketch_on(&mut history, WorkPlane::XY);
    polygon(&mut history, 0, corners);
    history.push(Operation::Revolve {
        sketch: 0,
        areas: clicked(&history, 0, place),
        axis: ABOUT_V,
        angle: 360.0_f64.into(),
        mode: ExtrusionMode::Add,
    });
    history
}

/// A frustum from Ø20 at its foot to Ø10 at 20, about Y: 3500π/3.
fn a_frustum() -> History {
    turned_whole(
        &[
            DVec2::new(0.0, 0.0),
            DVec2::new(10.0, 0.0),
            DVec2::new(5.0, 20.0),
            DVec2::new(0.0, 20.0),
        ],
        DVec2::new(2.0, 2.0),
    )
}

const FRUSTUM: f64 = 3500.0 * PI / 3.0;

/// The flat stands six from the axis, from two to eighteen along it, and
/// meets the slope along a hyperbola down to where the radius is six, at
/// sixteen: the segment beyond it, integrated over the radius, is `F`, four
/// along the axis for one of radius.
#[test]
fn a_flat_milled_on_a_chamfer_holds_the_arithmetics_volume() {
    let mut history = a_frustum();
    sketch_on(
        &mut history,
        WorkPlane {
            origin: DVec3::Z * 6.0,
            ..WorkPlane::XY
        },
    );
    rectangle(
        &mut history,
        1,
        DVec2::new(-15.0, 2.0),
        DVec2::new(15.0, 18.0),
    );
    raise(
        &mut history,
        1,
        DVec2::new(0.0, 10.0),
        5.0,
        ExtrusionMode::Cut,
    );
    let integral = |r: f64| {
        let half = (r * r - 36.0).sqrt();
        r.powi(3) / 3.0 * (6.0 / r).acos() - 4.0 * r * half + 72.0 * (r + half).ln()
    };

    built_exactly(
        &history,
        FRUSTUM - 4.0 * (integral(9.5) - integral(6.0)),
        "a flat six off the frustum's axis",
    );
}

/// A slab two thick leaning 30° off square to the frustum's axis, through
/// it at ten: it takes what lies between two ellipses, the difference of the
/// two points their planes cut off the cone, whose apex stands at forty,
/// thirty and `30 + 4/√3` from them along the axis.
#[test]
fn a_plane_at_a_slant_across_a_cone_holds_the_arithmetics_volume() {
    let mut history = a_frustum();
    let (sin, cos) = (PI / 6.0).sin_cos();
    sketch_on(
        &mut history,
        plane(DVec3::Y * 10.0, DVec3::X, DVec3::new(0.0, sin, cos)),
    );
    rectangle(&mut history, 1, DVec2::splat(-20.0), DVec2::splat(20.0));
    raise(&mut history, 1, DVec2::ZERO, 2.0, ExtrusionMode::Cut);
    let far = 30.0 + 4.0 / 3f64.sqrt();
    let between = PI / 48.0 * (far.powi(3) - 30f64.powi(3)) / (47.0f64 / 48.0).powf(1.5);

    built_exactly(
        &history,
        FRUSTUM - between,
        "a slab two thick at 30° across the frustum",
    );
}

/// A point Ø10 at its foot and ten high, about Y: 250π/3.
fn a_point() -> History {
    turned_whole(
        &[
            DVec2::new(0.0, 0.0),
            DVec2::new(5.0, 0.0),
            DVec2::new(0.0, 10.0),
        ],
        DVec2::new(1.0, 1.0),
    )
}

const POINT: f64 = 250.0 * PI / 3.0;

/// The area of the disc of radius `r` beyond a chord `d` from its centre.
fn segment(r: f64, d: f64) -> f64 {
    r * r * (d / r).acos() - d * (r * r - d * d).sqrt()
}

/// What lies on the far side of the plane through `on` holding Z and the
/// direction `down` in XY is cut away, from a rectangle drawn on that plane.
fn cut_beyond(history: &mut History, on: DVec3, down: DVec3) {
    let sketch = PartState::rebuild(history).sketches.len();
    sketch_on(history, plane(on, DVec3::Z, down.normalize()));
    rectangle(
        history,
        sketch,
        DVec2::new(-10.0, -15.0),
        DVec2::new(10.0, 15.0),
    );
    raise(history, sketch, DVec2::ZERO, 10.0, ExtrusionMode::Cut);
}

/// The plane through the tip and the chord `x = 2` of the foot, steeper than
/// the slope: it cuts two rulings, and takes the piece of the point standing
/// on the foot's segment beyond that chord.
#[test]
fn a_point_cut_through_its_tip_at_a_slant_holds_the_arithmetics_volume() {
    let mut history = a_point();
    cut_beyond(&mut history, DVec3::Y * 10.0, DVec3::new(0.2, -1.0, 0.0));

    built_exactly(
        &history,
        POINT - 10.0 / 3.0 * segment(5.0, 2.0),
        "a point cut through its tip where x > 0.2(10 − y)",
    );
}

/// The plane `x + y/2 = 3.5`, parallel to the ruling from the tip to (5, 0):
/// the tip's side goes, along a parabola, the point's volume from its tip
/// being a third of the distance to each face times that face's area — the
/// foot's segment beyond `x = 3.5`, and the parabola's two thirds of its
/// chord by its height.
#[test]
fn a_point_cut_parallel_to_its_slope_holds_the_arithmetics_volume() {
    let mut history = a_point();
    cut_beyond(&mut history, DVec3::X * 3.5, DVec3::new(0.5, -1.0, 0.0));
    let root = 12.75f64.sqrt();

    built_exactly(
        &history,
        POINT - (10.0 * segment(5.0, 3.5) + 17.0 * root) / 3.0,
        "a point cut where x > 3.5 − y/2",
    );
}
