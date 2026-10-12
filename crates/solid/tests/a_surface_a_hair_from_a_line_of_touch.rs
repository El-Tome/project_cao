//! A third surface a hair from a line of touch, as the campaigns of 5 October
//! 2026 found it on #526's branch (8a, 8b and 8c in
//! `docs/exact-kernel-journal.md`), each seed shrunk.
//!
//! A plane and a wall, or two walls, touch along a line — decided to touch,
//! or a hair from it — and a third surface stands a hair from that line: a
//! plane crossing it (1-2), a wall touching along a line a hair beside it
//! (2-1), a rim grazing it, a wall of another radius crossing at a grazing
//! angle where two walls all but touch. Decision 9 lays the band out only
//! where the pair is decided to touch and something of the operation lands
//! in it; these are what it leaves. Every case fails the same way at every
//! fineness the triangles are drawn at: the kernel's.

// The drawing, the promise and the checks are the campaigns'; this file
// only builds cases and holds them.
#[allow(dead_code, unused_imports)]
mod random_solids;

use random_solids::{Case, Leaf, Outline, Plane, Step};

// A plane a hair from the line a plane touches a wall along: declined at a tie
// winding a region (selection.rs).

/// Campaigns 8b and 8c, the profile draw, on the kernel and through the
/// application's body: Answers.
#[test]
#[ignore = "line-of-touch: a third surface a hair from a line of touch, declined at a tie winding a region (selection.rs)"]
fn seed_8524695_a_plane_a_hair_from_the_line_a_plane_touches_a_wall_along() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::yz(-10.0), Outline::circle([23.0, 0.0], 13.0), 48.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(5.0),
                Outline::rounded([35.0, 10.0], [50.0, 30.0], 2.5),
                -40.0,
            )),
            Step::add(Leaf::prism(
                Plane::yz(-10.0000003),
                Outline::rectangle([10.0, -13.0], [35.0, 13.0]),
                47.5,
            )),
        ],
    ));
}

/// Campaigns 8b and 8c, the profile draw, on the kernel and through the
/// application's body: Answers.
#[test]
#[ignore = "line-of-touch: a third surface a hair from a line of touch, declined at a tie winding a region (selection.rs)"]
fn seed_8553519_a_plane_a_hair_from_the_line_a_plane_touches_a_wall_along() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::yz(3.0), Outline::circle([5.0, 7.0], 0.5), 5.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(6.0),
                Outline::rounded([6.0, 0.0], [12.0, 5.5], 1.0),
                2.0,
            )),
            Step::cut(Leaf::prism(
                Plane::yz(2.0),
                Outline::rectangle([3.0, 5.0], [7.0, 9.0]),
                4.99999,
            )),
        ],
    ));
}

// A plane a hair from the line a plane touches a wall along: declined, an edge
// run more one way than the other (assembly.rs).

/// Campaigns 8b and 8c, the profile draw, on the kernel and through the
/// application's body: Answers.
#[test]
fn seed_8520500_a_plane_a_hair_from_the_line_a_plane_touches_a_wall_along() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(180.0),
            Outline::rectangle([300.0, 90.0], [375.0, 195.0]),
            150.0,
        ),
        vec![Step::add(Leaf::prism(
            Plane::xz(179.9999997),
            Outline::slot([359.9999997, 142.5], [299.9999997, 142.5], 15.0),
            300.0,
        ))],
    ));
}

/// Campaign 8c, the profile draw through the application's body: Answers.
/// Shrunk through the application's body.
#[test]
fn seed_8566781_a_plane_a_hair_from_the_line_a_plane_touches_a_wall_along() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(3.0),
            Outline::rectangle([2.00000001, 0.5000000099999999], [7.00000001, 8.50000001]),
            3.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(3.99999999),
                Outline::rectangle([-1.0, -3.0], [8.0, 7.0]),
                -15.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(4.0),
                Outline::rounded([-1.0, -1.0], [7.0, 6.0], 3.0),
                -8.0,
            )),
        ],
    ));
}

