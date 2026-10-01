//! Solids drawn at random and combined at random by the exact kernel of #498,
//! held to the rules every result must keep.
//!
//! The cases are #448's, drawn among what the kernel raises: prisms of
//! rectangles and circles on the three planes of the origin
//! (`Case::drawn_square`), still weighted towards coincidences and hairs —
//! or, in a campaign of its own, among the profiles the application makes all
//! the time (`Case::drawn_profiles`): rectangles with rounded corners, slots
//! and rings beside rectangles and circles, each handed to the kernel as its
//! straight runs and arcs. Both draws stay what they were for a seed, so a
//! campaign on either is comparable with the ones before it.
//! Every leaf is raised by `cao_solid::brep::Body::raised` and every step
//! joined or cut; a leaf or a step declined is no answer. At every step the
//! body's listing is held to its geometry, its triangles are closed and
//! uncrossed, they hold along the harness's lines what arithmetic promised
//! from the leaves — within the room their tolerance takes, the lines grazing
//! a curved wall left out and counted — and nothing beyond the box the leaves
//! span; and the whole case run twice gives the same triangles, bit for bit.
//!
//! The arithmetic is held to the flats here, where it can be: on straight
//! sides it is what the flats measure, on circles it is within their
//! sagitta, and the check that holds the exact kernel keeps every case the
//! flats keep their own rules on — some two thirds of the first three
//! thousand square cases, when it was written — and breaks every one they
//! break.
//!
//! A campaign is run by hand, the square one or the one of profiles:
//!
//! ```text
//! CAO_FUZZ_SECONDS=3600 cargo test --release -p cao_solid \
//!     --test random_exact_solids -- --ignored --nocapture \
//!     a_campaign_of_random_square_solids
//! CAO_FUZZ_SECONDS=3600 cargo test --release -p cao_solid \
//!     --test random_exact_solids -- --ignored --nocapture \
//!     a_campaign_of_random_profiles
//! ```
//!
//! `CAO_FUZZ_SEED` starts it from a given seed rather than from the clock, and
//! `CAO_FUZZ_PATIENCE` is how many seconds one case may take before it counts
//! as no answer. The seeds it names are shrunk one by one by
//! `the_seeds_a_campaign_named_are_shrunk_one_by_one`, given in
//! `CAO_TRIAGE_SEEDS` with `CAO_TRIAGE_DRAW=profiles` for the profiles.

// The drawing, the promise and the checks are shared with the flats'
// campaign; each file uses its own part of them.
#[allow(dead_code, unused_imports)]
mod random_solids;

use std::cell::Cell;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant, SystemTime};

use cao_solid::Mesh;
use cao_solid::brep::{Curve, Line, ListedEdge, Listing};
use cao_solid::profile::Run;
use cao_solid::soundness::{
    Check, Flaw, Lines, Mislisted, Random, Report, Silence, Spans, Triangle, answer, campaign,
    shrink,
};
use glam::{DVec2, DVec3};
use random_solids::{Case, Exact, Flats, Kernel, Leaf, Mode, Outline, Plane, Step, Stretch};

/// A bundle of lines laid over the box a leaf's flats span, and how far that
/// box reaches.
fn lines_over(leaf: &Leaf) -> (Lines, Vec<Spans>, f64) {
    let flats = leaf.solid().expect("a leaf the flats raise");
    let (low, high) = flats.bounds().expect("a solid with a box");
    let reach = low.abs().max(high.abs()).max_element().max(1.0);
    let lines = Lines::across(low - 1.0, high + 1.0, 24);
    let measured = lines.inside(&flats.triangles());
    (lines, measured, reach)
}

fn spans_of(stretches: &[Stretch]) -> Spans {
    Spans::gathered(
        stretches
            .iter()
            .map(|stretch| (stretch.from.at, stretch.to.at))
            .collect(),
    )
}

/// The prisms of the first cases of both draws that hold every outline: every
/// kind, and the profiles.
fn drawn_prisms(kept: impl Fn(&Outline) -> bool) -> Vec<Leaf> {
    (0..400)
        .flat_map(|seed| [Case::drawn(seed), Case::drawn_profiles(seed)])
        .flat_map(|case| case.leaves().cloned().collect::<Vec<Leaf>>())
        .filter(|leaf| {
            leaf.is_solid() && matches!(leaf, Leaf::Prism { outline, .. } if kept(outline))
        })
        .collect()
}

#[test]
fn a_prism_of_straight_sides_holds_along_every_line_exactly_what_its_flats_hold() {
    let leaves =
        drawn_prisms(|outline| matches!(outline, Outline::Rectangle { .. } | Outline::Star { .. }));
    assert!(leaves.len() > 100, "{} leaves", leaves.len());
    for leaf in &leaves {
        let (lines, measured, reach) = lines_over(leaf);
        for (index, measure) in measured.iter().enumerate() {
            let (origin, direction) = lines.line(index);
            let promise = leaf.along(origin, direction).expect("a prism");
            let promised = spans_of(&promise);
            assert_eq!(
                promised.stretches().len(),
                measure.stretches().len(),
                "line {index} across {leaf}: {promised:?} against {measure:?}"
            );
            for (one, other) in promised.stretches().iter().zip(measure.stretches()) {
                assert!(
                    (one.0 - other.0).abs() <= 1e-9 * reach
                        && (one.1 - other.1).abs() <= 1e-9 * reach,
                    "line {index} across {leaf}: {one:?} against {other:?}"
                );
            }
        }
    }
}

/// The same leaf with its circles drawn at other radii.
fn resized(leaf: &Leaf, outer: impl Fn(f64) -> f64, inner: impl Fn(f64) -> f64) -> Leaf {
    let Leaf::Prism {
        plane,
        outline,
        height,
    } = leaf
    else {
        unreachable!("only prisms are resized")
    };
    let outline = match outline {
        Outline::Circle {
            center,
            radius,
            from,
        } => Outline::Circle {
            center: *center,
            radius: outer(*radius),
            from: *from,
        },
        Outline::Ring {
            center,
            outer: rim,
            inner: bore,
        } => Outline::Ring {
            center: *center,
            outer: outer(*rim),
            inner: inner(*bore),
        },
        other => other.clone(),
    };
    Leaf::prism(*plane, outline, *height)
}

