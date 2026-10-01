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
/// It holds since a touch decided a hair apart is made exact on the
/// surfaces, the second operand's moved onto it (decision 2).
#[test]
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
/// It holds since a touch decided a hair apart is made exact on the
/// surfaces, the second operand's moved onto it (decision 2).
#[test]
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

/// Its triangles were fixed in round 2, and then the kernel stopped
/// answering it: two cylinders of one radius crossing at a grazing angle
/// (failure 1-9), whose tolerance a later leaf then grows (1-6), fail the
/// check of their own listing at the second step once round 2's refusals
/// keep apart what the first step decided. The kernel's, still open.
/// It holds since a pair of surfaces one operand alone carries is decided
/// at that operand's tolerance, and two arcs between the same corners are
/// one only where each stands within the tolerance of the other's arc, not
/// of its whole curve: the half of one post's rim and the other half of the
/// second's, a hair apart, were taken for one arc.
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

/// Its triangles were fixed in round 2, and then the kernel stopped
/// answering it: two cylinders of one radius crossing at a grazing angle
/// (failure 1-9), whose tolerance a later leaf then grows (1-6), fail the
/// check of their own listing at the second step once round 2's refusals
/// keep apart what the first step decided. The kernel's, still open.
/// It holds since a pair of surfaces one operand alone carries is decided
/// at that operand's tolerance, and two parallel cylinders of one operand
/// covering twins are read one above the other where they stand apart.
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
/// It holds since the boolean merges the faces of one surface and one side
/// across an arc nothing else uses, and joins two edges of one curve at a
/// vertex only they reach (step 8).
#[test]
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

#[test]
fn seed_39000224_a_crescent_left_by_a_cut_a_hair_off_the_axis_under_a_notch_closes_its_cap() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::yz(7.0), Outline::circle([5.5, 5.0], 1.0), 3.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::yz(7.0),
                Outline::circle([5.5, 5.5], 0.50000006),
                8.0,
            )),
            Step::cut(Leaf::prism(
                Plane::yz(5.99999994),
                Outline::circle([5.50001, 5.0], 1.0),
                5.0,
            )),
        ],
    ));
}

#[test]
fn seed_37000855_a_bore_filled_but_for_a_crescent_by_a_boss_it_touches_inside_stays_uncrossed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(120.0),
            Outline::rectangle([-180.0, -60.0], [180.0, 300.0]),
            150.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(120.0),
                Outline::circle([0.0, 165.0], 135.00001),
                300.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(120.0),
                Outline::circle([-1e-5, 165.0], 135.0),
                300.0,
            )),
        ],
    ));
}

/// A cut of the stock's radius 6e-6 off its axis leaves a crescent of it,
/// and a wide boss stands on the crescent, touching the stock inside at
/// (60, -15). The crescent's two walls cross at (59.999997, -15), 3e-6 from
/// that point, and the boss's floor is bounded there by both: its own
/// circle down to the point of contact, the stock's for 3e-6, then the
/// cut's. The boss's circle used to withhold every ray where it stands
/// within the tolerance of the stock's wall, the crossing's too, and its
/// chord into the point of contact passed 1.2e-7 inside the crossing, the
/// cut's chord from the crossing through it. It holds since a wall keeps
/// the rays through its own vertices inside another's room and beside an
/// arc's end, a place within rounding of a vertex is that vertex, and the
/// sweep follows a hair running out of the region: without any one of
/// them, the floor is left open again.
#[test]
fn seed_35000999_a_boss_touching_a_crescent_inside_beside_its_tip_leaves_its_floor_closed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(30.0), Outline::circle([60.0, 0.0], 15.0), -270.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(0.0),
                Outline::circle([59.999994, 0.0], 15.0),
                -540.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(-45.0),
                Outline::circle([60.0, 60.0], 75.0),
                135.0,
            )),
        ],
    ));
}

/// The cut only touches the bottom of the block and of the boss on it, and
/// prints its circle there, of the boss's radius 1e-5 off its axis. The two
/// circles cross 5e-6 from the block's side, and the kernel keeps both
/// arcs between the crossing and the side though only the bottom face uses
/// them, on both sides: the face's loop runs round the lens between them
/// twice, once each way, and with no sample between their ends the two
/// arcs are one segment four times over, which no sweep can lay out. An
/// arc a face uses on both sides and nothing else uses has to go when the
/// faces beside it are merged, which is the boolean's.
/// It held from the merge of the kernel's round 2, before step 8 was
/// written, and stayed ignored until round 3 found it holding.
#[test]
fn seed_36003534_a_circle_printed_a_hair_off_a_boss_s_own_leaves_the_bottom_closed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(17.5),
            Outline::rectangle([2.5, 10.0], [12.5, 42.5]),
            8.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(17.5),
                Outline::circle([12.5, 26.25], 15.0),
                50.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(17.5),
                Outline::circle([12.49999, 26.25], 15.0),
                -13.0,
            )),
        ],
    ));
}

#[test]
fn seed_32000053_a_pocket_whose_floor_a_hole_touches_inside_leaves_the_floor_closed() {
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
                -3.0,
            )),
        ],
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
/// It held from the merge of the kernel's round 2, before step 8 was
/// written, and stayed ignored until round 3 found it holding.
#[test]
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
/// It holds since the boolean merges the faces of one surface and one side
/// across an arc nothing else uses, and joins two edges of one curve at a
/// vertex only they reach (step 8).
#[test]
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
/// boolean's. Decision 7 keeps both, as the exact geometry has them: the
/// strip of the wall is part of the wall's region, whose point stands
/// beyond the band, and the top's region runs on past the corner, so no
/// strip of either is read on its own.
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
/// As for seed 1016356, the boolean has to keep the corner off one of them;
/// decision 7 keeps both, each strip part of a region read beyond the band.
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

