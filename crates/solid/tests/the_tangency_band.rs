//! The tangency band: every case of it known, gathered before its rule is
//! written, as the lesson of #422 asks of a heuristic rule — corrected case
//! by case, it breaks another shape at each turn.
//!
//! A plane and a cylinder decided to touch, or two cylinders touching or of
//! one radius crossing at a grazing angle, stand within the kernel's
//! tolerance of each other over a band about the square root of twice the
//! radius times the tolerance wide, a thousand times the tolerance. Each
//! pair is decided apart, so a third surface or line within the band leaves
//! lines a hair apart and strips thinner than the tolerance between them
//! (decision 7 of `docs/exact-kernel.md`).
//!
//! Here are the band's seeds of campaign 6, shrunk; the band's findings of
//! the earlier campaigns, kept here rather than in
//! `what_the_exact_campaigns_found_in_the_kernel.rs` and `..._triangles.rs`;
//! the findings that hold near the band and that laying the band's lines
//! and corners on every surface of it broke when round 4 tried it; and each
//! shape of the band built on purpose at hairs of half a tolerance, two,
//! twenty, two hundred and two thousand. A case ignored as "band" fails on
//! the kernel it was gathered on.

// The drawing, the promise and the checks are shared with the campaigns;
// this file uses its own part of them.
#[allow(dead_code, unused_imports)]
mod random_solids;

use random_solids::{Case, Leaf, Outline, Plane, Step};

// Campaign 6, an hour on each draw on the kernel after round 5: the seeds
// its samples sorted into the band, by family, shrunk.

// A plane tangent to a cylinder a hair from a line crossing the plane (1-2):
// strips of each between the line of touch and the third line, thinner than
// the tolerance, which no arcs bound in common.

/// Campaign 6a, the square draw: Uncrossed.
#[test]
fn seed_6012407_a_plane_touching_a_wall_a_hair_from_a_line_crossing_it() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(-1.0),
            Outline::rectangle([5.0, 0.0], [7.5, 3.0]),
            10.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xz(0.0),
                Outline::circle([7.4999999, 1.5], 1.5),
                9.0,
            )),
            Step::add(Leaf::prism(
                Plane::xz(0.0),
                Outline::rectangle([7.0, 3.0], [14.0, 9.0]),
                9.0,
            )),
        ],
    ));
}

/// Campaign 6a, the square draw: Uncrossed.
#[test]
fn seed_6186849_a_plane_touching_a_wall_a_hair_from_a_line_crossing_it() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(-2.0),
            Outline::rectangle([0.0, 1.0], [1.0, 6.0]),
            10.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xz(-0.99999999),
                Outline::rectangle([0.5, 1.5], [3.0, 5.5]),
                10.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(-1.0),
                Outline::circle([1.0000001, 3.5], 2.0),
                10.0,
            )),
        ],
    ));
}

/// Campaign 6a, the square draw: Uncrossed.
#[test]
fn seed_6221694_a_plane_touching_a_wall_a_hair_from_a_line_crossing_it() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(2.9999999995311555e-7),
            Outline::rectangle([0.0, 42.5], [37.5, 80.0]),
            13.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xz(1.9999999995311555e-7),
                Outline::rectangle([35.0, 58.75], [40.0, 63.75]),
                25.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(1.9999999995311555e-7),
                Outline::circle([37.50001, 61.25], 2.5),
                50.0,
            )),
        ],
    ));
}

/// Campaign 6a, the square draw: Uncrossed.
#[test]
fn seed_6017796_a_plane_touching_a_wall_a_hair_from_a_line_crossing_it() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(180.0000006),
            Outline::rectangle([-90.0, 60.0], [-30.0, 120.0]),
            30.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::yz(-60.0),
                Outline::rectangle([120.0, 30.0], [195.0, 210.0]),
                210.0,
            )),
            Step::add(Leaf::prism(
                Plane::yz(-60.0),
                Outline::circle([15.0, 180.0], 105.0),
                210.0,
            )),
        ],
    ));
}

/// Campaign 6a, the square draw: Uncrossed.
#[test]
fn seed_6144662_a_plane_touching_a_wall_a_hair_from_a_line_crossing_it() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(25.0),
            Outline::rectangle([35.0, 45.0], [43.0, 83.0]),
            5.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::yz(10.0),
                Outline::rectangle([40.0, 10.0], [80.0, 30.0]),
                25.0,
            )),
            Step::add(Leaf::prism(
                Plane::yz(5.0),
                Outline::circle([80.0000001, 20.0], 5.0),
                50.0,
            )),
        ],
    ));
}

/// Campaign 6a, the square draw: Uncrossed.
#[test]
fn seed_6199137_a_plane_touching_a_wall_a_hair_from_a_line_crossing_it() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(5.0),
            Outline::rectangle([27.5, 7.5], [32.5, 37.5]),
            10.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(0.0),
                Outline::circle([5.0, 5.0], 22.5),
                20.0,
            )),
            Step::add(Leaf::prism(
                Plane::yz(5.00000005),
                Outline::rectangle([23.0, 13.0], [43.0, 33.0]),
                10.0,
            )),
        ],
    ));
}

/// Campaign 6a, the square draw: Uncrossed.
#[test]
fn seed_6240282_a_plane_touching_a_wall_a_hair_from_a_line_crossing_it() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(8.0),
            Outline::rectangle([0.5, 1.0], [6.0, 7.5]),
            8.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::yz(5.0),
                Outline::circle([8.0, 6.5], 1.0),
                3.0,
            )),
            Step::add(Leaf::prism(
                Plane::xz(7.9999998),
                Outline::rectangle([-3.0, 1.0], [6.0, 10.0]),
                10.0,
            )),
        ],
    ));
}

/// Campaign 6a, the square draw: Uncrossed.
#[test]
fn seed_6023355_a_plane_touching_a_wall_a_hair_from_a_line_crossing_it() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(3.0),
            Outline::rectangle([8.0, 0.0], [16.0, 4.0]),
            9.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(2.0),
                Outline::circle([8.0, 8.0], 5.0),
                10.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(4.0),
                Outline::rectangle([8.00001, 0.0], [17.5, 3.5]),
                9.0,
            )),
        ],
    ));
}

/// Campaign 6a, the square draw: Uncrossed.
#[test]
fn seed_6205484_a_plane_touching_a_wall_a_hair_from_a_line_crossing_it() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(7.5),
            Outline::rectangle([10.0, 4.0], [11.5, 8.0]),
            9.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xz(7.0),
                Outline::rectangle([10.0, 4.0], [13.0, 5.0]),
                10.0,
            )),
            Step::add(Leaf::prism(
                Plane::xz(8.5),
                Outline::circle([11.50000006, 6.0], 1.0),
                17.0,
            )),
        ],
    ));
}

/// Campaign 6a, the square draw: Uncrossed.
#[test]
fn seed_6244958_a_plane_touching_a_wall_a_hair_from_a_line_crossing_it() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(0.0),
            Outline::rectangle([27.5, 15.0], [52.5, 47.5]),
            28.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(0.0),
                Outline::rectangle([40.0000003, 18.75], [65.00000030000001, 43.75]),
                55.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(-0.0),
                Outline::circle([52.5000003, 41.25], 2.5),
                55.0,
            )),
        ],
    ));
}

/// Campaign 6a, the square draw: Uncrossed.
#[test]
fn seed_6270911_a_plane_touching_a_wall_a_hair_from_a_line_crossing_it() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(40.0),
            Outline::rectangle([50.0, 35.0], [57.5, 55.0]),
            18.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xz(39.9999999),
                Outline::rectangle([50.0, 35.0], [68.0, 55.0]),
                35.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(40.0),
                Outline::circle([57.4999997, 45.0], 10.0),
                50.0,
            )),
        ],
    ));
}

/// Campaign 6a, the square draw: Uncrossed.
#[test]
#[ignore = "band"]
fn seed_6168409_a_plane_touching_a_wall_a_hair_from_a_line_crossing_it() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(60.0),
            Outline::rectangle([0.0, 30.0], [120.0, 225.0]),
            30.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::yz(60.0),
                Outline::rectangle([-3.0000000000000004e-7, 30.0], [180.0, 225.0]),
                -75.0,
            )),
            Step::add(Leaf::prism(
                Plane::yz(59.9999988),
                Outline::circle([0.0, 270.0], 45.0),
                210.0,
            )),
        ],
    ));
}

/// Campaign 6a, the square draw: Uncrossed.
#[test]
fn seed_6113041_a_plane_touching_a_wall_a_hair_from_a_line_crossing_it() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(5.0),
            Outline::rectangle([15.0, 40.0], [35.0, 45.0]),
            20.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::yz(30.0),
                Outline::circle([35.0, 20.0], 5.0),
                10.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(5.0),
                Outline::circle([34.9999999, 42.5], 2.5),
                20.0,
            )),
        ],
    ));
}

/// Campaign 6b, the profile draw: Uncrossed.
#[test]
fn seed_6500074_a_plane_touching_a_wall_a_hair_from_a_line_crossing_it() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(150.0),
            Outline::rounded([270.0, 60.0], [330.0, 150.0], 30.0),
            -120.0,
        ),
        vec![Step::add(Leaf::prism(
            Plane::xz(120.000006),
            Outline::rounded([210.0, 0.0], [270.0, 165.0], 30.0),
            330.0,
        ))],
    ));
}

/// Campaign 6b, the profile draw: Uncrossed.
#[test]
#[ignore = "band"]
fn seed_6504965_a_plane_touching_a_wall_a_hair_from_a_line_crossing_it() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(7.0),
            Outline::rectangle([6.0, 2.0], [12.0, 7.0]),
            5.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(2.0),
                Outline::circle([5.0, 6.0], 1.0),
                8.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(6.00000001),
                Outline::circle([2.0, 4.0], 6.0),
                7.0,
            )),
        ],
    ));
}

/// Campaign 6b, the profile draw: Uncrossed.
#[test]
fn seed_6567311_a_plane_touching_a_wall_a_hair_from_a_line_crossing_it() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(3.0),
            Outline::rectangle([9.0, 7.0], [14.0, 9.0]),
            6.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xz(3.0),
                Outline::rounded([9.00000006, 7.0], [13.50000006, 8.5], 0.5),
                6.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(8.0),
                Outline::rectangle([7.5, 4.0], [9.5, 7.0]),
                9.0,
            )),
        ],
    ));
}

