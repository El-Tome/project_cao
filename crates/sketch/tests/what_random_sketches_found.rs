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
//! - and two new ones, each its own issue —
//!   `a_circle_touching_the_one_around_it_at_their_lowest_point_is_a_hole_of_it`,
//!   `an_outline_with_a_corner_on_one_of_its_diagonals_is_tinted_as_it_encloses`
//!
//! Closes #493.
//! - the five cases the campaigns filed under it hold, their `ignore` taken
//!   off — `a_rectangle_on_part_of_another_s_side_is_tinted_as_it_encloses`,
//!   `a_trait_laid_along_a_side_leaves_the_shape_its_area`,
//!   `an_arc_drawn_along_its_circle_leaves_the_disc_its_area`,
//!   `a_trait_drawn_twice_between_two_points_is_tinted_once`,
//!   `a_circle_drawn_twice_on_one_centre_is_one_area`
//! - what it brought out where it meets #494 is written down there, ignored
//!   under #494 — `a_rectangle_drawn_into_another_s_corner_is_tinted_as_it_encloses`,
//!   `a_rectangle_with_no_width_along_another_s_side_leaves_it_its_area`
//!
//! Closes #494.
//! - the six cases filed under it hold, their `ignore` taken off —
//!   `a_pattern_whose_copies_land_on_each_other_tints_each_place_once`,
//!   `a_rectangle_mirrored_across_its_own_side_closes_the_copy`,
//!   `a_rectangle_copied_onto_its_own_top_closes_the_copy`,
//!   `a_second_rectangle_drawn_from_the_first_one_s_corner_keeps_both_areas`,
//!   `a_rectangle_drawn_into_another_s_corner_is_tinted_as_it_encloses`,
//!   `a_rectangle_with_no_width_along_another_s_side_leaves_it_its_area`
//!
//! Closes #503.
//! - the drawing filed under it holds every rule, its `ignore` taken off —
//!   `a_circle_touching_the_one_around_it_at_their_lowest_point_is_a_hole_of_it`
//!
//! Closes #504.
//! - the drawing filed under it is tinted as it encloses, its `ignore` taken
//!   off — `an_outline_with_a_corner_on_one_of_its_diagonals_is_tinted_as_it_encloses`
//! - the side a corner lies on is read exactly — no test: the reading is
//!   private to the crate, and held beside it in regions/side/tests.rs
//!
//! Closes #508.
//! - #503's drawing, which broke the second rule while its depth was wrong,
//!   holds both matter rules now that #524 fixed it —
//!   `a_circle_touching_the_one_around_it_at_their_lowest_point_is_a_hole_of_it`
//! - #540's twin windows, which gave 325 mm² of matter for a measure of 500
//!   until #543 fixed the bridging, hold both rules with their two neighbours
//!   and the owner's notch —
//!   `a_frame_whose_windows_share_a_whole_side_raises_its_matter`,
//!   `a_frame_whose_windows_share_part_of_a_side_raises_its_matter`,
//!   `a_frame_whose_windows_touch_at_one_corner_raises_its_matter`,
//!   `a_rectangle_set_in_a_notch_of_a_square_raises_its_matter`
//!
//! Closes #602.
//! - on a region whose hole touches its outline, the matter covers what the
//!   area measures and every closed place lies in the matter of exactly one
//!   area — `an_ellipse_tangent_to_a_rectangle_s_top_and_bottom_raises_its_matter`,
//!   `a_small_circle_tangent_to_both_long_sides_of_a_rectangle_raises_its_matter`,
//!   and the campaign's third drawing,
//!   `a_circle_tangent_to_both_sides_of_a_rectangle_raises_its_matter`, whose
//!   last flaw — the tangency flicker when turned — #516 closed, its ignore
//!   taken off with it
//! - #529's solid-level behaviour does not regress — no test: it is held by
//!   the solid crate's own suite, which the gate runs
//! - the corridors bridged out to holes still pass — no test: the window-frame
//!   tests of regions and the #540 frames above already hold them
//!
//! Closes #603.
//! - cutting a corner of an open chain with a chamfer never changes what the
//!   areas around it claim to cover: the three drawings the hunt kept hold
//!   every rule —
//!   `a_chamfer_on_an_open_chain_closed_by_a_circle_keeps_the_tint_the_measure`,
//!   `a_chamfer_on_an_open_chain_closed_by_an_arc_keeps_the_tint_the_measure`,
//!   `a_chamfer_on_an_open_chain_against_a_pattern_keeps_the_tint_the_measure`
//! - the chamfer's named tests and what a chamfer draws on closed shapes keep
//!   passing — no test: they are chamfer.rs and corner/tests.rs's own, run in
//!   the same gate
//! - where the fault lay — no test: it was not the chamfer's rewiring but the
//!   touches the cut leaves against the closing curve; #604 closed the arc and
//!   pattern drawings and #606 the circle one, and this file pins the family
//!   so no future sighting is lost

