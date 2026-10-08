//! The crescent's tip under a plane, as the campaigns of 5 October 2026
//! found it on #526's branch (8a, 8b and 8c in `docs/exact-kernel-journal.md`),
//! each seed shrunk.
//!
//! Two walls of one radius whose axes stand a hair apart, further than
//! decision 8's hair, so that they are not taken for one: they cross along
//! two rulings at a grazing angle, and a plane touches both, or passes
//! through the crescent's tip — a rounded corner, a slot's end or a disc
//! beside its twin, almost always. Every case failed the same way at every
//! fineness the triangles are drawn at: the band of the tip laid out no rule
//! of its own, and the operation declined or kept two faces on the same
//! vertices. Round 6's review left the family wanting that rule
//! (`docs/exact-kernel-failures.md`); #528 gave it, the two walls decided to
//! cross at a grazing angle once and their band laid out as decision 9 lays
//! a touch's (`docs/exact-kernel.md`). A case it leaves failing is ignored
//! with the reason, most of them not the tip's.

// The drawing, the promise and the checks are the campaigns'; this file
// only builds cases and holds them.
#[allow(dead_code, unused_imports)]
mod random_solids;

use random_solids::{Case, Leaf, Outline, Plane, Step};

// Two walls of one radius a hair apart, a plane at the tip: declined, before
// #528, at a tie ordering arcs round a corner (overlay/star.rs).

/// Campaign 8c, the profile draw through the application's body: Answers.
/// Shrunk through the application's body.
#[test]
fn seed_8560301_two_walls_of_one_radius_a_hair_apart_under_a_plane_touching_both() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(10.0),
            Outline::rectangle([-5.0, 32.5], [17.5, 37.5]),
            8.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(15.0),
                Outline::circle([0.0, 30.0], 7.5),
                40.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(10.0),
                Outline::circle([1e-5, 30.0], 7.5),
                30.0,
            )),
        ],
    ));
}

/// Campaign 8c, the profile draw through the application's body: Answers.
/// Shrunk through the application's body.
#[test]
fn seed_8561315_two_walls_of_one_radius_a_hair_apart_under_a_plane_touching_both() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(-1.5),
            Outline::rectangle([-2.0, 3.0], [2.0, 8.0]),
            1.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xz(-1.5),
                Outline::circle([0.0, 4.50001], 2.0),
                2.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(-1.5),
                Outline::circle([0.0, 4.5], 2.0),
                2.0,
            )),
        ],
    ));
}

/// Campaign 8c, the profile draw through the application's body: Answers.
/// Shrunk through the application's body.
#[test]
fn seed_8562728_two_walls_of_one_radius_a_hair_apart_under_a_plane_touching_both() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(195.0),
            Outline::ring([300.0, 120.0], 90.0, 30.0),
            -240.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(165.0),
                Outline::rectangle([270.0, 90.0], [495.0, 150.0]),
                -480.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(165.0),
                Outline::ring([299.99999, 120.0], 75.0, 30.0),
                -480.0,
            )),
        ],
    ));
}

/// Campaign 8c, the profile draw through the application's body: Answers.
/// Shrunk through the application's body.
#[test]
fn seed_8571051_two_walls_of_one_radius_a_hair_apart_under_a_plane_touching_both() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(0.0),
            Outline::rectangle([0.5, 9.0], [7.0, 12.5]),
            9.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::yz(-2e-7),
                Outline::slot([5.5, 10.75], [2.0, 10.75], 1.5),
                9.0,
            )),
            Step::add(Leaf::prism(
                Plane::yz(-2e-7),
                Outline::circle([2.0, 10.75001], 1.5),
                9.0,
            )),
        ],
    ));
}

/// Campaign 8c, the profile draw through the application's body: Answers.
/// Shrunk through the application's body.
#[test]
fn seed_8571878_two_walls_of_one_radius_a_hair_apart_under_a_plane_touching_both() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(4.0),
            Outline::rectangle([-1.0, 1.0], [9.0, 10.0]),
            5.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::yz(8.0),
                Outline::ring([6.0, 6.5], 5.5, 2.5),
                2.0,
            )),
            Step::cut(Leaf::prism(
                Plane::yz(7.0),
                Outline::circle([6.00001, 6.5], 2.5),
                3.0,
            )),
        ],
    ));
}

/// Campaign 8c, the profile draw through the application's body: Answers.
/// Shrunk through the application's body.
#[test]
fn seed_8583460_two_walls_of_one_radius_a_hair_apart_under_a_plane_touching_both() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(2.0),
            Outline::rectangle([6.0, 1.0], [11.0, 4.5]),
            8.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(3.0),
                Outline::ring([10.0, 3.0], 3.5, 1.5),
                4.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(3.0),
                Outline::circle([9.99999, 3.0], 1.5),
                2.0,
            )),
        ],
    ));
}

