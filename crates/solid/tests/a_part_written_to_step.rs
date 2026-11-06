//! A body written as an AP214 STEP file, read back from its text alone the
//! way another CAD program reads it, and held to what makes it one exact
//! solid.
//!
//! Closes #581.
//! - the written file is checked in a test: every reference resolves, the
//!   shell is closed, each face's loops lie on its surface within the
//!   tolerance, and the volume computed from the entities matches the
//!   kernel's — `a_block_written_to_step_reads_back_as_one_closed_solid_of_its_volume`,
//!   and every body of the next criterion the same way; the reader that
//!   checks it, `step_reader`, is held first to files written by hand in
//!   `the_reader_measures_a_box_written_by_hand`,
//!   `the_reader_measures_a_cylinder_written_by_hand`,
//!   `the_reader_measures_a_cone_written_by_hand`,
//!   `the_reader_measures_a_hemisphere_written_by_hand`,
//!   `the_reader_measures_a_ball_written_by_hand`,
//!   `the_reader_measures_a_lemon_written_by_hand`,
//!   `the_reader_refuses_a_reference_to_nothing`,
//!   `the_reader_refuses_a_shell_with_a_face_missing` and
//!   `the_reader_finds_a_loop_standing_off_its_face`. The tolerance is the
//!   one the file declares, itself no wider than the kernel's `Scale::HAIR`
//!   tolerances at the body's reach
//! - this holds for a block, a bored cylinder, a chamfered shaft (a cone), a
//!   shaft cut at a slant (a conic), a part with an elliptic wall, and a hole
//!   drilled from the side (a meet curve as a B-spline) —
//!   `a_block_written_to_step_reads_back_as_one_closed_solid_of_its_volume`,
//!   `a_bored_cylinder_written_to_step_reads_back_as_one_closed_solid_of_its_volume`,
//!   `a_chamfered_shaft_written_to_step_reads_back_as_one_closed_solid_of_its_volume`,
//!   `a_shaft_cut_at_a_slant_written_to_step_reads_back_as_one_closed_solid_of_its_volume`,
//!   `an_elliptic_post_written_to_step_reads_back_as_one_closed_solid_of_its_volume`,
//!   `a_part_drilled_from_the_side_written_to_step_reads_back_as_one_closed_solid_of_its_volume`;
//!   the conic the slant cuts is an ellipse; a cone cut along a ruling cuts
//!   a parabola and one cut steeper than its rulings a hyperbola, each on
//!   its cone and its plane all along —
//!   `a_cone_cut_along_a_ruling_written_to_step_reads_back_as_one_closed_solid_of_its_volume`,
//!   `a_cone_cut_steeper_than_its_rulings_written_to_step_reads_back_as_one_closed_solid_of_its_volume`;
//!   a parabola is a parabola whatever azimuth the ruling stands at, on two
//!   cones — `a_cone_cut_along_a_ruling_at_any_azimuth_is_written_as_its_parabola`;
//!   a cone cut a hair off a ruling, its conic's centre far beyond the part,
//!   reads back within the tolerance —
//!   `a_cone_cut_a_hair_off_a_ruling_reads_back_within_the_tolerance`;
//!   a cone widening along its axis as well as one narrowing —
//!   `a_shaft_chamfered_at_its_foot_written_to_step_reads_back_as_one_closed_solid_of_its_volume`;
//!   beyond the issue's list, since #576 brought them, a sphere and a torus —
//!   `a_turned_ball_written_to_step_reads_back_as_one_closed_solid_of_its_volume`,
//!   `a_turned_ring_written_to_step_reads_back_as_one_closed_solid_of_its_volume`;
//!   a spindle, whose major radius is below its minor and which no plain
//!   `TOROIDAL_SURFACE` holds, is written as the outer part of a
//!   `DEGENERATE_TOROIDAL_SURFACE` —
//!   `a_spindle_is_written_as_the_outer_part_of_a_degenerate_torus`; and
//!   its inner part, a rugby ball, faces out —
//!   `a_rugby_ball_reads_back_as_the_inner_part_of_a_degenerate_torus_facing_out`;
//!   a hole all but tangent to the shaft is written within the tolerance and
//!   at a size the tight spot alone does not blow up —
//!   `a_hole_within_a_tenth_of_a_micron_of_tangency_is_written_within_the_tolerance_and_no_larger_than_four_far_ones`;
//!   and the meet curve's B-spline stands within the kernel's tolerance of
//!   the kernel's own closed form —
//!   `the_meet_curve_is_written_within_the_kernel_s_tolerance_of_its_closed_form`
//! - beyond the issue, a meet curve the cap on its pieces cannot bring within
//!   the tolerance is refused rather than written out of it — no test: no
//!   construction found reaches it. The cap of 16 384 pieces was reached from
//!   a gap of 10⁻⁶ mm down to 5·10⁻⁸ mm to tangency, and on crossing walls of
//!   radii 10 and 10 − 10⁻⁶, and every fit stood within 10⁻⁸ mm of both walls
//!   against a tolerance of 6·10⁻⁷ mm; at 10⁻⁸ mm the kernel takes the walls
//!   for tangent and the curve needs few pieces
//! - a part on the flats is refused — `a_body_on_the_flats_is_not_written`;
//!   the message saying why is `cao_app`'s, held in `wording/export/tests.rs`
//! - with the step named — no test: since #632 no step sends a body to the
//!   flats, only a cache older than `REBUILT_BY = 17` held one, and it is
//!   rebuilt on opening; nothing records which step it was, so there is no
//!   step to name
//! - a drawing in units other than millimetres is written in millimetres —
//!   `a_part_drawn_in_units_of_two_and_a_half_millimetres_is_written_in_millimetres`;
//!   that the part's own scale is the one handed down is `cao_app`'s
//! - to try it: each of those parts opens in FreeCAD as one valid solid, and
//!   FreeCAD's volume matches the application's — no test: a human opens the
//!   files in FreeCAD, which nothing in this workspace runs

