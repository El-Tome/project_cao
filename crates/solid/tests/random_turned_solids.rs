//! Profiles of straight runs turned about lines of their planes, drawn at
//! random among prisms and combined at random, held to the rules every
//! result must keep (#533).
//!
//! The cases are #448's, drawn among turns (`Case::drawn_turned`): a section
//! of bands laid along a line of one of the planes of the origin — a
//! rectangle, a stepped shaft, an L, a tube, a band with a hole, on its axis,
//! off it, a hair either side of it, or across it — turned whole, by a
//! quarter turn's multiple or by a step of fifteen degrees, either way; half
//! of them drawn from a leaf before them, about a circle's axis, on a
//! block's edge, along the axis of a turn before them, and the prisms drawn
//! from turns, coaxial bosses, blocks tangent to a shaft, holes across it.
//!
//! The arithmetic holds a turned leaf to a slab of an annulus cut by a wedge
//! along every line, and to Pappus's volume, which the corners the kernels
//! are handed are held to here in turn. A section across its axis is handed
//! over as its two sides, each turned and the two joined, so that the
//! kernel's own joins of the halves are tried at every angle. The flats hold
//! the arithmetic where they can be trusted with a turn: lone leaves turned
//! forwards, off their axis, within the sagitta of their steps. Whole cases
//! of turns are too slow on the flats and too often broken for the gate
//! (#418, #486, #492), so they hold the arithmetic once, by hand, behind
//! `campaigns`: the flats answered 161 of the first 300 turned cases by
//! their own rules when it was written, and the arithmetic held every one.
//!
//! Runs slant too (#536), in a draw of their own (`Case::drawn_slanted`),
//! which keeps every seed of `drawn_turned` its case: three fresh sections
//! in four take a chamfer where a shaft steps — its slant landing on the
//! wall past the step, a hair short of it, or short on the lattice — a
//! point, a chamfered end or bore, a countersink, a taper, a ridge, two
//! slants on one line, or a slope a hair from parallel or from square; and
//! tools are drawn from them: a countersunk hole into a block, a chamfer cut
//! round a raised circle, a cone standing on it, and after a slant the same
//! cone again, a cylinder at its rim, a cone crossing it there, a slant a
//! hair off it, a circle on its rim. A slanted piece of a section turns into
//! the room between two cones, a line against each of them one quadratic,
//! and encloses a frustum's volume; a line through a cone's tip, where the
//! cone has no normal, is left out and counted as a grazing one is. The
//! reading lays a slanted run as drawn, and the exact kernel turns it into a
//! cone. The flats answered 231 of the first 300 slanted cases by their own
//! rules when the draw was written, and the arithmetic held every one.
//!
//! Planes lean into them (#552), in a draw of their own
//! (`Case::drawn_tilted`), which keeps every seed of `drawn_slanted` its
//! case: tools on planes leaning about a world axis, drawn from a leaf
//! before them — a slanted cut through a block, a hole or a boss square to
//! the face it left, a bore at a slant, a leaning block; a slant through a
//! shaft's middle or about a rim's tangent, a plane along its axis at its
//! radius, which touches it along a line, a flat; on a cone a flat along its
//! axis, a slant, a plane parallel to a ruling, planes through its tip — and
//! off the lattice angles a hair from square, from parallel and from a
//! cone's slope. A plane meets a cylinder or a cone there along an ellipse,
//! a parabola, a hyperbola, two lines through an apex, the apex alone or a
//! ruling it touches along: `Case::conics` tells which. The kernel builds
//! them, so a decline as unsupported is excused only on a case asking
//! beyond the conics (`Case::asks_beyond_the_conics`): two cylinders at a
//! skew angle, or a cone against a wall off its axis, where their leaves'
//! boxes meet. The volume a
//! round leaf keeps under a slanted plane is held besides to its slices
//! (`Leaf::kept_by_a_half_space`), which owe nothing to the lines. The flats
//! answered 244 of the first 300 tilted cases by their own rules when the
//! draw was written, and the arithmetic held every one.
//!
//! Edges bow along arcs (#576), in a draw of their own
//! (`Case::drawn_rounded`), which keeps every seed of `drawn_slanted` and
//! `drawn_tilted` its case: three fresh sections in four take an arc — a
//! ball, a zone or a cap of one, a ring torus or an arc of one raised on a
//! cylinder or bored in a tube, an apple, a shaft with a rounded end, a
//! round-bottomed groove, a rounded shoulder, a fillet, a barrel bore, a
//! round groove in a bore — or one of the hairs a kernel breaks on: a ball's
//! centre a hair off the axis, a cap's, an arc a hair from tangent to the
//! wall it continues, a sphere a hair larger than the shaft it ends; and
//! tools are drawn from an arc: a coaxial cylinder at its equator or a hair
//! off, a shaft ending square through its centre, at its pole or a hair from
//! either, the same arc again or a hair along the axis, the whole ball or
//! torus it lies on or a hair larger, a cone tangent at a lattice latitude,
//! a sleeve, a block through the axis, a circle or a ring square to the axis
//! at the centre or the pole, and a ball or a dome on a circle raised
//! before. An arc's centre stands on a coordinate of the lattice and its
//! radius on a length, its corners on the lattice points of its circle —
//! the ends of its diameters and the points a Pythagorean triple puts there
//! — so that no number of a gate case comes through a sine or a cosine. A
//! bowed piece of a section turns into the room between a sphere or a torus
//! and whatever its other edge turns into, a line against a sphere one
//! quadratic and against a torus a quartic, and encloses the volume Pappus
//! gives its arc; a line through a pole is counted as one through a cone's
//! tip. The flats turn an arc sampled as the application samples one, and
//! are held to the arithmetic within the sagittas of the profile's sampling
//! and of the turn together. A sphere or a torus is built along circles
//! alone, so a decline as unsupported is excused only on a case asking
//! beyond them (`Case::asks_a_round_beyond_its_circles`): a plane at a
//! slant, a block's side, a hole or a cone off the axis. The exact kernel
//! declines a profile with an arc today, so the tests that need it are
//! ignored until it turns one. The gate holds the drawn rounded leaves off
//! their axis to the flats, as it holds the turned and the slanted ones; a
//! whole rounded case on the flats takes the better part of a minute where
//! a torus is cut several times, so the cases are held to them once, by
//! hand, behind `campaigns`: the flats answered 236 of the first 300
//! rounded cases by their own rules when the draw was written, and the
//! arithmetic held every one.
//!
//! The exact kernel lays a section against its axis before turning it; a
//! hole a hair from its band's edge is laid onto the outline, and the wall
//! between them is not there. Where that leaves the matter in two pieces, or
//! touching itself at a corner, it declines the section, which the
//! application turns on the flats then, and that decline is counted apart;
//! so is a section a hair long from end to end, which laying makes nothing.
//! A campaign is run by hand, on the exact kernel or through the
//! application's body:
//!
//! ```text
//! CAO_FUZZ_SECONDS=3600 cargo test --release -p cao_solid --features campaigns \
//!     --test random_turned_solids -- --ignored --nocapture \
//!     a_campaign_of_turned_solids_on_the_exact_kernel
//! ```
//!
//! It draws past the gate's lattice (`Case::drawn_turned_off_the_lattice`):
//! angles a hair from half a turn, a quarter and a whole one, and axes
//! slanted or leaning a hair, where a case asking beyond the conics may be
//! declined as unsupported and is counted so. The campaigns of slanted
//! solids (`a_campaign_of_slanted_solids_…`) draw past it the same way
//! (`Case::drawn_slanted_off_the_lattice`), with slopes at thirty and sixty
//! degrees besides, and coaxial tools a hair off their axis; and those of
//! tilted solids (`a_campaign_of_tilted_solids_…`,
//! `Case::drawn_tilted_off_the_lattice`) with their planes leaning by a
//! degree's step, a real angle, or a hair; and those of rounded solids
//! (`a_campaign_of_rounded_solids_…`, `Case::drawn_rounded_off_the_lattice`)
//! with an arc a hair short of a quarter circle besides, which ends a hair
//! off the axis, and coaxial tools a hair off their axis.
//! `CAO_FUZZ_SEED`, `CAO_FUZZ_CASES` and `CAO_FUZZ_PATIENCE` are read as in
//! `random_exact_solids.rs`; the seeds a campaign names are shrunk by
//! `the_seeds_a_turning_campaign_named_are_shrunk_one_by_one`, given in
//! `CAO_TRIAGE_SEEDS`, with `CAO_TRIAGE_KERNEL=application` for the
//! campaign through the application's body, and `CAO_TRIAGE_DRAW=slanted`,
//! `tilted` or `rounded` for a campaign of slanted, tilted or rounded
//! solids.
//!
//! Closes #541.
//! - the fingerprint of the draws passes on macOS and on the CI's Linux —
//!   `the_draws_before_turns_still_give_each_seed_its_case`
//! - a case drawn differently still changes the fingerprint —
//!   `a_case_drawn_differently_changes_the_fingerprint`
//!
//! Closes #586.
//! - the two fingerprints that failed on macOS pass there and on the CI's
//!   Linux, and so do the others of the file —
//!   `the_draws_before_ellipses_still_give_each_seed_its_case`,
//!   `the_draws_of_ellipses_give_each_seed_its_case`
//! - no number a fingerprinted draw prints stands where the last bit decides
//!   its count — `no_number_a_fingerprinted_draw_prints_stands_where_the_last_bit_decides_its_count`
//!
//! Beside what #586 asked: the draws themselves gave a seed a case one lattice
//! step apart on the two platforms, where they floored, ceiled or rounded a
//! count of steps reached through sines and cosines — a box's corner seen on
//! a leaning plane, an ellipse's reach, a bound handed to `Random::on_lattice`.
//! A count a hair off a step is now counted on it (`in_steps`), and the
//! fingerprints are what holds it on both platforms.
//! - a case drawn differently still changes the fingerprint —
//!   `a_case_drawn_differently_changes_the_fingerprint`

// The drawing, the promise and the checks are shared with the other
// campaigns; each file uses its own part of them.
#[allow(dead_code, unused_imports)]
mod random_solids;

use std::f64::consts::PI;

use cao_solid::Declined;
use cao_solid::brep::Scale;
use cao_solid::profile::Run;
use cao_solid::soundness::{Lines, Spans, shrink};
use cao_solid::turning::Lie;
use cao_solid::turning::Straight;
use glam::{DQuat, DVec2, DVec3};
use random_solids::{
    Application, Axis, Bow, Case, Drawn, Exact, Flats, Kernel, Leaf, Mode, ON_THE_AXIS, Outline,
    Piece, Plane, Section, Step, Stretch, TESSELLATION, Trapezoid, Turned,
};

fn spans_of(stretches: &[Stretch]) -> Spans {
    Spans::gathered(
        stretches
            .iter()
            .map(|stretch| (stretch.from.at, stretch.to.at))
            .collect(),
    )
}

fn turned(leaf: &Leaf) -> Option<Turned> {
    match leaf {
        Leaf::Turned { .. } => leaf.as_turned(),
        _ => None,
    }
}

/// The turned leaves of the first cases of the draw.
fn drawn_turns(seeds: u64) -> Vec<Leaf> {
    (0..seeds)
        .flat_map(|seed| {
            Case::drawn_turned(seed)
                .leaves()
                .cloned()
                .collect::<Vec<_>>()
        })
        .filter(|leaf| matches!(leaf, Leaf::Turned { .. }))
        .collect()
}

/// A bundle of lines laid over the box a leaf spans, and how far it reaches.
fn lines_over(leaf: &Leaf) -> (Lines, f64) {
    let (low, high) = leaf.bounds().expect("a leaf with a box");
    let reach = low.abs().max(high.abs()).max_element().max(1.0);
    (Lines::across(low - 1.0, high + 1.0, 24), reach)
}

/// Where a number starts counting as one millionth more: this far from the
/// count below, and not halfway. The draws lay their hair offsets on the
/// halfway mark on purpose (`20.0000005`, `5.5e-6`, `179.9999985`), where the
/// last bit a platform's `sin_cos` rounds decides which way a number goes
/// (#586). No draw builds a number on a golden fraction of a millionth.
const NEXT_MILLIONTH_FROM: f64 = 0.381_966_011_250_105_2;

/// How near the mark a number may stand, in millionths: a hundred times what
/// the last bits of the platform's `sin` and `cos` move a number of the draws.
const CLEAR_OF_THE_MARK: f64 = 1e-6;

/// A case as printed, each number in it counted in millionths. A star's
/// corners are laid with `sin` and `cos`, which each platform's library rounds
/// its own way in the last bit: read to the last digit, the same draws would
/// have one fingerprint on a Mac and another on Linux (#541). A sign is read
/// with its number, so that a cosine of a right angle counts no millionth
/// either side of nothing.
fn in_millionths(printed: &str) -> String {
    each_number(printed, |value| {
        format!(
            "{}",
            (value * 1e6 + 1.0 - NEXT_MILLIONTH_FROM).floor() as i64
        )
    })
}

/// The printed case, each number with a fraction or an exponent replaced by
/// what `count` makes of it. Whole numbers stay as they are printed.
fn each_number(printed: &str, mut count: impl FnMut(f64) -> String) -> String {
    let mut rounded = String::with_capacity(printed.len());
    let mut characters = printed.chars().peekable();
    while let Some(first) = characters.next() {
        let signed = first == '-' && characters.peek().is_some_and(char::is_ascii_digit);
        if !first.is_ascii_digit() && !signed {
            rounded.push(first);
            continue;
        }
        let mut number = String::from(first);
        while let Some(&next) = characters.peek() {
            let signed_exponent = matches!(next, '-' | '+') && number.ends_with('e');
            if !(next.is_ascii_digit() || matches!(next, '.' | 'e') || signed_exponent) {
                break;
            }
            number.push(next);
            characters.next();
        }
        match number.parse::<f64>() {
            Ok(value) if number.contains(['.', 'e']) => rounded.push_str(&count(value)),
            _ => rounded.push_str(&number),
        }
    }
    rounded
}

/// A draw of cases, one to a seed.
type Draw = fn(u64) -> Case;

/// How many pairs of walls meet along one kind of conic.
type Count = fn(&random_solids::Conics) -> usize;

/// Every draw a fingerprint is asserted for below.
const FINGERPRINTED: [Draw; 14] = [
    Case::drawn,
    Case::drawn_square,
    Case::drawn_profiles,
    Case::drawn_turned,
    Case::drawn_turned_off_the_lattice,
    Case::drawn_slanted,
    Case::drawn_slanted_off_the_lattice,
    Case::drawn_tilted,
    Case::drawn_tilted_off_the_lattice,
    Case::drawn_ellipses,
    Case::drawn_ellipses_crossed,
    Case::drawn_ellipses_tilted,
    Case::drawn_rounded,
    Case::drawn_rounded_off_the_lattice,
];

/// The fingerprints of the draws before the rounds (#576), in the order
/// `FINGERPRINTED` lists them: the values the tests below pin one by one,
/// which no draw of arcs may move.
const BEFORE_ROUNDS: [u64; 12] = [
    0xf3d0_3d25_d6d6_62eb,
    0x0347_3e33_46ca_101b,
    0xc776_8886_b55b_afbd,
    0x9180_1d59_a432_9c6a,
    0x13be_c8c0_b781_93e1,
    0x32b3_b100_d73f_534f,
    0x5ae8_3b2b_d8e7_55f3,
    0x32d0_d851_1237_f618,
    0x1fd0_313a_bd3d_51fe,
    0x3472_512f_3462_be46,
    0x2e27_8c01_f49f_a60d,
    0x9fad_ec77_db9b_db13,
];

fn fingerprint(draw: Draw) -> u64 {
    (0..300)
        .flat_map(|seed| in_millionths(&draw(seed).to_string()).into_bytes())
        .fold(0xcbf2_9ce4_8422_2325, |hash, byte| {
            (hash ^ u64::from(byte)).wrapping_mul(0x0100_0000_01b3)
        })
}

#[test]
fn the_draws_before_turns_still_give_each_seed_its_case() {
    assert_eq!(
        Case::drawn(29).to_string(),
        "Case::new(
    Leaf::revolution(Plane::xz(7.0), [1.0, -3.0], [6.0, 0.0], -270.0),
    vec![
        Step::cut(Leaf::revolution(Plane::xy(4.0), [2.0, 4.0], [7.0, 8.0], 360.0)),
    ],
)"
    );
    assert_eq!(
        Case::drawn_profiles(175).to_string(),
        "Case::new(
    Leaf::prism(Plane::xy(1.0), Outline::slot([8.0, 2.0], [12.0, 2.0], 1.0), 9.5),
    vec![
        Step::cut(Leaf::prism(Plane::xy(1.0), Outline::circle([8.0, 2.0], 2.0), -7.0)),
    ],
)"
    );
    assert_eq!(
        Case::drawn_square(128).to_string(),
        "Case::new(
    Leaf::prism(Plane::xy(6.0), Outline::circle([6.0, 4.0], 3.5), 5.5),
    vec![
        Step::cut(Leaf::prism(Plane::xy(6.0), Outline::circle([6.0, 4.0], 2.0), 5.5)),
    ],
)"
    );
    assert_eq!(fingerprint(Case::drawn), 0xf3d0_3d25_d6d6_62eb);
    assert_eq!(fingerprint(Case::drawn_square), 0x0347_3e33_46ca_101b);
    assert_eq!(fingerprint(Case::drawn_profiles), 0xc776_8886_b55b_afbd);
}

#[test]
fn no_number_a_fingerprinted_draw_prints_stands_where_the_last_bit_decides_its_count() {
    for (rank, draw) in FINGERPRINTED.into_iter().enumerate() {
        for seed in 0..300 {
            each_number(&draw(seed).to_string(), |value| {
                let past = (value * 1e6 + 1.0 - NEXT_MILLIONTH_FROM).rem_euclid(1.0);
                assert!(
                    past.min(1.0 - past) > CLEAR_OF_THE_MARK,
                    "draw {rank}, seed {seed}: {value} stands {past} of a millionth past \
                     the mark, where a platform's last bit decides its count"
                );
                String::new()
            });
        }
    }
}

#[test]
fn a_case_drawn_differently_changes_the_fingerprint() {
    let moved: fn(u64) -> Case = |seed| Case::drawn(if seed == 7 { 8 } else { seed });

    assert_ne!(fingerprint(moved), fingerprint(Case::drawn));
}

#[test]
fn the_same_seed_draws_the_same_turned_case_and_not_one_of_the_other_draws() {
    for seed in 0..200 {
        assert_eq!(
            Case::drawn_turned(seed),
            Case::drawn_turned(seed),
            "seed {seed}"
        );
    }
    let distinct: std::collections::BTreeSet<String> = (0..200)
        .map(|seed| Case::drawn_turned(seed).to_string())
        .collect();
    assert!(distinct.len() > 190, "{} distinct cases", distinct.len());
    let same = (0..200)
        .filter(|seed| {
            let turned = Case::drawn_turned(*seed);
            turned == Case::drawn_profiles(*seed) || turned == Case::drawn_square(*seed)
        })
        .count();
    assert!(same < 20, "{same} turned cases are another draw's");
}

#[test]
fn every_seed_draws_a_turned_case_though_a_turn_before_has_no_radius_to_draw_from() {
    for seed in 0..20_000 {
        let drawn = std::panic::catch_unwind(|| {
            Case::drawn_turned(seed);
            Case::drawn_turned_off_the_lattice(seed);
            Case::drawn_slanted(seed);
            Case::drawn_slanted_off_the_lattice(seed);
            Case::drawn_rounded(seed);
            Case::drawn_rounded_off_the_lattice(seed);
        });
        assert!(drawn.is_ok(), "seed {seed} draws no case");
    }
}

#[test]
fn a_turned_case_holds_straight_profiles_turned_about_lines_of_the_planes_of_the_origin_and_prisms()
{
    for seed in 0..3000 {
        let case = Case::drawn_turned(seed);
        for leaf in case.leaves() {
            assert!(leaf.is_solid(), "seed {seed}: {case}");
            assert!(
                !matches!(leaf.plane(), Plane::Tilted { .. }),
                "seed {seed}: {case}"
            );
            match leaf {
                Leaf::Prism { outline, .. } => {
                    assert!(
                        !matches!(outline, Outline::Star { .. }),
                        "seed {seed}: {case}"
                    )
                }
                Leaf::Turned { axis, .. } => {
                    assert_eq!(axis.lean, 0.0, "seed {seed}: {case}");
                    let line = axis.line();
                    for side in turned(leaf).expect("a turn").drawn() {
                        for corners in std::iter::once(&side.outline).chain(&side.holes) {
                            for (index, corner) in corners.iter().enumerate() {
                                let run = corners[(index + 1) % corners.len()] - *corner;
                                assert!(
                                    run.dot(line.direction) == 0.0
                                        || run.perp_dot(line.direction) == 0.0,
                                    "seed {seed}, a run leaning off its axis: {case}"
                                );
                            }
                        }
                    }
                }
                Leaf::Revolution { .. } => panic!("seed {seed} drew a revolution: {case}"),
            }
        }
    }
}

/// The world line a leaf turns about, or a circle-like prism is raised
/// along: a point of it and the world axis it runs along.
fn world_line(leaf: &Leaf) -> Option<(DVec3, usize)> {
    match leaf {
        Leaf::Turned { plane, axis, .. } => {
            let line = axis.line();
            let point = plane.to_world(line.origin);
            let direction = plane.to_world(line.origin + line.direction) - point;
            Some((point, direction.abs().max_position()))
        }
        Leaf::Prism {
            plane,
            outline: Outline::Circle { center, .. } | Outline::Ring { center, .. },
            ..
        } => Some((plane.to_world(*center), plane.normal().abs().max_position())),
        _ => None,
    }
}

fn same_line(one: (DVec3, usize), other: (DVec3, usize)) -> bool {
    one.1 == other.1 && (0..3).all(|axis| axis == one.1 || one.0[axis] == other.0[axis])
}

/// Whether two lines, along different world axes, cross.
fn crossing_lines(one: (DVec3, usize), other: (DVec3, usize)) -> bool {
    one.1 != other.1 && {
        let third = 3 - one.1 - other.1;
        one.0[third] == other.0[third]
    }
}

fn radius_of(leaf: &Leaf) -> Option<f64> {
    match leaf {
        Leaf::Prism {
            outline: Outline::Circle { radius, .. },
            ..
        } => Some(*radius),
        _ => None,
    }
}

