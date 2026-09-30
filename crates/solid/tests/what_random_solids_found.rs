//! What the first campaigns over random solids found, shrunk and named, each
//! waiting on the issue that describes it.
//!
//! A case here fails today, which is why it is ignored: the gate cannot hold a
//! test that is red on purpose. It stays in the repository so that the case
//! is never lost, and `--ignored` runs them all. The commit that fixes one
//! takes its `ignore` off, and from then on the gate holds it.
//!
//! Closes #448.
//! - the cases a campaign finds come back as named tests —
//!   `a_quarter_turn_backwards_keeps_every_rule`,
//!   `a_profile_a_hair_across_its_axis_keeps_every_rule`,
//!   `a_turn_a_hair_off_its_axis_keeps_every_rule`,
//!   `a_cut_that_misses_the_block_leaves_it_closed`,
//!   `a_boss_flush_with_a_side_leaves_the_block_closed`,
//!   `a_floor_raised_a_billionth_leaves_the_block_closed`
//! - what the first runs find is written down, one issue per distinct failure,
//!   and nothing is fixed here — no test: #486, #487, #488, #489, #490, #491,
//!   #492, #493 and #494, found by campaigns over some hundred thousand
//!   random solids and five hundred random parts; this file and the ignored
//!   tests beside the part campaign keep their cases

// The drawing and the shrinking are the campaign's; this file only builds
// cases and holds them.
#[allow(dead_code, unused_imports)]
mod random_solids;

use random_solids::{Case, Leaf, Outline, Plane, Step};

#[test]
#[ignore = "#486"]
fn a_quarter_turn_backwards_keeps_every_rule() {
    random_solids::holds(&Case::new(
        Leaf::revolution(Plane::xy(0.0), [1.0, 0.0], [2.0, 1.0], -45.0),
        vec![],
    ));
}

#[test]
#[ignore = "#487"]
fn a_profile_a_hair_across_its_axis_keeps_every_rule() {
    random_solids::holds(&Case::new(
        Leaf::revolution(Plane::xy(0.0), [-0.0001, 0.0], [1.0, 1.0], 360.0),
        vec![],
    ));
}

#[test]
#[ignore = "#488"]
fn a_turn_a_hair_off_its_axis_keeps_every_rule() {
    random_solids::holds(&Case::new(
        Leaf::revolution(Plane::xy(0.0), [1e-6, 0.0], [1.0, 1.0], 360.0),
        vec![],
    ));
}

#[test]
#[ignore = "#489"]
fn a_cut_that_misses_the_block_leaves_it_closed() {
    random_solids::holds(&Case::new(
        Leaf::prism(
            Plane::xy(0.0),
            Outline::rectangle([0.0, 0.0], [1.0, 1.0]),
            1.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::yz(1e-7),
            Outline::rectangle([2.0, 0.0], [3.0, 1.0]),
            1.0,
        ))],
    ));
}

#[test]
#[ignore = "#489"]
fn a_boss_flush_with_a_side_leaves_the_block_closed() {
    random_solids::holds(&Case::new(
        Leaf::prism(
            Plane::xy(0.0),
            Outline::rectangle([0.0, 0.0], [10.0, 10.0]),
            5.0,
        ),
        vec![Step::add(Leaf::prism(
            Plane::xy(5.0),
            Outline::rectangle([1e-7, 2.0], [4.0, 8.0]),
            3.0,
        ))],
    ));
}

#[test]
#[ignore = "#490"]
fn a_floor_raised_a_billionth_leaves_the_block_closed() {
    random_solids::holds(&Case::new(
        Leaf::prism(
            Plane::xy(0.0),
            Outline::rectangle([0.0, 0.0], [2.0, 2.0]),
            1.0,
        ),
        vec![Step::add(Leaf::prism(
            Plane::xy(1e-9),
            Outline::rectangle([1.0, 0.5], [3.0, 1.5]),
            1.0,
        ))],
    ));
}