#[test]
fn seed_3224477_a_cut_of_one_radius_a_hair_and_a_half_off_the_axis_leaves_the_cap_above_it_round() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(-1.0), Outline::circle([8.0, 1.0], 6.0), 9.0),
        vec![Step::cut(Leaf::prism(
            Plane::xy(-1.0),
            Outline::circle([8.00000002, 1.0], 6.0),
            8.0,
        ))],
    ));
}

#[test]
fn seed_3044779_a_crescent_a_hair_wide_under_a_notch_leaves_the_disc_above_it_round() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(-1.0), Outline::circle([3.0, 6.0], 2.0), 9.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(-2.0),
                Outline::circle([2.99999999, 6.0], 2.0),
                9.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(0.0),
                Outline::rectangle([1.0, 5.0], [9.0, 9.0]),
                7.0,
            )),
        ],
    ));
}

#[test]
fn seed_3282114_a_boss_a_hair_off_the_hole_it_fills_keeps_its_rim_round_above_the_block() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(2.0),
            Outline::rectangle([9.0, 0.0], [15.0, 5.0]),
            9.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(2.0),
                Outline::circle([7.0, 0.0], 6.0),
                2.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(2.0),
                Outline::circle([7.00000002, 0.0], 6.0),
                4.0,
            )),
        ],
    ));
}

#[test]
fn seed_3217438_a_circle_a_hair_off_the_rim_printed_on_a_disc_s_cap_leaves_its_other_rim_round() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xz(2.0), Outline::circle([1.0, 4.0], 2.0), -1.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xz(1.9999998),
                Outline::rectangle([1.0, 4.0], [2.0, 5.0]),
                2.5,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(3.0),
                Outline::circle([1.00000001, 4.0], 2.0),
                -6.5,
            )),
        ],
    ));
}

#[test]
fn seed_3174852_a_bore_touching_a_boss_inside_keeps_its_rim_above_the_block_off_the_boss() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(4.0),
            Outline::rectangle([-1.0, 3.0], [11.0, 15.0]),
            2.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(3.0),
                Outline::circle([5.0, 9.0], 3.0),
                2.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(3.0),
                Outline::circle([5.0000001, 9.5], 2.5),
                3.0,
            )),
        ],
    ));
}

#[test]
fn seed_3174852_two_bores_one_a_hair_inside_the_other_touching_it_leave_the_block_whole_round_them()
{
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(3.0),
            Outline::rectangle([2.0, 6.0], [8.0, 12.0]),
            2.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(3.0),
                Outline::circle([5.0, 9.5], 2.4999999),
                3.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(3.00001),
                Outline::circle([5.0000001, 9.5], 2.5),
                3.0,
            )),
        ],
    ));
}

#[test]
fn seed_3115386_a_bore_a_hair_narrower_than_its_stock_on_one_axis_leaves_the_stock_round_once_the_tolerance_grows()
 {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(1.0), Outline::circle([6.0, 9.0], 0.50000001), 6.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(-0.0),
                Outline::circle([6.0, 9.0], 0.5),
                3.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(1.0),
                Outline::rectangle([7.0, 4.0], [11.0, 7.0]),
                6.0,
            )),
        ],
    ));
}

#[test]
fn seed_3175702_a_bore_touching_two_others_a_hair_from_their_line_leaves_the_caps_closed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(1.0), Outline::circle([8.0, 10.0], 4.5), 4.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(0.0),
                Outline::circle([12.0, 10.0], 0.50000002),
                4.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(1.0),
                Outline::circle([11.0, 10.0], 1.5),
                7.0,
            )),
        ],
    ));
}

#[test]
fn seed_3049074_three_bores_touching_inside_along_one_line_under_a_notch_leave_the_caps_closed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(-2.0), Outline::circle([8.0, 9.0], 2.5), 2.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(-3.0),
                Outline::circle([9.0, 9.0], 1.5),
                4.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(-2.0),
                Outline::circle([10.0, 9.0], 0.50000006),
                2.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(-3.0),
                Outline::rectangle([5.0, 9.0], [10.0, 16.0]),
                2.0,
            )),
        ],
    ));
}

#[test]
fn seed_3291902_a_cylinder_a_hair_below_and_aside_one_it_joins_keeps_its_rim_round_over_the_skirt()
{
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(5.0), Outline::circle([45.0, 0.0], 27.5), 28.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(5.0),
                Outline::rectangle([38.0, -8.0], [53.0, 8.0]),
                55.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(4.9999999),
                Outline::circle([44.9999999, 0.0], 27.5),
                48.0,
            )),
        ],
    ));
}

#[test]
fn seed_3112136_a_slot_whose_corners_stand_in_a_skin_under_a_bore_s_cap_leaves_the_skin_uncrossed()
{
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(-1.0), Outline::circle([5.0, 0.0], 3.0), 5.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(-1.00000002),
                Outline::circle([4.5, 0.0], 2.50000001),
                5.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(-2.0),
                Outline::rectangle([0.0, 10.0], [6.0, 16.0]),
                9.0,
            )),
        ],
    ));
}

#[test]
fn seed_3157560_a_boss_a_hair_across_a_bore_and_the_stock_it_touches_leaves_the_floor_closed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(-2.0), Outline::circle([2.0, 8.5], 4.0), 9.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(-2.0),
                Outline::circle([2.0, 9.5], 3.0),
                4.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(-2.0),
                Outline::circle([2.0, 11.5], 1.00000002),
                7.0,
            )),
        ],
    ));
}

#[test]
fn seed_3186142_a_boss_a_hair_across_two_cylinders_touching_inside_leaves_their_caps_closed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::yz(0.0), Outline::circle([3.0, 8.0], 5.0), 10.0),
        vec![
            Step::add(Leaf::prism(
                Plane::yz(-1.0),
                Outline::circle([4.0, 8.0], 4.0),
                10.0,
            )),
            Step::add(Leaf::prism(
                Plane::yz(0.0),
                Outline::circle([6.5, 8.0], 1.50000002),
                10.0,
            )),
        ],
    ));
}