// A plane a hair from the line a plane touches a wall along: two faces drawn
// crossing at every fineness.

/// Campaign 8a, the square draw: Uncrossed.
#[test]
fn seed_8010303_a_plane_a_hair_from_the_line_a_plane_touches_a_wall_along() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xz(5.0), Outline::circle([8.0, 8.0], 5.0), 3.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(7.0),
                Outline::rectangle([2.0, 1.0], [7.0, 5.0]),
                5.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(7.0),
                Outline::circle([7.00000001, 3.0], 2.0),
                9.0,
            )),
        ],
    ));
}

/// Campaign 8a, the square draw: Uncrossed.
#[test]
fn seed_8055558_a_plane_a_hair_from_the_line_a_plane_touches_a_wall_along() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(0.0),
            Outline::rectangle([30.0, 150.0], [90.0, 270.0]),
            225.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(0.0),
                Outline::rectangle([90.0, 90.0], [150.0, 330.0]),
                225.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(210.0),
                Outline::circle([270.0, 120.0], 180.0),
                240.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(-30.0000003),
                Outline::circle([90.0, 210.0], 90.0),
                150.0,
            )),
        ],
    ));
}

// A rim grazing the line a plane touches a wall along: declined at a tie
// placing a loop of the overlay (overlay/partition.rs).

/// Campaign 8c, the profile draw through the application's body: Answers.
/// Shrunk through the application's body.
#[test]
fn seed_8554524_a_rim_grazing_the_line_a_plane_touches_a_wall_along() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(6.0), Outline::circle([5.99999998, 0.0], 5.0), 8.0),
        vec![Step::add(Leaf::prism(
            Plane::xz(7.0),
            Outline::rounded([6.0, 4.0], [13.0, 8.5], 2.25),
            2.0,
        ))],
    ));
}

// A rim grazing the line a plane touches a wall along: declined at a tie
// winding a region (selection.rs).

/// Campaign 8a, the square draw: Answers.
///
/// Was ignored as: line-of-touch: a third surface a hair from a line of touch,
/// declined at a tie winding a region (selection.rs). Holds since the triangles
/// of two faces folded onto each other across an edge are cut the other way,
/// round 3 of #536.
#[test]
fn seed_8004524_a_rim_grazing_the_line_a_plane_touches_a_wall_along() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(3.0),
            Outline::rectangle([2.0, 3.0], [6.0, 7.0]),
            7.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(2.0),
                Outline::circle([6.00000002, 5.0], 0.5),
                7.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(7.0),
                Outline::circle([5.5, 7.0], 4.0),
                4.0,
            )),
        ],
    ));
}

// A rim grazing the line a plane touches a wall along: two faces drawn crossing
// at every fineness.

/// Campaigns 8b and 8c, the profile draw, on the kernel and through the
/// application's body: Uncrossed.
///
/// Was ignored as: line-of-touch: a third surface a hair from a line of touch,
/// two faces drawn crossing at every fineness. Holds since the triangles of two
/// faces folded onto each other across an edge are cut the other way, round 3
/// of #536.
#[test]
fn seed_8516406_a_rim_grazing_the_line_a_plane_touches_a_wall_along() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(1.0),
            Outline::rectangle([-5.0, -6.0], [7.0, 6.0]),
            8.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::yz(1.0),
                Outline::rounded([-5.00001, -6.0], [6.99999, 6.0], 6.0),
                7.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(-3.0),
                Outline::circle([2.0, 5.0], 4.0),
                13.0,
            )),
        ],
    ));
}

/// Campaign 8c, the profile draw through the application's body: Uncrossed.
/// Shrunk through the application's body.
///
/// Was ignored as: line-of-touch: a third surface a hair from a line of touch,
/// two faces drawn crossing at every fineness. Holds since the triangles of two
/// faces folded onto each other across an edge are cut the other way, round 3
/// of #536.
#[test]
fn seed_8569565_a_rim_grazing_the_line_a_plane_touches_a_wall_along() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(8.0),
            Outline::rectangle([-4.0, 2.0], [6.0, 12.0]),
            3.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::yz(7.00001),
                Outline::rounded([-4.00001, 2.0], [5.99999, 12.0], 5.0),
                5.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(5.0),
                Outline::circle([9.0, 4.0], 3.0),
                7.0,
            )),
        ],
    ));
}

