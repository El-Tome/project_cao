//! What the first campaigns over random drawings found, shrunk and named, each
//! waiting on the issue that describes it.
//!
//! A drawing here fails today, which is why it is ignored: the gate cannot
//! hold a test that is red on purpose. It stays in the repository so that the
//! drawing is never lost, and `--ignored` runs them all. The commit that fixes
//! one takes its `ignore` off, and from then on the gate holds it.
//!
//! Closes #500.
//! - what the first campaigns find is written down, the case added to an issue
//!   already open — `a_rectangle_on_part_of_another_s_side_is_tinted_as_it_encloses`,
//!   `a_trait_laid_along_a_side_leaves_the_shape_its_area`,
//!   `an_arc_drawn_along_its_circle_leaves_the_disc_its_area`,
//!   `a_trait_drawn_twice_between_two_points_is_tinted_once`,
//!   `a_circle_drawn_twice_on_one_centre_is_one_area`,
//!   `a_pattern_whose_copies_land_on_each_other_tints_each_place_once`,
//!   `a_rectangle_mirrored_across_its_own_side_closes_the_copy`,
//!   `a_rectangle_copied_onto_its_own_top_closes_the_copy`,
//!   `a_second_rectangle_drawn_from_the_first_one_s_corner_keeps_both_areas`,
//!   `an_ellipse_inside_a_rectangle_keeps_its_areas_when_turned`,
//!   `half_an_ellipse_under_a_rectangle_s_top_keeps_its_areas_when_turned`,
//!   `an_ellipse_and_its_mirror_image_crossing_lie_one_area_apart`,
//!   `two_circles_touching_off_the_lattice_keep_both_their_areas`,
//!   `a_circle_touching_two_sides_of_a_rectangle_keeps_its_areas_when_turned`,
//!   `a_circle_tangent_inside_another_off_the_lattice_keeps_its_areas_in_either_order`,
//!   `an_area_pinched_where_two_circles_touch_is_tinted_on_its_own_side_of_the_pinch`
//! - and a new one is its own issue —
//!   `a_circle_touching_the_one_around_it_at_their_lowest_point_is_a_hole_of_it`

// The drawing, the rules and the shrinking are the campaign's; this file only
// lays drawings and holds them.
#[allow(dead_code, unused_imports)]
mod random_sketches;

use random_sketches::{Axis, Gesture};

#[test]
#[ignore = "#493"]
fn a_rectangle_on_part_of_another_s_side_is_tinted_as_it_encloses() {
    random_sketches::holds(&[
        Gesture::Rectangle {
            corner: [7.0, 7.0],
            opposite: [4.0, 3.0],
            construction: false,
        },
        Gesture::Rectangle {
            corner: [7.0, 1.0],
            opposite: [11.0, 6.0],
            construction: false,
        },
    ]);
}

#[test]
#[ignore = "#493"]
fn a_trait_laid_along_a_side_leaves_the_shape_its_area() {
    random_sketches::holds(&[
        Gesture::Rectangle {
            corner: [10.0, 10.0],
            opposite: [9.0, 7.0],
            construction: false,
        },
        Gesture::Chain {
            through: vec![[9.0, 10.0], [14.0, 10.0]],
            closed: false,
            construction: false,
        },
    ]);
}

#[test]
#[ignore = "#493"]
fn an_arc_drawn_along_its_circle_leaves_the_disc_its_area() {
    random_sketches::holds(&[
        Gesture::Circle {
            centre: [6.0, 1.0],
            radius: 0.5,
            construction: false,
        },
        Gesture::Arc {
            centre: [6.0, 1.0],
            start: [5.5, 1.0],
            degrees: 135.0,
            construction: false,
        },
    ]);
}

#[test]
#[ignore = "#493"]
fn a_trait_drawn_twice_between_two_points_is_tinted_once() {
    random_sketches::holds(&[
        Gesture::Chain {
            through: vec![[0.0, 0.0], [0.0, -4.0], [-3.0, -4.0], [1.0, 1.0]],
            closed: false,
            construction: false,
        },
        Gesture::Chain {
            through: vec![[1.0, 1.0], [0.0, -4.0], [0.0, 0.0]],
            closed: false,
            construction: false,
        },
    ]);
}

#[test]
#[ignore = "#493"]
fn a_circle_drawn_twice_on_one_centre_is_one_area() {
    random_sketches::holds(&[
        Gesture::Circle {
            centre: [8.0, 4.0],
            radius: 2.0,
            construction: false,
        },
        Gesture::Circle {
            centre: [8.0, 4.0],
            radius: 2.0,
            construction: false,
        },
    ]);
}

#[test]
#[ignore = "#494"]
fn a_pattern_whose_copies_land_on_each_other_tints_each_place_once() {
    random_sketches::holds(&[
        Gesture::Rectangle {
            corner: [1.0, 1.0],
            opposite: [3.0, 2.0],
            construction: false,
        },
        Gesture::Point { at: [0.0, 0.0] },
        Gesture::PatternAround {
            of: vec![[2.0, 1.0], [3.0, 1.5], [2.0, 2.0], [1.0, 1.5]],
            centre: [0.0, 0.0],
            degrees: 180.0,
            count: 4,
        },
    ]);
}