#[test]
fn seed_3149531_a_boss_touching_a_crescent_inside_a_hair_from_its_tip_leaves_the_floor_closed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(5.0), Outline::circle([35.0, 5.0], 30.0), 48.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(-5.0),
                Outline::circle([34.99999, 5.0], 30.0),
                18.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(-5.0),
                Outline::circle([34.99999, 25.0], 10.0000001),
                35.0,
            )),
        ],
    ));
}

#[test]
fn seed_3102925_a_bore_a_hair_across_a_boss_touching_the_stock_inside_leaves_the_caps_closed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::yz(-1.0), Outline::circle([9.0, 7.0], 4.0), 7.0),
        vec![
            Step::add(Leaf::prism(
                Plane::yz(-0.99999994),
                Outline::circle([9.0, 10.0], 1.0),
                13.0,
            )),
            Step::cut(Leaf::prism(
                Plane::yz(-1.0000001),
                Outline::circle([9.0, 10.5], 0.50000002),
                6.0,
            )),
        ],
    ));
}

#[test]
fn seed_3180403_a_boss_touching_the_stock_inside_printed_on_the_floor_keeps_inside_the_stock_s_rim()
{
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(1.0), Outline::circle([6.0, 3.0], 2.5), 7.0),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(1.0),
                Outline::circle([8.0, 3.0], 0.5),
                1.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(1.00000002),
                Outline::circle([7.0, 3.0], 1.49999998),
                13.0,
            )),
        ],
    ));
}

#[test]
fn seed_3225040_a_bore_a_hair_inside_a_lying_cut_it_nearly_touches_under_a_bar_leaves_the_walls_closed()
 {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(1.0),
            Outline::rectangle([1.0, 2.0], [10.0, 11.0]),
            7.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::yz(1.0),
                Outline::circle([6.0, 0.0], 5.0),
                4.0,
            )),
            Step::cut(Leaf::prism(
                Plane::yz(0.9999999),
                Outline::circle([6.0, 1.5], 3.4999999),
                7.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(2.0),
                Outline::rectangle([0.0, 3.0], [6.0, 5.0]),
                10.0,
            )),
            Step::cut(Leaf::prism(
                Plane::yz(0.0),
                Outline::circle([6.0, 3.0], 2.0),
                4.0,
            )),
        ],
    ));
}

#[test]
fn seed_3008607_a_cut_a_hair_off_the_stock_s_axis_beside_a_boss_touching_it_inside_leaves_the_crescent_closed()
 {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(15.0), Outline::circle([50.0, 15.0], 25.0), 30.0),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(15.000001),
                Outline::circle([62.5, 15.0], 12.5),
                60.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(15.0),
                Outline::circle([49.9999997, 15.0], 25.0),
                20.0,
            )),
        ],
    ));
}

#[test]
fn seed_3011236_a_bore_a_hair_narrower_than_the_boss_it_runs_through_touching_the_stock_leaves_the_rings_closed()
 {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(210.0), Outline::circle([30.0, 90.0], 90.0), 45.0),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(180.0),
                Outline::circle([-30.0, 135.0], 15.000005999999999),
                180.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(180.0),
                Outline::circle([-30.0, 135.0], 15.0),
                360.0,
            )),
        ],
    ));
}

#[test]
fn seed_3092335_a_bore_crossing_two_lying_cylinders_a_hair_apart_keeps_its_curve_off_the_step_between_them()
 {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xz(240.0), Outline::circle([150.0, 0.0], 75.0), 210.0),
        vec![
            Step::add(Leaf::prism(
                Plane::xz(240.00001),
                Outline::circle([149.9999997, 0.0], 75.0),
                75.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(30.0),
                Outline::circle([90.000006, 90.0], 75.00001),
                240.0,
            )),
        ],
    ));
}

#[test]
fn seed_3160874_a_stock_touched_inside_by_a_bore_and_a_cut_a_hair_through_it_keeps_its_top_closed()
{
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(-60.0), Outline::circle([90.0, 240.0], 60.0), 90.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(-60.0000003),
                Outline::circle([75.0, 240.0], 45.0000003),
                90.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(-90.0),
                Outline::circle([45.0, 240.0], 15.0),
                180.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(150.0),
                Outline::rectangle([150.0, 210.0], [345.0, 300.0]),
                285.0,
            )),
        ],
    ));
}

#[test]
fn seed_3160874_a_bore_a_hair_wider_than_the_line_two_walls_touch_along_leaves_the_caps_closed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(-60.0), Outline::circle([90.0, 240.0], 60.0), 90.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(-60.0),
                Outline::circle([75.0, 240.0], 45.0),
                90.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(-90.0),
                Outline::circle([45.0, 240.0], 15.000000300000004),
                180.0,
            )),
        ],
    ));
}

#[test]
fn seed_3194537_a_bore_through_a_stock_it_touches_inside_a_hair_off_its_axis_leaves_the_crescent_closed()
 {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(-1.0),
            Outline::circle([-1.5, 0.5], 5.00000001),
            4.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::xy(-1.0),
            Outline::circle([-1.49999999, 0.5], 5.0),
            4.0,
        ))],
    ));
}

#[test]
fn seed_3194537_a_bore_through_a_stock_it_touches_inside_a_hair_off_its_axis_aslant_leaves_the_crescent_uncrossed()
 {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(-1.0),
            Outline::circle([-1.5, 0.5], 3.00000001),
            4.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::xy(-1.0),
            Outline::circle([-1.499999993, 0.500000007], 3.0),
            4.0,
        ))],
    ));
}