mod step_reader;

use std::f64::consts::{FRAC_PI_2, PI, TAU};

use cao_solid::brep::Scale;
use cao_solid::profile::{Contour, Frame, Profile, Run};
use cao_solid::turning::{Axis, Turn};
use cao_solid::{Body, Loop, step};
use glam::{DVec2, DVec3};

const GROUND: Frame = Frame {
    origin: DVec3::ZERO,
    u: DVec3::X,
    v: DVec3::Y,
};

fn exact(outline: Contour) -> Profile<'static> {
    Profile {
        exact: Some((outline, Vec::new())),
        sampled: Loop::straight(&[]),
        sampled_holes: Vec::new(),
        triangles: &[],
    }
}

fn raised(outline: Contour, frame: Frame, height: f64) -> Body {
    Body::default()
        .tool_raised(&exact(outline), frame, frame.normal() * height)
        .expect("the exact kernel raises it")
}

fn on_a_part(tool: Body) -> Body {
    Body::default()
        .union(&tool)
        .expect("the part takes the tool")
}

/// 30 by 20 by 10.
fn block() -> Body {
    on_a_part(raised(
        Contour::rectangle(DVec2::ZERO, DVec2::new(30.0, 20.0)),
        GROUND,
        10.0,
    ))
}

fn disc(center: DVec2, radius: f64) -> Contour {
    Contour {
        corners: vec![center + DVec2::X * radius],
        runs: vec![Run::Round { center, turn: TAU }],
    }
}

/// A cylinder of radius 20 standing from 0 to 10, bored through on its axis
/// with a radius of 5.
fn bored_cylinder() -> Body {
    let below = Frame {
        origin: DVec3::NEG_Z,
        ..GROUND
    };
    on_a_part(raised(disc(DVec2::ZERO, 20.0), GROUND, 10.0))
        .difference(&raised(disc(DVec2::ZERO, 5.0), below, 12.0))
        .expect("the exact kernel bores it")
}

/// A profile turned whole about the sketch's Y axis, `sampled` the drawing
/// the profile was read from.
fn turned(corners: Vec<DVec2>, runs: Vec<Run>, sampled: Vec<DVec2>) -> Body {
    let sampled: &'static [DVec2] = Box::leak(sampled.into_boxed_slice());
    let profile = Profile {
        exact: Some((Contour { corners, runs }, Vec::new())),
        sampled: Loop::straight(sampled),
        sampled_holes: Vec::new(),
        triangles: &[],
    };
    let about_y = Axis {
        origin: DVec2::ZERO,
        direction: DVec2::Y,
    };
    on_a_part(
        Body::default()
            .tool_turned(&profile, GROUND, &Turn::of(about_y, TAU, 1e-4, &profile))
            .expect("the exact kernel turns it"),
    )
}

fn arc(center: DVec2, radius: f64, from: f64, turn: f64) -> Vec<DVec2> {
    (0..=32)
        .map(|step| center + DVec2::from_angle(from + turn * f64::from(step) / 32.0) * radius)
        .collect()
}

/// A shaft of radius 10 and length 30 along Y, its far end chamfered by 2:
/// a cone between its cylinder and its end.
fn chamfered_shaft() -> Body {
    let corners = vec![
        DVec2::new(0.0, 0.0),
        DVec2::new(10.0, 0.0),
        DVec2::new(10.0, 28.0),
        DVec2::new(8.0, 30.0),
        DVec2::new(0.0, 30.0),
    ];
    turned(corners.clone(), vec![Run::Straight; 5], corners)
}