#[test]
fn a_prism_of_circles_holds_along_every_line_what_its_flats_hold_but_for_their_sagitta() {
    let inscribed = (std::f64::consts::PI / random_solids::CIRCLE_STEPS as f64).cos();
    let leaves =
        drawn_prisms(|outline| matches!(outline, Outline::Circle { .. } | Outline::Ring { .. }));
    assert!(leaves.len() > 100, "{} leaves", leaves.len());
    for leaf in &leaves {
        let least = resized(leaf, |radius| radius * inscribed, |radius| radius);
        let most = resized(leaf, |radius| radius, |radius| radius * inscribed);
        let (lines, measured, reach) = lines_over(leaf);
        for (index, measure) in measured.iter().enumerate() {
            let (origin, direction) = lines.line(index);
            let [least, most] = [&least, &most]
                .map(|bound| spans_of(&bound.along(origin, direction).expect("a prism")));
            let short = least.without(measure).length();
            let over = measure.without(&most).length();
            assert!(
                short <= 1e-9 * reach && over <= 1e-9 * reach,
                "line {index} across {leaf}: {measure:?} between {least:?} and {most:?}"
            );
        }
    }
}

/// Whether a point lies on one of the two ends of a prism, rather than on its
/// wall.
fn on_an_end(leaf: &Leaf, point: DVec3, reach: f64) -> bool {
    let Leaf::Prism { plane, height, .. } = leaf else {
        unreachable!("only prisms have ends")
    };
    let level = (point - plane.frame().0).dot(plane.normal());
    level.abs() <= 1e-9 * reach || (level - height).abs() <= 1e-9 * reach
}

fn corner_radius(leaf: &Leaf) -> f64 {
    match leaf {
        Leaf::Prism {
            outline: Outline::Rounded { radius, .. } | Outline::Slot { radius, .. },
            ..
        } => *radius,
        _ => unreachable!("only rounded rectangles and slots have a corner radius"),
    }
}

#[test]
fn a_rounded_rectangle_or_a_slot_holds_along_every_line_what_its_flats_hold_on_its_straight_runs_and_but_for_their_sagitta_on_its_arcs()
 {
    let inscribed = (std::f64::consts::PI / random_solids::CIRCLE_STEPS as f64).cos();
    let leaves =
        drawn_prisms(|outline| matches!(outline, Outline::Rounded { .. } | Outline::Slot { .. }));
    assert!(leaves.len() > 100, "{} leaves", leaves.len());
    let mut straight = 0;
    for leaf in &leaves {
        let sagitta = corner_radius(leaf) * (1.0 - inscribed);
        let (lines, measured, reach) = lines_over(leaf);
        for (index, measure) in measured.iter().enumerate() {
            let (origin, direction) = lines.line(index);
            let promise = leaf.along(origin, direction).expect("a prism");
            let most = spans_of(&promise);
            let least = spans_of(
                &leaf
                    .along_grown(origin, direction, -sagitta)
                    .expect("a prism"),
            );
            let short = least.without(measure).length();
            let over = measure.without(&most).length();
            assert!(
                short <= 1e-9 * reach && over <= 1e-9 * reach,
                "line {index} across {leaf}: {measure:?} between {least:?} and {most:?}"
            );
            let ends: Vec<f64> = measure
                .stretches()
                .iter()
                .flat_map(|&(from, to)| [from, to])
                .collect();
            for end in promise
                .iter()
                .flat_map(|stretch| [stretch.from, stretch.to])
                .filter(|end| {
                    !end.curved
                        && end.at.is_finite()
                        && !on_an_end(leaf, origin + direction * end.at, reach)
                })
            {
                straight += 1;
                assert!(
                    ends.iter().any(|at| (at - end.at).abs() <= 1e-9 * reach),
                    "line {index} across {leaf}: {measure:?} has no end at {end:?}"
                );
            }
        }
    }
    assert!(straight > 1000, "{straight} ends on straight runs");
}

#[test]
fn a_line_through_a_rounded_rectangle_or_a_slot_is_told_whether_it_crosses_a_straight_run_or_an_arc()
 {
    let along = |leaf: &Leaf, origin: [f64; 3], direction: DVec3, by: f64| {
        leaf.along_grown(DVec3::from(origin), direction, by)
            .expect("a prism")
    };
    let ends = |stretches: Vec<Stretch>| {
        stretches
            .iter()
            .map(|stretch| (stretch.from.at, stretch.to.at))
            .collect::<Vec<_>>()
    };
    let root = 3.0_f64.sqrt();

    let block = Leaf::prism(
        Plane::xy(0.0),
        Outline::rounded([0.0, 0.0], [10.0, 6.0], 2.0),
        10.0,
    );
    let [low] = along(&block, [-1.0, 1.0, 1.0], DVec3::X, 0.0)[..] else {
        panic!("one stretch across the lower corners");
    };
    assert!((low.from.at - (3.0 - root)).abs() < 1e-12, "{low:?}");
    assert!((low.to.at - (9.0 + root)).abs() < 1e-12, "{low:?}");
    for end in [low.from, low.to] {
        assert!(
            end.curved && (end.cosine - root / 2.0).abs() < 1e-12,
            "{end:?}"
        );
    }
    let [middle] = along(&block, [-1.0, 3.0, 1.0], DVec3::X, 0.0)[..] else {
        panic!("one stretch across the middle");
    };
    assert_eq!((middle.from.at, middle.to.at), (1.0, 11.0));
    for end in [middle.from, middle.to] {
        assert!(!end.curved && end.cosine == 1.0, "{end:?}");
    }
    assert_eq!(
        ends(along(&block, [1.0, 1.0, -1.0], DVec3::Z, 0.0)),
        [(1.0, 11.0)]
    );
    assert_eq!(ends(along(&block, [0.3, 0.3, -1.0], DVec3::Z, 0.0)), []);
    assert_eq!(
        ends(along(&block, [0.3, 0.3, -1.0], DVec3::Z, 0.5)),
        [(0.5, 11.5)]
    );
    assert_eq!(
        ends(along(&block, [2.6, 2.6, -1.0], DVec3::Z, -2.5)),
        [(3.5, 8.5)]
    );

    let slot = Leaf::prism(
        Plane::xy(0.0),
        Outline::slot([2.0, 5.0], [8.0, 5.0], 1.5),
        3.0,
    );
    let [cap] = along(&slot, [9.0, 0.0, 1.0], DVec3::Y, 0.0)[..] else {
        panic!("one stretch through a cap");
    };
    let half = 1.25_f64.sqrt();
    assert!((cap.from.at - (5.0 - half)).abs() < 1e-12, "{cap:?}");
    assert!((cap.to.at - (5.0 + half)).abs() < 1e-12, "{cap:?}");
    for end in [cap.from, cap.to] {
        assert!(
            end.curved && (end.cosine - half / 1.5).abs() < 1e-12,
            "{end:?}"
        );
    }
    let [side] = along(&slot, [5.0, 0.0, 1.0], DVec3::Y, 0.0)[..] else {
        panic!("one stretch across the straight runs");
    };
    assert_eq!((side.from.at, side.to.at), (3.5, 6.5));
    assert!(!side.from.curved && !side.to.curved, "{side:?}");
    assert_eq!(
        ends(along(&slot, [0.0, 5.0, 1.0], DVec3::X, 0.0)),
        [(0.5, 9.5)]
    );
    assert_eq!(ends(along(&slot, [5.0, 0.0, 1.0], DVec3::Y, -1.5)), []);
}

