//! Closes #286.
//! - what is drawn in the alert colour is read off the very call the click
//!   commits, run on a copy of the drawing —
//!   `reading_what_a_cut_would_take_leaves_the_drawing_as_it_was`
//! - nothing is drawn where a click would do nothing —
//!   `a_cut_that_cannot_be_made_takes_nothing_and_says_so`
//! - the stretch that goes is the one between the two points the click fell
//!   between, on a trait and on a curve alike —
//!   `the_stretch_a_cut_takes_runs_between_the_two_points_it_falls_between`,
//!   `the_stretch_a_cut_takes_out_of_an_arc_sweeps_the_way_the_arc_does`
//! - the rules and values that go with it are the ones no piece inherits —
//!   `a_rule_the_cut_would_take_with_it_is_named`,
//!   `a_value_the_cut_would_take_with_it_is_named`,
//!   `a_rule_a_piece_inherits_is_not_said_to_be_going`,
//!   `a_value_a_piece_inherits_is_not_said_to_be_going`,
//!   `a_reach_a_piece_of_the_arc_keeps_is_not_said_to_be_going_and_a_sweep_is`,
//!   `a_point_held_on_the_stretch_that_goes_loses_what_held_it`
//!
//! What the drawing is painted with is answered where it is painted, in
//! `crates/app/src/screens/viewport/render/`.

use glam::DVec2;

use super::*;
use crate::arc::ArcId;
use crate::arcing::sweep_of;
use crate::constraints::{Constraint, DimensionTarget, SketchAxis};
use crate::plane::WorkPlane;
use crate::sketch::{PointId, SegmentId, Sketch};

const TOLERANCE: f64 = 1e-9;

fn a_trait_with_two_points_on_it() -> (Sketch, SegmentId, [PointId; 4]) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(DVec2::new(0.0, 1.0));
    let end = sketch.add_point(DVec2::new(10.0, 1.0));
    let segment = sketch.add_segment(start, end);
    let near = sketch.add_point(DVec2::new(3.0, 1.0));
    let far = sketch.add_point(DVec2::new(7.0, 1.0));
    (sketch, segment, [start, near, far, end])
}

#[test]
fn the_stretch_a_cut_takes_runs_between_the_two_points_it_falls_between() {
    let (sketch, segment, [_, near, far, _]) = a_trait_with_two_points_on_it();

    let going = sketch
        .trim_takes(segment, near, far)
        .expect("a cut that can be made");

    let Stretch::Straight { from, to } = going.stretch else {
        panic!(
            "a straight trait loses a straight stretch, got {:?}",
            going.stretch
        );
    };
    assert!(
        from.distance(DVec2::new(3.0, 1.0)) < TOLERANCE
            && to.distance(DVec2::new(7.0, 1.0)) < TOLERANCE,
        "the stretch runs from the near point to the far one, got {from} to {to}",
    );
}

#[test]
fn a_value_the_cut_would_take_with_it_is_named() {
    let (mut sketch, segment, [_, near, far, _]) = a_trait_with_two_points_on_it();
    sketch.set_dimension(DimensionTarget::Length(segment), 10.0, false);

    let going = sketch
        .trim_takes(segment, near, far)
        .expect("a cut that can be made");

    assert_eq!(
        going.values,
        vec![DimensionTarget::Length(segment)],
        "a length measures a trait that is about to be two shorter ones",
    );
}

#[test]
fn a_value_a_piece_inherits_is_not_said_to_be_going() {
    let (mut sketch, segment, [_, near, far, _]) = a_trait_with_two_points_on_it();
    let angle = DimensionTarget::AxisAngle {
        segment,
        axis: SketchAxis::U,
    };
    sketch.set_dimension(angle, 0.0, false);

    let going = sketch
        .trim_takes(segment, near, far)
        .expect("a cut that can be made");

    assert!(
        going.values.is_empty(),
        "both pieces lie the way the trait lay, and each keeps the angle: {:?}",
        going.values,
    );
}

#[test]
fn a_rule_the_cut_would_take_with_it_is_named() {
    let (mut sketch, segment, [_, near, far, _]) = a_trait_with_two_points_on_it();
    let middle = sketch.add_point(DVec2::new(5.0, 1.0));
    let held = Constraint::OnSegment {
        point: middle,
        segment,
    };
    sketch.add_constraint(held);

    let going = sketch
        .trim_takes(segment, near, far)
        .expect("a cut that can be made");

    assert_eq!(
        going.rules,
        vec![held],
        "the point it holds sits in the stretch that goes, and no piece reaches it",
    );
}

