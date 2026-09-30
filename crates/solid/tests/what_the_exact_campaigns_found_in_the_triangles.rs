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

/// As seed 10001163: walls of radius 25 kept 1e-7 apart, a third more than
/// the kernel's tolerance, closer than a fifth of it over some nine degrees
/// either side of each line they cross along, the grid's steps a little
/// over eight: the chord across sags twice the tolerance.
///
/// What was tried. The stretch need only be withheld where the two walls
/// stand face to face — a sliver a cut leaves between them. In a union each
/// wall stands where the other is gone, and sampling the stretch there on
/// common rays draws this seed and 10003386 whole. But the cap where both
/// circles end at the crossing line then holds two arcs closer than the
/// rules tell apart, and rays through the vertex, seen from the other axis,
/// put samples a hair from it on both: the crescent between them folds
/// (seeds 7002169 and 7000204 were left open). Withholding below a tenth of
/// the tolerance there instead of a fifth kept those closed, but left
/// 10001163's chords too long and 7000204's front open.
#[test]
#[ignore = "zone"]
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

/// Two walls of one radius 2e-8 apart, the kernel's tolerance 1.2e-8: they
/// cross along two lines at a slant a hair from nought, and stand closer
/// than a fifth of the tolerance over three steps of the grid about each.
/// Those steps are withheld from both, and the chord from the line to the
/// next step left spans two steps and sags four times the tolerance: the
/// triangles are short by that along the lines passing there. Samples where
/// the walls first stand a fifth of the tolerance apart, either side of the
/// line, would bound the chord to one step; put there exactly, they broke
/// the rays two holes tangent inside one wall share through it, and the
/// sliver of seed 5000951.
#[test]
#[ignore = "zone"]
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

/// As seed 10001163: walls of radius 22.5 kept apart by 1e-7, closer than a
/// fifth of the tolerance over three steps of ten degrees about each line
/// they cross along, and the chord across them sags four times it.
#[test]
#[ignore = "zone"]
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
