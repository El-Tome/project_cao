//! What the tilted campaign of ellipses (#571) found on a plane meeting an
//! elliptic wall at a slant or leaning a hair off its axis, each seed shrunk
//! and kept as the test it was fixed against.
//!
//! Round 0, measured at `2982f55`, before this lane: such a plane leaning
//! off the wall's axis by less than a thousandth of a radian was declined,
//! and an elliptic wall of the second operand never turned. Laid square or
//! along where decision 11 lays it, and built and noted unstable past it,
//! 2928 of round 0's 4079 failures hold since `2e1ea5c`: a plane leaning a
//! hundredth of a degree off along an elliptic post, an elliptic boss drawn
//! on a plane leaning a ten-millionth of a degree and joined to a block.
//!
//! Round 1, measured at `2e1ea5c`: what #552 decides for a plane meeting a
//! circular wall a hair from a touch reaches an elliptic wall too. A line
//! two planes crossing the axis steeply share, inside the wall's section by
//! rounding alone, only touches it, whichever of its three curves reaches
//! the corner (`relation/crossing/section.rs`): two cuts at 45° and 75°
//! through one point of an elliptic post's rim. Where that line grazes the
//! wall and one of the planes meets it more grazingly still, its crossings
//! are decided on that plane's hairpin (`grazed.rs`): a plane leaning a
//! ten-thousandth of a degree on the ruling an elliptic post or hole
//! reaches furthest along. And a conic crosses a plane read along the
//! wall's axis where the lines that plane cuts the wall along do, once the
//! plane leans off the axis by more than rounding (`conic.rs`, `lying.rs`):
//! an elliptic hole reaching a hair past a cut laid along it.

#[allow(dead_code, unused_imports)]
mod random_solids;

use random_solids::{Case, Leaf, Outline, Plane, Step};

#[test]
fn seed_5571600008_an_elliptic_post_cut_by_a_plane_leaning_0_01_degrees_off_along_its_axis() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(15.0),
            Outline::ellipse([43.0, 30.0], [10.0, 0.0], 5.0),
            23.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::tilted([43.0, 33.0, 26.0], [89.99, 0.0, 0.0]),
            Outline::rectangle([-13.0, -15.0], [13.0, 15.0]),
            13.0,
        ))],
    ));
}

#[test]
fn seed_5571600015_an_elliptic_boss_drawn_on_a_plane_leaning_1e_7_degrees_is_joined_to_a_cut_block()
{
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(15.0),
            Outline::rectangle([-5.0, -10.0], [28.0, 40.0]),
            40.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::tilted([18.0, 13.0, 30.0], [90.0, 0.0, 0.0]),
                Outline::rectangle([-43.0, -45.0], [8.0, 13.0]),
                15.0,
            )),
            Step::add(Leaf::prism(
                Plane::tilted([12.0, 3.0, -1.0], [90.0, 89.9999999, 0.0]),
                Outline::ellipse([0.0, 0.0], [-8.0, -0.0], 10.0),
                8.0,
            )),
        ],
    ));
}

#[test]
fn seed_5571628398_an_elliptic_post_cut_at_45_and_75_degrees_through_one_point_of_its_rim() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(240.0),
            Outline::ellipse([30.0, 240.0], [-7.764571353075626, 28.97777478867205], 90.0),
            -105.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::tilted(
                    [240.0, -57.279387335292256, 219.37657383999357],
                    [90.0, -45.0, 0.0],
                ),
                Outline::rectangle([-225.0, -45.0], [30.0, 75.0]),
                120.0,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted(
                    [240.0, -57.279387335292256, 219.37657383999357],
                    [90.0, -75.0, 0.0],
                ),
                Outline::rectangle([-225.0, -45.0], [30.0, 75.0]),
                75.0,
            )),
        ],
    ));
}

#[test]
fn seed_5571639751_an_ellipse_cut_by_a_trait_and_cut_by_a_plane_leaning_1e_4_degrees_on_its_highest_ruling()
 {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(-15.0),
            Outline::chorded([300.0, 300.0], [45.0, 0.0], 75.0, &[[45.0, 90.0]]),
            150.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::tilted([60.0, 300.0, 375.0], [0.0, 0.0001, 0.0]),
            Outline::rectangle([-90.0, -15.0], [105.0, 45.0]),
            30.0,
        ))],
    ));
}

#[test]
fn seed_5571654147_an_elliptic_hole_cut_by_a_plane_leaning_1e_4_degrees_on_its_highest_ruling() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(1.0),
            Outline::rectangle([-1.0, 7.0], [3.0, 11.0]),
            6.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xz(-5.5000002),
                Outline::ellipse([2.0, 8.5], [1.0, 0.0], 1.6),
                -1.5000001999999997,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted([2.0, -4.0, 10.1], [0.0001, 0.0, 0.0]),
                Outline::rectangle([-2.0, -3.0], [2.0, 1.0]),
                -4.0,
            )),
        ],
    ));
}

#[test]
fn seed_5571636439_an_elliptic_hole_reaching_a_hair_past_a_cut_laid_along_it() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(-1.0),
            Outline::rectangle([-1.0, -2.0], [4.0, 3.0]),
            10.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::tilted([-0.99999994, -2.0, 9.0], [0.0, 89.99999998, 0.0]),
                Outline::rectangle([-1.0, -1.0], [11.0, 6.0]),
                5.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(-2.0),
                Outline::ellipse([1.0, 2.0], [-0.0, -1.0], 2.0),
                11.0,
            )),
        ],
    ));
}