#[test]
fn a_rule_a_piece_inherits_is_not_said_to_be_going() {
    let (mut sketch, segment, [_, near, far, _]) = a_trait_with_two_points_on_it();
    let foot = sketch.add_point(DVec2::new(0.0, 5.0));
    let up = sketch.add_segment(sketch.segments()[segment.0].start, foot);
    sketch.add_constraint(Constraint::Perpendicular {
        first: segment,
        second: up,
    });

    let going = sketch
        .trim_takes(segment, near, far)
        .expect("a cut that can be made");

    assert!(
        going.rules.is_empty(),
        "a right angle is about a direction, and both pieces keep it: {:?}",
        going.rules,
    );
}

const REACH: f64 = 10.0;

/// A half turn from due east round to due west, with two points sitting on it
/// a third and two thirds of the way along.
fn a_half_turn() -> (Sketch, ArcId, [PointId; 2]) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let on_the_rim = |sketch: &mut Sketch, degrees: f64| {
        sketch.add_point(DVec2::from_angle(f64::to_radians(degrees)) * REACH)
    };
    let east = on_the_rim(&mut sketch, 0.0);
    let west = on_the_rim(&mut sketch, 180.0);
    let arc = sketch.add_arc(Sketch::ORIGIN, east, west);
    let first = on_the_rim(&mut sketch, 60.0);
    let second = on_the_rim(&mut sketch, 120.0);
    (sketch, arc, [first, second])
}

#[test]
fn the_stretch_a_cut_takes_out_of_an_arc_sweeps_the_way_the_arc_does() {
    let (sketch, arc, [first, second]) = a_half_turn();

    let going = sketch
        .arc_trim_takes(arc, first, second)
        .expect("a cut that can be made");

    let Stretch::Curved(drawn) = going.stretch else {
        panic!("an arc loses a curved stretch, got {:?}", going.stretch);
    };
    let sweep = sweep_of(drawn).to_degrees();
    assert!(
        (sweep - 60.0).abs() < TOLERANCE,
        "the third of the half turn between the two points goes, got {sweep}",
    );
}

#[test]
fn a_cut_that_cannot_be_made_takes_nothing_and_says_so() {
    let (sketch, segment, [start, _, _, end]) = a_trait_with_two_points_on_it();

    assert!(
        sketch.trim_takes(segment, start, start).is_none(),
        "a cut that hands the trait back shows nothing at all",
    );
    assert!(
        sketch.trim_takes(segment, end, end).is_none(),
        "nor does one naming the same end twice",
    );
}

#[test]
fn reading_what_a_cut_would_take_leaves_the_drawing_as_it_was() {
    let (mut sketch, segment, [_, near, far, _]) = a_trait_with_two_points_on_it();
    sketch.set_dimension(DimensionTarget::Length(segment), 10.0, false);
    let before = sketch.clone();

    sketch
        .trim_takes(segment, near, far)
        .expect("a cut that can be made");

    assert!(
        !sketch.is_erased_segment(segment),
        "the drawing in front of the user is not the one the cut was tried on",
    );
    assert_eq!(sketch.segments().len(), before.segments().len());
    assert_eq!(sketch.dimensions().len(), before.dimensions().len());
}

#[test]
fn a_reach_a_piece_of_the_arc_keeps_is_not_said_to_be_going_and_a_sweep_is() {
    let (mut sketch, arc, [first, second]) = a_half_turn();
    sketch.set_dimension(DimensionTarget::ArcRadius(arc), REACH, false);
    sketch.set_dimension(DimensionTarget::ArcSweep(arc), 180.0, false);

    let going = sketch
        .arc_trim_takes(arc, first, second)
        .expect("a cut that can be made");

    assert_eq!(
        going.values,
        vec![DimensionTarget::ArcSweep(arc)],
        "the reach is the same on either piece, the sweep is not",
    );
}

#[test]
fn a_point_held_on_the_stretch_that_goes_loses_what_held_it() {
    let (mut sketch, arc, [first, second]) = a_half_turn();
    let between = sketch.add_point(DVec2::from_angle(f64::to_radians(90.0)) * REACH);
    let held = Constraint::OnArc {
        point: between,
        arc,
    };
    sketch.add_constraint(held);

    let going = sketch
        .arc_trim_takes(arc, first, second)
        .expect("a cut that can be made");

    assert_eq!(
        going.rules,
        vec![held],
        "nothing is left to hold a point that sat on the stretch taken away",
    );
}