// A third surface a hair from a line of touch: declined, an edge run more one
// way than the other (assembly.rs).

/// Campaign 8c, the profile draw through the application's body: Answers.
/// Shrunk through the application's body.
#[test]
fn seed_8582887_a_third_surface_a_hair_from_a_line_of_touch() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(1.0),
            Outline::rectangle([2.0, -2.0], [10.0, 6.0]),
            -7.0,
        ),
        vec![Step::add(Leaf::prism(
            Plane::xy(0.0),
            Outline::circle_from([6.0, 5.0], 5.0, 328.15191356852523),
            7.0,
        ))],
    ));
}

// A third surface a hair from a line of touch: two faces drawn crossing at
// every fineness.

/// Campaign 8a, the square draw: Uncrossed.
///
/// Was ignored as: line-of-touch: a third surface a hair from a line of touch,
/// two faces drawn crossing at every fineness. Holds since the triangles of two
/// faces folded onto each other across an edge are cut the other way, round 3
/// of #536.
#[test]
fn seed_8019535_a_third_surface_a_hair_from_a_line_of_touch() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(35.0),
            Outline::rectangle([42.5, 35.0], [65.0, 52.5]),
            -38.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(10.0),
                Outline::circle([50.0000001, 42.5], 12.5),
                8.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(5.0),
                Outline::rectangle([20.0, 20.0], [80.0, 80.0]),
                5.0,
            )),
        ],
    ));
}

// A wall a hair from the line a plane touches a wall along: declined at a tie
// ordering arcs round a corner (overlay/star.rs).

/// Campaigns 8b and 8c, the profile draw, on the kernel and through the
/// application's body: Answers.
#[test]
#[ignore = "line-of-touch: a third surface a hair from a line of touch, declined at a tie ordering arcs round a corner (overlay/star.rs)"]
fn seed_8502900_a_wall_a_hair_from_the_line_a_plane_touches_a_wall_along() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(2.0),
            Outline::rectangle([2.0, 2.0], [5.0, 9.0]),
            2.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::yz(4.0),
                Outline::circle([2.0, 5.0], 1.5),
                -2.0,
            )),
            Step::cut(Leaf::prism(
                Plane::yz(1.0),
                Outline::slot([4.00000001, 5.5], [3.50000001, 5.5], 0.5),
                9.0,
            )),
        ],
    ));
}

// A wall a hair from the line two walls touch along, or crossing a wall of
// another radius at a grazing angle: declined, an edge run more one way than
// the other (assembly.rs).

/// Campaign 8a, the square draw: Answers.
#[test]
#[ignore = "line-of-touch: a plane of the tool touching both walls of a crescent's tip a hair wide, which the body decided apart: a corner on each at one place, never merged (assembly.rs); the crescent tip's shape"]
fn seed_8079957_a_wall_a_hair_from_the_line_two_walls_touch_along() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::yz(5.0), Outline::circle([9.0, 1.0], 4.0), 8.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::yz(4.0000001),
                Outline::circle([9.0, 1.5], 3.49999998),
                8.0,
            )),
            Step::cut(Leaf::prism(
                Plane::yz(5.0),
                Outline::rectangle([6.0, 5.0], [13.0, 11.0]),
                16.0,
            )),
        ],
    ));
}

/// Campaigns 8b and 8c, the profile draw, on the kernel and through the
/// application's body: Answers.
#[test]
#[ignore = "line-of-touch: a third surface a hair from a line of touch, declined, an edge run more one way than the other (assembly.rs)"]
fn seed_8549117_a_wall_a_hair_from_the_line_two_walls_touch_along() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(240.0),
            Outline::slot([180.0, 120.0], [180.0, 255.0], 75.0),
            120.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::yz(119.9999997),
            Outline::rounded([179.9999997, 270.0], [329.9999997, 375.0], 30.0),
            135.0,
        ))],
    ));
}