/// A bore touching its stock inside, nearly concentric with it: their
/// radii and their axes a hair apart, so that the two walls stand within a
/// fifth of the kernel's tolerance of each other over three steps either
/// side of the line they touch along. Those steps are withheld from the
/// circles bounding the stretch the two face each other over — the stock's
/// floor rim — and the stock's wall, going on alone above the bore's cap,
/// is drawn from that rim's chord, four steps long and sagging fourteen
/// times the tolerance, up to its top rim: the triangles leave out a sliver
/// of the stock's matter over the height above the bore. Withholding steps
/// cannot draw it: the stock's wall needs the steps at the height the bore
/// ends, inside its face, or the two walls drawn apart by what the rules
/// tell apart rather than withheld where they stand closer.
///
/// Round 5 tried the steps back on the stock's top rim alone, the bore no
/// longer taken to stand on the stock's hollow side only beyond a tenth of
/// the tolerance: half the matter comes back, the wall still drawn from
/// the floor rim's chord up to the top, and seed 17000598 crosses again.
/// What draws it is a seam inside the stock's face at the bore's ceiling:
/// the floor rim's chord again there and, a hair above it, the whole grid,
/// both slits the sweep cuts along, so that the wall below lies on the
/// bore's, the wall above stands on its own surface, and the thin band
/// between closes the step. Its radii more than a tolerance apart, the
/// two walls touch inside rather than make one wall: taking walls of one
/// radius a hair off one axis for one (decision 8) would not reach it.
#[test]
#[ignore = "triangles"]
fn seed_3194537_a_bore_touching_its_stock_inside_a_hair_off_its_axis_keeps_the_stock_s_wall_round_above_it()
 {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(-1.0),
            Outline::circle([-1.5, 0.5], 2.00000001),
            8.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::xy(-1.0),
            Outline::circle([-1.50000001, 0.5], 2.0),
            4.0,
        ))],
    ));
}

#[test]
fn seed_3079219_a_bore_touching_a_cross_cut_at_a_point_a_hair_into_it_stays_uncrossed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(35.0),
            Outline::rectangle([45.0, 10.0], [63.0, 43.0]),
            35.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::yz(35.0),
                Outline::circle([62.49999995, 26.25], 2.5),
                35.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(5.0),
                Outline::circle([43.0, 45.0], 15.0),
                25.0,
            )),
        ],
    ));
}

#[test]
fn seed_3047709_a_boss_crossing_a_cut_of_one_radius_a_hair_off_a_far_plane_keeps_its_wall_round() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(7.0),
            Outline::rectangle([1.0, 8.0], [4.5, 15.0]),
            10.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xz(8.0000002),
                Outline::circle([4.500009899999999, 11.5], 3.0),
                19.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(6.99999),
                Outline::circle([4.5, 11.5], 3.0),
                3.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(-2.0),
                Outline::rectangle([10.0, 6.0], [13.0, 11.0]),
                6.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(-1.0),
                Outline::rectangle([7.0, 8.0], [11.0, 12.0]),
                9.5,
            )),
        ],
    ));
}

#[test]
fn seed_3138713_three_walls_touching_along_one_line_and_a_circle_printed_round_one_of_them_leave_the_top_closed()
 {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(-2.0), Outline::circle([9.0, 4.0], 4.0), 8.0),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(-2.0),
                Outline::circle([7.5, 4.0], 2.5),
                8.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(6.0),
                Outline::circle([8.0, 2.0], 5.0),
                3.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(6.0),
                Outline::circle([8.0, 4.0], 3.0),
                5.0,
            )),
        ],
    ));
}

#[test]
fn seed_53006876_a_bar_touching_a_post_at_a_point_keeps_both_walls_round() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(30.0),
            Outline::circle([300.0, 300.0], 30.0),
            285.0,
        ),
        vec![Step::add(Leaf::prism(
            Plane::xz(240.0),
            Outline::circle([180.0, 150.0], 90.0),
            -135.0,
        ))],
    ));
}

#[test]
fn seed_53006876_a_bar_a_hair_into_a_post_it_touches_at_a_point_stays_uncrossed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(30.0),
            Outline::circle([300.0, 300.0], 30.0),
            285.0,
        ),
        vec![Step::add(Leaf::prism(
            Plane::xz(240.0),
            Outline::circle([180.00000005, 150.0], 90.0),
            -135.0,
        ))],
    ));
}

/// A cut and a boss of one radius, their axes 1.25 tolerances apart, cross
/// along two lines; the boss fills the cut back but for a sliver over the
/// heights both stand at, beside the line they cross along at the block's
/// bottom. The steps beside that line are withheld from the circles
/// bounding that stretch, and the cut's wall, going on alone through the
/// block below it, is drawn from those circles' chords across two steps
/// down to its rim on the block's side: its triangles sag three and a half
/// times the tolerance, and the volume falls short. As for seed 3194537,
/// withholding steps cannot draw a wall that faces another over part of
/// its height only. Of one radius a hair off one axis, the two walls are
/// for the kernel to take for one (decision 8), which leaves nothing for
/// the cut's wall to face; the seam seed 3194537 describes would draw it
/// as it stands.
#[test]
#[ignore = "triangles"]
fn seed_3033420_a_cut_and_a_boss_of_one_radius_a_hair_off_one_axis_keep_the_cut_s_wall_round_below_the_boss()
 {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(90.0),
            Outline::rectangle([135.0, 30.0], [405.0, 300.0]),
            -150.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xz(120.0),
                Outline::circle([269.9999994, 180.0], 135.0),
                225.0,
            )),
            Step::add(Leaf::prism(
                Plane::xz(89.999994),
                Outline::circle([270.0, 180.0], 135.0),
                -300.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(180.0),
                Outline::rectangle([120.0, 270.0], [255.0, 300.0]),
                300.0,
            )),
        ],
    ));
}