#[test]
fn the_turned_generator_draws_every_kind_it_keeps() {
    let cases: Vec<Case> = (0..1000).map(Case::drawn_turned).collect();
    let leaves = || cases.iter().flat_map(Case::leaves);
    let turns = || leaves().filter_map(turned);
    let seen = |what: &str, found: bool| assert!(found, "no turned case drew {what}");

    seen(
        "a whole turn",
        turns().any(|turn| turn.degrees.abs() == 360.0),
    );
    seen(
        "a quarter turn's multiple short of a whole one",
        turns().any(|turn| turn.degrees.abs() < 360.0 && turn.degrees % 90.0 == 0.0),
    );
    seen(
        "a turn off the quarter turns",
        turns().any(|turn| turn.degrees % 90.0 != 0.0),
    );
    seen("a turn backwards", turns().any(|turn| turn.degrees < 0.0));
    seen(
        "three bands or more",
        turns().any(|turn| turn.section.bands.len() >= 3),
    );
    seen(
        "an L",
        turns().any(|turn| {
            let bands = &turn.section.bands;
            bands.len() == 2 && bands[0][1] == bands[1][1] && bands[0][1] > 0.0
        }),
    );
    seen(
        "a tube",
        turns().any(|turn| turn.section.bands.iter().all(|band| band[1] > 0.0)),
    );
    seen("a hole", turns().any(|turn| !turn.section.holes.is_empty()));
    seen(
        "a section on its axis",
        turns().any(|turn| turn.section.bands.iter().any(|band| band[1] == 0.0)),
    );
    for (name, sign) in [("on its side", 1.0), ("across", -1.0)] {
        seen(
            &format!("a section a hair {name} its axis"),
            turns().any(|turn| {
                let hair = 1e-5 * turn.section.reach();
                turn.section
                    .bands
                    .iter()
                    .any(|band| band[1] != 0.0 && band[1].abs() < hair && band[1].signum() == sign)
            }),
        );
    }
    seen(
        "a section on the other side of its axis",
        turns().any(|turn| turn.section.bands.iter().all(|band| band[2] <= 0.0)),
    );
    seen(
        "a section across its axis",
        turns().any(|turn| turn.section.side().is_none()),
    );
    seen(
        "a turn about the sketch's axis",
        turns().any(|turn| turn.axis.across == 0.0),
    );
    seen(
        "a turn about a line drawn parallel to it",
        turns().any(|turn| turn.axis.across != 0.0),
    );
    seen(
        "an axis run backwards",
        turns().any(|turn| turn.axis.backwards),
    );
    for (name, kind) in [
        ("XY", Plane::xy(0.0)),
        ("XZ", Plane::xz(0.0)),
        ("YZ", Plane::yz(0.0)),
    ] {
        seen(
            name,
            turns()
                .any(|turn| std::mem::discriminant(&turn.plane) == std::mem::discriminant(&kind)),
        );
    }

    let earlier_and_later = |related: &dyn Fn(&Leaf, &Leaf) -> bool| {
        cases.iter().any(|case| {
            let leaves: Vec<&Leaf> = case.leaves().collect();
            (1..leaves.len()).any(|later| {
                leaves[..later]
                    .iter()
                    .any(|earlier| related(earlier, leaves[later]))
            })
        })
    };
    let coaxial = |one: &Leaf, other: &Leaf| match (world_line(one), world_line(other)) {
        (Some(one), Some(other)) => same_line(one, other),
        _ => false,
    };
    seen(
        "a turn coaxial with a circle raised before it",
        earlier_and_later(&|earlier, later| {
            radius_of(earlier).is_some() && turned(later).is_some() && coaxial(earlier, later)
        }),
    );
    seen(
        "a turn whose wall is flush with a circle raised before it",
        earlier_and_later(&|earlier, later| {
            radius_of(earlier).is_some_and(|radius| {
                coaxial(earlier, later)
                    && turned(later).is_some_and(|turn| {
                        turn.section
                            .bands
                            .iter()
                            .any(|band| band[2].abs() == radius)
                    })
            })
        }),
    );
    seen(
        "a turn with a shoulder flush with the end of a circle raised before it",
        earlier_and_later(&|earlier, later| {
            let (Some(turn), Leaf::Prism { plane, height, .. }) = (turned(later), earlier) else {
                return false;
            };
            let Some((_, axis)) = world_line(earlier) else {
                return false;
            };
            let floor = plane.to_world(DVec2::ZERO)[axis];
            let caps = [floor, floor + plane.normal()[axis] * height];
            coaxial(earlier, later)
                && turn.section.ends().iter().any(|end| {
                    let level = if turn.axis.backwards { -end } else { *end };
                    caps.contains(&level)
                })
        }),
    );
    seen(
        "a turn touching a circle raised before it from across it",
        earlier_and_later(&|earlier, later| {
            let (Some(radius), Some(turn)) = (radius_of(earlier), turned(later)) else {
                return false;
            };
            let (Some(one), Some(other)) = (world_line(earlier), world_line(later)) else {
                return false;
            };
            one.1 != other.1
                && turn.section.bands.len() == 1
                && (one.0[3 - one.1 - other.1] - other.0[3 - one.1 - other.1]).abs()
                    == radius + turn.section.bands[0][2].abs()
        }),
    );
    seen(
        "a turn about the edge of a block raised before it",
        earlier_and_later(&|earlier, later| {
            let (
                Leaf::Prism {
                    plane,
                    outline: Outline::Rectangle { low, high },
                    height,
                },
                Some(line),
            ) = (earlier, world_line(later))
            else {
                return false;
            };
            let floor = plane.to_world(DVec2::ZERO);
            let caps = [floor, floor + plane.normal() * *height];
            [low.x, high.x, low.y, high.y].iter().any(|edge| {
                caps.iter().any(|cap| {
                    let (_, u, v) = plane.frame();
                    [u, v].iter().any(|across| {
                        let corner = *cap + *across * *edge;
                        (0..3).all(|axis| axis == line.1 || corner[axis] == line.0[axis])
                    })
                })
            })
        }),
    );
    seen(
        "a turn tangent to the top or the floor of a block raised before it",
        earlier_and_later(&|earlier, later| {
            let (
                Leaf::Prism {
                    plane,
                    outline: Outline::Rectangle { .. },
                    height,
                },
                Some(turn),
                Some((point, along)),
            ) = (earlier, turned(later), world_line(later))
            else {
                return false;
            };
            let normal = plane.normal();
            let across = normal.abs().max_position();
            let floor = plane.to_world(DVec2::ZERO)[across];
            let radius = turn.section.bands[0][2].abs();
            along != across
                && turn.section.bands.len() == 1
                && [floor, floor + normal[across] * height]
                    .iter()
                    .any(|cap| (point[across] - cap).abs() == radius)
        }),
    );
    seen(
        "two turns about one line",
        earlier_and_later(&|earlier, later| {
            turned(earlier).is_some() && turned(later).is_some() && coaxial(earlier, later)
        }),
    );
    seen(
        "a groove across the outer wall of a turn before it",
        earlier_and_later(&|earlier, later| {
            let (Some(first), Some(second)) = (turned(earlier), turned(later)) else {
                return false;
            };
            let outer = first
                .section
                .bands
                .iter()
                .map(|band| band[1].abs().max(band[2].abs()))
                .fold(0.0, f64::max);
            coaxial(earlier, later)
                && second.section.bands.iter().any(|band| {
                    let (near, far) = (
                        band[1].abs().min(band[2].abs()),
                        band[1].abs().max(band[2].abs()),
                    );
                    near < outer && outer < far
                })
        }),
    );
    seen(
        "a hole across a turn before it",
        earlier_and_later(
            &|earlier, later| match (world_line(earlier), world_line(later)) {
                (Some(one), Some(other)) => {
                    turned(earlier).is_some()
                        && radius_of(later).is_some()
                        && crossing_lines(one, other)
                }
                _ => false,
            },
        ),
    );

    let count = leaves().count();
    let share = turns().count() as f64 / count as f64;
    assert!(
        (0.35..0.65).contains(&share),
        "turns are {} leaves of {count}",
        turns().count()
    );
}

#[test]
fn a_section_across_its_axis_is_handed_over_as_its_two_sides() {
    let leaf = Leaf::turned(
        Plane::xz(1.0),
        Axis::second(2.0),
        Section::bands(0.0, &[[2.0, -4.0, 5.0], [1.0, -1.0, 3.0]])
            .with_holes(&[([0.5, 1.0], [1.5, 2.0])]),
        90.0,
    );
    let turn = turned(&leaf).expect("a turn");
    let sides = turn.section.sides();
    assert_eq!(
        sides,
        vec![
            Section::bands(0.0, &[[2.0, 0.0, 5.0], [1.0, 0.0, 3.0]])
                .with_holes(&[([0.5, 1.0], [1.5, 2.0])]),
            Section::bands(0.0, &[[2.0, -4.0, 0.0], [1.0, -1.0, 0.0]]),
        ]
    );
    let measure = |pieces: Vec<Piece>, side: f64| {
        pieces
            .iter()
            .filter(|piece| piece.side == side)
            .map(|piece| {
                let length = piece.along[1] - piece.along[0];
                DVec2::new(
                    length * (piece.away[1] - piece.away[0]),
                    length * (piece.away[1].powi(2) - piece.away[0].powi(2)),
                )
            })
            .sum::<DVec2>()
    };
    for side in [1.0, -1.0] {
        let halves = sides
            .iter()
            .map(|half| measure(half.pieces(), side))
            .sum::<DVec2>();
        assert_eq!(halves, measure(turn.section.pieces(), side), "side {side}");
    }

    let across: Vec<Turned> = drawn_turns(1000)
        .iter()
        .filter_map(turned)
        .filter(|turn| turn.section.side().is_none())
        .chain(std::iter::once(turn))
        .collect();
    assert!(across.len() > 20, "{} sections across", across.len());
    for turn in across {
        let drawn = turn.drawn();
        assert_eq!(drawn.len(), 2, "{:?}", turn.section);
        let line = turn.turn(0.0);
        for (side, expected) in drawn.iter().zip([1.0, -1.0]) {
            assert_eq!(
                line.lie(&side.profile(true)),
                Lie::Side(expected),
                "{:?}",
                turn.section
            );
        }
        for side in turn.section.sides() {
            assert!(side.is_solid(), "{side:?} of {:?}", turn.section);
        }
    }
}

/// What the corners a kernel is handed sweep per radian, by Pappus: the
/// first moment of the area about the axis, read off the corners alone,
/// those within a hair of the axis laid on it as the arithmetic lays them —
/// which moves a slant from such a corner by more than the hair's square.
fn moment(turn: &Turned) -> Vec<f64> {
    let line = turn.axis.line();
    let snap = ON_THE_AXIS * turn.section.reach();
    let snapped = |s: f64| if s.abs() <= snap { 0.0 } else { s };
    turn.drawn()
        .iter()
        .map(|side| {
            std::iter::once(&side.outline)
                .chain(&side.holes)
                .map(|corners| {
                    let read: Vec<DVec2> = corners
                        .iter()
                        .map(|corner| {
                            DVec2::new(
                                (*corner - line.origin).dot(line.direction),
                                snapped(line.side(*corner)),
                            )
                        })
                        .collect();
                    (0..read.len())
                        .map(|index| {
                            let (one, other) = (read[index], read[(index + 1) % read.len()]);
                            one.perp_dot(other) * (one.y + other.y) / 6.0
                        })
                        .sum::<f64>()
                })
                .sum::<f64>()
                .abs()
        })
        .collect()
}

#[test]
fn the_pieces_of_a_section_turn_into_pappus_s_volume_of_its_corners() {
    let block = Section::bands(0.0, &[[4.0, 2.0, 5.0]]);
    for degrees in [90.0, -90.0] {
        let leaf = Leaf::turned(Plane::xy(0.0), Axis::first(0.0), block.clone(), degrees);
        let volume = turned(&leaf).expect("a turn").volume();
        assert!((volume - 21.0 * PI).abs() < 1e-12, "{volume}");
    }
    let across = Section::bands(0.0, &[[2.0, -4.0, 5.0]]);
    for (degrees, expected) in [(360.0, 50.0 * PI), (90.0, 20.5 * PI)] {
        let leaf = Leaf::turned(Plane::xy(0.0), Axis::second(0.0), across.clone(), degrees);
        let volume = turned(&leaf).expect("a turn").volume();
        assert!((volume - expected).abs() < 1e-12, "{degrees}: {volume}");
    }

    let turns: Vec<Turned> = drawn_turns(1000).iter().filter_map(turned).collect();
    assert!(turns.len() > 300, "{} turns", turns.len());
    for turn in turns {
        let sides = moment(&turn);
        let pappus: f64 = sides.iter().map(|side| side * turn.angle()).sum();
        let volume = turn.volume();
        if sides.len() == 1 || turn.degrees.abs() <= 180.0 {
            assert!(
                (volume - pappus).abs() <= 1e-9 * pappus,
                "{:?}: {volume} against {pappus}",
                turn.section
            );
        } else {
            let largest = sides.iter().copied().fold(0.0, f64::max) * turn.angle();
            assert!(
                largest <= volume * (1.0 + 1e-9) && volume <= pappus * (1.0 + 1e-9),
                "{:?}: {volume} between {largest} and {pappus}",
                turn.section
            );
        }
    }
}

/// The lone turns among `leaves` that the flats answer by their own rules,
/// turned forwards and off their axis: where the flats can be trusted with
/// a turn.
fn turns_the_flats_hold(leaves: Vec<Leaf>) -> Vec<Leaf> {
    let leaves: Vec<Leaf> = leaves
        .into_iter()
        .filter(|leaf| {
            turned(leaf).is_some_and(|turn| {
                turn.degrees > 0.0
                    && turn.section.side().is_some()
                    && turn
                        .section
                        .pieces()
                        .iter()
                        .all(|piece| piece.away[0] > 0.0 && piece.ending[0] > 0.0)
            })
        })
        .collect();
    let kept = random_solids::on_every_core(&leaves, |leaf| {
        random_solids::check(&Case::new(leaf.clone(), vec![])).is_ok()
    });
    leaves
        .into_iter()
        .zip(kept)
        .filter_map(|(leaf, kept)| kept.then_some(leaf))
        .collect()
}

/// How far in a flat laid across a step of a turn stands from the circle:
/// the flats take at most a sixty-fourth of a turn a step.
fn step_cosine() -> f64 {
    (PI / 64.0).cos()
}

/// Whether the flats a leaf is turned into hold along every line what the
/// leaf holds with its outer walls drawn in by their sagitta, and no more
/// than it holds with its inner walls drawn in by theirs. A leaf an edge of
/// whose section bows is held between itself shrunk and grown by the
/// sagittas of its profile's sampling and of its turn together: a circle
/// drawn in by a share of its radius is an ellipse, which no piece holds.
fn holds_what_its_flats_hold(leaf: &Leaf, flats: &cao_solid::Body) {
    let turn = turned(leaf).expect("a turn");
    let (lines, reach) = lines_over(leaf);
    let measured = lines.inside(&flats.triangles());
    let room = 1e-9 * reach;
    if turn.section.bows() {
        let sagitta = random_solids::sagitta_of(&turn);
        for (index, measure) in measured.iter().enumerate() {
            let (origin, direction) = lines.line(index);
            let least = spans_of(&turn.along_grown(origin, direction, -sagitta));
            let most = spans_of(&turn.along_grown(origin, direction, sagitta));
            assert!(
                least.without(measure).length() <= room && measure.without(&most).length() <= room,
                "line {index} across {leaf}: {measure:?} not between {least:?} and {most:?}"
            );
        }
        return;
    }
    let pieces = turn.section.pieces();
    let least: Vec<Piece> = pieces
        .iter()
        .map(|piece| Piece {
            away: [piece.away[0], piece.away[1] * step_cosine()],
            ending: [piece.ending[0], piece.ending[1] * step_cosine()],
            ..*piece
        })
        .collect();
    let most: Vec<Piece> = pieces
        .iter()
        .map(|piece| Piece {
            away: [piece.away[0] * step_cosine(), piece.away[1]],
            ending: [piece.ending[0] * step_cosine(), piece.ending[1]],
            ..*piece
        })
        .collect();
    for (index, measure) in measured.iter().enumerate() {
        let (origin, direction) = lines.line(index);
        let least = spans_of(&turn.along_pieces(&least, origin, direction, 0.0));
        let most = spans_of(&turn.along_pieces(&most, origin, direction, 0.0));
        assert!(
            least.without(measure).length() <= room && measure.without(&most).length() <= room,
            "line {index} across {leaf}: {measure:?} not between {least:?} and {most:?}"
        );
    }
}

/// Whether the box a leaf spans holds the box its flats span, and stands
/// past it by no more than their sagitta, the profile's sampling counted
/// with the turn's where an edge bows.
fn spans_the_box_its_flats_span(leaf: &Leaf, flats: &cao_solid::Body) {
    let turn = turned(leaf).expect("a turn");
    let (low, high) = leaf.bounds().expect("a box");
    let (flat_low, flat_high) = flats.bounds().expect("a box of flats");
    let reach = low.abs().max(high.abs()).max_element();
    let sagitta = random_solids::sagitta_of(&turn) + 1e-9 * reach;
    assert!(
        (low - flat_low).max_element() <= 1e-9 * reach
            && (flat_high - high).max_element() <= 1e-9 * reach,
        "{leaf}: {low} {high} against the flats' {flat_low} {flat_high}"
    );
    assert!(
        (flat_low - low).max_element() <= sagitta && (high - flat_high).max_element() <= sagitta,
        "{leaf}: {low} {high} past the flats' {flat_low} {flat_high}"
    );
}

#[test]
fn a_turned_leaf_off_its_axis_holds_along_every_line_what_its_flats_hold_but_for_their_sagitta() {
    let leaves = turns_the_flats_hold(drawn_turns(400));
    assert!(leaves.len() > 60, "{} leaves", leaves.len());
    for leaf in &leaves {
        holds_what_its_flats_hold(leaf, &leaf.solid().expect("a turn the flats make"));
    }
}

#[test]
fn the_box_a_turned_leaf_spans_holds_the_box_its_flats_span_within_their_sagitta() {
    for leaf in &turns_the_flats_hold(drawn_turns(400)) {
        spans_the_box_its_flats_span(leaf, &leaf.solid().expect("a turn the flats make"));
    }
}

/// A line turned about the axis of a turn by `turn` radians.
fn turned_line(turn: &Turned, radians: f64, (origin, direction): (DVec3, DVec3)) -> (DVec3, DVec3) {
    let swept = turn.swept(1.0);
    let rotation = DQuat::from_axis_angle(swept.along, radians);
    (
        swept.origin + rotation * (origin - swept.origin),
        rotation * direction,
    )
}

#[test]
fn a_turn_backwards_holds_along_every_line_what_the_turn_forwards_holds_along_the_line_turned_back()
{
    let leaves: Vec<Leaf> = drawn_turns(300)
        .into_iter()
        .filter(|leaf| turned(leaf).is_some_and(|turn| turn.degrees > 0.0))
        .collect();
    assert!(leaves.len() > 100, "{} leaves", leaves.len());
    for leaf in &leaves {
        let forwards = turned(leaf).expect("a turn");
        let backwards = Turned {
            degrees: -forwards.degrees,
            ..forwards.clone()
        };
        let (lines, reach) = lines_over(leaf);
        for index in 0..lines.count() {
            let line = lines.line(index);
            let (origin, direction) = turned_line(&forwards, forwards.degrees.to_radians(), line);
            let back = spans_of(&backwards.along_grown(line.0, line.1, 0.0));
            let forth = spans_of(&forwards.along_grown(origin, direction, 0.0));
            assert!(
                back.without(&forth).length() + forth.without(&back).length() <= 1e-9 * reach,
                "line {index} across {leaf}: {back:?} against {forth:?}"
            );
        }
    }
}

#[test]
fn a_line_through_a_turned_leaf_is_told_how_squarely_it_crosses_each_wall_and_whether_it_curves() {
    let tube = Leaf::turned(
        Plane::xy(0.0),
        Axis::first(0.0),
        Section::bands(0.0, &[[4.0, 1.0, 3.0]]),
        360.0,
    );
    let along = tube
        .along(DVec3::new(-1.0, 2.0, 0.0), DVec3::X)
        .expect("a turn");
    let [stretch] = along.as_slice() else {
        panic!("{along:?}");
    };
    assert_eq!((stretch.from.at, stretch.to.at), (1.0, 5.0));
    for end in [stretch.from, stretch.to] {
        assert_eq!((end.cosine, end.curved), (1.0, false));
    }

    let across = tube
        .along(DVec3::new(2.0, 0.0, -5.0), DVec3::Z)
        .expect("a turn");
    let ends: Vec<(f64, bool)> = across
        .iter()
        .flat_map(|stretch| [stretch.from, stretch.to])
        .map(|end| (end.at, end.curved))
        .collect();
    assert_eq!(
        ends,
        vec![(2.0, true), (4.0, true), (6.0, true), (8.0, true)]
    );

    let quarter = Leaf::turned(
        Plane::xy(0.0),
        Axis::first(0.0),
        Section::bands(0.0, &[[4.0, 1.0, 3.0]]),
        90.0,
    );
    let through = quarter
        .along(DVec3::new(2.0, 2.0, -5.0), DVec3::Z)
        .expect("a turn");
    let [stretch] = through.as_slice() else {
        panic!("{through:?}");
    };
    assert_eq!(
        (stretch.from.at, stretch.from.cosine, stretch.from.curved),
        (5.0, 1.0, false)
    );
    assert!(
        (stretch.to.at - 5.0 - 5f64.sqrt()).abs() < 1e-12,
        "{stretch:?}"
    );
    assert!(stretch.to.curved);
    assert!(
        (stretch.to.cosine - 5f64.sqrt() / 3.0).abs() < 1e-12,
        "{stretch:?}"
    );

    let backwards = Leaf::turned(
        Plane::xy(0.0),
        Axis::first(0.0),
        Section::bands(0.0, &[[4.0, 1.0, 3.0]]),
        -90.0,
    );
    let below = backwards
        .along(DVec3::new(2.0, 2.0, -5.0), DVec3::Z)
        .expect("a turn");
    let [stretch] = below.as_slice() else {
        panic!("{below:?}");
    };
    assert!(
        (stretch.from.at - 5.0 + 5f64.sqrt()).abs() < 1e-12,
        "{stretch:?}"
    );
    assert_eq!((stretch.to.at, stretch.to.curved), (5.0, false));
}

#[test]
fn a_turned_leaf_grown_holds_every_line_it_held_and_shrunk_holds_none_it_did_not() {
    for leaf in drawn_turns(200) {
        let (lines, reach) = lines_over(&leaf);
        let by = 1e-3 * reach;
        for index in 0..lines.count() {
            let (origin, direction) = lines.line(index);
            let [shrunk, held, grown] = [-by, 0.0, by]
                .map(|by| spans_of(&leaf.along_grown(origin, direction, by).expect("a turn")));
            assert_eq!(
                shrunk.without(&held).length(),
                0.0,
                "line {index} of {leaf}"
            );
            assert_eq!(held.without(&grown).length(), 0.0, "line {index} of {leaf}");
            assert!(grown.length() > shrunk.length() || grown.length() == 0.0);
        }
    }
}

#[test]
fn a_case_asking_beyond_the_conics_is_told_from_one_that_does_not() {
    let shaft = |degrees: f64, axis: Axis| {
        Leaf::turned(
            Plane::xy(0.0),
            axis,
            Section::bands(-2.0, &[[6.0, 0.0, 2.0]]),
            degrees,
        )
    };
    let post = Leaf::prism(Plane::xy(-3.0), Outline::circle([1.0, 1.0], 1.0), 6.0);
    let bar = Leaf::prism(Plane::yz(-3.0), Outline::circle([0.0, 0.0], 1.0), 6.0);
    let block = Leaf::prism(
        Plane::xy(-1.0),
        Outline::rectangle([0.0, 0.0], [3.0, 3.0]),
        2.0,
    );
    let asks = |start: &Leaf, tool: &Leaf| {
        Case::new(start.clone(), vec![Step::cut(tool.clone())]).asks_beyond_the_conics()
    };

    for degrees in [360.0, 90.0, -180.0, 270.0] {
        assert!(!asks(&shaft(degrees, Axis::first(0.0)), &post), "{degrees}");
    }
    for degrees in [30.0, -105.0, 359.9] {
        assert!(!asks(&shaft(degrees, Axis::first(0.0)), &post), "{degrees}");
        assert!(!asks(&shaft(degrees, Axis::first(0.0)), &bar), "{degrees}");
        assert!(
            !asks(&shaft(degrees, Axis::first(0.0)), &block),
            "{degrees}"
        );
    }
    let slanted = Axis::first(0.0).leaning(30.0);
    assert!(!asks(&shaft(360.0, slanted), &block));
    assert!(asks(&shaft(360.0, slanted), &bar));
    assert!(!Case::new(shaft(45.0, slanted), vec![]).asks_beyond_the_conics());
    assert!(!asks(&post, &block));

    let asking = (0..1000)
        .filter(|seed| Case::drawn_turned_off_the_lattice(*seed).asks_beyond_the_conics())
        .count();
    assert!(
        (50..250).contains(&asking),
        "{asking} cases ask beyond the conics"
    );

    let cone = |degrees: f64| {
        Leaf::turned(
            Plane::xy(0.0),
            Axis::first(0.0),
            Section::bands(0.0, &[[4.0, 0.0, 3.0]]).sloping_to(&[[0.0, 1.0]]),
            degrees,
        )
    };
    let about = |axis: Axis| {
        Leaf::turned(
            Plane::xz(0.0),
            axis,
            Section::bands(2.0, &[[4.0, 0.0, 1.0]]).sloping_to(&[[0.0, 3.0]]),
            360.0,
        )
    };
    let told = [
        (
            "a cap square to its axis",
            Leaf::prism(Plane::yz(-1.0), Outline::circle([0.0, 0.0], 2.0), 3.0),
            false,
        ),
        (
            "a block whose face holds its axis",
            Leaf::prism(
                Plane::xy(0.0),
                Outline::rectangle([-1.0, -5.0], [5.0, 5.0]),
                10.0,
            ),
            false,
        ),
        (
            "a flat parallel to its axis and off it",
            Leaf::prism(
                Plane::xy(1.0),
                Outline::rectangle([-1.0, -5.0], [5.0, 5.0]),
                10.0,
            ),
            false,
        ),
        (
            "a flat tangent to its widest rim",
            Leaf::prism(
                Plane::xy(3.0),
                Outline::rectangle([-1.0, -5.0], [5.0, 5.0]),
                10.0,
            ),
            false,
        ),
        (
            "a flat a hair past its widest rim, closer than the kernel tells clear",
            Leaf::prism(
                Plane::xy(3.00000002),
                Outline::rectangle([-1.0, -5.0], [5.0, 5.0]),
                10.0,
            ),
            false,
        ),
        (
            "a flat past its widest rim",
            Leaf::prism(
                Plane::xy(3.001),
                Outline::rectangle([-1.0, -5.0], [5.0, 5.0]),
                10.0,
            ),
            false,
        ),
        (
            "a plane at a slant",
            Leaf::prism(
                Plane::tilted([0.0, 0.0, 0.0], [0.0, 30.0, 0.0]),
                Outline::rectangle([-5.0, -5.0], [5.0, 5.0]),
                1.0,
            ),
            false,
        ),
        (
            "a cylinder about its axis",
            Leaf::prism(Plane::yz(1.0), Outline::circle([0.0, 0.0], 2.0), 2.0),
            false,
        ),
        ("a cone about its axis", about(Axis::first(0.0)), false),
        (
            "a cylinder parallel to its axis and off it",
            Leaf::prism(Plane::yz(1.0), Outline::circle([1.0, 0.0], 1.0), 2.0),
            true,
        ),
        (
            "a hole across it",
            Leaf::prism(Plane::xz(5.0), Outline::circle([2.0, 0.0], 1.0), 10.0),
            true,
        ),
        ("a cone a hair off its axis", about(Axis::first(1e-7)), true),
        (
            "a block whose box stands apart from its own",
            Leaf::prism(
                Plane::xy(1.0),
                Outline::rectangle([20.0, 20.0], [25.0, 25.0]),
                2.0,
            ),
            false,
        ),
    ];
    for (what, tool, asking) in told {
        assert_eq!(asks(&cone(360.0), &tool), asking, "{what}");
    }
    assert!(!Case::new(cone(90.0), vec![]).asks_beyond_the_conics());
    assert!(!Case::new(cone(-215.0), vec![]).asks_beyond_the_conics());

    let asking = (0..1000)
        .filter(|seed| Case::drawn_slanted(*seed).asks_beyond_the_conics())
        .count();
    assert!(
        (200..450).contains(&asking),
        "{asking} slanted cases ask beyond the conics"
    );
}