/// A cone of radius 10 on the ground and height 20 along Y, its apex up:
/// a right triangle turned about its upright side.
fn cone() -> Body {
    let corners = vec![
        DVec2::new(0.0, 0.0),
        DVec2::new(10.0, 0.0),
        DVec2::new(0.0, 20.0),
    ];
    turned(corners.clone(), vec![Run::Straight; 3], corners)
}

/// A cone of `radius` on the ground and `height` along Y, its apex up.
fn cone_of(radius: f64, height: f64) -> Body {
    let corners = vec![
        DVec2::new(0.0, 0.0),
        DVec2::new(radius, 0.0),
        DVec2::new(0.0, height),
    ];
    turned(corners.clone(), vec![Run::Straight; 3], corners)
}

/// How far `point` stands from the wall of `cone_of(radius, height)`.
fn off_the_cone_of(radius: f64, height: f64, point: DVec3) -> f64 {
    let slope = radius / height;
    let across = DVec2::new(point.x, point.z).length();
    (across - radius + point.y * slope).abs() / (1.0 + slope * slope).sqrt()
}

/// A plane through `origin` holding the directions `along` and `across`, and
/// `cone_of(radius, height)` with everything beyond it taken away.
fn cut_by_a_plane(radius: f64, height: f64, origin: DVec3, along: DVec3, across: DVec3) -> Body {
    let plane = Frame {
        origin,
        u: along,
        v: across,
    };
    cone_of(radius, height)
        .difference(&raised(
            Contour::rectangle(DVec2::splat(-40.0), DVec2::splat(40.0)),
            plane,
            30.0,
        ))
        .expect("the exact kernel cuts the cone")
}

/// The ruling of `cone_of(radius, height)` at `azimuth` about Y, its
/// direction up the cone, and the direction square to it round the axis.
fn ruling(radius: f64, height: f64, azimuth: f64) -> (DVec3, DVec3, DVec3) {
    let out = DVec3::new(azimuth.cos(), 0.0, azimuth.sin());
    let up = (DVec3::Y * height - out * radius).normalize();
    (out, up, out.cross(DVec3::Y))
}

/// `cone_of(radius, height)` cut by the plane through `(radius / 2) out` that
/// holds the ruling's direction at `azimuth`, tilted from it by `tilt` towards
/// the axis: at no tilt a parabola, its vertex at three quarters of the
/// height on the far side.
fn cut_by_a_ruling(radius: f64, height: f64, azimuth: f64, tilt: f64) -> (Body, DVec3, DVec3) {
    let (out, up, across) = ruling(radius, height, azimuth);
    let normal = up.cross(across);
    let along = up * tilt.cos() + normal * tilt.sin();
    let origin = out * radius / 2.0;
    let body = cut_by_a_plane(radius, height, origin, along, across);
    (body, origin, along.cross(across).normalize())
}

/// How far `point` stands from the cone's wall, `r = 10 - y / 2`.
fn off_the_cone(point: DVec3) -> f64 {
    let across = DVec2::new(point.x, point.z).length();
    (across - 10.0 + point.y / 2.0).abs() * 2.0 / 5f64.sqrt()
}

/// Everything of the cone on the far side of a plane through `origin`, its
/// first direction `along` and its second Z, taken away.
fn cone_cut(origin: DVec3, along: DVec3) -> Body {
    let plane = Frame {
        origin,
        u: along,
        v: DVec3::Z,
    };
    cone()
        .difference(&raised(
            Contour::rectangle(DVec2::splat(-40.0), DVec2::splat(40.0)),
            plane,
            30.0,
        ))
        .expect("the exact kernel cuts the cone")
}

/// The cone cut by the plane `2x + y = 10`, parallel to its ruling
/// `(10 - y / 2, y, 0)`: the cut runs along a parabola from the base at
/// `(5, 0, ±√75)` up to its vertex at `(-2.5, 15, 0)`.
fn cone_cut_along_a_ruling() -> Body {
    cone_cut(
        DVec3::new(5.0, 0.0, 0.0),
        DVec3::new(-1.0, 2.0, 0.0).normalize(),
    )
}

fn off_the_plane_along_a_ruling(point: DVec3) -> f64 {
    (2.0 * point.x + point.y - 10.0).abs() / 5f64.sqrt()
}

/// The cone cut by the plane `x = 4`, steeper than its rulings: the cut runs
/// along a hyperbola from the base at `(4, 0, ±√84)` up to its vertex at
/// `(4, 12, 0)`.
fn cone_cut_steeper_than_its_rulings() -> Body {
    cone_cut(DVec3::new(4.0, 0.0, 0.0), DVec3::Y)
}

/// The shaft of `chamfered_shaft` chamfered at its foot rather than its far
/// end: its cone widens along Y where the other's narrows.
fn shaft_chamfered_at_its_foot() -> Body {
    let corners = vec![
        DVec2::new(0.0, 0.0),
        DVec2::new(8.0, 0.0),
        DVec2::new(10.0, 2.0),
        DVec2::new(10.0, 30.0),
        DVec2::new(0.0, 30.0),
    ];
    turned(corners.clone(), vec![Run::Straight; 5], corners)
}

