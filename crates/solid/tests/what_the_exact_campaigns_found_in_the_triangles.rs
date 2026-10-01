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

#[test]
fn seed_7000204_a_boss_touching_the_block_s_top_beside_a_cylinder_a_hair_inside_it_leaves_the_front_closed()
 {
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

/// The pocket, 1e-7 deep, pokes 1e-8 out of the block's side at x = 11; the
/// kernel decides it touches the side, and puts the vertex at the foot of
/// the line they touch along on the pocket's circle, at x = 11.00000001:
/// 1e-8 off the plane of the side it was decided on, ten times what the
/// rules tell apart. The side's triangles and the pocket's wall, a hair
/// high and so drawn by triangles lying nearly flat, both stand on that
/// vertex, and the wall's first triangle passes 1e-8 through the side
/// beside it. The triangles take the vertices where the kernel put them; a
/// vertex decided on a plane has to be put on it, which is the boolean's.
#[test]
#[ignore = "kernel"]
fn seed_6000059_a_pocket_a_hair_deep_touching_the_side_of_a_block_stays_uncrossed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(2.0),
            Outline::rectangle([3.0, -4.0], [11.0, 4.0]),
            -8.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::xy(1.9999999),
            Outline::circle([9.5, 0.0], 1.50000001),
            15.0,
        ))],
    ));
}

#[test]
fn seed_6002157_a_bore_a_hair_across_the_side_of_a_notch_stays_uncrossed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(1.0),
            Outline::rectangle([8.0, 5.0], [12.0, 7.0]),
            6.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::yz(1.00000006),
                Outline::rectangle([10.0000002, 3.75], [14.0000002, 7.75]),
                6.0,
            )),
            Step::cut(Leaf::prism(
                Plane::yz(-3.9999999920559044e-8),
                Outline::circle([8.0000002, 5.75], 2.00000001),
                11.0,
            )),
        ],
    ));
}

/// The boss, of radius 0.50000001 about (8, 4.5), passes 1e-8 beyond the
/// block's side at y = 4 and stands 2e-8 into its floor; the kernel decides
/// it touches the side, and puts the vertex where its top circle meets the
/// side at (8, 3.99999999, 4.00000002), on the circle and 1e-8 off the side's
/// plane — ten times what the rules tell apart — while the block's corner
/// beside it stands at (8, 4, 4). The boss's wall, fanned from that vertex,
/// passes through the block's floor between the two. As for seed 6000059,
/// the vertex has to be put on the plane it was decided on.
#[test]
#[ignore = "kernel"]
fn seed_8001749_a_boss_a_hair_across_the_side_it_stands_on_stays_uncrossed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(4.0),
            Outline::rectangle([10.0, 1.0], [16.0, 4.0]),
            3.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(4.0),
                Outline::rectangle([7.0, 2.0], [9.0, 4.0]),
                4.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(4.00000002),
                Outline::circle([8.0, 4.5], 0.50000001),
                -3.0,
            )),
        ],
    ));
}

#[test]
fn seed_9000909_two_cylinders_a_hair_across_each_other_side_by_side_stay_uncrossed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(6.0), Outline::circle([6.0, 7.0], 4.0), 5.0),
        vec![Step::add(Leaf::prism(
            Plane::xy(5.0),
            Outline::circle([0.5, 7.0], 1.50000001),
            10.0,
        ))],
    ));
}

#[test]
fn seed_9001168_a_bore_a_hair_out_of_the_cylinder_it_touches_inside_stays_uncrossed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::yz(35.0), Outline::circle([5.0, 45.0], 10.0), 33.0),
        vec![Step::cut(Leaf::prism(
            Plane::yz(35.00001),
            Outline::circle([7.5, 45.0], 7.50000005),
            65.0,
        ))],
    ));
}

#[test]
fn seed_9002637_a_block_a_hair_into_the_cylinder_it_rests_on_stays_uncrossed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(2.0), Outline::circle([1.0, 8.0], 5.5), 6.0),
        vec![Step::add(Leaf::prism(
            Plane::xy(2.0000000399999998),
            Outline::rectangle([-3.50000002, 13.49999998], [5.50000002, 22.50000002]),
            7.0,
        ))],
    ));
}

#[test]
fn seed_7003668_a_pocket_whose_corner_stands_a_hair_inside_the_wall_stays_under_its_chords() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(120.0),
            Outline::circle([225.0, 240.0], 75.0),
            -225.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::xy(-60.0),
            Outline::rectangle([165.0000018, 195.0], [405.0, 300.0]),
            60.0,
        ))],
    ));
}

