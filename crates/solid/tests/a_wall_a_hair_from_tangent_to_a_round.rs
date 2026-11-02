//! Closes #626.
//! - the four seeds of the issue hold on the exact kernel —
//!   `a_ball_cut_by_a_coaxial_bore_a_hair_narrower_keeps_every_rule`,
//!   `a_ball_cut_by_a_coaxial_band_a_hair_inside_its_equator_keeps_every_rule`,
//!   `a_ring_joined_to_a_disc_a_hair_below_its_crests_keeps_every_rule`,
//!   `a_rounded_cone_cut_by_itself_moved_a_hair_along_its_axis_keeps_every_rule`
//! - the rounded campaign's `Uncrossed` falls without one case built wrong
//!   that held or declined before — no test: measured, `main` against the
//!   branch on the same seeds of
//!   `a_campaign_of_rounded_solids_on_the_exact_kernel_keeps_every_rule`, in
//!   `docs/exact-kernel-failures.md`. Not met to the letter: `Uncrossed`
//!   falls, but five seeds held or declined on `main` break a rule here,
//!   four on a family `main` breaks on their shrunk case too and one the
//!   harness misjudges; every shrunk case of the decision's own is held
//!   below
//! - the rate of #576's draw measured again — no test: the same campaigns, in
//!   `docs/exact-kernel-failures.md`
//!
//! A coaxial surface standing a hair from tangent to a sphere or a torus: the
//! two cross along two circles a few ten-thousandths apart, real geometry but
//! thinner than any triangle laid between them, and the triangles of the two
//! faces crossed.

// The drawing, the promise and the checks are the campaigns'; this file
// only builds cases and holds them.
#[allow(dead_code, unused_imports)]
mod random_solids;

use random_solids::{Axis, Case, Leaf, Outline, Plane, Section, Step};

/// Seed 5576800624, lane C round 1 of #576: a ball of radius 2.5 cut by a
/// coaxial bore of radius 2.49999998 over half its height. Uncrossed.
#[test]
fn a_ball_cut_by_a_coaxial_bore_a_hair_narrower_keeps_every_rule() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(3.5),
            Axis::first(-3.0).backwards(),
            Section::bands(-4.5, &[[5.0, 0.0, 0.0]]).bowed(&[[[0.0, 0.0, 0.0], [-2.0, 0.0, 2.5]]]),
            360.0,
        ),
        vec![Step::cut(Leaf::turned(
            Plane::xz(-3.0),
            Axis::first(3.5),
            Section::bands(-0.5, &[[5.0, 0.0, 2.49999998]]),
            360.0,
        ))],
    ));
}

/// Seed 5576800706, lane C round 1 of #576: a ball of radius 5 cut by a
/// coaxial band of radius 4.9999999. Uncrossed.
#[test]
fn a_ball_cut_by_a_coaxial_band_a_hair_inside_its_equator_keeps_every_rule() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(2.5),
            Axis::second(25.0).backwards(),
            Section::bands(5.0, &[[30.0, 0.0, 0.0]])
                .bowed(&[[[0.0, 0.0, 0.0], [20.0, -20.0, 25.0]]]),
            360.0,
        ),
        vec![Step::cut(Leaf::turned(
            Plane::xy(2.5),
            Axis::second(25.0).backwards(),
            Section::bands(-7.5, &[[55.0, 0.0, 4.9999999]]),
            360.0,
        ))],
    ));
}

/// Seed 5576800899, lane C round 1 of #576: a ring whose crests stand at −2
/// and 0 joined to a coaxial disc from −2.00000001 to −0.00000001.
/// Uncrossed.
#[test]
fn a_ring_joined_to_a_disc_a_hair_below_its_crests_keeps_every_rule() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(-0.5),
            Axis::second(0.0),
            Section::bands(-2.0, &[[2.0, 2.5, 2.5]])
                .bowed(&[[[-1.0, 2.5, -1.0], [-1.0, 2.5, 1.0]]]),
            360.0,
        ),
        vec![Step::add(Leaf::turned(
            Plane::xy(-0.5),
            Axis::second(0.0),
            Section::bands(-2.00000001, &[[2.0, 0.0, 2.5]]),
            360.0,
        ))],
    ));
}

/// Seed 5576809934, lane C round 1 of #576: a zone of a ball, sloped and
/// rounded, cut by the same section unrounded and moved 5e-8 along the axis.
/// Listed, `Pointed`.
#[test]
fn a_rounded_cone_cut_by_itself_moved_a_hair_along_its_axis_keeps_every_rule() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xz(15.0),
            Axis::first(0.0),
            Section::bands(2.5, &[[15.0, 0.0, 25.0]])
                .sloping_to(&[[0.0, 20.0]])
                .bowed(&[[[0.0, 0.0, 0.0], [2.5, 0.0, 25.0]]]),
            360.0,
        ),
        vec![Step::cut(Leaf::turned(
            Plane::xy(0.0),
            Axis::first(15.0),
            Section::bands(2.49999995, &[[15.0, 0.0, 25.0]]).sloping_to(&[[0.0, 20.0]]),
            360.0,
        ))],
    ));
}