/// Campaign 6b, the profile draw: Uncrossed.
#[test]
fn seed_6604551_a_plane_touching_a_wall_a_hair_from_a_line_crossing_it() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(2.0),
            Outline::rectangle([3.5, 6.0], [4.5, 9.5]),
            -3.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xz(3.0),
                Outline::circle([4.0, 6.50001], 0.5),
                3.0,
            )),
            Step::add(Leaf::prism(
                Plane::xz(2.0000002),
                Outline::rectangle([3.5, 6.5], [4.5, 9.0]),
                4.0,
            )),
        ],
    ));
}

/// Campaign 6b, the profile draw: Uncrossed.
#[test]
fn seed_6523873_a_plane_touching_a_wall_a_hair_from_a_line_crossing_it() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(40.0),
            Outline::rectangle([15.0, 40.0], [23.0, 75.0]),
            -40.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xz(20.0),
                Outline::rectangle([15.0, 20.0], [55.0, 60.0]),
                18.0,
            )),
            Step::cut(Leaf::prism(
                Plane::yz(40.0),
                Outline::rounded([14.999999, 40.0], [22.499999, 75.0], 2.5),
                -80.0,
            )),
        ],
    ));
}

/// Campaign 6b, the profile draw: Uncrossed.
#[test]
fn seed_6556814_a_plane_touching_a_wall_a_hair_from_a_line_crossing_it() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(8.0),
            Outline::rectangle([2.0, 9.0], [10.0, 11.0]),
            7.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(7.0),
                Outline::rectangle([1.0, 9.0], [6.0, 16.0]),
                4.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(6.0),
                Outline::rounded([1.00000002, 9.0], [5.50000002, 16.0], 1.0),
                9.0,
            )),
        ],
    ));
}

/// Campaign 6b, the profile draw: Uncrossed.
#[test]
fn seed_6650709_a_plane_touching_a_wall_a_hair_from_a_line_crossing_it() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(20.0),
            Outline::rectangle([35.0, 10.0], [65.0, 63.0]),
            38.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(40.0),
                Outline::circle([40.0, 0.0], 20.0),
                13.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(20.00000025),
                Outline::rectangle([40.00001, 52.5], [65.0, 57.5]),
                150.0,
            )),
        ],
    ));
}

/// Campaign 6b, the profile draw: Answers.
#[test]
#[ignore = "band"]
fn seed_6509947_a_plane_touching_a_wall_a_hair_from_a_line_crossing_it() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(150.0),
            Outline::rectangle([240.0, 180.0], [285.0, 285.0]),
            180.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::yz(149.9999997),
            Outline::slot([270.0000003, 232.5], [240.0000003, 232.5], 15.0),
            180.0,
        ))],
    ));
}

/// Campaign 6b, the profile draw: Answers.
#[test]
#[ignore = "band"]
fn seed_6643772_a_plane_touching_a_wall_a_hair_from_a_line_crossing_it() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(-1.0),
            Outline::rectangle([9.0, 1.0], [13.0, 8.0]),
            2.5,
        ),
        vec![Step::add(Leaf::prism(
            Plane::xy(-1.0),
            Outline::slot([10.49999999, 4.5], [8.99999999, 4.5], 2.0),
            2.49999999,
        ))],
    ));
}

/// Campaign 6b, the profile draw: Answers.
#[test]
#[ignore = "band"]
fn seed_6638088_a_plane_touching_a_wall_a_hair_from_a_line_crossing_it() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(-2.0),
            Outline::rectangle([6.5, 8.0], [8.0, 9.5]),
            7.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::yz(8.0),
                Outline::circle([6.0, 3.0], 3.5),
                -1.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(-2.0),
                Outline::rounded([6.5000002, 8.0], [8.0000002, 9.5], 0.5),
                13.0,
            )),
        ],
    ));
}

// A plane and two cylinders touching one another along three lines a hair
// apart, each pair decided apart (2-1).

/// Campaign 6a, the square draw: Answers.
#[test]
fn seed_6005021_a_plane_and_two_walls_touching_along_three_lines_a_hair_apart() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(10.0),
            Outline::rectangle([10.0, 45.0], [45.0, 60.0]),
            33.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(10.0),
                Outline::circle([45.0, 40.0], 20.0),
                33.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(10.0),
                Outline::circle([44.9999997, 52.5], 7.5),
                65.0,
            )),
        ],
    ));
}

/// Campaign 6a, the square draw: Answers.
#[test]
fn seed_6056818_a_plane_and_two_walls_touching_along_three_lines_a_hair_apart() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(-1.0), Outline::circle([6.5, 7.0], 2.0), 8.0),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(-1.0),
                Outline::rectangle([5.0, 6.0], [8.0, 9.0]),
                7.99999,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(-2.0),
                Outline::circle([6.50001, 7.5], 1.5),
                16.0,
            )),
        ],
    ));
}

/// Campaign 6a, the square draw: Answers.
#[test]
#[ignore = "band"]
fn seed_6086149_a_plane_and_two_walls_touching_along_three_lines_a_hair_apart() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(15.0),
            Outline::rectangle([27.5, 32.5], [62.5, 67.5]),
            40.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(15.0),
                Outline::circle([30.0, 50.0], 2.4999999),
                -80.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(15.0000099),
                Outline::circle([30.00000005, 50.0], 2.5),
                -80.0,
            )),
        ],
    ));
}

/// Campaign 6a, the square draw: Answers.
#[test]
fn seed_6185036_a_plane_and_two_walls_touching_along_three_lines_a_hair_apart() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(8.0),
            Outline::rectangle([6.0, 3.0], [7.0, 8.0]),
            8.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xz(7.9999999),
                Outline::circle([7.0, 6.0], 3.0),
                16.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(7.99999999),
                Outline::circle([6.99999, 5.5], 2.5),
                8.0,
            )),
        ],
    ));
}

/// Campaign 6a, the square draw: Answers.
#[test]
fn seed_6024153_a_plane_and_two_walls_touching_along_three_lines_a_hair_apart() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(3.0),
            Outline::rectangle([5.0, 6.5], [8.0, 9.5]),
            8.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(2.0),
                Outline::circle([6.49999, 8.0], 1.5),
                16.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(2.0),
                Outline::circle([6.5, 8.5], 1.0),
                8.0,
            )),
        ],
    ));
}

/// Campaign 6a, the square draw: Answers.
#[test]
fn seed_6142447_a_plane_and_two_walls_touching_along_three_lines_a_hair_apart() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(8.0),
            Outline::rectangle([4.5, 5.0], [12.5, 13.0]),
            -5.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(8.0),
                Outline::circle([10.5, 9.0], 2.00000002),
                -5.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(7.0),
                Outline::circle([9.0, 9.0], 3.50000001),
                9.0,
            )),
        ],
    ));
}

/// Campaign 6a, the square draw: Answers.
#[test]
fn seed_6038027_a_plane_and_two_walls_touching_along_three_lines_a_hair_apart() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(30.0), Outline::circle([300.0, 90.0], 45.0), 210.0),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(30.0000018),
                Outline::rectangle([255.0, 45.0], [345.0, 135.0]),
                210.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(30.0),
                Outline::circle([299.99999, 165.0], 30.0),
                210.0,
            )),
        ],
    ));
}

/// Campaign 6a, the square draw: Uncrossed.
#[test]
fn seed_6155665_a_plane_and_two_walls_touching_along_three_lines_a_hair_apart() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::yz(37.5), Outline::circle([40.0, 5.0], 20.0), 47.5),
        vec![
            Step::cut(Leaf::prism(
                Plane::yz(37.499999),
                Outline::circle([40.0, 27.5], 2.5000001),
                47.5,
            )),
            Step::add(Leaf::prism(
                Plane::xy(25.0),
                Outline::rectangle([10.0, 10.0], [48.0, 40.0]),
                38.0,
            )),
        ],
    ));
}

/// Campaign 6b, the profile draw: Answers.
#[test]
fn seed_6562479_a_plane_and_two_walls_touching_along_three_lines_a_hair_apart() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(-1.0),
            Outline::rectangle([8.0, 0.0], [13.0, 2.0]),
            9.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(-1.0),
                Outline::circle([12.0, 4.0], 2.0),
                8.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(-1.0),
                Outline::rounded([7.9999999, 0.0], [12.9999999, 2.0], 1.0),
                8.0,
            )),
        ],
    ));
}

/// Campaign 6b, the profile draw: Answers.
#[test]
fn seed_6571302_a_plane_and_two_walls_touching_along_three_lines_a_hair_apart() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(7.0),
            Outline::rounded([4.0, 10.0], [9.0, 18.0], 2.0),
            2.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::xz(6.99999),
            Outline::circle([6.0, 11.9999998], 3.0),
            4.0,
        ))],
    ));
}

/// Campaign 6b, the profile draw: Answers.
#[test]
fn seed_6603192_a_plane_and_two_walls_touching_along_three_lines_a_hair_apart() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(210.0),
            Outline::slot([270.0, 30.0], [270.0, 105.0], 30.0),
            30.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(210.0),
                Outline::rectangle([240.0, 75.0], [300.0, 135.0]),
                -30.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(210.0),
                Outline::circle([225.0, 105.00001], 15.0),
                210.0,
            )),
        ],
    ));
}

/// Campaign 6b, the profile draw: Answers.
#[test]
fn seed_6516406_a_plane_and_two_walls_touching_along_three_lines_a_hair_apart() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(7.4999998),
            Outline::rounded([8.0, 7.0], [9.0, 11.0], 0.5),
            10.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::xz(7.5000002),
            Outline::circle([6.5, 7.4999998], 2.5),
            28.0,
        ))],
    ));
}

/// Campaign 6b, the profile draw: Answers.
#[test]
fn seed_6554504_a_plane_and_two_walls_touching_along_three_lines_a_hair_apart() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(120.0),
            Outline::rectangle([180.0, 150.0], [255.0, 255.0]),
            -225.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(120.0),
                Outline::rounded([195.0, 165.0], [240.0, 240.0], 15.0),
                -75.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(120.0),
                Outline::circle([209.99999, 179.9999997], 60.0),
                450.0,
            )),
        ],
    ));
}

/// Campaign 6b, the profile draw: Answers.
#[test]
fn seed_6593513_a_plane_and_two_walls_touching_along_three_lines_a_hair_apart() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(5.0),
            Outline::rectangle([25.0, 20.0], [55.0, 50.0]),
            25.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(5.0),
                Outline::slot([39.9999997, 35.0], [12.499999699999996, 35.0], 5.0),
                25.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(5.0),
                Outline::circle([40.0, 40.0], 10.0),
                50.0,
            )),
        ],
    ));
}