/// A floor cut a hair above a disc's bottom, then the disc's twin a hair
/// off its axis added from below: the two walls are taken for one, and the
/// two floors for one plane, but the two rims stay two circles, one a hair
/// higher, their centres a hair apart, with no vertex where they cross.
/// The kernel leaves a ring of the floor between them, and two circles of
/// one radius crossing each other bound no ring: its outline crosses
/// itself at both crossings, and no sampling of the two can sweep it. Two
/// circles of one wall on one plane want to be one circle, by support.
///
/// It holds since fix round 3's merge, and stayed ignored until round 5's
/// review found it held: two walls of one radius a hair off one axis, it
/// stands guard over decision 8, which takes such walls for one.
#[test]
fn seed_3243921_a_disc_floored_a_hair_up_and_joined_to_its_twin_a_hair_aside_leaves_the_floor_closed()
 {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(1.0), Outline::circle([6.0, 3.0], 4.5), -3.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(1e-8),
                Outline::rectangle([-1.0, -4.0], [13.0, 10.0]),
                -6.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(0.0),
                Outline::circle([5.99999999, 3.0], 4.5),
                6.0,
            )),
        ],
    ));
}

/// A stock and a cut of one radius a hair apart leave a sliver a
/// tolerance thick, crossing along two lines; a notch cut from it is
/// decided to leave its side as a line where both walls meet, a hair from
/// one of the crossing lines. Every circle of both walls is then sampled
/// at that line's angle where the kernel put its vertex, so the two walls
/// meet along it at every height, and between it and the crossing line
/// they are drawn on the same two rulings: the floor's outline runs out to
/// the crossing and back on itself as a hair, and the two walls lie on each
/// other there. The sliver beyond the notch's side is thinner than what the
/// rules tell apart; drawing it wants the two walls apart by that much
/// rather than pinched along the notch's side at every height. Of one
/// radius a hair off one axis, the two walls are for the kernel to take
/// for one (decision 8), which leaves no sliver (1-17).
#[test]
#[ignore = "triangles"]
fn seed_3155418_a_notch_cut_from_a_sliver_a_hair_from_where_its_walls_cross_leaves_its_floor_closed()
 {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(1.0), Outline::circle([7.0, 6.0], 2.5), 10.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(0.0),
                Outline::circle([7.00000002, 6.0], 2.5),
                19.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(4.0),
                Outline::rectangle([2.0, 3.0], [6.5, 5.0]),
                3.0,
            )),
        ],
    ));
}

/// Its triangles were fixed in round 3 against the kernel round 3 started
/// from; merged with round 3's kernel, the kernel declines it instead: a bore
/// tangent inside another, crossed by a cut of one radius a hair off, is the
/// band of 1-9 and 2-1. Decision 7 now winds the strips of the inner bore's
/// wall the cut lies over, beside the band, as the arena decided the bores:
/// the inner inside the cut it touches, though rounding leaves the strip's
/// point 4e-15 outside the cut, where a ray says so. The listing then comes
/// out with three edges used unevenly, two of them three times: the faces
/// kept along the lines the three walls touch and cross along in the band
/// do not close. Still the kernel's.
#[test]
#[ignore = "kernel"]
fn seed_3108235_a_bore_touching_another_inside_where_a_cut_of_one_radius_crosses_it_leaves_the_side_closed()
 {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(2.0),
            Outline::rectangle([1.0, 2.0], [7.0, 10.0]),
            -6.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::yz(1.99999),
                Outline::circle([7.0, 6.0], 3.0),
                6.0,
            )),
            Step::cut(Leaf::prism(
                Plane::yz(0.9999899),
                Outline::circle([7.0, 6.5], 2.5),
                6.0,
            )),
            Step::cut(Leaf::prism(
                Plane::yz(2.00001),
                Outline::circle([7.00000006, 6.0], 3.0),
                -12.0,
            )),
        ],
    ));
}

#[test]
fn seed_3061547_a_twin_a_hair_aside_joined_over_a_bore_touching_the_stock_inside_leaves_the_end_closed()
 {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::yz(3.0), Outline::circle([0.0, 4.0], 4.0), 9.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::yz(2.9999997),
                Outline::circle([0.0, 3.0], 3.0),
                8.0,
            )),
            Step::add(Leaf::prism(
                Plane::yz(3.0),
                Outline::circle([2e-8, 4.0], 4.0),
                8.0,
            )),
        ],
    ));
}

/// A post and a bore of one radius, their axes 30 tolerances apart, leave
/// a sliver beside the post's wall up to the bore's ceiling, and a lying
/// cut takes the post's top off above it. The post's wall faces the bore
/// over the sliver's height only, and goes on alone above the ceiling: its
/// strip beside the line the two cross along is one triangle from the
/// sliver's floor, through the vertex where that line meets the ceiling,
/// up to the curve the lying cut meets the post along. That triangle's
/// section at the ceiling's height chords inside the post and passes
/// through the ceiling's triangle reaching the same vertex: the post's
/// wall has no sample at the ceiling's height at that angle, its edge
/// there being the bore's rim, a hair inside. As for seed 3194537, a wall
/// facing another over part of its height only wants samples inside its
/// face where the facing ends. Of one radius, the axes 30 tolerances
/// apart, the two walls are for the kernel to take for one (decision 8)
/// if it reaches that far; the seam seed 3194537 describes would draw it
/// as it stands.
#[test]
#[ignore = "triangles"]
fn seed_3202168_a_post_hollowed_a_hair_off_its_axis_below_a_lying_cut_stays_uncrossed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(60.0),
            Outline::circle([270.0, 180.0], 15.0),
            195.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(60.0),
                Outline::circle([270.00001, 180.0], 15.0),
                60.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(210.0),
                Outline::circle([180.0, 210.0], 120.0),
                90.0,
            )),
        ],
    ));
}

