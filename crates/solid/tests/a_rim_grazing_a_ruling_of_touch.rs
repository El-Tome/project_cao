//! A square wall's rim grazing the ruling a plane touches a wall along, as
//! the campaigns of 5 October 2026 found it on #526's branch (8a, 8b and 8c
//! in `docs/exact-kernel-journal.md`), each seed shrunk.
//!
//! Two perpendicular walls; the cap of one stands on a plane touching the
//! other along a ruling, and the cap's rim passes that ruling a hair off.
//! Decision 6 keeps the rim's circle for the arc of the curve the two walls
//! meet along, and the other wall, which does not carry the circle, saw it
//! only as a chord standing further off than the tolerance, and declined the
//! operation as unsupported. It now sees the circle as the curve the rim's
//! own wall meets it along, traced as that curve's arc is. The 3-2 cases of
//! `the_tangency_band.rs` once ignored as "a circle kept for an arc of the
//! meet" are the same.

// The drawing, the promise and the checks are the campaigns'; this file
// only builds cases and holds them.
#[allow(dead_code, unused_imports)]
mod random_solids;

use random_solids::{Case, Leaf, Outline, Plane, Step};

// Declined where the rim's circle was laid on the other wall as a chord.

/// Campaign 8a, the square draw: Answers.
#[test]
fn seed_8039045_a_square_wall_s_rim_grazing_the_ruling_a_plane_touches_a_wall_along() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(8.0),
            Outline::rectangle([6.0, 1.0], [14.0, 9.0]),
            2.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::yz(8.00001),
                Outline::circle([9.9999998, 5.0], 2.0),
                -2.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(3.0),
                Outline::circle([3.0, 8.0], 5.0),
                4.0,
            )),
        ],
    ));
}

/// Campaigns 8b and 8c, the profile draw, on the kernel and through the
/// application's body: Answers.
#[test]
fn seed_8500599_a_square_wall_s_rim_grazing_the_ruling_a_plane_touches_a_wall_along() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(60.0), Outline::circle([180.0, 30.0], 75.0), 210.0),
        vec![Step::cut(Leaf::prism(
            Plane::xz(239.9999994),
            Outline::rounded([90.000006, 270.0], [270.000006, 465.0], 15.0),
            270.0,
        ))],
    ));
}

/// Campaigns 8b and 8c, the profile draw, on the kernel and through the
/// application's body: Answers.
#[test]
fn seed_8530220_a_square_wall_s_rim_grazing_the_ruling_a_plane_touches_a_wall_along() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(0.0),
            Outline::rectangle([20.0, 10.0], [55.0, 30.0]),
            38.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xz(-1e-5),
                Outline::rounded([20.000001, 10.0], [55.000001, 30.0], 7.5),
                38.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(15.0),
                Outline::circle([45.0, 0.0], 17.5),
                45.0,
            )),
        ],
    ));
}

/// Campaigns 8b and 8c, the profile draw, on the kernel and through the
/// application's body: Answers.
#[test]
fn seed_8530659_a_square_wall_s_rim_grazing_the_ruling_a_plane_touches_a_wall_along() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(30.0),
            Outline::rectangle([30.0, 5.0], [40.0, 25.0]),
            -50.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(30.0),
                Outline::rounded([29.99999, 5.0], [39.99999, 25.0], 2.5),
                -100.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(5.0),
                Outline::ring([35.0, 5.0], 17.5, 2.5),
                45.0,
            )),
        ],
    ));
}

/// Campaigns 8b and 8c, the profile draw, on the kernel and through the
/// application's body: Answers.
#[test]
fn seed_8544894_a_square_wall_s_rim_grazing_the_ruling_a_plane_touches_a_wall_along() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(-1.9999999),
            Outline::rectangle([8.0, 5.0], [15.0, 11.0]),
            -15.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(7.0),
                Outline::circle([7.5, 10.0], 5.5),
                -14.0,
            )),
            Step::add(Leaf::prism(
                Plane::xz(-1.9999999),
                Outline::rounded([7.99999998, 5.0], [14.99999998, 11.0], 2.0),
                -15.0,
            )),
        ],
    ));
}

/// Campaigns 8b and 8c, the profile draw, on the kernel and through the
/// application's body: Answers.
#[test]
fn seed_8548640_a_square_wall_s_rim_grazing_the_ruling_a_plane_touches_a_wall_along() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(35.0),
            Outline::rounded([12.50001, 2.5], [22.50001, 37.5], 2.5),
            10.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::xy(30.0),
            Outline::rounded([15.0, 25.0], [48.0, 63.0], 10.0),
            -27.5,
        ))],
    ));
}

/// Campaigns 8b and 8c, the profile draw, on the kernel and through the
/// application's body: Answers.
#[test]
fn seed_8550915_a_square_wall_s_rim_grazing_the_ruling_a_plane_touches_a_wall_along() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(-2.0), Outline::circle([6.00001, 3.0], 3.0), 6.0),
        vec![Step::cut(Leaf::prism(
            Plane::xz(6.0),
            Outline::ring([10.0, 3.0], 6.0, 4.0),
            7.0,
        ))],
    ));
}

/// Campaigns 8b and 8c, the profile draw, on the kernel and through the
/// application's body: Answers.
#[test]
fn seed_8552226_a_square_wall_s_rim_grazing_the_ruling_a_plane_touches_a_wall_along() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(1.0),
            Outline::rectangle([-1.0, 1.5], [5.0, 7.5]),
            -10.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xz(1.0),
                Outline::circle([1.99999, 4.5], 3.0),
                -20.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(5.0),
                Outline::circle([0.0, 9.0], 2.0),
                6.0,
            )),
        ],
    ));
}

/// Campaigns 8b and 8c, the profile draw, on the kernel and through the
/// application's body: Answers.
#[test]
fn seed_8553008_a_square_wall_s_rim_grazing_the_ruling_a_plane_touches_a_wall_along() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(40.0),
            Outline::rectangle([40.0, 15.0], [55.0, 28.0]),
            -23.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(40.0),
                Outline::rounded([39.9999997, 15.0], [54.9999997, 27.5], 2.5),
                -22.0,
            )),
            Step::add(Leaf::prism(
                Plane::xz(22.5),
                Outline::circle([50.0, 40.0], 2.5),
                25.0,
            )),
        ],
    ));
}

/// Campaign 8c, the profile draw through the application's body: Answers.
/// Shrunk through the application's body.
#[test]
fn seed_8578532_a_square_wall_s_rim_grazing_the_ruling_a_plane_touches_a_wall_along() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(8.0),
            Outline::rounded([6.9999999, 6.0], [12.9999999, 9.5], 1.5),
            -5.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::xz(8.0),
            Outline::circle([7.0, 5.0], 4.5),
            2.0,
        ))],
    ));
}