#[test]
fn a_plane_leaning_about_an_axis_of_the_origin_holds_that_axis_and_leans_by_its_angle() {
    let origin = DVec3::new(1.0, -2.0, 3.5);
    for degrees in [15.0, -30.0, 45.0, 75.0, 2e-8, -1e-5, 90.0 - 2e-8] {
        let radians = f64::to_radians(degrees);
        let expected = [
            DVec3::new(0.0, -radians.sin(), radians.cos()),
            DVec3::new(radians.sin(), 0.0, radians.cos()),
            DVec3::new(radians.sin(), -radians.cos(), 0.0),
        ];
        for (holds, (starts, expected)) in [DVec3::Z, DVec3::Z, DVec3::NEG_Y]
            .into_iter()
            .zip(expected)
            .enumerate()
        {
            let plane = Plane::leaning(holds, degrees, origin);
            let normal = plane.normal();
            assert_eq!(plane.to_world(DVec2::ZERO), origin);
            assert!(
                normal.dot(DVec3::AXES[holds]).abs() <= 1e-15,
                "holding axis {holds} by {degrees}: {normal}"
            );
            assert!(
                (normal - expected).length() <= 1e-15,
                "holding axis {holds} by {degrees}: {normal} against {expected}"
            );
            let leaned = normal.cross(starts).length().atan2(normal.dot(starts));
            assert!(
                (leaned.to_degrees() - degrees.abs()).abs() <= 1e-12,
                "holding axis {holds} by {degrees}: leaned {}",
                leaned.to_degrees()
            );
        }
    }
}

/// A disc of radius nine on a plane through `on` holding a world axis of
/// the origin and leaning `degrees` off it, raised a unit: its two faces
/// lean, its wall stands square to them, and it has no side to lean too.
fn disc(holds: usize, degrees: f64, on: [f64; 3]) -> Leaf {
    Leaf::prism(
        Plane::leaning(holds, degrees, DVec3::from(on)),
        Outline::circle([0.0, 0.0], 9.0),
        1.0,
    )
}

fn told(counts: [usize; 6]) -> random_solids::Conics {
    let [
        ellipses,
        parabolas,
        hyperbolas,
        lines_through_an_apex,
        apex_alone,
        tangent_rulings,
    ] = counts;
    random_solids::Conics {
        ellipses,
        parabolas,
        hyperbolas,
        lines_through_an_apex,
        apex_alone,
        tangent_rulings,
    }
}

#[test]
fn a_plane_meeting_a_cylinder_or_a_cone_is_told_by_the_conic_it_cuts() {
    let conics =
        |start: &Leaf, tool: Leaf| Case::new(start.clone(), vec![Step::cut(tool)]).conics();
    let shaft = Leaf::prism(Plane::xy(0.0), Outline::circle([0.0, 0.0], 5.0), 10.0);
    for (what, tool, expected) in [
        (
            "a plane square to the axis",
            Leaf::prism(Plane::xy(4.0), Outline::circle([0.0, 0.0], 9.0), 1.0),
            0,
        ),
        ("a slant", disc(0, 30.0, [0.0, 0.0, 5.0]), 4),
        (
            "a slant within the rounding of square",
            disc(0, 1e-11, [0.0, 0.0, 5.0]),
            0,
        ),
        (
            "a slant a hair off square",
            disc(0, 1e-8, [0.0, 0.0, 5.0]),
            4,
        ),
        (
            "a slant a hair off its axis",
            disc(0, 90.0 - 1e-8, [0.0, 0.0, 5.0]),
            4,
        ),
        (
            "a plane along its axis at its radius, a line",
            disc(0, 90.0, [0.0, 5.0, 5.0]),
            0,
        ),
    ] {
        assert_eq!(
            conics(&shaft, tool),
            told([expected, 0, 0, 0, 0, 0]),
            "{what}"
        );
    }

    let point = Leaf::turned(
        Plane::xz(0.0),
        Axis::second(0.0),
        Section::bands(0.0, &[[10.0, 0.0, 5.0]]).sloping_to(&[[0.0, 0.0]]),
        360.0,
    );
    let steep = 90.0 - 0.5f64.atan().to_degrees();
    for (what, tool, expected) in [
        (
            "a slant steeper than the slope",
            disc(0, 15.0, [0.0, 0.0, 6.0]),
            [6, 0, 0, 0, 0, 0],
        ),
        (
            "a slant as steep as the slope",
            disc(0, steep, [0.0, 0.0, 6.0]),
            [4, 2, 0, 0, 0, 0],
        ),
        (
            "a slant a hair steeper",
            disc(0, steep - 1e-9, [0.0, 0.0, 6.0]),
            [6, 0, 0, 0, 0, 0],
        ),
        (
            "a slant a hair flatter",
            disc(0, steep + 1e-9, [0.0, 0.0, 6.0]),
            [4, 0, 2, 0, 0, 0],
        ),
        (
            "a flat along the axis",
            disc(0, 90.0, [0.0, 2.0, 6.0]),
            [0, 0, 2, 0, 0, 0],
        ),
        (
            "a flatter plane through the tip",
            disc(0, 75.0, [0.0, 0.0, 10.0]),
            [4, 0, 1, 1, 0, 0],
        ),
        (
            "a steeper plane through the tip",
            disc(0, 15.0, [0.0, 0.0, 10.0]),
            [5, 0, 0, 0, 1, 0],
        ),
        (
            "a plane through the tip as steep",
            disc(0, steep, [0.0, 0.0, 10.0]),
            [4, 1, 0, 0, 0, 1],
        ),
        (
            "as steep a hair off the tip",
            disc(0, steep, [0.0, 0.0, 10.000001]),
            [4, 2, 0, 0, 0, 0],
        ),
        (
            "a plane holding the axis",
            disc(0, 90.0, [0.0, 0.0, 6.0]),
            [0, 0, 1, 0, 0, 0],
        ),
        (
            "a slant whose box stands apart",
            disc(0, 15.0, [60.0, 60.0, 60.0]),
            [4, 0, 0, 0, 0, 0],
        ),
    ] {
        assert_eq!(conics(&point, tool), told(expected), "{what}");
    }
}

/// The area of a disc of `radius` beyond a chord `distance` from its centre.
fn beyond_a_chord(radius: f64, distance: f64) -> f64 {
    radius * radius * (distance / radius).acos()
        - distance * (radius * radius - distance * distance).sqrt()
}

#[test]
fn the_slices_of_a_round_leaf_cut_by_a_plane_add_up_to_the_closed_forms() {
    let tan30 = 30f64.to_radians().tan();
    let tan15 = 15f64.to_radians().tan();
    let shaft = Leaf::prism(Plane::xy(0.0), Outline::circle([0.0, 0.0], 5.0), 30.0);
    let point = Leaf::turned(
        Plane::xz(0.0),
        Axis::second(0.0),
        Section::bands(0.0, &[[10.0, 0.0, 5.0]]).sloping_to(&[[0.0, 0.0]]),
        360.0,
    );
    let face = DVec3::new(10.0, 10.0, 5.0 + 10.0 * tan30);
    let hole = Leaf::prism(
        Plane::leaning(0, 30.0, face),
        Outline::circle([0.0, 0.0], 2.0),
        -20.0,
    );
    let wedge = 2000.0 + 4000.0 / 3f64.sqrt();
    let bore = Leaf::prism(Plane::xy(0.0), Outline::circle([33.0, -10.0], 2.0), 12.0);
    let chamfered = Leaf::turned(
        Plane::xz(0.0),
        Axis::second(0.0),
        Section::bands(0.0, &[[26.0, 0.0, 10.0], [4.0, 0.0, 10.0]])
            .sloping_to(&[[0.0, 10.0], [0.0, 6.0]]),
        360.0,
    );
    let flat = |rho: f64| {
        let d: f64 = 8.0;
        let root = (rho * rho - d * d).sqrt();
        rho.powi(3) / 3.0 * (d / rho).acos() - 2.0 / 3.0 * d * rho * root
            + d.powi(3) / 3.0 * (rho + root).ln()
    };
    let tube = Leaf::prism(Plane::xy(0.0), Outline::ring([0.0, 0.0], 5.0, 3.0), 30.0);
    let across = Leaf::turned(
        Plane::xz(0.0),
        Axis::second(0.0),
        Section::bands(0.0, &[[30.0, -2.0, 3.0]]),
        360.0,
    );
    let slant = |leaf: &Leaf| leaf.kept_by_a_half_space(DVec3::new(0.0, -tan30, 1.0), 15.0);
    let kept = [
        ("a shaft cut at a slant", slant(&shaft), 375.0 * PI, None),
        (
            "a hole square into a slanted face, out through the floor",
            hole.kept_by_a_half_space(DVec3::NEG_Z, 0.0)
                .map(|hole| wedge - hole),
            wedge - 4.0 * PI * (10.0 / 3f64.sqrt() + 20.0 / 3.0),
            Some(4153.0732980934),
        ),
        (
            "a bore crossing a slanted side",
            bore.kept_by_a_half_space(DVec3::new(1.0, 0.0, 1.0), 40.0)
                .map(|bore| 7000.0 - bore),
            7000.0 - 28.0 * PI,
            None,
        ),
        (
            "an ungula",
            shaft.kept_by_a_half_space(DVec3::new(0.0, -2.0, 1.0), 0.0),
            500.0 / 3.0,
            None,
        ),
        (
            "a point cut obliquely, an ellipse, its tip's side",
            point.kept_by_a_half_space(
                DVec3::new(0.0, 15f64.to_radians().sin(), -15f64.to_radians().cos()),
                -6.0 * 15f64.to_radians().cos(),
            ),
            16.0 * PI / 3.0 * (1.0 - tan15 * tan15 / 4.0).powf(-1.5),
            Some(17.2166109221),
        ),
        (
            "a point cut parallel to a ruling, a parabola, its tip's side",
            point.kept_by_a_half_space(DVec3::new(0.0, -2.0, -1.0), -2.0),
            (250.0 * 0.2f64.acos() + 44.0 * 6f64.sqrt()) / 3.0,
            Some(150.0457167279),
        ),
        (
            "a point cut through its tip, two lines, beyond them",
            point.kept_by_a_half_space(DVec3::new(0.0, -1.0, -tan15), -10.0 * tan15),
            10.0 / 3.0 * beyond_a_chord(5.0, 10.0 * tan15),
            Some(46.0646029578),
        ),
        (
            "a flat on a chamfer, a hyperbola",
            chamfered.kept_by_a_half_space(DVec3::Y, 8.0),
            8584.0 * PI / 3.0 - 26.0 * beyond_a_chord(10.0, 8.0) - (flat(10.0) - flat(8.0)),
            Some(8551.2434081957),
        ),
        ("a tube cut at a slant", slant(&tube), 240.0 * PI, None),
        (
            "a section across its axis cut at a slant",
            slant(&across),
            135.0 * PI,
            None,
        ),
        (
            "a shaft cut square to its axis",
            shaft.kept_by_a_half_space(DVec3::Z, 12.0),
            300.0 * PI,
            None,
        ),
    ];
    for (what, sliced, closed, printed) in kept {
        let sliced = sliced.unwrap_or_else(|| panic!("{what}: a round leaf"));
        assert!(
            (sliced - closed).abs() <= 1e-12 * closed,
            "{what}: {sliced} by slices against {closed}"
        );
        if let Some(printed) = printed {
            assert!(
                (closed - printed).abs() <= 1e-10,
                "{what}: {closed} against {printed}"
            );
        }
    }
    let partial = Leaf::turned(
        Plane::xz(0.0),
        Axis::second(0.0),
        Section::bands(0.0, &[[10.0, 0.0, 5.0]]),
        90.0,
    );
    let block = Leaf::prism(
        Plane::xy(0.0),
        Outline::rectangle([0.0, 0.0], [1.0, 1.0]),
        1.0,
    );
    assert_eq!(partial.kept_by_a_half_space(DVec3::Z, 1.0), None);
    assert_eq!(block.kept_by_a_half_space(DVec3::Z, 1.0), None);
}

#[test]
fn a_turned_case_prints_as_the_rust_that_builds_it_again() {
    let pasted = Case::new(
        Leaf::turned(
            Plane::xz(2.0),
            Axis::second(0.0).backwards(),
            Section::bands(0.0, &[[3.0, 0.0, 2.0], [1.5, 0.0, 1.0]])
                .with_holes(&[([0.5, 0.5], [1.0, 1.5])]),
            -90.0,
        ),
        vec![
            Step::cut(Leaf::turned(
                Plane::xy(-1.0),
                Axis::first(2.5).leaning(1e-7),
                Section::bands(-1.0, &[[2.0, -1.0, 3.0]]),
                360.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(4.0),
                Outline::circle([5.0, 3.0], 2.5),
                2.0,
            )),
        ],
    );
    assert_eq!(
        pasted.to_string(),
        "Case::new(
    Leaf::turned(Plane::xz(2.0), Axis::second(0.0).backwards(), Section::bands(0.0, &[[3.0, 0.0, 2.0], [1.5, 0.0, 1.0]]).with_holes(&[([0.5, 0.5], [1.0, 1.5])]), -90.0),
    vec![
        Step::cut(Leaf::turned(Plane::xy(-1.0), Axis::first(2.5).leaning(1e-7), Section::bands(-1.0, &[[2.0, -1.0, 3.0]]), 360.0)),
        Step::add(Leaf::prism(Plane::xy(4.0), Outline::circle([5.0, 3.0], 2.5), 2.0)),
    ],
)"
    );
    for seed in 0..200 {
        for case in [
            Case::drawn_turned(seed),
            Case::drawn_turned_off_the_lattice(seed),
        ] {
            let printed = case.to_string();
            assert!(
                !printed.contains("NaN") && !printed.contains("inf"),
                "seed {seed} prints a number Rust cannot read back: {printed}"
            );
        }
    }
}

#[test]
fn a_turned_leaf_shrinks_into_one_band_on_the_sketch_s_axis_turned_whole_and_into_round_numbers() {
    let leaf = Leaf::turned(
        Plane::yz(2.3),
        Axis::second(1.2).backwards().leaning(1e-7),
        Section::bands(0.3, &[[2.2, 0.7, 3.4], [1.0, 0.7, 2.0]])
            .with_holes(&[([0.6, 1.0], [1.8, 2.0])]),
        -105.0,
    );
    let shrunk: Vec<Leaf> = Case::new(leaf.clone(), vec![])
        .smaller()
        .into_iter()
        .map(|case| case.start)
        .collect();
    let with = |change: &dyn Fn(&mut Leaf)| {
        let mut changed = leaf.clone();
        change(&mut changed);
        changed
    };
    let expected = [
        with(&|leaf| {
            if let Leaf::Turned { plane, .. } = leaf {
                *plane = Plane::xy(2.3);
            }
        }),
        with(&|leaf| {
            if let Leaf::Turned { axis, .. } = leaf {
                axis.across = 0.0;
            }
        }),
        with(&|leaf| {
            if let Leaf::Turned { axis, .. } = leaf {
                axis.backwards = false;
            }
        }),
        with(&|leaf| {
            if let Leaf::Turned { axis, .. } = leaf {
                axis.lean = 0.0;
            }
        }),
        with(&|leaf| {
            if let Leaf::Turned { degrees, .. } = leaf {
                *degrees = 360.0;
            }
        }),
        with(&|leaf| {
            if let Leaf::Turned { degrees, .. } = leaf {
                *degrees = -90.0;
            }
        }),
        with(&|leaf| {
            if let Leaf::Turned { degrees, .. } = leaf {
                *degrees = 105.0;
            }
        }),
        with(&|leaf| {
            if let Leaf::Turned { section, .. } = leaf {
                section.bands.remove(1);
            }
        }),
        with(&|leaf| {
            if let Leaf::Turned { section, .. } = leaf {
                section.holes.clear();
            }
        }),
        with(&|leaf| {
            if let Leaf::Turned { section, .. } = leaf {
                for band in &mut section.bands {
                    band[1] = 0.0;
                }
            }
        }),
        with(&|leaf| {
            if let Leaf::Turned { section, .. } = leaf {
                *section = Section::bands(0.3, &[[3.2, 0.7, 3.4]]);
            }
        }),
        with(&|leaf| {
            if let Leaf::Turned { section, .. } = leaf {
                *section = Section::bands(0.0, &[[2.0, 1.0, 3.0], [1.0, 1.0, 2.0]])
                    .with_holes(&[([1.0, 1.0], [2.0, 2.0])]);
            }
        }),
    ];
    for expected in expected {
        assert!(shrunk.contains(&expected), "{expected} among {shrunk:#?}");
    }
}

#[test]
fn shrinking_any_drawn_turned_case_comes_to_an_end() {
    for seed in 0..200 {
        let mut rounds = 0;
        let shrunk = shrink(
            Case::drawn_turned_off_the_lattice(seed),
            Case::smaller,
            |_| true,
            || {
                rounds += 1;
                rounds < 100_000
            },
        );
        assert!(rounds < 100_000, "seed {seed} was still shrinking");
        assert!(
            shrunk.smaller().is_empty(),
            "seed {seed} stopped short: {shrunk}"
        );
    }
}

#[test]
fn every_turned_leaf_raised_by_the_exact_kernel_alone_keeps_every_rule_and_encloses_pappus_s_volume()
 {
    let leaves = drawn_turns(200);
    assert!(leaves.len() > 60, "{} leaves", leaves.len());
    raised_alone_keep_every_rule_and_enclose_pappus_s_volume(&leaves);
}

/// Whether every one of `leaves`, raised alone by the exact kernel, keeps
/// every rule held to the arithmetic and encloses the volume the arithmetic
/// promised, and the volume of its section as the kernel laid it; a leaf
/// with a wall a hair thin may be declined.
fn raised_alone_keep_every_rule_and_enclose_pappus_s_volume(leaves: &[Leaf]) {
    let weighed = random_solids::on_every_core(leaves, |leaf| {
        let alone = Case::new(leaf.clone(), vec![]);
        let held = random_solids::held_to_arithmetic(&alone, &Exact);
        (held, Exact.raised(leaf).map(|body| body.volume()))
    });
    for (leaf, (held, volume)) in leaves.iter().zip(weighed) {
        assert!(held.is_ok(), "{leaf}: {held:?}");
        let turn = turned(leaf).expect("a turn");
        let volume = match volume {
            Err(Declined::Profile) if turn.has_a_wall_a_hair_thin() => continue,
            volume => volume.expect("a turned body"),
        };
        let promised = turn.volume();
        assert!(
            (volume - promised).abs() <= 1e-9 * promised + laying_room(&turn),
            "{leaf}: {volume} against {promised}"
        );
        if let [side] = turn.drawn().as_slice() {
            let (outline, holes) = side.contours();
            let Some(straight) = Straight::of(&outline, &holes, turn.frame(), &turn.turn(0.0), 0.0)
            else {
                assert!(
                    turn.section.bows(),
                    "{leaf}: a section of straight runs not laid"
                );
                continue;
            };
            let laid = turn.angle() * moment_laid(&straight);
            assert!(
                (volume - laid).abs() <= 1e-9 * laid,
                "{leaf}: {volume} against {laid} as laid"
            );
        }
    }
}

/// What a section laid square to its axis sweeps per radian: the moment of
/// its area about the axis, its holes taken out.
fn moment_laid(straight: &Straight) -> f64 {
    straight
        .contours
        .iter()
        .map(|contour| {
            let corners: Vec<DVec2> = contour.iter().map(|corner| corner.at).collect();
            corners
                .iter()
                .zip(corners.iter().cycle().skip(1))
                .map(|(from, to)| from.perp_dot(*to) * (from.y + to.y))
                .sum::<f64>()
                / 6.0
        })
        .sum::<f64>()
        .abs()
}

/// How far laying a section square to its axis may move the volume of its
/// turn: no corner moves by more than twice the tolerance the exact kernel
/// reads it at, which sweeps at most a strip that wide along every edge, an
/// arc's measured along the arc, at the furthest radius, an arc's bulge
/// counted.
fn laying_room(turn: &Turned) -> f64 {
    let frame = turn.frame();
    let section = &turn.section;
    let bulges = (0..section.bands.len())
        .flat_map(|index| [0, 1].map(|edge| (index, edge)))
        .filter_map(|(index, edge)| {
            let bow = section.bow(index, edge)?;
            Some(bow.extreme(section.chord(index, edge)).y.abs())
        });
    let furthest = section
        .bands
        .iter()
        .flat_map(|band| [band[1].abs(), band[2].abs()])
        .chain(section.sloping_to.iter().flatten().map(|edge| edge.abs()))
        .chain(bulges)
        .fold(0.0, f64::max);
    let placed = turn
        .drawn()
        .iter()
        .flat_map(|side| side.outline.clone())
        .map(|corner| frame.at(corner).abs().max_element())
        .fold(0.0, f64::max);
    let tolerance = Scale::HAIR * Scale::of(placed + furthest).eps();
    let edges: f64 = section
        .trapezoids()
        .iter()
        .map(|trapezoid| {
            let along = trapezoid.along;
            let length = along[1] - along[0];
            let [low, high] = [trapezoid.low, trapezoid.high];
            let edge = |edge: usize, chord: [f64; 2]| match trapezoid.bows[edge] {
                Some(bow) => {
                    let corners = [
                        DVec2::new(along[0], chord[0]),
                        DVec2::new(along[1], chord[1]),
                    ];
                    bow.turn(corners).abs() * bow.radius.abs()
                }
                None => length.hypot(chord[1] - chord[0]),
            };
            (high[0] - low[0]) + (high[1] - low[1]) + edge(0, low) + edge(1, high)
        })
        .sum();
    turn.angle() * furthest * 2.0 * tolerance * edges
}

/// A run of a lone turn as the kernel lays it: its number, its two laid
/// corners, along the axis and away from it, and the way out of the matter
/// across it.
struct Laid {
    run: u32,
    from: DVec2,
    to: DVec2,
    out: DVec2,
}

/// The numbers of the runs of a one-sided turn the kernel lays to some
/// length, and those of them off the axis as it lays them: read off the
/// reading it is handed, at the tolerances the application's body reads an
/// empty part's tools at; nothing when the reading lays no section. The way
/// out across a run is told by the way the outline turns, its holes turning
/// the other way, since a point probed beside a run may land past a wall a
/// hair thin.
fn laid_runs(turn: &Turned) -> Option<(Vec<u32>, Vec<Laid>)> {
    let [side] = <[Drawn; 1]>::try_from(turn.drawn()).ok().expect("one side");
    let (outline, holes) = side.contours();
    let straight = Straight::of(&outline, &holes, turn.frame(), &turn.turn(0.0), 1.0)?;
    let outline: Vec<DVec2> = straight.contours[0]
        .iter()
        .map(|corner| corner.at)
        .collect();
    let turning = outline
        .iter()
        .zip(outline.iter().cycle().skip(1))
        .map(|(from, to)| from.perp_dot(*to))
        .sum::<f64>()
        .signum();
    let mut runs = Vec::new();
    let mut numbers = Vec::new();
    for contour in &straight.contours {
        for (index, corner) in contour.iter().enumerate() {
            numbers.push(corner.run);
            let (from, to) = (corner.at, contour[(index + 1) % contour.len()].at);
            if from.y == 0.0 && to.y == 0.0 {
                continue;
            }
            let along = (to - from).normalize();
            let out = DVec2::new(along.y, -along.x) * turning;
            runs.push(Laid {
                run: corner.run,
                from,
                to,
                out,
            });
        }
    }
    Some((numbers, runs))
}

#[test]
fn a_turned_leaf_names_its_faces_as_the_flats_name_them() {
    let leaves: Vec<Leaf> = drawn_turns(300)
        .into_iter()
        .filter(|leaf| turned(leaf).is_some_and(|turn| turn.section.side().is_some()))
        .collect();
    assert!(leaves.len() > 100, "{} leaves", leaves.len());
    let exact = name_their_faces_as_the_flats_name_them(&leaves);
    assert_eq!(
        exact,
        leaves.len(),
        "every leaf is turned by the exact kernel"
    );
}

/// Whether every one of `leaves`, turned on one side of its axis, is
/// numbered by the exact kernel as the flats number it, and each run it
/// lays turns into the face of its own number: or of a run on its line, or
/// of one the kernel tells apart past the flats' numbers; and how many of
/// them the exact kernel turned, the others held to the flats by the flats.
fn name_their_faces_as_the_flats_name_them(leaves: &[Leaf]) -> usize {
    let mut turned_exactly = 0;
    for leaf in leaves {
        let turn = turned(leaf).expect("a turn");
        let exact = Application.raised(leaf).expect("a turned body");
        turned_exactly += usize::from(exact.is_exact());
        let flats = Flats::for_case(&Case::new(leaf.clone(), vec![]))
            .raised(leaf)
            .expect("a turn the flats make");
        if turn.has_a_wall_a_hair_thin() && exact.is_exact() {
            assert!(exact.faces_end() >= flats.faces_end(), "{leaf}");
        } else {
            assert_eq!(exact.faces_end(), flats.faces_end(), "{leaf}");
        }
        let Some((numbers, runs)) = laid_runs(&turn) else {
            assert!(
                turn.has_a_wall_a_hair_thin() || turn.section.bows(),
                "{leaf}: a section of straight runs not laid"
            );
            continue;
        };
        let laid = |face: usize| numbers.contains(&(face as u32)) || face as u32 >= runs_of(&turn);
        for face in (0..flats.faces_end()).filter(|face| laid(*face)) {
            assert_eq!(
                exact.has_face(face),
                flats.has_face(face),
                "{leaf}: face {face}"
            );
        }
        let (_, reach) = lines_over(leaf);
        let swept = turn.swept(turn.section.side().expect("one side"));
        let (sine, cosine) = (turn.angle() / 2.0).sin_cos();
        let radial = swept.out * cosine + swept.onward * sine;
        let at = |place: DVec2| swept.origin + swept.along * place.x + radial * place.y;
        let drawn_to = TESSELLATION * reach;
        for run in runs
            .iter()
            .filter(|run| run.from.distance(run.to) > drawn_to)
        {
            let middle = (run.from + run.to) / 2.0;
            let way = (at(middle + run.out) - at(middle)).normalize();
            let hit = exact
                .ray_hit(at(middle) + way * drawn_to, -way)
                .unwrap_or_else(|| panic!("{leaf}: no face where run {} turns", run.run));
            let along = (run.to - run.from).normalize();
            let on_its_line = runs.iter().any(|other| {
                other.run as usize == hit.face
                    && [other.from, other.to]
                        .iter()
                        .all(|end| along.perp_dot(*end - run.from).abs() <= 1e-12 * reach)
            });
            let told_apart = hit.face >= flats.faces_end()
                && runs.iter().filter(|other| other.run == run.run).count() > 1;
            assert!(
                hit.face == run.run as usize || on_its_line || told_apart,
                "{leaf}: run {} turned into face {}",
                run.run,
                hit.face
            );
        }
    }
    turned_exactly
}

