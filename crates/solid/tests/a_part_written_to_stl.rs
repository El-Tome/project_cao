//! A body written as a binary STL, read back from the bytes alone the way a
//! slicer reads it, and held to the rules every solid keeps.
//!
//! Closes #580.
//! - a block, a bored cylinder, a cone, a part with conics and a part with an
//!   elliptic wall export, and read back from the file as closed and
//!   uncrossed — `a_block_written_to_stl_reads_back_closed_and_uncrossed`,
//!   `a_bored_cylinder_written_to_stl_reads_back_closed_and_uncrossed`,
//!   `a_cone_written_to_stl_reads_back_closed_and_uncrossed`,
//!   `a_shaft_cut_at_a_slant_written_to_stl_reads_back_closed_and_uncrossed`,
//!   `an_elliptic_post_written_to_stl_reads_back_closed_and_uncrossed`
//! - the volume read back from the file is within the level's gap times the
//!   area of the arithmetic's —
//!   `the_volume_read_back_is_within_the_level_s_gap_times_the_arithmetic_s_area`
//! - the three levels give the gaps they promise, and a finer level gives more
//!   triangles —
//!   `every_triangle_read_back_stands_within_the_level_s_gap_of_the_true_surface`,
//!   `a_finer_level_writes_more_triangles`; which gap each level asks for is
//!   `cao_app`'s, held in `commands/tests.rs`
//! - a drawing in units other than millimetres is written in millimetres —
//!   `a_part_drawn_in_units_of_two_and_a_half_millimetres_is_written_in_millimetres`;
//!   that the part's own scale is the one handed down is held in
//!   `cao_app`'s `commands/tests.rs`
//! - a body the triangles cannot cover at the level asked is not written —
//!   `a_body_the_triangles_cannot_cover_is_not_written`; the message saying so
//!   is `cao_app`'s, held in `wording/export/tests.rs`
//! - to try it: the file opens in a slicer, PrusaSlicer or Cura, at the right
//!   size — no test: a human opens the file in a slicer, which nothing in this
//!   workspace runs

use std::f64::consts::TAU;

use cao_solid::profile::{Contour, Frame, Profile, Run};
use cao_solid::soundness::{Triangle, closed, enclosed, uncrossed};
use cao_solid::turning::{Axis, Turn};
use cao_solid::{Body, Loop, stl};
use glam::{DVec2, DVec3, Vec3};

/// « Grossière », « Normale » and « Fine », in millimetres.
const LEVELS: [f64; 3] = [0.1, 0.02, 0.005];

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

fn disc(center: DVec2, radius: f64) -> Contour {
    Contour {
        corners: vec![center + DVec2::X * radius],
        runs: vec![Run::Round { center, turn: TAU }],
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

/// A cone of radius 10 and height 20, turned about the sketch's Y axis.
fn cone() -> Body {
    let corners: &'static [DVec2] = Box::leak(Box::new([
        DVec2::new(0.0, 0.0),
        DVec2::new(10.0, 0.0),
        DVec2::new(0.0, 20.0),
    ]));
    let triangles: &'static [[DVec2; 3]] =
        Box::leak(Box::new([[corners[0], corners[1], corners[2]]]));
    let profile = Profile {
        exact: Some((Contour::straight(corners.to_vec()), Vec::new())),
        sampled: Loop::straight(corners),
        sampled_holes: Vec::new(),
        triangles,
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

/// The triangles a binary STL holds, read from its bytes alone: an 80-byte
/// header, the count of triangles, then fifty bytes each — the normal, the
/// three corners, two bytes of nothing — every number a little-endian `f32`.
/// Each normal is held to the corners' winding, which is how a slicer tells
/// the outside.
fn read(bytes: &[u8]) -> Vec<Triangle> {
    assert!(
        bytes.len() >= 84,
        "a binary STL opens on 84 bytes, this one has {}",
        bytes.len()
    );
    let count = u32::from_le_bytes(bytes[80..84].try_into().unwrap()) as usize;
    assert_eq!(
        bytes.len(),
        84 + 50 * count,
        "{count} triangles take {} bytes",
        84 + 50 * count
    );
    let number = |at: usize| f32::from_le_bytes(bytes[at..at + 4].try_into().unwrap());
    let vector = |at: usize| Vec3::new(number(at), number(at + 4), number(at + 8)).as_dvec3();
    (0..count)
        .map(|index| {
            let at = 84 + 50 * index;
            let normal = vector(at);
            let corners = [vector(at + 12), vector(at + 24), vector(at + 36)];
            let winding = (corners[1] - corners[0]).cross(corners[2] - corners[0]);
            if winding.length() > 1e-9 {
                assert!(
                    (normal.length() - 1.0).abs() < 1e-5 && normal.dot(winding) > 0.0,
                    "triangle {index}: its normal {normal} is not the unit normal of \
                     its winding {winding}",
                );
            }
            corners
        })
        .collect()
}

/// The body written at each level and read back: something, closed,
/// uncrossed, facing out, and spanning what the body spans.
fn reads_back_sound(body: &Body) {
    let (low, high) = body.bounds().expect("the body holds matter");
    for gap in LEVELS {
        let bytes = stl::of(body, gap, 1.0).expect("the body is written");
        let triangles = read(&bytes);

        assert!(!triangles.is_empty(), "nothing was written at {gap} mm");
        assert_eq!(closed(&triangles), Ok(()), "at {gap} mm");
        assert_eq!(uncrossed(&triangles), Ok(()), "at {gap} mm");
        assert!(
            enclosed(&triangles) > 0.0,
            "at {gap} mm the triangles face into the matter"
        );
        let corners = triangles.iter().flatten();
        let (read_low, read_high) = corners.fold(
            (DVec3::INFINITY, DVec3::NEG_INFINITY),
            |(low, high), corner| (low.min(*corner), high.max(*corner)),
        );
        let near = gap * 1.01 + 1e-5;
        assert!(
            (read_low - low).abs().max_element() <= near
                && (read_high - high).abs().max_element() <= near,
            "at {gap} mm the file spans {read_low} to {read_high}, the body {low} to {high}",
        );
    }
}

#[test]
fn a_block_written_to_stl_reads_back_closed_and_uncrossed() {
    reads_back_sound(&block());
}

#[test]
fn a_bored_cylinder_written_to_stl_reads_back_closed_and_uncrossed() {
    reads_back_sound(&bored_cylinder());
}

#[test]
fn a_cone_written_to_stl_reads_back_closed_and_uncrossed() {
    reads_back_sound(&cone());
}

#[test]
fn a_shaft_cut_at_a_slant_written_to_stl_reads_back_closed_and_uncrossed() {
    reads_back_sound(&shaft_cut_at_a_slant());
}

#[test]
fn an_elliptic_post_written_to_stl_reads_back_closed_and_uncrossed() {
    reads_back_sound(&elliptic_post());
}