/// The cylinder of radius 20 from 0 to 10, everything above the plane
/// `z = 5 + 0.2 x` taken away: its top an ellipse, its rim a conic.
fn shaft_cut_at_a_slant() -> Body {
    let slant = Frame {
        origin: DVec3::new(0.0, 0.0, 5.0),
        u: DVec3::new(1.0, 0.0, 0.2).normalize(),
        v: DVec3::Y,
    };
    on_a_part(raised(disc(DVec2::ZERO, 20.0), GROUND, 10.0))
        .difference(&raised(
            Contour::rectangle(DVec2::splat(-40.0), DVec2::splat(40.0)),
            slant,
            30.0,
        ))
        .expect("the exact kernel cuts it")
}

/// An ellipse of semi-axes 15 and 8 raised 10.
fn elliptic_post() -> Body {
    on_a_part(raised(
        Contour {
            corners: vec![DVec2::new(15.0, 0.0)],
            runs: vec![Run::Oval {
                center: DVec2::ZERO,
                first: DVec2::new(15.0, 0.0),
                second: 8.0,
                turn: TAU,
            }],
        },
        GROUND,
        10.0,
    ))
}

/// A shaft of radius 10 standing from 0 to 30, drilled through from the side
/// along X by a hole of radius 4 about the line `y = 0, z = 15`: the two walls
/// meet along two closed curves no conic holds.
fn drilled_from_the_side() -> Body {
    let side = Frame {
        origin: DVec3::new(-15.0, 0.0, 0.0),
        u: DVec3::Y,
        v: DVec3::Z,
    };
    on_a_part(raised(disc(DVec2::ZERO, 10.0), GROUND, 30.0))
        .difference(&raised(disc(DVec2::new(0.0, 15.0), 4.0), side, 30.0))
        .expect("the exact kernel drills it")
}

/// How far `point` stands from the shaft's wall and from the hole's.
fn off_the_drilled_walls(point: DVec3) -> f64 {
    let shaft = (point.truncate().length() - 10.0).abs();
    let hole = (DVec2::new(point.y, point.z - 15.0).length() - 4.0).abs();
    shaft.max(hole)
}

/// A ball of radius 5: a half disc turned about its diameter.
fn turned_ball() -> Body {
    turned(
        vec![DVec2::new(0.0, -5.0), DVec2::new(0.0, 5.0)],
        vec![
            Run::Round {
                center: DVec2::ZERO,
                turn: PI,
            },
            Run::Straight,
        ],
        arc(DVec2::ZERO, 5.0, -FRAC_PI_2, PI),
    )
}

/// A ring from radius 2 to 6 and 4 high, its outer wall bulging by a half
/// circle of radius 2: a band of a torus between two planes and a cylinder.
fn turned_ring() -> Body {
    let center = DVec2::new(6.0, 2.0);
    let mut sampled = vec![DVec2::new(2.0, 0.0)];
    sampled.extend(arc(center, 2.0, -FRAC_PI_2, PI));
    sampled.push(DVec2::new(2.0, 4.0));
    turned(
        vec![
            DVec2::new(2.0, 0.0),
            DVec2::new(6.0, 0.0),
            DVec2::new(6.0, 4.0),
            DVec2::new(2.0, 4.0),
        ],
        vec![
            Run::Straight,
            Run::Round { center, turn: PI },
            Run::Straight,
            Run::Straight,
        ],
        sampled,
    )
}

/// An arc of radius 3 about (2, 0), from the axis back to the axis the long
/// way round, turned: a spindle, its major radius 2 below its minor 3, its
/// two apexes on the axis.
fn turned_spindle() -> Body {
    let center = DVec2::new(2.0, 0.0);
    let reach = 5f64.sqrt();
    let from = DVec2::new(-2.0, -reach).to_angle();
    let turn = (DVec2::new(-2.0, reach).to_angle() - from).rem_euclid(TAU);
    turned(
        vec![DVec2::new(0.0, -reach), DVec2::new(0.0, reach)],
        vec![Run::Round { center, turn }, Run::Straight],
        arc(center, 3.0, from, turn),
    )
}

/// An arc of radius 3 about (-2, 0), from the axis back to the axis the
/// short way, through (1, 0), turned: a rugby ball, the inner part of a
/// spindle of major radius 2, `π (44 √5 / 3 - 36 asin(√5 / 3))` of matter.
fn turned_rugby_ball() -> Body {
    let center = DVec2::new(-2.0, 0.0);
    let reach = 5f64.sqrt();
    let from = DVec2::new(2.0, -reach).to_angle();
    let turn = DVec2::new(2.0, reach).to_angle() - from;
    turned(
        vec![DVec2::new(0.0, -reach), DVec2::new(0.0, reach)],
        vec![Run::Round { center, turn }, Run::Straight],
        arc(center, 3.0, from, turn),
    )
}

