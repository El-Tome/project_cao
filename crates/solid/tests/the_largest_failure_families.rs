//! Closes #644.
//! - each family fixed is named in `docs/exact-kernel-failures.md`, with a
//!   test that fails before its fix —
//!   `a_block_under_a_slab_leaning_a_ten_millionth_of_a_degree_keeps_every_rule`,
//!   `a_ring_a_tenth_of_a_degree_short_of_a_turn_against_a_post_s_end_keeps_every_rule`,
//!   `a_notched_ring_between_two_whole_latitudes_keeps_every_rule`,
//!   `a_ring_round_a_post_of_its_inner_radius_keeps_every_rule`,
//!   `a_ring_in_a_band_of_its_outer_radius_keeps_every_rule`,
//!   `a_ball_a_half_band_touches_along_half_its_equator_keeps_every_rule`,
//!   `a_block_cut_at_45_degrees_a_hair_from_its_top_edge_keeps_every_rule`
//! - every draw is measured on #585's tip and on this branch's tip, same
//!   seeds, verdicts compared case by case, and no case that held is built
//!   wrong or refused — no test: a campaign is run by hand
//!   (`docs/soundness.md`), and the comparison is written in
//!   `docs/exact-kernel-failures.md`, "#644"
//! - what is left is sorted by family, for the next pass — no test: prose,
//!   the same section
//!
//! Each case is the shrunk case of a seed #585's campaigns named, every one
//! of which broke a rule on #585's branch.

// The drawing, the promise and the checks are the campaigns'; this file
// only builds cases and holds them.
#[allow(dead_code, unused_imports)]
mod random_solids;

use random_solids::{Axis, Case, Leaf, Outline, Plane, Section, Step};

/// Seed 5585501573: a block whose top stands at 40, cut by a slab whose
/// floor leans a ten-millionth of a degree off it through a point on it.
/// Three corners of the top stood within the tolerance of the floor and
/// one 9 % past it, so the two planes neither grazed nor crossed cleanly.
/// Declined `Unverified`.
#[test]
fn a_block_under_a_slab_leaning_a_ten_millionth_of_a_degree_keeps_every_rule() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::tilted([10.0, 15.0, 20.0], [90.0, -15.0, 0.0]),
            Outline::rectangle([-10.0, 15.0], [18.0, 20.0]),
            45.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::tilted([5.0, -5.0, 40.0], [1e-7, 0.0, 0.0]),
            Outline::rectangle([-23.0, -33.0], [25.0, 28.0]),
            5.0,
        ))],
    ));
}

/// Seed 5585602707: a ring turned 359.9°, the end of a coaxial post resting
/// on its crest. The ring's face, slit along the crest, was laid from the
/// widest gap between its two meridians, through the face. `Closed`.
#[test]
fn a_ring_a_tenth_of_a_degree_short_of_a_turn_against_a_post_s_end_keeps_every_rule() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(20.0),
            Axis::first(5.0),
            Section::bands(-5.0, &[[10.0, 10.0, 10.0]])
                .bowed(&[[[0.0, 10.0, -5.0], [0.0, 10.0, 5.0]]]),
            359.9,
        ),
        vec![Step::add(Leaf::prism(
            Plane::yz(5.0),
            Outline::circle([5.0, 20.0], 10.0),
            40.0,
        ))],
    ));
}

/// Seed 5585617453: a ring notched by a coaxial band turned 270° and cut
/// down to the stretch between two whole latitude circles. The latitudes
/// sample every angle of the grid, and the face was laid from the widest
/// gap between the notch's meridians, inside the notch. `Closed`.
#[test]
fn a_notched_ring_between_two_whole_latitudes_keeps_every_rule() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xz(-2.0),
            Axis::first(1.0).backwards(),
            Section::bands(-0.5, &[[4.0, 4.5, 4.5]]).bowed(&[[[1.5, 4.5, -2.0], [1.5, 4.5, 2.0]]]),
            360.0,
        ),
        vec![
            Step::cut(Leaf::turned(
                Plane::xz(-2.0),
                Axis::first(1.0).backwards(),
                Section::bands(1.5, &[[1.0, 0.0, 7.0]]),
                270.0,
            )),
            Step::cut(Leaf::turned(
                Plane::xz(-2.0),
                Axis::first(1.0),
                Section::bands(-4.0, &[[3.0, -6.0, -0.0]]),
                360.0,
            )),
        ],
    ));
}

