//! Closes #633.
//! - the shape's first leaf, its axis leaning 0.1° and 1e-5°, is raised by
//!   the exact kernel and keeps every rule —
//!   `a_grooved_ring_about_an_axis_leaning_a_tenth_of_a_degree_or_1e_5_is_raised_exactly`
//! - #632's whole case, its axis leaning 0.1°, keeps every rule through the
//!   application's body, in about the time it takes with a straight axis —
//!   `seed_5577103202_leaning_a_tenth_of_a_degree_keeps_every_rule_through_the_body_as_fast_as_straight`
//! - the cause is written in `docs/exact-kernel-failures.md` — no test: it is
//!   prose, held by `language.rs` only
//! - the rounded draw measured through the body before and after, no case
//!   built right becoming wrong — no test: measured, base against tip, in
//!   `docs/exact-kernel-failures.md`
//!
//! A ring with a groove whose half round bottom is tangent to its straight
//! walls, turned about an axis leaning a fraction of a degree: the reading
//! lands the groove's end on its circle along the wall, and the wall is
//! tangent there, so the root it takes is of nought up to the rounding the
//! lean leaves. Where that came out an ulp below nought the wall missed the
//! circle and the reading declined the section — at 1e-3°, 0.1°, 1° and 5°,
//! not at 1e-5°, 0.01° or 15°, as the bits fell — and where it came out an
//! ulp above, the end was laid the root of it, some 1e-6, off its level.
//!
//! Laid, the ring is no longer coaxial with the case's second leaf, turned
//! about a line parallel to the sketch's own axis: about a leaning axis the
//! two meet along cylinders at a skew angle, which the exact kernel declined
//! as unsupported until #577 built them; at 0.1° the step is now built, as
//! about the straight axis, and the next one refused, and at 1e-5° — a
//! sliver past decision 11, where the walls of one size meet — it is
//! refused itself, as on main.

// The drawing, the promise and the checks are the campaigns'; this file
// only builds cases and holds them.
#[allow(dead_code, unused_imports)]
mod random_solids;

use std::time::Instant;

use cao_solid::Declined;
use random_solids::{Application, Axis, Case, Exact, Kernel, Leaf, Outline, Plane, Section, Step};

/// Seed 5577103202's first leaf, turned about `axis`: a ring from 30 to 90
/// off it, 105 long, a groove cut into its outer side between 15 and 75
/// from its end, its walls 15 deep and its bottom a half round of 30
/// tangent to them.
fn grooved_ring(axis: Axis) -> Leaf {
    Leaf::turned(
        Plane::xy(90.0),
        axis,
        Section::bands(
            -90.0,
            &[[15.0, 30.0, 90.0], [60.0, 30.0, 75.0], [30.0, 30.0, 90.0]],
        )
        .bowed(&[
            [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0]],
            [[0.0, 0.0, 0.0], [-45.0, 75.0, -30.0]],
            [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0]],
        ]),
        -360.0,
    )
}

/// Seed 5577103202's case, its first leaf turned about `axis`, then five
/// steps. About an axis leaning 1e-5° and a hair off sideways, the reading
/// declined the groove; the flats turned it, and every step after it was
/// their boolean — two minutes for the fifth, twenty-two for the sixth, on
/// the window's thread — until #632 refused it at its first leaf, and #633
/// laid it.
fn seed_5577103202(axis: Axis) -> Case {
    Case::new(
        grooved_ring(axis),
        vec![
            Step::add(Leaf::turned(
                Plane::xy(240.0),
                Axis::second(-90.0),
                Section::bands(
                    90.0,
                    &[
                        [30.0, 0.0, 90.0],
                        [30.0, 0.0, 90.0],
                        [15.0, 0.0, 90.0],
                        [60.0, 0.0, 120.0],
                        [30.0, 0.0, 105.0],
                    ],
                )
                .bowed(&[
                    [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0]],
                    [[0.0, 0.0, 0.0], [135.0, 90.0, -15.0]],
                    [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0]],
                    [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0]],
                    [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0]],
                ]),
                360.0,
            )),
            Step::add(Leaf::turned(
                Plane::xy(0.0),
                Axis::second(75.0),
                Section::bands(-120.0, &[[180.0, -0.0, -0.0]])
                    .bowed(&[[[-30.0, -0.0, -90.0], [0.0, -0.0, -0.0]]]),
                360.0,
            )),
            Step::cut(Leaf::turned(
                Plane::yz(210.0),
                Axis::first(180.0),
                Section::bands(
                    -30.0,
                    &[
                        [60.0, 120.0, 225.0],
                        [15.0, 120.0, 285.0],
                        [45.0, 210.0, 285.0],
                    ],
                )
                .sloping_to(&[[120.0, 285.0], [120.0, 285.0], [210.0, 285.0]]),
                -0.01,
            )),
            Step::add(Leaf::prism(
                Plane::xz(-30.0),
                Outline::ring([210.0, 180.0], 285.0, 225.0),
                -90.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(15.0),
                Outline::rectangle([120.0, 120.0], [240.0, 240.0]),
                210.0,
            )),
        ],
    )
}

