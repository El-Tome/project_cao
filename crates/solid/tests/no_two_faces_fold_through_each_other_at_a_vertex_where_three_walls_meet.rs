//! Two faces meeting at a vertex where three walls meet never fold through
//! each other, whatever a chord near it carries (#821, part of #755).
//!
//! A meet ending at a vertex where a third wall's cylinder stands leaves that
//! cylinder there and stays outside it, but a chord of it can pass inside,
//! deeper than the cylinder's own triangles sag. The chord bounds the faces
//! either side of it, so a triangle of one crosses the cylinder's, and no cut
//! of either face undoes it: the meet is sampled until its chords stay outside
//! wherever it does.
//!
//! In 354000000011719 a cone, a cone leaning 27 degrees joined to it and a
//! bore at 40 degrees meet at (2.75, 3.5, 1.0). The meet of the two cones
//! once held there only because a sample its rulings read wrongly split the
//! chord; read rightly (#811), the base cone's triangle on that chord crossed
//! the bore's.
//!
//! The pass is held beside its code, in
//! `crates/solid/src/brep/tessellation/sampling/meet/outside/tests.rs`, and
//! the seam of a ruled meet in `crates/solid/src/brep/meet/tests/ruled.rs`.
//! The mutants those tests leave alive are equivalent:
//! - `outside.rs` `122:16` and `123:22` `>` → `>=` in `dips`, `189:89` and
//!   `204:68` `>` → `>=` in `kept_outside`: each differs only where a depth
//!   or a distance equals the kernel's tolerance, the sag or the tolerance
//!   asked to the last bit, which no probe of a chord between rounded points
//!   lands on.
//! - `outside.rs` `149:62` `+` → `-` in `sag_at`: the first angle a turn
//!   back rather than a turn on moves the half step and the cell's middle by
//!   a whole turn each, which neither cosine sees.
//! - `ruled/seen.rs` `200:38` and `200:53` `<` → `<=` in `Ruled::at_angle`:
//!   an end of a window standing exactly at the angle is taken by the
//!   `== 0.0` arms above, so the sign test never meets a nought.
//!
//! Four mutants of `outside.rs` `189`, where the curve is read at the share
//! a chord dips at — `189:65` `+` → `*`, `189:71` `-` → `+` and `-` → `/`,
//! `189:79` `*` → `+` — time out rather than fail: read elsewhere, the curve
//! of a chord running inside the wall stands outside, and every half of it
//! is halved again, forty levels down.
//!
//! Closes #821.
//! - 354000000011719's shrunk case is pinned and holds through the body with
//!   #811's guard applied —
//!   `seed_354000000011719_a_cone_joined_to_a_cone_leaning_27_degrees_then_bored_at_40_degrees_keeps_every_rule_through_the_application_s_body`
//! - No draw regresses against `main`, seed by seed — the draws a first
//!   version of the pass broke are pinned:
//!   `seed_352000000016261_an_elliptic_bar_cut_by_an_elliptic_bar_and_two_round_ones_keeps_every_rule_through_the_application_s_body`,
//!   `seed_352000000000339_an_elliptic_bar_joined_to_a_round_one_leaning_80_degrees_then_cut_by_a_round_and_an_elliptic_one_at_30_degrees_keeps_every_rule`,
//!   `seed_353000000000880_a_cone_bored_at_47_degrees_then_joined_to_a_bar_at_31_degrees_keeps_every_rule`,
//!   `seed_354000000007974_a_round_bar_cut_by_two_bores_of_its_radius_at_46_and_42_degrees_keeps_every_rule_through_the_application_s_body`,
//!   `seed_354000000009148_an_elliptic_bar_cut_by_two_bores_crossing_square_one_6e_8_off_keeps_every_rule_through_the_application_s_body`;
//!   the rest is a campaign measurement, the walls sets on both kernels and
//!   the turned set run on `main` and on the branch and compared seed by
//!   seed, recorded in `docs/exact-kernel-failures.md`
//!
//! Seed 352000000000339 is pinned as drawn, not shrunk: its shrunk case
//! crosses on `main` too, the same with the pass or without it.
//!
//! Closes #840.
//! - The two rules of `outside.rs` are one, and the five draws pinned under
//!   #821 and 354000000011719 still hold —
//!   `seed_354000000011719_a_cone_joined_to_a_cone_leaning_27_degrees_then_bored_at_40_degrees_keeps_every_rule_through_the_application_s_body`,
//!   `seed_352000000016261_an_elliptic_bar_cut_by_an_elliptic_bar_and_two_round_ones_keeps_every_rule_through_the_application_s_body`,
//!   `seed_352000000000339_an_elliptic_bar_joined_to_a_round_one_leaning_80_degrees_then_cut_by_a_round_and_an_elliptic_one_at_30_degrees_keeps_every_rule`,
//!   `seed_353000000000880_a_cone_bored_at_47_degrees_then_joined_to_a_bar_at_31_degrees_keeps_every_rule`,
//!   `seed_354000000007974_a_round_bar_cut_by_two_bores_of_its_radius_at_46_and_42_degrees_keeps_every_rule_through_the_application_s_body`,
//!   `seed_354000000009148_an_elliptic_bar_cut_by_two_bores_crossing_square_one_6e_8_off_keeps_every_rule_through_the_application_s_body`;
//!   the one rule is held beside its code, in
//!   `crates/solid/src/brep/tessellation/sampling/meet/outside/tests.rs`, by
//!   `a_chord_dipping_into_a_pierced_wall_no_deeper_than_its_triangles_sag_is_left_whole`,
//!   `a_chord_dipping_into_a_wall_deeper_than_its_triangles_sag_is_halved_only_until_it_dips_no_deeper_than_they_sag`,
//!   `a_chord_dipping_into_a_pierced_wall_deeper_than_its_triangles_sag_where_it_is_bored_away_is_left_whole`
//!   and
//!   `a_chord_dipping_into_a_wall_only_where_the_curve_itself_runs_inside_it_is_left_whole`
//! - A unit test pins the seam case of point 4, red before — no test: in this
//!   file; held by
//!   `at_an_angle_between_the_two_ends_of_its_table_a_ruled_meet_is_found_at_its_seam`,
//!   `at_an_angle_neither_end_step_of_its_table_holds_a_ruled_meet_is_found_at_its_seam`,
//!   `a_ruled_meet_found_a_hair_before_the_end_of_its_lap_is_kept_at_its_seam`,
//!   `a_ruled_meet_crossing_an_angle_at_its_seam_is_found_there_once`,
//!   `on_a_lap_winding_round_the_other_wall_a_ruled_meet_is_found_only_where_it_stands_at_the_angle`
//!   and
//!   `on_a_lap_winding_round_the_other_wall_a_ruled_meet_crossing_an_angle_at_its_seam_is_found_there_once`,
//!   in `crates/solid/src/brep/meet/tests/ruled.rs`
//! - No draw regresses against `main`, seed by seed, in the walls sets 352,
//!   353, 354 and the turned set; the measurement is recorded in
//!   `docs/exact-kernel-failures.md` — no test: a campaign measurement, the
//!   walls sets 352000000000000, 353000000000000 and 354000000000000 and the
//!   turned set run on `main` and on the branch and compared seed by seed,
//!   recorded in a document
//! - From the comment: a third wall that is a cone is covered by the one rule
//!   or named out of reach — no test: named out of reach; the pass sees only
//!   the body's contact walls, `contact::Wall`, which are cylinders, and a
//!   cone third wall would need a list of cone walls that does not exist

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