/// Campaign 6b, the profile draw: Answers.
#[test]
fn seed_6624468_a_plane_and_two_walls_touching_along_three_lines_a_hair_apart() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(15.00000005),
            Outline::rounded([15.00001, 20.0], [75.00001, 80.0], 30.0),
            55.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::yz(14.999999050000001),
            Outline::rounded([42.5, 20.0], [65.0, 35.0], 2.5),
            110.0,
        ))],
    ));
}

/// Campaign 6b, the profile draw: Uncrossed.
#[test]
fn seed_6533999_a_plane_and_two_walls_touching_along_three_lines_a_hair_apart() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(5.5),
            Outline::slot([4.0, 0.0], [4.0, -3.0], 2.5),
            10.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::yz(5.49999),
                Outline::rectangle([1.5, -5.5], [6.5, -0.5]),
                4.0,
            )),
            Step::add(Leaf::prism(
                Plane::yz(4.499990100000001),
                Outline::circle([0.0, -3.00000002], 1.50000001),
                5.0,
            )),
        ],
    ));
}

/// Campaign 6b, the profile draw: Uncrossed.
#[test]
#[ignore = "band"]
fn seed_6576153_a_plane_and_two_walls_touching_along_three_lines_a_hair_apart() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(6.0),
            Outline::rectangle([-2.0, -3.0], [4.0, 8.0]),
            6.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(6.0),
                Outline::circle([1.0, 0.0], 3.0),
                2.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(6.0),
                Outline::slot([0.99999999, 1e-8], [2.99999999, 1e-8], 1.0),
                4.0,
            )),
        ],
    ));
}

/// Campaign 6b, the profile draw: Closed.
#[test]
#[ignore = "band"]
fn seed_6529277_a_plane_and_two_walls_touching_along_three_lines_a_hair_apart() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::yz(-1.0), Outline::circle([8.0, 6.0], 5.0), -15.0),
        vec![
            Step::add(Leaf::prism(
                Plane::yz(-1.99999),
                Outline::circle([-1.5, 6.0], 4.49999998),
                -15.0,
            )),
            Step::cut(Leaf::prism(
                Plane::yz(-0.99999999),
                Outline::rectangle([3.0, 6.0], [13.0, 9.0]),
                -30.0,
            )),
        ],
    ));
}

// A plane tangent to, or a hair inside, one of two perpendicular cylinders
// along a ruling that crosses the other (3-2): the strip of the plane is
// bounded by the other's circle, the sliver of the wall by the curve the two
// meet along.

/// Campaign 6a, the square draw: Answers.
#[test]
#[ignore = "band"]
fn seed_6153255_a_plane_touching_a_wall_along_a_ruling_that_crosses_a_square_wall() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xz(4.0), Outline::circle([3.0, 5.0], 6.0), 1.0),
        vec![Step::cut(Leaf::prism(
            Plane::xy(5.0),
            Outline::circle([8.99999999, 4.0], 1.0),
            2.0,
        ))],
    ));
}

/// Campaign 6a, the square draw: Answers.
#[test]
#[ignore = "band"]
fn seed_6136557_a_plane_touching_a_wall_along_a_ruling_that_crosses_a_square_wall() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(3.0), Outline::circle([6.0, 0.0], 1.0), 9.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::yz(3.0),
                Outline::circle([1.0, 4.0], 1.5),
                5.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(2.0),
                Outline::rectangle([3.9999999, -0.5000001], [5.0000001, 0.5000001]),
                9.0,
            )),
        ],
    ));
}

/// Campaign 6a, the square draw: Answers.
#[test]
#[ignore = "band"]
fn seed_6199123_a_plane_touching_a_wall_along_a_ruling_that_crosses_a_square_wall() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(35.0), Outline::circle([20.0, 5.0], 27.5), 28.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::yz(30.0),
                Outline::circle([22.5, 22.5], 25.0),
                -43.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(30.0),
                Outline::rectangle(
                    [-32.5000001, -7.500000099999999],
                    [-7.499999900000001, 17.5000001],
                ),
                28.0,
            )),
        ],
    ));
}

/// Campaign 6a, the square draw: Answers.
#[test]
#[ignore = "band"]
fn seed_6072532_a_plane_touching_a_wall_along_a_ruling_that_crosses_a_square_wall() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(60.0),
            Outline::circle([165.0, 240.0], 135.0),
            165.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::yz(150.0),
                Outline::circle([210.0, 210.0], 45.0),
                240.0,
            )),
            Step::cut(Leaf::prism(
                Plane::yz(119.999994),
                Outline::rectangle([120.0, 150.0], [420.0, 450.0]),
                180.0,
            )),
        ],
    ));
}

/// Campaign 6a, the square draw: Answers.
#[test]
#[ignore = "band"]
fn seed_6143816_a_plane_touching_a_wall_along_a_ruling_that_crosses_a_square_wall() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(20.0),
            Outline::rectangle([50.0, 50.0], [60.0, 83.0]),
            45.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(40.0),
                Outline::circle([10.0, 50.0], 10.0),
                33.0,
            )),
            Step::cut(Leaf::prism(
                Plane::yz(19.999999),
                Outline::circle([60.0000001, 66.25], 10.0),
                8.0,
            )),
        ],
    ));
}

/// Campaign 6b, the profile draw: Uncrossed.
#[test]
fn seed_6512044_a_plane_touching_a_wall_along_a_ruling_that_crosses_a_square_wall() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(1.0),
            Outline::rounded([7.50001, 0.5], [14.50001, 4.5], 0.5),
            3.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::xz(2.0),
            Outline::circle([8.0, 5.0], 4.0),
            -7.0,
        ))],
    ));
}

/// Campaign 6b, the profile draw: Uncrossed.
#[test]
#[ignore = "band"]
fn seed_6550239_a_plane_touching_a_wall_along_a_ruling_that_crosses_a_square_wall() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(-30.0),
            Outline::circle([90.0, 120.0], 120.0),
            300.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(-90.0),
                Outline::circle([180.0000006, 180.0], 75.0),
                210.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(60.0),
                Outline::slot([210.0, 120.0], [45.0, 120.0], 75.0),
                225.0,
            )),
        ],
    ));
}

/// Campaign 6b, the profile draw: Answers.
#[test]
#[ignore = "band"]
fn seed_6611564_a_plane_touching_a_wall_along_a_ruling_that_crosses_a_square_wall() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(6.0),
            Outline::rectangle([6.0, 0.0], [12.0, 8.0]),
            4.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(-2e-8),
                Outline::rounded([9.0, 5.0], [12.0, 10.0], 1.0),
                8.0,
            )),
            Step::cut(Leaf::prism(
                Plane::yz(8.0),
                Outline::circle([8.0, 8.0], 3.0),
                8.0,
            )),
        ],
    ));
}

// The same, the other cylinder's rim on the plane grazing the ruling:
// decision 6 takes the meet's arc for the rim's, and the circle is left on a
// wall that does not carry it.

/// Campaign 6a, the square draw: Answers.
#[test]
#[ignore = "band"]
fn seed_6185693_a_square_wall_s_rim_grazing_the_ruling_a_plane_touches_a_wall_along() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(-60.0),
            Outline::circle([210.0, 150.0], 30.0),
            270.0,
        ),
        vec![Step::add(Leaf::prism(
            Plane::xz(120.0),
            Outline::circle([120.000006, 105.0], 90.0),
            255.0,
        ))],
    ));
}

/// Campaign 6a, the square draw: Answers.
#[test]
#[ignore = "band"]
fn seed_6123681_a_square_wall_s_rim_grazing_the_ruling_a_plane_touches_a_wall_along() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(0.0),
            Outline::rectangle([6.0, 3.0], [8.0, 10.0]),
            -4.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xz(1.00000001),
                Outline::rectangle([5.0, 3.0], [11.0, 9.0]),
                4.0,
            )),
            Step::add(Leaf::prism(
                Plane::xz(2e-8),
                Outline::circle([7.99999998, 6.0], 1.0),
                -4.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(7.0),
                Outline::circle([7.0, 5.0], 4.0),
                -4.0,
            )),
        ],
    ));
}

// Two walls of one radius further apart than decision 8's hair, crossing at
// a grazing angle, with a plane touching both or crossing at the crescent's
// tip (1-9).

/// Campaign 6a, the square draw: Uncrossed.
#[test]
fn seed_6048098_two_walls_of_one_radius_beyond_the_hair_beside_a_third_surface() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(10.0), Outline::circle([25.0, 25.0], 15.0), -13.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(5.0),
                Outline::circle([25.00001, 25.0], 15.0),
                25.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(4.99999),
                Outline::circle([25.00001, 27.5], 12.5),
                30.0,
            )),
        ],
    ));
}

/// Campaign 6a, the square draw: Uncrossed.
#[test]
fn seed_6218170_two_walls_of_one_radius_beyond_the_hair_beside_a_third_surface() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(4.0),
            Outline::rectangle([3.0, 1.0], [9.0, 6.0]),
            9.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(5.0),
                Outline::circle([6.0, 0.0], 1.0),
                8.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(5.0),
                Outline::circle([6.00001, 0.0], 1.0),
                8.0,
            )),
        ],
    ));
}

/// Campaign 6a, the square draw: Uncrossed.
#[test]
#[ignore = "band"]
fn seed_6074790_two_walls_of_one_radius_beyond_the_hair_beside_a_third_surface() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(-60.0),
            Outline::circle([120.0, 30.0], 120.0),
            -210.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(-90.0),
                Outline::circle([120.0, 180.0], 30.000006),
                255.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(-90.00001),
                Outline::circle([119.9999994, 180.0], 30.0),
                150.0,
            )),
        ],
    ));
}

/// Campaign 6b, the profile draw: Uncrossed.
#[test]
#[ignore = "band"]
fn seed_6563127_two_walls_of_one_radius_beyond_the_hair_beside_a_third_surface() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(165.0),
            Outline::ring([120.0, 300.0], 90.0, 30.0),
            180.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(165.0),
                Outline::circle([119.99999, 300.0], 30.0),
                255.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(135.0),
                Outline::rectangle([120.0, 180.0], [150.0, 330.0]),
                510.0,
            )),
        ],
    ));
}

/// Campaign 6b, the profile draw: Uncrossed.
#[test]
fn seed_6594068_two_walls_of_one_radius_beyond_the_hair_beside_a_third_surface() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(0.0),
            Outline::rectangle([3.5, 8.5], [4.5, 15.0]),
            2.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::yz(0.0),
                Outline::circle([4.0, 14.5], 0.5),
                2.0,
            )),
            Step::cut(Leaf::prism(
                Plane::yz(-1.0),
                Outline::circle([4.0, 14.49999], 0.5),
                3.0,
            )),
        ],
    ));
}

