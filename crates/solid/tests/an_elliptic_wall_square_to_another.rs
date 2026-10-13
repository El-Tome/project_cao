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
//!
//! Round 2 again: a round bar ending inside an elliptic boss a hair above
//! the boss's rim leaves its cap's circle standing just inside the boss's
//! wall, closer than the rim's chords sag there; the face the rim bounds
//! was drawn across the cap. An elliptic wall's rims and the curves on it
//! now take a sample at the angle of every sample of another curve standing
//! that close inside it, as a cylinder's circles take a ray through it
//! (`tessellation/sampling/beneath/oval.rs`).
//!
//! Round 4: an elliptic wall cut from a wall square to it lies over that
//! wall along the curve the two meet along, and the two part slowly from
//! it; sampled on its own grid alone, its triangles fanned across a step
//! sagged under the other wall's and crossed them. Its rims take the
//! eccentric angles of every sample of that curve, as a cylinder's circles
//! take the rays through them, and each wall is cut in strips between them
//! (`tessellation/contact/own.rs`).
//!
//! Still failing, ignored with their reason: a third surface through a node
//! of two walls square to each other, or a hair from one; a round bar
//! touching an elliptic hole at a point; a crescent between two parallel
//! walls a hair apart, each at a node of an elliptic hole.

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

#[test]
fn seed_5571519943_a_round_bar_ending_a_hair_inside_an_elliptic_boss_s_rim_is_drawn_uncrossed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(-1.0),
            Outline::rectangle([-1.0, 1.0], [11.0, 11.0]),
            -3.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::yz(-1.0),
                Outline::ellipse([5.0, 6.0], [5.480903307962513, 0.45792895602438877], 4.0),
                4.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(-1.9910123116881127),
                Outline::circle([-1.0, 5.0], 3.0),
                -7.0,
            )),
        ],
    ));
}

#[test]
fn seed_5571523190_an_elliptic_bar_joined_across_a_boss_two_holes_crossed_is_drawn_closed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(2.0),
            Outline::rectangle(
                [-2.8831406486676987, 1.3660865617868154],
                [6.883140648667698, 10.633913438213185],
            ),
            -3.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(2.0),
                Outline::ellipse([2.0, 6.0], [2.598076211353316, 1.4999999999999998], 2.5),
                8.5,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(2.3660865617868154),
                Outline::ellipse_from(
                    [2.0, 5.5],
                    [2.883140648667699, 0.0],
                    2.883140648667699,
                    242.99394652258277,
                ),
                -7.267826876426369,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(1.8660865617868154),
                Outline::circle([2.883140648667699, 6.5], 2.0),
                -4.133913438213185,
            )),
            Step::add(Leaf::prism(
                Plane::yz(5.383140648667699),
                Outline::ellipse([4.0, 6.5], [2.8571428571428568, 0.0], 2.0),
                -2.5,
            )),
        ],
    ));
}

#[test]
fn seed_5571534144_an_elliptic_post_cut_by_an_elliptic_hole_and_a_round_bar_is_drawn_uncrossed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(-2.0),
            Outline::ellipse([0.0, 0.0], [3.464101615137755, 1.9999999999999998], 6.0),
            5.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(-6.0677643628300215),
                Outline::ellipse(
                    [0.0, -4.101492007535095],
                    [0.4581544946141771, 1.7098558516161508],
                    4.720461426967995,
                ),
                12.135528725660045,
            )),
            Step::cut(Leaf::prism(
                Plane::yz(-6.08257569495584),
                Outline::circle([-2.5, 0.0], 5.567764262830022),
                12.165151389911678,
            )),
        ],
    ));
}

#[test]
fn seed_5571537754_an_elliptic_post_bored_across_and_joined_to_an_elliptic_bar_is_drawn_uncrossed()
{
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(-10.0),
            Outline::ellipse([40.0, 10.0], [-20.0, 0.0], 17.5),
            47.5,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xz(-15.0),
                Outline::circle([40.0, 10.0], 20.0),
                -50.0,
            )),
            Step::add(Leaf::prism(
                Plane::yz(17.5),
                Outline::ellipse(
                    [10.0, 20.0],
                    [-7.337993857053425, 12.70977818604485],
                    18.34498464263357,
                ),
                45.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(-9.999999999999998),
                Outline::circle([45.0, 32.5], 15.0),
                -40.0,
            )),
        ],
    ));
}

#[test]
#[ignore = "#571: a hole's end cap through the node of a figure of eight an elliptic hole makes with a round pocket is left unverified"]
fn seed_5571542016_an_elliptic_hole_ending_at_its_node_with_a_round_pocket_is_cut() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(-2.0),
            Outline::ellipse([4.0, 5.0], [0.0, 2.0], 2.5),
            6.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::yz(7.0),
                Outline::circle([5.0, 2.0], 1.99999999),
                -3.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(7.99999999),
                Outline::ellipse([5.0, 2.99999999], [8.0, 0.0], 1.0),
                2.99999999,
            )),
        ],
    ));
}

#[test]
#[ignore = "#571: a round bore's cap through the near-nodes an elliptic hole a hair narrower than a post makes with it is a tie"]
fn seed_5571541652_a_bore_ending_at_the_near_nodes_of_a_post_and_an_elliptic_hole_is_cut() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(2.0), Outline::circle([10.0, 8.0], 2.0), 10.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::yz(7.0),
                Outline::ellipse([8.0, 7.5], [0.0, 3.99999998], 1.99999999),
                6.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(10.49999999),
                Outline::circle([10.0, 5.49999002], 2.0),
                2.499999989999999,
            )),
        ],
    ));
}

#[test]
#[ignore = "#571: a round bar touching an elliptic hole at a point is drawn crossing it there"]
fn seed_5571548076_a_round_bar_touching_an_elliptic_hole_at_a_point_is_drawn_uncrossed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(-1.0), Outline::circle([10.0, 3.0], 3.0), 7.5),
        vec![
            Step::cut(Leaf::prism(
                Plane::xz(-0.5),
                Outline::ellipse([10.00000002, 2.0], [3.0, 0.0], 2.6999999999999997),
                -3.5,
            )),
            Step::cut(Leaf::prism(
                Plane::yz(6.00000002),
                Outline::circle([1.0, -4.199999999999999], 3.5),
                8.0,
            )),
        ],
    ));
}

#[test]
#[ignore = "#571: a crescent between two parallel walls a hair apart, each at a node of an elliptic hole, is read backwards"]
fn seed_5571546569_a_post_bored_by_an_elliptic_hole_then_by_its_own_wall_a_hair_off_is_listed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(60.0), Outline::circle([60.0, 30.0], 90.0), 240.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xz(-90.0),
                Outline::ellipse([60.00001, 180.0], [90.0, 0.0], 18.0),
                -120.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(117.0),
                Outline::circle([60.00001, 30.0], 90.0),
                126.0,
            )),
        ],
    ));
}
