//! Two walls at any angle meet along their curve (#577): a slanted hole
//! through a shaft, a radial hole through a cone, a pin through a taper, two
//! cones of other axes, an elliptic wall against a cone — each held to a
//! volume reasoned out apart from the kernel, its listing and its triangles.
//!
//! Part of #577: lane 2, two cylinders and elliptic walls at a skew angle.
//! The cones are lane 3, a pull request of their own; the criteria about
//! them are not answered here but in `a_wall_against_a_cone_at_any_angle.rs`,
//! whose head transcribes the whole issue.
//!
//! Done when:
//!
//! - A slanted hole through a shaft, at 30° and at 80°, holds its volume. The
//!   reference is a quadrature written in the test —
//!   `a_hole_slanted_at_30_degrees_through_a_shaft_holds_its_volume`,
//!   `a_hole_slanted_at_80_degrees_through_a_shaft_holds_its_volume`.
//! - A radial hole through a cone, and a pin through a taper, hold their
//!   volume — not answered here: lane 3, whose tests will be
//!   `a_radial_hole_through_a_cone_holds_its_volume` and
//!   `a_pin_through_a_taper_holds_its_volume`.
//! - Two cones of other axes, and an elliptic wall against a cone, hold their
//!   volume — not answered here: lane 3, whose tests will be
//!   `two_cones_of_other_axes_hold_their_volume` and
//!   `an_elliptic_wall_against_a_cone_holds_its_volume`.
//! - Pairs a hair from a touch or a node are snapped. A lean a hair off square
//!   or parallel is laid exactly so, and one in the band is noted unstable —
//!   `a_slanted_bore_a_hair_from_touching_a_shaft_is_moved_onto_the_touch`,
//!   `a_slanted_bar_a_hair_from_a_node_is_moved_onto_it`,
//!   `a_bore_a_hair_off_square_to_a_shaft_is_laid_square`,
//!   `a_bore_a_hair_off_parallel_to_a_shaft_is_laid_parallel`,
//!   `a_bore_leaning_in_the_band_off_square_is_noted_unstable`; and the
//!   campaign's findings, each laid square, in `random_walls_at_any_angle.rs`:
//!   `seed_121233920451060_a_pin_a_hair_off_square_beside_a_bore_a_hair_inside_the_wall_is_laid_square`,
//!   `seed_121233920454262_two_walls_a_hair_off_square_a_hair_from_their_nodes_are_laid_square`,
//!   `seed_187613593885498_a_pin_and_an_elliptic_boss_a_hair_off_square_are_laid_square_through_the_application_s_body`.
//!   Decided on the issue: two walls of one size, or touching, leaning in
//!   the band along a stretch are declined as unsupported, in the same file —
//!   `two_shafts_of_one_radius_crossing_nearly_coaxially_at_1e_5_degrees_are_declined_as_unsupported`,
//!   `seed_214398260937555_a_bar_touching_an_elliptic_shaft_inside_leaning_1e_5_degrees_is_declined`,
//!   `seed_175696479588841_two_posts_of_one_radius_crossing_at_1e_5_degrees_are_declined_as_unsupported`,
//!   and the same pair at 1e-4°, closer than a chord's sag, declined too
//!   until #706 laid such pairs on one shared order, and built since,
//!   `the_same_shafts_crossing_at_1e_4_degrees_are_built_uncrossed_and_noted_unstable`.
//!   Decided in lane 2's rounds: two walls leaning a sliver past decision
//!   11, under a sine of 2e-7, are declined where they meet —
//!   `two_shafts_crossing_at_1e_5_degrees_a_sliver_past_decision_11_are_declined_as_unsupported`,
//!   `seed_12099367009951_a_bore_crossing_a_post_s_wall_at_1e_5_degrees_is_declined_as_unsupported`,
//!   the same at 3e-5° still built,
//!   `the_same_shafts_crossing_at_3e_5_degrees_are_built_and_keep_every_rule`;
//!   two walls in the band closer than a chord's sag of their grids were
//!   declined until #706 laid two cylinders on one shared order, and are
//!   built since —
//!   `two_posts_of_one_radius_crossing_nearly_coaxially_at_1e_3_degrees_are_built_uncrossed_and_noted_unstable`,
//!   `seed_224442167739556_two_shafts_of_radius_150_crossing_at_1e_4_degrees_are_built_uncrossed_and_noted_unstable`,
//!   `seed_224442167740434_two_posts_of_radius_22_5_bored_at_30_degrees_crossing_at_1e_4_degrees_are_built_uncrossed_and_noted_unstable`,
//!   `the_same_posts_crossing_at_3e_4_or_1e_4_degrees_are_built_uncrossed_and_noted_unstable`;
//!   an elliptic wall or a cone that close still is (#720).
//!   A lean in the band built and noted unstable stays answered, off square
//!   by `a_bore_leaning_in_the_band_off_square_is_noted_unstable` and off
//!   parallel by `a_bore_leaning_in_the_band_off_parallel_is_built_and_noted_unstable`.
//! - #448's harness draws walls at any angle before the kernel code.
//!   Campaigns end under one failure in a thousand on fresh seeds, with no
//!   regression. A family that does not move in two rounds is reported and
//!   set aside — in `random_walls_at_any_angle.rs`:
//!   `the_draw_of_walls_at_any_angle_draws_every_pair_the_issue_names`,
//!   `random_walls_at_any_angle_keep_every_rule_on_the_exact_kernel`, and the
//!   campaigns `a_campaign_of_walls_at_any_angle_on_the_exact_kernel_keeps_every_rule`
//!   and `…_through_the_application_s_body_keeps_every_rule`. No test holds
//!   the rate: a campaign fails on any finding, and the rate, the regression
//!   and the families set aside are a record in
//!   `docs/exact-kernel-failures.md`, measured by hand on fresh seeds. The
//!   oblique families lane 2 leaves are kept as reds ignored on #707.
//! - `REBUILT_BY` moves, and `docs/exact-kernel.md` says what is built and
//!   what still declines — no test: the number is a constant a test would
//!   only restate, and the document is prose.
//!
//! Must not break: perpendicular and parallel pairs keep their bits —
//! `perpendicular_and_parallel_pairs_keep_their_bits`, pinned before any
//! kernel code; everything #526 to #571 deliver — the gate as it stands.

