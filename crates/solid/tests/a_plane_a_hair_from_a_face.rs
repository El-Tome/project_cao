//! A plane a hair from a face, as the campaigns of 5 October 2026 found it on
//! #526's branch (8a, 8b and 8c in `docs/exact-kernel-journal.md`), each seed
//! shrunk.
//!
//! Two parallel planes further apart than the tolerance and closer than a
//! micron, which decision 1 does not take for one: a rounded rectangle or a
//! slot drawn a hair beside a block, its sides a hair from the block's and
//! its corner walls touching the block's sides a hair off (the wall touching
//! two planes from one side of 4-1); or blocks, slots and discs whose floors
//! stand a hair apart, crossed by a third surface. Every case failed the same
//! way at every fineness the triangles were drawn at: the kernel's.
//! Decisions 1, 2, 3 and 10 of `docs/exact-kernel.md` hold all but one,
//! ignored with what still breaks it.

// The drawing, the promise and the checks are the campaigns'; this file
// only builds cases and holds them.
#[allow(dead_code, unused_imports)]
mod random_solids;

use random_solids::{Case, Leaf, Outline, Plane, Step};

// A wall touching two planes a hair apart.

/// Campaign 8c, the profile draw through the application's body: Answers.
/// Shrunk through the application's body.
#[test]
fn seed_8559776_a_wall_touching_two_planes_a_hair_apart() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(2.0),
            Outline::rectangle([5.0, 2.5], [7.5, 8.0]),
            2.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(1.0),
                Outline::rounded([5.00000001, 2.5], [7.50000001, 8.0], 0.5),
                4.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(0.0),
                Outline::rectangle([4.0, 6.0], [7.5, 11.5]),
                2.0,
            )),
        ],
    ));
}

/// Campaigns 8b and 8c, the profile draw, on the kernel and through the
/// application's body: Answers.
#[test]
fn seed_8510444_a_wall_touching_two_planes_a_hair_apart() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(20.0),
            Outline::rectangle([30.0, 30.0], [50.0, 48.0]),
            -20.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::xz(25.0),
            Outline::rounded([30.00000005, 30.0], [50.00000005, 47.5], 5.0),
            33.0,
        ))],
    ));
}

/// Campaigns 8b and 8c, the profile draw, on the kernel and through the
/// application's body: Answers.
#[test]
fn seed_8516622_a_wall_touching_two_planes_a_hair_apart() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(5.0),
            Outline::rectangle([9.0, 5.0], [11.0, 9.0]),
            6.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(4.0),
                Outline::rectangle([8.49999998, 5.0], [10.99999998, 9.0]),
                12.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(4.0),
                Outline::rounded([8.0, 5.0], [11.0, 9.0], 1.0),
                24.0,
            )),
        ],
    ));
}

/// Campaigns 8b and 8c, the profile draw, on the kernel and through the
/// application's body: Answers.
#[test]
fn seed_8540801_a_wall_touching_two_planes_a_hair_apart() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(5.0),
            Outline::rectangle([1.0, 3.0], [8.0, 9.0]),
            5.0,
        ),
        vec![Step::add(Leaf::prism(
            Plane::yz(5.0),
            Outline::rounded([0.99999999, 3.0], [7.99999999, 8.5], 1.0),
            5.0,
        ))],
    ));
}

/// Campaigns 8b and 8c, the profile draw, on the kernel and through the
/// application's body: Answers.
#[test]
#[ignore = "plane-a-hair: a wall touching its own side and crossing a block's face ten microns off it, a hundred and sixty tolerances: past decision 10's hair, the grazing crossing declines"]
fn seed_8550233_a_wall_touching_two_planes_a_hair_apart() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(40.0),
            Outline::ring([15.0, 30.0], 23.0, 13.0),
            30.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(0.0),
                Outline::rounded([10.0, 0.0], [50.0, 35.0], 17.5),
                40.0,
            )),
            Step::add(Leaf::prism(
                Plane::yz(10.00001),
                Outline::rectangle([8.0, 0.0], [25.0, 40.0]),
                20.0,
            )),
        ],
    ));
}

/// Campaign 8c, the profile draw through the application's body: Answers.
/// Shrunk through the application's body.
#[test]
fn seed_8554448_a_wall_touching_two_planes_a_hair_apart() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(20.0),
            Outline::rectangle([15.0, 20.0], [17.5, 50.0000001]),
            40.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::xz(20.000001),
            Outline::circle([17.5, 35.00000005], 15.0),
            43.0,
        ))],
    ));
}

