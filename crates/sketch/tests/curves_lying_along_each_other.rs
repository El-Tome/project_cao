//! Drawings where two curves lie along each other, held to every rule an area
//! keeps — laid as drawn, laid again in another order, moved and turned.
//!
//! Two curves lying along each other between the same two vertices are one
//! border (#493). These are the drawings that rule has to hold for whatever
//! the arithmetic does to them: turned by an odd angle, a stretch that lay
//! exactly along another one lies along it only to the last digit or two.

// The drawing and the rules are the campaign's; this file only lays drawings
// and holds them.
#[allow(dead_code, unused_imports)]
mod random_sketches;

use std::f64::consts::SQRT_2;

use random_sketches::Gesture;

fn rectangle(corner: [f64; 2], opposite: [f64; 2]) -> Gesture {
    Gesture::Rectangle {
        corner,
        opposite,
        construction: false,
    }
}

fn chain(through: &[[f64; 2]]) -> Gesture {
    Gesture::Chain {
        through: through.to_vec(),
        closed: false,
        construction: false,
    }
}

/// The arc grazes the rectangle's top where a trait lies along it. Whether a
/// graze is seen is a matter of rounding, and turned it is seen on one of the
/// two and not on the other: the two then end on different vertices there,
/// and lie along each other as two borders.
#[test]
fn an_arc_grazing_a_stretch_two_traits_lie_along_cuts_both_alike() {
    random_sketches::holds(&[
        rectangle([0.0, -3.0], [6.0, 0.0]),
        chain(&[[1.0, 0.0], [5.0, 0.0]]),
        Gesture::Arc {
            centre: [4.5, 1.0],
            start: [3.5, 1.0],
            degrees: 180.0,
            construction: false,
        },
    ]);
}

#[test]
fn a_trait_grazing_two_arcs_of_one_circle_where_they_overlap_cuts_both_alike() {
    random_sketches::holds(&[
        Gesture::Arc {
            centre: [0.0, 0.0],
            start: [3.0, 0.0],
            degrees: 120.0,
            construction: false,
        },
        Gesture::Arc {
            centre: [0.0, 0.0],
            start: [1.4999999999999996, 2.598076211353316],
            degrees: 120.0,
            construction: false,
        },
        chain(&[[-3.0, 0.0], [3.0, 0.0]]),
        chain(&[[-1.5, 3.0], [0.25, 3.0]]),
    ]);
}

fn closed(through: &[[f64; 2]]) -> Gesture {
    Gesture::Chain {
        through: through.to_vec(),
        closed: true,
        construction: false,
    }
}

fn circle(centre: [f64; 2], radius: f64) -> Gesture {
    Gesture::Circle {
        centre,
        radius,
        construction: false,
    }
}

fn arc(centre: [f64; 2], start: [f64; 2], degrees: f64) -> Gesture {
    Gesture::Arc {
        centre,
        start,
        degrees,
        construction: false,
    }
}

fn ellipse(centre: [f64; 2], reach: [f64; 2], across: f64) -> Gesture {
    Gesture::Ellipse {
        centre,
        reach,
        across,
        construction: false,
    }
}

#[test]
fn two_rectangles_whose_sides_lie_along_each_other_over_part_of_their_length_keep_every_rule() {
    random_sketches::holds(&[
        rectangle([0.0, 0.0], [1.0, 2.0]),
        rectangle([1.0, 1.0], [2.0, 3.0]),
    ]);
}

#[test]
fn a_short_side_lying_wholly_along_a_longer_one_keeps_every_rule() {
    random_sketches::holds(&[
        rectangle([0.0, 0.0], [1.0, 3.0]),
        rectangle([1.0, 1.0], [2.0, 2.0]),
    ]);
}

#[test]
fn a_rectangle_overhanging_another_s_top_keeps_every_rule() {
    random_sketches::holds(&[
        rectangle([0.0, 0.0], [2.0, 2.0]),
        rectangle([1.0, 2.0], [3.0, 3.0]),
    ]);
}

#[test]
fn two_rectangles_overlapping_on_one_baseline_keep_every_rule() {
    random_sketches::holds(&[
        rectangle([0.0, 0.0], [2.0, 1.0]),
        rectangle([1.0, 0.0], [3.0, 2.0]),
    ]);
}