#[test]
#[ignore = "#494"]
fn a_rectangle_mirrored_across_its_own_side_closes_the_copy() {
    random_sketches::holds(&[
        Gesture::Rectangle {
            corner: [20.0, 20.0],
            opposite: [27.5, 2.5],
            construction: false,
        },
        Gesture::Mirror {
            of: vec![[24.0, 2.5], [27.5, 11.5], [24.0, 20.0]],
            axis: Axis::Trait([20.0, 11.5]),
        },
    ]);
}

#[test]
#[ignore = "#494"]
fn a_rectangle_copied_onto_its_own_top_closes_the_copy() {
    random_sketches::holds(&[
        Gesture::Rectangle {
            corner: [20.0, 5.0],
            opposite: [12.5, 15.0],
            construction: false,
        },
        Gesture::PatternAlong {
            of: vec![[20.0, 10.0], [16.5, 15.0], [12.5, 10.0]],
            axis: Axis::V,
            along: (10.0, 2),
            across: (25.0, 1),
        },
    ]);
}

#[test]
#[ignore = "#494"]
fn a_second_rectangle_drawn_from_the_first_one_s_corner_keeps_both_areas() {
    random_sketches::holds(&[
        Gesture::Rectangle {
            corner: [7.0, 1.0],
            opposite: [5.5, -3.5],
            construction: false,
        },
        Gesture::Rectangle {
            corner: [7.0, 1.0],
            opposite: [7.5, -3.5],
            construction: false,
        },
    ]);
}

#[test]
#[ignore = "#419"]
fn an_ellipse_inside_a_rectangle_keeps_its_areas_when_turned() {
    random_sketches::holds(&[
        Gesture::Rectangle {
            corner: [2.0, 7.0],
            opposite: [7.0, 11.0],
            construction: false,
        },
        Gesture::Ellipse {
            centre: [4.25, 8.75],
            reach: [6.5, 8.75],
            across: 1.75,
            construction: false,
        },
    ]);
}

#[test]
#[ignore = "#419"]
fn half_an_ellipse_under_a_rectangle_s_top_keeps_its_areas_when_turned() {
    random_sketches::holds(&[
        Gesture::Rectangle {
            corner: [4.0, 0.0],
            opposite: [2.5, 2.0],
            construction: false,
        },
        Gesture::HalfEllipse {
            from: [2.5, 2.0],
            to: [4.0, 2.0],
            rise: -1.0,
            construction: false,
        },
    ]);
}

#[test]
#[ignore = "#419"]
fn an_ellipse_and_its_mirror_image_crossing_lie_one_area_apart() {
    random_sketches::holds(&[
        Gesture::Ellipse {
            centre: [2.5, 0.5],
            reach: [4.974873734152916, 2.974873734152916],
            across: 1.5,
            construction: false,
        },
        Gesture::Mirror {
            of: vec![[1.1212254968042847, -2.125655316528075]],
            axis: Axis::V,
        },
    ]);
}

#[test]
#[ignore = "#419"]
fn two_circles_touching_off_the_lattice_keep_both_their_areas() {
    random_sketches::holds(&[
        Gesture::Circle {
            centre: [35.0, 20.0],
            radius: 22.498995,
            construction: false,
        },
        Gesture::Circle {
            centre: [54.44472584031496, 39.44472584031496],
            radius: 5.0,
            construction: false,
        },
    ]);
}

#[test]
#[ignore = "#419"]
fn a_circle_touching_two_sides_of_a_rectangle_keeps_its_areas_when_turned() {
    random_sketches::holds(&[
        Gesture::Rectangle {
            corner: [120.0, 285.0],
            opposite: [195.0, 225.0],
            construction: false,
        },
        Gesture::Circle {
            centre: [158.0, 255.0],
            radius: 30.0,
            construction: false,
        },
    ]);
}

#[test]
#[ignore = "#419"]
fn a_circle_tangent_inside_another_off_the_lattice_keeps_its_areas_in_either_order() {
    random_sketches::holds(&[
        Gesture::Circle {
            centre: [11.414213562373096, f64::from_bits(0x3ff6a09e667f3bcc)],
            radius: 3.0,
            construction: false,
        },
        Gesture::Circle {
            centre: [10.0, 0.0],
            radius: 1.0,
            construction: false,
        },
    ]);
}

#[test]
#[ignore = "#419"]
fn an_area_pinched_where_two_circles_touch_is_tinted_on_its_own_side_of_the_pinch() {
    random_sketches::holds(&[
        Gesture::Rectangle {
            corner: [40.0, 45.0],
            opposite: [68.0, 15.0],
            construction: false,
        },
        Gesture::Circle {
            centre: [30.0, 30.0],
            radius: 25.0,
            construction: false,
        },
        Gesture::Circle {
            centre: [25.0, 30.0],
            radius: 20.0,
            construction: false,
        },
    ]);
}

#[test]
#[ignore = "#503"]
fn a_circle_touching_the_one_around_it_at_their_lowest_point_is_a_hole_of_it() {
    random_sketches::holds(&[
        Gesture::Circle {
            centre: [5.0, 7.0],
            radius: 5.0,
            construction: false,
        },
        Gesture::Circle {
            centre: [5.0, 3.0],
            radius: 1.0,
            construction: false,
        },
    ]);
}