/// A shaft of radius 10 standing from 0 to 30, drilled from the side along X
/// by a hole of radius 4 about `y = 6 - gap, z = 15`: the hole's wall comes
/// within `gap` of the shaft's, and the curves they meet along all but touch.
fn drilled_within(gap: f64) -> Body {
    let side = Frame {
        origin: DVec3::new(-15.0, 0.0, 0.0),
        u: DVec3::Y,
        v: DVec3::Z,
    };
    on_a_part(raised(disc(DVec2::ZERO, 10.0), GROUND, 30.0))
        .difference(&raised(disc(DVec2::new(6.0 - gap, 15.0), 4.0), side, 30.0))
        .expect("the exact kernel drills it")
}

/// A disc of radius 20 drawn as 32 flat sides, raised 10 by the flats: the
/// drawing handed no runs.
fn flats_disc() -> Body {
    let corners: &'static [DVec2] = Box::leak(
        (0..32)
            .map(|index| DVec2::from_angle(index as f64 * TAU / 32.0) * 20.0)
            .collect::<Vec<_>>()
            .into_boxed_slice(),
    );
    let triangles: &'static [[DVec2; 3]] = Box::leak(
        (1..corners.len() - 1)
            .map(|index| [corners[0], corners[index], corners[index + 1]])
            .collect::<Vec<_>>()
            .into_boxed_slice(),
    );
    let profile = Profile {
        exact: None,
        sampled: Loop::straight(corners),
        sampled_holes: Vec::new(),
        triangles,
    };
    Body::default()
        .tool_raised(&profile, GROUND, DVec3::Z * 10.0)
        .expect("the flats never decline")
}

/// The kernel's widest tolerance at the body's reach, in millimetres: what
/// the kernel lets a surface move by, and so what a file of it may declare.
fn kernel_tolerance(body: &Body, millimetres_per_unit: f64) -> f64 {
    let (low, high) = body.bounds().expect("the body holds matter");
    let reach = low.abs().max(high.abs()).max_element();
    Scale::HAIR * Scale::of(reach).eps() * millimetres_per_unit
}

fn read(text: &str) -> step_reader::Solid {
    step_reader::read(text).unwrap_or_else(|fault| panic!("the file does not read back: {fault}"))
}

/// The body written at `millimetres_per_unit`, read back as one closed solid
/// whose loops lie on their faces within the tolerance the file declares,
/// that tolerance no wider than the kernel's, the file in millimetres, and
/// the matter its entities enclose the kernel's — within the tolerance times
/// the area, which is all a surface moved by the tolerance can change.
fn reads_back_exact(body: &Body, millimetres_per_unit: f64) -> step_reader::Solid {
    let text = step::of(body, millimetres_per_unit).expect("an exact body is written");
    let solid = read(&text);
    let tolerance = kernel_tolerance(body, millimetres_per_unit);
    assert!(
        solid.uncertainty <= tolerance * (1.0 + 1e-12),
        "the file declares points one under {} mm, the kernel only under {tolerance} mm",
        solid.uncertainty
    );
    assert!(
        solid.worst <= solid.uncertainty,
        "a loop stands {} off its face or its edge, the file's tolerance is {}",
        solid.worst,
        solid.uncertainty
    );
    assert_eq!(
        solid.metres_per_unit, 1e-3,
        "the file is not in millimetres"
    );
    let kernel = body.volume() * millimetres_per_unit.powi(3);
    assert!(
        (solid.volume - kernel).abs() <= tolerance * solid.area * (1.0 + 1e-12),
        "the file encloses {} mm³, the kernel {kernel} mm³ — {} apart, {} allowed",
        solid.volume,
        (solid.volume - kernel).abs(),
        tolerance * solid.area
    );
    solid
}

#[test]
fn a_block_written_to_step_reads_back_as_one_closed_solid_of_its_volume() {
    let solid = reads_back_exact(&block(), 1.0);

    assert_eq!(solid.faces, 6, "a block has six faces");
}

#[test]
fn a_bored_cylinder_written_to_step_reads_back_as_one_closed_solid_of_its_volume() {
    reads_back_exact(&bored_cylinder(), 1.0);
}

#[test]
fn a_chamfered_shaft_written_to_step_reads_back_as_one_closed_solid_of_its_volume() {
    let shaft = chamfered_shaft();
    let text = step::of(&shaft, 1.0).expect("the shaft is written");

    reads_back_exact(&shaft, 1.0);
    assert!(
        text.contains("CONICAL_SURFACE"),
        "the chamfer is written on its cone"
    );
}