#[test]
fn the_box_a_prism_spans_is_the_box_its_flats_span_on_any_plane() {
    let inscribed = (std::f64::consts::PI / random_solids::CIRCLE_STEPS as f64).cos();
    let leaves = drawn_prisms(|_| true);
    assert!(
        leaves
            .iter()
            .any(|leaf| matches!(leaf.plane(), Plane::Tilted { .. })),
        "no prism drawn on a turned plane"
    );
    for leaf in &leaves {
        let (low, high) = leaf.bounds().expect("a prism has a box");
        let flats = leaf.solid().expect("a leaf the flats raise");
        let (flat_low, flat_high) = flats.bounds().expect("a solid with a box");
        let reach = flat_low.abs().max(flat_high.abs()).max_element().max(1.0);
        let sagitta = match leaf {
            Leaf::Prism {
                outline: Outline::Circle { radius, .. },
                ..
            } => radius * (1.0 - inscribed),
            Leaf::Prism {
                outline: Outline::Ring { outer, .. },
                ..
            } => outer * (1.0 - inscribed),
            Leaf::Prism {
                outline: Outline::Rounded { radius, .. } | Outline::Slot { radius, .. },
                ..
            } => radius * (1.0 - inscribed),
            _ => 0.0,
        };
        let near = 1e-9 * reach;
        let within =
            |one: DVec3, other: DVec3, room: f64| (one - other).abs().max_element() <= room;
        assert!(
            flat_low.cmpge(low - near).all() && flat_high.cmple(high + near).all(),
            "{leaf}: the flats span {flat_low}..{flat_high} beyond {low}..{high}"
        );
        assert!(
            within(low, flat_low, sagitta + near) && within(high, flat_high, sagitta + near),
            "{leaf}: {low}..{high} against the flats' {flat_low}..{flat_high}"
        );
    }
}

#[test]
fn a_block_on_a_turned_plane_is_held_by_the_flats_to_the_arithmetic() {
    let case = Case::new(
        Leaf::prism(
            Plane::tilted([1.0, 2.0, 0.0], [0.0, 0.0, 45.0]),
            Outline::rectangle([0.0, 0.0], [4.0, 3.0]),
            2.0,
        ),
        vec![],
    );
    let measured = random_solids::held_to_arithmetic(&case, &Flats::for_case(&case));
    assert!(measured.is_ok(), "{measured:?}");
}