/// Campaign 8c, the profile draw through the application's body: Answers.
/// Shrunk through the application's body.
#[test]
#[ignore = "line-of-touch: a plane of the tool touching both walls of a crescent's tip a hair wide, which the body decided apart: a corner on each at one place, never merged (assembly.rs); the crescent tip's shape"]
fn seed_8578812_a_wall_a_hair_from_the_line_two_walls_touch_along() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(20.0), Outline::ring([40.0, 20.0], 8.0, 5.0), 35.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(20.0),
                Outline::circle([47.5000001, 20.0], 2.5),
                45.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(20.0),
                Outline::rectangle([45.0, 0.0], [53.0, 40.0]),
                140.0,
            )),
        ],
    ));
}

// A wall a hair from the line two walls touch along, or crossing a wall of
// another radius at a grazing angle: two faces drawn crossing at every
// fineness.

/// Campaign 8a, the square draw: Uncrossed.
#[test]
fn seed_8010399_a_wall_a_hair_from_the_line_two_walls_touch_along() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(60.0),
            Outline::rectangle([75.0, 270.0], [165.0, 480.0]),
            300.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(60.0),
                Outline::circle([165.0000006, 375.0], 75.0),
                600.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(30.0),
                Outline::circle([165.0000006, 390.0], 60.0),
                1200.0,
            )),
        ],
    ));
}

/// Campaign 8a, the square draw: Uncrossed.
///
/// Was ignored as: line-of-touch: two walls crossing at a grazing angle along
/// two rulings closer than a grid step, each drawn by the one chord between
/// them (the triangles). Holds since the triangles of two faces folded onto
/// each other across an edge are cut the other way, round 3 of #536.
#[test]
fn seed_8039443_a_wall_a_hair_from_the_line_two_walls_touch_along() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(6.0), Outline::circle([5.9999998, 6.5], 1.5), 3.0),
        vec![Step::cut(Leaf::prism(
            Plane::xy(6.0),
            Outline::circle([8.0, 5.0], 4.0),
            3.0,
        ))],
    ));
}

/// Campaigns 8b and 8c, the profile draw, on the kernel and through the
/// application's body: Uncrossed.
#[test]
fn seed_8530908_a_wall_a_hair_from_the_line_two_walls_touch_along() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(-1.0),
            Outline::rectangle([8.0, 3.5], [14.5, 6.0]),
            9.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(-1.0),
                Outline::rounded([7.99999994, 3.5], [14.49999994, 6.0], 1.0),
                8.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(-1.0),
                Outline::ring([7.0, 3.0], 2.0, 1.5),
                4.0,
            )),
        ],
    ));
}

/// Campaign 8c, the profile draw through the application's body: Uncrossed.
/// Shrunk through the application's body.
///
/// Was ignored as: line-of-touch: a third surface a hair from a line of touch,
/// two faces drawn crossing at every fineness. Holds since the triangles of two
/// faces folded onto each other across an edge are cut the other way, round 3
/// of #536.
#[test]
fn seed_8565786_a_wall_a_hair_from_the_line_two_walls_touch_along() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(35.0), Outline::circle([10.0, 45.0], 12.5), 25.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(35.0),
                Outline::circle([25.0, 45.0], 2.5000001),
                24.9999999,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(35.0),
                Outline::circle([12.5, 45.0], 10.0),
                50.0,
            )),
        ],
    ));
}

/// Campaign 8c, the profile draw through the application's body: Uncrossed.
/// Shrunk through the application's body.
///
/// Was ignored as: line-of-touch: a third surface a hair from a line of touch,
/// two faces drawn crossing at every fineness. Holds since the triangles of two
/// faces folded onto each other across an edge are cut the other way, round 3
/// of #536.
#[test]
fn seed_8580382_a_wall_a_hair_from_the_line_two_walls_touch_along() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(15.0),
            Outline::rectangle([-25.0, 20.0], [35.0, 80.0]),
            18.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(15.0),
                Outline::circle([5.0, 98.0], 18.0),
                35.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(15.0),
                Outline::rounded([-12.49999995, 80.0], [27.50000005, 115.0], 17.5),
                10.0,
            )),
        ],
    ));
}