// Two walls of one radius a hair apart, a plane at the tip: declined, before
// #528, at a tie placing a loop of the overlay (overlay/partition.rs).

/// Campaign 8a, the square draw: Answers.
#[test]
fn seed_8032898_two_walls_of_one_radius_a_hair_apart_beside_a_third_surface() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::yz(5.0), Outline::circle([0.0, 4.0], 6.0), 5.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::yz(5.0),
                Outline::circle([1e-5, 4.0], 6.0),
                9.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(0.0),
                Outline::circle([4.0, 7.0], 5.0),
                -9.0,
            )),
        ],
    ));
}

/// Campaigns 8b and 8c, the profile draw, on the kernel and through the
/// application's body: Answers.
#[test]
fn seed_8539696_two_walls_of_one_radius_a_hair_apart_under_a_plane_touching_both() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(8.0), Outline::circle([9.0, 3.0], 5.0), 7.0),
        vec![
            Step::add(Leaf::prism(
                Plane::yz(6.0),
                Outline::slot([8.0, 6.0], [12.0, 6.0], 3.0),
                8.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(8.0),
                Outline::rounded([4.00001, -2.0], [14.00001, 8.0], 5.0),
                7.0,
            )),
        ],
    ));
}

/// Campaigns 8b and 8c, the profile draw, on the kernel and through the
/// application's body: Answers.
#[test]
fn seed_8541588_two_walls_of_one_radius_a_hair_apart_under_a_plane_touching_both() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(1.5),
            Outline::slot([1.0, 2.0], [1.0, 5.5], 2.5),
            6.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(1.0),
                Outline::rounded([5.0, 2.0], [12.5, 4.5], 1.25),
                10.0,
            )),
            Step::cut(Leaf::prism(
                Plane::yz(0.5),
                Outline::circle([1.0, 5.50001], 2.5),
                11.0,
            )),
        ],
    ));
}

/// Campaigns 8b and 8c, the profile draw, on the kernel and through the
/// application's body: Answers.
#[test]
fn seed_8544067_two_walls_of_one_radius_a_hair_apart_under_a_plane_touching_both() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::yz(0.0), Outline::circle([7.0, 7.0], 5.0), 10.0),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(0.0),
                Outline::rounded([9.0, 7.0], [14.5, 13.5], 0.5),
                7.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(-1.0),
                Outline::circle([9.5, 7.49999], 0.5),
                7.0,
            )),
        ],
    ));
}

/// Campaign 8c, the profile draw through the application's body: Answers.
/// Shrunk through the application's body.
#[test]
fn seed_8568600_two_walls_of_one_radius_a_hair_apart_under_a_plane_touching_both() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(30.0), Outline::circle([210.0, 60.0], 60.0), 195.0),
        vec![
            Step::add(Leaf::prism(
                Plane::xz(29.999994),
                Outline::circle([210.0, 15.0], 60.0),
                195.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(29.999994),
                Outline::rounded([150.00001, -45.0], [360.00001, 75.0], 60.0),
                195.0,
            )),
        ],
    ));
}

// Two walls of one radius a hair apart, a plane at the tip: declined, before
// #528, an edge run more one way than the other (assembly.rs).

/// Campaigns 8b and 8c, the profile draw, on the kernel and through the
/// application's body: Answers.
#[test]
fn seed_8500450_two_walls_of_one_radius_a_hair_apart_under_a_plane_touching_both() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(2.0),
            Outline::slot([1.0, 8.0], [1.0, 8.5], 0.5),
            3.0,
        ),
        vec![Step::add(Leaf::prism(
            Plane::xy(1.0),
            Outline::rounded([0.50000001, 8.0], [3.00000001, 10.0], 0.5),
            2.0,
        ))],
    ));
}

/// Campaigns 8b and 8c, the profile draw, on the kernel and through the
/// application's body: Answers.
#[test]
fn seed_8506582_two_walls_of_one_radius_a_hair_apart_under_a_plane_touching_both() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(150.0),
            Outline::circle([120.0, 300.0], 75.0),
            225.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(120.0),
                Outline::rounded([44.99999, 225.0], [254.99999, 375.0], 75.0),
                45.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(90.0),
                Outline::circle([119.99999, 300.0], 75.0),
                135.0,
            )),
        ],
    ));
}

/// Campaigns 8b and 8c, the profile draw, on the kernel and through the
/// application's body: Answers.
#[test]
fn seed_8532939_two_walls_of_one_radius_a_hair_apart_under_a_plane_touching_both() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(4.0), Outline::circle([9.0, 8.0], 4.0), 5.0),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(4.00000001),
                Outline::slot([9.00000001, 8.0], [6.000000010000001, 8.0], 4.0),
                10.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(-1.0),
                Outline::rectangle([3.0, 8.0], [7.0, 12.0]),
                7.0,
            )),
        ],
    ));
}