/// A crescent 6e-7 thick, left by a cut of the stock's radius a hair off
/// its axis, touches the side x = 45 of a box joined to it along y = 300,
/// where both its walls touch that side. Campaign 3 read it as the
/// kernel's, the tolerance the box grows taking the crescent's walls
/// within it of each other: the side keeps the lines each wall touches it
/// along, 6e-7 apart along one parameter line, and its outline runs back
/// on itself there. The side was left open, the box's bottom edge along it
/// with it. It holds since the sweep follows a hair running out of the
/// region, and is left open again without that.
#[test]
fn seed_3159337_a_box_touching_both_walls_of_a_crescent_a_hair_thick_leaves_its_side_closed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(210.0),
            Outline::circle([60.0, 300.0], 15.0),
            255.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(210.0),
                Outline::circle([60.0000006, 300.0], 15.0),
                255.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(180.0),
                Outline::rectangle([15.0, 285.0], [45.0, 315.0]),
                510.0,
            )),
        ],
    ));
}

/// A boss and a cut of one radius, their axes ten tolerances apart, cross
/// at a grazing angle along two lines five tolerances from the side
/// x = 5.5 of the block they stand on. Campaign 3 read it as the kernel's:
/// the two corners on the side stand 4.3 tolerances apart, one of them off
/// the side, and a triangle of the side crossed one of a wall. It holds
/// since a wall keeps the rays through its own vertices inside another's
/// room, and an arc keeps a ray beside its end where no arc of the wall it
/// nears ends at that vertex: without either, the two cross again.
#[test]
fn seed_3000782_a_boss_and_a_cut_of_one_radius_crossing_a_hair_from_a_block_s_side_stay_uncrossed()
{
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(5.0),
            Outline::rectangle([0.5, 7.0], [5.5, 14.5]),
            9.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(5.0),
                Outline::circle([5.4999998, 9.25], 3.0),
                15.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(5.0),
                Outline::circle([5.5, 9.25], 3.0),
                8.0,
            )),
        ],
    ));
}

/// A cut lying along x touches the floor z = 120 along a line a hair,
/// 3e-7, from the block's side y = 195, and crosses the post joined under
/// the block, whose wall a cut box touches too. Campaign 3 read it as the
/// kernel's, a plane tangent to a cylinder a hair from a line crossing
/// that plane; the floor was left open along the post's rim beside the
/// block's side. It holds since the curve two cylinders meet along takes
/// no sample a hair from its end on the cap that end lies on, and is left
/// open again without that.
#[test]
fn seed_3108326_a_lying_cut_touching_the_floor_a_hair_from_the_block_s_side_leaves_the_post_s_rim_closed()
 {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(0.0),
            Outline::rectangle([30.0, 120.0], [195.0, 240.0]),
            225.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(120.0),
                Outline::circle([30.0, 150.0], 90.0),
                105.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(120.0),
                Outline::rectangle([-240.0, 60.0], [-60.0, 240.0]),
                210.0,
            )),
            Step::cut(Leaf::prism(
                Plane::yz(-29.9999982),
                Outline::circle([194.9999997, 180.0], 60.0),
                -135.0,
            )),
        ],
    ));
}

/// The four cuts take nothing from the disc, but the second splits its top
/// rim at 30 degrees and the merge after it leaves the rim a ring whose
/// range starts there, on a step of its grid, a rounding past it. The step
/// fell outside the range at both ends, and the chord across it, two steps
/// long, sagged four times the tolerance. It holds since a ring takes every
/// step of its grid, wherever its range starts.
#[test]
fn seed_4136179_a_ring_whose_range_starts_on_a_step_of_its_grid_keeps_that_step() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(4.0), Outline::circle([3.0, 6.0], 4.0), 3.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(4.0),
                Outline::rectangle([-1.0, 10.0], [1.0, 11.0]),
                6.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(8.0),
                Outline::rectangle([5.0, 8.0], [11.0, 15.5]),
                -1.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(8.0),
                Outline::rectangle([9.0, 3.0], [16.0, 7.0]),
                -4.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(-2.0),
                Outline::rectangle([0.0, 4.0], [6.0, 5.0]),
                3.0,
            )),
        ],
    ));
}

/// A bore along x ends at x = 52.5 inside a post along y, and its cap's
/// rim passes 0.0035 inside the post's wall where the block's top, y = 40,
/// meets it — no vertex there, the block's top being covered by the post.
/// The post's circle at y = 40 was sampled on its grid alone, its chord
/// past x = 52.5 sagging 0.08 inside the wall, and the cap's triangles
/// poked through the wall's. It holds since a wall's circles take a ray
/// through every sample of a curve not parallel to it standing just inside
/// it where it holds a face, as they do through a vertex there.
#[test]
fn seed_4058038_a_bore_s_cap_a_hair_inside_a_lying_post_stays_under_its_chords() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(20.0000001),
            Outline::circle([47.5, 15.0], 20.0),
            -65.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xz(40.0),
                Outline::rectangle([13.0, -3.0], [68.0, 53.0]),
                23.0,
            )),
            Step::cut(Leaf::prism(
                Plane::yz(20.0),
                Outline::circle([30.0, 20.0], 17.5),
                32.5,
            )),
        ],
    ));
}

/// A disc cut by a slot of its radius whose arc stands one tolerance off
/// the disc's axis: the two walls cross along two lines and leave a
/// crescent as thin as the tolerance between them, which their circles
/// draw on the very same rays — the two walls' triangles lie on each
/// other. The profile campaign's commonest triangle failure: a slot's or a
/// rounded rectangle's arc a hair off a circle of its radius. Two walls of
/// one radius a hair off one axis want to be one wall, which is the
/// kernel's (decision 8).
#[test]
#[ignore = "kernel"]
fn seed_80503332_a_disc_cut_by_a_slot_of_its_radius_a_hair_off_its_axis_stays_uncrossed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(20.0), Outline::circle([10.0, 0.0], 15.0), 25.0),
        vec![Step::cut(Leaf::prism(
            Plane::xy(15.0),
            Outline::slot([10.00000005, 0.0], [35.00000005, 0.0], 15.0),
            25.0,
        ))],
    ));
}