// What the first measure of decision 12 broke, each shrunk to a case `main`
// holds, held as `main` holds it.

/// Seed 5576859311: a disc cut by a quarter of a ball whose equator touches the
/// disc's wall on its floor and whose pole stands 5e-8 under its top: raised
/// onto the top, the ball took its equator off the floor.
#[test]
fn a_ball_quarter_a_hair_under_a_disc_s_top_keeps_its_equator_on_the_disc_s_floor() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(5.0), Outline::circle([10.0, 10.0], 30.0), -30.0),
        vec![Step::cut(Leaf::turned(
            Plane::xz(10.0),
            Axis::second(10.0).backwards(),
            Section::bands(-4.999999949999999, &[[59.9999999, 0.0, 0.0]])
                .bowed(&[[[0.0, 0.0, 0.0], [25.0, 0.0, 29.99999995]]]),
            90.0,
        ))],
    ));
}

/// Seed 5576860880: a ball cut by half of itself a hair smaller, then by a ring
/// whose bore is the ball's radius.
#[test]
fn a_ball_cut_by_its_half_a_hair_smaller_then_by_a_ring_keeps_every_rule() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::yz(5.0),
            Axis::second(0.0).backwards(),
            Section::bands(-4.5, &[[5.0, 0.0, 0.0]]).bowed(&[[[0.0, 0.0, 0.0], [-2.0, 0.0, 2.5]]]),
            360.0,
        ),
        vec![
            Step::cut(Leaf::turned(
                Plane::yz(5.0),
                Axis::second(0.0),
                Section::bands(-0.4999999800000001, &[[4.99999996, -0.0, -0.0]])
                    .bowed(&[[[2.0, -0.0, -2.49999998], [0.0, -0.0, -0.0]]]),
                180.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(5.0),
                Outline::ring([5.0, 0.0], 3.5, 2.5),
                -10.0,
            )),
        ],
    ));
}

/// Seed 5576873713: a cap turned half a turn joined to a bar about its axis,
/// 1e-6 narrower than the cap's rim and far from it along the axis: grown onto
/// the rim for nothing.
#[test]
fn a_bar_a_hair_narrower_than_a_far_cap_s_rim_is_left_as_drawn() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(30.0),
            Axis::second(20.0),
            Section::bands(35.0, &[[5.0, 0.0, 15.0]])
                .sloping_to(&[[0.0, 0.0]])
                .bowed(&[[[0.0, 0.0, 0.0], [15.0, 0.0, 25.0]]]),
            180.0,
        ),
        vec![Step::add(Leaf::turned(
            Plane::yz(20.0),
            Axis::first(30.0),
            Section::bands(-47.5, &[[65.0, -14.999999, -0.0]]),
            360.0,
        ))],
    ));
}

/// Seed 5576883559: a cone rounded by a ring's tube, turned half a turn, cut by
/// a whole ring whose tube is 1e-8 thinner about the same circle: raised onto a
/// crest, the thinner tube came to touch the other inside.
#[test]
fn a_rounded_cone_cut_by_a_ring_a_hair_thinner_about_its_tube_keeps_every_rule() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::yz(-2.0),
            Axis::second(0.0),
            Section::bands(-4.5, &[[4.0, 0.0, 7.0]])
                .sloping_to(&[[0.0, 5.0]])
                .bowed(&[[[0.0, 0.0, 0.0], [-3.0, 5.0, 2.5]]]),
            180.0,
        ),
        vec![Step::cut(Leaf::turned(
            Plane::yz(-2.0),
            Axis::second(0.0),
            Section::bands(-5.49999999, &[[4.99999998, -5.0, -5.0]])
                .bowed(&[[[-3.0, -5.0, -2.49999999], [0.0, 0.0, 0.0]]]),
            360.0,
        ))],
    ));
}

/// Seed 5577019074: a shaft bored, then cut by half a ball a hair inside the
/// bore's radius.
#[test]
fn a_bored_shaft_cut_by_a_half_ball_a_hair_inside_its_bore_keeps_every_rule() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(8.0),
            Axis::first(-1.0).backwards(),
            Section::bands(-5.0, &[[8.0, 0.0, 3.0]]),
            360.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::yz(5.0),
                Outline::circle([-1.0, 8.0], 3.0),
                -5.0,
            )),
            Step::cut(Leaf::turned(
                Plane::xy(8.0),
                Axis::first(-1.0),
                Section::bands(-2.99999999, &[[5.99999998, 0.0, 0.0]])
                    .bowed(&[[[0.0, 0.0, 0.0], [-0.0, 0.0, 2.99999999]]]),
                180.0,
            )),
        ],
    ));
}