/// Campaigns 8b and 8c, the profile draw, on the kernel and through the
/// application's body: Answers.
#[test]
fn seed_8533871_two_walls_of_one_radius_a_hair_apart_under_a_plane_touching_both() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(-1.0),
            Outline::rectangle([2.0, 6.0], [10.0, 7.0]),
            7.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xz(2.0),
                Outline::ring([4.0, 4.0], 6.0, 2.0),
                5.0,
            )),
            Step::add(Leaf::prism(
                Plane::xz(3.0),
                Outline::circle([4.00001, 4.0], 2.0),
                9.0,
            )),
        ],
    ));
}

/// Campaigns 8b and 8c, the profile draw, on the kernel and through the
/// application's body: Answers.
#[test]
fn seed_8534200_two_walls_of_one_radius_a_hair_apart_under_a_plane_touching_both() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(10.0),
            Outline::rectangle([17.5, 35.0], [57.5, 50.0]),
            48.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xz(15.0),
                Outline::circle([22.5, 40.00001], 5.0),
                95.0,
            )),
            Step::add(Leaf::prism(
                Plane::xz(15.0000003),
                Outline::circle([22.5, 40.0], 5.0),
                95.0,
            )),
        ],
    ));
}

/// Campaigns 8b and 8c, the profile draw, on the kernel and through the
/// application's body: Answers.
#[test]
fn seed_8547716_two_walls_of_one_radius_a_hair_apart_under_a_plane_touching_both() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xz(4.0), Outline::ring([7.0, 7.0], 4.0, 2.5), 7.0),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(4.0),
                Outline::rectangle([5.0, 2.0], [9.0, 6.0]),
                5.5,
            )),
            Step::add(Leaf::prism(
                Plane::xz(4.0),
                Outline::circle([7.00001, 7.0], 2.5),
                5.0,
            )),
        ],
    ));
}

/// Campaigns 8b and 8c, the profile draw, on the kernel and through the
/// application's body: Answers.
#[test]
fn seed_8553263_two_walls_of_one_radius_a_hair_apart_under_a_plane_touching_both() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xz(6.5), Outline::ring([3.0, 4.0], 2.5, 0.5), 5.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xz(7.5000002),
                Outline::rectangle([-0.5, -0.5], [6.0, 4.5]),
                4.0,
            )),
            Step::add(Leaf::prism(
                Plane::xz(7.5),
                Outline::circle([3.0000002, 4.0], 0.5),
                5.0,
            )),
        ],
    ));
}

/// Campaign 8c, the profile draw through the application's body: Answers.
/// Shrunk through the application's body.
#[test]
fn seed_8563281_two_walls_of_one_radius_a_hair_apart_under_a_plane_touching_both() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(-30.0),
            Outline::rectangle([0.0, 285.0], [195.0, 375.0]),
            240.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(-30.0),
                Outline::circle([15.00001, 299.9999997], 15.0),
                480.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(-30.0),
                Outline::circle([15.0, 300.0], 15.0),
                480.0,
            )),
        ],
    ));
}

/// Campaign 8c, the profile draw through the application's body: Answers.
/// Shrunk through the application's body.
#[test]
fn seed_8565492_two_walls_of_one_radius_a_hair_apart_under_a_plane_touching_both() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(0.0),
            Outline::rectangle([90.0, -30.0], [330.0, 210.0]),
            -120.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(90.0),
                Outline::circle([180.0, 225.0], 105.0),
                -30.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(59.9999997),
                Outline::rounded([74.9999997, 120.0], [299.9999997, 330.0], 105.0),
                -210.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(-30.0),
                Outline::rectangle([75.0, -165.0], [345.0, 105.0]),
                240.0,
            )),
        ],
    ));
}

/// Campaign 8c, the profile draw through the application's body: Answers.
/// Shrunk through the application's body.
#[test]
fn seed_8567503_two_walls_of_one_radius_a_hair_apart_under_a_plane_touching_both() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(20.0), Outline::ring([30.0, 15.0], 8.0, 5.0), 28.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(20.0),
                Outline::slot([30.00000005, 15.0], [30.00000005, 22.5], 5.0),
                48.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(20.0),
                Outline::rectangle([24.99999995, 10.0], [42.49999995, 30.0]),
                20.0,
            )),
        ],
    ));
}

/// Campaign 8c, the profile draw through the application's body: Answers.
/// Shrunk through the application's body.
#[test]
fn seed_8569734_two_walls_of_one_radius_a_hair_apart_under_a_plane_touching_both() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(2.0), Outline::circle([7.0, 5.0], 0.5), 6.0),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(1.99999999),
                Outline::slot([6.99999999, 5.0], [2.99999999, 5.0], 0.5),
                10.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(-2.0),
                Outline::rectangle([-3.0, 0.0], [5.0, 8.0]),
                3.0,
            )),
        ],
    ));
}

