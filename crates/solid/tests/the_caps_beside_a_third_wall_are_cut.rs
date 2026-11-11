//! The plane caps beside a pair of walls a hair off parallel with a third
//! wall across them, sorted out of #858's 59 by #861: two shrunk cases on
//! both kernels, ignored while they wait on #866. The sort itself is in
//! `a_cone_s_cap_beside_an_elliptic_shaft_or_a_leaning_hole_is_cut.rs`.

// The drawing, the promise and the checks are the campaigns'; this file
// only builds cases and holds them.
#[allow(dead_code, unused_imports)]
mod random_solids;

use random_solids::{Axis, Case, Leaf, Outline, Plane, Section, Step};

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
