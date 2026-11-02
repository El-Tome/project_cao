//! Closes #628.
//! - the shrunk seeds hold on the exact kernel and through the application's
//!   body — `a_ball_turned_300_cut_by_a_coaxial_tube_turned_270_is_drawn`,
//!   `a_quarter_ball_joined_to_a_coaxial_cone_turned_the_other_way_is_drawn`,
//!   `a_half_ball_on_a_cone_cut_by_a_coaxial_shaft_turned_90_from_the_other_plane_is_drawn`,
//!   `a_ball_turned_back_240_joined_to_a_coaxial_tube_turned_back_270_is_drawn`,
//!   `a_ball_turned_back_270_joined_to_the_same_ball_turned_270_from_the_other_plane_is_drawn`,
//!   `a_whole_ring_joined_to_a_coaxial_disc_turned_345_is_drawn`,
//!   `a_half_spindle_joined_to_a_coaxial_half_shaft_from_the_other_plane_is_drawn`,
//!   `a_half_ball_turned_back_joined_to_a_coaxial_half_shaft_from_the_other_plane_is_drawn`,
//!   `a_half_ball_joined_to_a_wider_coaxial_half_shaft_from_the_other_plane_is_drawn`;
//!   the tenth, 5576854214, is another family — a spindle's face holding one
//!   pole of two whose only loop is a slit, the boolean's — named so in
//!   `docs/exact-kernel-failures.md`
//! - no case `main` builds right is built wrong — no test: measured base
//!   against tip on #623's seed sets, in `docs/exact-kernel-failures.md`
//! - the rate measured again, the plain cases and the hairs apart — no test:
//!   measured, in `docs/exact-kernel-failures.md`
//!
//! A ball, or a ring, turned part of the way and combined with a body turned
//! about the same axis through another extent: the exact kernel built the
//! boolean since #623, and the triangles of one face stayed open — a
//! meridian parted by a vertex where a rim of another face meets it, a rim
//! elsewhere on the face a meridian ran past, an angle an ulp off between
//! two edges of one meridian — so the application refused the step as
//! `Undrawn`.

// The drawing, the promise and the checks are the campaigns'; this file
// only builds cases and holds them.
#[allow(dead_code, unused_imports)]
mod random_solids;

use random_solids::{Axis, Case, Leaf, Plane, Section, Step};

/// Seed 5576853450, shrunk: `Closed` on the exact kernel, `Undrawn` through
/// the application's body.
#[test]
fn a_ball_turned_300_cut_by_a_coaxial_tube_turned_270_is_drawn() {
    let case = Case::new(
        Leaf::turned(
            Plane::xy(-10.0),
            Axis::second(20.0),
            Section::bands(15.0, &[[2.5, 0.0, 7.5]])
                .sloping_to(&[[0.0, 0.0]])
                .bowed(&[[[0.0, 0.0, 0.0], [5.0, 0.0, 12.5]]]),
            300.0,
        ),
        vec![Step::cut(Leaf::turned(
            Plane::xy(-10.0),
            Axis::second(20.0),
            Section::bands(15.0, &[[3.0, 4.0, 13.0]]),
            270.0,
        ))],
    );
    random_solids::holds_exactly(&case);
    random_solids::holds_through_the_application(&case);
}

/// Seed 5576867141, shrunk: `Closed` on the exact kernel, `Undrawn` through
/// the application's body.
#[test]
fn a_quarter_ball_joined_to_a_coaxial_cone_turned_the_other_way_is_drawn() {
    let case = Case::new(
        Leaf::turned(
            Plane::xy(1.5),
            Axis::second(0.0),
            Section::bands(-1.0, &[[8.0, -0.0, -0.0]])
                .sloping_to(&[[-4.0, -0.0]])
                .bowed(&[[[4.0, -0.0, -5.0], [0.0, -0.0, -0.0]]]),
            90.0,
        ),
        vec![Step::add(Leaf::turned(
            Plane::yz(0.0),
            Axis::first(1.5).backwards(),
            Section::bands(-8.0, &[[2.0, 0.0, 3.0]]).sloping_to(&[[0.0, 6.0]]),
            -90.0,
        ))],
    );
    random_solids::holds_exactly(&case);
    random_solids::holds_through_the_application(&case);
}

/// Seed 5576850917, shrunk: `Closed` on the exact kernel, `Undrawn` through
/// the application's body.
#[test]
fn a_half_ball_on_a_cone_cut_by_a_coaxial_shaft_turned_90_from_the_other_plane_is_drawn() {
    let case = Case::new(
        Leaf::turned(
            Plane::xy(6.0),
            Axis::first(0.0).backwards(),
            Section::bands(-1.0, &[[0.5, 0.0, 1.5]])
                .sloping_to(&[[0.0, 0.0]])
                .bowed(&[[[0.0, 0.0, 0.0], [-3.0, 0.0, 2.5]]]),
            180.0,
        ),
        vec![Step::cut(Leaf::turned(
            Plane::xz(0.0),
            Axis::first(6.0),
            Section::bands(0.5, &[[5.0, 0.0, 1.0]]),
            90.0,
        ))],
    );
    random_solids::holds_exactly(&case);
    random_solids::holds_through_the_application(&case);
}