use std::f64::consts::{PI, TAU};

use cao_solid::Lean;
use cao_solid::brep::{Body, Curve, Declined, Surface};
use cao_solid::profile::{Contour, Frame, Run};
use cao_solid::soundness::{closed, listed, uncrossed};
use glam::{DVec2, DVec3};

/// How far a triangle may stand from the surface it stands for.
const MESH: f64 = 0.01;

const SHAFT: f64 = 10.0;
const HALF_LENGTH: f64 = 40.0;
const HOLE: f64 = 3.0;
/// How far the hole's axis passes from the shaft's, along X, square to both.
const OFFSET: f64 = 2.0;
/// How far the hole runs either side of where it crosses the shaft's axis:
/// past the wall on both sides at either angle.
const REACH: f64 = 30.0;

fn raised(outline: Contour, origin: DVec3, u: DVec3, v: DVec3, travel: DVec3) -> Body {
    Body::raised(&outline, &[], Frame { origin, u, v }, travel).expect("a leaf raises")
}

/// A shaft of radius ten about Z, eighty long: long enough that a hole
/// slanted at thirty degrees leaves it through its wall alone.
fn shaft() -> Body {
    raised(
        Contour::circle(DVec2::ZERO, SHAFT),
        DVec3::Z * -HALF_LENGTH,
        DVec3::X,
        DVec3::Y,
        DVec3::Z * 2.0 * HALF_LENGTH,
    )
}

/// The direction in the plane `YZ` leaning `degrees` from `Z` towards `Y`.
fn leaning(degrees: f64) -> DVec3 {
    let (sin, cos) = degrees.to_radians().sin_cos();
    DVec3::new(0.0, sin, cos)
}

/// A hole of radius three along `leaning(degrees)`, its axis passing
/// `OFFSET` from the shaft's along X, right through the shaft's wall.
fn slanted_hole(degrees: f64) -> Body {
    slanted(degrees, OFFSET, HOLE, REACH)
}

/// A cylinder of `radius` along `leaning(degrees)`, its axis passing
/// `offset` from the shaft's along X, from `reach` before where it crosses
/// the plane `YZ` to `reach` past it.
fn slanted(degrees: f64, offset: f64, radius: f64, reach: f64) -> Body {
    let along = leaning(degrees);
    let across = DVec3::X.cross(along) * -1.0;
    raised(
        Contour::circle(DVec2::new(offset, 0.0), radius),
        -along * reach,
        DVec3::X,
        across,
        along * 2.0 * reach,
    )
}