/// Seed 5576855166: a ring of tube 45 about a ring radius of 60, joined to a
/// coaxial post of radius 15 that touches its inner equator. The circle
/// they touch along parts neither face, and the ring's chords along it, on
/// its own grid, cut across the post's between two of its steps.
/// `Uncrossed`.
#[test]
fn a_ring_round_a_post_of_its_inner_radius_keeps_every_rule() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xz(-45.0),
            Axis::second(150.0),
            Section::bands(-105.0, &[[90.0, 60.0, 60.0]])
                .bowed(&[[[-60.0, 60.0, -45.0], [-60.0, 60.0, 45.0]]]),
            360.0,
        ),
        vec![Step::add(Leaf::turned(
            Plane::xz(-45.0),
            Axis::second(150.0),
            Section::bands(-105.0, &[[90.0, 0.0, 15.0]]),
            360.0,
        ))],
    ));
}

/// Seed 5585606247: a ring of tube 2 about a ring radius of 5, joined to a
/// coaxial band whose inner wall, of radius 7, touches its outer equator.
/// The ring stands a hair inside the circle for some way on either side,
/// and so do the band's chords between two of their steps. `Uncrossed`.
#[test]
fn a_ring_in_a_band_of_its_outer_radius_keeps_every_rule() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(6.0),
            Axis::first(0.0),
            Section::bands(-3.0, &[[4.0, 5.0, 5.0]])
                .bowed(&[[[-1.0, 5.0, -2.0], [-1.0, 5.0, 2.0]]]),
            360.0,
        ),
        vec![Step::add(Leaf::turned(
            Plane::xy(6.0),
            Axis::first(0.0),
            Section::bands(-2.0, &[[4.0, 7.0, 8.0]]),
            360.0,
        ))],
    ));
}

/// Seed 5585606550: a ball of radius 5 joined to a coaxial band of radius 5
/// to 6 turned half a turn, which touches it along half its equator. The
/// ball's face, slit along that half, was read as holding one of its poles
/// alone. `Listed`, refused through the application's body.
#[test]
fn a_ball_a_half_band_touches_along_half_its_equator_keeps_every_rule() {
    let case = Case::new(
        Leaf::turned(
            Plane::xy(1.0),
            Axis::second(3.0),
            Section::bands(-2.0, &[[10.0, 0.0, 0.0]]).bowed(&[[[3.0, 0.0, -5.0], [0.0, 0.0, 0.0]]]),
            360.0,
        ),
        vec![Step::add(Leaf::turned(
            Plane::xy(1.0),
            Axis::second(3.0),
            Section::bands(-2.0, &[[10.0, 5.0, 6.0]]),
            180.0,
        ))],
    );
    random_solids::holds_exactly(&case);
    random_solids::holds_through_the_application(&case);
}

/// Seed 5585500692: a rounded block cut by a slab at 45 degrees whose
/// plane passes its top edge by a third of the tolerance. The top and the
/// side cross the cut along two lines that far from the edge, three planes
/// nearly sharing one line. Declined `Unverified`.
#[test]
fn a_block_cut_at_45_degrees_a_hair_from_its_top_edge_keeps_every_rule() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(150.0),
            Outline::rounded([180.0, 90.0], [255.0, 285.0], 30.0),
            60.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::tilted(
                    [179.99999979160248, 90.0, 209.99999978419805],
                    [0.0, 45.0, 0.0],
                ),
                Outline::rectangle([-15.0, -15.0], [120.0, 210.0]),
                -60.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(-30.0),
                Outline::rectangle([180.0, 300.0], [240.0, 510.0]),
                225.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(210.0),
                Outline::rectangle([-105.0, -120.0], [90.0, 135.0]),
                105.0,
            )),
        ],
    ));
}