#[test]
fn seed_11003014_two_cylinders_whose_walls_would_cross_on_the_grid_at_heights_apart_enclose_their_volume()
 {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(90.0), Outline::circle([90.0, 90.0], 60.0), -180.0),
        vec![Step::add(Leaf::prism(
            Plane::xy(210.0),
            Outline::circle([30.0, 150.0], 60.0),
            285.0,
        ))],
    ));
}

#[test]
fn seed_11000066_a_boss_swallowed_by_the_block_it_touches_a_bore_beside_leaves_the_face_it_marks_closed()
 {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(6.0),
            Outline::rectangle([-6.0, 4.0], [6.0, 16.0]),
            4.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(4.5),
                Outline::rectangle([0.0, 4.0], [8.0, 8.0]),
                6.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(5.0),
                Outline::circle([0.0, 1.0], 3.5),
                8.0,
            )),
            Step::add(Leaf::prism(
                Plane::xz(6.0),
                Outline::circle([-6e-8, 10.0], 5.5),
                4.0,
            )),
        ],
    ));
}

#[test]
fn seed_17000598_a_boss_a_hair_off_a_bore_touching_the_stock_inside_stays_uncrossed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(3.0), Outline::circle([4.0, 6.0], 1.5), -7.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(1.0),
                Outline::circle([5.0, 6.0], 0.5),
                14.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(1.49999),
                Outline::circle([5.00000006, 6.0], 0.5),
                -28.0,
            )),
        ],
    ));
}

#[test]
fn seed_16000754_a_boss_a_hair_inside_a_notched_cylinder_stays_uncrossed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(-2.0), Outline::circle([2.0, 9.5], 1.0), 7.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(-2.0),
                Outline::rectangle([1.0, 9.0], [2.0, 10.0]),
                13.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(-2.0),
                Outline::circle([1.50001, 9.5], 0.5),
                13.0,
            )),
        ],
    ));
}

#[test]
fn seed_17000616_a_boss_crossing_the_rim_of_a_wider_one_under_a_block_leaves_its_foot_closed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(90.0),
            Outline::rectangle([300.0, 180.0], [465.0, 405.0]),
            30.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(60.0),
                Outline::circle([465.0000003, 292.5], 15.0),
                225.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(60.0),
                Outline::rectangle([435.0, 278.0], [465.0, 308.0]),
                450.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(30.0),
                Outline::circle([450.0, 293.0], 30.0),
                75.0,
            )),
        ],
    ));
}

#[test]
fn seed_13000096_two_cylinders_of_one_radius_barely_apart_at_heights_apart_enclose_their_union() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(-5.0), Outline::circle([50.0, 10.0], 25.0), 23.0),
        vec![Step::add(Leaf::prism(
            Plane::xy(-5.0),
            Outline::circle([50.0000001, 10.0], 25.0),
            35.0,
        ))],
    ));
}

#[test]
fn seed_10001163_two_cylinders_of_one_radius_barely_apart_enclose_their_union() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(6.0), Outline::circle([6.0, 8.0], 6.0), 2.0),
        vec![Step::add(Leaf::prism(
            Plane::xy(5.0),
            Outline::circle([5.99999998, 8.0], 6.0),
            7.0,
        ))],
    ));
}

#[test]
fn seed_10003386_two_cylinders_one_wall_but_for_a_hair_enclose_their_union() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::yz(15.0), Outline::circle([27.5, 30.0], 22.5), 43.0),
        vec![Step::add(Leaf::prism(
            Plane::yz(15.0000003),
            Outline::circle([27.4999999, 30.0], 22.5),
            85.0,
        ))],
    ));
}

#[test]
fn seed_1006751_two_cylinders_of_one_radius_a_hair_apart_one_above_the_other_enclose_their_union() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(149.9999982),
            Outline::circle([165.0, 135.0], 75.0),
            45.0,
        ),
        vec![Step::add(Leaf::prism(
            Plane::xz(150.0000003),
            Outline::circle([164.9999997, 135.0], 75.0),
            -60.0,
        ))],
    ));
}

