//! What the tilted campaign of ellipses (#571) found on a plane meeting an
//! elliptic wall at a slant or leaning a hair off its axis, each seed shrunk
//! and kept as the test it was fixed against.
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