/// Seed 5577020735: a ring cut by a ring whose tube is a hair thinner about the
/// same circle and whose ends stand a hair along the axis.
#[test]
fn a_ring_cut_by_a_ring_a_hair_thinner_and_a_hair_along_keeps_every_rule() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(0.0),
            Axis::second(0.0),
            Section::bands(2.0, &[[4.0, -2.5, 0.0]]).bowed(&[[[4.0, -2.5, -2.0], [0.0, 0.0, 0.0]]]),
            360.0,
        ),
        vec![Step::cut(Leaf::turned(
            Plane::yz(0.0),
            Axis::first(0.0),
            Section::bands(2.00000001, &[[3.99999998, 0.0, 2.5]])
                .bowed(&[[[0.0, 0.0, 0.0], [4.0, 2.5, 1.99999999]]]),
            360.0,
        ))],
    ));
}

/// Seed 5577022049: a ring touching a post along its inner equator, joined to a
/// band whose end plane passes through that circle, its face short of it: the
/// plane counted as parting the ring there.
#[test]
fn a_ring_touching_a_post_inside_is_kept_whole_beside_a_plane_short_of_the_touch() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::yz(180.0),
            Axis::second(-30.0),
            Section::bands(-15.0, &[[210.0, 165.0, 165.0]])
                .bowed(&[[[90.0, 165.0, -105.0], [90.0, 165.0, 105.0]]]),
            360.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(-15.0),
                Outline::circle([180.0, -30.0], 60.0),
                180.0,
            )),
            Step::add(Leaf::turned(
                Plane::yz(180.0),
                Axis::second(-30.0),
                Section::bands(45.0, &[[45.0, -45.0, -0.0]]),
                360.0,
            )),
        ],
    ));
}

/// Seed 5577024047: a disc cut by a ball 6e-6 inside its wall, then by a
/// rounded square as round as the disc: shrunk onto the ball, the square's wall
/// left the disc's.
#[test]
fn a_disc_cut_by_a_ball_a_hair_inside_its_wall_then_by_its_own_wall_keeps_every_rule() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(150.0),
            Outline::circle([300.0, 240.0], 135.0),
            300.0,
        ),
        vec![
            Step::cut(Leaf::turned(
                Plane::xy(240.0),
                Axis::second(300.0).backwards(),
                Section::bands(15.000006000000013, &[[269.999988, 0.0, 0.0]])
                    .bowed(&[[[0.0, 0.0, 0.0], [150.0, 0.0, 134.999994]]]),
                360.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(150.0),
                Outline::rounded([165.0, 105.0], [435.0, 375.0], 135.0),
                300.0,
            )),
        ],
    ));
}

/// Seed 5577028469: a ball joined to three quarters of itself a hair larger,
/// then cut by a band a hair along.
#[test]
fn a_ball_joined_to_three_quarters_of_itself_a_hair_larger_then_cut_keeps_every_rule() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::yz(8.0),
            Axis::first(0.0),
            Section::bands(-1.0, &[[4.0, 0.0, 0.0]]).bowed(&[[[0.0, 0.0, 0.0], [1.0, 0.0, 2.0]]]),
            360.0,
        ),
        vec![
            Step::add(Leaf::turned(
                Plane::xy(0.0),
                Axis::second(8.0),
                Section::bands(-1.0000000199999999, &[[4.00000004, -0.0, -0.0]])
                    .bowed(&[[[1.0, -0.0, -2.00000002], [0.0, -0.0, -0.0]]]),
                270.0,
            )),
            Step::cut(Leaf::turned(
                Plane::yz(8.0),
                Axis::first(0.0),
                Section::bands(-1.0000000199999999, &[[4.00000004, 0.0, 2.00000022]]),
                270.0,
            )),
        ],
    ));
}

/// Seed 5577028782: a cap cut by its own copy 1e-7 along the axis: laid onto
/// it, the copy lost the lens between the two, which a line through it at a
/// slant saw.
#[test]
fn a_cap_cut_by_its_copy_a_hair_along_its_axis_keeps_the_lens_between_them() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(-0.5),
            Axis::second(0.0),
            Section::bands(2.0, &[[1.0, 0.0, 3.0]])
                .sloping_to(&[[0.0, 0.0]])
                .bowed(&[[[0.0, 0.0, 0.0], [-2.0, 0.0, 5.0]]]),
            360.0,
        ),
        vec![Step::cut(Leaf::turned(
            Plane::xy(-0.5),
            Axis::second(0.0),
            Section::bands(1.9999999, &[[1.0000000000000002, -3.0, -0.0]])
                .sloping_to(&[[-0.0, -0.0]])
                .bowed(&[[[-2.0000001, -0.0, -5.0], [0.0, -0.0, -0.0]]]),
            360.0,
        ))],
    ));
}
