//! Undoing folds does not stop while a fold it can still undo remains,
//! whatever other faces of the body count (#832, part of #755).
//!
//! `folds::unfolded` passes over every face again while a pass takes a flip
//! and meets fewer folds than the one before. The count is the body's, not a
//! face's: a fold on one face kept the passes going for another, and its
//! absence stopped them early. 354000000008162 held on `main` by that
//! accident — its passes counted 4, 3, 1, 0, the first holding the triangle
//! #820 turns right side up. With #820 they count 3, 3, and the loop stops
//! with a triangle of the leaning cone (face 2) still folded through the
//! bore's cylinder near (-0.73, -1.87, -0.71), its fan undone one flip per
//! pass and one pass short.
//!
//! Closes #832.
//! - 354000000008162's shrunk case is pinned and holds through the body on
//!   top of #820 —
//!   `seed_354000000008162_a_cone_cut_by_a_cone_leaning_18_degrees_then_bored_at_35_degrees_keeps_every_rule_through_the_application_s_body`
//! - No draw regresses against `main` in the walls sets 352, 353, 354 and the
//!   turned set, seed by seed — no test: a campaign measurement, run on
//!   `main` and on the branch and compared seed by seed, recorded in
//!   `docs/exact-kernel-failures.md`

#[allow(dead_code, unused_imports)]
mod random_solids;

use random_solids::{Axis, Case, Leaf, Outline, Plane, Section, Step};

#[test]
fn seed_354000000008162_a_cone_cut_by_a_cone_leaning_18_degrees_then_bored_at_35_degrees_keeps_every_rule_through_the_application_s_body()
 {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(0.0),
            Axis::second(0.5),
            Section::bands(-4.0, &[[10.0, 0.0, 1.0]]).sloping_to(&[[0.0, 3.0]]),
            360.0,
        ),
        vec![
            Step::cut(Leaf::turned(
                Plane::yz(1.0646795621535396),
                Axis::first(-1.0).leaning(-18.02429181144154),
                Section::bands(-18.5, &[[30.5, 0.0, 1.0]]).sloping_to(&[[0.0, 2.5]]),
                360.0,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted(
                    [0.5, 8.623020548565576, -12.962428358201974],
                    [35.0, 0.0, 0.0],
                ),
                Outline::circle([0.0, 0.0], 2.0),
                32.0,
            )),
        ],
    ));
}