/// How many runs a section's outline and holes have.
fn runs_of(turn: &Turned) -> u32 {
    let (outline, holes) = turn.section.corners();
    (outline.len() + holes.iter().map(Vec::len).sum::<usize>()) as u32
}

#[test]
fn turned_cases_the_exact_kernel_keeps_are_kept_through_the_application_s_body_and_stay_exact() {
    let held = kept_through_the_application_s_body_and_exact(Case::drawn_turned, 0..11);
    assert!(held > 5, "{held} cases held");
}

/// How many of the cases `draw` gives `seeds` the exact kernel keeps with
/// no wall a hair thin, each of them asserted kept through the
/// application's body too, and on the exact kernel at every step there.
fn kept_through_the_application_s_body_and_exact(
    draw: fn(u64) -> Case,
    seeds: std::ops::Range<u64>,
) -> usize {
    let seeds: Vec<u64> = seeds.collect();
    let weighed = random_solids::on_every_core(&seeds, |seed| {
        let case = draw(*seed);
        random_solids::held_to_arithmetic(&case, &Exact)
            .ok()
            .filter(|kept| kept.thin == 0)?;
        let mut exact = true;
        if let Ok(leaves) = case
            .leaves()
            .map(|leaf| Application.raised(leaf))
            .collect::<Result<Vec<_>, _>>()
        {
            exact = leaves.iter().all(cao_solid::Body::is_exact);
            let mut body = leaves[0].clone();
            for (step, tool) in case.steps.iter().zip(&leaves[1..]) {
                match Application.combined(&body, tool, step.mode) {
                    Ok(next) => body = next,
                    Err(_) => break,
                }
                exact &= body.is_exact();
            }
        }
        Some((
            random_solids::held_to_arithmetic(&case, &Application),
            exact,
            case,
        ))
    });
    let mut held = 0;
    for (seed, (measured, exact, case)) in seeds
        .iter()
        .zip(weighed)
        .filter_map(|(seed, one)| Some((seed, one?)))
    {
        held += 1;
        assert!(measured.is_ok(), "seed {seed}: {measured:?}\n{case}");
        assert!(exact, "seed {seed} left the exact kernel:\n{case}");
    }
    held
}

/// A shaft cut by a block whose floor leans a ten-thousandth of a degree off
/// square to it: a step the kernel builds and reads unstably (#552), which
/// the campaigns count beside the rules, and the judge does not excuse.
#[test]
fn a_step_built_at_an_unstable_lean_is_counted_on_both_kernels() {
    let case = Case::new(
        Leaf::prism(Plane::xy(0.0), Outline::circle([0.0, 0.0], 5.0), 30.0),
        vec![Step::cut(Leaf::prism(
            Plane::tilted([0.0, 0.0, 20.0], [1e-4, 0.0, 0.0]),
            Outline::rectangle([-10.0, -10.0], [10.0, 10.0]),
            20.0,
        ))],
    );

    for (kernel, measured) in [
        ("exact", random_solids::held_to_arithmetic(&case, &Exact)),
        (
            "application",
            random_solids::held_to_arithmetic(&case, &Application),
        ),
    ] {
        assert_eq!(
            measured.map(|measured| measured.unstable),
            Ok(1),
            "{kernel}"
        );
    }
}

#[test]
fn a_hole_a_hair_from_its_band_s_edge_opens_onto_it_on_the_exact_kernel() {
    let holed = |top: f64| {
        Leaf::turned(
            Plane::yz(1.0),
            Axis::second(0.0),
            Section::bands(5.0, &[[6.0, -10.0, -4.0]]).with_holes(&[([6.0, top], [10.5, -7.0])]),
            360.0,
        )
    };
    let (thin, thick) = (holed(-9.99999998), holed(-9.5));
    assert!(turned(&thin).expect("a turn").has_a_wall_a_hair_thin());
    assert!(!turned(&thick).expect("a turn").has_a_wall_a_hair_thin());
    let alone = Case::new(thin.clone(), vec![]);
    let measured = random_solids::held_to_arithmetic(&alone, &Exact).expect("held");
    assert_eq!(measured.thin, 0);
    let body = Application.raised(&thin).expect("a turned body");
    assert!(body.is_exact());
    random_solids::holds_through_the_application(&alone);
    let measured = random_solids::held_to_arithmetic(&Case::new(thick, vec![]), &Exact);
    assert_eq!(measured.map(|measured| measured.thin), Ok(0));
}

#[test]
fn random_turned_cases_keep_every_rule_on_the_exact_kernel() {
    keep_every_rule_on_the_exact_kernel(Case::drawn_turned, 0..40);
}

/// Whether every case `draw` gives `seeds` keeps every rule on the exact
/// kernel, held to the arithmetic.
fn keep_every_rule_on_the_exact_kernel(draw: fn(u64) -> Case, seeds: std::ops::Range<u64>) {
    let seeds: Vec<u64> = seeds.collect();
    let weighed = random_solids::on_every_core(&seeds, |seed| {
        let case = draw(*seed);
        (random_solids::held_to_arithmetic(&case, &Exact), case)
    });
    for (seed, (measured, case)) in seeds.iter().zip(weighed) {
        assert!(measured.is_ok(), "seed {seed}: {measured:?}\n{case}");
    }
}

/// The turned leaves whose section slopes, of the first cases of the
/// slanted draw.
fn drawn_slants(seeds: u64) -> Vec<Leaf> {
    (0..seeds)
        .flat_map(|seed| {
            Case::drawn_slanted(seed)
                .leaves()
                .cloned()
                .collect::<Vec<_>>()
        })
        .filter(|leaf| turned(leaf).is_some_and(|turn| turn.section.slopes()))
        .collect()
}

#[test]
fn every_sloped_leaf_raised_by_the_exact_kernel_alone_keeps_every_rule_and_encloses_pappus_s_volume()
 {
    let leaves = drawn_slants(80);
    assert!(leaves.len() > 30, "{} leaves", leaves.len());
    raised_alone_keep_every_rule_and_enclose_pappus_s_volume(&leaves);
}

#[test]
fn a_sloped_leaf_names_its_faces_as_the_flats_name_them() {
    let leaves: Vec<Leaf> = drawn_slants(120)
        .into_iter()
        .filter(|leaf| turned(leaf).is_some_and(|turn| turn.section.side().is_some()))
        .collect();
    assert!(leaves.len() > 40, "{} leaves", leaves.len());
    let exact = name_their_faces_as_the_flats_name_them(&leaves);
    assert_eq!(
        exact,
        leaves.len(),
        "every leaf is turned by the exact kernel"
    );
}

#[test]
fn sloped_cases_the_exact_kernel_keeps_are_kept_through_the_application_s_body_and_stay_exact() {
    let held = kept_through_the_application_s_body_and_exact(Case::drawn_slanted, 0..8);
    assert!(held > 3, "{held} cases held");
}

#[test]
fn random_slanted_cases_keep_every_rule_on_the_exact_kernel() {
    keep_every_rule_on_the_exact_kernel(Case::drawn_slanted, 0..16);
}

#[test]
fn the_draws_before_cones_still_give_each_seed_its_case() {
    assert_eq!(fingerprint(Case::drawn_turned), 0x9180_1d59_a432_9c6a);
    assert_eq!(
        fingerprint(Case::drawn_turned_off_the_lattice),
        0x13be_c8c0_b781_93e1
    );
}

#[test]
fn the_draws_before_conics_still_give_each_seed_its_case() {
    assert_eq!(fingerprint(Case::drawn_slanted), 0x32b3_b100_d73f_534f);
    assert_eq!(
        fingerprint(Case::drawn_slanted_off_the_lattice),
        0x5ae8_3b2b_d8e7_55f3
    );
}

#[test]
fn the_draws_before_ellipses_still_give_each_seed_its_case() {
    assert_eq!(fingerprint(Case::drawn_tilted), 0x32d0_d851_1237_f618);
    assert_eq!(
        fingerprint(Case::drawn_tilted_off_the_lattice),
        0x1fd0_313a_bd3d_51fe
    );
}

#[test]
fn the_draws_of_ellipses_give_each_seed_its_case() {
    assert_eq!(fingerprint(Case::drawn_ellipses), 0x3472_512f_3462_be46);
    assert_eq!(
        fingerprint(Case::drawn_ellipses_crossed),
        0x2e27_8c01_f49f_a60d
    );
    assert_eq!(
        fingerprint(Case::drawn_ellipses_tilted),
        0x9fad_ec77_db9b_db13
    );
}

#[test]
fn the_same_seed_draws_the_same_slanted_case_and_not_one_of_the_other_draws() {
    for seed in 0..200 {
        assert_eq!(
            Case::drawn_slanted(seed),
            Case::drawn_slanted(seed),
            "seed {seed}"
        );
    }
    let distinct: std::collections::BTreeSet<String> = (0..200)
        .map(|seed| Case::drawn_slanted(seed).to_string())
        .collect();
    assert!(distinct.len() > 190, "{} distinct cases", distinct.len());
    let same = (0..200)
        .filter(|seed| {
            let slanted = Case::drawn_slanted(*seed);
            slanted == Case::drawn_turned(*seed)
                || slanted == Case::drawn_profiles(*seed)
                || slanted == Case::drawn_square(*seed)
        })
        .count();
    assert!(same < 20, "{same} slanted cases are another draw's");
}

#[test]
fn a_slanted_case_holds_profiles_of_straight_runs_turned_about_lines_of_the_planes_of_the_origin_and_prisms()
 {
    let mut sloped = 0;
    for seed in 0..3000 {
        let case = Case::drawn_slanted(seed);
        for leaf in case.leaves() {
            assert!(leaf.is_solid(), "seed {seed}: {case}");
            assert!(
                !matches!(leaf.plane(), Plane::Tilted { .. }),
                "seed {seed}: {case}"
            );
            match leaf {
                Leaf::Prism { outline, .. } => {
                    assert!(
                        !matches!(outline, Outline::Star { .. }),
                        "seed {seed}: {case}"
                    )
                }
                Leaf::Turned { axis, section, .. } => {
                    assert_eq!(axis.lean, 0.0, "seed {seed}: {case}");
                    sloped += usize::from(section.slopes());
                    for side in turned(leaf).expect("a turn").drawn() {
                        for corners in std::iter::once(&side.outline).chain(&side.holes) {
                            assert!(corners.len() >= 3, "seed {seed}: {case}");
                            for (index, corner) in corners.iter().enumerate() {
                                let run = corners[(index + 1) % corners.len()] - *corner;
                                assert!(
                                    run != DVec2::ZERO,
                                    "seed {seed}, a run of no length: {case}"
                                );
                            }
                        }
                    }
                }
                Leaf::Revolution { .. } => panic!("seed {seed} drew a revolution: {case}"),
            }
        }
    }
    assert!(sloped > 1000, "{sloped} sloped leaves");
}

/// A section read on the left of its axis: itself, or turned over when it
/// lies on the right.
fn on_the_left(section: &Section) -> Section {
    if section.side() != Some(-1.0) {
        return section.clone();
    }
    Section {
        from: section.from,
        bands: section
            .bands
            .iter()
            .map(|&[length, low, high]| [length, -high, -low])
            .collect(),
        sloping_to: section
            .sloping_to
            .iter()
            .map(|&[low, high]| [-high, -low])
            .collect(),
        bowed: section
            .bowed
            .iter()
            .map(|&[low, high]| [high, low].map(|[along, away, radius]| [along, -away, -radius]))
            .collect(),
        holes: section
            .holes
            .iter()
            .map(|[low, high]| [DVec2::new(low.x, -high.y), DVec2::new(high.x, -low.y)])
            .collect(),
    }
}

/// The bands of a section with both of their ends: length, low and high
/// where it starts, low and high where it ends.
fn ends_of(section: &Section) -> Vec<[f64; 5]> {
    (0..section.bands.len())
        .map(|index| {
            let ([low, high], [low_end, high_end]) = section.edges(index);
            [section.bands[index][0], low, high, low_end, high_end]
        })
        .collect()
}

/// An edge of a section read about the world line its leaf turns about:
/// where it starts and where it ends, each a level along the line's world
/// axis and a radius.
type Edge = [DVec2; 2];

/// The edges of a turned leaf read about the world line it turns about: the
/// line, as a point of it and its world axis, its slanted edges and its
/// level ones.
struct Meridian {
    line: (DVec3, usize),
    sloped: Vec<Edge>,
    level: Vec<Edge>,
}

fn meridian(leaf: &Leaf) -> Option<Meridian> {
    let Leaf::Turned {
        plane,
        axis,
        section,
        ..
    } = leaf
    else {
        return None;
    };
    let line = world_line(leaf)?;
    let level = |along: f64| plane.to_world(axis.at(DVec2::new(along, 0.0)))[line.1];
    let ends = section.ends();
    let mut read = Meridian {
        line,
        sloped: Vec::new(),
        level: Vec::new(),
    };
    for index in 0..section.bands.len() {
        let (start, end) = section.edges(index);
        for edge in 0..2 {
            let run = [
                DVec2::new(level(ends[index]), start[edge].abs()),
                DVec2::new(level(ends[index + 1]), end[edge].abs()),
            ];
            if start[edge] == end[edge] {
                read.level.push(run);
            } else {
                read.sloped.push(run);
            }
        }
    }
    Some(read)
}

/// How far a place stands from the line through a run, as a share of how
/// far the run reaches.
fn off_the_line(run: [DVec2; 2], place: DVec2) -> f64 {
    let along = run[1] - run[0];
    along.perp_dot(place - run[0]).abs() / along.length() / run[1].abs().max_element().max(1.0)
}

#[test]
fn the_slanted_generator_draws_every_kind_it_keeps() {
    let cases: Vec<Case> = (0..1000).map(Case::drawn_slanted).collect();
    let leaves = || cases.iter().flat_map(Case::leaves);
    let turns = || leaves().filter_map(turned);
    let sloped = || turns().filter(|turn| turn.section.slopes());
    let left = || sloped().map(|turn| on_the_left(&turn.section));
    let seen = |what: &str, found: bool| assert!(found, "no slanted case drew {what}");
    let hair = |section: &Section| 1e-4 * section.reach();

    seen(
        "a chamfer whose slant lands on the wall past the step",
        left().any(|section| {
            let bands = ends_of(&section);
            (1..bands.len().saturating_sub(1)).any(|index| {
                let ([.., before], [_, low, high, low_end, high_end], [_, next_low, next, ..]) =
                    (bands[index - 1], bands[index], bands[index + 1]);
                low == low_end
                    && high != high_end
                    && before == high
                    && next == high_end
                    && next_low == low
                    && before != next
            })
        }),
    );
    seen(
        "a chamfer whose slant stops short of the wall past the step",
        left().any(|section| {
            let bands = ends_of(&section);
            (1..bands.len().saturating_sub(1)).any(|index| {
                let ([.., before], [_, low, high, low_end, high_end], [_, _, next, ..]) =
                    (bands[index - 1], bands[index], bands[index + 1]);
                low == low_end && high != high_end && (before == high) != (next == high_end)
            })
        }),
    );
    seen(
        "a point on the axis",
        left().any(|section| {
            let bands = ends_of(&section);
            let (first, last) = (bands[0], bands[bands.len() - 1]);
            (first[1] == 0.0 && first[2] == 0.0) || (last[3] == 0.0 && last[4] == 0.0)
        }),
    );
    seen(
        "a cone cut a hair short of its point",
        left().any(|section| {
            let bands = ends_of(&section);
            let (first, last) = (bands[0], bands[bands.len() - 1]);
            (first[1] == 0.0 && 0.0 < first[2] && first[2] < hair(&section))
                || (last[3] == 0.0 && 0.0 < last[4] && last[4] < hair(&section))
        }),
    );
    seen(
        "a knife's edge",
        left().any(|section| {
            let bands = ends_of(&section);
            let (first, last) = (bands[0], bands[bands.len() - 1]);
            (first[1] == first[2] && first[1] != 0.0) || (last[3] == last[4] && last[3] != 0.0)
        }),
    );
    seen(
        "a countersink",
        left().any(|section| {
            let bands = ends_of(&section);
            let last = bands[bands.len() - 1];
            last[1] == 0.0 && last[3] == 0.0 && last[4] > last[2]
        }),
    );
    seen(
        "a chamfered bore",
        left().any(|section| {
            let bands = ends_of(&section);
            let (first, last) = (bands[0], bands[bands.len() - 1]);
            (first[1] > first[3] && first[3] > 0.0) || (last[3] > last[1] && last[1] > 0.0)
        }),
    );
    seen(
        "two slants on one line",
        left().any(|section| {
            let bands = ends_of(&section);
            bands.windows(2).any(|pair| {
                let ([length, _, high, _, high_end], [next_length, _, next, _, next_end]) =
                    (pair[0], pair[1]);
                high != high_end
                    && high_end == next
                    && ((high_end - high) / length - (next_end - next) / next_length).abs()
                        <= 1e-9 * (high_end - high).abs() / length
            })
        }),
    );
    seen(
        "a slope a hair from parallel",
        left().any(|section| {
            ends_of(&section)
                .iter()
                .any(|band| band[2] != band[4] && (band[2] - band[4]).abs() < hair(&section))
        }),
    );
    seen(
        "a slope a hair from square",
        left().any(|section| {
            ends_of(&section)
                .iter()
                .any(|band| band[0] < hair(&section) && (band[2] - band[4]).abs() > 1e3 * band[0])
        }),
    );
    seen(
        "a ridge",
        left().any(|section| {
            ends_of(&section).windows(2).any(|pair| {
                let ([_, _, high, _, rim], [_, _, next, _, next_end]) = (pair[0], pair[1]);
                rim == next && high != rim && next_end != rim && (rim > high) == (rim > next_end)
            })
        }),
    );
    seen(
        "a sloped section across its axis",
        sloped().any(|turn| turn.section.side().is_none()),
    );
    seen(
        "a sloped section on the other side of its axis",
        sloped().any(|turn| turn.section.side() == Some(-1.0)),
    );
    seen(
        "a hole in a sloped section",
        sloped().any(|turn| !turn.section.holes.is_empty()),
    );
    seen(
        "a sloped section turned whole",
        sloped().any(|turn| turn.degrees.abs() == 360.0),
    );
    seen(
        "a sloped section turned part of the way",
        sloped().any(|turn| turn.degrees.abs() < 360.0),
    );
    seen(
        "a sloped section turned backwards",
        sloped().any(|turn| turn.degrees < 0.0),
    );
    let sloped_tool = |mode: Mode| {
        cases.iter().any(|case| {
            case.steps.iter().any(|step| {
                step.mode == mode && turned(&step.tool).is_some_and(|turn| turn.section.slopes())
            })
        })
    };
    seen("a sloped tool added", sloped_tool(Mode::Add));
    seen("a sloped tool cut", sloped_tool(Mode::Cut));

    let earlier_and_later = |related: &dyn Fn(&Leaf, &Leaf) -> bool| {
        cases.iter().any(|case| {
            let leaves: Vec<&Leaf> = case.leaves().collect();
            (1..leaves.len()).any(|later| {
                leaves[..later]
                    .iter()
                    .any(|earlier| related(earlier, leaves[later]))
            })
        })
    };
    seen(
        "a countersunk hole into a block",
        earlier_and_later(&|earlier, later| {
            let (
                Leaf::Prism {
                    plane,
                    outline: Outline::Rectangle { .. },
                    ..
                },
                Some(turn),
                Some((_, along)),
            ) = (earlier, turned(later), world_line(later))
            else {
                return false;
            };
            let bands = ends_of(&on_the_left(&turn.section));
            along == plane.normal().abs().max_position()
                && bands.iter().any(|band| band[1] == 0.0 && band[4] > band[2])
                && bands
                    .iter()
                    .any(|band| band[1] == 0.0 && band[2] == band[4])
        }),
    );
    let on_a_circle = |earlier: &Leaf, later: &Leaf, kind: &dyn Fn(f64, [f64; 5]) -> bool| {
        let (Some(radius), Some(turn), Some(one), Some(other)) = (
            radius_of(earlier),
            turned(later),
            world_line(earlier),
            world_line(later),
        ) else {
            return false;
        };
        same_line(one, other)
            && ends_of(&on_the_left(&turn.section))
                .iter()
                .any(|band| kind(radius, *band))
    };
    seen(
        "a chamfer cut round a raised circle",
        earlier_and_later(&|earlier, later| {
            on_a_circle(earlier, later, &|radius, band| {
                band[1] == radius && band[3] < radius
            })
        }),
    );
    seen(
        "a cone standing on a raised circle",
        earlier_and_later(&|earlier, later| {
            on_a_circle(earlier, later, &|radius, band| {
                band[1] == 0.0 && band[2] == radius && band[4] < radius
            })
        }),
    );
    let after_a_slant = |related: &dyn Fn(&[Edge], &Meridian) -> bool| {
        earlier_and_later(&|earlier, later| {
            let (Some(earlier), Some(later)) = (meridian(earlier), meridian(later)) else {
                return false;
            };
            same_line(earlier.line, later.line) && related(&earlier.sloped, &later)
        })
    };
    seen(
        "the same cone again",
        after_a_slant(&|slants, later| {
            slants.iter().any(|slant| {
                later
                    .sloped
                    .iter()
                    .any(|other| other.iter().all(|end| off_the_line(*slant, *end) <= 1e-12))
            })
        }),
    );
    seen(
        "a cylinder at a cone's rim",
        after_a_slant(&|slants, later| {
            slants.iter().flatten().any(|rim| {
                rim.y > 0.0
                    && later
                        .level
                        .iter()
                        .any(|edge| edge[0].y == rim.y && edge.iter().any(|end| end.x == rim.x))
            })
        }),
    );
    seen(
        "a cone crossing a cone at its rim",
        after_a_slant(&|slants, later| {
            slants.iter().any(|slant| {
                later.sloped.iter().any(|other| {
                    slant.iter().any(|rim| off_the_line(*other, *rim) <= 1e-12)
                        && other.iter().any(|end| off_the_line(*slant, *end) > 1e-3)
                })
            })
        }),
    );
    seen(
        "a slant a hair off a slant",
        after_a_slant(&|slants, later| {
            slants.iter().any(|slant| {
                later.sloped.iter().any(|other| {
                    other.iter().all(|end| {
                        let off = off_the_line(*slant, *end);
                        0.0 < off && off < 1e-4
                    })
                })
            })
        }),
    );
    seen(
        "a circle on a cone's rim",
        earlier_and_later(&|earlier, later| {
            let (Some(earlier), Some(other), Leaf::Prism { plane, outline, .. }) =
                (meridian(earlier), world_line(later), later)
            else {
                return false;
            };
            let radii = match outline {
                Outline::Circle { radius, .. } => vec![*radius],
                Outline::Ring { outer, inner, .. } => vec![*outer, *inner],
                _ => return false,
            };
            let level = plane.to_world(DVec2::ZERO)[earlier.line.1];
            same_line(earlier.line, other)
                && earlier
                    .sloped
                    .iter()
                    .flatten()
                    .any(|rim| rim.x == level && radii.contains(&rim.y))
        }),
    );

    let share = sloped().count() as f64 / leaves().count() as f64;
    assert!(
        (0.2..0.5).contains(&share),
        "slanted leaves are {} of {}",
        sloped().count(),
        leaves().count()
    );
}

#[test]
fn a_sloped_section_runs_its_corners_along_its_slopes_and_closes_at_a_point() {
    let point = Section::bands(0.0, &[[3.0, 0.0, 2.0], [2.0, 0.0, 2.0]])
        .sloping_to(&[[0.0, 2.0], [0.0, 0.0]]);
    assert!(point.is_solid());
    let (outline, holes) = point.corners();
    assert_eq!(
        outline,
        [[0.0, 0.0], [3.0, 0.0], [5.0, 0.0], [3.0, 2.0], [0.0, 2.0]].map(DVec2::from)
    );
    assert!(holes.is_empty());

    let knife = Section::bands(0.0, &[[1.0, 2.0, 2.0], [3.0, 1.0, 2.0]])
        .sloping_to(&[[1.0, 2.0], [1.0, 2.0]]);
    assert!(knife.is_solid());
    assert_eq!(
        knife.corners().0,
        [[0.0, 2.0], [1.0, 1.0], [4.0, 1.0], [4.0, 2.0], [1.0, 2.0]].map(DVec2::from)
    );

    let level = Section::bands(1.0, &[[2.0, 1.0, 3.0]]);
    assert_eq!(level.clone().sloping_to(&[[1.0, 3.0]]), level);
    assert!(!level.slopes());
    assert!(point.slopes());

    let holed = Section::bands(0.0, &[[6.0, 1.0, 5.0]])
        .sloping_to(&[[1.0, 8.0]])
        .with_holes(&[([1.0, 2.0], [3.0, 5.4])]);
    assert!(holed.is_solid());
    assert_eq!(
        holed.trapezoids(),
        vec![
            Trapezoid {
                along: [0.0, 1.0],
                low: [1.0, 1.0],
                high: [5.0, 5.5],
                bows: [None; 2],
            },
            Trapezoid {
                along: [3.0, 6.0],
                low: [1.0, 1.0],
                high: [6.5, 8.0],
                bows: [None; 2],
            },
            Trapezoid {
                along: [1.0, 3.0],
                low: [1.0, 1.0],
                high: [2.0, 2.0],
                bows: [None; 2],
            },
            Trapezoid {
                along: [1.0, 3.0],
                low: [5.4, 5.4],
                high: [5.5, 6.5],
                bows: [None; 2],
            },
        ]
    );
    let through = Section::bands(0.0, &[[6.0, 1.0, 5.0]])
        .sloping_to(&[[1.0, 8.0]])
        .with_holes(&[([1.0, 2.0], [3.0, 6.0])]);
    assert!(!through.is_solid(), "a hole past the slope where it starts");

    let pieces = point.pieces();
    assert_eq!(
        pieces[1],
        Piece {
            along: [3.0, 5.0],
            away: [0.0, 2.0],
            ending: [0.0, 0.0],
            side: 1.0,
            bows: [None; 2],
        }
    );
}