/// Campaign 6b, the profile draw: Uncrossed.
#[test]
fn seed_6625237_two_walls_of_one_radius_beyond_the_hair_beside_a_third_surface() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(-2.0),
            Outline::rectangle([3.0, 5.0], [5.0, 10.0]),
            3.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(-3.0),
                Outline::circle([4.0, 8.5], 1.0),
                3.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(-3.0),
                Outline::circle([4.0, 8.49999], 1.0),
                3.0,
            )),
        ],
    ));
}

/// Campaign 6b, the profile draw: Answers.
#[test]
#[ignore = "band"]
fn seed_6598801_two_walls_of_one_radius_beyond_the_hair_beside_a_third_surface() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(1.0),
            Outline::rounded([0.0, 2.0], [7.5, 7.0], 1.5),
            2.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::yz(0.99999999),
                Outline::circle([1.5, 3.4999998], 1.5),
                2.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(2.0),
                Outline::rectangle([-2.0, 0.0], [2.0, 4.0]),
                2.0,
            )),
        ],
    ));
}

/// Campaign 6b, the profile draw: Answers.
#[test]
fn seed_6631754_two_walls_of_one_radius_beyond_the_hair_beside_a_third_surface() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(2.0),
            Outline::rectangle([8.0, 2.0], [15.0, 10.0]),
            9.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(2.0),
                Outline::circle([9.5, 3.50001], 1.5),
                9.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(2.0),
                Outline::circle([9.5, 3.5], 1.5),
                18.0,
            )),
        ],
    ));
}

/// Campaign 6b, the profile draw: Uncrossed.
#[test]
#[ignore = "band"]
fn seed_6637572_two_walls_of_one_radius_beyond_the_hair_beside_a_third_surface() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(4.0),
            Outline::rectangle([2.0, 6.0], [8.0, 11.0]),
            -6.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xz(4.0),
                Outline::circle([3.0, 7.00001], 1.0),
                7.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(3.99999998),
                Outline::circle([3.0, 7.0], 1.0),
                -11.0,
            )),
        ],
    ));
}

/// Campaign 6b, the profile draw: Answers.
#[test]
fn seed_6533713_two_walls_of_one_radius_beyond_the_hair_beside_a_third_surface() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(15.0),
            Outline::rectangle([15.0, 20.0], [45.0, 55.0]),
            13.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(10.0),
                Outline::circle([30.0, 40.0], 15.0),
                10.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(10.0),
                Outline::circle([30.0, 39.99999], 15.0),
                25.0,
            )),
        ],
    ));
}

/// Campaign 6b, the profile draw: Uncrossed.
#[test]
fn seed_6645220_two_walls_of_one_radius_beyond_the_hair_beside_a_third_surface() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(-1.0), Outline::ring([9.0, 2.0], 3.0, 1.0), 6.0),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(-1.0),
                Outline::circle([9.00001, 2.0], 1.0),
                9.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(6.0),
                Outline::rectangle([5.0, -3.0], [16.0, 8.0]),
                3.0,
            )),
        ],
    ));
}

/// Campaign 6b, the profile draw: Answers.
#[test]
fn seed_6547043_two_walls_of_one_radius_beyond_the_hair_beside_a_third_surface() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(-30.0),
            Outline::circle([120.0, -15.0], 90.0),
            120.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::xy(-30.0),
            Outline::slot([119.99999, -15.0000003], [269.99999, -15.0000003], 90.0),
            120.0,
        ))],
    ));
}

/// Campaign 6b, the profile draw: Answers.
#[test]
fn seed_6579931_two_walls_of_one_radius_beyond_the_hair_beside_a_third_surface() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(7.49999),
            Outline::rectangle([6.0, 2.0], [10.0, 10.0]),
            5.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xz(8.49999),
                Outline::circle([8.0, 3.50001], 2.0),
                5.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(7.49999),
                Outline::circle([8.0, 3.5], 2.0),
                10.0,
            )),
        ],
    ));
}

/// Campaign 6b, the profile draw: Uncrossed.
#[test]
fn seed_6582528_two_walls_of_one_radius_beyond_the_hair_beside_a_third_surface() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(3.0),
            Outline::rectangle([-1.0, 1.0], [9.0, 11.0]),
            10.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(1.0),
                Outline::circle([6.0, 6.0], 3.0),
                7.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(0.0),
                Outline::circle([6.00001, 6.0], 3.0),
                13.0,
            )),
        ],
    ));
}

/// Campaign 6b, the profile draw: Uncrossed.
#[test]
#[ignore = "band"]
fn seed_6621877_two_walls_of_one_radius_beyond_the_hair_beside_a_third_surface() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::yz(25.0), Outline::ring([5.0, 15.0], 15.0, 7.5), 23.0),
        vec![
            Step::add(Leaf::prism(
                Plane::yz(20.0),
                Outline::circle([4.99999, 15.0], 7.5),
                45.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(5.0),
                Outline::rectangle([27.5, 22.5], [42.5, 37.5]),
                50.0,
            )),
        ],
    ));
}

/// Campaign 6b, the profile draw: Answers.
#[test]
#[ignore = "band"]
fn seed_6650833_two_walls_of_one_radius_beyond_the_hair_beside_a_third_surface() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(60.0),
            Outline::rectangle([255.0, 240.0], [390.0, 465.0]),
            60.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xz(90.0),
                Outline::rounded([255.0, 240.0], [390.0, 465.0], 30.0),
                60.0,
            )),
            Step::add(Leaf::prism(
                Plane::xz(89.9999982),
                Outline::circle([284.9999997, 269.99999], 30.0),
                195.0,
            )),
        ],
    ));
}

/// Campaign 6b, the profile draw: Closed.
#[test]
#[ignore = "band"]
fn seed_6546656_two_walls_of_one_radius_beyond_the_hair_beside_a_third_surface() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(0.0), Outline::circle([5.0, 3.0], 2.0), 7.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(-1.0),
                Outline::rounded([2.99999, 1.0], [6.99999, 6.0], 2.0),
                3.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(2e-7),
                Outline::circle([6.0, 3.0], 1.0),
                8.0,
            )),
        ],
    ));
}

/// Campaign 6b, the profile draw: Closed.
#[test]
#[ignore = "band"]
fn seed_6517915_two_walls_of_one_radius_beyond_the_hair_beside_a_third_surface() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(0.0),
            Outline::rectangle([210.0, 120.0], [390.0, 285.0]),
            45.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xz(-3.0000000000000004e-7),
                Outline::rounded([210.0, 120.0], [390.0, 285.0], 30.0),
                210.0,
            )),
            Step::add(Leaf::prism(
                Plane::xz(9.7e-6),
                Outline::circle([239.9999994, 149.99999], 30.0),
                60.0,
            )),
        ],
    ));
}

/// Campaign 6b, the profile draw: Closed.
#[test]
#[ignore = "band"]
fn seed_6587058_two_walls_of_one_radius_beyond_the_hair_beside_a_third_surface() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(3.0),
            Outline::slot([5.0, 3.0], [-1.0, 3.0], 2.5),
            6.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(3.00000006),
                Outline::circle([-1.0, 2.99999], 2.5),
                11.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(3.00000006),
                Outline::slot([-0.99999999, 2.99999], [-0.99999999, 1.99999], 1.5),
                2.0,
            )),
        ],
    ));
}

/// Campaign 6b, the profile draw: Closed.
#[test]
#[ignore = "band"]
fn seed_6657327_two_walls_of_one_radius_beyond_the_hair_beside_a_third_surface() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(-2.0),
            Outline::rectangle([8.0, 1.0], [11.0, 7.0]),
            1.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(-2.0000001),
                Outline::rounded([8.0, 1.0], [10.5, 6.5], 0.5),
                2.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(-2.0000000399999998),
                Outline::circle([8.5, 1.49999], 0.5),
                5.0,
            )),
        ],
    ));
}

// A rounded rectangle a hair taller than its two corners: decision 8's guard
// leaves two walls of one radius five tolerances apart beside the run's
// plane (1-9).

/// Campaign 6b, the profile draw: Answers.
#[test]
#[ignore = "band"]
fn seed_6500611_a_rounded_rectangle_a_hair_taller_than_its_corners_beside_a_circle() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(29.99999),
            Outline::circle([25.0, 2.5], 7.4999999),
            50.0,
        ),
        vec![Step::add(Leaf::prism(
            Plane::xz(29.99999005),
            Outline::rounded(
                [17.5000001, -4.9999999],
                [37.5000001, 10.000000100000001],
                7.4999999,
            ),
            48.0,
        ))],
    ));
}

// Two parallel walls decided to touch, the line they touch along on a plane
// crossing both square (the parallel form of 4-5).

/// Campaign 6a, the square draw: Uncrossed.
#[test]
#[ignore = "band"]
fn seed_6063426_two_walls_touching_on_a_plane_through_both_axes() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(150.0),
            Outline::rectangle([120.0, 30.0], [285.0, 75.0]),
            255.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(150.0),
                Outline::circle([270.0, 30.0], 135.0),
                510.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(150.0),
                Outline::circle([180.0, 30.0], 44.9999997),
                1020.0,
            )),
        ],
    ));
}

/// Campaign 6a, the square draw: Uncrossed.
#[test]
#[ignore = "band"]
fn seed_6122353_two_walls_touching_on_a_plane_through_both_axes() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(4.0), Outline::circle([2.0, 10.0], 0.5), 3.0),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(-1.0),
                Outline::rectangle([0.0, 5.0], [2.0, 11.0]),
                6.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(-2.0),
                Outline::circle([2.00000001, 8.0], 1.5),
                11.0,
            )),
        ],
    ));
}

// A crescent of two walls crossing at a grazing angle falls under a later
// leaf's tolerance (1-6).

/// Campaign 6a, the square draw: Uncrossed.
#[test]
#[ignore = "band"]
fn seed_6091545_a_crescent_falling_under_a_later_leaf_s_tolerance() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::yz(30.0), Outline::circle([15.0, 60.0], 30.0), 60.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::yz(29.9999976),
                Outline::circle([30.0000003, 60.0], 45.0),
                60.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(150.0),
                Outline::rectangle([120.0, 240.0], [360.0, 420.0]),
                270.0,
            )),
        ],
    ));
}