#[test]
fn seed_352000000016261_an_elliptic_bar_cut_by_an_elliptic_bar_and_two_round_ones_keeps_every_rule_through_the_application_s_body()
 {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::prism(
            Plane::xz(-10.0),
            Outline::ellipse([0.0, 10.0], [-12.5, 0.0], 20.0),
            -43.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::tilted([46.0, 92.0, 10.0], [90.0, -30.0, 0.0]),
                Outline::ellipse([0.0, 0.0], [-10.0, 0.0], 8.0),
                177.0,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted([0.0, -70.0, 10.0], [-90.0, 0.0, 0.0]),
                Outline::circle([-2.5, -0.0], 10.0),
                162.0,
            )),
            Step::cut(Leaf::turned(
                Plane::xy(10.0),
                Axis::first(31.0),
                Section::bands(-88.0, &[[177.0, 0.0, 10.0]]),
                360.0,
            )),
        ],
    ));
}

#[test]
fn seed_352000000000339_an_elliptic_bar_joined_to_a_round_one_leaning_80_degrees_then_cut_by_a_round_and_an_elliptic_one_at_30_degrees_keeps_every_rule()
 {
    random_solids::holds_exactly(&Case::drawn_walls_at_any_angle(352000000000339));
}

#[test]
fn seed_353000000000880_a_cone_bored_at_47_degrees_then_joined_to_a_bar_at_31_degrees_keeps_every_rule()
 {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xz(0.0),
            Axis::second(2.0),
            Section::bands(-1.0, &[[8.5, 0.0, 3.5]]).sloping_to(&[[0.0, 1.5]]),
            360.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::tilted(
                    [2.0, -12.180447424633714, -7.951456110284646],
                    [-47.39755190015385, 0.0, 0.0],
                ),
                Outline::circle([-0.0, -0.0], 2.5),
                33.0,
            )),
            Step::add(Leaf::prism(
                Plane::tilted(
                    [2.0, -8.522844063778471, -10.93439450661942],
                    [-31.0, 0.0, 0.0],
                ),
                Outline::circle([-0.9999999899999998, -0.0], 1.5),
                33.0,
            )),
        ],
    ));
}

#[test]
fn seed_354000000007974_a_round_bar_cut_by_two_bores_of_its_radius_at_46_and_42_degrees_keeps_every_rule_through_the_application_s_body()
 {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::prism(Plane::xy(-2.0), Outline::circle([2.0, -1.0], 3.0), 4.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::tilted(
                    [11.625364673707903, -1.0, -9.295106618824276],
                    [0.0, -46.0, 0.0],
                ),
                Outline::circle([-0.0, -0.0], 3.0),
                27.0,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted(
                    [10.953523908327025, -1.0, -9.943895704410382],
                    [0.0, -42.0, 0.0],
                ),
                Outline::circle([-0.0, -0.0], 3.0),
                27.0,
            )),
        ],
    ));
}

#[test]
fn seed_354000000009148_an_elliptic_bar_cut_by_two_bores_crossing_square_one_6e_8_off_keeps_every_rule_through_the_application_s_body()
 {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::prism(
            Plane::xz(-3.0),
            Outline::ellipse([0.0, 0.0], [0.0, -3.0], 5.0),
            -9.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::tilted(
                    [0.0, 16.158974015119114, -14.658974015119114],
                    [45.0, 0.0, 0.0],
                ),
                Outline::circle([-6e-8, -0.0], 5.0),
                41.0,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted(
                    [0.0, -13.158974015119115, -14.658974015119114],
                    [-45.0, 0.0, 0.0],
                ),
                Outline::circle([0.0, 0.0], 5.0),
                41.0,
            )),
        ],
    ));
}