#[test]
fn a_sloped_section_across_its_axis_is_handed_over_as_its_two_sides() {
    let section = Section::bands(0.0, &[[2.0, -4.0, 5.0], [1.0, -1.0, 3.0]])
        .sloping_to(&[[-2.0, 5.0], [-1.0, 1.0]])
        .with_holes(&[([0.5, 1.0], [1.5, 2.0])]);
    assert_eq!(section.side(), None);
    let sides = section.sides();
    assert_eq!(
        sides,
        vec![
            Section::bands(0.0, &[[2.0, 0.0, 5.0], [1.0, 0.0, 3.0]])
                .sloping_to(&[[0.0, 5.0], [0.0, 1.0]])
                .with_holes(&[([0.5, 1.0], [1.5, 2.0])]),
            Section::bands(0.0, &[[2.0, -4.0, 0.0], [1.0, -1.0, 0.0]])
                .sloping_to(&[[-2.0, 0.0], [-1.0, 0.0]]),
        ]
    );
    let measure = |pieces: Vec<Piece>, side: f64| {
        pieces
            .iter()
            .filter(|piece| piece.side == side)
            .map(|piece| {
                let length = piece.along[1] - piece.along[0];
                let width = (piece.away[1] - piece.away[0]) + (piece.ending[1] - piece.ending[0]);
                DVec2::new(length * width / 2.0, piece.swept())
            })
            .sum::<DVec2>()
    };
    for side in [1.0, -1.0] {
        let halves = sides
            .iter()
            .map(|half| measure(half.pieces(), side))
            .sum::<DVec2>();
        let whole = measure(section.pieces(), side);
        assert!(
            (halves - whole).abs().max_element() <= 1e-12 * whole.max_element(),
            "side {side}: {halves} against {whole}"
        );
    }

    let across: Vec<Turned> = drawn_slants(1000)
        .iter()
        .filter_map(turned)
        .filter(|turn| turn.section.side().is_none())
        .collect();
    assert!(across.len() > 10, "{} sloped sections across", across.len());
    for turn in across {
        let drawn = turn.drawn();
        assert_eq!(drawn.len(), 2, "{:?}", turn.section);
        let line = turn.turn(0.0);
        for (side, expected) in drawn.iter().zip([1.0, -1.0]) {
            assert_eq!(
                line.lie(&side.profile(true)),
                Lie::Side(expected),
                "{:?}",
                turn.section
            );
        }
        for side in turn.section.sides() {
            assert!(side.is_solid(), "{side:?} of {:?}", turn.section);
        }
    }
}

#[test]
fn a_sloped_section_pinched_on_its_axis_is_not_solid() {
    let leaving = Section::bands(0.0, &[[3.0, 0.0, 4.0]]).sloping_to(&[[2.0, 4.0]]);
    assert!(!leaving.is_solid(), "a corner on the axis two slants leave");
    let valley = Section::bands(0.0, &[[2.0, 2.0, 4.0], [2.0, 0.0, 4.0]])
        .sloping_to(&[[0.0, 4.0], [2.0, 4.0]]);
    assert!(
        !valley.is_solid(),
        "a bore narrowing to the axis and out again"
    );
    let point = Section::bands(0.0, &[[3.0, 0.0, 4.0]]).sloping_to(&[[0.0, 0.0]]);
    assert!(point.is_solid(), "a point whose low run lies on the axis");
    let knife = Section::bands(0.0, &[[2.0, 0.0, 4.0], [2.0, 0.0, 4.0]])
        .sloping_to(&[[0.0, 0.0], [0.0, 4.0]]);
    assert!(!knife.is_solid(), "two bands meeting at a point");
    let crossing = Section::bands(0.0, &[[2.0, -2.0, 3.0]]).sloping_to(&[[1.0, 3.0]]);
    assert!(!crossing.is_solid(), "a slant crossing the axis");
    let across = Section::bands(0.0, &[[2.0, -2.0, 3.0]]).sloping_to(&[[-1.0, 3.0]]);
    assert!(across.is_solid());

    for leaf in drawn_slants(1000) {
        let turn = turned(&leaf).expect("a turn");
        let snap = 1e-5 * turn.section.reach();
        let (outline, _) = turn.section.corners();
        for (index, corner) in outline.iter().enumerate() {
            let before = outline[(index + outline.len() - 1) % outline.len()];
            let after = outline[(index + 1) % outline.len()];
            assert!(
                corner.y.abs() > snap || before.y.abs() <= snap || after.y.abs() <= snap,
                "{leaf}: pinched at {corner}"
            );
        }
    }
}

#[test]
fn the_trapezoids_of_a_sloped_section_turn_into_pappus_s_volume_of_its_corners() {
    let volume = |section: Section, degrees: f64| {
        turned(&Leaf::turned(
            Plane::xz(0.0),
            Axis::second(0.0),
            section,
            degrees,
        ))
        .expect("a turn")
        .volume()
    };
    let point = Section::bands(0.0, &[[10.0, 0.0, 5.0]]).sloping_to(&[[0.0, 0.0]]);
    let chamfered = Section::bands(0.0, &[[28.0, 0.0, 10.0], [2.0, 0.0, 10.0]])
        .sloping_to(&[[0.0, 10.0], [0.0, 8.0]]);
    let partial = Section::bands(0.0, &[[4.0, 2.0, 5.0]]).sloping_to(&[[2.0, 4.0]]);
    let across = Section::bands(0.0, &[[2.0, -4.0, 5.0]]).sloping_to(&[[-2.0, 3.0]]);
    let crossing = Section::bands(0.0, &[[2.0, -4.0, 3.0]]).sloping_to(&[[-2.0, 5.0]]);
    for (what, found, expected) in [
        ("a point", volume(point, 360.0), 250.0 * PI / 3.0),
        (
            "a chamfered shaft",
            volume(chamfered, 360.0),
            8888.0 * PI / 3.0,
        ),
        (
            "a quarter turn",
            volume(partial.clone(), 90.0),
            49.0 * PI / 3.0,
        ),
        (
            "a quarter turn back",
            volume(partial, -90.0),
            49.0 * PI / 3.0,
        ),
        ("across, whole", volume(across, 360.0), 98.0 * PI / 3.0),
        ("radii crossing", volume(crossing, 360.0), 103.25 * PI / 3.0),
    ] {
        assert!(
            (found - expected).abs() <= 1e-12 * expected,
            "{what}: {found} against {expected}"
        );
    }

    let turns: Vec<Turned> = drawn_slants(1000).iter().filter_map(turned).collect();
    assert!(turns.len() > 300, "{} turns", turns.len());
    for turn in turns {
        let sides = moment(&turn);
        let pappus: f64 = sides.iter().map(|side| side * turn.angle()).sum();
        let volume = turn.volume();
        let rounding = 1e-12 * turn.angle() * turn.section.reach().powi(3);
        if sides.len() == 1 || turn.degrees.abs() <= 180.0 {
            assert!(
                (volume - pappus).abs() <= 1e-9 * pappus + rounding,
                "{:?}: {volume} against {pappus}",
                turn.section
            );
        } else {
            let largest = sides.iter().copied().fold(0.0, f64::max) * turn.angle();
            assert!(
                largest <= volume * (1.0 + 1e-9) && volume <= pappus * (1.0 + 1e-9),
                "{:?}: {volume} between {largest} and {pappus}",
                turn.section
            );
        }
    }
}

/// The ends of the stretches a line holds, where it crosses there, how
/// squarely and whether what it crosses curves.
fn crossings_of(stretches: &[Stretch]) -> Vec<(f64, f64, bool)> {
    stretches
        .iter()
        .flat_map(|stretch| [stretch.from, stretch.to])
        .map(|end| (end.at, end.cosine, end.curved))
        .collect()
}

fn close(found: &[(f64, f64, bool)], expected: &[(f64, f64, bool)]) -> bool {
    found.len() == expected.len()
        && found.iter().zip(expected).all(|(one, other)| {
            (one.0 - other.0).abs() < 1e-12 && (one.1 - other.1).abs() < 1e-12 && one.2 == other.2
        })
}

#[test]
fn a_line_through_a_cone_is_told_where_it_crosses_how_squarely_and_that_it_curves() {
    let point = Leaf::turned(
        Plane::xy(0.0),
        Axis::first(0.0),
        Section::bands(0.0, &[[4.0, 0.0, 4.0]]).sloping_to(&[[0.0, 0.0]]),
        360.0,
    );
    let along = |origin: [f64; 3], direction: [f64; 3]| {
        crossings_of(
            &point
                .along(DVec3::from(origin), DVec3::from(direction))
                .expect("a turn"),
        )
    };
    let half = 0.5f64.sqrt();

    let axis = along([-1.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    assert!(
        close(&axis, &[(1.0, 1.0, false), (5.0, 0.0, true)]),
        "{axis:?}"
    );
    let square = along([1.0, 0.0, -5.0], [0.0, 0.0, 1.0]);
    assert!(
        close(&square, &[(2.0, half, true), (8.0, half, true)]),
        "{square:?}"
    );
    let ruling = along([0.0, 2.0, 0.0], [1.0, -1.0, 0.0]);
    assert!(
        close(&ruling, &[(0.0, half, false), (3.0, 1.0, true)]),
        "{ruling:?}"
    );
    let tip = along([6.0, -1.0, 0.0], [-2.0, 1.0, 0.0]);
    assert!(
        close(&tip, &[(1.0, 0.0, true), (3.0, 2.0 / 5f64.sqrt(), false)]),
        "{tip:?}"
    );
    let past = along([6.0, 1.0, 0.0], [-1.0, 0.0, 0.0]);
    assert!(
        close(&past, &[(3.0, half, true), (6.0, 1.0, false)]),
        "{past:?}"
    );
    assert!(along([5.0, 0.0, -5.0], [0.0, 0.0, 1.0]).is_empty());

    let bored = Leaf::turned(
        Plane::xy(0.0),
        Axis::first(0.0),
        Section::bands(0.0, &[[2.0, 1.0, 4.0]]).sloping_to(&[[3.0, 4.0]]),
        360.0,
    );
    let through = crossings_of(
        &bored
            .along(DVec3::new(1.0, 0.0, -5.0), DVec3::Z)
            .expect("a turn"),
    );
    assert!(
        close(
            &through,
            &[
                (1.0, 1.0, true),
                (3.0, half, true),
                (7.0, half, true),
                (9.0, 1.0, true)
            ]
        ),
        "{through:?}"
    );
}

/// The pieces of a section, each sloped one cut into `steps` level ones
/// along it: inside its walls when `inside`, around them otherwise.
fn stairs(pieces: &[Piece], steps: usize, inside: bool) -> Vec<Piece> {
    pieces
        .iter()
        .flat_map(|piece| {
            if piece.is_level() {
                return vec![*piece];
            }
            let length = piece.along[1] - piece.along[0];
            (0..steps)
                .map(|step| {
                    let along = [step, step + 1]
                        .map(|end| piece.along[0] + length * end as f64 / steps as f64);
                    let [start, end] = along.map(|at| piece.at(at));
                    let bulge = |edge: usize| {
                        piece.bows[edge]
                            .filter(|bow| along[0] < bow.center.x && bow.center.x < along[1])
                            .map_or(start[edge], |bow| bow.center.y + bow.radius)
                    };
                    let away = if inside {
                        [
                            start[0].max(end[0]).max(bulge(0)),
                            start[1].min(end[1]).min(bulge(1)),
                        ]
                    } else {
                        [
                            start[0].min(end[0]).min(bulge(0)),
                            start[1].max(end[1]).max(bulge(1)),
                        ]
                    };
                    Piece {
                        along,
                        away,
                        ending: away,
                        side: piece.side,
                        bows: [None; 2],
                    }
                })
                .filter(|piece| piece.away[0] < piece.away[1])
                .collect()
        })
        .collect()
}

#[test]
fn a_cone_holds_along_every_line_what_rings_stacked_inside_it_hold_and_no_more_than_rings_stacked_around_it()
 {
    let leaves: Vec<Leaf> = drawn_slants(60).into_iter().take(40).collect();
    assert!(leaves.len() >= 30, "{} leaves", leaves.len());
    for leaf in &leaves {
        let turn = turned(leaf).expect("a turn");
        let pieces = turn.section.pieces();
        let (inside, around) = (stairs(&pieces, 16, true), stairs(&pieces, 16, false));
        let (lines, reach) = lines_over(leaf);
        let room = 1e-9 * reach;
        for index in 0..lines.count() {
            let (origin, direction) = lines.line(index);
            let [held, least, most] = [&pieces, &inside, &around]
                .map(|pieces| spans_of(&turn.along_pieces(pieces, origin, direction, 0.0)));
            assert!(
                least.without(&held).length() <= room && held.without(&most).length() <= room,
                "line {index} across {leaf}: {held:?} not between {least:?} and {most:?}"
            );
        }
    }
}

#[test]
fn a_cone_a_hair_from_a_cylinder_holds_along_every_line_what_the_cylinder_holds_but_for_the_hair() {
    for hair in [1e-7, 1e-9, 3e-6] {
        for degrees in [360.0, 90.0, -270.0] {
            let leaf = |section: Section| {
                Leaf::turned(Plane::xz(1.0), Axis::second(2.0), section, degrees)
            };
            let cylinder = leaf(Section::bands(-1.0, &[[4.0, 1.0, 3.0]]));
            let cone =
                leaf(Section::bands(-1.0, &[[4.0, 1.0, 3.0]]).sloping_to(&[[1.0, 3.0 + hair]]));
            let (lines, reach) = lines_over(&cone);
            for index in 0..lines.count() {
                let (origin, direction) = lines.line(index);
                let spans = |leaf: &Leaf, by: f64| {
                    spans_of(&leaf.along_grown(origin, direction, by).expect("a turn"))
                };
                let (held, inner, outer) = (
                    spans(&cone, 0.0),
                    spans(&cylinder, 0.0),
                    spans(&cylinder, hair),
                );
                assert!(
                    inner.without(&held).length() <= 1e-12 * reach
                        && held.without(&outer).length() <= 1e-12 * reach,
                    "line {index} of {degrees} at a hair of {hair}"
                );
            }
        }
    }
}

#[test]
fn a_sloped_leaf_grown_holds_every_line_it_held_and_shrunk_holds_none_it_did_not() {
    for leaf in drawn_slants(120) {
        let (lines, reach) = lines_over(&leaf);
        let by = 1e-3 * reach;
        for index in 0..lines.count() {
            let (origin, direction) = lines.line(index);
            let [shrunk, held, grown] = [-by, 0.0, by]
                .map(|by| spans_of(&leaf.along_grown(origin, direction, by).expect("a turn")));
            assert_eq!(
                shrunk.without(&held).length(),
                0.0,
                "line {index} of {leaf}"
            );
            assert_eq!(held.without(&grown).length(), 0.0, "line {index} of {leaf}");
            assert!(grown.length() > shrunk.length() || grown.length() == 0.0);
        }
    }
}

#[test]
fn a_sloped_turn_backwards_holds_along_every_line_what_the_turn_forwards_holds_along_the_line_turned_back()
 {
    let leaves: Vec<Leaf> = drawn_slants(200)
        .into_iter()
        .filter(|leaf| turned(leaf).is_some_and(|turn| turn.degrees > 0.0))
        .collect();
    assert!(leaves.len() > 60, "{} leaves", leaves.len());
    for leaf in &leaves {
        let forwards = turned(leaf).expect("a turn");
        let backwards = Turned {
            degrees: -forwards.degrees,
            ..forwards.clone()
        };
        let (lines, reach) = lines_over(leaf);
        for index in 0..lines.count() {
            let line = lines.line(index);
            let (origin, direction) = turned_line(&forwards, forwards.degrees.to_radians(), line);
            let back = spans_of(&backwards.along_grown(line.0, line.1, 0.0));
            let forth = spans_of(&forwards.along_grown(origin, direction, 0.0));
            assert!(
                back.without(&forth).length() + forth.without(&back).length() <= 1e-9 * reach,
                "line {index} across {leaf}: {back:?} against {forth:?}"
            );
        }
    }
}

#[test]
fn a_sloped_leaf_off_its_axis_holds_along_every_line_and_spans_the_box_its_flats_hold_but_for_their_sagitta()
 {
    let leaves = turns_the_flats_hold(drawn_slants(300).into_iter().take(160).collect());
    assert!(leaves.len() > 20, "{} leaves", leaves.len());
    for leaf in &leaves {
        let flats = leaf.solid().expect("a turn the flats make");
        holds_what_its_flats_hold(leaf, &flats);
        spans_the_box_its_flats_span(leaf, &flats);
    }
}

#[test]
fn a_slanted_case_prints_as_the_rust_that_builds_it_again() {
    let pasted = Case::new(
        Leaf::turned(
            Plane::xz(2.0),
            Axis::second(0.0).backwards(),
            Section::bands(0.0, &[[3.0, 0.0, 2.0], [1.5, 0.0, 1.0]])
                .sloping_to(&[[0.0, 2.0], [0.0, 0.0]])
                .with_holes(&[([0.5, 0.5], [1.0, 1.5])]),
            -90.0,
        ),
        vec![Step::cut(Leaf::turned(
            Plane::xy(-1.0),
            Axis::first(2.5),
            Section::bands(-1.0, &[[2.0, -1.0, 3.0]]).sloping_to(&[[-1.5, 2.5]]),
            360.0,
        ))],
    );
    assert_eq!(
        pasted.to_string(),
        "Case::new(
    Leaf::turned(Plane::xz(2.0), Axis::second(0.0).backwards(), Section::bands(0.0, &[[3.0, 0.0, 2.0], [1.5, 0.0, 1.0]]).sloping_to(&[[0.0, 2.0], [0.0, 0.0]]).with_holes(&[([0.5, 0.5], [1.0, 1.5])]), -90.0),
    vec![
        Step::cut(Leaf::turned(Plane::xy(-1.0), Axis::first(2.5), Section::bands(-1.0, &[[2.0, -1.0, 3.0]]).sloping_to(&[[-1.5, 2.5]]), 360.0)),
    ],
)"
    );
    for seed in 0..200 {
        for case in [
            Case::drawn_slanted(seed),
            Case::drawn_slanted_off_the_lattice(seed),
        ] {
            let printed = case.to_string();
            assert!(
                !printed.contains("NaN") && !printed.contains("inf"),
                "seed {seed} prints a number Rust cannot read back: {printed}"
            );
        }
    }
}

#[test]
fn a_sloped_leaf_shrinks_into_level_bands_and_round_numbers() {
    let leaf = |section: Section| Leaf::turned(Plane::xy(0.0), Axis::first(0.0), section, 360.0);
    let sloped = Section::bands(0.3, &[[2.2, 0.0, 3.4], [1.0, 0.0, 2.0]])
        .sloping_to(&[[0.0, 2.7], [0.0, 0.2]]);
    let shrunk: Vec<Leaf> = Case::new(leaf(sloped.clone()), vec![])
        .smaller()
        .into_iter()
        .map(|case| case.start)
        .collect();
    let expected = [
        Section::bands(0.3, &[[2.2, 0.0, 3.4], [1.0, 0.0, 2.0]])
            .sloping_to(&[[0.0, 3.4], [0.0, 0.2]]),
        Section::bands(0.3, &[[2.2, 0.0, 3.4], [1.0, 0.0, 2.0]])
            .sloping_to(&[[0.0, 2.7], [0.0, 2.0]]),
        Section::bands(0.3, &[[2.2, 0.0, 3.4], [1.0, 0.0, 2.0]]),
        Section::bands(0.3, &[[2.2, 0.0, 3.4]]).sloping_to(&[[0.0, 2.7]]),
        Section::bands(0.3, &[[1.0, 0.0, 2.0]]).sloping_to(&[[0.0, 0.2]]),
        Section::bands(0.3, &[[3.2, 0.0, 3.4]]),
        Section::bands(0.0, &[[2.0, 0.0, 3.0], [1.0, 0.0, 2.0]])
            .sloping_to(&[[0.0, 3.0], [0.0, 0.0]]),
        Section::bands(0.5, &[[2.0, 0.0, 3.5], [1.0, 0.0, 2.0]])
            .sloping_to(&[[0.0, 2.5], [0.0, 0.0]]),
    ];
    for expected in expected {
        let expected = leaf(expected);
        assert!(shrunk.contains(&expected), "{expected} among {shrunk:#?}");
    }
    let level = Section::bands(0.0, &[[2.0, 0.0, 3.0]]);
    assert!(
        Case::new(leaf(level), vec![])
            .smaller()
            .iter()
            .all(|case| turned(&case.start).is_none_or(|turn| !turn.section.slopes())),
        "a level section shrinks into a sloped one"
    );
}

#[test]
fn shrinking_any_drawn_slanted_case_comes_to_an_end() {
    for seed in 0..200 {
        let mut rounds = 0;
        let shrunk = shrink(
            Case::drawn_slanted_off_the_lattice(seed),
            Case::smaller,
            |_| true,
            || {
                rounds += 1;
                rounds < 100_000
            },
        );
        assert!(rounds < 100_000, "seed {seed} was still shrinking");
        assert!(
            shrunk.smaller().is_empty(),
            "seed {seed} stopped short: {shrunk}"
        );
    }
}

#[test]
fn the_same_seed_draws_the_same_tilted_case_and_not_one_of_the_other_draws() {
    for seed in 0..200 {
        assert_eq!(
            Case::drawn_tilted(seed),
            Case::drawn_tilted(seed),
            "seed {seed}"
        );
    }
    let distinct: std::collections::BTreeSet<String> = (0..200)
        .map(|seed| Case::drawn_tilted(seed).to_string())
        .collect();
    assert!(distinct.len() > 190, "{} distinct cases", distinct.len());
    let same = (0..200)
        .filter(|seed| {
            let tilted = Case::drawn_tilted(*seed);
            tilted == Case::drawn_slanted(*seed)
                || tilted == Case::drawn_turned(*seed)
                || tilted == Case::drawn_profiles(*seed)
                || tilted == Case::drawn_square(*seed)
        })
        .count();
    assert!(same < 20, "{same} tilted cases are another draw's");
}

#[test]
fn every_seed_draws_a_tilted_case_though_nothing_before_it_has_a_slanted_face() {
    for seed in 0..5000 {
        let drawn = std::panic::catch_unwind(|| {
            [
                Case::drawn_tilted(seed),
                Case::drawn_tilted_off_the_lattice(seed),
            ]
        });
        let Ok(cases) = drawn else {
            panic!("seed {seed} draws no case");
        };
        for case in cases {
            for leaf in case.leaves() {
                assert!(leaf.is_solid(), "seed {seed}: {case}");
            }
        }
    }
}

/// The steps of a case on a plane leaning off the planes of the origin.
fn leaning(case: &Case) -> impl Iterator<Item = &Step> {
    case.steps
        .iter()
        .filter(|step| matches!(step.tool.plane(), Plane::Tilted { .. }))
}

/// The holes cut and the bosses added square to a slanted face a cut before
/// them left: on a plane turned as that cut's was.
fn on_a_slanted_face(case: &Case, mode: Mode) -> bool {
    case.steps.iter().enumerate().any(|(index, step)| {
        let Leaf::Prism {
            plane: Plane::Tilted { turn, .. },
            outline: Outline::Circle { .. } | Outline::Slot { .. } | Outline::Ring { .. },
            ..
        } = step.tool
        else {
            return false;
        };
        step.mode == mode
            && case.steps[..index].iter().any(|before| {
                before.mode == Mode::Cut
                    && matches!(
                        before.tool,
                        Leaf::Prism {
                            plane: Plane::Tilted { turn: cut, .. },
                            outline: Outline::Rectangle { .. },
                            ..
                        } if cut == turn
                    )
            })
    })
}

/// The walls of a case's shafts, bores and cones read about the world lines
/// their leaves turn about, or circles are raised along: a point of the
/// line, its world axis, a radius, and the slope of the wall.
fn round_walls(case: &Case) -> Vec<(DVec3, usize, f64, f64)> {
    let mut walls = Vec::new();
    for leaf in case.leaves() {
        match leaf {
            Leaf::Prism {
                plane,
                outline: Outline::Circle { center, radius, .. },
                ..
            } if !matches!(plane, Plane::Tilted { .. }) => walls.push((
                plane.to_world(*center),
                plane.normal().abs().max_position(),
                *radius,
                0.0,
            )),
            _ => {
                if let Some(read) = meridian(leaf) {
                    let (point, axis) = read.line;
                    for [from, to] in read.level.iter().chain(&read.sloped) {
                        walls.push((point, axis, from.y, (to.y - from.y) / (to.x - from.x)));
                    }
                }
            }
        }
    }
    walls
}

/// Whether a step's plane runs along a shaft's axis at its radius, touching
/// it along a line.
fn along_a_shaft_at_its_radius(case: &Case) -> bool {
    let walls = round_walls(case);
    case.steps.iter().any(|step| {
        let plane = *step.tool.plane();
        let (origin, normal) = (plane.to_world(DVec2::ZERO), plane.normal());
        walls.iter().any(|&(point, axis, radius, slope)| {
            slope == 0.0
                && radius > 0.0
                && normal[axis].abs() <= 1e-12
                && ((origin - point).dot(normal).abs() - radius).abs() <= 1e-6 * radius
        })
    })
}

/// Whether a leaning step of a case stands a hair, in angle, from square to
/// the axis of one of its shafts or cones, from parallel to it, or from a
/// cone's own slope.
fn leaning_hairs(case: &Case) -> [bool; 3] {
    let hair = |angle: f64| (1e-14..1e-4).contains(&angle.abs());
    let walls = round_walls(case);
    let mut found = [false; 3];
    for step in leaning(case) {
        let normal = step.tool.plane().normal();
        for &(_, axis, _, slope) in &walls {
            let along = normal[axis].abs();
            let with_the_axis = along.atan2((1.0 - along * along).max(0.0).sqrt());
            found[0] |= hair(std::f64::consts::FRAC_PI_2 - with_the_axis);
            found[1] |= hair(with_the_axis);
            found[2] |= slope != 0.0 && hair(with_the_axis - slope.abs().atan());
        }
    }
    found
}

#[test]
fn the_tilted_generator_draws_every_kind_it_keeps() {
    let cases: Vec<Case> = (0..1000).map(Case::drawn_tilted).collect();
    let seen = |what: &str, found: bool| assert!(found, "no tilted case drew {what}");
    let share = |count: usize| count as f64 / cases.len() as f64;
    let conics = random_solids::on_every_core(&cases, Case::conics);
    let with = |kind: fn(&random_solids::Conics) -> usize| {
        share(conics.iter().filter(|conics| kind(conics) > 0).count())
    };
    let asking = random_solids::on_every_core(&cases, Case::asks_beyond_the_conics);

    seen(
        "a case started on a leaning block",
        cases
            .iter()
            .any(|case| matches!(case.start.plane(), Plane::Tilted { .. })),
    );
    seen(
        "a slanted cut",
        cases.iter().flat_map(leaning).any(|step| {
            step.mode == Mode::Cut
                && matches!(
                    step.tool,
                    Leaf::Prism {
                        outline: Outline::Rectangle { .. },
                        ..
                    }
                )
        }),
    );
    seen(
        "a leaning block added",
        cases.iter().flat_map(leaning).any(|step| {
            step.mode == Mode::Add
                && matches!(
                    step.tool,
                    Leaf::Prism {
                        outline: Outline::Rectangle { .. },
                        ..
                    }
                )
        }),
    );
    seen(
        "a hole square into a slanted face",
        cases.iter().any(|case| on_a_slanted_face(case, Mode::Cut)),
    );
    seen(
        "a boss square on a slanted face",
        cases.iter().any(|case| on_a_slanted_face(case, Mode::Add)),
    );
    seen(
        "a bore at a slant",
        cases.iter().flat_map(leaning).any(|step| {
            step.mode == Mode::Cut
                && matches!(
                    step.tool,
                    Leaf::Prism {
                        outline: Outline::Circle { .. },
                        ..
                    }
                )
        }),
    );
    seen(
        "a plane along a shaft's axis at its radius",
        cases.iter().any(along_a_shaft_at_its_radius),
    );
    for case in &cases {
        for step in leaning(case) {
            assert!(
                random_solids::clear_of_the_lines(*step.tool.plane()),
                "a plane square to the lines of measure:\n{case}"
            );
        }
    }

    let thresholds: [(&str, Count, f64); 6] = [
        ("ellipses", |conics| conics.ellipses, 0.3),
        ("hyperbolas", |conics| conics.hyperbolas, 0.15),
        ("parabolas", |conics| conics.parabolas, 0.04),
        (
            "lines through an apex",
            |conics| conics.lines_through_an_apex,
            0.003,
        ),
        ("an apex alone", |conics| conics.apex_alone, 0.003),
        ("tangent rulings", |conics| conics.tangent_rulings, 0.01),
    ];
    for (kind, count, least) in thresholds {
        let found = with(count);
        assert!(found >= least, "{found} of the tilted cases cut {kind}");
    }
    let beyond = share(asking.iter().filter(|asks| **asks).count());
    assert!(
        beyond <= 0.25,
        "{beyond} of the tilted cases ask beyond the conics"
    );

    let hairs = (0..1000)
        .map(|seed| leaning_hairs(&Case::drawn_tilted_off_the_lattice(seed)))
        .fold([false; 3], |seen, found| {
            [0, 1, 2].map(|kind| seen[kind] || found[kind])
        });
    seen("a plane a hair from square to an axis", hairs[0]);
    seen("a plane a hair from parallel to an axis", hairs[1]);
    seen("a plane a hair from a cone's slope", hairs[2]);
}

#[test]
fn a_tilted_case_prints_as_the_rust_that_builds_it_again() {
    let pasted = Case::new(
        Leaf::prism(
            Plane::xy(0.0),
            Outline::rectangle([0.0, 0.0], [20.0, 20.0]),
            20.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::tilted([0.0, 0.0, 5.0], [30.0, 0.0, 0.0]),
                Outline::rectangle([-1.0, -3.0], [21.0, 26.0]),
                25.0,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted([10.0, 10.0, 10.773502691896258], [90.0, 2e-8, 0.0]),
                Outline::circle([0.0, 0.0], 2.0),
                -21.0,
            )),
        ],
    );
    assert_eq!(
        pasted.to_string(),
        "Case::new(
    Leaf::prism(Plane::xy(0.0), Outline::rectangle([0.0, 0.0], [20.0, 20.0]), 20.0),
    vec![
        Step::cut(Leaf::prism(Plane::tilted([0.0, 0.0, 5.0], [30.0, 0.0, 0.0]), Outline::rectangle([-1.0, -3.0], [21.0, 26.0]), 25.0)),
        Step::cut(Leaf::prism(Plane::tilted([10.0, 10.0, 10.773502691896258], [90.0, 2e-8, 0.0]), Outline::circle([0.0, 0.0], 2.0), -21.0)),
    ],
)"
    );
    for seed in 0..200 {
        for case in [
            Case::drawn_tilted(seed),
            Case::drawn_tilted_off_the_lattice(seed),
        ] {
            let printed = case.to_string();
            assert!(
                !printed.contains("NaN") && !printed.contains("inf"),
                "seed {seed} prints a number Rust cannot read back: {printed}"
            );
        }
    }
}