// The drawing, the rules and the shrinking are the campaign's; this file only
// lays drawings and holds them.
#[allow(dead_code, unused_imports)]
mod random_sketches;

use random_sketches::{Axis, Gesture};

#[test]
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
fn a_rectangle_drawn_into_another_s_corner_is_tinted_as_it_encloses() {
    random_sketches::holds(&[
        Gesture::Rectangle {
            corner: [6.0, 2.0],
            opposite: [13.0, 9.0],
            construction: false,
        },
        Gesture::Rectangle {
            corner: [5.0, 5.0],
            opposite: [9.5, 2.5],
            construction: false,
        },
        Gesture::Rectangle {
            corner: [8.5, 5.0],
            opposite: [9.5, 2.5],
            construction: false,
        },
    ]);
}

#[test]
fn a_rectangle_with_no_width_along_another_s_side_leaves_it_its_area() {
    random_sketches::holds(&[
        Gesture::Rectangle {
            corner: [8.0, 10.0],
            opposite: [8.0, 9.0],
            construction: false,
        },
        Gesture::Rectangle {
            corner: [8.0, 8.0],
            opposite: [13.0, 10.0],
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
#[ignore = "#419"]
fn a_circle_tangent_under_a_rectangle_s_top_keeps_its_areas_mirrored() {
    random_sketches::holds(&[
        Gesture::Rectangle {
            corner: [5.0, 5.0],
            opposite: [0.5, 8.5],
            construction: false,
        },
        Gesture::Circle {
            centre: [1.0, 6.0],
            radius: 2.5,
            construction: false,
        },
    ]);
}

#[test]
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

fn a_frame_with_two_windows(windows: [[[f64; 2]; 2]; 2]) -> Vec<Gesture> {
    let mut gestures = vec![Gesture::Rectangle {
        corner: [0.0, 0.0],
        opposite: [30.0, 30.0],
        construction: false,
    }];
    gestures.extend(windows.iter().map(|window| Gesture::Rectangle {
        corner: window[0],
        opposite: window[1],
        construction: false,
    }));
    gestures
}

#[test]
fn a_frame_whose_windows_share_a_whole_side_raises_its_matter() {
    random_sketches::holds(&a_frame_with_two_windows([
        [[5.0, 5.0], [15.0, 25.0]],
        [[15.0, 5.0], [25.0, 25.0]],
    ]));
}

#[test]
fn a_frame_whose_windows_share_part_of_a_side_raises_its_matter() {
    random_sketches::holds(&a_frame_with_two_windows([
        [[5.0, 5.0], [15.0, 25.0]],
        [[15.0, 10.0], [25.0, 20.0]],
    ]));
}

#[test]
fn a_frame_whose_windows_touch_at_one_corner_raises_its_matter() {
    random_sketches::holds(&a_frame_with_two_windows([
        [[5.0, 5.0], [15.0, 15.0]],
        [[15.0, 15.0], [25.0, 25.0]],
    ]));
}

#[test]
fn a_rectangle_set_in_a_notch_of_a_square_raises_its_matter() {
    random_sketches::holds(&[
        Gesture::Rectangle {
            corner: [0.0, 0.0],
            opposite: [40.0, 40.0],
            construction: false,
        },
        Gesture::Rectangle {
            corner: [0.0, 20.0],
            opposite: [15.0, 30.0],
            construction: false,
        },
    ]);
}

#[test]
fn an_outline_with_a_corner_on_one_of_its_diagonals_is_tinted_as_it_encloses() {
    random_sketches::holds(&[Gesture::Chain {
        through: vec![
            [2.0, 5.0],
            [0.5303300858899107, 1.237436867076458],
            [0.0, 1.0],
            [0.8838834764831843, 0.8838834764831843],
            [4.383883476483184, -2.6161165235168156],
        ],
        closed: true,
        construction: false,
    }]);
}

#[test]
fn a_circle_tangent_to_both_sides_of_a_rectangle_raises_its_matter() {
    random_sketches::holds(&[
        Gesture::Rectangle {
            corner: [240.0, 240.0],
            opposite: [330.0, 90.0],
            construction: false,
        },
        Gesture::Circle {
            centre: [285.0, 165.0],
            radius: 45.0,
            construction: false,
        },
    ]);
}

#[test]
fn an_ellipse_tangent_to_a_rectangle_s_top_and_bottom_raises_its_matter() {
    random_sketches::holds(&[
        Gesture::Rectangle {
            corner: [6.0, 0.0],
            opposite: [9.0, -6.0],
            construction: false,
        },
        Gesture::Ellipse {
            centre: [8.0, -3.0],
            reach: [9.0, -3.0],
            across: 3.0,
            construction: false,
        },
    ]);
}

#[test]
fn a_small_circle_tangent_to_both_long_sides_of_a_rectangle_raises_its_matter() {
    random_sketches::holds(&[
        Gesture::Rectangle {
            corner: [6.0, 2.0],
            opposite: [8.0, 1.0],
            construction: false,
        },
        Gesture::Circle {
            centre: [7.0, 1.5],
            radius: 0.5,
            construction: false,
        },
    ]);
}

#[test]
fn a_chamfer_on_an_open_chain_closed_by_a_circle_keeps_the_tint_the_measure() {
    random_sketches::holds(&[
        Gesture::Circle {
            centre: [2.0, 5.0],
            radius: 4.0,
            construction: false,
        },
        Gesture::Chain {
            through: vec![
                [2.0, 5.0],
                [8.0, 0.0],
                [6.0, 5.0],
                [6.0, 2.0],
                [2.0000205, 5.0],
            ],
            closed: false,
            construction: false,
        },
        Gesture::Chamfer {
            at: [6.0, 2.0],
            length: 1.0,
        },
    ]);
}

#[test]
fn a_chamfer_on_an_open_chain_closed_by_an_arc_keeps_the_tint_the_measure() {
    random_sketches::holds(&[
        Gesture::Chain {
            through: vec![[10.0, 3.0], [4.0, 1.0], [6.0, 10.0]],
            closed: false,
            construction: false,
        },
        Gesture::Chamfer {
            at: [4.0, 1.0],
            length: 1.0,
        },
        Gesture::Arc {
            centre: [7.0, 3.5],
            start: [4.0, 3.5],
            degrees: 180.0,
            construction: false,
        },
    ]);
}

#[test]
fn a_chamfer_on_an_open_chain_against_a_pattern_keeps_the_tint_the_measure() {
    random_sketches::holds(&[
        Gesture::Chain {
            through: vec![[9.0, 2.0], [2.0, 3.0], [2.0, 1.0]],
            closed: false,
            construction: false,
        },
        Gesture::Chamfer {
            at: [2.0, 3.0],
            length: 1.0,
        },
        Gesture::Rectangle {
            corner: [1.0, 2.4795],
            opposite: [4.5, 3.5],
            construction: false,
        },
        Gesture::Circle {
            centre: [1.0, 1.0],
            radius: 5.0,
            construction: false,
        },
        Gesture::PatternAround {
            of: vec![[-0.4776010333066978, 5.77668244562803]],
            centre: [4.5, 2.4795],
            degrees: 90.0,
            count: 4,
        },
    ]);
}
