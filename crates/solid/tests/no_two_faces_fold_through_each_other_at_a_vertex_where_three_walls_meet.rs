//! Two faces meeting at a vertex where three walls meet never fold through
//! each other, whatever a chord near it carries (#821, part of #755).
//!
//! 354000000011719 holds on `main` and fails `Uncrossed` with #811's guard.
//! Near the vertex where the base cone, a cone leaning 27 degrees and a bore's
//! cylinder meet, at (2.75, 3.5, 1.0), a triangle of the base cone on the
//! chord of an edge from that vertex crosses a triangle of the bore's
//! cylinder. The chord sags under the tolerance, so dropping the spurious
//! sample the guard removes is legitimate; the sample only happened to split
//! the chord that crosses the other face.
//!
//! Closes #821.
//! - 354000000011719's shrunk case is pinned and holds through the body with
//!   #811's guard applied —
//!   `seed_354000000011719_a_cone_joined_to_a_cone_leaning_27_degrees_then_bored_at_40_degrees_keeps_every_rule_through_the_application_s_body`
//! - No draw regresses against `main`, seed by seed — no test: a campaign
//!   measurement, the walls sets and the turned set run on `main` and on the
//!   branch and compared seed by seed, recorded in
//!   `docs/exact-kernel-failures.md`

#[allow(dead_code, unused_imports)]
mod random_solids;

use random_solids::{Axis, Case, Leaf, Outline, Plane, Section, Step};

#[test]
fn seed_354000000011719_a_cone_joined_to_a_cone_leaning_27_degrees_then_bored_at_40_degrees_keeps_every_rule_through_the_application_s_body()
 {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::yz(-1.0),
            Axis::first(1.0),
            Section::bands(-1.0, &[[9.0, 0.0, 2.5]]).sloping_to(&[[0.0, 5.0]]),
            360.0,
        ),
        vec![
            Step::add(Leaf::turned(
                Plane::yz(0.75),
                Axis::first(2.7833390732305006).leaning(-27.0),
                Section::bands(-17.58491278251995, &[[43.02610922848042, 0.0, 1.5]])
                    .sloping_to(&[[0.0, 2.5]]),
                360.0,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted(
                    [-1.0, 17.32832495254344, -15.479955941753804],
                    [40.0, 0.0, 0.0],
                ),
                Outline::circle([3.24999994, 0.0], 0.5),
                43.0,
            )),
        ],
    ));
}