#[test]
fn a_shaft_chamfered_at_its_foot_written_to_step_reads_back_as_one_closed_solid_of_its_volume() {
    let shaft = shaft_chamfered_at_its_foot();
    let text = step::of(&shaft, 1.0).expect("the shaft is written");

    reads_back_exact(&shaft, 1.0);
    assert!(
        text.contains("CONICAL_SURFACE"),
        "the chamfer is written on its cone"
    );
}

/// Every point of every conic edge of `kind` on both the cone and the plane
/// that cut it, and the highest of them at the vertex `top`.
fn conic_holds(
    points: &[Vec<DVec3>],
    off_the_plane: impl Fn(DVec3) -> f64,
    off_the_cone: impl Fn(DVec3) -> f64,
    top: f64,
    tolerance: f64,
    kind: &str,
) {
    assert!(!points.is_empty(), "no {kind} is read from the file");
    let all: Vec<DVec3> = points.iter().flatten().copied().collect();
    for point in &all {
        let off = off_the_cone(*point).max(off_the_plane(*point));
        assert!(
            off <= tolerance,
            "a point of the {kind} stands {off} mm off the cone or the plane it is the meet of, \
             the kernel's tolerance is {tolerance} mm: {point}"
        );
    }
    let highest = all
        .iter()
        .map(|point| point.y)
        .fold(f64::NEG_INFINITY, f64::max);
    assert!(
        (highest - top).abs() <= tolerance,
        "the {kind} rises to {highest}, its vertex stands at {top}"
    );
}

#[test]
fn a_cone_cut_along_a_ruling_written_to_step_reads_back_as_one_closed_solid_of_its_volume() {
    let cut = cone_cut_along_a_ruling();
    let text = step::of(&cut, 1.0).expect("the cut cone is written");

    let solid = reads_back_exact(&cut, 1.0);
    assert!(
        text.contains("PARABOLA("),
        "the cut is written as a parabola"
    );
    conic_holds(
        &solid.along_parabolas,
        off_the_plane_along_a_ruling,
        off_the_cone,
        15.0,
        kernel_tolerance(&cut, 1.0),
        "parabola",
    );
}

#[test]
fn a_cone_cut_steeper_than_its_rulings_written_to_step_reads_back_as_one_closed_solid_of_its_volume()
 {
    let cut = cone_cut_steeper_than_its_rulings();
    let text = step::of(&cut, 1.0).expect("the cut cone is written");

    let solid = reads_back_exact(&cut, 1.0);
    assert!(
        text.contains("HYPERBOLA("),
        "the cut is written as a hyperbola"
    );
    conic_holds(
        &solid.along_hyperbolas,
        |point| (point.x - 4.0).abs(),
        off_the_cone,
        12.0,
        kernel_tolerance(&cut, 1.0),
        "hyperbola",
    );
}

#[test]
fn a_cone_cut_along_a_ruling_at_any_azimuth_is_written_as_its_parabola() {
    for (radius, height, degrees) in [(10.0, 20.0, 37.0), (10.0, 20.0, 45.0), (7.0, 14.0, 61.0)] {
        let (cut, origin, normal) = cut_by_a_ruling(radius, height, f64::to_radians(degrees), 0.0);
        let text = step::of(&cut, 1.0).expect("the cut cone is written");

        let solid = reads_back_exact(&cut, 1.0);
        assert!(
            text.contains("PARABOLA("),
            "the {radius} by {height} cone cut along its ruling at {degrees}° is written \
             with no parabola"
        );
        conic_holds(
            &solid.along_parabolas,
            |point| (point - origin).dot(normal).abs(),
            |point| off_the_cone_of(radius, height, point),
            0.75 * height,
            kernel_tolerance(&cut, 1.0),
            &format!("parabola at {degrees}°"),
        );
    }
}

#[test]
fn a_cone_cut_a_hair_off_a_ruling_reads_back_within_the_tolerance() {
    for tilt in [1e-6, 1e-9, -1e-9] {
        let (cut, _, _) = cut_by_a_ruling(10.0, 20.0, f64::to_radians(37.0), tilt);

        reads_back_exact(&cut, 1.0);
    }
}

#[test]
fn a_rugby_ball_reads_back_as_the_inner_part_of_a_degenerate_torus_facing_out() {
    let ball = turned_rugby_ball();
    let reach = 5f64.sqrt();
    let volume = PI * (44.0 * reach / 3.0 - 36.0 * (reach / 3.0).asin());
    assert!(
        (ball.volume() - volume).abs() <= 1e-9 * volume,
        "the kernel holds {}, the arithmetic {volume}",
        ball.volume()
    );
    let text = step::of(&ball, 1.0).expect("the rugby ball is written");

    let solid = reads_back_exact(&ball, 1.0);
    assert!(solid.volume > 0.0, "the ball reads back inside out");
    assert!(
        text.lines()
            .any(|line| line.contains("DEGENERATE_TOROIDAL_SURFACE") && line.ends_with(",.F.);")),
        "the rugby ball is not written as the inner part of a degenerate torus"
    );
}

