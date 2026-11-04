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

/// A body, how far a point stands from the nearest of its true surfaces, and
/// what the arithmetic says it encloses and how much surface it has.
struct Case {
    name: &'static str,
    body: Body,
    off: fn(DVec3) -> f64,
    volume: f64,
    area: f64,
}

fn radial(point: DVec3) -> f64 {
    point.truncate().length()
}

fn block_off(p: DVec3) -> f64 {
    [p.x, p.x - 30.0, p.y, p.y - 20.0, p.z, p.z - 10.0]
        .map(f64::abs)
        .into_iter()
        .fold(f64::INFINITY, f64::min)
}

fn bored_cylinder_off(p: DVec3) -> f64 {
    [p.z, p.z - 10.0, radial(p) - 20.0, radial(p) - 5.0]
        .map(f64::abs)
        .into_iter()
        .fold(f64::INFINITY, f64::min)
}

fn cone_off(p: DVec3) -> f64 {
    let across = DVec2::new(p.x, p.z).length();
    p.y.abs()
        .min((2.0 * across + p.y - 20.0).abs() / 5f64.sqrt())
}

fn shaft_cut_at_a_slant_off(p: DVec3) -> f64 {
    let slant = (p.z - 5.0 - 0.2 * p.x).abs() / 1.04f64.sqrt();
    p.z.abs().min((radial(p) - 20.0).abs()).min(slant)
}

fn ellipse_off(point: DVec2, a: f64, b: f64) -> f64 {
    let at = |t: f64| (DVec2::new(a * t.cos(), b * t.sin()) - point).length();
    let steps = 4096;
    let step = TAU / steps as f64;
    let nearest = (0..steps)
        .map(|index| index as f64 * step)
        .min_by(|one, other| at(*one).total_cmp(&at(*other)))
        .unwrap();
    let (mut low, mut high) = (nearest - step, nearest + step);
    for _ in 0..80 {
        let (one, other) = (low + (high - low) / 3.0, high - (high - low) / 3.0);
        if at(one) < at(other) {
            high = other;
        } else {
            low = one;
        }
    }
    at((low + high) / 2.0)
}

fn elliptic_post_off(p: DVec3) -> f64 {
    p.z.abs()
        .min((p.z - 10.0).abs())
        .min(ellipse_off(p.truncate(), 15.0, 8.0))
}

fn ellipse_perimeter(a: f64, b: f64) -> f64 {
    let steps = 200_000;
    let step = TAU / steps as f64;
    (0..steps)
        .map(|index| {
            let t = (index as f64 + 0.5) * step;
            DVec2::new(a * t.sin(), b * t.cos()).length() * step
        })
        .sum()
}

fn cases() -> Vec<Case> {
    use std::f64::consts::PI;
    vec![
        Case {
            name: "block",
            body: block(),
            off: block_off,
            volume: 30.0 * 20.0 * 10.0,
            area: 2.0 * (30.0 * 20.0 + 30.0 * 10.0 + 20.0 * 10.0),
        },
        Case {
            name: "bored cylinder",
            body: bored_cylinder(),
            off: bored_cylinder_off,
            volume: PI * (400.0 - 25.0) * 10.0,
            area: 2.0 * PI * (400.0 - 25.0) + TAU * 20.0 * 10.0 + TAU * 5.0 * 10.0,
        },
        Case {
            name: "cone",
            body: cone(),
            off: cone_off,
            volume: PI * 100.0 * 20.0 / 3.0,
            area: PI * 100.0 + PI * 10.0 * 500f64.sqrt(),
        },
        Case {
            name: "shaft cut at a slant",
            body: shaft_cut_at_a_slant(),
            off: shaft_cut_at_a_slant_off,
            volume: 5.0 * PI * 400.0,
            area: PI * 400.0 + TAU * 20.0 * 5.0 + PI * 400.0 * 1.04f64.sqrt(),
        },
        Case {
            name: "elliptic post",
            body: elliptic_post(),
            off: elliptic_post_off,
            volume: PI * 15.0 * 8.0 * 10.0,
            area: 2.0 * PI * 15.0 * 8.0 + ellipse_perimeter(15.0, 8.0) * 10.0,
        },
    ]
}

/// What rounding every number to an `f32` may move a corner by, on triangles
/// reaching `reach` from the origin.
fn rounding(triangles: &[Triangle]) -> f64 {
    let reach = triangles
        .iter()
        .flatten()
        .map(|corner| corner.abs().max_element())
        .fold(0.0, f64::max);
    4.0 * f64::from(f32::EPSILON) * reach
}

