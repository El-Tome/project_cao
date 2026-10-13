//! What the crossed campaign of ellipses (#571) found on two walls square to
//! each other, an elliptic one among them, each seed kept as the test it was
//! fixed against.
//!
//! Round 1, measured with the relation wired: an elliptic bar a hair from a
//! node with both walls of a ring, its section reaching the outer wall and
//! the inner one at once. Decision 2 moves no wall onto either node, since
//! made exact at one the bar would stand further off the other; the curve
//! the bar meets the outer wall along was built on a copy of that wall moved
//! onto the node, and the corners where the ring's rim crosses the bar,
//! found on the walls as they stand, stood further than the tolerance from
//! it where it leaves the top at a slant. The curve is now built on the walls
//! as they stand (`relation/perpendicular.rs`).
//!
//! Round 2: a round bar lying on an elliptic post's cap, square to the post,
//! meets the post's wall along a curve touching the cap where the bar does,
//! and an arc of the post's rim there is taken for an arc of that curve
//! (decision 6). The rim, a conic, was seen on the bar as nothing it could
//! carry, and the step declined; it is seen there as the curve the post
//! meets the bar along, as a circle square to a cylinder's axis is (and a
//! circle on an elliptic wall square to its own likewise), and the curve on
//! the cap as the section the cap cuts the post along (`domain/traced/square.rs`).

#[allow(dead_code, unused_imports)]
mod random_solids;

use random_solids::{Case, Leaf, Outline, Plane, Step};

#[test]
fn seed_5571519047_an_elliptic_bar_a_hair_from_a_node_with_both_walls_of_a_ring_is_cut_through_it()
{
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(30.0),
            Outline::ring([60.0, 90.0], 120.0, 30.0),
            225.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::yz(-105.0),
            Outline::ellipse_from(
                [135.0, 180.0],
                [0.0, 150.0000006],
                75.0000003,
                265.11021060929164,
            ),
            330.0,
        ))],
    ));
}

#[test]
fn seed_5571523703_a_round_bar_lying_on_an_elliptic_post_s_floor_is_cut_through_the_post() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(-2.0),
            Outline::rectangle([1.0, -1.5], [17.0, 13.5]),
            -1.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(-2.0),
                Outline::ellipse_from([9.0, 6.0], [-5.5, 0.0], 5.0, 254.8004897066198),
                7.5,
            )),
            Step::cut(Leaf::prism(
                Plane::yz(2.5),
                Outline::circle([10.99999, 3.0], 5.0),
                6.5,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(0.5),
                Outline::ellipse([5.0, 0.0], [5.5, 0.0], 20.166666666666664),
                -11.0,
            )),
        ],
    ));
}

#[test]
fn seed_5571525354_a_round_bar_lying_on_an_elliptic_post_s_cap_is_joined_to_it() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(-30.0),
            Outline::ellipse([225.0, 300.0], [0.0, 60.0], 105.0),
            -240.0,
        ),
        vec![Step::add(Leaf::prism(
            Plane::yz(90.0),
            Outline::ellipse([359.99999, -210.0], [0.0, 60.0], 60.0),
            270.0,
        ))],
    ));
}