#[test]
fn a_leaf_grown_holds_every_line_it_held_and_a_leaf_shrunk_holds_none_it_did_not() {
    let leaves = drawn_prisms(|outline| !matches!(outline, Outline::Star { .. }));
    for leaf in &leaves {
        let (lines, _, reach) = lines_over(leaf);
        let by = 1e-3 * reach;
        for index in 0..lines.count() {
            let (origin, direction) = lines.line(index);
            let [shrunk, held, grown] = [-by, 0.0, by]
                .map(|by| spans_of(&leaf.along_grown(origin, direction, by).expect("a prism")));
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
fn the_same_seed_draws_the_same_square_case() {
    for seed in 0..200 {
        assert_eq!(
            Case::drawn_square(seed),
            Case::drawn_square(seed),
            "seed {seed}"
        );
    }
    let distinct: std::collections::BTreeSet<String> = (0..200)
        .map(|seed| Case::drawn_square(seed).to_string())
        .collect();
    assert!(distinct.len() > 190, "{} distinct cases", distinct.len());
}

#[test]
fn a_square_case_holds_prisms_of_rectangles_and_circles_on_the_planes_of_the_origin_alone() {
    for seed in 0..3000 {
        let case = Case::drawn_square(seed);
        for leaf in case.leaves() {
            let kept = match leaf {
                Leaf::Prism { plane, outline, .. } => {
                    !matches!(plane, Plane::Tilted { .. })
                        && matches!(outline, Outline::Rectangle { .. } | Outline::Circle { .. })
                }
                Leaf::Revolution { .. } => false,
            };
            assert!(kept, "seed {seed}: {case}");
            assert!(leaf.is_solid(), "seed {seed}: {case}");
        }
    }
}

#[test]
fn the_square_generator_draws_every_kind_it_keeps() {
    let cases: Vec<Case> = (0..1000).map(Case::drawn_square).collect();
    let leaves = || cases.iter().flat_map(Case::leaves);
    let seen = |what: &str, found: bool| assert!(found, "no square case drew {what}");
    let outline = |leaf: &Leaf| match leaf {
        Leaf::Prism { outline, .. } => Some(outline.clone()),
        Leaf::Revolution { .. } => None,
    };

    seen(
        "a rectangle",
        leaves().any(|leaf| matches!(outline(leaf), Some(Outline::Rectangle { .. }))),
    );
    seen(
        "a circle",
        leaves().any(|leaf| matches!(outline(leaf), Some(Outline::Circle { .. }))),
    );
    seen(
        "a circle not started at nought",
        leaves()
            .any(|leaf| matches!(outline(leaf), Some(Outline::Circle { from, .. }) if from != 0.0)),
    );
    seen(
        "a prism pushed backwards",
        leaves().any(|leaf| matches!(leaf, Leaf::Prism { height, .. } if *height < 0.0)),
    );
    for (name, kind) in [
        ("XY", Plane::xy(0.0)),
        ("XZ", Plane::xz(0.0)),
        ("YZ", Plane::yz(0.0)),
    ] {
        seen(
            name,
            leaves()
                .any(|leaf| std::mem::discriminant(leaf.plane()) == std::mem::discriminant(&kind)),
        );
    }
    let steps = || cases.iter().flat_map(|case| &case.steps);
    seen("a cut", steps().any(|step| step.mode == Mode::Cut));
    seen("an addition", steps().any(|step| step.mode == Mode::Add));
    for count in 1..=5 {
        seen(
            &format!("{count} steps"),
            cases.iter().any(|case| case.steps.len() == count),
        );
    }
    seen(
        "a tool on the plane of the solid before it",
        cases.iter().any(|case| {
            case.steps
                .iter()
                .any(|step| step.tool.plane() == case.start.plane())
        }),
    );
    seen(
        "a plane missed by a hair",
        cases.iter().any(|case| {
            case.steps
                .iter()
                .any(|step| match (case.start.plane(), step.tool.plane()) {
                    (Plane::Xy(first), Plane::Xy(second)) => {
                        first != second && (first - second).abs() < 1e-4
                    }
                    _ => false,
                })
        }),
    );
    seen(
        "a case drawn thirty times larger",
        leaves().any(|leaf| matches!(leaf, Leaf::Prism { height, .. } if height.abs() > 60.0)),
    );
    seen(
        "two circles on one centre",
        cases.iter().any(|case| {
            let centres: Vec<DVec2> = case
                .leaves()
                .filter_map(|leaf| match outline(leaf) {
                    Some(Outline::Circle { center, .. }) => Some(center),
                    _ => None,
                })
                .collect();
            (1..centres.len()).any(|index| centres[..index].contains(&centres[index]))
        }),
    );
}

fn outline_of(leaf: &Leaf) -> Option<&Outline> {
    match leaf {
        Leaf::Prism { outline, .. } => Some(outline),
        Leaf::Revolution { .. } => None,
    }
}

#[test]
fn the_same_seed_draws_the_same_profile_case_and_not_the_square_one() {
    for seed in 0..200 {
        assert_eq!(
            Case::drawn_profiles(seed),
            Case::drawn_profiles(seed),
            "seed {seed}"
        );
    }
    let distinct: std::collections::BTreeSet<String> = (0..200)
        .map(|seed| Case::drawn_profiles(seed).to_string())
        .collect();
    assert!(distinct.len() > 190, "{} distinct cases", distinct.len());
    let same = (0..200)
        .filter(|seed| Case::drawn_profiles(*seed) == Case::drawn_square(*seed))
        .count();
    assert!(same < 100, "{same} profile cases are the square ones");
}

#[test]
fn a_profile_case_holds_solid_prisms_on_the_planes_of_the_origin_alone_and_no_star() {
    for seed in 0..3000 {
        let case = Case::drawn_profiles(seed);
        for leaf in case.leaves() {
            let kept = match leaf {
                Leaf::Prism { plane, outline, .. } => {
                    !matches!(plane, Plane::Tilted { .. })
                        && !matches!(outline, Outline::Star { .. })
                }
                Leaf::Revolution { .. } => false,
            };
            assert!(kept, "seed {seed}: {case}");
            assert!(leaf.is_solid(), "seed {seed}: {case}");
        }
    }
}

#[test]
fn the_profile_generator_draws_every_kind_it_keeps_and_rectangles_and_circles_for_about_half() {
    let cases: Vec<Case> = (0..1000).map(Case::drawn_profiles).collect();
    let leaves = || cases.iter().flat_map(Case::leaves);
    let outlines = || leaves().filter_map(outline_of);
    let seen = |what: &str, found: bool| assert!(found, "no profile case drew {what}");

    seen(
        "a rounded rectangle with a straight run on every side",
        outlines().any(|outline| {
            matches!(outline, Outline::Rounded { low, high, radius }
                if 2.0 * radius < (*high - *low).min_element())
        }),
    );
    seen(
        "a rounded rectangle whose radius is half its shorter side",
        outlines().any(|outline| {
            matches!(outline, Outline::Rounded { low, high, radius }
                if 2.0 * radius == (*high - *low).min_element())
        }),
    );
    seen(
        "a slot along the plane's first axis",
        outlines()
            .any(|outline| matches!(outline, Outline::Slot { from, to, .. } if from.y == to.y)),
    );
    seen(
        "a slot along the plane's second axis",
        outlines()
            .any(|outline| matches!(outline, Outline::Slot { from, to, .. } if from.x == to.x)),
    );
    seen(
        "a ring",
        outlines().any(|outline| matches!(outline, Outline::Ring { .. })),
    );
    seen(
        "a prism pushed backwards",
        leaves().any(|leaf| matches!(leaf, Leaf::Prism { height, .. } if *height < 0.0)),
    );
    for (name, kind) in [
        ("XY", Plane::xy(0.0)),
        ("XZ", Plane::xz(0.0)),
        ("YZ", Plane::yz(0.0)),
    ] {
        seen(
            name,
            leaves()
                .any(|leaf| std::mem::discriminant(leaf.plane()) == std::mem::discriminant(&kind)),
        );
    }
    seen(
        "a case drawn thirty times larger",
        leaves().any(|leaf| matches!(leaf, Leaf::Prism { height, .. } if height.abs() > 60.0)),
    );

    let earlier_and_later = |related: &dyn Fn(&Outline, &Outline) -> bool| {
        cases.iter().any(|case| {
            let outlines: Vec<&Outline> = case.leaves().filter_map(outline_of).collect();
            (1..outlines.len()).any(|later| {
                outlines[..later]
                    .iter()
                    .any(|earlier| related(earlier, outlines[later]))
            })
        })
    };
    seen(
        "a corner radius equal to a circle's drawn before, on its centre",
        earlier_and_later(&|earlier, later| match (earlier, later) {
            (
                Outline::Circle { center, radius, .. },
                Outline::Rounded {
                    low,
                    radius: corner,
                    ..
                },
            ) => corner == radius && *low + *corner == *center,
            _ => false,
        }),
    );
    seen(
        "a slot's end on a circle's centre",
        earlier_and_later(&|earlier, later| match (earlier, later) {
            (Outline::Circle { center, .. }, Outline::Slot { from, .. }) => from == center,
            _ => false,
        }),
    );
    seen(
        "a slot's end missed by a hair",
        earlier_and_later(&|earlier, later| match (earlier, later) {
            (Outline::Circle { center, .. }, Outline::Slot { from, .. }) => {
                from != center && from.distance(*center) < 1e-4
            }
            _ => false,
        }),
    );
    seen(
        "a slot's straight run on a rectangle's side",
        earlier_and_later(&|earlier, later| match (earlier, later) {
            (Outline::Rectangle { high, .. }, Outline::Slot { from, to, radius }) => {
                from.y == to.y && (from.y - radius == high.y || from.y + radius == high.y)
            }
            _ => false,
        }),
    );
    seen(
        "a ring's bore equal to a circle drawn before",
        earlier_and_later(&|earlier, later| match (earlier, later) {
            (
                Outline::Circle { center, radius, .. },
                Outline::Ring {
                    center: hub, inner, ..
                },
            ) => center == hub && radius == inner,
            _ => false,
        }),
    );

    let count = leaves().count();
    let plain = outlines()
        .filter(|outline| matches!(outline, Outline::Rectangle { .. } | Outline::Circle { .. }))
        .count();
    let share = plain as f64 / count as f64;
    assert!(
        (0.4..0.6).contains(&share),
        "rectangles and circles are {plain} leaves of {count}"
    );
}

#[test]
fn a_profile_case_prints_as_the_rust_that_builds_it_again() {
    let pasted = Case::new(
        Leaf::prism(
            Plane::xy(0.0),
            Outline::rounded([0.0, 0.0], [10.0, 6.0], 1.5),
            4.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xz(-1.0),
                Outline::slot([2.0, 1.0], [2.0, 3.5], 0.75),
                -8.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(4.0),
                Outline::ring([5.0, 3.0], 2.5, 1.0),
                2.0,
            )),
        ],
    );
    assert_eq!(
        pasted.to_string(),
        "Case::new(
    Leaf::prism(Plane::xy(0.0), Outline::rounded([0.0, 0.0], [10.0, 6.0], 1.5), 4.0),
    vec![
        Step::cut(Leaf::prism(Plane::xz(-1.0), Outline::slot([2.0, 1.0], [2.0, 3.5], 0.75), -8.0)),
        Step::add(Leaf::prism(Plane::xy(4.0), Outline::ring([5.0, 3.0], 2.5, 1.0), 2.0)),
    ],
)"
    );
    for seed in 0..200 {
        let printed = Case::drawn_profiles(seed).to_string();
        assert!(
            !printed.contains("NaN") && !printed.contains("inf"),
            "seed {seed} prints a number Rust cannot read back: {printed}"
        );
    }
}