/// Campaign 8c, the profile draw through the application's body: Answers.
/// Shrunk through the application's body.
#[test]
fn seed_8572596_two_walls_of_one_radius_a_hair_apart_under_a_plane_touching_both() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(35.0), Outline::circle([20.0, 30.0], 27.5), 38.0),
        vec![Step::add(Leaf::prism(
            Plane::xy(35.0000001),
            Outline::slot([20.0000001, 30.0], [20.0000001, 10.0], 27.5),
            75.0,
        ))],
    ));
}

/// Campaign 8c, the profile draw through the application's body: Answers.
/// Shrunk through the application's body.
#[test]
fn seed_8573071_two_walls_of_one_radius_a_hair_apart_under_a_plane_touching_both() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(0.0), Outline::circle([210.0, 90.0], 15.0), 75.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xz(120.0),
                Outline::circle([210.0, 45.0], 30.0),
                45.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(-0.0),
                Outline::circle([209.999994, 90.0], 15.0),
                285.0,
            )),
        ],
    ));
}

/// Campaign 8c, the profile draw through the application's body: Answers.
/// Shrunk through the application's body.
#[test]
fn seed_8575631_two_walls_of_one_radius_a_hair_apart_under_a_plane_touching_both() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(1.0),
            Outline::rectangle([0.0, 5.0], [7.0, 8.0]),
            2.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xz(3.0),
                Outline::circle([1.5, 6.50000001], 1.5),
                4.0,
            )),
            Step::add(Leaf::prism(
                Plane::xz(2.0000002),
                Outline::rectangle([3.0, 1.0], [5.0, 4.0]),
                -9.0,
            )),
            Step::add(Leaf::prism(
                Plane::xz(4.0),
                Outline::rounded([-1e-8, 5.00000001], [3.99999999, 8.00000001], 1.5),
                8.0,
            )),
        ],
    ));
}

/// Campaign 8c, the profile draw through the application's body: Answers.
/// Shrunk through the application's body.
#[test]
#[ignore = "crescent-tip: left: a ring's wall square to two walls of one radius a hair apart crosses each tens of tolerances from the other inside their band, and the band lays no corner where the curves it meets them along cross a third surface: an edge used more often one way than the other"]
fn seed_8586384_two_walls_of_one_radius_a_hair_apart_beside_a_third_surface() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(0.0),
            Outline::ring([180.0, 180.0], 75.0, 30.0),
            135.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(90.0),
                Outline::ring([30.0, 60.0], 105.0, 90.0),
                90.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(90.0),
                Outline::circle([29.99999, 60.0], 90.0),
                180.0,
            )),
        ],
    ));
}

// Two walls of one radius a hair apart, a plane at the tip: two faces drawn
// crossing at every fineness, before #528.

/// Campaign 8a, the square draw: Uncrossed.
#[test]
fn seed_8021944_two_walls_of_one_radius_a_hair_apart_beside_a_third_surface() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(20.0),
            Outline::rectangle([35.0, 50.0], [55.0, 73.0]),
            23.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(20.0),
                Outline::circle([55.0, 61.25], 2.5),
                45.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(15.0),
                Outline::circle([54.99999, 61.25], 2.5),
                45.0,
            )),
        ],
    ));
}

/// Campaign 8a, the square draw: Uncrossed.
#[test]
fn seed_8022576_two_walls_of_one_radius_a_hair_apart_beside_a_third_surface() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(1.9999998),
            Outline::rectangle([7.0, 2.0], [13.0, 5.0]),
            10.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::yz(0.9999998000000001),
                Outline::circle([13.0, 3.25], 0.5),
                20.0,
            )),
            Step::cut(Leaf::prism(
                Plane::yz(0.9999997000000002),
                Outline::circle([13.00001, 3.25], 0.5),
                20.0,
            )),
        ],
    ));
}

/// Campaign 8a, the square draw: Uncrossed.
#[test]
fn seed_8084143_two_walls_of_one_radius_a_hair_apart_beside_a_third_surface() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(3.0), Outline::circle([6.0, 1.0], 3.5), 9.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(2.0),
                Outline::circle([6.00001, 1.0], 3.5),
                18.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(-1.0),
                Outline::rectangle([4.0, 6.0], [6.0, 11.0]),
                6.0,
            )),
        ],
    ));
}

/// Campaigns 8b and 8c, the profile draw, on the kernel and through the
/// application's body: Uncrossed.
#[test]
fn seed_8500593_two_walls_of_one_radius_a_hair_apart_under_a_plane_touching_both() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(4.0),
            Outline::slot([2.5, 1.0], [4.0, 1.0], 2.5),
            2.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xz(3.99999999),
                Outline::circle([4.0, 0.99999], 2.5),
                3.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(3.99999999),
                Outline::rectangle([2.0, 1.0], [7.0, 5.0]),
                6.0,
            )),
        ],
    ));
}

