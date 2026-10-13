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
//! as they stand (`relation/square.rs`).

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
