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
//!
//! The three draws the campaign gained against `main` are pinned as drawn,
//! on both kernels: 352000000000787 of the walls set, `Uncrossed` there, and
//! 533301186 and 533312139 of the turned set, `Closed` on the exact kernel
//! and `Answers` through the body there.
//!
//! The rule the fix added — a turn whose ray stands within a hair of the ray
//! through an end, no turn between, is no sample — is held beside its code,
//! in `crates/solid/src/brep/tessellation/sampling/tests.rs`:
//! `a_meet_is_not_sampled_where_it_turns_back_within_a_hair_of_the_ray_through_its_end`,
//! and `a_meet_is_sampled_where_it_turns_back_in_a_wall_s_angle_far_from_the_rays_through_its_ends`,
//! which kills the three mutants of `meet.rs` the campaign tests let live,
//! `106:17` and `108:25` `&&` → `||` and `109:71` `*` → `+`. No meet of two
//! walls makes its guard that no turn stands between false: a ray a hair
//! from a turn's crosses the other wall only beside that turn, so an end on
//! it reaches the turn past another only the long way round, and the edge
//! then either runs past its period or ends nearer the turn on the near
//! side, whose ray drops it. The mutant `120:58`, `*` → `/` or `+` on the
//! grid's angle, times out rather than surviving.

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

#[test]
fn seed_352000000000787_a_shaft_joined_to_a_post_tangent_to_it_leaning_a_hair_then_bored_across_twice_keeps_every_rule()
 {
    random_solids::holds_exactly(&Case::drawn_walls_at_any_angle(352000000000787));
}

#[test]
fn seed_352000000000787_the_same_shaft_post_and_bores_keep_every_rule_through_the_application_s_body()
 {
    random_solids::holds_through_the_application(&Case::drawn_walls_at_any_angle(352000000000787));
}

#[test]
fn seed_533301186_a_turn_of_60_degrees_joined_to_a_holed_turn_leaning_1e_3_degrees_and_a_turn_of_225_degrees_keeps_every_rule()
 {
    random_solids::holds_exactly(&Case::drawn_turned_off_the_lattice(533301186));
}

#[test]
fn seed_533301186_the_same_three_turns_keep_every_rule_through_the_application_s_body() {
    random_solids::holds_through_the_application(&Case::drawn_turned_off_the_lattice(533301186));
}

#[test]
fn seed_533312139_a_holed_turn_leaning_1e_5_degrees_cut_by_three_turns_keeps_every_rule() {
    random_solids::holds_exactly(&Case::drawn_turned_off_the_lattice(533312139));
}

#[test]
fn seed_533312139_the_same_holed_turn_and_cuts_keep_every_rule_through_the_application_s_body() {
    random_solids::holds_through_the_application(&Case::drawn_turned_off_the_lattice(533312139));
}