/// Seed 5576871098, shrunk: `Closed` on the exact kernel, `Undrawn` through
/// the application's body.
#[test]
fn a_ball_turned_back_240_joined_to_a_coaxial_tube_turned_back_270_is_drawn() {
    let case = Case::new(
        Leaf::turned(
            Plane::xy(30.0),
            Axis::first(0.0),
            Section::bands(-45.0, &[[30.0, -0.0, -0.0]])
                .bowed(&[[[-30.0, -0.0, -15.0], [0.0, -0.0, -0.0]]]),
            -240.0,
        ),
        vec![Step::add(Leaf::turned(
            Plane::xy(30.0),
            Axis::first(0.0).backwards(),
            Section::bands(15.0, &[[30.0, 15.0, 75.0]]),
            -270.0,
        ))],
    );
    random_solids::holds_exactly(&case);
    random_solids::holds_through_the_application(&case);
}

/// Seed 5576860117, shrunk: `Closed` on the exact kernel, `Undrawn` through
/// the application's body.
#[test]
fn a_ball_turned_back_270_joined_to_the_same_ball_turned_270_from_the_other_plane_is_drawn() {
    let case = Case::new(
        Leaf::turned(
            Plane::xy(5.0),
            Axis::first(0.0),
            Section::bands(-3.0, &[[4.0, -5.0, -0.0]])
                .sloping_to(&[[-3.0, -0.0]])
                .bowed(&[[[-3.0, -0.0, -5.0], [0.0, -0.0, -0.0]]]),
            -270.0,
        ),
        vec![Step::add(Leaf::turned(
            Plane::xz(0.0),
            Axis::first(5.0),
            Section::bands(-3.0, &[[4.0, 1.0, 5.0]])
                .sloping_to(&[[1.0, 3.0]])
                .bowed(&[[[0.0, 0.0, 0.0], [-3.0, 0.0, 5.0]]]),
            270.0,
        ))],
    );
    random_solids::holds_exactly(&case);
    random_solids::holds_through_the_application(&case);
}

/// Seed 5576862224, shrunk: `Closed` on the exact kernel, `Undrawn` through
/// the application's body.
#[test]
fn a_whole_ring_joined_to_a_coaxial_disc_turned_345_is_drawn() {
    let case = Case::new(
        Leaf::turned(
            Plane::yz(25.0),
            Axis::first(0.0),
            Section::bands(-12.5, &[[5.0, -12.5, -12.5]])
                .bowed(&[[[-10.0, -12.5, -2.5], [-10.0, -12.5, 2.5]]]),
            360.0,
        ),
        vec![Step::add(Leaf::turned(
            Plane::xy(0.0),
            Axis::second(25.0).backwards(),
            Section::bands(7.5, &[[10.0, 0.0, 15.0]]),
            345.0,
        ))],
    );
    random_solids::holds_exactly(&case);
    random_solids::holds_through_the_application(&case);
}

/// Seed 5576850021, shrunk: `Listed` on the exact kernel — a loop through
/// both poles, closed along neither floor — and `Undrawn` through the
/// application's body.
#[test]
fn a_half_spindle_joined_to_a_coaxial_half_shaft_from_the_other_plane_is_drawn() {
    let case = Case::new(
        Leaf::turned(
            Plane::xy(0.0),
            Axis::first(0.0),
            Section::bands(1.0, &[[4.0, 0.0, 0.0]]).bowed(&[[[0.0, 0.0, 0.0], [3.0, -1.5, 2.5]]]),
            180.0,
        ),
        vec![Step::add(Leaf::turned(
            Plane::xz(0.0),
            Axis::first(0.0),
            Section::bands(2.0, &[[2.0, 0.0, 3.0]]),
            180.0,
        ))],
    );
    random_solids::holds_exactly(&case);
    random_solids::holds_through_the_application(&case);
}

/// Seed 5576860674, shrunk: `Listed` on the exact kernel — a lune through
/// both poles, closed along neither floor — and `Undrawn` through the
/// application's body.
#[test]
fn a_half_ball_turned_back_joined_to_a_coaxial_half_shaft_from_the_other_plane_is_drawn() {
    let case = Case::new(
        Leaf::turned(
            Plane::yz(6.0),
            Axis::second(-2.0).backwards(),
            Section::bands(1.5, &[[5.0, 0.0, 0.0]]).bowed(&[[[0.0, 0.0, 0.0], [4.0, 0.0, 2.5]]]),
            180.0,
        ),
        vec![Step::add(Leaf::turned(
            Plane::xz(-2.0),
            Axis::second(6.0),
            Section::bands(-6.0, &[[2.0, 0.0, 4.0]]),
            180.0,
        ))],
    );
    random_solids::holds_exactly(&case);
    random_solids::holds_through_the_application(&case);
}

/// Seed 5576869948, shrunk: `Listed` on the exact kernel — a lune through
/// both poles, closed along neither floor — and `Undrawn` through the
/// application's body.
#[test]
fn a_half_ball_joined_to_a_wider_coaxial_half_shaft_from_the_other_plane_is_drawn() {
    let case = Case::new(
        Leaf::turned(
            Plane::xy(40.0),
            Axis::first(0.0),
            Section::bands(-20.0, &[[20.0, 0.0, 0.0]])
                .bowed(&[[[0.0, 0.0, 0.0], [-10.0, 0.0, 10.0]]]),
            180.0,
        ),
        vec![Step::add(Leaf::turned(
            Plane::xz(0.0),
            Axis::first(40.0),
            Section::bands(-20.0, &[[10.0, 0.0, 15.0]]),
            180.0,
        ))],
    );
    random_solids::holds_exactly(&case);
    random_solids::holds_through_the_application(&case);
}