/// Campaigns 8b and 8c, the profile draw, on the kernel and through the
/// application's body: Uncrossed.
#[test]
fn seed_8505079_two_walls_of_one_radius_a_hair_apart_beside_a_third_surface() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(-2.0), Outline::circle([1.0, 0.0], 0.5), 5.5),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(-2.0),
                Outline::circle([0.99999, 0.0], 0.5),
                6.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(6.5),
                Outline::circle([3.0, 2.0], 2.5),
                8.0,
            )),
        ],
    ));
}

/// Campaigns 8b and 8c, the profile draw, on the kernel and through the
/// application's body: Uncrossed.
#[test]
fn seed_8509502_two_walls_of_one_radius_a_hair_apart_beside_a_third_surface() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(7.0),
            Outline::rectangle([7.0, 1.0], [10.5, 7.5]),
            3.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xz(8.00001),
                Outline::circle([10.5, 4.25], 0.5),
                6.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(9.00001),
                Outline::circle([10.50001, 4.25], 0.5),
                12.0,
            )),
        ],
    ));
}

/// Campaigns 8b and 8c, the profile draw, on the kernel and through the
/// application's body: Uncrossed.
#[test]
fn seed_8512856_two_walls_of_one_radius_a_hair_apart_under_a_plane_touching_both() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::yz(1.5), Outline::ring([0.0, 6.0], 3.0, 1.0), 6.0),
        vec![
            Step::add(Leaf::prism(
                Plane::yz(1.50000006),
                Outline::circle([-1e-5, 6.0], 1.0),
                7.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(5.0),
                Outline::circle([6.0, 5.0], 6.0),
                7.0,
            )),
        ],
    ));
}

/// Campaigns 8b and 8c, the profile draw, on the kernel and through the
/// application's body: Uncrossed.
#[test]
fn seed_8518778_two_walls_of_one_radius_a_hair_apart_under_a_plane_touching_both() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(3.0),
            Outline::slot([3.0, 4.0], [6.5, 4.0], 2.5),
            10.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::yz(2.00000001),
                Outline::circle([6.5, 3.99999], 2.5),
                10.0,
            )),
            Step::cut(Leaf::prism(
                Plane::yz(1.99999981),
                Outline::ring([10.0, 4.0], 4.0, 1.0),
                2.0,
            )),
        ],
    ));
}

/// Campaigns 8b and 8c, the profile draw, on the kernel and through the
/// application's body: Uncrossed.
#[test]
fn seed_8519620_two_walls_of_one_radius_a_hair_apart_under_a_plane_touching_both() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::yz(5.0), Outline::circle([7.0, 2.0], 2.0), -7.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::yz(4.9999999),
                Outline::circle([6.9999998, 2.0], 2.0),
                -7.0,
            )),
            Step::cut(Leaf::prism(
                Plane::yz(4.9999999),
                Outline::rectangle([5.0, 0.0], [11.0, 4.0]),
                1.0,
            )),
        ],
    ));
}

/// Campaigns 8b and 8c, the profile draw, on the kernel and through the
/// application's body: Uncrossed.
#[test]
fn seed_8523312_two_walls_of_one_radius_a_hair_apart_under_a_plane_touching_both() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(1.0),
            Outline::slot([7.0, 7.5], [11.0, 7.5], 2.0),
            9.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(3.0),
                Outline::rectangle([10.0, 0.0], [14.5, 7.5]),
                9.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(0.0),
                Outline::circle([11.0, 7.50001], 2.0),
                18.0,
            )),
        ],
    ));
}

/// Campaigns 8b and 8c, the profile draw, on the kernel and through the
/// application's body: Uncrossed.
#[test]
fn seed_8535148_two_walls_of_one_radius_a_hair_apart_under_a_plane_touching_both() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(-60.0),
            Outline::ring([210.0, 120.0], 75.0, 15.0),
            -135.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(-60.0),
                Outline::circle([209.999994, 120.0], 15.0),
                -225.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(-60.0),
                Outline::rectangle([195.0, 105.0], [390.0, 135.0]),
                225.0,
            )),
        ],
    ));
}

/// Campaigns 8b and 8c, the profile draw, on the kernel and through the
/// application's body: Uncrossed.
#[test]
fn seed_8535914_two_walls_of_one_radius_a_hair_apart_under_a_plane_touching_both() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(3.0),
            Outline::rounded([0.0, 10.0], [3.0, 17.0], 1.0),
            1.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(3.0),
                Outline::circle([1.0, 11.0], 1.0),
                2.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(2.0),
                Outline::circle([0.99999999, 10.99999], 1.0),
                4.0,
            )),
        ],
    ));
}