/// The volume a cylinder of `radius` at `degrees` off the axis, passing
/// `offset` from it along X, shares with an endless shaft: each of its lines
/// a distance `a` across from the shaft's axis along X runs
/// `2√(R² − a²)/sin θ` inside it, whatever its place along the other way;
/// over the cylinder's disc, `a = c + r sin φ`, the whole turn counting the
/// disc twice. The integrand is periodic and analytic in a strip of
/// half-width about 1.6 when the walls stand apart, so the trapezoid rule
/// over the turn converges geometrically, 32 samples agreeing with 256 to the
/// last bit or two; where they touch, a kink like `|φ − π/2|³` sits on a
/// sample and 4096 of them hold it to some 1e-14.
fn bored_out(degrees: f64, offset: f64, radius: f64) -> f64 {
    let samples = 4096;
    let sum: f64 = (0..samples)
        .map(|k| {
            let (sin, cos) = (TAU * k as f64 / samples as f64).sin_cos();
            let across = offset + radius * sin;
            cos * cos * (SHAFT * SHAFT - across * across).max(0.0).sqrt()
        })
        .sum();
    2.0 * radius * radius / degrees.to_radians().sin() * sum * TAU / samples as f64
}

/// A result held to the volume reasoned out, its listing and its triangles.
fn holds(result: Result<Body, Declined>, reasoned: f64) -> Body {
    holds_within(result, reasoned, 1e-9)
}

/// The same, the volume within `share` of what was reasoned.
fn holds_within(result: Result<Body, Declined>, reasoned: f64, share: f64) -> Body {
    let body = result.expect("the kernel answers");
    let volume = body.volume();
    assert!(
        ((volume - reasoned) / reasoned).abs() <= share,
        "the exact volume is {volume}, the quadrature {reasoned}"
    );
    assert_eq!(listed(&body.listing(), body.scale().reach()), Ok(()));
    let triangles = body.triangles(MESH);
    assert_eq!(closed(&triangles), Ok(()));
    assert_eq!(uncrossed(&triangles), Ok(()));
    body
}

const WHOLE_SHAFT: f64 = PI * SHAFT * SHAFT * 2.0 * HALF_LENGTH;

#[test]
fn a_hole_slanted_at_30_degrees_through_a_shaft_holds_its_volume() {
    holds(
        shaft().cut_by(&slanted_hole(30.0)),
        WHOLE_SHAFT - bored_out(30.0, OFFSET, HOLE),
    );
}

#[test]
fn a_hole_slanted_at_80_degrees_through_a_shaft_holds_its_volume() {
    holds(
        shaft().cut_by(&slanted_hole(80.0)),
        WHOLE_SHAFT - bored_out(80.0, OFFSET, HOLE),
    );
}

/// A whole ellipse about `center`, handed as one run all the way round.
fn ellipse(center: DVec2, first: DVec2, second: f64) -> Contour {
    Contour {
        corners: vec![center + first],
        runs: vec![Run::Oval {
            center,
            first,
            second,
            turn: TAU,
        }],
    }
}

/// What a body is to the bit: every vertex, the ends and the middle of
/// every edge as its curve gives them, sorted so that the order the kernel
/// lists them in does not count, and the volume — folded into one number.
fn bits(body: &Body) -> u64 {
    let listing = body.listing();
    let mut points: Vec<[u64; 3]> = listing
        .vertices
        .iter()
        .copied()
        .chain(listing.edges.iter().flat_map(|edge| {
            [edge.from, (edge.from + edge.to) / 2.0, edge.to].map(|t| edge.curve.point(t))
        }))
        .map(|point| point.to_array().map(f64::to_bits))
        .collect();
    points.sort_unstable();
    points
        .into_iter()
        .flatten()
        .chain([body.volume().to_bits()])
        .flat_map(u64::to_le_bytes)
        .fold(0xcbf2_9ce4_8422_2325, |hash, byte| {
            (hash ^ u64::from(byte)).wrapping_mul(0x0100_0000_01b3)
        })
}

/// Pinned before any kernel code of #577; a Mac and Linux gave the same
/// bits, though their `sin` and `cos` may part in the last one (#541).
#[test]
fn perpendicular_and_parallel_pairs_keep_their_bits() {
    let square_hole = raised(
        Contour::circle(DVec2::new(OFFSET, 5.0), HOLE),
        DVec3::Y * 20.0,
        DVec3::X,
        DVec3::Z,
        DVec3::Y * -40.0,
    );
    let equal_bar = raised(
        Contour::circle(DVec2::ZERO, SHAFT),
        DVec3::Y * 20.0,
        DVec3::X,
        DVec3::Z,
        DVec3::Y * -40.0,
    );
    let elliptic_hole = raised(
        ellipse(DVec2::new(1.0, -4.0), DVec2::new(4.0, 2.0), 1.5),
        DVec3::Y * 20.0,
        DVec3::X,
        DVec3::Z,
        DVec3::Y * -40.0,
    );
    let parallel_bore = raised(
        Contour::circle(DVec2::new(8.0, 1.0), HOLE),
        DVec3::Z * -50.0,
        DVec3::X,
        DVec3::Y,
        DVec3::Z * 100.0,
    );
    for (what, made, pinned) in [
        (
            "a hole square through the shaft off its axis",
            shaft().cut_by(&square_hole),
            0xf911_09fe_75b2_0db0,
        ),
        (
            "a bar of the shaft's radius square across it",
            shaft().joined(&equal_bar),
            0x4277_01e8_6721_c5fa,
        ),
        (
            "an elliptic hole square through the shaft",
            shaft().cut_by(&elliptic_hole),
            0xc1fc_38b1_98fa_1caa,
        ),
        (
            "a bore parallel to the shaft through its wall",
            shaft().cut_by(&parallel_bore),
            0xc1ec_e6c9_726f_6a38,
        ),
    ] {
        let body = made.unwrap_or_else(|declined| panic!("{what}: {declined:?}"));
        assert_eq!(bits(&body), pinned, "{what}");
    }
}

