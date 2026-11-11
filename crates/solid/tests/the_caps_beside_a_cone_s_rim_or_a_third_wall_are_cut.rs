//! A plane cap between a cone's rim and another wall's rim is cut into
//! triangles, so the step is drawn through the body and closed on the exact
//! kernel (#861, after #858, part of #497). The third wall's caps are #866's.
//!
//! Closes #861.
//! - One shrunk case of each sorted shape is pinned and holds on both
//!   kernels; each is red before —
//!   `seed_352000000002235_a_cone_s_cap_an_elliptic_shaft_of_its_radius_nearly_shares_keeps_every_rule`,
//!   `seed_352000000002235_the_same_cone_and_shaft_keep_every_rule_through_the_application_s_body`,
//!   `seed_352000000001257_a_frustum_s_caps_a_round_hole_leaning_3_degrees_crosses_keep_every_rule`,
//!   `seed_352000000001257_the_same_frustum_and_hole_keep_every_rule_through_the_application_s_body`,
//!   and beside the code, in `crates/solid/src/brep/tessellation/tests/cones.rs`,
//!   `a_frustum_s_cap_an_elliptic_shaft_of_its_radius_as_major_half_axis_nearly_shares_is_cut_and_drawn_closed`,
//!   `a_frustum_s_cap_an_elliptic_shaft_of_its_radius_as_minor_half_axis_nearly_shares_is_cut_and_drawn_closed`
//! - No draw regresses against `main`, seed by seed, in the walls sets 352,
//!   353, 354 and the turned set; the measurement is recorded in
//!   `docs/exact-kernel-failures.md`, with how many of the 26 now hold — no
//!   test: a campaign measurement and a document; the walls sets
//!   352000000000000, 353000000000000 and 354000000000000 on both kernels and
//!   the turned set, run on `main` and on the branch and compared seed by
//!   seed
//!
//! The 26 of #858's 59 left on `main` (59f9e801), replayed through the body
//! and sorted by the face the drawn case leaves uncut. #861 answers the
//! first shape, the cone's cap; the other two went to #866.
//!
//! A cone's plane cap, twelve: its rim beside an elliptic shaft whose axis
//! runs within a hair of the cone's, of the radius of one of its half-axes
//! (352000000001305, 352000000002235, 353000000003224, 353000000005523,
//! 353000000016799, 353000000017454, 354000000003072, 354000000006518,
//! 354000000016139), or beside a round hole leaning 3° to 15° across it
//! (352000000001257, 354000000002316, 354000000010845). Both shrunk cases
//! here hold with a cylinder in the cone's place. The miniatures put a
//! frustum's cap beside an elliptic shaft whose larger or smaller half-axis
//! is the cap's radius: the rim, sampled alone on the cone's grid, had a
//! chord sag across the elliptic rim beside the tip.
//!
//! A pair of walls a hair off parallel with a third wall across them, five
//! with a plane cap uncut, to #866: #858's elliptic shaft and leaning round
//! (352000000009685, which holds without its cross bore), two rounds
//! (352000000015008, 353000000011281, 354000000011413), and a round shaft
//! and an elliptic hole of its radius (353000000007467). Two are pinned
//! here and ignored until #866 answers them:
//! `seed_352000000009685_an_elliptic_shaft_a_round_hole_leaning_5e_2_degrees_and_a_cross_bore_keep_every_rule`,
//! `seed_352000000009685_the_same_shaft_and_holes_keep_every_rule_through_the_application_s_body`,
//! `seed_352000000015008_a_round_shaft_a_hole_inside_its_wall_leaning_1e_2_degrees_and_a_slanted_wall_keep_every_rule`
//! and
//! `seed_352000000015008_the_same_shaft_hole_and_wall_keep_every_rule_through_the_application_s_body`.
//!
//! Nine leave a curved wall uncut rather than a cap, to #866 too, and are
//! not pinned: 352000000007451 and 353000000004193 (#858's pair with a
//! third wall), 354000000007244, 353000000003675, 352000000016386,
//! 353000000017533, 354000000009673, 354000000013646 and 354000000013724.

// The drawing, the promise and the checks are the campaigns'; this file
// only builds cases and holds them.
#[allow(dead_code, unused_imports)]
mod random_solids;

use random_solids::{Axis, Case, Leaf, Outline, Plane, Section, Step};

/// A cone about Y through x 1.5, z 2, from its apex at y -3 to a cap of
/// radius 2 at y 2, joined by an elliptic shaft along it of half-axes 2
/// along X and 1.5, 1.8e-5 off its axis along X: the cap is two lunes
/// between the cone's rim and the ellipse, tipped where the two touch
/// (seed 352000000002235, shrunk).
fn a_cone_s_cap_an_elliptic_shaft_of_its_radius_nearly_shares() -> Case {
    Case::new(
        Leaf::turned(
            Plane::xy(2.0),
            Axis::second(1.5),
            Section::bands(-3.0, &[[5.0, 0.0, 0.0]]).sloping_to(&[[0.0, 2.0]]),
            360.0,
        ),
        vec![Step::add(Leaf::prism(
            Plane::tilted(
                [1.5000183192581993, 12.361980163677913, 1.9999999999999984],
                [90.0, 0.0, 0.0],
            ),
            Outline::ellipse([-0.0, -0.0], [-2.0, 0.0], 1.5),
            21.0,
        ))],
    )
}