// A wall within the tolerance of both faces of a skin, from one side of one
// ruling (4-1).

/// Campaign 6a, the square draw: Answers.
#[test]
#[ignore = "band"]
fn seed_6080810_a_wall_touching_both_faces_of_a_skin_from_one_side() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(35.0),
            Outline::rectangle([-10.0, 33.0], [15.0, 58.0]),
            -48.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(30.0),
                Outline::rectangle([-4.999999900000001, 35.0000001], [14.9999999, 54.9999999]),
                -95.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(30.0),
                Outline::circle([12.5, 45.0], 2.49999995),
                40.0,
            )),
        ],
    ));
}

/// Campaign 6a, the square draw: Answers.
#[test]
#[ignore = "band"]
fn seed_6074582_a_wall_touching_both_faces_of_a_skin_from_one_side() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(30.0),
            Outline::rectangle([0.0, 20.0], [50.0, 70.0]),
            48.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(30.0000001),
                Outline::rectangle([-10.0, 10.0], [60.0, 80.0]),
                15.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(25.0),
                Outline::rectangle([20.0, 70.0], [30.0, 80.0]),
                95.0,
            )),
            Step::add(Leaf::prism(
                Plane::yz(15.0),
                Outline::circle([30.0, 25.0], 5.0),
                45.0,
            )),
        ],
    ));
}

// A wall touching two parallel planes a hair apart from the same side (4-1).

/// Campaign 6b, the profile draw: Answers.
#[test]
#[ignore = "band"]
fn seed_6528115_a_wall_touching_two_planes_a_hair_apart_from_one_side() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(-60.0),
            Outline::rectangle([120.0, 180.0], [195.0, 285.0]),
            270.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(-60.0),
                Outline::rectangle([104.99999969999999, 187.5], [194.9999997, 277.5]),
                270.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(-60.0),
                Outline::circle([135.0, 233.0], 60.0),
                540.0,
            )),
        ],
    ));
}

/// Campaign 6b, the profile draw: Answers.
#[test]
#[ignore = "band"]
fn seed_6521564_a_wall_touching_two_planes_a_hair_apart_from_one_side() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(7.0),
            Outline::rectangle([5.0, 5.0], [7.0, 8.0]),
            9.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(7.0),
                Outline::rounded([5.00000002, 5.0], [7.00000002, 8.0], 1.0),
                9.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(7.0),
                Outline::rectangle([5.00000001, 5.0], [9.0, 8.0]),
                18.0,
            )),
        ],
    ));
}

// A ruling of a perpendicular wall crossing exactly the line two parallel
// walls touch along inside (1-12).

/// Campaign 6a, the square draw: Answers.
#[test]
#[ignore = "band"]
fn seed_6128869_a_square_ruling_crossing_the_line_two_walls_touch_along() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(5.0), Outline::circle([2.0, 1.0], 4.5), 8.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(4.0),
                Outline::circle([3.5, 1.0], 3.0),
                7.0,
            )),
            Step::add(Leaf::prism(
                Plane::yz(5.0),
                Outline::circle([4.0, 10.0], 3.0),
                5.0,
            )),
        ],
    ));
}

// The triangles' side of 2-1: a wall touching another a hair from where a
// run's plane touches both has a step withheld where it stands alone (1-1).

/// Campaign 6b, the profile draw: Volume.
#[test]
#[ignore = "band"]
fn seed_6518903_a_wall_touching_another_a_hair_from_a_run_s_end_keeps_its_step() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(5.0),
            Outline::rounded([2.5, 6.0], [7.0, 13.0], 2.0),
            6.0,
        ),
        vec![Step::add(Leaf::prism(
            Plane::xy(5.0),
            Outline::circle([0.5, 7.99999994], 2.0),
            7.0,
        ))],
    ));
}

/// Campaign 6b, the profile draw: Volume.
#[test]
#[ignore = "band"]
fn seed_6628386_a_wall_touching_another_a_hair_from_a_run_s_end_keeps_its_step() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(120.0),
            Outline::rounded([105.0, 0.0], [195.0, 60.0], 30.0),
            45.0,
        ),
        vec![Step::add(Leaf::prism(
            Plane::xy(60.0),
            Outline::rounded([-14.999994, 60.0], [345.000006, 420.0], 180.0),
            150.0,
        ))],
    ));
}

// A wall facing a bore a hair off its axis over part of its height (1-3).

/// Campaign 6b, the profile draw: Volume.
#[test]
#[ignore = "band"]
fn seed_6513598_a_wall_facing_a_bore_a_hair_off_its_axis_over_part_of_its_height() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(-1.0),
            Outline::rounded([9.0, 8.0], [11.5, 12.5], 0.5),
            6.5,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xz(-1.0),
                Outline::circle([9.5, 8.49999], 0.5),
                -13.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(-1.0),
                Outline::circle([9.5, 8.5], 0.5),
                -6.0,
            )),
        ],
    ));
}

// The band's findings of the earlier campaigns. Those that hold are what a
// rule for the band must not break: seeds 3150523, 3192987, 28000951,
// 3157560 and 3175702 broke when round 4 laid the band's lines and corners
// on every surface of it, and two walls of one radius a hair off one axis
// hold since decision 8 takes them for one wall.

/// Seed 1014146 shrunk once the wall and the notch hold — those two stay in
/// `what_the_exact_campaigns_found_in_the_kernel.rs`: the block bored
/// through across its end but for a wall three tenths of a micron thick,
/// the bore's bottom at the height of the post's cap, which it touches along
/// the line through the corner the post's wall passes.
///
/// Understood and left: failure 1-12's configuration. The curve the post
/// and the bore meet along touches the post's rim at that corner to the
/// second order, and runs within a femtometre of it back to where the rim
/// crosses the block's end — a corner four tenths of a micron off the line
/// the cap and the bore touch along, so not on the bore by its support. The
/// two arcs share one corner and overlap, set off from it at one angle, and
/// are both too short for their bends to part them: the star ties.
#[test]
fn seed_1014146_a_wall_a_hair_thick_bored_where_the_post_s_cap_touches_the_bore() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(60.0),
            Outline::rectangle([225.0, 60.0], [255.0, 270.0]),
            -30.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::yz(59.9999997),
                Outline::circle([255.0, 165.0], 45.0),
                -30.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(120.0),
                Outline::circle([3.0000000000000004e-7, 300.0], 75.0),
                255.0,
            )),
        ],
    ));
}

/// Failure 1-2 of campaign 1: a bar bored by a hole touching its side at
/// its end, then by a hole of that radius ten microns aside,
/// touching the side too.
///
/// Understood and left: the side's strip between the two lines the holes
/// touch it along is ten microns wide, and the holes' walls over
/// it stand less than a picometre above it — a skin decision 6 is meant to
/// leave out. But the walls over the strip are two, parted by the line the
/// holes cross along, so the strip has no twin bounded by its own arcs, and
/// the skin is kept: the triangles of the strip and of the walls cross.
/// Decision 7 winds each strip beside the faces lying over it as the exact
/// geometry has it, and keeps the skin too. The line the holes cross along
/// stands within a picometre of the side; laid on the side as well, it
/// would part the side's strip where the walls' end and make them twins —
/// the band's lines shared by every surface of the band, which decisions 3
/// and 5 do not make.
#[test]
fn seed_1032742_a_bar_bored_twice_a_hair_apart_touching_its_side() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(-10.0),
            Outline::rectangle([40.0, 40.0], [45.0, 70.0]),
            13.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(-10.0),
                Outline::circle([45.0, 55.0], 15.0),
                25.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(-15.0),
                Outline::circle([44.99999, 55.0], 15.0),
                25.0,
            )),
        ],
    ));
}

/// Failure 2-5 of campaign 3: a bore of radius 2.00000001 pokes 1e-8
/// through the stock it touches inside, and a third circle crosses both
/// within the band where the two stand within the tolerance of each other.
/// Its two crossings, one with each, were computed pair by pair and stood a
/// sliver apart, which no region of the bottom could enclose. Moved onto the
/// touch, the bore crosses the third circle where the stock does.
#[test]
fn seed_3150523_a_bore_a_hair_through_the_stock_crossed_by_a_third_circle_at_the_touch() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(1.0), Outline::circle([6.0, 0.0], 2.5), 10.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(1.0),
                Outline::circle([6.0, 0.5], 2.00000001),
                9.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(1.0),
                Outline::circle([6.0, 3.0], 0.5000002),
                10.0,
            )),
        ],
    ));
}

/// Failure 1-2 of campaign 3: a bore touching a block's side inside leaves
/// a cusp of matter between the side and its wall, which a block cut from it
/// crosses 1e-5 from the touch. The kernel declined, the block covering both
/// strips of the cusp; it now takes the cusp for a skin and answers right.
/// Its triangles still cross below the cut, where the cut's corner on the
/// side, 1e-5 from the line of touch and within the band where side and wall
/// stand within the tolerance, was given the wall by distance (failure 1-2):
/// a strip of the wall lies on the side. Under the cut's floor the cusp runs
/// on past the cut's side, so the side's region and the wall's there are
/// the cusp a tangency leaves, kept as the exact geometry has it, not
/// strips decision 7 reads; what lies on the side is the wall's triangle
/// from the line of touch to that corner, a vertex of both faces 1e-5 into
/// the band.
#[test]
fn seed_3130833_a_cusp_a_bore_touching_a_side_leaves_is_cut_a_hair_from_its_tip() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(0.0),
            Outline::rectangle([1.5, 0.5], [8.5, 7.5]),
            5.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(0.0),
                Outline::circle([4.99999, 4.0], 3.5),
                10.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(1.0),
                Outline::rectangle([3.0, 2.0], [5.0, 9.0]),
                10.0,
            )),
        ],
    ));
}