/// Campaign 8c, the profile draw through the application's body: Uncrossed.
/// Shrunk through the application's body.
///
/// Was ignored as: line-of-touch: a third surface a hair from a line of touch,
/// two faces drawn crossing at every fineness. Holds since the triangles of two
/// faces folded onto each other across an edge are cut the other way, round 3
/// of #536.
#[test]
fn seed_8586895_a_wall_a_hair_from_the_line_two_walls_touch_along() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::prism(
            Plane::xz(1.99999),
            Outline::rectangle([2.0, -2.0], [8.0, 4.0]),
            -14.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xz(1.99999),
                Outline::ring([5.00000002, 1.0], 3.0, 2.5),
                -28.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(2.0),
                Outline::rounded([4.5, 0.5], [7.5, 2.0], 0.5),
                10.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(1.99999),
                Outline::circle([5.0, 1.0], 2.5),
                2.0,
            )),
        ],
    ));
}

/// Campaign 8c, the profile draw through the application's body: Uncrossed.
/// Shrunk through the application's body.
///
/// Was ignored as: line-of-touch: a third surface a hair from a line of touch,
/// two faces drawn crossing at every fineness. Holds since the triangles of two
/// faces folded onto each other across an edge are cut the other way, round 3
/// of #536.
#[test]
fn seed_8587802_a_wall_a_hair_from_the_line_two_walls_touch_along() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xz(8.0), Outline::circle([6.0, 3.0], 2.5), -9.0),
        vec![Step::cut(Leaf::prism(
            Plane::xz(10.0000001),
            Outline::rounded([5.00000002, 2.0], [10.50000002, 5.5], 1.0),
            -11.0,
        ))],
    ));
}

// Held before #528, broken by the line-of-touch lane's corner on two planes:
// the like-for-like campaigns of #528 on the night's seeds, each case shrunk
// among those that still held before.

/// The profile draw, on the kernel and through the application's body:
/// Uncrossed. The rounded rectangle's corner, a hundredth of a micron off
/// the bore's wall, was taken for it and left the rectangle's sides where
/// they stood, a hair from the wall they touched; the rectangle now moves
/// onto the bore's wall with its corner (decision 8, #533).
#[test]
fn seed_8515425_a_rounded_rectangle_a_hair_from_the_line_a_bore_touches_a_disc_s_top_along() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(6.0), Outline::circle([5.5, 2.0], 2.5), 1.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::yz(7.5),
                Outline::circle([1.0, 7.0], 1.0),
                -10.0,
            )),
            Step::add(Leaf::prism(
                Plane::yz(7.4999998),
                Outline::rounded([1e-8, 6.0], [7.50000001, 8.5], 1.0),
                8.0,
            )),
        ],
    ));
}

/// The profile draw, on the kernel and through the application's body:
/// Closed. A corner on a disc's top and a block's side the disc's wall
/// touches, laid on the line the two planes share a hair off the wall, had
/// every rim of the wall sampled there, and the block top's edge along the
/// side ran past that sample: the top was left uncut. The edge takes it
/// (`tessellation/sampling/touches.rs`).
#[test]
fn seed_8542629_a_wall_touching_a_side_along_a_line_across_the_side_s_line_with_a_cap() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(0.0),
            Outline::rectangle([180.0, 60.0], [285.0, 300.0]),
            255.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(0.0),
                Outline::slot([255.0000003, 180.0], [210.0000003, 180.0], 30.0),
                255.0,
            )),
            Step::cut(Leaf::prism(
                Plane::yz(90.0),
                Outline::circle([300.0, 15.0], 120.0),
                180.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(0.0),
                Outline::circle([210.0000003, 180.0], 30.0),
                60.0,
            )),
        ],
    ));
}