#[test]
fn a_side_drawn_once_twice_and_three_times_keeps_every_rule() {
    random_sketches::holds(&[
        closed(&[[0.0, 0.0], [2.0, 0.0], [2.0, 2.0], [0.0, 2.0]]),
        chain(&[[2.0, 0.0], [2.0, 2.0]]),
        chain(&[[2.0, 2.0], [2.0, 0.0]]),
    ]);
}

#[test]
fn a_trait_laid_along_part_of_a_side_keeps_every_rule() {
    random_sketches::holds(&[
        rectangle([0.0, 0.0], [2.0, 2.0]),
        chain(&[[2.0, 1.0], [2.0, 3.0]]),
    ]);
}

#[test]
fn traits_overlapping_one_another_along_a_side_with_no_end_in_common_keep_every_rule() {
    random_sketches::holds(&[
        rectangle([10.0, -2.0], [16.0, 0.0]),
        chain(&[[11.0, 0.0], [13.0, 0.0]]),
        chain(&[[12.0, 0.0], [17.0, 0.0]]),
        chain(&[[9.0, 0.0], [12.5, 0.0]]),
    ]);
}

#[test]
fn a_trait_standing_on_a_stretch_two_traits_lie_along_keeps_every_rule() {
    random_sketches::holds(&[
        rectangle([0.0, 0.0], [4.0, 2.0]),
        chain(&[[1.0, 0.0], [3.0, 0.0]]),
        chain(&[[2.0, 0.0], [2.0, -2.0], [5.0, -2.0], [3.5, 0.0]]),
    ]);
}

#[test]
fn a_triangle_whose_tip_touches_a_side_keeps_every_rule() {
    random_sketches::holds(&[
        rectangle([0.0, 0.0], [4.0, 2.0]),
        closed(&[[2.0, 2.0], [3.0, 4.0], [1.0, 4.0]]),
    ]);
}

#[test]
fn an_arc_drawn_along_part_of_its_circle_keeps_every_rule() {
    random_sketches::holds(&[circle([0.0, 0.0], 2.0), arc([0.0, 0.0], [2.0, 0.0], 90.0)]);
}

#[test]
fn two_arcs_of_one_circle_overlapping_past_its_start_keep_every_rule() {
    random_sketches::holds(&[
        arc([0.0, 0.0], [0.0, -2.0], 135.0),
        arc([0.0, 0.0], [SQRT_2, -SQRT_2], 225.0),
        chain(&[[-2.0, 0.0], [2.0, 0.0]]),
    ]);
}

#[test]
fn an_arc_along_its_circle_and_a_trait_across_both_keep_every_rule() {
    random_sketches::holds(&[
        circle([0.0, 0.0], 2.0),
        arc([0.0, 0.0], [2.0, 0.0], 90.0),
        chain(&[[-3.0, 1.0], [3.0, 1.0]]),
    ]);
}

#[test]
fn a_circle_drawn_twice_and_a_trait_across_it_keep_every_rule() {
    random_sketches::holds(&[
        circle([0.0, 0.0], 2.0),
        circle([0.0, 0.0], 2.0),
        chain(&[[-3.0, 0.5], [3.0, 0.5]]),
        circle([0.0, 0.0], 1.0),
    ]);
}

#[test]
fn an_ellipse_drawn_twice_and_a_trait_across_it_keep_every_rule() {
    random_sketches::holds(&[
        ellipse([0.0, 0.0], [3.0, 0.0], 1.0),
        ellipse([0.0, 0.0], [3.0, 0.0], 1.0),
        chain(&[[0.5, -2.0], [0.5, 2.0]]),
    ]);
}

#[test]
fn a_round_ellipse_laid_on_a_circle_keeps_every_rule() {
    random_sketches::holds(&[
        circle([4.25, 4.25], 1.25),
        ellipse([4.25, 4.25], [5.5, 4.25], 1.25),
        chain(&[[4.0, 2.0], [4.0, 6.0]]),
    ]);
}

#[test]
fn an_arc_drawn_along_a_round_ellipse_keeps_every_rule() {
    random_sketches::holds(&[
        ellipse([0.0, 0.0], [2.0, 0.0], 2.0),
        arc([0.0, 0.0], [SQRT_2, -SQRT_2], 180.0),
    ]);
}