/// A rounded rectangle whose corners are as round as it is wide is a disc,
/// added a hair beside a disc of its radius; a lying bore then crosses the
/// two walls where they cross each other. Its cap is left open along the
/// sliver of the second wall between the line the bore's wall meets it
/// along and the first's: two walls of one radius a hair apart, which the
/// kernel is to take for one (decision 8).
#[test]
#[ignore = "kernel"]
fn seed_80510549_a_disc_and_its_twin_a_hair_aside_crossed_by_a_bore_leave_its_cap_closed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::yz(-1.0), Outline::circle([4.0, 7.0], 2.5), 6.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(4.0),
                Outline::circle([0.0, 6.5], 1.0),
                3.0,
            )),
            Step::add(Leaf::prism(
                Plane::yz(-1.99999998),
                Outline::rounded([1.49999, 4.5], [6.49999, 9.5], 2.5),
                10.0,
            )),
        ],
    ));
}

/// A rounded rectangle's floor, z = 10, touches its corner's arc along
/// y = 5, and a bore of the corner's radius 6e-8 below the corner's axis
/// crosses that floor 3.5e-4 either side of the line, inside the band
/// where the floor and both walls stand within the tolerance of each
/// other. The edge the block's side x = 6 meets the floor along is left
/// open from that crossing on: a plane tangent to a cylinder a hair from a
/// line crossing it (1-2) beside two walls of one radius a hair apart, the
/// kernel's.
#[test]
#[ignore = "kernel"]
fn seed_80504138_a_rounded_corner_bored_a_hair_below_its_axis_leaves_the_floor_closed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(4.0),
            Outline::rounded([4.0, 10.0], [10.0, 13.0], 1.0),
            6.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::yz(4.0),
                Outline::circle([5.0, 10.99999994], 1.0),
                11.0,
            )),
            Step::add(Leaf::prism(
                Plane::yz(6.0),
                Outline::rectangle([-2.0, 5.0], [8.0, 14.0]),
                2.0,
            )),
            Step::add(Leaf::prism(
                Plane::yz(3.99999),
                Outline::rectangle([3.0, 8.0], [4.0, 14.0]),
                22.0,
            )),
        ],
    ));
}

/// A block cut by a rounded rectangle of its own size 3e-7 aside, under
/// the tolerance, then by a lying bore whose cap is the plane x = 210: the
/// rounded corner's wall touches the face y = 30 along x = 210.0000003, a
/// hair from where the cap crosses that face, and the face is bounded
/// there by an edge the kernel lays slanted across the hair, from the touch
/// line to the block's corner on the cap. Its triangles and the wall's
/// pass through each other along it. A plane tangent to a cylinder a hair
/// from a line crossing it (1-2): the band, the kernel's.
#[test]
#[ignore = "kernel"]
fn seed_80507291_a_block_cut_by_a_rounded_rectangle_a_hair_aside_stays_uncrossed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(0.0),
            Outline::rectangle([180.0, 30.0], [255.0, 105.0]),
            195.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(0.0),
                Outline::rounded([180.0000003, 30.0], [255.0000003, 105.0], 30.0),
                195.0,
            )),
            Step::cut(Leaf::prism(
                Plane::yz(210.0),
                Outline::circle([150.0, 240.0], 135.0),
                300.0,
            )),
        ],
    ));
}

/// A slot cut through a slot of twice its width, the cut 1e-8 deeper than
/// the stock: its floor is taken for the stock's, but the corners where its
/// flat sides meet its arcs stay where the cut put them, 1e-8 under that
/// floor. The floor's triangle from two of them to the sample of the outer
/// arc standing on the same line x = 0.75 is no wider than rounding across
/// the floor, and stands 1e-8 high in the plane of the cut's side, lying on
/// that side's triangle. A corner decided on a plane has to stand on it,
/// which is the kernel's.
#[test]
#[ignore = "kernel"]
fn seed_80511824_a_slot_cut_a_hair_deeper_through_a_wider_slot_stays_uncrossed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(-0.5),
            Outline::slot([0.0, 9.0], [0.0, 15.0], 1.5),
            3.5,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::xz(-0.5),
            Outline::slot([0.0, 9.0], [0.0, 15.0], 0.75),
            3.50000001,
        ))],
    ));
}

/// A ring's hole filled by a disc of its radius one tolerance off its axis
/// and standing above it: the hole's wall and the disc's cross at a grazing
/// angle and leave a crescent, the steps beside the lines they cross along
/// are withheld from every circle of both, and the disc's wall above the
/// ring, facing nothing, is drawn from chords two steps long: the volume
/// falls short (1-3). Two walls of one radius a hair off one axis, which
/// the kernel is to take for one (decision 8).
#[test]
#[ignore = "kernel"]
fn seed_80511593_a_ring_filled_by_a_disc_a_hair_off_its_axis_encloses_their_union() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(15.0),
            Outline::ring([0.0, 10.0], 27.5, 17.5),
            18.0,
        ),
        vec![Step::add(Leaf::prism(
            Plane::xy(14.99999995),
            Outline::circle([5.0000000000000004e-8, 10.0], 17.5),
            35.0,
        ))],
    ));
}

/// A cut of the stock's radius 1.3 tolerances off its axis takes the top
/// of the stock but for a crescent a tolerance thick over its last unit of
/// height. The steps beside the lines the two walls cross along are
/// withheld from the circles bounding that stretch, the stock's top rim
/// among them, and the stock's wall below the crescent, facing nothing, is
/// drawn from that rim's chord, two steps long, down to its floor: its
/// triangles sag three and a half times the tolerance from the top down,
/// and the volume falls short (1-3). Drawing it wants the steps inside the
/// wall's face where the facing ends; but two walls of one radius a hair
/// off one axis are the kernel's to take for one (decision 8), which
/// leaves no crescent to face.
#[test]
#[ignore = "kernel"]
fn seed_4066617_a_cut_of_the_stock_s_radius_a_hair_off_its_axis_keeps_the_stock_s_wall_round_below()
{
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(6.0), Outline::circle([5.0, 3.0], 6.0), -8.0),
        vec![Step::cut(Leaf::prism(
            Plane::xy(5.0),
            Outline::circle([4.99999998, 3.0], 6.0),
            10.0,
        ))],
    ));
}

