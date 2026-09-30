//! What the campaigns over the exact kernel of #498 found in its triangles,
//! each case shrunk and named after the seed that drew it.
//!
//! A case the triangles now keep every rule on is held by the gate. A case
//! understood but not fixed stays, ignored, with what was understood of it
//! written above it.

// The drawing and the checks are the campaign's; this file only builds cases
// and holds them.
#[allow(dead_code, unused_imports)]
mod random_solids;

use random_solids::{Case, Leaf, Outline, Plane, Step};

#[test]
fn seed_595_a_skin_left_under_a_cap_is_drawn_without_a_wall_folding_over_it() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xz(-1.0), Outline::circle([3.0, 3.0], 3.0), 3.0),
        vec![Step::cut(Leaf::prism(
            Plane::xz(-1.0000002),
            Outline::rectangle([5.99999994, 0.9999999399999999], [10.00000006, 5.00000006]),
            3.0,
        ))],
    ));
}

#[test]
fn seed_469_a_pocket_whose_ceiling_touches_a_lying_wall_at_a_point_stays_under_its_chords() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::yz(7.0), Outline::circle([7.0, 2.0], 5.0), -7.0),
        vec![Step::cut(Leaf::prism(
            Plane::xy(5.0),
            Outline::circle([4.0, 6.0], 5.0),
            -4.0,
        ))],
    ));
}

#[test]
fn seed_982_a_boss_slit_by_a_box_touching_it_is_drawn_whole_round_the_block_it_passes() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(2.0),
            Outline::rectangle([-2.0, -3.0], [6.0, 5.0]),
            8.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xz(3.0),
                Outline::circle([6.0, 1.0], 0.5),
                16.0,
            )),
            Step::add(Leaf::prism(
                Plane::xz(3.0000002),
                Outline::rectangle([6.5, 0.5], [7.5, 1.5]),
                32.0,
            )),
        ],
    ));
}

#[test]
fn seed_1317_a_cylinder_swallowed_by_one_it_touches_leaves_both_caps_drawn_closed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(240.0),
            Outline::circle([300.0, 30.0], 135.0),
            165.0,
        ),
        vec![Step::add(Leaf::prism(
            Plane::yz(240.0),
            Outline::circle([315.0, 30.0], 120.0),
            165.0,
        ))],
    ));
}

#[test]
fn seed_3900_a_wall_touched_by_a_block_and_by_a_cylinder_inside_it_is_drawn_closed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xz(30.0), Outline::circle([10.0, 25.0], 5.0), 38.0),
        vec![
            Step::add(Leaf::prism(
                Plane::xz(30.0),
                Outline::circle([8.0, 25.0], 3.0),
                75.0,
            )),
            Step::add(Leaf::prism(
                Plane::yz(15.0),
                Outline::rectangle([15.0, 0.0], [23.0, 38.0]),
                10.0,
            )),
        ],
    ));
}

#[test]
fn seed_979_two_cylinders_touching_at_a_point_of_their_caps_enclose_what_they_promise() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xz(7.0), Outline::circle([1.5, 1.0], 4.5), -7.0),
        vec![Step::add(Leaf::prism(
            Plane::xz(7.0),
            Outline::circle([1.5, 10.0], 4.5),
            7.0,
        ))],
    ));
}

#[test]
fn seed_504_a_boss_touching_a_notch_cut_in_a_disc_encloses_what_it_promises() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(1.0), Outline::circle([5.0, 5.0], 4.5), -2.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(-2.0),
                Outline::circle([3.0, 8.0], 1.0),
                3.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(-2.0),
                Outline::circle([1.0, 8.0], 1.0),
                6.0,
            )),
        ],
    ));
}

#[test]
fn seed_268_a_boss_inside_a_bore_touching_its_line_where_its_wall_is_gone_encloses_its_volume() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(210.0),
            Outline::rectangle([60.0, 90.0], [420.0, 450.0]),
            270.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::yz(210.0000018),
                Outline::circle([90.0, 285.0], 75.0),
                270.0,
            )),
            Step::add(Leaf::prism(
                Plane::yz(210.0000024),
                Outline::circle([45.0, 285.0], 30.0),
                540.0,
            )),
        ],
    ));
}

#[test]
fn seed_5000951_two_cylinders_of_one_radius_a_hair_apart_leave_their_slivers_drawn_closed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(7.0), Outline::circle([10.0, 4.0], 1.5), 9.0),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(7.0),
                Outline::circle([9.99999, 4.0], 1.5),
                6.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(6.0),
                Outline::circle([8.99999, 4.0], 0.50000001),
                11.0,
            )),
        ],
    ));
}

#[test]
fn seed_6000920_two_cylinders_a_hair_off_one_axis_at_heights_apart_enclose_their_volume() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(25.0),
            Outline::circle([35.00001, 40.0], 25.0),
            25.0,
        ),
        vec![Step::add(Leaf::prism(
            Plane::xy(20.0),
            Outline::circle([35.0, 40.0], 25.0),
            -13.0,
        ))],
    ));
}