/// The sine each pair of walls of a body leans off square or off parallel
/// by, for the pairs within a thousandth of either: what decision 11 for
/// walls is to lay to nought.
fn hairs_off(body: &Body) -> Vec<f64> {
    let mut walls: Vec<Surface> = Vec::new();
    for face in body.listing().faces {
        if !walls.contains(&face.surface) {
            walls.push(face.surface);
        }
    }
    let axes: Vec<DVec3> = walls
        .iter()
        .filter_map(|wall| match wall {
            Surface::Cylinder(wall) => Some(wall.axis),
            Surface::Elliptic(wall) => Some(wall.axis),
            Surface::Cone(wall) => Some(wall.axis),
            _ => None,
        })
        .collect();
    let mut hairs = Vec::new();
    for (index, one) in axes.iter().enumerate() {
        for other in &axes[index + 1..] {
            let sine = one.cross(*other).length().min(one.dot(*other).abs());
            if sine < 1e-3 {
                hairs.push(sine);
            }
        }
    }
    hairs
}

/// A body whose walls a hair off square or off parallel were laid exactly
/// so, and noted as nothing unstable.
fn laid(body: &Body) {
    let hairs = hairs_off(body);
    assert!(
        !hairs.is_empty() && hairs.iter().all(|sine| *sine == 0.0),
        "walls left a hair off: {hairs:?}"
    );
    assert_eq!(body.unstable(), None);
}