/// The outlines a lone prism of `outline` could shrink into.
fn shrunk_outlines(outline: Outline) -> Vec<Outline> {
    Case::new(Leaf::prism(Plane::xy(0.0), outline, 1.0), vec![])
        .smaller()
        .into_iter()
        .filter_map(|case| outline_of(&case.start).cloned())
        .collect()
}

#[test]
fn a_rounded_rectangle_shrinks_into_a_rectangle_and_a_slot_or_a_ring_into_a_circle_and_all_into_round_numbers()
 {
    let rounded = shrunk_outlines(Outline::rounded([0.3, 0.7], [10.2, 6.4], 1.7));
    for expected in [
        Outline::rectangle([0.3, 0.7], [10.2, 6.4]),
        Outline::rounded([0.0, 1.0], [10.0, 6.0], 2.0),
        Outline::rounded([0.5, 0.5], [10.0, 6.5], 1.5),
    ] {
        assert!(rounded.contains(&expected), "{expected} among {rounded:?}");
    }
    let tight = shrunk_outlines(Outline::rounded([0.0, 0.0], [10.0, 2.6], 1.3));
    assert!(
        tight.contains(&Outline::rounded([0.0, 0.0], [10.0, 3.0], 1.0)),
        "{tight:?}"
    );

    let slot = shrunk_outlines(Outline::slot([2.2, 5.0], [8.1, 5.0], 1.3));
    for expected in [
        Outline::circle([2.2, 5.0], 1.3),
        Outline::slot([2.0, 5.0], [8.0, 5.0], 1.0),
        Outline::slot([2.0, 5.0], [8.0, 5.0], 1.5),
    ] {
        assert!(slot.contains(&expected), "{expected} among {slot:?}");
    }
    assert!(
        slot.iter()
            .any(|outline| matches!(outline, Outline::Rectangle { low, high }
            if low.distance(DVec2::new(0.9, 3.7)) < 1e-12
                && high.distance(DVec2::new(9.4, 6.3)) < 1e-12)),
        "no box among {slot:?}"
    );
    let short = shrunk_outlines(Outline::slot([2.2, 5.0], [2.2, 5.4], 1.0));
    assert!(
        short
            .iter()
            .all(|outline| !matches!(outline, Outline::Slot { from, to, .. } if from == to)),
        "{short:?}"
    );

    let ring = shrunk_outlines(Outline::ring([5.2, 3.0], 2.6, 1.1));
    for expected in [
        Outline::circle([5.2, 3.0], 2.6),
        Outline::ring([5.0, 3.0], 3.0, 1.0),
        Outline::ring([5.0, 3.0], 2.5, 1.0),
    ] {
        assert!(ring.contains(&expected), "{expected} among {ring:?}");
    }
    let thin = shrunk_outlines(Outline::ring([5.0, 3.0], 2.6, 2.4));
    assert!(
        thin.iter().all(
            |outline| !matches!(outline, Outline::Ring { outer, inner, .. } if inner >= outer)
        ),
        "{thin:?}"
    );
}

#[test]
fn shrinking_any_drawn_profile_case_comes_to_an_end() {
    for seed in 0..200 {
        let mut rounds = 0;
        let shrunk = shrink(
            Case::drawn_profiles(seed),
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

fn bored_and_bossed() -> Case {
    Case::new(
        Leaf::prism(
            Plane::xy(0.0),
            Outline::rectangle([0.0, 0.0], [20.0, 10.0]),
            10.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(-1.0),
                Outline::circle([6.0, 5.0], 2.5),
                12.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(10.0),
                Outline::rectangle([12.0, 2.0], [18.0, 8.0]),
                4.0,
            )),
        ],
    )
}

/// A block bored through along Y from a circle not started at nought, given a
/// boss on its side along X, and pocketed from its top by a circle.
fn bored_across() -> Case {
    Case::new(
        Leaf::prism(
            Plane::xy(0.0),
            Outline::rectangle([0.0, 0.0], [10.0, 8.5]),
            10.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xz(-1.0),
                Outline::circle_from([3.0, 4.0], 2.5, 30.0),
                -12.0,
            )),
            Step::add(Leaf::prism(
                Plane::yz(10.0),
                Outline::rectangle([2.0, 2.0], [6.0, 6.0]),
                3.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(10.0),
                Outline::circle([7.0, 5.0], 1.5),
                -3.0,
            )),
        ],
    )
}

#[test]
fn a_block_bored_and_given_a_boss_by_the_flats_keeps_every_rule_held_to_the_arithmetic() {
    let case = bored_and_bossed();
    let measured = random_solids::held_to_arithmetic(&case, &Flats::for_case(&case));
    assert!(measured.is_ok(), "{measured:?}");
}