/// Campaigns 8b and 8c, the profile draw, on the kernel and through the
/// application's body: Uncrossed.
#[test]
fn seed_8540477_two_walls_of_one_radius_a_hair_apart_under_a_plane_touching_both() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(0.0),
            Outline::ring([17.5, 15.0], 25.0, 10.0),
            38.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xz(0.0),
                Outline::circle([17.50001, 15.0], 10.0),
                38.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(5.0),
                Outline::rectangle([8.0, -13.0], [63.0, 43.0]),
                15.0,
            )),
        ],
    ));
}

/// Campaigns 8b and 8c, the profile draw, on the kernel and through the
/// application's body: Uncrossed.
#[test]
fn seed_8545376_two_walls_of_one_radius_a_hair_apart_under_a_plane_touching_both() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(1.0), Outline::circle([2.0, 4.0], 4.0), -7.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(1.0),
                Outline::circle([2.00001, 4.0], 4.0),
                -6.9999998,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(-2.0),
                Outline::rectangle([0.0, 0.0], [8.0, 3.0]),
                3.0,
            )),
        ],
    ));
}

/// Campaigns 8b and 8c, the profile draw, on the kernel and through the
/// application's body: Uncrossed.
#[test]
fn seed_8546580_two_walls_of_one_radius_a_hair_apart_under_a_plane_touching_both() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(7.0), Outline::circle([8.0, 7.0], 4.0), 6.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(7.0),
                Outline::rounded([3.99999, 3.0], [11.99999, 11.0], 4.0),
                11.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(7.0),
                Outline::rectangle([8.0, 10.0], [16.0, 14.0]),
                4.0,
            )),
        ],
    ));
}

/// Campaigns 8b and 8c, the profile draw, on the kernel and through the
/// application's body: Uncrossed.
#[test]
fn seed_8550564_two_walls_of_one_radius_a_hair_apart_under_a_plane_touching_both() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xz(30.0), Outline::circle([10.0, 5.0], 17.5), 43.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xz(34.9999999),
                Outline::rounded([-7.50001, -12.5], [27.49999, 22.5], 17.5),
                43.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(-5.0),
                Outline::rectangle([10.0, 10.0], [18.0, 20.0]),
                28.0,
            )),
        ],
    ));
}

/// Campaigns 8b and 8c, the profile draw, on the kernel and through the
/// application's body: Uncrossed.
#[test]
fn seed_8552353_two_walls_of_one_radius_a_hair_apart_under_a_plane_touching_both() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(5.0),
            Outline::slot([9.0, 5.0], [8.5, 5.0], 3.0),
            7.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(5.0),
                Outline::circle([8.5, 4.99999], 3.0),
                5.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(5.0),
                Outline::rectangle([5.0, 5.0], [7.0, 8.0]),
                5.0,
            )),
        ],
    ));
}

/// Campaign 8c, the profile draw through the application's body: Uncrossed.
/// Shrunk through the application's body.
#[test]
fn seed_8554561_two_walls_of_one_radius_a_hair_apart_under_a_plane_touching_both() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(-1.0),
            Outline::slot([7.0, 5.0], [3.0, 5.0], 1.5),
            -8.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(5.0),
                Outline::rectangle([1.5, 4.5], [6.5, 9.5]),
                3.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(1e-7),
                Outline::circle([3.0, 4.99999], 1.5),
                -16.0,
            )),
        ],
    ));
}

/// Campaign 8c, the profile draw through the application's body: Uncrossed.
/// Shrunk through the application's body.
#[test]
fn seed_8555088_two_walls_of_one_radius_a_hair_apart_under_a_plane_touching_both() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(210.0),
            Outline::circle([210.0, 90.0], 45.0),
            165.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(210.0000006),
                Outline::slot([209.9999997, 90.0], [194.9999997, 90.0], 45.0),
                330.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(210.0),
                Outline::circle([210.0, 90.0], 45.0),
                330.0,
            )),
        ],
    ));
}

/// Campaign 8c, the profile draw through the application's body: Uncrossed.
/// Shrunk through the application's body.
///
/// Was ignored as: crescent-tip: left: the crescent's tip, ended on a slot's
/// cap a hair from the line a box's side touches it along, stands past the end
/// of that side's face, where no band is laid, and the cap's triangle reaching
/// it crosses the side's. Holds since the triangles of two faces folded onto
/// each other across an edge are cut the other way, round 3 of #536.
#[test]
fn seed_8555390_two_walls_of_one_radius_a_hair_apart_under_a_plane_touching_both() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(-1.0),
            Outline::slot([1.0, 3.0], [1.0, 5.5], 0.5),
            5.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::yz(-1.0),
                Outline::circle([1.0, 5.50001], 0.5),
                2.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(6.0),
                Outline::rectangle([1.5, 1.5], [9.5, 4.5]),
                -1.0,
            )),
        ],
    ));
}

