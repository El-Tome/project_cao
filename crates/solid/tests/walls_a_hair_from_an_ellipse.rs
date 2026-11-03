//! Closes #574.
//! - the three ellipse draws end under one failure in a thousand on the exact
//!   kernel, 20 000 fresh seeds each — no test: a campaign is run by hand
//!   (`docs/soundness.md`), and its count is written in
//!   `docs/exact-kernel-failures.md`, "#574"
//! - on the same seeds of every other draw, exact kernel and application's
//!   body, no seed that holds on `main` breaks — no test: both trees are
//!   measured on the same seeds and their verdicts compared case by case, in
//!   the same section
//! - each family fixed has a test that fails on `main` —
//!   `an_arc_of_an_ellipse_a_hair_from_round_cut_from_its_circle_keeps_every_rule`,
//!   `half_an_ellipse_a_hair_from_round_added_to_its_circle_keeps_every_rule`,
//!   `an_ellipse_a_millionth_short_of_round_touching_its_circle_keeps_every_rule`,
//!   `the_same_ellipse_turned_a_hair_past_decision_8_keeps_every_rule`,
//!   `the_same_ellipse_cut_from_itself_a_ten_thousandth_aside_keeps_every_rule`
//! - what stays above the bar is written in `docs/exact-kernel-failures.md` —
//!   no test: prose, the same section
//!
//! Each case is the shrunk case of a seed of the ellipses' draw that broke a
//! rule on `main`.

// The drawing, the promise and the checks are the campaigns'; this file
// only builds cases and holds them.
#[allow(dead_code, unused_imports)]
mod random_solids;

use random_solids::{Case, Leaf, Outline, Plane, Step};

/// Seed 5585703945: a disc of radius 4, cut by an ellipse of its centre
/// whose half-axes are 4 and 4.00000004, chorded to an arc of 3.8°. The arc
/// stood a hair past the tolerance from the disc's wall, within decision
/// 8's hair of it at every foot, and the pair was declined `Unsupported`.
/// Its corners laid on the circle of its mean, it is that circle's arc.
#[test]
fn an_arc_of_an_ellipse_a_hair_from_round_cut_from_its_circle_keeps_every_rule() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(0.0), Outline::circle([8.0, 10.0], 4.0), 5.0),
        vec![Step::cut(Leaf::prism(
            Plane::xy(0.0),
            Outline::chorded(
                [8.0, 10.0],
                [4.0, 0.0],
                4.00000004,
                &[[-101.79004343093729, -97.97568746899904]],
            ),
            1.0,
        ))],
    ));
}

/// Seed 5585718355: half an ellipse whose half-axes are 1.99999998 and
/// 2.000000001, added to a disc of radius 2 about the same centre. Declined
/// `Unsupported`, as above.
#[test]
fn half_an_ellipse_a_hair_from_round_added_to_its_circle_keeps_every_rule() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(3.0), Outline::circle([2.0, 5.0], 2.0), 1.0),
        vec![Step::add(Leaf::prism(
            Plane::xy(3.0),
            Outline::chorded(
                [2.0, 5.0],
                [-1.9999999811112164, -0.0],
                2.000000001111216,
                &[[270.0, 450.0]],
            ),
            1.0,
        ))],
    ));
}

/// Seed 5585700308: a disc of radius 2, an ellipse of its centre whose
/// half-axes are 2 and 1.999999 added below and through it, and a second
/// disc beside them. The ellipse touches the disc's wall at the ends of its
/// major axis, and stands within the tolerance of it over a band either
/// side, where the disc's rims were seen on the elliptic wall as their
/// chords, which bulge off them past the tolerance: declined `Unsupported`.
#[test]
fn an_ellipse_a_millionth_short_of_round_touching_its_circle_keeps_every_rule() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(-2.0), Outline::circle([4.0, 5.0], 2.0), 9.0),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(-3.0),
                Outline::ellipse(
                    [4.0, 5.0],
                    [-1.7320508075688774, 0.9999999999999999],
                    1.999999,
                ),
                4.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(-2.0),
                Outline::circle([6.0, 3.0], 1.0),
                4.0,
            )),
        ],
    ));
}

/// Seed 5585704198: an elliptic disc of half-axes 0.5 and 3, cut by the
/// same ellipse turned a ten-millionth of a radian, a twenty-fifth past
/// decision 8's hair. Read at the feet of the normals the two walls stood
/// within the hair of each other, and the pair was declined `Unsupported`.
#[test]
fn the_same_ellipse_turned_a_hair_past_decision_8_keeps_every_rule() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(-1.0),
            Outline::ellipse([9.0, 2.0], [0.0, -0.5], 3.0),
            2.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::xy(-1.0),
            Outline::ellipse([9.0, 2.0], [4.999999999999991e-8, -0.4999999999999975], 3.0),
            3.0,
        ))],
    ));
}

/// Seed 5585706342: an elliptic disc, a disc added under it, and the same
/// ellipse cut from it a ten-thousandth aside: two lenses a ten-thousandth
/// thick, between rulings where the two walls cross at a grazing angle.
/// Near each, the two rims took places of their own, one a vertex's angle,
/// and the cap's chords crossed: left open, `Closed`.
#[test]
fn the_same_ellipse_cut_from_itself_a_ten_thousandth_aside_keeps_every_rule() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(4.0),
            Outline::ellipse([10.0, 5.0], [0.7927060177944079, 3.9206653988006335], 2.5),
            9.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(3.0),
                Outline::circle([8.774792072676465, 5.247720628578988], 1.25),
                9.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(4.0),
                Outline::ellipse(
                    [10.000050436808616, 5.000086348875712],
                    [0.7927060177944079, 3.9206653988006335],
                    2.5,
                ),
                17.0,
            )),
        ],
    ));
}