/// Failure 1-2 of campaign 3, found by the triangles, its largest group: a
/// block given a second block beside it, then a bore touching the first's
/// side y = 12 1e-7 from where the second's side x = 11.5 crosses it, inside
/// the band where the side and the bore's wall stand within the tolerance
/// of each other.
///
/// Understood and left: the line x = 11.5 on the side and the bore's ruling
/// on x = 11.5, 3e-15 apart, are one line, on the side, the second block's
/// side and the wall, and its corners stand on the side. The strip of the
/// wall between that line and the line of touch, and the strip of the side
/// between the same two lines, hold a skin 3e-15 thick, and the wall's
/// triangles there lie on the side's. They are twins, but not bounded by
/// the same arcs: the second block ends at z = 11 and the line with it,
/// while the side's region runs on into the band above and the wall's all
/// round. Decision 6 asks for regions bounded by the same arcs; decision 7
/// reads a region standing within the tolerance of the other surface all
/// across, and neither strip is a region: each is the part of a region
/// whose point stands beyond the band, decided there — kept, both, as the
/// exact geometry has them, a cusp of matter 3e-15 thick that the second
/// block ends. Dropping it wants both regions parted where the line ends,
/// along an arc across the band the boolean has no curve for.
#[test]
fn seed_3156716_a_bore_touching_a_side_a_hair_from_where_a_second_block_s_side_crosses_it() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(6.0),
            Outline::rectangle([9.0, 6.0], [15.0, 12.0]),
            6.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(6.0),
                Outline::rectangle([7.0, 7.5], [11.5, 12.5]),
                5.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(6.0),
                Outline::circle([11.5000001, 10.5], 1.5),
                6.0,
            )),
        ],
    ));
}

/// Failure 2-1 of campaign 3: a bar bored along its length by a bore whose
/// wall touches the bar's top from inside, 1.8e-6 off the line where a
/// wider bar joined to it touches the top too, and touches the bore inside
/// along a third line 3.6e-6 off — three lines in one band, each pair
/// decided apart. The strip of the top between the bore's line and the wide
/// bar's stands within a hundredth of a femtometre of the wide bar's wall
/// all across, and no ray told whether the wide bar wraps it: the kernel
/// declined. The wide bar wraps it as its wall's matter lies, the top
/// standing outside the wall it touches (decision 7), and the strip of the
/// wide bar's wall between its two lines of touch lies between the bore and
/// the top, inside the cusp they leave.
#[test]
fn seed_3192987_a_bar_bored_under_its_top_joined_to_a_wider_bar_touching_both() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(-89.99999),
            Outline::rectangle([255.0, 195.0], [435.0, 375.0]),
            570.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::yz(-89.99999),
                Outline::circle([344.9999982, 315.0], 60.0),
                1140.0,
            )),
            Step::add(Leaf::prism(
                Plane::yz(-60.0),
                Outline::circle([345.0, 285.0], 90.0),
                255.0,
            )),
        ],
    ));
}

/// A rounded block bored at a corner by a hole of the corner's radius 1e-5
/// across, thirty-three tolerances at a reach of three hundred, then given a
/// cylinder lying across it: the kernel declined.
///
/// Understood and left: with a hair of fifty tolerances, decision 8 took the
/// hole for the corner's wall and it held. The hair is twenty, which no
/// line of measure crossing the moved wall at a slant can see: the two walls
/// stay two, crossing at a grazing angle within a hair of the sides the
/// corner touches, and the kernel declines — failure 1-9, the band.
#[test]
fn seed_80506598_a_rounded_block_bored_a_hair_off_its_corner_then_given_a_cylinder_across() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(60.0),
            Outline::rounded([120.0, 120.0], [195.0, 315.0], 15.0),
            180.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(30.0),
                Outline::circle([135.0, 134.99999], 15.0),
                225.0,
            )),
            Step::add(Leaf::prism(
                Plane::yz(120.0),
                Outline::circle([150.0, 30.0], 120.0),
                105.0,
            )),
        ],
    ));
}

/// A block bored by a hole touching its side and 1e-5 off its floor, then
/// cut by a bore of its radius touching both, 1e-5 aside: seven hundred
/// tolerances at a reach of fourteen.
///
/// Understood and left: beyond decision 8's hair, which a line of measure
/// crossing the moved wall at a slant would see, the two walls stay two.
/// They cross along two rulings at an angle of four millionths, within a
/// hair of the side both touch, and the kernel declines: failure 1-9, the
/// band of two walls of one radius a few hundred tolerances apart.
#[test]
fn seed_80505830_a_hole_bored_again_seven_hundred_tolerances_aside_where_both_touch_a_side() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(5.0),
            Outline::rectangle([7.0, 2.0], [13.0, 9.0]),
            5.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::yz(5.0),
                Outline::circle([9.75, 4.75001], 2.75),
                5.0,
            )),
            Step::cut(Leaf::prism(
                Plane::yz(4.99999994),
                Outline::circle([9.75, 4.75], 2.75),
                9.0,
            )),
        ],
    ));
}

/// The same with a profile: a slot given a disc of its radius 1e-5 across
/// its cap, five hundred tolerances at a reach of fifteen, then a block
/// over the cap. Understood and left, as for seed 80505830: the disc and
/// the cap stay two walls crossing at a grazing angle.
#[test]
fn seed_80509593_a_slot_given_a_disc_five_hundred_tolerances_across_its_cap() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(1.0),
            Outline::slot([7.0, 3.0], [13.0, 3.0], 2.0),
            10.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xz(1.0),
                Outline::circle([13.0, 2.99999], 2.0),
                10.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(1.0),
                Outline::rectangle([11.0, 1.0], [15.0, 7.0]),
                10.0,
            )),
        ],
    ));
}

/// A block bored by a hole touching two of its sides but for 1e-5 off one,
/// then the hole filled by a post touching both: thirty-seven tolerances at
/// a reach of two hundred and seventy.
///
/// Understood and left: with a hair of fifty tolerances, decision 8 took the
/// post for the hole, and it filled it. The hair is twenty, which no line of
/// measure can see the merge through: the post and the hole stay two walls
/// of one radius crossing at a grazing angle where both touch a side, and
/// the kernel declines — failure 1-9, the band.
#[test]
fn seed_80509744_a_hole_a_hair_off_a_side_filled_by_a_post_touching_it() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(60.0),
            Outline::rectangle([90.0, 90.0], [270.0, 255.0]),
            60.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xz(60.0),
                Outline::circle([105.0, 105.00001], 15.0),
                120.0,
            )),
            Step::add(Leaf::prism(
                Plane::xz(60.0),
                Outline::circle([105.0, 105.0], 15.0),
                120.0,
            )),
        ],
    ));
}

/// Found by the review of round 5 in a campaign of profiles, and made by
/// decision 8: a slot given a disc of its cap's radius 1e-7 across, three
/// tolerances at a reach of thirty-five, then a post touching the slot's
/// side where the disc touches it. Taken for the cap, the disc is moved
/// 1e-7 along the side, and the line it touched the side along is the
/// cap's. The post, drawn on the disc's line, then touches the side and the
/// cap along three lines within three tolerances of one another.
///
/// Understood and left: failure 2-1, the band. Before decision 8 the disc,
/// the side and the post touched along one line, and the case held; a
/// merge moves what the second operand touched, and a later leaf drawn on
/// it stands a hair off.
#[test]
fn seed_82512408_a_slot_given_a_disc_a_hair_off_its_cap_then_a_post_on_the_disc_s_line() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(5.0),
            Outline::slot([20.0, 15.0], [20.0, 2.5], 15.0),
            25.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(5.0),
                Outline::circle([20.0, 2.5000001], 15.0),
                25.0000001,
            )),
            Step::add(Leaf::prism(
                Plane::xy(5.0),
                Outline::circle([2.5, 2.5000001], 2.5),
                43.0,
            )),
        ],
    ));
}

/// The same with a rounded block a unit wide, its end one wall: bored at
/// the end by a hole of its radius 6e-8 across, six tolerances, then given
/// a post touching the side where the hole does. Taken for the end's wall,
/// the hole's line of touch with the side moves with it, and the post
/// stands a hair from the end's: the kernel declines.
///
/// Understood and left, as for seed 82512408: failure 2-1, the band.
#[test]
fn seed_82508905_a_rounded_end_bored_a_hair_across_then_given_a_post_on_the_bore_s_line() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(-1.0),
            Outline::rounded([2.0, 2.0], [3.0, 10.0], 0.5),
            8.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(-2.0),
                Outline::circle([2.5, 2.50000006], 0.5),
                3.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(-2.0),
                Outline::circle([1.5, 2.50000006], 0.5),
                6.0,
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
/// face where the facing ends.
/// It held while decision 8 took the bore for the post's wall with a hair
/// of fifty tolerances. The hair is twenty, which no line of measure can
/// see the merge through, and thirty tolerances are left two walls again:
/// the sliver is back.
#[test]
#[ignore = "band"]
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

/// A disc cut by a slot of its radius whose arc stands one tolerance off
/// the disc's axis: the two walls cross along two lines and leave a
/// crescent as thin as the tolerance between them, which their circles
/// draw on the very same rays — the two walls' triangles lie on each
/// other. The profile campaign's commonest triangle failure: a slot's or a
/// rounded rectangle's arc a hair off a circle of its radius. Two walls of
/// one radius a hair off one axis want to be one wall, which is the
/// kernel's (decision 8).
#[test]
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
#[ignore = "band"]
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
#[ignore = "band"]
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

/// A ring's hole filled by a disc of its radius one tolerance off its axis
/// and standing above it: the hole's wall and the disc's cross at a grazing
/// angle and leave a crescent, the steps beside the lines they cross along
/// are withheld from every circle of both, and the disc's wall above the
/// ring, facing nothing, is drawn from chords two steps long: the volume
/// falls short (1-3). Two walls of one radius a hair off one axis, which
/// the kernel is to take for one (decision 8).
#[test]
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
#[ignore = "band"]
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

// What round 6's two 300-second campaigns, from seeds 90 000 000 and
// 90 500 000, found failing on the rule as first written that held before
// it, shrunk.

/// Round 6's campaign of the square draw from seed 90 000 000, on the rule
/// as first written: a stock bored by a hole touching it inside at its top,
/// then cut by a wall dipping two hundredths of a micron into both. The
/// face of that wall between the lines it crosses them along is bounded
/// over a stretch by the stock's rim, taken for its own; the listing's
/// check read that circle as a cross-section of the wall, though it turns
/// about the stock's axis, the other way round, and found the face
/// backwards.
#[test]
fn seed_90005701_a_wall_dipping_into_a_stock_and_the_bore_touching_it_is_listed_its_own_way() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xz(7.0), Outline::circle([4.0, 1.0], 2.0), 10.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xz(7.0000001),
                Outline::circle([4.0, 2.0], 1.0),
                9.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(8.0),
                Outline::circle([4.0, 5.0], 2.00000002),
                10.0,
            )),
        ],
    ));
}