#[test]
fn a_bore_a_hair_off_square_to_a_shaft_is_laid_square() {
    let body = holds_within(
        shaft().cut_by(&slanted_hole(90.0 - 1e-7)),
        WHOLE_SHAFT - bored_out(90.0, OFFSET, HOLE),
        1e-8,
    );
    laid(&body);
    let met = body
        .listing()
        .edges
        .iter()
        .filter_map(|edge| match edge.curve {
            Curve::Meet(meet) => Some(meet.first.axis().dot(meet.second.axis())),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert!(
        !met.is_empty() && met.iter().all(|dot| *dot == 0.0),
        "the hole meets the shaft along the curve of two square walls: {met:?}"
    );
}

/// The area two discs of radii `big` and `small`, their centres `apart`,
/// share.
fn lens(big: f64, small: f64, apart: f64) -> f64 {
    let near = ((apart * apart + small * small - big * big) / (2.0 * apart * small)).acos();
    let far = ((apart * apart + big * big - small * small) / (2.0 * apart * big)).acos();
    let kite = ((-apart + small + big)
        * (apart + small - big)
        * (apart - small + big)
        * (apart + small + big))
        .sqrt();
    small * small * near + big * big * far - kite / 2.0
}

#[test]
fn a_bore_a_hair_off_parallel_to_a_shaft_is_laid_parallel() {
    let bore = slanted(1e-7, 8.0, HOLE, 50.0);
    let body = holds_within(
        shaft().cut_by(&bore),
        WHOLE_SHAFT - 2.0 * HALF_LENGTH * lens(SHAFT, HOLE, 8.0),
        1e-8,
    );
    laid(&body);
    let lines = body
        .listing()
        .edges
        .iter()
        .filter(|edge| matches!(edge.curve, Curve::Line(line) if line.direction == DVec3::Z))
        .count();
    assert_eq!(
        lines, 2,
        "the bore leaves the wall along two lines up its axis"
    );
}

#[test]
fn a_bore_leaning_in_the_band_off_square_is_noted_unstable() {
    let lean = 1e-3;
    let body = holds(
        shaft().cut_by(&slanted_hole(90.0 - lean)),
        WHOLE_SHAFT - bored_out(90.0 - lean, OFFSET, HOLE),
    );
    let unstable = body.unstable().expect("the step is noted unstable");
    assert_eq!(unstable.lean, Lean::Across, "the two axes nearly square");
    assert!(
        ((unstable.degrees - lean) / lean).abs() <= 1e-6,
        "noted {} degrees off, drawn {lean}",
        unstable.degrees
    );
}

/// The shaft between two parallel planes leaning 30° about X through its
/// axis at ±30: every line along its axis runs sixty inside it, and none
/// of its faces stands square to it any longer.
fn shaft_cut_at_a_slant_at_both_ends() -> Body {
    let (sin, cos) = 30f64.to_radians().sin_cos();
    let v = DVec3::new(0.0, cos, sin);
    let normal = DVec3::X.cross(v);
    let beyond = |level: f64, way: f64| {
        raised(
            Contour::rectangle(DVec2::splat(-20.0), DVec2::splat(20.0)),
            DVec3::Z * level,
            DVec3::X,
            v,
            normal * way * 30.0,
        )
    };
    shaft()
        .cut_by(&beyond(30.0, 1.0))
        .and_then(|cut| cut.cut_by(&beyond(-30.0, -1.0)))
        .expect("the ends cut at a slant")
}

/// A bore of radius 3 eight off the shaft's axis along Y, crossing its
/// wall at a clear angle, its axis leaning `degrees` off the shaft's towards
/// X, square to the way the ends slope: from `reach` below the middle to
/// `reach` above it.
fn leaning_beside(degrees: f64, reach: f64) -> Body {
    let (sin, cos) = degrees.to_radians().sin_cos();
    let along = DVec3::new(sin, 0.0, cos);
    raised(
        Contour::circle(DVec2::new(8.0, 0.0), HOLE),
        -along * reach,
        DVec3::Y,
        along.cross(DVec3::Y),
        along * 2.0 * reach,
    )
}

/// The bore of `leaning_beside` a hundredth of a degree off the shaft's
/// axis: in the band, nearly parallel, and neither within the tolerance
/// along a stretch, a sliver, nor closer than a chord's sag. Between the two
/// slanted ends it takes sixty times the lens of the two discs: the lean
/// moves the bore along X, which the ends do not slope along, and parts
/// the axes by no more than `(36·tan θ)²/16`, some 2.5e-6, a few billionths
/// of the volume.
#[test]
fn a_bore_leaning_in_the_band_off_parallel_is_built_and_noted_unstable() {
    let lean = 1e-2;
    let ends = shaft_cut_at_a_slant_at_both_ends();
    let body = holds_within(
        ends.cut_by(&leaning_beside(lean, 60.0)),
        PI * SHAFT * SHAFT * 60.0 - 60.0 * lens(SHAFT, HOLE, 8.0),
        1e-7,
    );
    let unstable = body.unstable().expect("the step is noted unstable");
    assert_eq!(unstable.lean, Lean::Abreast, "the two axes nearly parallel");
    assert!(
        ((unstable.degrees - lean) / lean).abs() <= 1e-6,
        "noted {} degrees off, drawn {lean}",
        unstable.degrees
    );
}

/// How far, past where the hole would touch the shaft's wall from inside,
/// its axis is drawn: a hair under the kernel's tolerance.
const HAIR: f64 = 1e-8;

#[test]
fn a_slanted_bore_a_hair_from_touching_a_shaft_is_moved_onto_the_touch() {
    let touching = SHAFT - HOLE;
    let body = holds(
        shaft().cut_by(&slanted(30.0, touching + HAIR, HOLE, REACH)),
        WHOLE_SHAFT - bored_out(30.0, touching, HOLE),
    );
    let node = DVec3::new(SHAFT, 0.0, 0.0);
    let at = body.listing().vertices;
    assert!(
        at.iter().any(|vertex| vertex.distance(node) <= 1e-9),
        "the two loops join at a node where the walls touch: {at:?}"
    );
}

#[test]
fn a_slanted_bar_a_hair_from_a_node_is_moved_onto_it() {
    let bar = slanted(60.0, HAIR, SHAFT, REACH);
    let common = 16.0 * SHAFT.powi(3) / (3.0 * 60f64.to_radians().sin());
    let body = holds(
        shaft().joined(&bar),
        WHOLE_SHAFT + PI * SHAFT * SHAFT * 2.0 * REACH - common,
    );
    let at = body.listing().vertices;
    for node in [DVec3::X * SHAFT, DVec3::X * -SHAFT] {
        assert!(
            at.iter().any(|vertex| vertex.distance(node) <= 1e-9),
            "two ellipses crossing at {node}: {at:?}"
        );
    }
}