#[test]
fn shrinking_any_drawn_tilted_case_comes_to_an_end() {
    for seed in 0..200 {
        let mut rounds = 0;
        let shrunk = shrink(
            Case::drawn_tilted_off_the_lattice(seed),
            Case::smaller,
            |_| true,
            || {
                rounds += 1;
                rounds < 100_000
            },
        );
        assert!(rounds < 100_000, "seed {seed} was still shrinking");
        assert!(
            shrunk.smaller().is_empty(),
            "seed {seed} stopped short: {shrunk}"
        );
    }
}

#[test]
fn a_tilted_plane_shrinks_to_the_lattice_and_to_a_plane_of_the_origin() {
    let block = |plane: Plane| Leaf::prism(plane, Outline::rectangle([0.0, 0.0], [2.0, 2.0]), 1.0);
    let tilted = Plane::tilted([0.3, 1.2, 2.7], [89.99999998, 37.0, 0.0]);
    let shrunk: Vec<Leaf> = Case::new(block(tilted), vec![])
        .smaller()
        .into_iter()
        .map(|case| case.start)
        .collect();
    for expected in [
        Plane::xy(2.7),
        Plane::tilted([0.3, 1.2, 2.7], [0.0, 37.0, 0.0]),
        Plane::tilted([0.3, 1.2, 2.7], [89.99999998, 0.0, 0.0]),
        Plane::tilted([0.3, 1.2, 2.7], [90.0, 37.0, 0.0]),
        Plane::tilted([0.3, 1.2, 2.7], [89.99999998, 30.0, 0.0]),
        Plane::tilted([0.3, 1.2, 2.7], [89.99999998, 45.0, 0.0]),
        Plane::tilted([0.0, 1.0, 3.0], [89.99999998, 37.0, 0.0]),
    ] {
        let expected = block(expected);
        assert!(shrunk.contains(&expected), "{expected} among {shrunk:#?}");
    }
    let lattice = Plane::tilted([0.0, 1.0, 3.0], [45.0, 30.0, 0.0]);
    assert!(
        Case::new(block(lattice), vec![])
            .smaller()
            .iter()
            .all(|case| {
                match case.start.plane() {
                    Plane::Tilted { turn, .. } => {
                        turn.to_array().iter().all(|degrees| degrees % 15.0 == 0.0)
                    }
                    _ => true,
                }
            }),
        "a turn on the lattice shrinks off it"
    );
}

#[test]
fn a_shaft_cut_at_a_slant_and_a_holed_wedge_are_held_by_the_flats_to_the_arithmetic() {
    let tan30 = 30f64.to_radians().tan();
    let face = DVec3::new(10.0, 10.0, 5.0 + 10.0 * tan30);
    let normal = Plane::leaning(0, 30.0, face).normal();
    let cases = [
        (
            Case::new(
                Leaf::prism(Plane::xy(0.0), Outline::circle([0.0, 0.0], 5.0), 30.0),
                vec![Step::cut(Leaf::prism(
                    Plane::leaning(0, 30.0, DVec3::new(0.0, 0.0, 15.0)),
                    Outline::rectangle([-10.0, -15.0], [10.0, 15.0]),
                    20.0,
                ))],
            ),
            375.0 * PI,
        ),
        (
            Case::new(
                Leaf::prism(
                    Plane::xy(0.0),
                    Outline::rectangle([0.0, 0.0], [20.0, 20.0]),
                    20.0,
                ),
                vec![
                    Step::cut(Leaf::prism(
                        Plane::leaning(0, 30.0, DVec3::new(0.0, 0.0, 5.0)),
                        Outline::rectangle([-1.0, -3.0], [21.0, 26.0]),
                        25.0,
                    )),
                    Step::cut(Leaf::prism(
                        Plane::leaning(0, 30.0, face + normal),
                        Outline::circle([0.0, 0.0], 2.0),
                        -21.0,
                    )),
                ],
            ),
            2000.0 + 4000.0 / 3f64.sqrt() - 4.0 * PI * (10.0 / 3f64.sqrt() + 20.0 / 3.0),
        ),
    ];
    for (case, arithmetic) in cases {
        random_solids::holds(&case);
        let flats = Flats::for_case(&case);
        let held = random_solids::held_to_arithmetic(&case, &flats);
        assert!(held.is_ok(), "{held:?}\n{case}");
        let body = case
            .steps
            .iter()
            .fold(case.start.solid().expect("a solid"), |body, step| {
                let tool = step.tool.solid().expect("a solid");
                match step.mode {
                    Mode::Add => body.union(&tool).expect("a union"),
                    Mode::Cut => body.difference(&tool).expect("a difference"),
                }
            });
        let enclosed = cao_solid::soundness::enclosed(&body.triangles());
        assert!(
            (enclosed - arithmetic).abs() <= 1e-2 * arithmetic,
            "{enclosed} against {arithmetic}\n{case}"
        );
    }
}

/// The round leaves of the first cases of the tilted draw: circles and
/// rings raised, and whole turns.
fn round_leaves(count: usize) -> Vec<Leaf> {
    (0..)
        .flat_map(|seed| {
            Case::drawn_tilted(seed)
                .leaves()
                .cloned()
                .collect::<Vec<_>>()
        })
        .filter(|leaf| leaf.kept_by_a_half_space(DVec3::Z, 0.0).is_some())
        .take(count)
        .collect()
}

/// A leaf cut by a prism on a plane leaning through the middle of its box
/// and covering it, and the half-space it keeps: `normal · x ≤ offset`.
fn cut_at_a_slant(leaf: &Leaf, index: usize) -> (Case, DVec3, f64) {
    let (low, high) = leaf.bounds().expect("a leaf with a box");
    let middle = (low + high) / 2.0;
    let holds = index % 3;
    let degrees = [20.0, 35.0, 50.0, 65.0][index / 3 % 4];
    let plane = [degrees, -degrees]
        .into_iter()
        .map(|degrees| Plane::leaning(holds, degrees, middle))
        .find(|plane| random_solids::clear_of_the_lines(*plane))
        .expect("a leaning clear of the lines");
    let normal = plane.normal();
    let reach = (high - low).length() + 1.0;
    let tool = Leaf::prism(
        plane,
        Outline::rectangle([-reach, -reach], [reach, reach]),
        reach,
    );
    (
        Case::new(leaf.clone(), vec![Step::cut(tool)]),
        normal,
        normal.dot(middle),
    )
}

#[test]
fn a_lone_round_leaf_cut_by_a_slanted_half_space_encloses_the_volume_of_its_slices() {
    let leaves = round_leaves(48);
    let cut: Vec<(Case, DVec3, f64)> = leaves
        .iter()
        .enumerate()
        .map(|(index, leaf)| cut_at_a_slant(leaf, index))
        .collect();
    let weighed = random_solids::on_every_core(&cut, |(case, _, _)| {
        let held = random_solids::held_to_arithmetic(case, &Exact);
        let volume = Exact
            .raised(&case.start)
            .and_then(|leaf| {
                let tool = Exact.raised(&case.steps[0].tool)?;
                Exact.combined(&leaf, &tool, Mode::Cut)
            })
            .map(|body| body.volume());
        (held, volume)
    });
    for ((case, normal, offset), (held, volume)) in cut.iter().zip(weighed) {
        let measured = held.unwrap_or_else(|flaw| panic!("{flaw:?}\n{case}"));
        assert_eq!(measured.declined, 0, "declined:\n{case}");
        let volume = volume.unwrap_or_else(|declined| panic!("{declined:?}\n{case}"));
        let sliced = case
            .start
            .kept_by_a_half_space(*normal, *offset)
            .expect("a round leaf");
        let room = turned(&case.start).map_or(0.0, |turn| laying_room(&turn));
        assert!(
            (volume - sliced).abs() <= 1e-9 * sliced + room,
            "{volume} against {sliced} by slices\n{case}"
        );
    }
}

#[test]
fn random_tilted_cases_keep_every_rule_on_the_exact_kernel() {
    keep_every_rule_on_the_exact_kernel(Case::drawn_tilted, 0..16);
}

#[test]
fn tilted_cases_the_exact_kernel_keeps_are_kept_through_the_application_s_body_and_stay_exact() {
    let held = kept_through_the_application_s_body_and_exact(Case::drawn_tilted, 0..8);
    assert!(held > 3, "{held} cases held");
}

/// The cases of the first two thousand seeds of `draw` that `asks` picks
/// out, `count` at most.
fn picked(draw: Draw, asks: fn(&Case) -> bool, count: usize) -> Vec<Case> {
    (0..2000).map(draw).filter(asks).take(count).collect()
}

#[test]
fn a_case_that_asked_for_a_conic_is_built_now() {
    let conic = |case: &Case| case.meets_along_a_conic() && !case.asks_beyond_the_conics();
    let turned = picked(Case::drawn_turned_off_the_lattice, conic, 10);
    let slanted = picked(Case::drawn_slanted_off_the_lattice, conic, 6);
    assert_eq!(turned.len(), 10, "{} turned cases", turned.len());
    assert_eq!(slanted.len(), 6, "{} slanted cases", slanted.len());
    let asking: Vec<Case> = turned.into_iter().chain(slanted).collect();
    let weighed = random_solids::on_every_core(&asking, |case| {
        (
            random_solids::held_to_arithmetic(case, &Exact),
            random_solids::held_to_arithmetic(case, &Application),
        )
    });
    for (case, (exactly, through_the_application)) in asking.iter().zip(weighed) {
        for measured in [exactly, through_the_application] {
            let measured = measured.unwrap_or_else(|flaw| panic!("{flaw:?}\n{case}"));
            assert_eq!(measured.declined, 0, "declined:\n{case}");
        }
    }
}

#[test]
fn a_case_asking_beyond_the_conics_is_declined_as_unsupported_or_held() {
    let turned = picked(
        Case::drawn_turned_off_the_lattice,
        Case::asks_beyond_the_conics,
        12,
    );
    let slanted = picked(
        Case::drawn_slanted_off_the_lattice,
        Case::asks_beyond_the_conics,
        8,
    );
    assert_eq!(turned.len(), 12, "{} turned cases", turned.len());
    assert_eq!(slanted.len(), 8, "{} slanted cases", slanted.len());
    let asking: Vec<Case> = turned.into_iter().chain(slanted).collect();
    let weighed = random_solids::on_every_core(&asking, |case| {
        random_solids::held_to_arithmetic(case, &Exact)
    });
    let mut declined = 0;
    for (case, measured) in asking.iter().zip(weighed) {
        let measured = measured.unwrap_or_else(|flaw| panic!("{flaw:?}\n{case}"));
        declined += measured.declined;
    }
    println!("{declined} of {} declined as unsupported", asking.len());
}

/// A ball of radius two about the origin, turned by `degrees` about the
/// plane's first axis.
fn ball(degrees: f64) -> Leaf {
    Leaf::turned(
        Plane::xy(0.0),
        Axis::first(0.0),
        Section::bands(-2.0, &[[4.0, 0.0, 0.0]]).bowed(&[[[0.0, 0.0, 0.0], [0.0, 0.0, 2.0]]]),
        degrees,
    )
}

/// A ring torus whose tube of radius one runs three from the axis, about
/// the origin, turned by `degrees` about the plane's first axis.
fn ring(degrees: f64) -> Leaf {
    Leaf::turned(
        Plane::xy(0.0),
        Axis::first(0.0),
        Section::bands(-1.0, &[[2.0, 3.0, 3.0]]).bowed(&[[[0.0, 3.0, -1.0], [0.0, 3.0, 1.0]]]),
        degrees,
    )
}

#[test]
fn a_rounded_section_bows_its_edges_along_arcs_and_is_solid_only_where_they_keep_off_the_axis() {
    let straight = [[0.0; 3]; 2];
    let ball =
        Section::bands(-2.0, &[[4.0, 0.0, 0.0]]).bowed(&[[[0.0, 0.0, 0.0], [0.0, 0.0, 2.0]]]);
    assert!(ball.is_solid());
    assert!(ball.bows());
    assert_eq!(ball.corners().0, [[-2.0, 0.0], [2.0, 0.0]].map(DVec2::from));
    let (contour, holes) = ball.contour();
    assert!(holes.is_empty());
    assert_eq!(contour.corners, ball.corners().0);
    assert_eq!(contour.runs[0], Run::Straight);
    let Run::Round { center, turn } = contour.runs[1] else {
        panic!("{:?}", contour.runs);
    };
    assert_eq!(center, DVec2::ZERO);
    assert!((turn - PI).abs() < 1e-12, "{turn}");

    let capped = Section::bands(0.0, &[[3.0, 0.0, 2.0], [2.0, 0.0, 2.0]])
        .sloping_to(&[[0.0, 2.0], [0.0, 0.0]])
        .bowed(&[straight, [[0.0; 3], [3.0, 0.0, 2.0]]]);
    assert!(capped.is_solid());
    let (contour, _) = capped.contour();
    assert_eq!(
        contour.corners,
        [[0.0, 0.0], [3.0, 0.0], [5.0, 0.0], [3.0, 2.0], [0.0, 2.0]].map(DVec2::from)
    );
    let Run::Round { center, turn } = contour.runs[2] else {
        panic!("{:?}", contour.runs);
    };
    assert_eq!(center, DVec2::new(3.0, 0.0));
    assert!((turn - PI / 2.0).abs() < 1e-12, "{turn}");
    assert!(
        contour
            .runs
            .iter()
            .enumerate()
            .all(|(index, run)| index == 2 || *run == Run::Straight)
    );

    let grooved = Section::bands(0.0, &[[2.0, 0.0, 4.0], [2.0, 0.0, 3.0], [2.0, 0.0, 4.0]])
        .bowed(&[straight, [[0.0; 3], [3.0, 3.0, -1.0]], straight]);
    assert!(grooved.is_solid());
    let (contour, _) = grooved.contour();
    assert_eq!(contour.corners.len(), 10);
    assert_eq!(contour.corners[6], DVec2::new(4.0, 3.0));
    let Run::Round { center, turn } = contour.runs[6] else {
        panic!("{:?}", contour.runs);
    };
    assert_eq!(center, DVec2::new(3.0, 3.0));
    assert!((turn + PI).abs() < 1e-12, "{turn}");

    let level = Section::bands(0.0, &[[2.0, 1.0, 3.0]]);
    assert_eq!(level.clone().bowed(&[straight]), level);
    assert!(!level.bows());
    assert_eq!(
        Section::bands(0.0, &[[2.0, 1.0, 3.0]]).bowed(&[[[5.0, 5.0, 0.0], [0.0; 3]]]),
        level
    );

    for (what, section) in [
        (
            "an arc crossing the axis",
            Section::bands(0.0, &[[4.0, 0.0, 1.0]]).bowed(&[[[2.0, 1.0, -2.0], [0.0; 3]]]),
        ),
        (
            "an arc tangent to the axis between its corners",
            Section::bands(0.0, &[[4.0, 2.0, 4.0]]).bowed(&[[[2.0, 2.0, -2.0], [0.0; 3]]]),
        ),
        (
            "an arc tangent to the axis at its corner",
            Section::bands(0.0, &[[2.0, 0.0, 0.0]])
                .sloping_to(&[[0.0, 2.0]])
                .bowed(&[[[0.0; 3], [0.0, 2.0, -2.0]]]),
        ),
        (
            "a corner off its circle",
            Section::bands(0.0, &[[4.0, 0.0, 0.0]]).bowed(&[[[0.0; 3], [1.0, 0.0, 2.0]]]),
        ),
        (
            "an arc said to bow the other way",
            Section::bands(0.0, &[[3.0, 0.0, 2.0]]).bowed(&[[[0.0; 3], [1.5, 0.0, -2.5]]]),
        ),
        (
            "a hole under an arc",
            Section::bands(0.0, &[[6.0, 0.0, 0.0]])
                .bowed(&[[[0.0; 3], [3.0, 0.0, 3.0]]])
                .with_holes(&[([2.0, 0.5], [4.0, 1.0])]),
        ),
        (
            "an arc on a section across its axis",
            Section::bands(0.0, &[[4.0, -1.0, 1.0]]).bowed(&[[[0.0; 3], [2.0, 1.0, 2.0]]]),
        ),
        (
            "fewer bows than bands",
            Section::bands(0.0, &[[2.0, 0.0, 2.0], [2.0, 0.0, 2.0]])
                .bowed(&[[[0.0; 3], [1.0, 2.0, 1.0]]]),
        ),
    ] {
        assert!(!section.is_solid(), "{what}");
    }
    assert!(
        Section::bands(0.0, &[[3.0, 0.0, 2.0]])
            .bowed(&[[[0.0; 3], [1.5, 0.0, 2.5]]])
            .is_solid(),
        "a zone whose corners stand on its circle"
    );
}

/// A shaft of radius three from minus three to three about the plane's
/// first axis, with a spherical cavity of radius one about the origin.
fn hollowed() -> Leaf {
    Leaf::turned(
        Plane::xy(0.0),
        Axis::first(0.0),
        Section::bands(-3.0, &[[2.0, 0.0, 3.0], [2.0, 0.0, 3.0], [2.0, 0.0, 3.0]]).bowed(&[
            [[0.0; 3]; 2],
            [[0.0, 0.0, 1.0], [0.0; 3]],
            [[0.0; 3]; 2],
        ]),
        360.0,
    )
}

/// A shaft of radius four with a round-bottomed groove of radius one cut a
/// unit deep across its middle, about the plane's first axis.
fn grooved() -> Leaf {
    Leaf::turned(
        Plane::xy(0.0),
        Axis::first(0.0),
        Section::bands(0.0, &[[2.0, 0.0, 4.0], [2.0, 0.0, 3.0], [2.0, 0.0, 4.0]]).bowed(&[
            [[0.0; 3]; 2],
            [[0.0; 3], [3.0, 3.0, -1.0]],
            [[0.0; 3]; 2],
        ]),
        360.0,
    )
}

/// The rounded leaves held by hand: a ball, a zone and a cap of one, a
/// shaft with a rounded end, a ring torus, an apple of a spindle torus, a
/// round-bottomed groove, a spherical cavity, a barrel bore and a rounded
/// shoulder, turned whole or part of the way.
fn rounded_by_hand() -> Vec<Leaf> {
    let turned = |section: Section, degrees: f64| {
        Leaf::turned(Plane::xz(1.0), Axis::second(2.0), section, degrees)
    };
    let zone = Section::bands(-1.5, &[[3.0, 0.0, 2.0]]).bowed(&[[[0.0; 3], [0.0, 0.0, 2.5]]]);
    let cap = Section::bands(-2.5, &[[4.0, 0.0, 0.0]])
        .sloping_to(&[[0.0, 2.0]])
        .bowed(&[[[0.0; 3], [0.0, 0.0, 2.5]]]);
    let ended = Section::bands(0.0, &[[3.0, 0.0, 2.0], [2.0, 0.0, 2.0]])
        .sloping_to(&[[0.0, 2.0], [0.0, 0.0]])
        .bowed(&[[[0.0; 3]; 2], [[0.0; 3], [3.0, 0.0, 2.0]]]);
    let apple = Section::bands(-2.0, &[[4.0, 0.0, 0.0]]).bowed(&[[[0.0; 3], [0.0, -1.5, 2.5]]]);
    let barrel = Section::bands(0.0, &[[3.0, 2.0, 4.0]]).bowed(&[[[1.5, 4.0, -2.5], [0.0; 3]]]);
    let shoulder = Section::bands(0.0, &[[2.0, 0.0, 4.0], [1.0, 0.0, 4.0], [2.0, 0.0, 2.0]])
        .sloping_to(&[[0.0, 4.0], [0.0, 3.0], [0.0, 2.0]])
        .bowed(&[[[0.0; 3]; 2], [[0.0; 3], [2.0, 3.0, 1.0]], [[0.0; 3]; 2]]);
    let grooved_tube = Section::bands(0.0, &[[2.0, 1.0, 4.0], [2.0, 1.0, 3.0], [2.0, 1.0, 4.0]])
        .bowed(&[[[0.0; 3]; 2], [[0.0; 3], [3.0, 3.0, -1.0]], [[0.0; 3]; 2]]);
    let mut leaves = vec![
        ball(360.0),
        ball(90.0),
        ball(-210.0),
        ring(360.0),
        ring(-90.0),
        ring(135.0),
        hollowed(),
        grooved(),
    ];
    for (section, degrees) in [
        (zone, 360.0),
        (cap, 180.0),
        (ended, 360.0),
        (ended_backwards(), -90.0),
        (apple, 360.0),
        (barrel, 270.0),
        (shoulder, 360.0),
        (grooved_tube, 225.0),
    ] {
        leaves.push(turned(section, degrees));
    }
    leaves
}

