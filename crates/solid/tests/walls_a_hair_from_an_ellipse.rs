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
//!   `an_ellipse_joined_with_its_osculating_circle_at_a_vertex_keeps_every_rule`,
//!   `a_circle_added_to_an_ellipse_a_hair_from_round_of_its_size_keeps_every_rule`,
//!   `an_ellipse_a_hair_from_round_added_in_a_bore_of_its_size_keeps_every_rule`,
//!   `the_same_ellipse_turned_a_hundred_thousandth_added_back_in_its_bore_keeps_every_rule`,
//!   `an_elliptic_pocket_reaching_a_round_bar_s_sides_at_its_axis_keeps_every_rule`,
//!   `an_elliptic_tunnel_as_wide_as_a_tube_s_wall_keeps_every_rule`,
//!   `an_elliptic_wall_cut_a_hundred_thousandth_of_a_degree_off_its_tangent_keeps_every_rule`
//! - what stays above the bar is written in `docs/exact-kernel-failures.md` —
//!   no test: prose, the same section
//!
//! Closes #677.
//! - the choice is written as decided in `docs/exact-kernel.md` (decision 8)
//!   and in `docs/exact-kernel-failures.md` — no test: prose, held by
//!   `language.rs` only
//! - if the merge is widened or the crossings are built, the ellipses draw is
//!   measured on fresh seeds against `main` — no test: neither is done, the
//!   two walls are left two
//!
//! Each case is the shrunk case of a seed that broke a rule on `main`, of the
//! ellipses' draw or, where its comment says so, of the crossed or the tilted
//! draw.

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

/// Seed 5585700934: an elliptic disc joined with a disc of the radius it
/// curves by at the end of its major axis, touching it there: the
/// osculating circle. Leaving the vertex they touch at, the two set off
/// together and bend alike, and were told apart by where each stood once
/// it had run as far — the ellipse read by the share of its parameter,
/// further along than that, and ordered outside the circle on the wrong
/// side. Declined as a tie.
#[test]
fn an_ellipse_joined_with_its_osculating_circle_at_a_vertex_keeps_every_rule() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(-2.0),
            Outline::ellipse([5.0, 8.0], [-3.5, 0.0], 3.49965),
            2.0,
        ),
        vec![Step::add(Leaf::prism(
            Plane::yz(-2.0),
            Outline::circle([5.000699965, 8.0], 3.499300035),
            2.0,
        ))],
    ));
}

/// Seed 5585703794: an elliptic prism whose half-axes are 2.5 and
/// 2.5000025, raised alone at a tolerance finer than the hair between
/// them, joined with a circle of radius 2.5 about the same centre and
/// reaching further. At the boolean's coarser tolerance the two walls stood
/// within decision 8's hair of each other all round, and the pair was
/// declined `Unsupported`.
#[test]
fn a_circle_added_to_an_ellipse_a_hair_from_round_of_its_size_keeps_every_rule() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(-10.00000005),
            Outline::ellipse([2.5, 47.5], [0.0, 2.5], 2.5000025),
            90.0,
        ),
        vec![Step::add(Leaf::prism(
            Plane::yz(-10.00000005),
            Outline::circle([2.5, 47.5], 2.5),
            180.0,
        ))],
    ));
}

/// Seed 5585717426: a block bored through by a circle, and an ellipse of
/// its centre added back in the bore, 3e-6 narrower than the circle one
/// way. As above, the other way round.
#[test]
fn an_ellipse_a_hair_from_round_added_in_a_bore_of_its_size_keeps_every_rule() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(60.0),
            Outline::rectangle([-90.0, 15.0], [90.0, 165.0]),
            120.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xz(-15.0),
                Outline::circle([72.9358872521045, 94.30614136886825], 5.854876852788398),
                -150.0,
            )),
            Step::add(Leaf::prism(
                Plane::xz(-15.00001),
                Outline::ellipse(
                    [72.9358872521045, 94.30614136886825],
                    [-0.0, -5.854876852788398],
                    5.854873852794253,
                ),
                -150.0,
            )),
        ],
    ));
}

