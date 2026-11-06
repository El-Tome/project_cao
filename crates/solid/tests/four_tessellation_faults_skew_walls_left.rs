//! Four faults of the triangles the exact kernel draws, each local to the
//! tessellation, that the walls and turned draws of #577 left (#707's
//! triage): a cylinder face whose hole is laid a turn off, a flip leaving a
//! round face, two meet samples out of order beside a vertex, and a conic
//! sample withheld on another wall.
//!
//! Part of #707.
//! - 86751972230147, `Closed`: a cylinder face wider than a turn, not
//!   winding, its hole laid in the near turn rather than the far one —
//!   `a_hole_in_the_far_turn_of_a_face_wider_than_a_turn_is_laid_where_the_face_holds_it`
//!   in `tessellation/outline/tests.rs`, and the seed's shrunk case,
//!   `seed_86751972230147_a_post_bored_square_then_by_its_radius_at_45_degrees_keeps_every_rule`
//!   in `random_walls_at_any_angle.rs`, un-ignored
//! - 86751972229989, `Volume`: a diagonal flipped on a round face spans two
//!   grid steps, its chord sagging past the tolerance — the seed's own
//!   one-step case,
//!   `seed_86751972229989_two_posts_of_one_radius_crossing_at_45_degrees_keep_every_rule`
//!   in `random_walls_at_any_angle.rs`, un-ignored
//! - 122431270514586, `Closed`: two meet samples a hair from an end vertex,
//!   out of order —
//!   `a_pin_widening_by_a_cone_astride_a_disc_s_wall_is_drawn_closed`, and
//!   `seed_122431270514586_a_slanted_case_is_drawn_closed` in
//!   `random_turned_solids.rs`, un-ignored
//! - 28743870519278, `Closed`: a conic sample lying on another wall withheld
//!   as crowded — `two_turned_cuts_and_a_leaning_slab_are_drawn_closed`, and
//!   `seed_28743870519278_a_tilted_case_is_drawn_closed` in
//!   `random_turned_solids.rs`, un-ignored

#[allow(dead_code, unused_imports)]
mod random_solids;

use random_solids::{Axis, Case, Leaf, Outline, Plane, Section, Step};

/// A disc of radius 6.5 joined by a pin along its axis, astride its wall:
/// a cylinder of radius 1.5 widening by a cone to 2 (seed 122431270514586,
/// shrunk). Two samples of a curve land a hair from its end vertex, out of
/// order.
#[test]
fn a_pin_widening_by_a_cone_astride_a_disc_s_wall_is_drawn_closed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xz(0.0), Outline::circle([8.0, 0.0], 6.5), -7.0),
        vec![Step::add(Leaf::turned(
            Plane::xy(-2.0),
            Axis::second(14.5).backwards(),
            Section::bands(-9.0, &[[6.5, 0.0, 1.5], [0.5, 0.0, 1.5]])
                .sloping_to(&[[0.0, 1.5], [0.0, 2.0]]),
            360.0,
        ))],
    ));
}

/// A turned solid of three bands cut by another turned about a parallel
/// axis, then by a leaning slab (seed 28743870519278, its first and third
/// cuts dropped): a sample of a conic lying on another wall is withheld,
/// and the chord passes a corner on the wrong side.
#[test]
fn two_turned_cuts_and_a_leaning_slab_are_drawn_closed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(210.0),
            Axis::first(0.0),
            Section::bands(
                30.0,
                &[
                    [90.0, 90.0, 195.0],
                    [30.0, 90.0, 270.0],
                    [15.0, 90.0, 270.0],
                ],
            )
            .sloping_to(&[[90.0, 195.0], [90.0, 270.0], [105.0, 270.0]]),
            360.0,
        ),
        vec![
            Step::cut(Leaf::turned(
                Plane::xy(0.0),
                Axis::first(90.0),
                Section::bands(
                    -60.0,
                    &[
                        [120.0, 60.0, 225.0],
                        [15.0, 150.0, 225.0],
                        [30.0, 150.0, 270.0],
                    ],
                )
                .sloping_to(&[[210.0, 225.0], [150.0, 270.0], [150.0, 225.0]]),
                360.0,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted(
                    [59.99999292893219, 7.0710678118654756e-6, 210.0],
                    [90.0, 45.0, 0.0],
                ),
                Outline::rectangle([-240.0, -285.0], [285.0, 285.0]),
                -240.0,
            )),
        ],
    ));
}