/// A shaft with a rounded end drawn from the other end of its axis.
fn ended_backwards() -> Section {
    Section::bands(0.0, &[[2.0, 0.0, 0.0], [3.0, 0.0, 2.0]])
        .sloping_to(&[[0.0, 2.0], [0.0, 2.0]])
        .bowed(&[[[0.0; 3], [2.0, 0.0, 2.0]], [[0.0; 3]; 2]])
}

/// What a rounded section's corners and arcs sweep per radian, by Pappus,
/// from another road than its pieces: the moment of its corners' polygon,
/// and the moment of each arc's circular segment added or taken away as
/// the arc bulges out of the polygon or into it.
fn moment_with_arcs(turn: &Turned) -> f64 {
    let section = &turn.section;
    let polygon = |corners: &[DVec2]| {
        (0..corners.len())
            .map(|index| {
                let (one, other) = (corners[index], corners[(index + 1) % corners.len()]);
                one.perp_dot(other) * (one.y + other.y) / 6.0
            })
            .sum::<f64>()
    };
    let (outline, holes) = section.corners();
    let mut moment = polygon(&outline) + holes.iter().map(|hole| polygon(hole)).sum::<f64>();
    for index in 0..section.bands.len() {
        for edge in 0..2 {
            if let Some(bow) = section.bow(index, edge) {
                let out = if edge == 1 { 1.0 } else { -1.0 };
                moment += out * bow.bulge() * bow.segment_moment(section.chord(index, edge));
            }
        }
    }
    moment.abs()
}

#[test]
fn the_pieces_of_a_rounded_section_turn_into_pappus_s_volume_of_its_corners_and_arcs() {
    let volume = |leaf: &Leaf| turned(leaf).expect("a turn").volume();
    let asin = 0.8f64.asin();
    let closed = [
        ("a ball", volume(&ball(360.0)), 32.0 * PI / 3.0),
        ("a quarter of a ball", volume(&ball(90.0)), 8.0 * PI / 3.0),
        ("a ring torus", volume(&ring(360.0)), 6.0 * PI * PI),
        ("a quarter of a ring", volume(&ring(-90.0)), 1.5 * PI * PI),
        (
            "a shaft with a spherical cavity",
            volume(&hollowed()),
            54.0 * PI - 4.0 * PI / 3.0,
        ),
        (
            "a round-bottomed groove",
            volume(&grooved()),
            250.0 * PI / 3.0 - 3.0 * PI * PI,
        ),
    ];
    for (what, found, expected) in closed {
        assert!(
            (found - expected).abs() <= 1e-12 * expected,
            "{what}: {found} against {expected}"
        );
    }
    let by_hand = rounded_by_hand();
    let found = |index: usize| volume(&by_hand[index]);
    for (what, found, expected) in [
        ("a zone", found(8), 16.5 * PI),
        ("a cap, half way round", found(9), 28.0 * PI / 3.0),
        ("a shaft with a rounded end", found(10), 52.0 * PI / 3.0),
        (
            "the same from the other end, a quarter back",
            found(11),
            13.0 * PI / 3.0,
        ),
        ("an apple", found(12), PI * (59.0 / 3.0 - 18.75 * asin)),
    ] {
        assert!(
            (found - expected).abs() <= 1e-12 * expected,
            "{what}: {found} against {expected}"
        );
    }

    for leaf in &by_hand {
        let turn = turned(leaf).expect("a turn");
        let pappus = moment_with_arcs(&turn) * turn.angle();
        let volume = turn.volume();
        assert!(
            (volume - pappus).abs() <= 1e-9 * pappus,
            "{leaf}: {volume} against {pappus}"
        );
    }
}