#[test]
fn a_block_bored_across_and_given_a_boss_on_its_side_by_the_flats_keeps_every_rule_held_to_the_arithmetic()
 {
    let case = bored_across();
    let measured = random_solids::held_to_arithmetic(&case, &Flats::for_case(&case));
    assert!(measured.is_ok(), "{measured:?}");
}

#[test]
fn square_cases_the_flats_answer_by_their_own_rules_keep_every_rule_held_to_the_arithmetic() {
    let mut held = 0;
    for seed in 0..120 {
        let case = Case::drawn_square(seed);
        if random_solids::check(&case).is_err() {
            continue;
        }
        held += 1;
        let measured = random_solids::held_to_arithmetic(&case, &Flats::for_case(&case));
        assert!(measured.is_ok(), "seed {seed}: {measured:?}\n{case}");
    }
    assert!(held > 60, "{held} cases held");
}

#[test]
fn profile_cases_the_flats_answer_by_their_own_rules_keep_every_rule_held_to_the_arithmetic() {
    let mut held = 0;
    let mut profiles = 0;
    for seed in 0..120 {
        let case = Case::drawn_profiles(seed);
        if random_solids::check(&case).is_err() {
            continue;
        }
        held += 1;
        profiles += case
            .leaves()
            .filter_map(outline_of)
            .filter(|outline| {
                matches!(
                    outline,
                    Outline::Rounded { .. } | Outline::Slot { .. } | Outline::Ring { .. }
                )
            })
            .count();
        let measured = random_solids::held_to_arithmetic(&case, &Flats::for_case(&case));
        assert!(measured.is_ok(), "seed {seed}: {measured:?}\n{case}");
    }
    assert!(held > 60, "{held} cases held");
    assert!(profiles > 60, "{profiles} profiles held");
}

#[test]
fn lines_grazing_a_curved_wall_are_left_out_and_counted() {
    let case = Case::new(
        Leaf::prism(Plane::xz(2.0), Outline::circle([1.0, 3.0], 4.0), 5.0),
        vec![],
    );
    let measured =
        random_solids::held_to_arithmetic(&case, &Flats::for_case(&case)).expect("a sound disc");
    assert!(measured.grazing > 0, "{measured:?}");
    assert!(measured.held > 20 * measured.grazing, "{measured:?}");
}

/// A kernel broken on purpose, the flats underneath.
struct Broken {
    flats: Flats,
    breaking: Breaking,
    calls: Cell<usize>,
}

#[derive(Clone, Copy, PartialEq)]
enum Breaking {
    ForgetsItsCuts,
    DeclinesLeaves,
    DeclinesSteps,
    ListsWrong,
    AnswersTwiceDifferently,
    LeavesMatterAside,
    HoldsNothingExactly,
}

impl Broken {
    fn on(case: &Case, breaking: Breaking) -> Broken {
        Broken {
            flats: Flats::for_case(case),
            breaking,
            calls: Cell::new(0),
        }
    }
}

impl Kernel for Broken {
    type Body = Mesh;

    fn raised(&self, leaf: &Leaf) -> Option<Mesh> {
        (self.breaking != Breaking::DeclinesLeaves)
            .then(|| self.flats.raised(leaf))
            .flatten()
    }

    fn combined(&self, body: &Mesh, tool: &Mesh, mode: Mode) -> Option<Mesh> {
        match (self.breaking, mode) {
            (Breaking::ForgetsItsCuts, Mode::Cut) => Some(body.clone()),
            (Breaking::DeclinesSteps, _) => None,
            _ => self.flats.combined(body, tool, mode),
        }
    }

    fn triangles(&self, body: &Mesh) -> (Vec<Triangle>, f64) {
        let (mut triangles, tolerance) = self.flats.triangles(body);
        self.calls.set(self.calls.get() + 1);
        match self.breaking {
            Breaking::AnswersTwiceDifferently => {
                let corner = &mut triangles[0][0].x;
                *corner = f64::from_bits(corner.to_bits() + self.calls.get() as u64);
            }
            Breaking::LeavesMatterAside => {
                let far = Leaf::prism(
                    Plane::xy(0.0),
                    Outline::rectangle([100.0, 100.0], [101.0, 101.0]),
                    1.0,
                );
                triangles.extend(far.solid().expect("a cube").triangles());
            }
            _ => {}
        }
        (triangles, tolerance)
    }

    fn crossings(
        &self,
        _body: &Mesh,
        _origin: DVec3,
        _direction: DVec3,
    ) -> Option<Vec<(f64, i32)>> {
        (self.breaking == Breaking::HoldsNothingExactly).then(Vec::new)
    }

    fn listing(&self, _body: &Mesh) -> Option<(Listing, f64)> {
        (self.breaking == Breaking::ListsWrong).then(|| {
            let line = Line::through(DVec3::ZERO, DVec3::X);
            let edge = ListedEdge {
                curve: Curve::Line(line),
                from: 0.0,
                to: 1.0,
                ends: Some([0, 1]),
                sides: Vec::new(),
            };
            let listing = Listing {
                faces: Vec::new(),
                edges: vec![edge],
                vertices: vec![DVec3::ZERO, DVec3::X],
            };
            (listing, 1.0)
        })
    }
}

fn broken(breaking: Breaking) -> Result<random_solids::Measured, Flaw> {
    let case = bored_and_bossed();
    random_solids::held_to_arithmetic(&case, &Broken::on(&case, breaking))
}

#[test]
fn a_kernel_that_forgets_its_cuts_is_caught_along_a_line_that_should_have_lost_matter() {
    let found = broken(Breaking::ForgetsItsCuts);
    assert!(
        matches!(&found, Err(Flaw::Volume { worst: Some(worst), .. }) if worst.enclosed > worst.promised),
        "{found:?}"
    );
}

#[test]
fn a_kernel_that_declines_a_leaf_or_a_step_gives_no_answer() {
    for breaking in [Breaking::DeclinesLeaves, Breaking::DeclinesSteps] {
        assert!(matches!(
            broken(breaking),
            Err(Flaw::NoAnswer(Silence::Refused))
        ));
    }
}

#[test]
fn a_kernel_that_lists_its_body_wrong_is_caught_by_the_listing() {
    assert!(matches!(
        broken(Breaking::ListsWrong),
        Err(Flaw::Mislisted(Mislisted::Unused { edge: 0 }))
    ));
}