/// The same campaign: a post, a pin dipping into its top, and a block whose
/// top touches the post where the pin dips.
#[test]
#[ignore = "band"]
fn seed_90016363_a_block_touching_a_post_under_a_pin_dipping_into_it() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::yz(1.0), Outline::circle([3.5, 2.0], 2.5), 7.0),
        vec![
            Step::add(Leaf::prism(
                Plane::yz(1.0000001),
                Outline::circle([3.5, 5.0], 0.50000006),
                14.0,
            )),
            Step::add(Leaf::prism(
                Plane::yz(0.9999999),
                Outline::rectangle([2.0, 2.0], [4.5, 4.5]),
                14.0,
            )),
        ],
    ));
}

/// Round 6's campaign of the profile draw from seed 90 500 000, on the rule
/// as first written: a slot, a disc of its radius at its end ten microns
/// off, and a block a hair proud of both.
#[test]
#[ignore = "band"]
fn seed_90501389_a_slot_cut_by_a_disc_at_its_end_and_a_block_a_hair_proud() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(8.0),
            Outline::slot([0.0, 4.0], [-3.5, 4.0], 0.5),
            3.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xz(8.0000002),
                Outline::circle([-3.5, 4.00001], 0.5),
                1.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(8.000000400000001),
                Outline::rectangle([-4.0, 4.0], [-1.0, 12.0]),
                1.0,
            )),
        ],
    ));
}

/// The same campaign: a block given a rounded rectangle of its size a hair
/// aside, then a block cut from it far above.
#[test]
fn seed_90504120_a_block_given_a_rounded_rectangle_a_hair_aside_then_cut_above() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(15.0),
            Outline::rectangle([25.0, 10.0], [38.0, 20.0]),
            25.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(15.0),
                Outline::rounded([24.99999995, 10.0], [37.49999995, 20.0], 2.5),
                15.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(40.0),
                Outline::rectangle([25.0, 35.0], [63.0, 70.0]),
                20.0,
            )),
        ],
    ));
}

/// The same campaign: a block given a post touching its side, then bored by
/// a hole of the post's radius ten microns aside.
#[test]
#[ignore = "band"]
fn seed_90504257_a_block_given_a_post_then_bored_ten_microns_aside() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(1.0),
            Outline::rectangle([-1.0, 1.0], [3.0, 9.0]),
            10.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(1.0),
                Outline::circle([1.0, 2.49999], 2.0),
                19.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(1.0),
                Outline::circle([1.0, 2.5], 2.0),
                19.0,
            )),
        ],
    ));
}

/// The same campaign: a slot given a bar touching its run's plane, a hair
/// longer than the run. The bar's cap stands six hundredths of a micron
/// past where the run turns into the slot's end, and a band laid on the
/// run's plane there, beyond its face, crossed the end's line with no
/// corner: a band is laid only where faces of both its surfaces stand.
#[test]
fn seed_90509248_a_slot_given_a_bar_touching_its_side_a_hair_past_its_end() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(5.0),
            Outline::slot([4.0, 7.0], [4.0, 5.0], 1.0),
            -4.0,
        ),
        vec![Step::add(Leaf::prism(
            Plane::xz(5.0),
            Outline::circle([6.0, 2.0], 3.0),
            -2.00000006,
        ))],
    ));
}

// Each shape of the band, built.

/// The kernel's tolerance on every case built here: each reaches ten.
const TOLERANCE: f64 = 1e-8;

/// A bar along Y resting on a block's top, the line it touches the top
/// along `hair` tolerances inside the top's edge x = 5: a plane tangent to
/// a cylinder a hair from a line crossing it (1-2), joined.
fn a_bar_resting_on_a_block_s_top_inside_its_edge(hair: f64) -> Case {
    Case::new(
        Leaf::prism(
            Plane::xy(0.0),
            Outline::rectangle([0.0, 0.0], [5.0, 10.0]),
            3.0,
        ),
        vec![Step::add(Leaf::prism(
            Plane::xz(10.0),
            Outline::circle([5.0 - hair * TOLERANCE, 4.5], 1.5),
            10.0,
        ))],
    )
}

#[test]
fn a_bar_resting_on_a_block_s_top_half_a_tolerance_inside_its_edge() {
    random_solids::holds_exactly(&a_bar_resting_on_a_block_s_top_inside_its_edge(0.5));
}

#[test]
fn a_bar_resting_on_a_block_s_top_two_tolerances_inside_its_edge() {
    random_solids::holds_exactly(&a_bar_resting_on_a_block_s_top_inside_its_edge(2.0));
}

#[test]
fn a_bar_resting_on_a_block_s_top_twenty_tolerances_inside_its_edge() {
    random_solids::holds_exactly(&a_bar_resting_on_a_block_s_top_inside_its_edge(20.0));
}

#[test]
fn a_bar_resting_on_a_block_s_top_two_hundred_tolerances_inside_its_edge() {
    random_solids::holds_exactly(&a_bar_resting_on_a_block_s_top_inside_its_edge(200.0));
}

#[test]
fn a_bar_resting_on_a_block_s_top_two_thousand_tolerances_inside_its_edge() {
    random_solids::holds_exactly(&a_bar_resting_on_a_block_s_top_inside_its_edge(2000.0));
}

/// A block bored along Y by a hole touching its top inside along x = 3,
/// then notched from x = 3 + `hair` tolerances on: the notch's side
/// crosses the top a hair from the line the hole touches it along, the
/// cusp of matter between the top and the hole cut inside the band (1-2,
/// the shape of seed 3130833).
fn a_bore_touching_a_block_s_top_notched_beside_the_touch(hair: f64) -> Case {
    Case::new(
        Leaf::prism(
            Plane::xy(-1.0),
            Outline::rectangle([0.0, 0.0], [6.0, 10.0]),
            4.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xz(10.0),
                Outline::circle([3.0, 1.5], 1.5),
                10.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(2.0),
                Outline::rectangle([3.0 + hair * TOLERANCE, 2.0], [7.0, 8.0]),
                2.0,
            )),
        ],
    )
}

#[test]
fn a_bore_touching_a_block_s_top_notched_half_a_tolerance_beside_the_touch() {
    random_solids::holds_exactly(&a_bore_touching_a_block_s_top_notched_beside_the_touch(0.5));
}

#[test]
fn a_bore_touching_a_block_s_top_notched_two_tolerances_beside_the_touch() {
    random_solids::holds_exactly(&a_bore_touching_a_block_s_top_notched_beside_the_touch(2.0));
}

#[test]
fn a_bore_touching_a_block_s_top_notched_twenty_tolerances_beside_the_touch() {
    random_solids::holds_exactly(&a_bore_touching_a_block_s_top_notched_beside_the_touch(
        20.0,
    ));
}

#[test]
fn a_bore_touching_a_block_s_top_notched_two_hundred_tolerances_beside_the_touch() {
    random_solids::holds_exactly(&a_bore_touching_a_block_s_top_notched_beside_the_touch(
        200.0,
    ));
}

#[test]
fn a_bore_touching_a_block_s_top_notched_two_thousand_tolerances_beside_the_touch() {
    random_solids::holds_exactly(&a_bore_touching_a_block_s_top_notched_beside_the_touch(
        2000.0,
    ));
}

/// A block bored twice by holes of one radius `hair` tolerances apart,
/// both touching its side y = 4: two walls of one radius crossing at a
/// grazing angle between the two lines the side touches them along (1-9,
/// the shape of seed 80505830). Up to twenty tolerances, decision 8 takes
/// the second for the first.
fn a_block_bored_twice_apart_where_both_touch_its_side(hair: f64) -> Case {
    Case::new(
        Leaf::prism(
            Plane::xy(0.0),
            Outline::rectangle([0.0, 0.0], [10.0, 4.0]),
            6.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(0.0),
                Outline::circle([5.0, 2.5], 1.5),
                6.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(0.0),
                Outline::circle([5.0 + hair * TOLERANCE, 2.5], 1.5),
                6.0,
            )),
        ],
    )
}

#[test]
fn a_block_bored_twice_half_a_tolerance_apart_where_both_touch_its_side() {
    random_solids::holds_exactly(&a_block_bored_twice_apart_where_both_touch_its_side(0.5));
}

#[test]
fn a_block_bored_twice_two_tolerances_apart_where_both_touch_its_side() {
    random_solids::holds_exactly(&a_block_bored_twice_apart_where_both_touch_its_side(2.0));
}

#[test]
fn a_block_bored_twice_twenty_tolerances_apart_where_both_touch_its_side() {
    random_solids::holds_exactly(&a_block_bored_twice_apart_where_both_touch_its_side(20.0));
}

#[test]
fn a_block_bored_twice_two_hundred_tolerances_apart_where_both_touch_its_side() {
    random_solids::holds_exactly(&a_block_bored_twice_apart_where_both_touch_its_side(200.0));
}

#[test]
fn a_block_bored_twice_two_thousand_tolerances_apart_where_both_touch_its_side() {
    random_solids::holds_exactly(&a_block_bored_twice_apart_where_both_touch_its_side(2000.0));
}

/// A block bored by a hole touching its side y = 6 inside along x = 5,
/// then given a post inside the hole touching the hole's wall inside and
/// the side along x = 5 + `hair` tolerances: a plane and two walls touching
/// one another along three lines a hair apart (2-1, the shape of seed
/// 6005021).
fn a_post_touching_a_bore_and_the_side_it_touches_beside_its_line(hair: f64) -> Case {
    Case::new(
        Leaf::prism(
            Plane::xy(0.0),
            Outline::rectangle([0.0, 0.0], [10.0, 6.0]),
            6.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(0.0),
                Outline::circle([5.0, 2.0], 4.0),
                6.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(0.0),
                Outline::circle([5.0 + hair * TOLERANCE, 4.5], 1.5),
                6.0,
            )),
        ],
    )
}

#[test]
fn a_post_touching_a_bore_and_the_side_it_touches_half_a_tolerance_beside_its_line() {
    random_solids::holds_exactly(
        &a_post_touching_a_bore_and_the_side_it_touches_beside_its_line(0.5),
    );
}

#[test]
fn a_post_touching_a_bore_and_the_side_it_touches_two_tolerances_beside_its_line() {
    random_solids::holds_exactly(
        &a_post_touching_a_bore_and_the_side_it_touches_beside_its_line(2.0),
    );
}

#[test]
fn a_post_touching_a_bore_and_the_side_it_touches_twenty_tolerances_beside_its_line() {
    random_solids::holds_exactly(
        &a_post_touching_a_bore_and_the_side_it_touches_beside_its_line(20.0),
    );
}

#[test]
fn a_post_touching_a_bore_and_the_side_it_touches_two_hundred_tolerances_beside_its_line() {
    random_solids::holds_exactly(
        &a_post_touching_a_bore_and_the_side_it_touches_beside_its_line(200.0),
    );
}