#[test]
fn seed_1008037_two_cylinders_of_one_radius_a_hair_apart_joined_enclose_their_union_once_the_tolerance_grows()
 {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xz(0.0), Outline::circle([1.0, 0.0], 4.5), 10.0),
        vec![
            Step::add(Leaf::prism(
                Plane::xz(0.0),
                Outline::circle([0.99999998, 0.0], 4.5),
                4.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(0.0),
                Outline::rectangle([4.0, 4.0], [7.0, 9.0]),
                20.0,
            )),
        ],
    ));
}

#[test]
fn seed_1023983_a_boss_filling_a_bore_but_for_a_hair_leaves_the_wall_the_bore_crossed_uncrossed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xz(-7.5), Outline::circle([30.0, 0.0], 15.0), 38.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::yz(10.0),
                Outline::circle([0.0, 0.0], 20.0),
                30.0,
            )),
            Step::add(Leaf::prism(
                Plane::yz(10.0),
                Outline::circle([1e-7, 0.0], 20.0),
                30.0,
            )),
        ],
    ));
}

#[test]
fn seed_1006613_a_boss_dipping_a_hair_into_a_disc_whose_wall_a_cut_crosses_is_drawn_closed_and_uncrossed()
 {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(6.0), Outline::circle([8.0, 3.0], 1.5), -3.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(6.0),
                Outline::circle([7.9999999, 3.0], 1.5),
                -2.49999994,
            )),
            Step::add(Leaf::prism(
                Plane::xz(6.0),
                Outline::circle([9.0, 5.0], 1.5),
                9.0,
            )),
        ],
    ));
}

#[test]
fn seed_1006613_a_boss_lying_on_a_disc_a_hair_thick_leaves_no_sample_where_it_grazes_the_disc_s_top()
 {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(6.0), Outline::circle([8.0, 3.0], 1.5), -2.5),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(6.0),
                Outline::circle([7.9999999, 3.0], 1.5),
                -2.49999994,
            )),
            Step::add(Leaf::prism(
                Plane::xz(6.0),
                Outline::circle([9.0, 5.0], 1.5),
                8.5,
            )),
        ],
    ));
}

#[test]
fn seed_20001018_two_cylinders_of_one_radius_a_hair_apart_joined_leave_their_cap_closed_where_they_cross()
 {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(-2.0),
            Outline::circle_from([9.0, 7.0], 6.0, 30.78758763175507),
            6.0,
        ),
        vec![Step::add(Leaf::prism(
            Plane::yz(-2.0),
            Outline::circle([8.99999998, 7.0], 6.0),
            12.0,
        ))],
    ));
}

#[test]
fn seed_22003491_a_block_notched_by_a_cylinder_a_hair_inside_its_side_leaves_its_top_closed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(15.0),
            Outline::rectangle([0.0, 40.0], [17.5, 80.0]),
            45.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::xy(10.0),
            Outline::circle([17.4999999, 60.0], 10.0),
            90.0,
        ))],
    ));
}

#[test]
fn seed_22005443_two_cylinders_a_hair_apart_joined_then_marked_by_a_cut_leave_their_caps_closed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(-2.0),
            Outline::circle_from([3.0, 7.0], 2.0, 337.60470308278093),
            4.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(-2.0),
                Outline::circle([3.00000001, 7.0], 2.0),
                3.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(-2.0),
                Outline::rectangle([1.5, 5.5], [4.5, 8.5]),
                -10.0,
            )),
        ],
    ));
}

#[test]
fn seed_23003087_a_bore_poking_a_hair_through_the_stock_it_touches_inside_leaves_the_caps_closed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(120.0),
            Outline::circle([210.0, 90.0], 165.0),
            165.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xz(150.0),
                Outline::circle([209.9999982, 90.0], 45.0),
                45.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(120.000006),
                Outline::circle([210.0, 150.0], 105.0000003),
                330.0,
            )),
        ],
    ));
}

#[test]
fn seed_28000951_a_bar_a_hair_proud_of_the_side_a_cylinder_touches_leaves_that_side_uncrossed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(20.0), Outline::circle([0.0, 30.0], 25.0), 15.0),
        vec![
            Step::add(Leaf::prism(
                Plane::yz(25.0),
                Outline::rectangle([15.0, 20.0], [45.0, 50.0]),
                15.0,
            )),
            Step::add(Leaf::prism(
                Plane::yz(24.99999),
                Outline::circle([29.99999, 35.0], 10.0),
                50.0,
            )),
        ],
    ));
}