#[test]
fn a_line_through_a_ball_or_a_torus_is_told_where_it_crosses_how_squarely_and_that_it_curves() {
    let along = |leaf: &Leaf, origin: [f64; 3], direction: [f64; 3]| {
        crossings_of(
            &leaf
                .along(DVec3::from(origin), DVec3::from(direction))
                .expect("a turn"),
        )
    };
    let whole = ball(360.0);
    let poles = along(&whole, [-5.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    assert!(
        close(&poles, &[(3.0, 0.0, true), (7.0, 0.0, true)]),
        "{poles:?}"
    );
    let root = 3f64.sqrt();
    let chord = along(&whole, [1.0, 0.0, -5.0], [0.0, 0.0, 1.0]);
    assert!(
        close(
            &chord,
            &[
                (5.0 - root, root / 2.0, true),
                (5.0 + root, root / 2.0, true)
            ]
        ),
        "{chord:?}"
    );
    let square = along(&whole, [0.0, -5.0, 0.0], [0.0, 1.0, 0.0]);
    assert!(
        close(&square, &[(3.0, 1.0, true), (7.0, 1.0, true)]),
        "{square:?}"
    );
    assert!(along(&whole, [0.0, 2.0, -5.0], [0.0, 0.0, 1.0]).is_empty());
    assert!(along(&whole, [0.0, 2.5, -5.0], [0.0, 0.0, 1.0]).is_empty());

    let quarter = along(&ball(90.0), [0.0, -5.0, 1.0], [0.0, 1.0, 0.0]);
    assert!(
        close(
            &quarter,
            &[(5.0, 1.0, false), (5.0 + root, root / 2.0, true)]
        ),
        "{quarter:?}"
    );

    let torus = ring(360.0);
    let through = along(&torus, [0.0, -5.0, 0.0], [0.0, 1.0, 0.0]);
    assert!(
        close(
            &through,
            &[
                (1.0, 1.0, true),
                (3.0, 1.0, true),
                (7.0, 1.0, true),
                (9.0, 1.0, true)
            ]
        ),
        "{through:?}"
    );
    let tube = along(&torus, [-5.0, 3.0, 0.0], [1.0, 0.0, 0.0]);
    assert!(
        close(&tube, &[(4.0, 1.0, true), (6.0, 1.0, true)]),
        "{tube:?}"
    );
    let rim = 3.75f64.sqrt();
    let beside = along(&torus, [0.0, 3.5, -5.0], [0.0, 0.0, 1.0]);
    assert!(
        close(
            &beside,
            &[(5.0 - rim, rim / 4.0, true), (5.0 + rim, rim / 4.0, true)]
        ),
        "{beside:?}"
    );
    assert!(along(&torus, [-5.0, 0.0, 0.0], [1.0, 0.0, 0.0]).is_empty());
    let over = 9.75f64.sqrt();
    let above = along(&torus, [0.0, -5.0, 2.5], [0.0, 1.0, 0.0]);
    assert!(
        close(
            &above,
            &[
                (5.0 - over, over / 4.0, true),
                (5.0 + over, over / 4.0, true)
            ]
        ),
        "{above:?}"
    );

    let cavity = along(&hollowed(), [0.0, 0.0, -5.0], [0.0, 0.0, 1.0]);
    assert!(
        close(
            &cavity,
            &[
                (2.0, 1.0, true),
                (4.0, 1.0, true),
                (6.0, 1.0, true),
                (8.0, 1.0, true)
            ]
        ),
        "{cavity:?}"
    );
    let groove = along(&grooved(), [3.0, 0.0, -5.0], [0.0, 0.0, 1.0]);
    assert!(
        close(&groove, &[(3.0, 1.0, true), (7.0, 1.0, true)]),
        "{groove:?}"
    );
    let flank = along(&grooved(), [2.5, 0.0, -5.0], [0.0, 0.0, 1.0]);
    let (bottom, slant) = (3.0 - 0.75f64.sqrt(), 0.75f64.sqrt());
    assert!(
        close(
            &flank,
            &[(5.0 - bottom, slant, true), (5.0 + bottom, slant, true)]
        ),
        "{flank:?}"
    );
}

#[test]
fn a_round_holds_along_every_line_what_rings_stacked_inside_it_hold_and_no_more_than_rings_stacked_around_it()
 {
    for leaf in &rounded_by_hand() {
        let turn = turned(leaf).expect("a turn");
        let pieces = turn.section.pieces();
        let (inside, around) = (stairs(&pieces, 24, true), stairs(&pieces, 24, false));
        let (lines, reach) = lines_over(leaf);
        let room = 1e-9 * reach;
        for index in 0..lines.count() {
            let (origin, direction) = lines.line(index);
            let [held, least, most] = [&pieces, &inside, &around]
                .map(|pieces| spans_of(&turn.along_pieces(pieces, origin, direction, 0.0)));
            assert!(
                least.without(&held).length() <= room && held.without(&most).length() <= room,
                "line {index} across {leaf}: {held:?} not between {least:?} and {most:?}"
            );
        }
    }
}

#[test]
fn a_rounded_leaf_grown_holds_every_line_it_held_and_shrunk_holds_none_it_did_not() {
    for leaf in &rounded_by_hand() {
        let (lines, reach) = lines_over(leaf);
        let by = 1e-3 * reach;
        for index in 0..lines.count() {
            let (origin, direction) = lines.line(index);
            let [shrunk, held, grown] = [-by, 0.0, by]
                .map(|by| spans_of(&leaf.along_grown(origin, direction, by).expect("a turn")));
            assert_eq!(
                shrunk.without(&held).length(),
                0.0,
                "line {index} of {leaf}"
            );
            assert_eq!(held.without(&grown).length(), 0.0, "line {index} of {leaf}");
            assert!(grown.length() > shrunk.length() || grown.length() == 0.0);
        }
    }
}

#[test]
fn a_rounded_leaf_off_its_axis_holds_along_every_line_and_spans_the_box_its_flats_hold_but_for_their_sagittas()
 {
    let leaves = turns_the_flats_hold(rounded_by_hand());
    assert!(leaves.len() >= 4, "{} leaves", leaves.len());
    for leaf in &leaves {
        let flats = leaf.solid().expect("a turn the flats make");
        holds_what_its_flats_hold(leaf, &flats);
        spans_the_box_its_flats_span(leaf, &flats);
    }
}

#[test]
fn a_ball_whose_centre_stands_a_hair_off_the_axis_spans_the_box_its_flats_span_on_either_side() {
    for (low, bows, degrees) in [
        (-2e-7, [[3.5, -2e-7, -2.5], [0.0; 3]], -180.0),
        (2e-7, [[0.0; 3], [3.5, 2e-7, 2.5]], 270.0),
    ] {
        let leaf = Leaf::turned(
            Plane::xy(2.0),
            Axis::first(0.0),
            Section::bands(1.0, &[[5.0, low, low]]).bowed(&[bows]),
            degrees,
        );
        let flats = leaf.solid().expect("a turn the flats make");
        spans_the_box_its_flats_span(&leaf, &flats);
        let alone = Case::new(leaf.clone(), vec![]);
        let held = random_solids::held_to_arithmetic(&alone, &Flats::for_case(&alone));
        assert!(held.is_ok(), "{leaf}: {held:?}");
    }
}

#[test]
fn a_rounded_leaf_shrinks_into_straight_edges_and_round_numbers() {
    let leaf = |section: Section| Leaf::turned(Plane::xy(0.0), Axis::first(0.0), section, 360.0);
    let rounded = Section::bands(0.3, &[[2.0, 0.0, 2.7], [2.0, 0.0, 2.0]])
        .sloping_to(&[[0.0, 2.7], [0.0, 0.0]])
        .bowed(&[[[0.0; 3], [1.3, 2.7, -1.0]], [[0.0; 3], [2.3, 0.0, 2.0]]]);
    let shrunk: Vec<Leaf> = Case::new(leaf(rounded.clone()), vec![])
        .smaller()
        .into_iter()
        .map(|case| case.start)
        .collect();
    let expected = [
        Section::bands(0.3, &[[2.0, 0.0, 2.7], [2.0, 0.0, 2.0]])
            .sloping_to(&[[0.0, 2.7], [0.0, 0.0]])
            .bowed(&[[[0.0; 3]; 2], [[0.0; 3], [2.3, 0.0, 2.0]]]),
        Section::bands(0.3, &[[2.0, 0.0, 2.7], [2.0, 0.0, 2.0]])
            .sloping_to(&[[0.0, 2.7], [0.0, 0.0]])
            .bowed(&[[[0.0; 3], [1.3, 2.7, -1.0]], [[0.0; 3]; 2]]),
        Section::bands(0.3, &[[2.0, 0.0, 2.7], [2.0, 0.0, 2.0]])
            .sloping_to(&[[0.0, 2.7], [0.0, 0.0]]),
        Section::bands(0.3, &[[2.0, 0.0, 2.0]])
            .sloping_to(&[[0.0, 0.0]])
            .bowed(&[[[0.0; 3], [2.3, 0.0, 2.0]]]),
        Section::bands(0.3, &[[4.0, 0.0, 2.7]]),
        Section::bands(0.0, &[[2.0, 0.0, 3.0], [2.0, 0.0, 2.0]])
            .sloping_to(&[[0.0, 3.0], [0.0, 0.0]])
            .bowed(&[[[0.0; 3], [1.0, 3.0, -1.0]], [[0.0; 3], [2.0, 0.0, 2.0]]]),
        Section::bands(0.5, &[[2.0, 0.0, 2.5], [2.0, 0.0, 2.0]])
            .sloping_to(&[[0.0, 2.5], [0.0, 0.0]])
            .bowed(&[[[0.0; 3], [1.5, 2.5, -1.0]], [[0.0; 3], [2.5, 0.0, 2.0]]]),
    ];
    for expected in expected {
        let expected = leaf(expected);
        assert!(shrunk.contains(&expected), "{expected} among {shrunk:#?}");
    }
    let straight = Section::bands(0.0, &[[2.0, 0.0, 3.0]]).sloping_to(&[[0.0, 1.0]]);
    assert!(
        Case::new(leaf(straight), vec![])
            .smaller()
            .iter()
            .all(|case| turned(&case.start).is_none_or(|turn| !turn.section.bows())),
        "a straight section shrinks into a bowed one"
    );
}

#[test]
fn a_case_asking_a_round_beyond_its_circles_is_told_from_one_that_does_not() {
    let asks = |start: &Leaf, tool: Leaf| {
        Case::new(start.clone(), vec![Step::cut(tool)]).asks_a_round_beyond_its_circles()
    };
    let knob = ball(360.0);
    let coaxial = |degrees: f64, section: Section| {
        Leaf::turned(Plane::xz(0.0), Axis::first(0.0), section, degrees)
    };
    let told = [
        (
            "a coaxial cylinder tangent at the equator",
            Leaf::prism(Plane::yz(-3.0), Outline::circle([0.0, 0.0], 2.0), 6.0),
            false,
        ),
        (
            "a coaxial cylinder a hair off the equator",
            Leaf::prism(Plane::yz(-3.0), Outline::circle([0.0, 0.0], 2.0000001), 6.0),
            false,
        ),
        (
            "a plane square through the centre",
            Leaf::prism(Plane::yz(0.0), Outline::circle([0.0, 0.0], 3.0), 3.0),
            false,
        ),
        (
            "a plane square at the pole",
            Leaf::prism(Plane::yz(2.0), Outline::circle([0.0, 0.0], 3.0), 3.0),
            false,
        ),
        (
            "a block whose face holds the axis",
            Leaf::prism(
                Plane::xy(0.0),
                Outline::rectangle([-1.0, -5.0], [5.0, 5.0]),
                10.0,
            ),
            true,
        ),
        (
            "a flat parallel to the axis and off it",
            Leaf::prism(
                Plane::xy(1.0),
                Outline::rectangle([-1.0, -5.0], [5.0, 5.0]),
                10.0,
            ),
            true,
        ),
        (
            "a plane at a slant",
            Leaf::prism(
                Plane::tilted([0.0, 0.0, 0.0], [0.0, 30.0, 0.0]),
                Outline::rectangle([-5.0, -5.0], [5.0, 5.0]),
                1.0,
            ),
            true,
        ),
        (
            "a cylinder parallel to the axis and off it",
            Leaf::prism(Plane::yz(-1.0), Outline::circle([1.0, 0.0], 1.0), 2.0),
            true,
        ),
        (
            "a radial hole",
            Leaf::prism(Plane::xz(5.0), Outline::circle([0.0, 0.0], 1.0), 10.0),
            true,
        ),
        (
            "a coaxial cone tangent at a latitude",
            coaxial(
                360.0,
                Section::bands(-1.0, &[[2.0, 0.0, 3.5]]).sloping_to(&[[0.0, 0.5]]),
            ),
            false,
        ),
        (
            "a cone a hair off the axis",
            Leaf::turned(
                Plane::xz(0.0),
                Axis::first(1e-7),
                Section::bands(-1.0, &[[2.0, 0.0, 3.5]]).sloping_to(&[[0.0, 0.5]]),
                360.0,
            ),
            true,
        ),
        ("the same ball again", ball(180.0), false),
        (
            "a coaxial torus sharing the tube circle",
            coaxial(
                360.0,
                Section::bands(-2.0, &[[4.0, 0.0, 0.0]]).bowed(&[[[0.0; 3], [0.0, 1.5, 2.5]]]),
            ),
            false,
        ),
        ("a coaxial ring", ring(360.0), false),
        (
            "a ball about another axis",
            Leaf::turned(
                Plane::xy(0.0),
                Axis::second(0.0),
                Section::bands(-2.0, &[[4.0, 0.0, 0.0]]).bowed(&[[[0.0; 3], [0.0, 0.0, 2.0]]]),
                360.0,
            ),
            true,
        ),
        (
            "a block whose box stands apart",
            Leaf::prism(
                Plane::xy(1.0),
                Outline::rectangle([20.0, 20.0], [25.0, 25.0]),
                2.0,
            ),
            false,
        ),
    ];
    for (what, tool, asking) in told {
        assert_eq!(asks(&knob, tool), asking, "{what}");
    }
    assert!(!Case::new(ball(90.0), vec![]).asks_a_round_beyond_its_circles());
    assert!(!Case::new(ring(-215.0), vec![]).asks_a_round_beyond_its_circles());
    let post = Leaf::prism(Plane::xy(-3.0), Outline::circle([1.0, 1.0], 1.0), 6.0);
    let shaft = Leaf::turned(
        Plane::xy(0.0),
        Axis::first(0.0),
        Section::bands(-2.0, &[[6.0, 0.0, 2.0]]),
        360.0,
    );
    assert!(!asks(&shaft, post.clone()), "no round asks");
    assert!(asks(&knob, post), "a post across a ball");
}

#[test]
fn a_rounded_case_prints_as_the_rust_that_builds_it_again() {
    let pasted = Case::new(
        Leaf::turned(
            Plane::xz(2.0),
            Axis::second(0.0),
            Section::bands(0.0, &[[3.0, 0.0, 2.0], [2.0, 0.0, 2.0]])
                .sloping_to(&[[0.0, 2.0], [0.0, 0.0]])
                .bowed(&[[[0.0; 3]; 2], [[0.0; 3], [3.0, 0.0, 2.0]]]),
            360.0,
        ),
        vec![Step::cut(ring(90.0))],
    );
    assert_eq!(
        pasted.to_string(),
        "Case::new(
    Leaf::turned(Plane::xz(2.0), Axis::second(0.0), Section::bands(0.0, &[[3.0, 0.0, 2.0], [2.0, 0.0, 2.0]]).sloping_to(&[[0.0, 2.0], [0.0, 0.0]]).bowed(&[[[0.0, 0.0, 0.0], [0.0, 0.0, 0.0]], [[0.0, 0.0, 0.0], [3.0, 0.0, 2.0]]]), 360.0),
    vec![
        Step::cut(Leaf::turned(Plane::xy(0.0), Axis::first(0.0), Section::bands(-1.0, &[[2.0, 3.0, 3.0]]).bowed(&[[[0.0, 3.0, -1.0], [0.0, 3.0, 1.0]]]), 90.0)),
    ],
)"
    );
}

#[test]
fn the_same_seed_draws_the_same_rounded_case_and_not_one_of_the_other_draws() {
    for seed in 0..200 {
        assert_eq!(
            Case::drawn_rounded(seed),
            Case::drawn_rounded(seed),
            "seed {seed}"
        );
    }
    let distinct: std::collections::BTreeSet<String> = (0..200)
        .map(|seed| Case::drawn_rounded(seed).to_string())
        .collect();
    assert!(distinct.len() > 190, "{} distinct cases", distinct.len());
    let same = (0..200)
        .filter(|seed| {
            let rounded = Case::drawn_rounded(*seed);
            FINGERPRINTED[..12]
                .iter()
                .any(|draw| rounded == draw(*seed))
        })
        .count();
    assert!(same < 20, "{same} rounded cases are another draw's");
}

#[test]
fn a_rounded_case_holds_profiles_of_runs_and_arcs_turned_about_lines_of_the_planes_of_the_origin_and_prisms()
 {
    let mut rounded = 0;
    for seed in 0..3000 {
        let case = Case::drawn_rounded(seed);
        for leaf in case.leaves() {
            assert!(leaf.is_solid(), "seed {seed}: {case}");
            assert!(
                !matches!(leaf.plane(), Plane::Tilted { .. }),
                "seed {seed}: {case}"
            );
            match leaf {
                Leaf::Prism { outline, .. } => {
                    assert!(
                        !matches!(outline, Outline::Star { .. }),
                        "seed {seed}: {case}"
                    )
                }
                Leaf::Turned { axis, section, .. } => {
                    assert_eq!(axis.lean, 0.0, "seed {seed}: {case}");
                    rounded += usize::from(section.bows());
                    for side in turned(leaf).expect("a turn").drawn() {
                        for corners in std::iter::once(&side.outline).chain(&side.holes) {
                            assert!(corners.len() >= 3, "seed {seed}: {case}");
                            for (index, corner) in corners.iter().enumerate() {
                                let run = corners[(index + 1) % corners.len()] - *corner;
                                assert!(
                                    run != DVec2::ZERO,
                                    "seed {seed}, a run of no length: {case}"
                                );
                            }
                        }
                    }
                }
                Leaf::Revolution { .. } => panic!("seed {seed} drew a revolution: {case}"),
            }
        }
    }
    assert!(rounded > 1000, "{rounded} rounded leaves");
}

/// The turned leaves whose section bows, of the first cases of the rounded
/// draw.
fn drawn_rounds(seeds: u64) -> Vec<Leaf> {
    (0..seeds)
        .flat_map(|seed| {
            Case::drawn_rounded(seed)
                .leaves()
                .cloned()
                .collect::<Vec<_>>()
        })
        .filter(|leaf| turned(leaf).is_some_and(|turn| turn.section.bows()))
        .collect()
}

/// An arc of a section read on the left of its axis: the band it bows, the
/// edge, the corners of the edge and the arc.
type Arc = (usize, usize, [DVec2; 2], Bow);

fn arcs_of(section: &Section) -> Vec<Arc> {
    (0..section.bands.len())
        .flat_map(|index| [0, 1].map(|edge| (index, edge)))
        .filter_map(|(index, edge)| {
            let bow = section.bow(index, edge)?;
            Some((index, edge, section.chord(index, edge), bow))
        })
        .collect()
}

/// The arcs of a turned leaf read about the world line it turns about: the
/// line, and each arc's centre as a level along the line's world axis and
/// a radius, its radius, and its corners read the same way.
struct Rounds {
    line: (DVec3, usize),
    arcs: Vec<(DVec2, f64, [DVec2; 2])>,
}

fn rounds(leaf: &Leaf) -> Option<Rounds> {
    let Leaf::Turned {
        plane,
        axis,
        section,
        ..
    } = leaf
    else {
        return None;
    };
    let line = world_line(leaf)?;
    let level = |along: f64| plane.to_world(axis.at(DVec2::new(along, 0.0)))[line.1];
    let read = |place: DVec2| DVec2::new(level(place.x), place.y.abs());
    let arcs = arcs_of(section)
        .into_iter()
        .map(|(_, _, chord, bow)| {
            (
                DVec2::new(level(bow.center.x), bow.center.y.abs()),
                bow.radius.abs(),
                chord.map(read),
            )
        })
        .collect();
    Some(Rounds { line, arcs })
}

/// The band whose wall a quarter arc of band `index` continues: the one
/// before it when the arc leaves the top of its circle, the one after it
/// when it arrives there, and none where the section ends.
fn wall_band(section: &Section, index: usize, chord: &[DVec2; 2], bow: &Bow) -> Option<usize> {
    if chord[0].x == bow.center.x {
        index.checked_sub(1)
    } else {
        Some(index + 1).filter(|at| *at < section.bands.len())
    }
}

#[test]
fn the_rounded_generator_draws_every_kind_it_keeps() {
    let cases: Vec<Case> = (0..1000).map(Case::drawn_rounded).collect();
    let leaves = || cases.iter().flat_map(Case::leaves);
    let turns = || leaves().filter_map(turned);
    let rounded = || turns().filter(|turn| turn.section.bows());
    let left = || rounded().map(|turn| on_the_left(&turn.section));
    let seen = |what: &str, found: bool| assert!(found, "no rounded case drew {what}");
    let hair = |section: &Section| 1e-4 * section.reach();
    let on_the_axis = |section: &Section, s: f64| s.abs() < hair(section);
    let lone = |section: &Section| -> Option<Arc> {
        let arcs = arcs_of(section);
        (section.bands.len() == 1 && arcs.len() == 1).then(|| arcs[0])
    };
    let is_ball = |section: &Section| {
        lone(section).is_some_and(|(_, edge, chord, bow)| {
            edge == 1
                && chord.iter().all(|corner| corner.y == bow.center.y)
                && on_the_axis(section, bow.center.y)
        })
    };

    seen(
        "a ball turned whole",
        rounded().any(|turn| turn.degrees.abs() == 360.0 && is_ball(&on_the_left(&turn.section))),
    );
    seen(
        "a ball turned part of the way",
        rounded().any(|turn| turn.degrees.abs() < 360.0 && is_ball(&on_the_left(&turn.section))),
    );
    seen(
        "a ball whose centre stands a hair off the axis",
        left().any(|section| {
            is_ball(&section) && lone(&section).is_some_and(|(.., bow)| bow.center.y != 0.0)
        }),
    );
    seen(
        "a zone",
        left().any(|section| {
            lone(&section).is_some_and(|(_, edge, chord, bow)| {
                edge == 1 && bow.center.y == 0.0 && chord.iter().all(|corner| corner.y > 0.0)
            })
        }),
    );
    seen(
        "a cap",
        left().any(|section| {
            lone(&section).is_some_and(|(_, edge, chord, bow)| {
                edge == 1
                    && bow.center.y == 0.0
                    && chord.iter().filter(|corner| corner.y == 0.0).count() == 1
                    && chord.iter().all(|corner| corner.x != bow.center.x)
            })
        }),
    );
    seen(
        "a ring torus",
        left().any(|section| {
            let arcs = arcs_of(&section);
            section.bands.len() == 1
                && arcs.len() == 2
                && arcs
                    .iter()
                    .all(|(_, _, chord, bow)| chord.iter().all(|corner| corner.y == bow.center.y))
                && arcs[0].3.center.y > 0.0
                && arcs[0].3.radius == -arcs[1].3.radius
        }),
    );
    seen(
        "an arc of a ring raised on a cylinder",
        left().any(|section| {
            arcs_of(&section).iter().any(|(_, edge, chord, bow)| {
                *edge == 1
                    && bow.radius > 0.0
                    && bow.center.y > 0.0
                    && chord.iter().all(|corner| corner.y > bow.center.y)
            })
        }),
    );
    seen(
        "an apple",
        left().any(|section| {
            lone(&section).is_some_and(|(_, _, chord, bow)| {
                bow.center.y < 0.0 && chord.iter().all(|corner| corner.y == 0.0)
            })
        }),
    );
    let quarter = |section: &Section, (_, edge, chord, bow): &Arc| {
        *edge == 1
            && bow.radius > 0.0
            && on_the_axis(section, bow.center.y)
            && chord.iter().any(|corner| on_the_axis(section, corner.y))
            && chord.iter().any(|corner| corner.x == bow.center.x)
    };
    seen(
        "a shaft with a rounded end",
        left().any(|section| {
            section.bands.len() >= 2 && arcs_of(&section).iter().any(|arc| quarter(&section, arc))
        }),
    );
    seen(
        "a round-bottomed groove",
        left().any(|section| {
            arcs_of(&section).iter().any(|(index, edge, chord, bow)| {
                *edge == 1
                    && bow.radius < 0.0
                    && chord[0].y == chord[1].y
                    && *index > 0
                    && index + 1 < section.bands.len()
            })
        }),
    );
    seen(
        "a groove below walls",
        left().any(|section| {
            arcs_of(&section).iter().any(|(index, edge, chord, bow)| {
                *edge == 1
                    && bow.radius < 0.0
                    && chord[0].y == chord[1].y
                    && *index > 0
                    && section.edges(index - 1).1[1] > chord[0].y
            })
        }),
    );
    seen(
        "a rounded shoulder",
        left().any(|section| {
            arcs_of(&section).iter().any(|(_, edge, chord, bow)| {
                *edge == 1 && bow.radius > 0.0 && chord[0].y != chord[1].y && bow.center.y > 0.0
            })
        }),
    );
    seen(
        "a fillet",
        left().any(|section| {
            arcs_of(&section).iter().any(|(_, edge, chord, bow)| {
                *edge == 1 && bow.radius < 0.0 && chord[0].y != chord[1].y
            })
        }),
    );
    seen(
        "a shoulder rounded across the whole step",
        left().any(|section| {
            arcs_of(&section).iter().any(|(index, edge, chord, bow)| {
                *edge == 1
                    && bow.radius > 0.0
                    && chord[0].y != chord[1].y
                    && bow.center.y > 0.0
                    && (chord[0].y - chord[1].y).abs() == bow.radius
                    && {
                        let (before, after) = (
                            index.checked_sub(1).map(|at| section.edges(at).1[1]),
                            section.edges(index + 1).0[1],
                        );
                        before == Some(chord[0].y.max(chord[1].y))
                            || after == chord[0].y.max(chord[1].y)
                    }
            })
        }),
    );
    seen(
        "a barrel bore",
        left().any(|section| {
            arcs_of(&section)
                .iter()
                .any(|(_, edge, chord, bow)| *edge == 0 && bow.radius < 0.0 && chord[0].y > 0.0)
        }),
    );
    seen(
        "a round groove in a bore",
        left().any(|section| {
            arcs_of(&section)
                .iter()
                .any(|(_, edge, chord, bow)| *edge == 0 && bow.radius > 0.0 && chord[0].y > 0.0)
        }),
    );
    seen(
        "a sphere a hair larger than the shaft it ends",
        left().any(|section| {
            arcs_of(&section)
                .iter()
                .any(|arc @ (index, _, chord, bow)| {
                    quarter(&section, arc) && {
                        let wall = match wall_band(&section, *index, chord, bow) {
                            Some(at) if at < *index => section.edges(at).1[1],
                            Some(at) => section.edges(at).0[1],
                            None => f64::NAN,
                        };
                        wall != bow.radius && (wall - bow.radius).abs() < hair(&section)
                    }
                })
        }),
    );
    seen(
        "an arc a hair from tangent to the wall it continues",
        left().any(|section| {
            arcs_of(&section)
                .iter()
                .any(|arc @ (index, _, chord, bow)| {
                    quarter(&section, arc) && {
                        let Some(at) = wall_band(&section, *index, chord, bow) else {
                            return false;
                        };
                        let (start, end) = section.edges(at);
                        start[1] != end[1] && (start[1] - end[1]).abs() < hair(&section)
                    }
                })
        }),
    );
    seen(
        "a quarter circle whose centre stands a hair off the axis",
        left().any(|section| {
            arcs_of(&section)
                .iter()
                .any(|arc @ (.., bow)| quarter(&section, arc) && bow.center.y != 0.0)
        }),
    );
    seen(
        "an arc a hair short of a quarter circle",
        (0..1000)
            .map(Case::drawn_rounded_off_the_lattice)
            .flat_map(|case| case.leaves().cloned().collect::<Vec<_>>())
            .filter_map(|leaf| turned(&leaf).map(|turn| on_the_left(&turn.section)))
            .any(|section| {
                arcs_of(&section).iter().any(|(_, edge, chord, bow)| {
                    *edge == 1
                        && bow.center.y == 0.0
                        && chord.iter().any(|corner| corner.x == bow.center.x)
                        && chord
                            .iter()
                            .any(|corner| corner.y != 0.0 && corner.y < hair(&section))
                })
            }),
    );

    let earlier_and_later = |related: &dyn Fn(&Leaf, &Leaf) -> bool| {
        cases.iter().any(|case| {
            let leaves: Vec<&Leaf> = case.leaves().collect();
            (1..leaves.len()).any(|later| {
                leaves[..later]
                    .iter()
                    .any(|earlier| related(earlier, leaves[later]))
            })
        })
    };
    let after_a_round = |related: &dyn Fn(&Rounds, &Leaf) -> bool| {
        earlier_and_later(&|earlier, later| {
            rounds(earlier).is_some_and(|earlier| {
                world_line(later).is_some_and(|line| same_line(earlier.line, line))
                    && related(&earlier, later)
            })
        })
    };
    let level_walls =
        |leaf: &Leaf| -> Vec<[DVec2; 2]> { meridian(leaf).map_or(Vec::new(), |read| read.level) };
    seen(
        "a coaxial cylinder at a ball's equator",
        after_a_round(&|earlier, later| {
            earlier.arcs.iter().any(|(center, radius, _)| {
                level_walls(later).iter().any(|wall| {
                    wall[0].y == center.y + radius && wall[0].x < center.x && center.x < wall[1].x
                })
            })
        }),
    );
    seen(
        "a coaxial cylinder a hair off a ball's equator",
        after_a_round(&|earlier, later| {
            earlier.arcs.iter().any(|(center, radius, _)| {
                level_walls(later).iter().any(|wall| {
                    let off = (wall[0].y - center.y - radius).abs();
                    0.0 < off && off < 1e-4 * radius
                })
            })
        }),
    );
    seen(
        "a shaft ending square through a ball's centre",
        after_a_round(&|earlier, later| {
            earlier.arcs.iter().any(|(center, _, _)| {
                level_walls(later)
                    .iter()
                    .any(|wall| wall.iter().any(|end| end.x == center.x))
            })
        }),
    );
    seen(
        "a shaft ending square at a ball's pole",
        after_a_round(&|earlier, later| {
            earlier.arcs.iter().any(|(center, radius, _)| {
                level_walls(later).iter().any(|wall| {
                    wall.iter()
                        .any(|end| end.x == center.x + radius || end.x == center.x - radius)
                })
            })
        }),
    );
    seen(
        "a shaft ending a hair from a ball's pole",
        after_a_round(&|earlier, later| {
            earlier.arcs.iter().any(|(center, radius, _)| {
                level_walls(later).iter().any(|wall| {
                    wall.iter().any(|end| {
                        let off = (end.x - center.x - radius).abs();
                        0.0 < off && off < 1e-4 * radius
                    })
                })
            })
        }),
    );
    seen(
        "the same ball again",
        after_a_round(&|earlier, later| {
            rounds(later)
                .is_some_and(|later| later.arcs.iter().any(|arc| earlier.arcs.contains(arc)))
        }),
    );
    seen(
        "the same ball a hair off",
        after_a_round(&|earlier, later| {
            rounds(later).is_some_and(|later| {
                later.arcs.iter().any(|(center, radius, _)| {
                    earlier.arcs.iter().any(|(other, other_radius, _)| {
                        let off = (center.x - other.x)
                            .abs()
                            .max((radius - other_radius).abs());
                        center.y == other.y && 0.0 < off && off < 1e-4 * radius
                    })
                })
            })
        }),
    );
    seen(
        "a coaxial torus sharing the tube circle",
        after_a_round(&|earlier, later| {
            rounds(later).is_some_and(|later| {
                later.arcs.len() == 2
                    && later.arcs.iter().all(|(center, radius, _)| {
                        center.y > 0.0
                            && earlier.arcs.iter().any(|(other, other_radius, _)| {
                                center == other && radius == other_radius
                            })
                    })
            })
        }),
    );
    seen(
        "a cone tangent at a latitude",
        after_a_round(&|earlier, later| {
            meridian(later).is_some_and(|read| {
                read.sloped.iter().any(|slope| {
                    earlier.arcs.iter().any(|(center, radius, _)| {
                        let touch = DVec2::new(center.x + 0.6 * radius, center.y + 0.8 * radius);
                        off_the_line(*slope, touch) <= 1e-12
                            && (slope[1] - slope[0])
                                .normalize()
                                .dot((touch - *center).normalize())
                                .abs()
                                <= 1e-12
                    })
                })
            })
        }),
    );
    seen(
        "a circle square to the axis through a ball's centre or at its pole",
        after_a_round(&|earlier, later| {
            let (Some(line), Leaf::Prism { plane, outline, .. }) = (world_line(later), later)
            else {
                return false;
            };
            let level = plane.to_world(DVec2::ZERO)[line.1];
            matches!(outline, Outline::Circle { .. } | Outline::Ring { .. })
                && earlier.arcs.iter().any(|(center, radius, _)| {
                    [center.x, center.x + radius, center.x - radius].contains(&level)
                })
        }),
    );
    seen(
        "a block through a ball's axis",
        earlier_and_later(&|earlier, later| {
            let (
                Some(earlier),
                Leaf::Prism {
                    plane,
                    outline: Outline::Rectangle { low, high },
                    ..
                },
            ) = (rounds(earlier), later)
            else {
                return false;
            };
            let (point, axis) = earlier.line;
            let normal = plane.normal();
            normal[axis] == 0.0
                && (plane.to_world(DVec2::ZERO) - point).dot(normal) == 0.0
                && [
                    *low,
                    *high,
                    DVec2::new(low.x, high.y),
                    DVec2::new(high.x, low.y),
                ]
                .iter()
                .any(|corner| plane.to_world(*corner)[axis] != point[axis])
        }),
    );
    seen(
        "a ball on a circle raised before it",
        earlier_and_later(&|earlier, later| {
            let (Some(radius), Some(one), Some(later)) =
                (radius_of(earlier), world_line(earlier), rounds(later))
            else {
                return false;
            };
            same_line(one, later.line)
                && later.arcs.iter().any(|(center, arc, chord)| {
                    center.y == 0.0 && *arc >= radius && chord.iter().all(|corner| corner.y == 0.0)
                })
        }),
    );
    seen(
        "a dome on a circle raised before it",
        earlier_and_later(&|earlier, later| {
            let (Some(radius), Some(one), Some(later)) =
                (radius_of(earlier), world_line(earlier), rounds(later))
            else {
                return false;
            };
            same_line(one, later.line)
                && later.arcs.iter().any(|(center, arc, chord)| {
                    center.y == 0.0
                        && *arc == radius
                        && chord
                            .iter()
                            .any(|corner| corner.y == radius && corner.x == center.x)
                })
        }),
    );
    let rounded_tool = |mode: Mode| {
        cases.iter().any(|case| {
            case.steps.iter().any(|step| {
                step.mode == mode && turned(&step.tool).is_some_and(|turn| turn.section.bows())
            })
        })
    };
    seen("a rounded tool added", rounded_tool(Mode::Add));
    seen("a rounded tool cut", rounded_tool(Mode::Cut));

    let share = rounded().count() as f64 / leaves().count() as f64;
    assert!(
        (0.2..0.55).contains(&share),
        "rounded leaves are {} of {}",
        rounded().count(),
        leaves().count()
    );
}

#[test]
#[ignore = "#576: the exact kernel does not turn an arc yet"]
fn every_rounded_leaf_raised_by_the_exact_kernel_alone_keeps_every_rule_and_encloses_pappus_s_volume()
 {
    let leaves = drawn_rounds(80);
    assert!(leaves.len() > 30, "{} leaves", leaves.len());
    raised_alone_keep_every_rule_and_enclose_pappus_s_volume(&leaves);
}

#[test]
#[ignore = "#576: the exact kernel does not turn an arc yet"]
fn a_rounded_leaf_names_its_faces_as_the_flats_name_them() {
    let leaves: Vec<Leaf> = drawn_rounds(120)
        .into_iter()
        .filter(|leaf| turned(leaf).is_some_and(|turn| turn.section.side().is_some()))
        .collect();
    assert!(leaves.len() > 40, "{} leaves", leaves.len());
    let exact = name_their_faces_as_the_flats_name_them(&leaves);
    assert_eq!(
        exact,
        leaves.len(),
        "every leaf is turned by the exact kernel"
    );
}

#[test]
#[ignore = "#576: the exact kernel does not turn an arc yet"]
fn rounded_cases_the_exact_kernel_keeps_are_kept_through_the_application_s_body_and_stay_exact() {
    let held = kept_through_the_application_s_body_and_exact(Case::drawn_rounded, 0..8);
    assert!(held > 3, "{held} cases held");
}

#[test]
#[ignore = "#576: the exact kernel does not turn an arc yet"]
fn random_rounded_cases_keep_every_rule_on_the_exact_kernel() {
    keep_every_rule_on_the_exact_kernel(Case::drawn_rounded, 0..16);
}

#[test]
fn a_drawn_rounded_leaf_off_its_axis_holds_along_every_line_and_spans_the_box_its_flats_hold_but_for_their_sagittas()
 {
    let leaves = turns_the_flats_hold(drawn_rounds(300));
    assert!(leaves.len() > 20, "{} leaves", leaves.len());
    for leaf in &leaves {
        let flats = leaf.solid().expect("a turn the flats make");
        holds_what_its_flats_hold(leaf, &flats);
        spans_the_box_its_flats_span(leaf, &flats);
    }
}

#[test]
fn the_draws_before_rounds_still_give_each_seed_its_case() {
    for (rank, (draw, expected)) in FINGERPRINTED.iter().zip(BEFORE_ROUNDS).enumerate() {
        assert_eq!(fingerprint(*draw), expected, "draw {rank}");
    }
}

#[test]
fn the_draws_of_rounds_give_each_seed_its_case() {
    assert_eq!(fingerprint(Case::drawn_rounded), 0x3020_0021_b2d6_1f1d);
    assert_eq!(
        fingerprint(Case::drawn_rounded_off_the_lattice),
        0x8627_22c7_af4c_e33a
    );
}

/// The campaigns, compiled only when asked for — `--features campaigns` —
/// so that work on any other issue never pays for them.
#[cfg(feature = "campaigns")]
mod campaign {
    use std::sync::Arc;
    use std::sync::atomic::Ordering;

    use cao_solid::soundness::{Check, Flaw};
    use random_solids::campaigning::{
        Answering, DECLINED, GRAZING, HELD, THIN, TIPS, UNSTABLE, campaign_over, shrunk_one_by_one,
        tally_conics,
    };

    use super::*;

    fn tallied(measured: random_solids::Measured) {
        HELD.fetch_add(measured.held, Ordering::Relaxed);
        GRAZING.fetch_add(measured.grazing, Ordering::Relaxed);
        TIPS.fetch_add(measured.through_a_tip, Ordering::Relaxed);
        DECLINED.fetch_add(measured.declined, Ordering::Relaxed);
        UNSTABLE.fetch_add(measured.unstable, Ordering::Relaxed);
        THIN.fetch_add(measured.thin, Ordering::Relaxed);
    }

    fn exactly(case: &Case) -> Result<(), Flaw> {
        tallied(random_solids::held_to_arithmetic(case, &Exact)?);
        tally_conics(case);
        Ok(())
    }

    fn through_the_application(case: &Case) -> Result<(), Flaw> {
        tallied(random_solids::held_to_arithmetic(case, &Application)?);
        tally_conics(case);
        Ok(())
    }

    /// The case a seed stands for, with the seed written where a campaign
    /// that ends the program still leaves it to be read.
    fn drawn(seed: u64) -> Case {
        eprint!("\rseed {seed} ");
        Case::drawn_turned_off_the_lattice(seed)
    }

    /// The same, among slanted turns (#536).
    fn drawn_slanted(seed: u64) -> Case {
        eprint!("\rseed {seed} ");
        Case::drawn_slanted_off_the_lattice(seed)
    }

    /// The same, among tools on leaning planes (#552).
    fn drawn_tilted(seed: u64) -> Case {
        eprint!("\rseed {seed} ");
        Case::drawn_tilted_off_the_lattice(seed)
    }

    /// The same, among turns whose edges bow (#576).
    fn drawn_rounded(seed: u64) -> Case {
        eprint!("\rseed {seed} ");
        Case::drawn_rounded_off_the_lattice(seed)
    }

    #[test]
    #[ignore = "a campaign, run by hand: see the head of this file"]
    fn a_campaign_of_turned_solids_on_the_exact_kernel_keeps_every_rule() {
        campaign_over("turned solids", Answering::Exactly, drawn, exactly);
    }

    #[test]
    #[ignore = "a campaign, run by hand: see the head of this file"]
    fn a_campaign_of_turned_solids_through_the_application_s_body_keeps_every_rule() {
        campaign_over(
            "turned solids",
            Answering::ThroughTheApplication,
            drawn,
            through_the_application,
        );
    }

    #[test]
    #[ignore = "a campaign, run by hand: see the head of this file"]
    fn a_campaign_of_slanted_solids_on_the_exact_kernel_keeps_every_rule() {
        campaign_over("slanted solids", Answering::Exactly, drawn_slanted, exactly);
    }

    #[test]
    #[ignore = "a campaign, run by hand: see the head of this file"]
    fn a_campaign_of_slanted_solids_through_the_application_s_body_keeps_every_rule() {
        campaign_over(
            "slanted solids",
            Answering::ThroughTheApplication,
            drawn_slanted,
            through_the_application,
        );
    }

    #[test]
    #[ignore = "a campaign, run by hand: see the head of this file"]
    fn a_campaign_of_tilted_solids_on_the_exact_kernel_keeps_every_rule() {
        campaign_over("tilted solids", Answering::Exactly, drawn_tilted, exactly);
    }

    #[test]
    #[ignore = "a campaign, run by hand: see the head of this file"]
    fn a_campaign_of_tilted_solids_through_the_application_s_body_keeps_every_rule() {
        campaign_over(
            "tilted solids",
            Answering::ThroughTheApplication,
            drawn_tilted,
            through_the_application,
        );
    }

    #[test]
    #[ignore = "a campaign, run by hand: see the head of this file"]
    fn a_campaign_of_rounded_solids_on_the_exact_kernel_keeps_every_rule() {
        campaign_over("rounded solids", Answering::Exactly, drawn_rounded, exactly);
    }

    #[test]
    #[ignore = "a campaign, run by hand: see the head of this file"]
    fn a_campaign_of_rounded_solids_through_the_application_s_body_keeps_every_rule() {
        campaign_over(
            "rounded solids",
            Answering::ThroughTheApplication,
            drawn_rounded,
            through_the_application,
        );
    }

    /// Each seed named in `CAO_TRIAGE_SEEDS`, commas between them, drawn as
    /// a turning campaign draws it — as a slanted one when `CAO_TRIAGE_DRAW`
    /// is `slanted`, as a tilted one when it is `tilted`, as a rounded one
    /// when it is `rounded` — run again on the exact kernel — through the
    /// application's body when `CAO_TRIAGE_KERNEL` is `application` —
    /// shrunk while it still breaks the same rule, and printed as a test.
    #[test]
    #[ignore = "run by hand on the seeds a campaign named"]
    fn the_seeds_a_turning_campaign_named_are_shrunk_one_by_one() {
        let seeds: Vec<u64> = std::env::var("CAO_TRIAGE_SEEDS")
            .unwrap_or_default()
            .split(',')
            .filter_map(|seed| seed.trim().parse().ok())
            .collect();
        let check: Check<Case> = match std::env::var("CAO_TRIAGE_KERNEL").as_deref() {
            Ok("application") => Arc::new(through_the_application),
            _ => Arc::new(exactly),
        };
        let draw: fn(u64) -> Case = match std::env::var("CAO_TRIAGE_DRAW").as_deref() {
            Ok("slanted") => Case::drawn_slanted_off_the_lattice,
            Ok("tilted") => Case::drawn_tilted_off_the_lattice,
            Ok("rounded") => Case::drawn_rounded_off_the_lattice,
            _ => Case::drawn_turned_off_the_lattice,
        };
        shrunk_one_by_one(seeds, draw, check);
    }

    /// How many of the cases `draw` gives the first 300 seeds the flats
    /// answer by their own rules, each of them asserted answered by the
    /// arithmetic too: every seed that is not is printed before the
    /// assertion, since a campaign this long is not run twice to find the
    /// second.
    fn held_by_the_flats_and_the_arithmetic(draw: fn(u64) -> Case) -> usize {
        let seeds: Vec<u64> = (0..300).collect();
        let weighed = random_solids::on_every_core(&seeds, |seed| {
            let case = draw(*seed);
            random_solids::check(&case).ok()?;
            let measured = random_solids::held_to_arithmetic(&case, &Flats::for_case(&case));
            Some((measured, case))
        });
        let mut held = 0;
        let mut failed = Vec::new();
        for (seed, (measured, case)) in seeds
            .iter()
            .zip(weighed)
            .filter_map(|(seed, one)| Some((seed, one?)))
        {
            held += 1;
            if let Err(flaw) = measured {
                println!("seed {seed}: {flaw:?}\n{case}");
                failed.push(*seed);
            }
        }
        println!("{held} cases of {} held", seeds.len());
        assert!(failed.is_empty(), "seeds {failed:?} broke a rule");
        held
    }

    /// The harness held to the flats, where whole cases of turns are too slow
    /// and too often broken for the gate (#418, #486, #492): every case the
    /// flats answer by their own rules is answered by the arithmetic too.
    #[test]
    #[ignore = "a campaign, run by hand once: see the head of this file"]
    fn turned_cases_the_flats_answer_by_their_own_rules_keep_every_rule_held_to_the_arithmetic() {
        let held = held_by_the_flats_and_the_arithmetic(Case::drawn_turned);
        assert!(held > 100, "{held} cases held");
    }

    /// The cone arithmetic held to the flats before the exact kernel turns a
    /// cone (#536): the flats were written long before it, and owe it
    /// nothing.
    #[test]
    #[ignore = "a campaign, run by hand once: see the head of this file"]
    fn slanted_cases_the_flats_answer_by_their_own_rules_keep_every_rule_held_to_the_arithmetic() {
        let held = held_by_the_flats_and_the_arithmetic(Case::drawn_slanted);
        assert!(held > 100, "{held} cases held");
    }

    /// The arithmetic of leaning tools held to the flats before the exact
    /// kernel cuts a conic (#552): a seam where a conic was approximated
    /// would show on the flats, which owe the draw nothing.
    #[test]
    #[ignore = "a campaign, run by hand once: see the head of this file"]
    fn tilted_cases_the_flats_answer_by_their_own_rules_keep_every_rule_held_to_the_arithmetic() {
        let held = held_by_the_flats_and_the_arithmetic(Case::drawn_tilted);
        assert!(held > 100, "{held} cases held");
    }

    /// The arithmetic of arcs held to the flats before the exact kernel
    /// turns one (#576): the flats were written long before it, and owe it
    /// nothing. A whole rounded case on the flats takes the better part of a
    /// minute where a torus is cut several times, so the gate holds the
    /// drawn leaves alone to them and the cases are held here, once.
    #[test]
    #[ignore = "a campaign, run by hand once: see the head of this file"]
    fn rounded_cases_the_flats_answer_by_their_own_rules_keep_every_rule_held_to_the_arithmetic() {
        let held = held_by_the_flats_and_the_arithmetic(Case::drawn_rounded);
        assert!(held > 100, "{held} cases held");
    }
}