/// The axis seed 5577103202 drew: the sketch's second, run backwards, a
/// hair off it sideways and leaning 1e-5°.
fn as_drawn() -> Axis {
    Axis::second(-3.0000000000000004e-7)
        .backwards()
        .leaning(-1e-5)
}

#[test]
fn a_grooved_ring_about_an_axis_leaning_a_tenth_of_a_degree_or_1e_5_is_raised_exactly() {
    let leaning = |lean: f64| Axis::second(0.0).backwards().leaning(lean);
    for axis in [leaning(0.1), leaning(1e-5), as_drawn()] {
        let ring = grooved_ring(axis);
        assert!(
            Exact.raised(&ring).is_ok(),
            "{axis:?}: raised by the exact kernel"
        );
        let alone = Case::new(ring, Vec::new());
        let measured = random_solids::held_to_arithmetic(&alone, &Exact)
            .unwrap_or_else(|flaw| panic!("{axis:?}: {:?} broke the rule: {flaw:?}", flaw.rule()));
        assert_eq!(measured.bowed, 0, "{axis:?}: laid, not excused");
        random_solids::holds_through_the_application(&alone);
    }
}

/// The step of `case` the application's body declines, and why, its leaves
/// combined one by one.
fn declined_at(case: &Case) -> Option<(usize, Declined)> {
    let mut body = Application.raised(&case.start).expect("the first leaf");
    for (index, step) in case.steps.iter().enumerate() {
        let tool = Application.raised(&step.tool).expect("a tool");
        match Application.combined(&body, &tool, step.mode) {
            Ok(next) => body = next,
            Err(declined) => return Some((index, declined)),
        }
    }
    None
}

#[test]
fn seed_5577103202_leaning_a_tenth_of_a_degree_keeps_every_rule_through_the_body_as_fast_as_straight()
 {
    let timed = |axis: Axis| {
        let case = seed_5577103202(axis);
        assert!(
            Application
                .raised(&case.start)
                .is_ok_and(|body| body.is_exact()),
            "{axis:?}: the first leaf raised exactly",
        );
        let started = Instant::now();
        let measured = random_solids::held_to_arithmetic(&case, &Application)
            .unwrap_or_else(|flaw| panic!("{axis:?}: {:?} broke the rule: {flaw:?}", flaw.rule()));
        assert_eq!(
            (measured.bowed, measured.thin, measured.declined),
            (0, 0, 1),
            "{axis:?}: laid, and one step declined as unsupported",
        );
        (started.elapsed().as_secs_f64(), declined_at(&case))
    };
    let (straight, refused) = timed(Axis::second(0.0).backwards());
    assert_eq!(
        refused,
        Some((1, Declined::Unsupported)),
        "about the straight axis"
    );
    for (axis, step) in [
        (Axis::second(0.0).backwards().leaning(0.1), 1),
        (as_drawn(), 0),
    ] {
        let (leaning, refused) = timed(axis);
        assert_eq!(
            refused,
            Some((step, Declined::Unsupported)),
            "{axis:?}: the second leaf skew to the ring built at 0.1°, the next refused; \
             at 1e-5°, a sliver past decision 11, the second refused (#577)"
        );
        assert!(
            leaning < 2.0 * straight + 1.0,
            "{axis:?}: {leaning} s, where the straight axis takes {straight} s",
        );
    }
}