/// Campaign 8c, the profile draw through the application's body: Answers.
/// Shrunk through the application's body.
#[test]
fn seed_8578887_a_wall_touching_two_planes_a_hair_apart() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(-1.0),
            Outline::rectangle([9.0, 0.0], [14.0, 8.0]),
            -4.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(-2.0),
                Outline::rectangle([5.0, 2.0], [14.0, 6.0]),
                4.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(-1.99999999),
                Outline::slot([9.5, 4.00000001], [4.5, 4.00000001], 2.0),
                8.0,
            )),
        ],
    ));
}

/// Campaign 8c, the profile draw through the application's body: Answers.
/// Shrunk through the application's body.
#[test]
fn seed_8581852_a_wall_touching_two_planes_a_hair_apart() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(12.5),
            Outline::rectangle([20.0, 30.0], [43.0, 43.0]),
            45.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::yz(12.5),
                Outline::rectangle([20.0000001, 30.0], [42.5000001, 42.5]),
                -90.0,
            )),
            Step::add(Leaf::prism(
                Plane::yz(12.5000001),
                Outline::circle([25.0, 35.0], 5.0),
                180.0,
            )),
        ],
    ));
}

/// Campaign 8c, the profile draw through the application's body: Closed. Shrunk
/// through the application's body.
#[test]
fn seed_8581445_a_wall_touching_two_planes_a_hair_apart() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(3.0),
            Outline::rectangle([3.0, 2.0], [8.0, 8.0]),
            1.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::yz(3.00000001),
                Outline::rectangle([3.0, 2.0], [7.0, 6.0]),
                2.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(-2.0),
                Outline::rectangle([7.0, 7.0], [12.0, 12.0]),
                7.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(7.0),
                Outline::circle([2.0, 5.0], 1.0),
                -6.0,
            )),
        ],
    ));
}

/// Campaign 8c, the profile draw through the application's body: Uncrossed.
/// Shrunk through the application's body.
#[test]
fn seed_8588829_a_wall_touching_two_planes_a_hair_apart() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(5.0),
            Outline::rectangle([10.0, 30.0], [37.5, 40.0]),
            15.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(5.0),
                Outline::rectangle([9.99999995, 30.0], [37.49999995, 40.0]),
                30.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(0.0),
                Outline::rectangle([40.0, 28.0], [48.0, 68.0]),
                28.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(0.0),
                Outline::slot([15.0, 35.0], [35.0, 35.0], 2.5),
                38.0,
            )),
        ],
    ));
}

/// The like-for-like campaigns of #528, the profile draw, on the kernel and
/// through the application's body: Answers. Held before decision 10: its
/// move carried the upper corners' walls onto the lower ones, a hair below.
#[test]
fn seed_8528992_a_rounded_rectangle_a_hair_taller_than_its_corners_moved_onto_a_face() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(6.0),
            Outline::rectangle([-1.5, 3.5], [6.0, 6.5]),
            6.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::xy(6.0),
            Outline::rounded([-2e-7, 3.5], [4.4999998, 6.5000001], 1.5),
            1.0,
        ))],
    ));
}

// Two faces a hair apart crossed by a third surface.

/// Campaign 8b, the profile draw: Listed.
#[test]
fn seed_8505208_two_faces_a_hair_apart_crossed_by_a_third_surface() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(0.0),
            Outline::rectangle([180.0, 30.0], [270.0, 195.0]),
            255.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(30.0),
                Outline::rectangle([-90.0, -60.0], [270.0, 300.0]),
                195.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(-3.0000000000000004e-7),
                Outline::slot([90.0, 120.0], [195.0, 120.0], 75.0),
                195.0,
            )),
        ],
    ));
}

/// Campaign 8a, the square draw: Answers.
#[test]
fn seed_8087943_two_faces_a_hair_apart_crossed_by_a_third_surface() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(10.0),
            Outline::rectangle([20.0, 33.0], [58.0, 40.0]),
            43.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(9.9999999),
                Outline::rectangle([35.0, 13.0], [75.0, 25.0]),
                30.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(9.999999950000001),
                Outline::rectangle([40.0, 30.0], [50.0, 68.0]),
                60.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(9.999999950000001),
                Outline::rectangle([45.0, 18.0], [55.0, 35.0]),
                60.0,
            )),
        ],
    ));
}

/// Campaigns 8b and 8c, the profile draw, on the kernel and through the
/// application's body: Answers.
#[test]
fn seed_8537287_two_faces_a_hair_apart_crossed_by_a_third_surface() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(-2.0),
            Outline::rectangle([-1.0, 1.0], [3.0, 5.0]),
            9.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(-2.0000001),
                Outline::rectangle([2.0, 3.0], [4.0, 6.0]),
                17.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(-2.0000000399999998),
                Outline::rectangle([3.0, 3.0], [6.0, 6.0]),
                34.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(-2.0000000399999998),
                Outline::rectangle([3.0, 4.0], [5.0, 5.0]),
                68.0,
            )),
        ],
    ));
}