/// Every corner on a true surface, and every point of every triangle within
/// `gap` of one.
fn stands_within(triangles: &[Triangle], off: impl Fn(DVec3) -> f64, gap: f64, what: &str) {
    let slack = rounding(triangles);
    for (index, corners) in triangles.iter().enumerate() {
        for corner in corners {
            assert!(
                off(*corner) <= slack,
                "{what}: a corner of triangle {index} stands {} off every surface",
                off(*corner),
            );
        }
        for one in 0..=8 {
            for other in 0..=8 - one {
                let (a, b) = (one as f64 / 8.0, other as f64 / 8.0);
                let point = corners[0] * (1.0 - a - b) + corners[1] * a + corners[2] * b;
                assert!(
                    off(point) <= gap * (1.0 + 1e-9) + slack,
                    "{what}: triangle {index} stands {} off its surface, the gap is {gap} mm",
                    off(point),
                );
            }
        }
    }
}

/// A cylinder of radius 2 m standing 10 mm high: at « Fine », a round so wide
/// that a thousand and twenty-four chords to the turn stand 9.4 µm off it.
fn wide_disc() -> Body {
    on_a_part(raised(disc(DVec2::ZERO, 2000.0), GROUND, 10.0))
}

fn wide_disc_off(p: DVec3) -> f64 {
    p.z.abs()
        .min((p.z - 10.0).abs())
        .min((radial(p) - 2000.0).abs())
}

#[test]
fn the_volume_read_back_is_within_the_level_s_gap_times_the_arithmetic_s_area() {
    for case in cases() {
        for gap in LEVELS {
            let bytes = stl::of(&case.body, gap, 1.0).expect("the body is written");
            let read = enclosed(&read(&bytes));

            assert!(
                (read - case.volume).abs() <= gap * case.area + 1e-6 * case.volume,
                "the {} read back at {gap} mm encloses {read}, the arithmetic {}, \
                 within {gap} × {}",
                case.name,
                case.volume,
                case.area,
            );
        }
    }
}

#[test]
fn every_triangle_read_back_stands_within_the_level_s_gap_of_the_true_surface() {
    for case in cases() {
        for gap in LEVELS {
            let bytes = stl::of(&case.body, gap, 1.0).expect("the body is written");
            stands_within(
                &read(&bytes),
                case.off,
                gap,
                &format!("the {} at {gap} mm", case.name),
            );
        }
    }
    let wide = wide_disc();
    for gap in LEVELS {
        match stl::of(&wide, gap, 1.0) {
            Ok(bytes) => stands_within(
                &read(&bytes),
                wide_disc_off,
                gap,
                &format!("the disc 4 m across at {gap} mm"),
            ),
            Err(_) => assert_eq!(
                gap, 0.005,
                "the disc 4 m across is held within {gap} mm and was not written"
            ),
        }
    }
}

#[test]
fn a_finer_level_writes_more_triangles() {
    for case in cases().into_iter().filter(|case| case.name != "block") {
        let counts = LEVELS
            .map(|gap| read(&stl::of(&case.body, gap, 1.0).expect("the body is written")).len());

        assert!(
            counts[0] < counts[1] && counts[1] < counts[2],
            "the {} written at {LEVELS:?} mm holds {counts:?} triangles",
            case.name,
        );
    }
}

#[test]
fn a_part_drawn_in_units_of_two_and_a_half_millimetres_is_written_in_millimetres() {
    let body = bored_cylinder();
    let gap = 0.02;
    let in_millimetres = read(&stl::of(&body, gap, 2.5).expect("the body is written"));
    let in_units = read(&stl::of(&body, gap, 1.0).expect("the body is written"));

    let (low, high) = body.bounds().expect("the body holds matter");
    let (read_low, read_high) = in_millimetres.iter().flatten().fold(
        (DVec3::INFINITY, DVec3::NEG_INFINITY),
        |(low, high), corner| (low.min(*corner), high.max(*corner)),
    );
    assert!(
        (read_low - low * 2.5).abs().max_element() <= gap * 1.01 + 1e-4
            && (read_high - high * 2.5).abs().max_element() <= gap * 1.01 + 1e-4,
        "the file spans {read_low} to {read_high} mm, the part {} to {} mm",
        low * 2.5,
        high * 2.5,
    );
    stands_within(
        &in_millimetres,
        |point| 2.5 * bored_cylinder_off(point / 2.5),
        gap,
        "the bored cylinder at 2.5 mm a unit",
    );
    assert!(
        in_millimetres.len() > in_units.len(),
        "a gap of {gap} mm is finer in units of 2.5 mm than in units of 1 mm: \
         {} triangles against {}",
        in_millimetres.len(),
        in_units.len(),
    );
}

#[test]
fn a_body_the_triangles_cannot_cover_is_not_written() {
    let wide = wide_disc();

    assert_eq!(
        stl::of(&wide, 0.005, 1.0).map(|bytes| bytes.len()),
        Err(stl::Uncovered),
        "no thousand and twenty-four chords hold a round 4 m across within 5 µm",
    );
    assert!(
        stl::of(&wide, 0.02, 1.0).is_ok(),
        "at 0.02 mm the same round is held, and written"
    );
}