#[test]
fn seed_29002495_a_disc_hollowed_from_below_a_hair_off_its_axis_keeps_its_rim_round_above() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(0.0), Outline::circle([120.0, 270.0], 90.0), 30.0),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(30.0),
                Outline::rectangle([255.0, 180.0], [405.0, 390.0]),
                180.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(-30.0),
                Outline::circle([119.99999, 270.0], 90.0),
                45.0,
            )),
        ],
    ));
}

#[test]
fn seed_31003842_a_circle_printed_a_hair_past_the_corner_of_a_pad_leaves_the_top_closed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(5.0),
            Outline::rectangle([10.0, 4.0], [16.0, 11.0]),
            -1.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(5.0),
                Outline::rectangle([10.0, 5.0], [13.0, 8.0]),
                -1.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(5.0),
                Outline::circle([15.00000002, 6.5], 2.5),
                6.0,
            )),
        ],
    ));
}

#[test]
fn seed_32000053_a_circle_printed_round_a_hole_it_touches_inside_leaves_the_top_closed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(0.0),
            Outline::rectangle([7.0, 0.0], [14.0, 7.0]),
            -4.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(-0.0),
                Outline::circle([10.00001, 3.5], 1.5),
                -7.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(0.0),
                Outline::circle([10.0, 5.0], 3.0),
                3.0,
            )),
        ],
    ));
}

/// The cylinder, swallowed by the block, leaves its circle on the block's
/// side x = 7, and that circle touches the edge of the pad standing 2e-8
/// off the same side, y = 7, at (7, 7, 1): the cylinder was tangent to the
/// pad's top there. The kernel puts no vertex at that point — the gap of
/// 2e-8 fell under the tolerance only once the block grew the reach
/// (failure 1-6) — and the side's face touches itself there between two of
/// its edges. A sample of the circle lies on the pad's edge, and no sweep
/// can lay out a boundary touching itself away from a vertex. The point has
/// to be a vertex of both edges, which is the boolean's.
#[test]
#[ignore = "kernel"]
fn seed_34002812_a_swallowed_cylinder_tangent_to_a_pad_a_hair_off_the_side_leaves_the_side_closed()
{
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::yz(7.0), Outline::circle([3.0, 1.0], 4.0), 10.0),
        vec![
            Step::add(Leaf::prism(
                Plane::yz(6.99999998),
                Outline::rectangle([7.0, 0.0], [11.0, 2.0]),
                -8.0,
            )),
            Step::add(Leaf::prism(
                Plane::yz(7.0),
                Outline::rectangle([-1.0, -6.0], [11.0, 6.0]),
                20.0,
            )),
        ],
    ));
}

/// Two cylinders of one radius side by side touch along x = 7, y = 8.5, and
/// a third, 2e-8 off the first's axis, touches the second along a line
/// 1e-8 beside it. The kernel holds the first and third apart but makes
/// their two lines of contact one by distance, and puts the vertex at the
/// top of the third's on its own line, 1e-8 off the first's (failure 1-8):
/// the third's face is bounded along the line by a kink 6e-9 deep. Its
/// sliver over the kink lies on the plane the second wall is tangent to
/// there, and within what the rules tell apart of the second wall's
/// triangle along the line, folded onto it. The line has to be one line,
/// which is the boolean's.
#[test]
#[ignore = "kernel"]
fn seed_30000434_a_cylinder_a_hair_off_one_touching_its_neighbour_leaves_the_neighbour_uncrossed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(4.0), Outline::circle([7.0, 5.0], 3.5), 5.0),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(4.0),
                Outline::circle([7.0, 12.0], 3.5),
                3.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(4.00001),
                Outline::circle([6.99999998, 5.0], 3.5),
                5.0,
            )),
        ],
    ));
}

#[test]
fn seed_38002065_a_disc_hollowed_from_below_a_hair_off_its_axis_keeps_its_sliver_uncrossed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(38.0), Outline::circle([20.0, 25.0], 12.5), 5.0),
        vec![Step::cut(Leaf::prism(
            Plane::xy(37.5000001),
            Outline::circle([20.0000001, 25.0], 12.5),
            5.0,
        ))],
    ));
}

