//! The plane caps of an elliptic shaft that a round wall of its smaller
//! half-axis nearly runs along are cut into triangles, so the step is drawn
//! through the body and closed on the exact kernel (#858, part of #497).
//!
//! Closes #858.
//! - The shrunk case of the largest sub-shape is pinned and holds on both
//!   kernels; it is red before —
//!   `seed_354000000000100_an_elliptic_shaft_bored_by_a_round_hole_of_its_smaller_half_axis_leaning_1e_2_degrees_keeps_every_rule`,
//!   `seed_354000000000100_the_same_shaft_and_bore_keep_every_rule_through_the_application_s_body`
//! - No draw regresses against `main`, seed by seed, in the walls sets 352,
//!   353, 354 and the turned set; the measurement is recorded in
//!   `docs/exact-kernel-failures.md`, with how many of the 59 now hold — no
//!   test: a campaign measurement and a document; the walls sets
//!   352000000000000, 353000000000000 and 354000000000000 on both kernels and
//!   the turned set, run on `main` and on the branch and compared seed by
//!   seed
//!
//! The largest sub-shape of the 59 is an elliptic shaft and a round bore or
//! pin of the radius of its smaller half-axis on nearly the same axis: the
//! round rim pokes a hair out of the elliptic one beside each end of that
//! half-axis, and each cap is two crescents whose tips stand there.
//! 353000000000723, 354000000000100 and 352000000003907 shrink to it. The
//! others shrunk so far put the round rim on a cone's cap instead, the
//! ellipse's major or minor half-axis its radius (352000000001305,
//! 352000000002235, 353000000003224), or bore an ellipse into a cone's cap
//! through a slanted round hole (352000000001257), or set a small circle
//! inside an ellipse's major vertex (354000000000499). Every one is declined
//! `Undrawn` because a plane cap is left uncut, and is open on the exact
//! kernel.

// The drawing, the promise and the checks are the campaigns'; this file
// only builds cases and holds them.
#[allow(dead_code, unused_imports)]
mod random_solids;

use random_solids::{Case, Leaf, Outline, Plane, Step};

/// An elliptic shaft of half-axes 5 along X and 3.5, five long, bored along
/// its axis by a round hole of radius 3.5 leaning 0.01° about Y: at each cap
/// the round rim pokes out of the elliptic one beside both ends of its minor
/// axis, and the crescents between them were left uncut (seed
/// 354000000000100, shrunk).
fn an_elliptic_shaft_bored_by_a_round_hole_of_its_smaller_half_axis_leaning_1e_2_degrees() -> Case {
    Case::new(
        Leaf::prism(
            Plane::xy(-3.0),
            Outline::ellipse([-2.0, -2.0], [5.0, 0.0], 3.5),
            5.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::tilted([-2.0, -2.0, -20.0], [0.0, 0.01, 0.0]),
            Outline::circle([-0.0, -0.0], 3.5),
            35.0,
        ))],
    )
}

#[test]
fn seed_354000000000100_an_elliptic_shaft_bored_by_a_round_hole_of_its_smaller_half_axis_leaning_1e_2_degrees_keeps_every_rule()
 {
    random_solids::holds_exactly(
        &an_elliptic_shaft_bored_by_a_round_hole_of_its_smaller_half_axis_leaning_1e_2_degrees(),
    );
}

#[test]
fn seed_354000000000100_the_same_shaft_and_bore_keep_every_rule_through_the_application_s_body() {
    random_solids::holds_through_the_application(
        &an_elliptic_shaft_bored_by_a_round_hole_of_its_smaller_half_axis_leaning_1e_2_degrees(),
    );
}