#[test]
fn a_hole_within_a_tenth_of_a_micron_of_tangency_is_written_within_the_tolerance_and_no_larger_than_four_far_ones()
 {
    let near = drilled_within(1e-4);
    let far = step::of(&drilled_within(1.0), 1.0).expect("the hole a millimetre off is written");
    let text = step::of(&near, 1.0).expect("the hole a tenth of a micron off is written");

    reads_back_exact(&near, 1.0);
    assert!(
        text.len() <= 4 * far.len(),
        "a hole a tenth of a micron from tangency writes {} bytes, one a millimetre off {}: \
         the meet curve is refined everywhere for one tight spot",
        text.len(),
        far.len()
    );
}

#[test]
fn a_shaft_cut_at_a_slant_written_to_step_reads_back_as_one_closed_solid_of_its_volume() {
    let shaft = shaft_cut_at_a_slant();
    let text = step::of(&shaft, 1.0).expect("the shaft is written");

    reads_back_exact(&shaft, 1.0);
    assert!(
        text.contains("ELLIPSE("),
        "the rim the slant cuts on the cylinder is written as its ellipse"
    );
}

#[test]
fn an_elliptic_post_written_to_step_reads_back_as_one_closed_solid_of_its_volume() {
    let post = elliptic_post();
    let text = step::of(&post, 1.0).expect("the post is written");

    reads_back_exact(&post, 1.0);
    assert!(
        text.contains("SURFACE_OF_LINEAR_EXTRUSION"),
        "the elliptic wall is written as an ellipse swept along its axis"
    );
}

#[test]
fn a_part_drilled_from_the_side_written_to_step_reads_back_as_one_closed_solid_of_its_volume() {
    let solid = reads_back_exact(&drilled_from_the_side(), 1.0);

    assert!(
        solid.along_splines.len() >= 2,
        "the hole meets the shaft along two curves, written as {} B-splines",
        solid.along_splines.len()
    );
}

#[test]
fn the_meet_curve_is_written_within_the_kernel_s_tolerance_of_its_closed_form() {
    let drilled = drilled_from_the_side();
    let tolerance = kernel_tolerance(&drilled, 1.0);
    let solid = read(&step::of(&drilled, 1.0).expect("the part is written"));

    assert!(!solid.along_splines.is_empty(), "no meet curve is written");
    for (index, points) in solid.along_splines.iter().enumerate() {
        let worst = points
            .iter()
            .map(|point| off_the_drilled_walls(*point))
            .fold(0.0, f64::max);
        assert!(
            worst <= tolerance,
            "B-spline {index} strays {worst} mm off the walls it is the meet of, \
             the kernel's tolerance is {tolerance} mm"
        );
    }
}

#[test]
fn a_turned_ball_written_to_step_reads_back_as_one_closed_solid_of_its_volume() {
    let ball = turned_ball();
    let text = step::of(&ball, 1.0).expect("the ball is written");

    reads_back_exact(&ball, 1.0);
    assert!(
        text.contains("SPHERICAL_SURFACE"),
        "the ball is written on its sphere"
    );
}

#[test]
fn a_turned_ring_written_to_step_reads_back_as_one_closed_solid_of_its_volume() {
    let ring = turned_ring();
    let text = step::of(&ring, 1.0).expect("the ring is written");

    reads_back_exact(&ring, 1.0);
    assert!(
        text.contains("TOROIDAL_SURFACE"),
        "the bulge is written on its torus"
    );
}

#[test]
fn a_spindle_is_written_as_the_outer_part_of_a_degenerate_torus() {
    let spindle = turned_spindle();
    let text = step::of(&spindle, 1.0).expect("the spindle is written");

    reads_back_exact(&spindle, 1.0);
    assert!(
        text.contains("DEGENERATE_TOROIDAL_SURFACE"),
        "a torus whose major radius is below its minor is the degenerate kind"
    );
}

#[test]
fn a_body_on_the_flats_is_not_written() {
    let flats = flats_disc();
    assert!(!flats.is_exact(), "the disc is on the flats");

    assert_eq!(step::of(&flats, 1.0), Err(step::Faceted));
}

#[test]
fn a_part_drawn_in_units_of_two_and_a_half_millimetres_is_written_in_millimetres() {
    let solid = reads_back_exact(&block(), 2.5);

    let near = 1e-9 * 75.0;
    assert!(
        solid.low.abs().max_element() <= near
            && (solid.high - DVec3::new(75.0, 50.0, 25.0))
                .abs()
                .max_element()
                <= near,
        "a block of 30 by 20 by 10 units of 2.5 mm spans {} to {} mm",
        solid.low,
        solid.high
    );
    let volume = 75.0 * 50.0 * 25.0;
    assert!(
        (solid.volume - volume).abs() <= 1e-9 * volume,
        "it encloses {} mm³, not {volume}",
        solid.volume
    );
}