#[test]
fn a_post_touching_a_bore_and_the_side_it_touches_two_thousand_tolerances_beside_its_line() {
    random_solids::holds_exactly(
        &a_post_touching_a_bore_and_the_side_it_touches_beside_its_line(2000.0),
    );
}

/// A bar along X dipping `hair` tolerances into a block's top z = 3, then a
/// hole bored square through both: the top cuts the bar's wall along two
/// lines a hair apart, the hole's circle bounds the strip of the top
/// between them and the curve the hole meets the bar along bounds the
/// bar's sliver (3-2). Under a tolerance, the bar is moved onto the top.
fn a_bar_dipping_into_a_block_s_top_then_bored_square(hair: f64) -> Case {
    Case::new(
        Leaf::prism(
            Plane::xy(0.0),
            Outline::rectangle([0.0, 0.0], [10.0, 10.0]),
            3.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::yz(0.0),
                Outline::circle([5.0, 4.5 - hair * TOLERANCE], 1.5),
                10.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(0.0),
                Outline::circle([5.0, 5.0], 1.0),
                7.0,
            )),
        ],
    )
}

#[test]
fn a_bar_dipping_half_a_tolerance_into_a_block_s_top_then_bored_square() {
    random_solids::holds_exactly(&a_bar_dipping_into_a_block_s_top_then_bored_square(0.5));
}

#[test]
fn a_bar_dipping_two_tolerances_into_a_block_s_top_then_bored_square() {
    random_solids::holds_exactly(&a_bar_dipping_into_a_block_s_top_then_bored_square(2.0));
}

#[test]
fn a_bar_dipping_twenty_tolerances_into_a_block_s_top_then_bored_square() {
    random_solids::holds_exactly(&a_bar_dipping_into_a_block_s_top_then_bored_square(20.0));
}

#[test]
fn a_bar_dipping_two_hundred_tolerances_into_a_block_s_top_then_bored_square() {
    random_solids::holds_exactly(&a_bar_dipping_into_a_block_s_top_then_bored_square(200.0));
}

#[test]
fn a_bar_dipping_two_thousand_tolerances_into_a_block_s_top_then_bored_square() {
    random_solids::holds_exactly(&a_bar_dipping_into_a_block_s_top_then_bored_square(2000.0));
}

/// A block given a post whose axis its side y = 5 holds, then a hole bored
/// inside the post, `hair` tolerances from touching its wall inside at x =
/// 1, its axis on the side too: two walls touching, or all but, along a
/// line on a plane that crosses both square there (the parallel form of
/// 4-5, the shape of seed 6063426). Under a tolerance, they are decided
/// to touch.
fn a_bore_inside_a_post_on_a_side_through_both_axes(hair: f64) -> Case {
    Case::new(
        Leaf::prism(
            Plane::xy(0.0),
            Outline::rectangle([0.0, 0.0], [10.0, 5.0]),
            5.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(0.0),
                Outline::circle([5.0, 5.0], 4.0),
                10.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(0.0),
                Outline::circle([3.0 + hair * TOLERANCE, 5.0], 2.0),
                10.0,
            )),
        ],
    )
}

#[test]
fn a_bore_half_a_tolerance_inside_a_post_on_a_side_through_both_axes() {
    random_solids::holds_exactly(&a_bore_inside_a_post_on_a_side_through_both_axes(0.5));
}

#[test]
fn a_bore_two_tolerances_inside_a_post_on_a_side_through_both_axes() {
    random_solids::holds_exactly(&a_bore_inside_a_post_on_a_side_through_both_axes(2.0));
}

#[test]
fn a_bore_twenty_tolerances_inside_a_post_on_a_side_through_both_axes() {
    random_solids::holds_exactly(&a_bore_inside_a_post_on_a_side_through_both_axes(20.0));
}

#[test]
fn a_bore_two_hundred_tolerances_inside_a_post_on_a_side_through_both_axes() {
    random_solids::holds_exactly(&a_bore_inside_a_post_on_a_side_through_both_axes(200.0));
}

#[test]
fn a_bore_two_thousand_tolerances_inside_a_post_on_a_side_through_both_axes() {
    random_solids::holds_exactly(&a_bore_inside_a_post_on_a_side_through_both_axes(2000.0));
}

/// A block joined to a second block across its side y = 6, the second's
/// side x = 3 crossing it, then bored by a hole touching the side inside
/// along x = 3 + `hair` tolerances: a plane tangent to a cylinder a hair
/// from a line crossing it, the line ending at the second block's top
/// while the side runs on above (1-2, the shape of seed 3156716).
fn a_bore_touching_a_side_beside_a_joined_block_s_side(hair: f64) -> Case {
    Case::new(
        Leaf::prism(
            Plane::xy(0.0),
            Outline::rectangle([0.0, 0.0], [6.0, 6.0]),
            10.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(0.0),
                Outline::rectangle([-2.0, 2.0], [3.0, 7.0]),
                4.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(0.0),
                Outline::circle([3.0 + hair * TOLERANCE, 4.5], 1.5),
                10.0,
            )),
        ],
    )
}

#[test]
fn a_bore_touching_a_side_half_a_tolerance_beside_a_joined_block_s_side() {
    random_solids::holds_exactly(&a_bore_touching_a_side_beside_a_joined_block_s_side(0.5));
}

#[test]
fn a_bore_touching_a_side_two_tolerances_beside_a_joined_block_s_side() {
    random_solids::holds_exactly(&a_bore_touching_a_side_beside_a_joined_block_s_side(2.0));
}

#[test]
fn a_bore_touching_a_side_twenty_tolerances_beside_a_joined_block_s_side() {
    random_solids::holds_exactly(&a_bore_touching_a_side_beside_a_joined_block_s_side(20.0));
}

#[test]
fn a_bore_touching_a_side_two_hundred_tolerances_beside_a_joined_block_s_side() {
    random_solids::holds_exactly(&a_bore_touching_a_side_beside_a_joined_block_s_side(200.0));
}

#[test]
fn a_bore_touching_a_side_two_thousand_tolerances_beside_a_joined_block_s_side() {
    random_solids::holds_exactly(&a_bore_touching_a_side_beside_a_joined_block_s_side(2000.0));
}

/// A block, a bar along Y against its end y = 0 whose top ruling stands
/// `hair` tolerances inside the block's side x = 7.5, and a second block
/// resting on the bar: the plane it rests on touches the bar a hair from
/// the corner where the first block's side meets it (1-2, the shape of
/// seed 6012407).
fn a_block_resting_on_a_bar_beside_another_block_s_corner(hair: f64) -> Case {
    Case::new(
        Leaf::prism(
            Plane::xy(-1.0),
            Outline::rectangle([5.0, 0.0], [7.5, 3.0]),
            10.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xz(0.0),
                Outline::circle([7.5 - hair * TOLERANCE, 1.5], 1.5),
                9.0,
            )),
            Step::add(Leaf::prism(
                Plane::xz(0.0),
                Outline::rectangle([7.0, 3.0], [10.0, 9.0]),
                9.0,
            )),
        ],
    )
}

#[test]
fn a_block_resting_on_a_bar_half_a_tolerance_beside_another_block_s_corner() {
    random_solids::holds_exactly(&a_block_resting_on_a_bar_beside_another_block_s_corner(0.5));
}

#[test]
fn a_block_resting_on_a_bar_two_tolerances_beside_another_block_s_corner() {
    random_solids::holds_exactly(&a_block_resting_on_a_bar_beside_another_block_s_corner(2.0));
}

#[test]
fn a_block_resting_on_a_bar_twenty_tolerances_beside_another_block_s_corner() {
    random_solids::holds_exactly(&a_block_resting_on_a_bar_beside_another_block_s_corner(
        20.0,
    ));
}

#[test]
fn a_block_resting_on_a_bar_two_hundred_tolerances_beside_another_block_s_corner() {
    random_solids::holds_exactly(&a_block_resting_on_a_bar_beside_another_block_s_corner(
        200.0,
    ));
}

#[test]
fn a_block_resting_on_a_bar_two_thousand_tolerances_beside_another_block_s_corner() {
    random_solids::holds_exactly(&a_block_resting_on_a_bar_beside_another_block_s_corner(
        2000.0,
    ));
}

/// A disc along Y, cut by a pin along Z whose wall touches the disc's cap
/// y = 3 along a ruling `hair` tolerances inside the disc's widest place
/// x = 9: the ruling crosses the disc's wall square to the pin's at a
/// grazing angle, twice, a hair either side of the pin's cap (3-2, the
/// shape of seed 6153255).
fn a_pin_touching_a_disc_s_cap_along_a_ruling_grazing_its_wall(hair: f64) -> Case {
    Case::new(
        Leaf::prism(Plane::xz(4.0), Outline::circle([3.0, 5.0], 6.0), 1.0),
        vec![Step::cut(Leaf::prism(
            Plane::xy(5.0),
            Outline::circle([9.0 - hair * TOLERANCE, 4.0], 1.0),
            2.0,
        ))],
    )
}

#[test]
#[ignore = "band"]
fn a_pin_touching_a_disc_s_cap_along_a_ruling_half_a_tolerance_inside_its_wall() {
    random_solids::holds_exactly(&a_pin_touching_a_disc_s_cap_along_a_ruling_grazing_its_wall(0.5));
}

#[test]
fn a_pin_touching_a_disc_s_cap_along_a_ruling_two_tolerances_inside_its_wall() {
    random_solids::holds_exactly(&a_pin_touching_a_disc_s_cap_along_a_ruling_grazing_its_wall(2.0));
}

#[test]
#[ignore = "band"]
fn a_pin_touching_a_disc_s_cap_along_a_ruling_twenty_tolerances_inside_its_wall() {
    random_solids::holds_exactly(
        &a_pin_touching_a_disc_s_cap_along_a_ruling_grazing_its_wall(20.0),
    );
}

#[test]
#[ignore = "band"]
fn a_pin_touching_a_disc_s_cap_along_a_ruling_two_hundred_tolerances_inside_its_wall() {
    random_solids::holds_exactly(
        &a_pin_touching_a_disc_s_cap_along_a_ruling_grazing_its_wall(200.0),
    );
}

#[test]
#[ignore = "band"]
fn a_pin_touching_a_disc_s_cap_along_a_ruling_two_thousand_tolerances_inside_its_wall() {
    random_solids::holds_exactly(
        &a_pin_touching_a_disc_s_cap_along_a_ruling_grazing_its_wall(2000.0),
    );
}
