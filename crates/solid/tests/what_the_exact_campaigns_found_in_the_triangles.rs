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