#[test]
fn a_kernel_that_answers_the_same_case_twice_differently_is_caught() {
    assert!(matches!(
        broken(Breaking::AnswersTwiceDifferently),
        Err(Flaw::Unrepeatable { .. })
    ));
}

#[test]
fn a_kernel_whose_exact_body_holds_nothing_where_matter_was_promised_is_caught_before_its_triangles()
 {
    assert!(
        matches!(broken(Breaking::HoldsNothingExactly), Err(Flaw::Spans(worst)) if worst.enclosed == 0.0 && worst.promised > 0.0)
    );
}

#[test]
fn matter_a_kernel_leaves_beyond_what_its_leaves_span_is_caught() {
    assert!(matches!(
        broken(Breaking::LeavesMatterAside),
        Err(Flaw::Volume { worst: None, .. })
    ));
}

#[test]
fn a_line_through_a_prism_is_told_how_squarely_it_crosses_each_end_and_whether_it_curves() {
    let leaf = Leaf::prism(Plane::xy(1.0), Outline::circle([0.0, 0.0], 2.0), 3.0);
    let slanted = DVec3::new(0.6, 0.0, 0.8);
    let stretches = leaf
        .along(DVec3::new(-0.5, 0.0, 0.0), slanted)
        .expect("a prism");
    let [stretch] = stretches.as_slice() else {
        panic!("one stretch: {stretches:?}");
    };
    assert!((stretch.from.at - 1.25).abs() < 1e-12, "{stretch:?}");
    assert!((stretch.from.cosine - 0.8).abs() < 1e-12 && !stretch.from.curved);
    assert!((stretch.to.at - 2.5 / 0.6).abs() < 1e-12, "{stretch:?}");
    assert!((stretch.to.cosine - 0.6).abs() < 1e-12 && stretch.to.curved);
}

#[test]
fn a_line_square_to_a_star_holds_the_whole_prism_inside_the_star_and_nothing_between_its_points() {
    let center = DVec2::new(1.0, 1.0);
    let corners: Vec<DVec2> = (0..10)
        .map(|index| {
            let reach = if index % 2 == 0 { 2.0 } else { 0.8 };
            center + DVec2::from_angle(0.3 + std::f64::consts::TAU * index as f64 / 10.0) * reach
        })
        .collect();
    let leaf = Leaf::prism(
        Plane::xy(1.0),
        Outline::Star {
            center,
            corners: corners.clone(),
        },
        3.0,
    );
    let square = |at: DVec2| {
        leaf.along(at.extend(-1.0), DVec3::Z)
            .expect("a prism")
            .iter()
            .map(|stretch| (stretch.from.at, stretch.to.at))
            .collect::<Vec<_>>()
    };
    assert_eq!(square(center), [(2.0, 5.0)]);
    let between = center + (corners[1] - center) * 1.5;
    assert_eq!(square(between), []);
    assert_eq!(square(center + DVec2::X * 5.0), []);
}

/// The area a profile encloses, by the formula for its shape.
fn area(outline: &Outline) -> f64 {
    use std::f64::consts::PI;
    match outline {
        Outline::Rectangle { low, high } => (*high - *low).x * (*high - *low).y,
        Outline::Circle { radius, .. } => PI * radius * radius,
        Outline::Ring { outer, inner, .. } => PI * (outer * outer - inner * inner),
        Outline::Rounded { low, high, radius } => {
            (*high - *low).x * (*high - *low).y - (4.0 - PI) * radius * radius
        }
        Outline::Slot { from, to, radius } => {
            2.0 * radius * from.distance(*to) + PI * radius * radius
        }
        Outline::Star { .. } => unreachable!("a star is not raised by the exact kernel"),
    }
}

#[test]
fn every_profile_raised_by_the_exact_kernel_alone_keeps_every_rule_and_encloses_its_area_times_its_height()
 {
    let leaves: Vec<Leaf> = (0..200)
        .flat_map(|seed| {
            Case::drawn_profiles(seed)
                .leaves()
                .cloned()
                .collect::<Vec<_>>()
        })
        .filter(|leaf| {
            matches!(
                leaf,
                Leaf::Prism {
                    outline: Outline::Rounded { .. } | Outline::Slot { .. } | Outline::Ring { .. },
                    ..
                }
            )
        })
        .collect();
    assert!(leaves.len() > 60, "{} leaves", leaves.len());
    for leaf in &leaves {
        let alone = Case::new(leaf.clone(), vec![]);
        let held = random_solids::held_to_arithmetic(&alone, &Exact);
        assert!(held.is_ok(), "{leaf}: {held:?}");
        let Leaf::Prism {
            outline, height, ..
        } = leaf
        else {
            unreachable!("only prisms were kept")
        };
        let promised = area(outline) * height.abs();
        let volume = Exact.raised(leaf).expect("a raised profile").volume();
        assert!(
            (volume - promised).abs() <= 1e-9 * promised,
            "{leaf}: {volume} against {promised}"
        );
    }
}

#[test]
fn a_rounded_rectangle_is_handed_over_with_a_straight_run_only_on_a_side_longer_than_its_two_corners()
 {
    let straight = |outline: Outline| {
        let contour = outline.contour().expect("an outline of runs and arcs");
        assert_eq!(contour.corners.len(), contour.runs.len());
        contour
            .runs
            .iter()
            .filter(|run| matches!(run, Run::Straight))
            .count()
    };
    assert_eq!(straight(Outline::rounded([0.0, 0.0], [10.0, 6.0], 2.0)), 4);
    assert_eq!(straight(Outline::rounded([0.0, 0.0], [10.0, 6.0], 3.0)), 2);
    assert_eq!(straight(Outline::rounded([0.0, 0.0], [6.0, 6.0], 3.0)), 0);
    assert_eq!(straight(Outline::slot([0.0, 0.0], [0.0, 4.0], 1.0)), 2);
}

#[test]
fn a_rounded_rectangle_is_its_rectangle_less_four_corners_a_circle_never_filled() {
    let block = Leaf::prism(
        Plane::xz(1.0),
        Outline::rounded([0.0, 0.0], [10.0, 6.0], 2.0),
        -3.0,
    );
    let volume = Exact.raised(&block).expect("a rounded block").volume();
    let promised = 3.0 * (60.0 - (4.0 - std::f64::consts::PI) * 4.0);
    assert!((volume - promised).abs() < 1e-9, "{volume}");
}