#[test]
fn seed_352000000002235_a_cone_s_cap_an_elliptic_shaft_of_its_radius_nearly_shares_keeps_every_rule()
 {
    random_solids::holds_exactly(&a_cone_s_cap_an_elliptic_shaft_of_its_radius_nearly_shares());
}

#[test]
fn seed_352000000002235_the_same_cone_and_shaft_keep_every_rule_through_the_application_s_body() {
    random_solids::holds_through_the_application(
        &a_cone_s_cap_an_elliptic_shaft_of_its_radius_nearly_shares(),
    );
}

/// A frustum about Z through x -1, y 2, of radius 4 at z 0 narrowing to 3.5
/// at z 6.5, cut by a round hole of radius 3.75 leaning 2.8° about X: the
/// hole's rim crosses each cap's rim (seed 352000000001257, shrunk).
fn a_frustum_s_caps_a_round_hole_leaning_3_degrees_crosses() -> Case {
    Case::new(
        Leaf::turned(
            Plane::xz(2.0),
            Axis::second(-1.0),
            Section::bands(0.0, &[[6.5, 0.0, 4.0]]).sloping_to(&[[0.0, 3.5]]),
            360.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::tilted(
                [-1.0, 2.874250616226533, -14.526503457982859],
                [2.815544941143946, 0.0, 0.0],
            ),
            Outline::circle([0.2040656281809984, 0.0], 3.75001),
            35.5,
        ))],
    )
}

#[test]
fn seed_352000000001257_a_frustum_s_caps_a_round_hole_leaning_3_degrees_crosses_keep_every_rule() {
    random_solids::holds_exactly(&a_frustum_s_caps_a_round_hole_leaning_3_degrees_crosses());
}

#[test]
fn seed_352000000001257_the_same_frustum_and_hole_keep_every_rule_through_the_application_s_body() {
    random_solids::holds_through_the_application(
        &a_frustum_s_caps_a_round_hole_leaning_3_degrees_crosses(),
    );
}

/// An elliptic shaft from z 0 to 9 of half-axes 3.5 along X and 5, bored by
/// a round hole of radius 3.5 leaning 0.05° about X, and across both by a
/// round hole of radius 1 leaning 45° about Y (seed 352000000009685,
/// shrunk). Without the cross bore the pair holds.
fn an_elliptic_shaft_a_round_hole_leaning_5e_2_degrees_and_a_cross_bore() -> Case {
    Case::new(
        Leaf::prism(
            Plane::xy(0.0),
            Outline::ellipse([0.0, -2.0], [-3.5, 0.0], 5.0),
            9.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::tilted(
                    [0.0, -1.983092583357614, -15.124467403390838],
                    [0.05, 0.0, 0.0],
                ),
                Outline::circle([0.0, 0.0], 3.5),
                39.0,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted([18.0, -2.0, -6.0], [0.0, -45.0, 0.0]),
                Outline::circle([0.0, 0.0], 1.0),
                42.0,
            )),
        ],
    )
}

#[test]
#[ignore = "a plane cap beside a third wall, #866"]
fn seed_352000000009685_an_elliptic_shaft_a_round_hole_leaning_5e_2_degrees_and_a_cross_bore_keep_every_rule()
 {
    random_solids::holds_exactly(
        &an_elliptic_shaft_a_round_hole_leaning_5e_2_degrees_and_a_cross_bore(),
    );
}

#[test]
#[ignore = "a plane cap beside a third wall, #866"]
fn seed_352000000009685_the_same_shaft_and_holes_keep_every_rule_through_the_application_s_body() {
    random_solids::holds_through_the_application(
        &an_elliptic_shaft_a_round_hole_leaning_5e_2_degrees_and_a_cross_bore(),
    );
}

/// A round shaft of radius 25 about Z through y -7.5, from z -5 to 33,
/// bored by a round hole of radius 2.5 whose wall runs 1e-5 inside the
/// shaft's and leans 0.01° off it, and cut by a round wall of radius 25
/// leaning 15° about Y (seed 352000000015008, shrunk).
fn a_round_shaft_a_hole_inside_its_wall_leaning_1e_2_degrees_and_a_slanted_wall() -> Case {
    Case::new(
        Leaf::prism(Plane::xy(-5.0), Outline::circle([0.0, -7.5], 25.0), 38.0),
        vec![
            Step::cut(Leaf::turned(
                Plane::yz(22.49999),
                Axis::second(-7.5).leaning(-0.01),
                Section::bands(-96.5, &[[220.0, 0.0, 2.5]]),
                360.0,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted([30.0, -8.0, -92.0], [0.0, -15.0, 0.0]),
                Outline::circle([-0.0, -0.0], 25.0),
                220.0,
            )),
        ],
    )
}

#[test]
#[ignore = "a plane cap beside a third wall, #866"]
fn seed_352000000015008_a_round_shaft_a_hole_inside_its_wall_leaning_1e_2_degrees_and_a_slanted_wall_keep_every_rule()
 {
    random_solids::holds_exactly(
        &a_round_shaft_a_hole_inside_its_wall_leaning_1e_2_degrees_and_a_slanted_wall(),
    );
}

#[test]
#[ignore = "a plane cap beside a third wall, #866"]
fn seed_352000000015008_the_same_shaft_hole_and_wall_keep_every_rule_through_the_application_s_body()
 {
    random_solids::holds_through_the_application(
        &a_round_shaft_a_hole_inside_its_wall_leaning_1e_2_degrees_and_a_slanted_wall(),
    );
}