/// Campaign 8c, the profile draw through the application's body: Uncrossed.
/// Shrunk through the application's body.
#[test]
fn seed_8557175_two_walls_of_one_radius_a_hair_apart_under_a_plane_touching_both() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(40.0),
            Outline::rectangle([0.0, 30.0], [30.0, 55.0]),
            18.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(30.0),
                Outline::rounded([1e-5, 30.0], [35.00001, 50.0], 10.0),
                70.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(35.0),
                Outline::ring([10.0, 40.0], 13.0, 10.0),
                23.0,
            )),
        ],
    ));
}

/// Campaign 8c, the profile draw through the application's body: Uncrossed.
/// Shrunk through the application's body.
#[test]
fn seed_8564239_two_walls_of_one_radius_a_hair_apart_under_a_plane_touching_both() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::yz(1.0), Outline::circle([8.0, 9.0], 2.0), 9.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::yz(0.99999),
                Outline::circle([7.99999, 9.0], 2.0),
                17.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(8.0),
                Outline::rectangle([-5.0, -1.0], [5.0, 9.0]),
                2.0,
            )),
        ],
    ));
}

/// Campaign 8c, the profile draw through the application's body: Uncrossed.
/// Shrunk through the application's body.
#[test]
fn seed_8566888_two_walls_of_one_radius_a_hair_apart_under_a_plane_touching_both() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(18.0), Outline::circle([50.0, 35.0], 27.5), -20.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(18.0),
                Outline::rounded([22.49999, 7.5], [77.49999, 62.5], 27.5),
                -20.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(12.0),
                Outline::rectangle([0.0, 7.5], [77.5, 62.5]),
                40.0,
            )),
        ],
    ));
}

/// Campaign 8c, the profile draw through the application's body: Uncrossed.
/// Shrunk through the application's body.
#[test]
fn seed_8567656_two_walls_of_one_radius_a_hair_apart_under_a_plane_touching_both() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(-5.0), Outline::circle([0.0, 25.0], 10.0), -43.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(-5.0),
                Outline::circle([-1e-6, 25.0], 10.0),
                -43.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(-10.0),
                Outline::rectangle([-13.0, -5.0], [30.0, 15.0]),
                43.0,
            )),
        ],
    ));
}

/// Campaign 8c, the profile draw through the application's body: Uncrossed.
/// Shrunk through the application's body.
#[test]
fn seed_8574405_two_walls_of_one_radius_a_hair_apart_under_a_plane_touching_both() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(8.0),
            Outline::rectangle([3.0, 1.0], [5.0, 4.0]),
            2.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(8.00000011),
                Outline::circle([3.5, 1.5], 0.5),
                2.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(7.0),
                Outline::rounded([3.00001, 1.0], [5.00001, 5.0], 0.5),
                4.0,
            )),
        ],
    ));
}

/// Campaign 8c, the profile draw through the application's body: Uncrossed.
/// Shrunk through the application's body.
#[test]
fn seed_8575724_two_walls_of_one_radius_a_hair_apart_under_a_plane_touching_both() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(-60.0),
            Outline::circle([300.0, 300.0], 30.0),
            210.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xz(-29.9999997),
                Outline::rectangle([300.0, 270.0], [435.0, 330.0]),
                210.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(-30.000006),
                Outline::circle([300.00001, 300.0], 30.0),
                210.0,
            )),
        ],
    ));
}

/// Campaign 8c, the profile draw through the application's body: Uncrossed.
/// Shrunk through the application's body.
#[test]
fn seed_8576469_two_walls_of_one_radius_a_hair_apart_under_a_plane_touching_both() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(40.0), Outline::circle([15.0, 25.0], 10.0), -23.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(40.0),
                Outline::rounded([4.999999, 15.0], [24.999999, 35.0], 10.0),
                -23.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(-10.0),
                Outline::rectangle([15.0, 3.0], [40.0, 28.0]),
                38.0,
            )),
        ],
    ));
}

/// Campaign 8c, the profile draw through the application's body: Uncrossed.
/// Shrunk through the application's body.
#[test]
fn seed_8579668_two_walls_of_one_radius_a_hair_apart_under_a_plane_touching_both() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(-1.0),
            Outline::rectangle([-4.0, -4.0], [8.0, 8.0]),
            4.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::yz(2e-7),
                Outline::circle([0.0, 9.0], 6.0),
                10.0,
            )),
            Step::add(Leaf::prism(
                Plane::yz(2e-7),
                Outline::slot([-2e-8, 9.0], [0.49999998, 9.0], 6.0),
                20.0,
            )),
        ],
    ));
}

/// Campaign 8c, the profile draw through the application's body: Uncrossed.
/// Shrunk through the application's body.
#[test]
fn seed_8579762_two_walls_of_one_radius_a_hair_apart_under_a_plane_touching_both() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(180.0),
            Outline::rounded([90.0, 150.0], [300.0, 270.0], 60.0),
            150.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xz(180.0),
                Outline::circle([150.0, 210.0], 60.0),
                270.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(180.0),
                Outline::rounded([89.99999, 150.0], [209.99999, 345.0], 60.0),
                540.0,
            )),
        ],
    ));
}