#[test]
fn the_reader_measures_a_box_written_by_hand() {
    let solid = read(&step_reader::hand_written::box_with_its_top_at(5.0));

    assert_eq!(solid.faces, 6);
    assert!(
        solid.worst < 1e-12,
        "the box stands {} off itself",
        solid.worst
    );
    assert!(
        (solid.volume - 24.0).abs() < 1e-9,
        "the box holds {}",
        solid.volume
    );
    assert!(
        (solid.area - 52.0).abs() < 1e-9,
        "the box's area is {}",
        solid.area
    );
}

#[test]
fn the_reader_measures_a_cylinder_written_by_hand() {
    let solid = read(&step_reader::hand_written::cylinder());

    assert_eq!(solid.faces, 3);
    assert!(
        solid.worst < 1e-12,
        "the cylinder stands {} off itself",
        solid.worst
    );
    let volume = PI * 4.0 * 5.0;
    assert!(
        (solid.volume - volume).abs() < 1e-9,
        "the cylinder holds {}, not {volume}",
        solid.volume
    );
    let area = PI * 28.0;
    assert!(
        (solid.area - area).abs() < 1e-9,
        "the cylinder's area is {}, not {area}",
        solid.area
    );
}

#[test]
fn the_reader_finds_the_cylinder_a_wall_written_by_hand_stands_on() {
    let solid = read(&step_reader::hand_written::cylinder());

    let [(origin, axis, radius)] = solid.cylinders[..] else {
        panic!("one cylinder, under the wall: {:?}", solid.cylinders);
    };
    assert!(
        origin.distance(DVec3::new(1.0, 1.0, 0.0)) < 1e-12
            && axis.distance(DVec3::Z) < 1e-12
            && (radius - 2.0).abs() < 1e-12,
        "the wall stands on the cylinder of radius 2 about the upright through (1, 1, 0), \
         the file says {radius} about {origin} along {axis}"
    );
}

fn measures(text: &str, volume: f64, area: f64) {
    let solid = read(text);

    assert!(solid.worst < 1e-12, "it stands {} off itself", solid.worst);
    assert!(
        (solid.volume - volume).abs() < 1e-9,
        "it holds {}, not {volume}",
        solid.volume
    );
    assert!(
        (solid.area - area).abs() < 1e-9,
        "its area is {}, not {area}",
        solid.area
    );
}

#[test]
fn the_reader_measures_a_cone_written_by_hand() {
    measures(&step_reader::hand_written::cone(), 12.0 * PI, 24.0 * PI);
}

#[test]
fn the_reader_measures_a_hemisphere_written_by_hand() {
    measures(
        &step_reader::hand_written::hemisphere(),
        16.0 * PI / 3.0,
        12.0 * PI,
    );
}

#[test]
fn the_reader_measures_a_ball_written_by_hand() {
    measures(
        &step_reader::hand_written::ball(),
        32.0 * PI / 3.0,
        16.0 * PI,
    );
}

#[test]
fn the_reader_measures_a_lemon_written_by_hand() {
    let solid = read(&step_reader::hand_written::lemon());

    let reach = 5f64.sqrt();
    let volume = PI * (44.0 * reach / 3.0 - 36.0 * (reach / 3.0).asin());
    assert!(solid.worst < 1e-12, "it stands {} off itself", solid.worst);
    assert!(
        (solid.volume - volume).abs() < 1e-9,
        "it holds {}, not {volume}",
        solid.volume
    );
}

#[test]
fn the_reader_refuses_a_reference_to_nothing() {
    let text = step_reader::hand_written::box_with_its_top_at(5.0).replacen(
        "ENDSEC;\nEND-ISO",
        "#9000=VERTEX_POINT('',#9001);\nENDSEC;\nEND-ISO",
        1,
    );

    let fault = step_reader::read(&text).expect_err("#9001 names nothing");

    assert!(fault.contains("#9000 names #9001"), "{fault}");
}

#[test]
fn the_reader_refuses_a_shell_with_a_face_missing() {
    let text = step_reader::hand_written::box_with_its_top_at(5.0);
    let at =
        text.find("CLOSED_SHELL('',(").expect("the box has a shell") + "CLOSED_SHELL('',(".len();
    let first = text[at..].find(',').expect("the shell has six faces") + 1;
    let text = format!("{}{}", &text[..at], &text[at + first..]);

    let fault = step_reader::read(&text).expect_err("five faces enclose nothing");

    assert!(fault.contains("the shell is not closed"), "{fault}");
}

#[test]
fn the_reader_finds_a_loop_standing_off_its_face() {
    let solid = read(&step_reader::hand_written::box_with_its_top_at(5.001));

    assert!(
        (solid.worst - 0.001).abs() < 1e-9,
        "the top loop stands {} off its plane, not 0.001",
        solid.worst
    );
}