/// The same as seed 4066617 on another plane: a crescent a tolerance thick
/// over a unit of the stock's height, and the stock's wall beyond it drawn
/// from the chords of the crescent's rims (1-3).
#[test]
#[ignore = "kernel"]
fn seed_4170414_a_cut_of_the_stock_s_radius_a_hair_off_its_axis_keeps_the_stock_s_wall_round_beyond()
 {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::yz(3.0), Outline::circle([8.0, 4.0], 6.0), -7.0),
        vec![Step::cut(Leaf::prism(
            Plane::yz(1.9999998),
            Outline::circle([8.00000002, 4.0], 6.0),
            9.0,
        ))],
    ));
}

/// A post, a block, a bore of the post's radius 2e-7 off its axis, and a
/// boss touching the bore inside at its top. The bore's wall takes the
/// boss's rays where the two touch, over the height the boss stands at,
/// and its triangle from the block's face down through the boss's cap
/// sags past the boss's rim there: the cap pokes through it (1-3, the
/// shape of seed 3202168). It holds with the bore on the post's axis:
/// two walls of one radius a hair off one axis, the kernel's to take for
/// one (decision 8).
#[test]
#[ignore = "kernel"]
fn seed_4037597_a_boss_touching_a_bore_a_hair_off_the_post_s_axis_stays_uncrossed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xz(0.0), Outline::circle([0.0, 5.0], 3.5), 8.0),
        vec![
            Step::add(Leaf::prism(
                Plane::xz(1.00000001),
                Outline::rectangle([-5.0, 0.0], [5.0, 10.0]),
                8.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(1.00000001),
                Outline::circle([-2e-7, 5.0], 3.5),
                8.0,
            )),
            Step::add(Leaf::prism(
                Plane::xz(1.00000001),
                Outline::circle([-0.0, 5.5], 3.0),
                3.0,
            )),
        ],
    ));
}

/// A stock and a cut of its radius 6e-8 apart leave a crescent five
/// tolerances thick, and a lying boss crosses it: its rays, passed on to
/// both walls, put a sample of each where the two stand closer than the
/// rules tell apart, and the lune between their rims on the floor cannot
/// be swept (1-17). It holds without the boss: two walls of one radius a
/// hair off one axis, the kernel's to take for one (decision 8).
#[test]
#[ignore = "kernel"]
fn seed_4109794_a_lying_boss_across_a_crescent_leaves_its_floor_closed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(-2.0), Outline::circle([2.0, 4.5], 5.0), 10.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(-2.0),
                Outline::circle([2.00000006, 4.5], 5.0),
                10.0,
            )),
            Step::add(Leaf::prism(
                Plane::yz(0.0),
                Outline::circle([2.0, 1.0], 2.5),
                3.0,
            )),
        ],
    ));
}

/// A boss of the stock's radius 1e-5 off its axis joined over it, and a
/// small bore across the stock's far rim: the lune the two rims leave on
/// the boss's floor, beside the line the walls cross along, is left open
/// (1-17). It holds without the bore: two walls of one radius a hair off
/// one axis, the kernel's to take for one (decision 8).
#[test]
#[ignore = "kernel"]
fn seed_4165661_a_boss_a_hair_off_the_stock_s_axis_beside_a_small_bore_leaves_its_floor_closed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(1.0), Outline::circle([9.0, 4.0], 1.5), -10.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(1.0),
                Outline::circle([9.0, 6.0], 0.50001),
                -10.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(0.0),
                Outline::circle([9.00001, 4.0], 1.5),
                5.0,
            )),
        ],
    ));
}

/// A crescent 1.25 tolerances thick, and a circle printed on its top,
/// centred on the line its walls cross along: the circle's vertices give
/// both walls one ray there, and the two walls are drawn on the very same
/// points beside it, their triangles lying on each other (1-17). It holds
/// with the circle centred elsewhere: two walls of one radius a hair off
/// one axis, the kernel's to take for one (decision 8).
#[test]
#[ignore = "kernel"]
fn seed_4159169_a_circle_printed_on_a_crescent_where_its_walls_cross_stays_uncrossed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(6.0), Outline::circle([9.0, 1.0], 2.0), -4.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(6.0),
                Outline::circle([8.99999998, 1.0], 2.0),
                -7.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(6.0),
                Outline::circle([9.0, 3.0], 1.5),
                10.0,
            )),
        ],
    ));
}

/// Not a seed: round 5's review built it. A bore along x ends at x = 2
/// inside a post of radius 10 about z, its cap's rim passing the line the
/// cap's plane leaves on the post's wall by the kernel's tolerance or a
/// hair more, so the kernel puts no vertex there. Seen from the post's
/// axis, the rim stands inside the wall by that distance times the cosine
/// of the angle between the wall and the cap, a fiftieth less: under the
/// tolerance, where samples of a curve standing inside a wall were taken
/// to lie on it, though a vertex there gives the wall a ray. The post's
/// chords passed inside the rim and the cap poked through them. It holds
/// since a sample of a curve gives the wall a ray wherever a vertex there
/// would.
#[test]
fn a_bore_s_cap_passing_the_kernel_s_tolerance_inside_a_post_with_no_vertex_stays_under_its_chords()
{
    for center in [7.797958961132711, 7.797958961032711, 7.797958960932711] {
        random_solids::holds_exactly(&Case::new(
            Leaf::prism(Plane::xy(0.0), Outline::circle([0.0, 0.0], 10.0), 10.0),
            vec![Step::cut(Leaf::prism(
                Plane::yz(0.0),
                Outline::circle([center, 5.0], 2.0),
                2.0,
            ))],
        ));
    }
}