/// Campaign 8c, the profile draw through the application's body: Uncrossed.
/// Shrunk through the application's body.
#[test]
fn seed_8581871_two_walls_of_one_radius_a_hair_apart_under_a_plane_touching_both() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xz(3.5), Outline::circle([8.5, 3.0], 1.0), -3.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xz(3.5),
                Outline::rounded([7.50001, 2.0], [11.00001, 9.5], 1.0),
                -3.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(4.5),
                Outline::rectangle([8.0, 2.0], [14.0, 5.0]),
                6.0,
            )),
        ],
    ));
}

/// Campaign 8c, the profile draw through the application's body: Uncrossed.
/// Shrunk through the application's body.
#[test]
#[ignore = "crescent-tip: left, the triangles': two walls of one radius just past decision 8's hair stand within a fifth of a tolerance of each other over a tenth of a turn about their crossing, where the triangles withhold every step, and a wall's triangle spanning it crosses the slot's ceiling"]
fn seed_8585454_two_walls_of_one_radius_a_hair_apart_under_a_plane_touching_both() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(-60.0),
            Outline::rectangle([60.0, 90.0], [180.0, 210.0]),
            255.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(-60.0),
                Outline::circle([120.00001, 150.0], 15.0),
                510.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(60.0),
                Outline::slot([120.0, 150.0], [45.0, 150.0], 15.0),
                225.0,
            )),
        ],
    ));
}

/// Campaign 8c, the profile draw through the application's body: Uncrossed.
/// Shrunk through the application's body.
#[test]
fn seed_8586289_two_walls_of_one_radius_a_hair_apart_beside_a_third_surface() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(4.0), Outline::ring([1.0, 3.0], 5.0, 1.5), 7.0),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(4.0),
                Outline::circle([1.00001, 3.0], 1.5),
                1.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(3.0),
                Outline::rectangle([1.0, 0.0], [3.0, 6.0]),
                1.0,
            )),
        ],
    ));
}

/// Campaign 8c, the profile draw through the application's body: Uncrossed.
/// Shrunk through the application's body.
#[test]
#[ignore = "crescent-tip: left, the triangles': two walls of one radius just past decision 8's hair stand within a fifth of a tolerance of each other over a tenth of a turn about their crossing, where the triangles withhold every step, and a wall's triangle spanning it crosses the floor's"]
fn seed_8587307_two_walls_of_one_radius_a_hair_apart_under_a_plane_touching_both() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(240.0),
            Outline::rounded([90.0, 60.0], [120.0, 300.0], 15.0),
            -225.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(210.0),
                Outline::circle([105.0, 74.99999], 15.0),
                210.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(240.0),
                Outline::rounded([90.0, 60.0], [120.0, 300.0], 15.0),
                -135.0,
            )),
        ],
    ));
}

// What #528's band of two walls crossing at a grazing angle broke on the
// seeds of campaigns 8a and 8c before it held them, each shrunk while it held
// on the kernel before #528: they keep the band to its own line, and the
// curve a square wall meets one of the walls along off the other wall.

/// Campaign 8a's seeds on #528's first band: Uncrossed. The band of the
/// crescent's lower line took the corners of its upper tip, where they stand
/// within the tolerance of both walls as well, and laid its lines and corners
/// across a wall dipping a hair into both at the top.
#[test]
fn seed_8002613_two_walls_of_one_radius_a_hair_apart_their_tip_dipped_into_by_a_wall() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xz(-2.0), Outline::circle([3.5, 4.0], 5.0), 7.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xz(-2.0),
                Outline::circle([3.50001, 4.0], 5.0),
                6.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(-2.00000006),
                Outline::circle([3.5, 12.0], 3.0000002),
                13.0,
            )),
        ],
    ));
}

/// Campaign 8c's seeds on #528's first band: Uncrossed through the body,
/// Answers on the kernel. The curve a square wall meets one of the two walls
/// along, laid on the other over a stretch the band holds, was taken for one
/// the other carries all along, and bounded a sliver of it with its own.
#[test]
fn seed_8501866_two_walls_of_one_radius_a_hair_apart_crossed_by_a_square_wall() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::yz(2.0), Outline::circle([1.0, 3.0], 3.0), 8.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(-1.0),
                Outline::rectangle([6.0, 1.0], [11.0, 4.0]),
                8.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(4.0),
                Outline::circle([3.0, 3.0], 4.0),
                9.0,
            )),
            Step::add(Leaf::prism(
                Plane::yz(2.0),
                Outline::circle([1.00001, 3.0], 3.0),
                8.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(-1.0),
                Outline::rectangle([7.0, 2.0], [10.0, 3.0]),
                16.0,
            )),
        ],
    ));
}