/// Seed 5585713456: a block, an ellipse of half-axes 6.5 and 5 cut through
/// it, then the same ellipse turned by a hundred-thousandth of a radian added
/// back, reaching further. The two elliptic walls cross at a grazing angle
/// near the ellipse's four vertices, a hair apart between, and the block's
/// sides touch the ellipse at those vertices. The body is built, but two of
/// its triangles cross, breaking `Uncrossed`: one on the block's face in its
/// sketch's plane, beside the end of the minor axis where the block's side
/// touches the ellipse, an edge of it a chord of the bore's rim that reaches
/// inside the added ellipse; the other on the added ellipse's wall beside
/// that end, standing through that face.
#[test]
fn the_same_ellipse_turned_a_hundred_thousandth_added_back_in_its_bore_keeps_every_rule() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(4.0),
            Outline::rectangle([1.0, -2.0], [14.0, 8.0]),
            2.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xz(4.5),
                Outline::ellipse([7.5, 3.0], [6.5, 0.0], 5.0),
                3.0,
            )),
            Step::add(Leaf::prism(
                Plane::xz(4.5),
                Outline::ellipse([7.5, 3.0], [6.4999999996749995, -6.499999999891667e-5], 5.0),
                7.0,
            )),
        ],
    ));
}

/// Seed 5585800487 of the crossed draw: a round bar along x of radius 180, cut
/// from its top down to its axis by an elliptic prism standing along z, the
/// ellipse turned 45° and reaching exactly the bar's sides along y. The two
/// walls, square to each other, touch at both ends of that reach: a node. The
/// elliptic wall was grown onto the touch by a hair, its minor half-axis moved
/// by about 1e-6, but the rims cut along it still stood on the wall as it was,
/// and seen on the grown wall were no curve it carries: declined
/// `Unsupported`.
#[test]
fn an_elliptic_pocket_reaching_a_round_bar_s_sides_at_its_axis_keeps_every_rule() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::yz(0.0), Outline::circle([15.0, 240.0], 180.0), 135.0),
        vec![Step::cut(Leaf::prism(
            Plane::xy(450.0),
            Outline::ellipse(
                [90.0, 15.0],
                [80.4984473241565, -80.49844732415654],
                227.68399191159668,
            ),
            -210.0,
        ))],
    ));
}

/// Seed 5585800308 of the crossed draw: a tube standing along z, of radius 90
/// bored to 30, and an elliptic tunnel of half-axes 30 and 30.0000003 cut
/// along y in its wall, as wide as the wall where it is thinnest and 3e-7
/// past it on either side, nine tenths of the tolerance. The tunnel's wall
/// stands square to the tube's outside and to its bore, and touches each at
/// one point, an end of the tunnel's reach: a node of their crossing. The
/// body is built, but triangles of the bore and of the tunnel's wall cross
/// beside that touch, breaking `Uncrossed`. Within decision 8's hair of
/// round, the tunnel is raised as a circle (Tom, 2026-10-13): what this holds
/// is two round walls touching, not an elliptic one.
#[test]
fn an_elliptic_tunnel_as_wide_as_a_tube_s_wall_keeps_every_rule() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(0.0),
            Outline::ring([210.0, 165.0], 90.0, 30.0),
            255.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::xz(90.0),
            Outline::ellipse([150.0, 105.0], [0.0, 30.0], 30.000000299999996),
            -240.0,
        ))],
    ));
}

/// Seed 5585909807 of the tilted draw: a bar of elliptic section, half-axes
/// 2.5 and 5 turned 30° in its sketch, ten long along y, cut by a block whose face is the
/// wall's tangent plane along its highest ruling, leaning a
/// hundred-thousandth of a degree along the bar. The face touches the wall
/// at the top of the far end's rim and stands 1.7e-6 into it at the near
/// end: a sliver of the wall some five thousandths wide is cut away, along
/// a conic whose vertex is that touch. The lean is past what decision 11
/// lays along the axis and inside the band the kernel builds as drawn and
/// warns of, and the step was declined as a tie, the overlay finding no
/// point inside one of the regions it parts.
#[test]
fn an_elliptic_wall_cut_a_hundred_thousandth_of_a_degree_off_its_tangent_keeps_every_rule() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(3.5),
            Outline::ellipse([3.0, 8.0], [-2.1650635094610964, -1.2500000000000002], 5.0),
            -10.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::tilted(
                [1.1985582696927697, 13.5, 12.506939094329987],
                [1e-5, 0.0, 0.0],
            ),
            Outline::rectangle([-3.0, -11.0], [6.0, 1.0]),
            10.0,
        ))],
    ));
}