#[test]
fn a_circle_is_handed_to_the_exact_kernel_as_one_whole_turn_from_where_its_flats_start() {
    let circle = random_solids::whole_circle(DVec2::new(3.0, -1.0), 2.0, 90.0);
    let [corner] = circle.corners.as_slice() else {
        panic!("one corner: {circle:?}");
    };
    assert!(
        (*corner - DVec2::new(3.0, 1.0)).length() < 1e-12,
        "{corner}"
    );
    assert!(matches!(
        circle.runs.as_slice(),
        [Run::Round { center, turn }] if *center == DVec2::new(3.0, -1.0) && *turn == std::f64::consts::TAU
    ));
}

#[test]
#[ignore = "the exact kernel raises nothing yet"]
fn a_block_bored_and_given_a_boss_by_the_exact_kernel_keeps_every_rule() {
    random_solids::holds_exactly(&bored_and_bossed());
}

#[test]
#[ignore = "the exact kernel raises nothing yet"]
fn a_block_bored_across_and_given_a_boss_on_its_side_by_the_exact_kernel_keeps_every_rule() {
    random_solids::holds_exactly(&bored_across());
}

/// How many lines the campaign held its cases along, and how many it left
/// out for grazing a curved wall.
static HELD: AtomicUsize = AtomicUsize::new(0);
static GRAZING: AtomicUsize = AtomicUsize::new(0);

fn exactly(case: &Case) -> Result<(), Flaw> {
    let measured = random_solids::held_to_arithmetic(case, &Exact)?;
    HELD.fetch_add(measured.held, Ordering::Relaxed);
    GRAZING.fetch_add(measured.grazing, Ordering::Relaxed);
    Ok(())
}

/// The case a seed stands for, with the seed written where a campaign that
/// ends the program — a stack blown by a kernel — still leaves it to be read.
fn drawn(seed: u64) -> Case {
    eprint!("\rseed {seed} ");
    Case::drawn_square(seed)
}

/// The same, among the profiles.
fn drawn_profile(seed: u64) -> Case {
    eprint!("\rseed {seed} ");
    Case::drawn_profiles(seed)
}

fn from_the_environment(name: &str) -> Option<u64> {
    std::env::var(name).ok()?.parse().ok()
}

#[test]
#[ignore = "a campaign, run by hand: see the head of this file"]
fn a_campaign_of_random_square_solids_on_the_exact_kernel_keeps_every_rule() {
    campaign_over("square solids", drawn);
}

#[test]
#[ignore = "a campaign, run by hand: see the head of this file"]
fn a_campaign_of_random_profiles_on_the_exact_kernel_keeps_every_rule() {
    campaign_over("profiles", drawn_profile);
}

/// A campaign on the exact kernel over the cases `draw` gives, for as long as
/// the environment says, and its report.
fn campaign_over(what: &str, draw: fn(u64) -> Case) {
    let seconds = from_the_environment("CAO_FUZZ_SECONDS").unwrap_or(60);
    let first = from_the_environment("CAO_FUZZ_SEED").unwrap_or_else(|| {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_or(1, |since| since.as_nanos() as u64);
        Random::seeded(now).number() >> 16
    });
    let patience = Duration::from_secs(from_the_environment("CAO_FUZZ_PATIENCE").unwrap_or(30));
    let deadline = Instant::now() + Duration::from_secs(seconds);
    println!("campaign of {what} on the exact kernel from seed {first}, for {seconds} s");

    let check: Check<Case> = Arc::new(exactly);
    let quiet = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let report = campaign(first.., draw, check, Case::smaller, patience, || {
        Instant::now() < deadline
    });
    std::panic::set_hook(quiet);

    print_report(&report);
    assert!(
        report.findings.is_empty(),
        "{} cases of {} broke a rule",
        report.broken.iter().map(|(_, count)| count).sum::<usize>(),
        report.tried
    );
}

/// Each seed named in `CAO_TRIAGE_SEEDS`, commas between them, run again,
/// shrunk while it still breaks the same rule, and printed as a test: what a
/// campaign's failures are sorted into distinct ones from. The seeds name
/// square cases, or profiles when `CAO_TRIAGE_DRAW` is `profiles`.
#[test]
#[ignore = "run by hand on the seeds a campaign named"]
fn the_seeds_a_campaign_named_are_shrunk_one_by_one() {
    let seeds: Vec<u64> = std::env::var("CAO_TRIAGE_SEEDS")
        .unwrap_or_default()
        .split(',')
        .filter_map(|seed| seed.trim().parse().ok())
        .collect();
    let draw: fn(u64) -> Case = match std::env::var("CAO_TRIAGE_DRAW").as_deref() {
        Ok("profiles") => Case::drawn_profiles,
        _ => Case::drawn_square,
    };
    let patience = Duration::from_secs(from_the_environment("CAO_FUZZ_PATIENCE").unwrap_or(30));
    let check: Check<Case> = Arc::new(exactly);
    let quiet = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    for seed in seeds {
        let drawn = draw(seed);
        let Err(flaw) = answer(drawn.clone(), &check, patience) else {
            println!("\n── holds, seed {seed} ──");
            continue;
        };
        let mut last = flaw.clone();
        let shrunk = shrink(
            drawn.clone(),
            Case::smaller,
            |candidate| match answer(candidate.clone(), &check, patience) {
                Err(broken) if broken.is_like(&flaw) => {
                    last = broken;
                    true
                }
                _ => false,
            },
            || true,
        );
        println!(
            "\n── {:?}, seed {seed} ──\nas drawn: {flaw:?}\nshrunk:   {last:?}\n{}",
            flaw.rule(),
            shrunk.to_string().replace('\n', "\n    "),
        );
    }
    std::panic::set_hook(quiet);
}

fn print_report(report: &Report<Case>) {
    println!("{} cases tried", report.tried);
    println!(
        "{} lines held, {} left out for grazing a curved wall",
        HELD.load(Ordering::Relaxed),
        GRAZING.load(Ordering::Relaxed)
    );
    for (rule, count) in &report.broken {
        println!("  {rule:?}: broken by {count}");
    }
    for (seed, rule) in &report.failed {
        println!("failed {seed} {rule:?}");
    }
    for finding in &report.findings {
        println!(
            "\n── {:?}, seed {} ──\nas drawn: {:?}\nshrunk:   {:?}\n\n#[test]\nfn seed_{}_keeps_every_rule_on_the_exact_kernel() {{\n    random_solids::holds_exactly(&{});\n}}",
            finding.flaw.rule(),
            finding.seed,
            finding.flaw,
            finding.shrunk_flaw,
            finding.seed,
            finding.shrunk.to_string().replace('\n', "\n    "),
        );
    }
}