/// Campaigns 8b and 8c, the profile draw, on the kernel and through the
/// application's body: Answers.
#[test]
fn seed_8511092_two_faces_a_hair_apart_crossed_by_a_third_surface() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(0.0),
            Outline::rectangle([-45.0, 165.0], [105.0, 315.0]),
            -225.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(-3.0000000000000004e-7),
                Outline::slot([30.0, 240.0], [-30.0, 240.0], 37.5),
                -450.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(3.0000000000000004e-7),
                Outline::slot([30.0, 240.0], [-30.0, 240.0], 37.5),
                225.0,
            )),
        ],
    ));
}

/// Campaigns 8b and 8c, the profile draw, on the kernel and through the
/// application's body: Answers.
#[test]
fn seed_8527977_two_faces_a_hair_apart_crossed_by_a_third_surface() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(22.5),
            Outline::rectangle([25.0, 15.0], [30.0, 50.0]),
            40.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(22.4999999),
                Outline::rounded([26.25, 16.25], [28.75, 48.75], 1.25),
                80.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(22.50000005),
                Outline::rounded([26.25, 16.25], [28.75, 48.75], 1.25),
                48.0,
            )),
        ],
    ));
}

/// Campaigns 8b and 8c, the profile draw, on the kernel and through the
/// application's body: Uncrossed.
///
/// Was ignored as: plane-a-hair: a later leaf far off grows the tolerance past
/// the hair two corners of the body stand apart, and decision 5 takes them for
/// one, off a wall: two faces drawn crossing. Holds since the triangles of two
/// faces folded onto each other across an edge are cut the other way, round 3
/// of #536.
#[test]
fn seed_8519077_two_faces_a_hair_apart_crossed_by_a_third_surface() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(3.0),
            Outline::rectangle([1.5, 0.5], [2.5, 1.5]),
            1.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::yz(3.0),
                Outline::rectangle([1.0, 0.0], [3.0, 2.0]),
                0.99999999,
            )),
            Step::add(Leaf::prism(
                Plane::xy(6.0),
                Outline::circle([6.0, 1.0], 2.5),
                -8.0,
            )),
            Step::cut(Leaf::prism(
                Plane::yz(3.0),
                Outline::rectangle([6.0, 10.0], [7.0, 14.0]),
                1.00000001,
            )),
        ],
    ));
}

/// Campaigns 8b and 8c, the profile draw, on the kernel and through the
/// application's body: Uncrossed.
#[test]
fn seed_8529592_two_faces_a_hair_apart_crossed_by_a_third_surface() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(12.5),
            Outline::rectangle([0.0, 0.0], [38.0, 15.0]),
            43.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::yz(12.5),
                Outline::circle([5.0, 5.0], 5.0),
                85.0,
            )),
            Step::add(Leaf::prism(
                Plane::yz(12.50000005),
                Outline::rounded([0.0, 0.0], [10.0, 25.0], 5.0),
                5.0,
            )),
            Step::cut(Leaf::prism(
                Plane::yz(12.499999),
                Outline::circle([2.5, 5.0000001], 2.5),
                50.0,
            )),
        ],
    ));
}

/// Campaigns 8b and 8c, the profile draw, on the kernel and through the
/// application's body: Uncrossed.
#[test]
fn seed_8534613_two_faces_a_hair_apart_crossed_by_a_third_surface() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(1.0),
            Outline::slot([2.0, 5.0], [2.0, 2.0], 3.0),
            6.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::yz(0.99999999),
                Outline::slot([2.0, 5.0], [2.0, 1.5], 1.5),
                10.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(1.0),
                Outline::circle([2.0, 5.0], 1.5),
                11.0,
            )),
        ],
    ));
}

/// Campaigns 8b and 8c, the profile draw, on the kernel and through the
/// application's body: Uncrossed.
///
/// Was ignored as: plane-a-hair: the line two walls of one radius cross along
/// stands a tolerance from a block's side, and the corners either wall makes on
/// the side are merged off the other: a line a hair from a surface, two faces
/// drawn crossing. Holds since the triangles of two faces folded onto each
/// other across an edge are cut the other way, round 3 of #536.
#[test]
fn seed_8550377_two_faces_a_hair_apart_crossed_by_a_third_surface() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xz(60.0), Outline::circle([240.0, 180.0], 60.0), 30.0),
        vec![
            Step::add(Leaf::prism(
                Plane::xz(60.000006),
                Outline::rectangle([225.0000003, 165.0], [255.0000003, 315.0]),
                60.0,
            )),
            Step::add(Leaf::prism(
                Plane::xz(60.000005699999996),
                Outline::circle([210.0, 180.0], 60.0),
                120.0,
            )),
        ],
    ));
}