#[test]
fn seed_6001028_two_cylinders_of_one_radius_barely_apart_enclose_their_union() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::yz(8.0), Outline::circle([2.0, 9.0], 2.0), 7.0),
        vec![Step::add(Leaf::prism(
            Plane::yz(8.0),
            Outline::circle([2.00000002, 9.0], 2.0),
            10.0,
        ))],
    ));
}

#[test]
fn seed_7000173_two_cylinders_of_one_radius_barely_apart_enclose_their_union() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(2.0), Outline::circle([1.0, 3.0], 2.0), 5.0),
        vec![Step::add(Leaf::prism(
            Plane::xy(2.0),
            Outline::circle([0.99999998, 3.0], 2.0),
            9.0,
        ))],
    ));
}

#[test]
fn seed_7000227_a_cylinder_cut_by_one_barely_off_its_axis_encloses_what_is_left() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(38.0), Outline::circle([10.0, 0.0], 10.0), 43.0),
        vec![Step::cut(Leaf::prism(
            Plane::xy(33.0),
            Outline::circle([10.0000001, 0.0], 10.0),
            43.0,
        ))],
    ));
}

#[test]
fn seed_7002169_two_cylinders_of_one_radius_barely_apart_enclose_their_union() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(0.0),
            Outline::circle([27.4999999, 15.0], 15.0),
            38.0,
        ),
        vec![Step::add(Leaf::prism(
            Plane::xy(0.0),
            Outline::circle([27.5, 15.0], 15.0),
            10.0,
        ))],
    ));
}

#[test]
fn seed_7002841_a_cylinder_cut_by_one_barely_off_its_axis_encloses_what_is_left() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(39.99999),
            Outline::circle([62.49999995, 8.75], 15.0),
            65.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::xz(35.0000003),
            Outline::circle([62.4999997, 8.75], 15.0),
            45.0,
        ))],
    ));
}

#[test]
fn seed_6000349_two_lying_cylinders_barely_apart_leave_their_sliver_drawn_closed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(23.0), Outline::circle([23.0, 30.0], 25.0), -5.0),
        vec![
            Step::add(Leaf::prism(
                Plane::xz(2.5),
                Outline::circle([15.0, 12.5], 10.0),
                35.0,
            )),
            Step::add(Leaf::prism(
                Plane::xz(7.49999995),
                Outline::circle([15.000001, 12.5], 10.0),
                35.0,
            )),
        ],
    ));
}

/// The boss, of radius 1.5 about (8.00000002, 3), touches the block's top at
/// its highest point, a vertex; the cylinder it swallows, of radius
/// 1.49999998 about (8, 3), stands 2e-8 below that top. The two share their
/// rays, and the smaller one's grid step at the top, 2e-8 from the plane and
/// so kept, puts a sample on the boss 2e-8 along its circle from the vertex,
/// which is 1e-16 from the top's edge: on it, for the sweep of the front
/// face, which is left open. A sample within a fifth of the kernel's
/// tolerance of a plane touching its wall, but on the line they touch along,
/// should be left out of both walls, as beside a wall it touches; the rule
/// is not written yet.
#[test]
#[ignore = "tangency"]
fn seed_7000204_a_boss_barely_off_a_cylinder_of_one_radius_leaves_the_block_closed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(-2.0),
            Outline::rectangle([5.5, -0.5], [10.5, 4.5]),
            10.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xz(-2.0),
                Outline::circle([8.0, 3.0], 1.49999998),
                10.0,
            )),
            Step::add(Leaf::prism(
                Plane::xz(-2.00000001),
                Outline::circle([8.00000002, 3.0], 1.5),
                2.0,
            )),
        ],
    ));
}

#[test]
fn seed_6002092_three_cylinders_each_touching_the_next_inside_it_leave_the_caps_closed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(7.0), Outline::circle([6.0, 2.0], 2.5), 10.0),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(7.0),
                Outline::circle([6.0, 3.0], 1.5),
                8.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(6.0),
                Outline::circle([6.5, 3.0], 1.0),
                16.0,
            )),
        ],
    ));
}

#[test]
fn seed_9000717_a_hole_touching_two_crossing_cylinders_inside_leaves_their_cap_closed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xz(6.0), Outline::circle([5.0, 5.0], 3.5), 10.0),
        vec![
            Step::add(Leaf::prism(
                Plane::xz(6.0),
                Outline::circle([3.0, 2.0], 2.5),
                20.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(6.0),
                Outline::circle([5.0, 2.0], 0.5),
                40.0,
            )),
        ],
    ));
}

#[test]
fn seed_9002788_a_boss_touching_a_bore_touching_the_stock_inside_leaves_its_caps_closed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::yz(4.0), Outline::circle([5.5, 1.0], 6.0), 7.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::yz(3.0000002),
                Outline::circle([5.5, 3.5], 3.5),
                7.0,
            )),
            Step::add(Leaf::prism(
                Plane::yz(2.9999902),
                Outline::circle([5.0, 3.5], 3.0),
                3.0,
            )),
        ],
    ));
}