/// The cylinder, of radius 74.9999997 about (165, 60), passes 3e-7 inside the
/// corner (210, 0) of the bar joined to it, under the kernel's tolerance,
/// 3.6e-7. The kernel keeps the corner off the cylinder and puts a vertex
/// where the circle crosses the bar's side x = 210, 3.75e-7 above the
/// corner, lying on the front plane alone, the side being gone. One of the
/// front's faces then passes through both twice, along an arc and a line
/// between them, each about the tolerance long: a loop pinched round a
/// sliver of no area, which no sweep can lay out, and the face is left open.
/// The corner and the crossing have to be one point or two decided apart,
/// which is the boolean's.
#[test]
#[ignore = "kernel"]
fn seed_25002495_a_bar_whose_corner_stands_a_hair_off_the_cylinder_it_joins_leaves_the_front_closed()
 {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(180.0),
            Outline::rectangle([-120.0, -120.0], [240.0, 240.0]),
            195.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xz(180.0),
                Outline::circle([165.0, 60.0], 74.9999997),
                390.0,
            )),
            Step::add(Leaf::prism(
                Plane::xz(180.0),
                Outline::rectangle([210.0, 0.0], [300.0, 75.0]),
                195.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(180.0),
                Outline::rectangle([225.0, 15.0], [360.0, 60.0]),
                150.0,
            )),
        ],
    ));
}

/// The cut leaves a sliver of the first cylinder 5e-8 thick, more than the
/// kernel's tolerance then, 4.5e-8. The block reaches further, and the
/// tolerance grows to 5.5e-8 (failure 1-6): its side x = 45 is decided to
/// touch both walls of the sliver, along two lines 5e-8 apart that are never
/// made one. The side holds both as slits, one a hair beside the other,
/// which lie along the same line of its parameters: no sweep can lay that
/// out, and the side is left open. The two lines have to be one, which is
/// the boolean's.
#[test]
#[ignore = "kernel"]
fn seed_1026142_a_block_touching_both_walls_of_a_sliver_the_tolerance_outgrew_is_drawn_closed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xz(32.5), Outline::circle([40.0, 35.0], 5.0), 23.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xz(37.5),
                Outline::circle([39.99999995, 35.0], 5.0),
                23.0,
            )),
            Step::add(Leaf::prism(
                Plane::xz(37.4999999),
                Outline::rectangle([45.0, 30.0], [55.0, 40.0]),
                23.0,
            )),
        ],
    ));
}

/// The boss, of radius 0.49999999 about z = 8, stands 1e-8 over the top of
/// the second block, z = 7.5, and is decided to touch it along x =
/// 12.5000002; the first block's side x = 12.5 crosses the boss's wall 2e-7
/// from that line, 1e-8 over the top, and the kernel puts the corner there
/// on the side, the top and the boss's wall alike (failure 1-2). The strip
/// of the wall between that corner and the line is then a face lying on the
/// top: the triangles of both lie on each other, however they are cut. The
/// corner has to stay off one of them, or the strip go, which is the
/// boolean's.
#[test]
#[ignore = "kernel"]
fn seed_1016356_a_boss_a_hair_over_a_block_it_touches_beside_a_side_stays_uncrossed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(-1.5),
            Outline::rectangle([8.0, 0.0], [12.5, 8.0]),
            2.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xz(-0.5),
                Outline::rectangle([9.0, 0.5], [16.0, 7.5]),
                7.0,
            )),
            Step::add(Leaf::prism(
                Plane::xz(-0.5),
                Outline::circle([12.5000002, 8.0], 0.49999999),
                14.0,
            )),
        ],
    ));
}

/// The block's floor, z = 30, touches the boss along x = 45.0000003; the
/// first block's side x = 45 crosses the boss's wall 3e-7 from that line,
/// 1.8e-14 under the floor, and the corner there is put on the side, the
/// floor and the wall (failure 1-2). The strip of the wall between that
/// corner and the line, and the strip of the floor over it, are two faces
/// lying on each other, and their triangles are the same triangle twice.
/// As for seed 1016356, the boolean has to keep the corner off one of them.
#[test]
#[ignore = "kernel"]
fn seed_1016543_a_block_resting_on_a_boss_beside_the_side_it_touches_stays_uncrossed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(-5.0),
            Outline::rectangle([10.0, 15.0], [45.0, 40.0]),
            15.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xz(1e-6),
                Outline::circle([45.0000003, 27.5], 2.5),
                30.0,
            )),
            Step::add(Leaf::prism(
                Plane::xz(10.0),
                Outline::rectangle([35.0, 30.0], [53.0, 50.0]),
                45.0,
            )),
        ],
    ));
}
