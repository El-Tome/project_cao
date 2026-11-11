//! A block face a bore leaning a hair off its length touches is drawn without
//! a fold where the bore's meet with it leaves the block's edge (#855, part of
//! #755).
//!
//! Closes #855.
//! - The shrunk case of 354000000000132 holds on the exact kernel and through
//!   the application's body, pinned as tests —
//!   `seed_354000000000132_a_block_bored_by_a_cylinder_leaning_1e_3_degrees_tangent_to_two_faces_then_joined_to_a_post_keeps_every_rule`,
//!   `seed_354000000000132_the_same_bore_and_post_keep_every_rule_through_the_application_s_body`
//! - 352000000002266's shrunk case, which holds on `main` now (it was
//!   declined `Undrawn` when #755 was written), is pinned as tests that hold —
//!   `seed_352000000002266_a_shaft_cut_by_a_cone_across_then_bored_by_its_radius_1e_5_off_its_axis_keeps_every_rule`,
//!   `seed_352000000002266_the_same_cone_and_bore_keep_every_rule_through_the_application_s_body`
//! - No draw regresses against `main`, seed by seed, in the walls sets 352,
//!   353, 354 and the turned set; the measurement is recorded in
//!   `docs/exact-kernel-failures.md` — no test: a campaign measurement and a
//!   document; the walls sets 352000000000000, 353000000000000 and
//!   354000000000000 on both kernels and the turned set, run on `main` and on
//!   the branch and compared seed by seed

// The drawing, the promise and the checks are the campaigns'; this file
// only builds cases and holds them.
#[allow(dead_code, unused_imports)]
mod random_solids;

use random_solids::{Axis, Case, Leaf, Outline, Plane, Section, Step};

/// A block of side 10 bored through by a cylinder of radius 5 leaning
/// 0.001° off its length, touching two of its faces, then joined to a post
/// of radius 5 square to the bore: the block's face at z = 5 folded where
/// the bore's meet with it leaves the block's edge at y = -3 (seed
/// 354000000000132, shrunk; #755).
fn a_block_bored_by_a_cylinder_leaning_1e_3_degrees_tangent_to_two_faces_then_joined_to_a_post()
-> Case {
    Case::new(
        Leaf::prism(
            Plane::xz(-3.0),
            Outline::rectangle([-4.0, -5.0], [6.0, 5.0]),
            -4.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::tilted([1.0, 18.0, -0.0], [89.999, 0.0, 0.0]),
                Outline::circle([0.0, 0.0], 5.0),
                41.0,
            )),
            Step::add(Leaf::prism(
                Plane::tilted([-20.0, -3.0, -0.0], [0.0, 90.0, 0.0]),
                Outline::circle([-0.0, -0.0], 5.0),
                41.0,
            )),
        ],
    )
}

#[test]
fn seed_354000000000132_a_block_bored_by_a_cylinder_leaning_1e_3_degrees_tangent_to_two_faces_then_joined_to_a_post_keeps_every_rule()
 {
    random_solids::holds_exactly(
        &a_block_bored_by_a_cylinder_leaning_1e_3_degrees_tangent_to_two_faces_then_joined_to_a_post(
        ),
    );
}

#[test]
fn seed_354000000000132_the_same_bore_and_post_keep_every_rule_through_the_application_s_body() {
    random_solids::holds_through_the_application(
        &a_block_bored_by_a_cylinder_leaning_1e_3_degrees_tangent_to_two_faces_then_joined_to_a_post(
        ),
    );
}

/// A shaft of radius 3.5 along X cut across by a cone turned about Y, then
/// bored along it by a hole of its radius 1e-5 off its axis: declined
/// `Undrawn` when #755 was written (seed 352000000002266, shrunk).
fn a_shaft_cut_by_a_cone_across_then_bored_by_its_radius_1e_5_off_its_axis() -> Case {
    Case::new(
        Leaf::prism(Plane::yz(0.0), Outline::circle([-2.0, 1.0], 3.5), 9.0),
        vec![
            Step::cut(Leaf::turned(
                Plane::xy(-0.7007200544014198),
                Axis::second(0.0),
                Section::bands(-19.5, &[[35.0, 0.0, 2.5]]).sloping_to(&[[0.0, 0.5]]),
                360.0,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted([21.0, -2.0, 1.0], [0.0, -90.0, 0.0]),
                Outline::circle([0.0, 9.999999999999999e-6], 3.5),
                35.0,
            )),
        ],
    )
}

#[test]
fn seed_352000000002266_a_shaft_cut_by_a_cone_across_then_bored_by_its_radius_1e_5_off_its_axis_keeps_every_rule()
 {
    random_solids::holds_exactly(
        &a_shaft_cut_by_a_cone_across_then_bored_by_its_radius_1e_5_off_its_axis(),
    );
}

#[test]
fn seed_352000000002266_the_same_cone_and_bore_keep_every_rule_through_the_application_s_body() {
    random_solids::holds_through_the_application(
        &a_shaft_cut_by_a_cone_across_then_bored_by_its_radius_1e_5_off_its_axis(),
    );
}
