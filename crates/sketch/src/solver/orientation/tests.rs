//! What sketch · solver/orientation.rs is held to.
//!
//! Closes #456.
//! - a squared shape held by one fixed point keeps its direction through
//!   several heights retyped —
//!   `a_square_held_by_one_fixed_point_keeps_its_direction_through_several_heights`
//! - the same shape held by nothing, or by the origin, still does —
//!   `a_square_held_by_nothing_or_by_the_origin_keeps_its_direction_too`
//! - a shape pulled by a corner, every size typed, still pivots about the
//!   fixed point holding it, and keeps the new direction afterwards —
//!   `a_square_pulled_by_a_corner_pivots_about_the_fixed_point_and_keeps_the_new_direction`
//! - a shape held by two fixed points still turns with them when one is moved
//!   — `a_square_held_by_two_fixed_points_turns_with_them`
//! - a slanted shape held by one fixed point is not green without an angle
//!   against an axis —
//!   `a_slanted_square_held_by_one_fixed_point_is_not_settled_without_an_angle`
//!
//! Closes #471.
//! - the rectangle, the parallel laid to a free trait: every corner of the
//!   rectangle is still settled —
//!   `a_square_tied_by_a_parallel_to_a_free_trait_is_still_settled`
//! - no gesture but the hand turns the rectangle off the axes —
//!   `a_square_tied_by_a_parallel_to_a_free_trait_keeps_its_way_up_through_gestures`
//! - a leaning shape tied by a parallel to a free trait is still not settled —
//!   `a_leaning_square_tied_by_a_parallel_to_a_free_trait_is_not_settled`

use super::*;

use crate::constraints::{Constraint, DimensionTarget, SketchAxis};
use crate::plane::WorkPlane;
use crate::sketch::{Element, SegmentId};

const SCALE: f64 = 1.0;

/// A tenth of a degree: the drift this is about is read in degrees, and a
/// settle that lands square lands far inside that.
const A_HAIR: f64 = 0.1;

/// A micrometre, for a length read after a settle. The solver stops at a
/// thousandth of a percent of the drawing's own size, which on a shape this
/// big is a good deal more than that.
const A_SETTLED_LENGTH: f64 = 1e-3;

/// Four traits made square, with their width and their height typed. Laid
/// away from the origin, which is where a turn about the origin shows.
fn a_square(sketch: &mut Sketch, corner: DVec2) -> [SegmentId; 4] {
    let a = sketch.add_point(corner);
    let b = sketch.add_point(corner + DVec2::new(50.0, 0.0));
    let c = sketch.add_point(corner + DVec2::new(50.0, 50.0));
    let d = sketch.add_point(corner + DVec2::new(0.0, 50.0));

    let sides = [
        sketch.add_segment(a, b),
        sketch.add_segment(b, c),
        sketch.add_segment(c, d),
        sketch.add_segment(d, a),
    ];
    for pair in 0..4 {
        sketch.add_constraint(Constraint::Perpendicular {
            first: sides[pair],
            second: sides[(pair + 1) % 4],
        });
    }
    sketch.set_dimension(DimensionTarget::Length(sides[0]), 50.0, false);
    sketch.set_dimension(DimensionTarget::Length(sides[3]), 50.0, false);
    sides
}

fn direction(sketch: &Sketch, side: SegmentId) -> f64 {
    let (start, end) = sketch.endpoints(side);
    (end - start).to_angle().to_degrees()
}

fn retype_the_height(sketch: &mut Sketch, side: SegmentId) {
    for height in [80.0, 30.0, 120.0] {
        sketch.set_dimension(DimensionTarget::Length(side), height, false);
        sketch.resolve_keeping_places(SCALE);
    }
}

#[test]
fn a_square_held_by_one_fixed_point_keeps_its_direction_through_several_heights() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let sides = a_square(&mut sketch, DVec2::new(100.0, 100.0));
    let corner = sketch.segments()[sides[0].0].start;
    sketch.add_constraint(Constraint::Fixed {
        element: Element::Point(corner),
    });
    sketch.solve(SCALE);

    retype_the_height(&mut sketch, sides[3]);

    assert!(
        direction(&sketch, sides[0]).abs() < A_HAIR,
        "the bottom side was left at {}°",
        direction(&sketch, sides[0]),
    );
}

#[test]
fn a_square_held_by_nothing_or_by_the_origin_keeps_its_direction_too() {
    let mut adrift = Sketch::new(WorkPlane::XY);
    let sides = a_square(&mut adrift, DVec2::new(100.0, 100.0));
    adrift.solve(SCALE);
    retype_the_height(&mut adrift, sides[3]);

    assert!(
        direction(&adrift, sides[0]).abs() < A_HAIR,
        "held by nothing, the bottom side was left at {}°",
        direction(&adrift, sides[0]),
    );

    let mut on_the_origin = Sketch::new(WorkPlane::XY);
    let sides = a_square(&mut on_the_origin, DVec2::ZERO);
    let corner = on_the_origin.segments()[sides[0].0].start;
    on_the_origin.merge_points(Sketch::ORIGIN, corner);
    assert!(
        on_the_origin.is_origin(on_the_origin.segments()[sides[0].0].start),
        "the square was to hang off the origin",
    );
    on_the_origin.solve(SCALE);
    retype_the_height(&mut on_the_origin, sides[3]);

    assert!(
        direction(&on_the_origin, sides[0]).abs() < A_HAIR,
        "held by the origin, the bottom side was left at {}°",
        direction(&on_the_origin, sides[0]),
    );
}

#[test]
fn a_square_pulled_by_a_corner_pivots_about_the_fixed_point_and_keeps_the_new_direction() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let sides = a_square(&mut sketch, DVec2::new(100.0, 100.0));
    let held = sketch.segments()[sides[0].0].start;
    let far = sketch.segments()[sides[1].0].end;
    sketch.add_constraint(Constraint::Fixed {
        element: Element::Point(held),
    });
    sketch.solve(SCALE);

    let corner = sketch.point(held);
    let reach = (sketch.point(far) - corner).length();
    let asked = corner + DVec2::from_angle(60_f64.to_radians()) * reach;
    sketch.settle_around(far, asked, SCALE);

    let turned = direction(&sketch, sides[0]);
    assert!(
        (turned - 15.0).abs() < A_HAIR,
        "the diagonal was drawn to 60°, so the bottom side lies at 15°, not {turned}°",
    );
    assert!(
        (sketch.point(held) - corner).length() < 1e-6,
        "it pivoted about the corner holding it, which stayed at {corner:?}",
    );
    assert!(
        (sketch.segment_length(sides[0]) - 50.0).abs() < A_SETTLED_LENGTH,
        "the width typed was kept: {}",
        sketch.segment_length(sides[0]),
    );

    retype_the_height(&mut sketch, sides[3]);

    assert!(
        (direction(&sketch, sides[0]) - turned).abs() < A_HAIR,
        "the way round it was turned to is the one kept, not {}°",
        direction(&sketch, sides[0]),
    );
}

#[test]
fn a_square_held_by_two_fixed_points_turns_with_them() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let sides = a_square(&mut sketch, DVec2::new(100.0, 100.0));
    let first = sketch.segments()[sides[0].0].start;
    let second = sketch.segments()[sides[0].0].end;
    for corner in [first, second] {
        sketch.add_constraint(Constraint::Fixed {
            element: Element::Point(corner),
        });
    }
    sketch.solve(SCALE);

    let along = sketch.point(second) - sketch.point(first);
    let moved = sketch.point(first) + DVec2::from_angle(30_f64.to_radians()) * along.length();
    sketch.move_point(second, moved);
    sketch.resolve_keeping_places(SCALE);

    let turned = direction(&sketch, sides[0]);
    assert!(
        (turned - 30.0).abs() < A_HAIR,
        "the side both corners hold was taken to 30°, and the drawing went with it, not to {turned}°",
    );
    assert!(
        (sketch.point(second) - moved).length() < 1e-6,
        "the corner moved stayed where it was put",
    );
    assert!(
        (direction(&sketch, sides[3]) - (turned - 90.0)).abs() < A_HAIR,
        "and the shape is still square: the side running back down lies at {}°",
        direction(&sketch, sides[3]),
    );
}

#[test]
fn a_slanted_square_held_by_one_fixed_point_is_not_settled_without_an_angle() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let sides = a_square(&mut sketch, DVec2::new(100.0, 100.0));
    let corner = sketch.segments()[sides[0].0].start;
    sketch.add_constraint(Constraint::Fixed {
        element: Element::Point(corner),
    });
    sketch.solve(SCALE);

    let leaning = sketch.segments()[sides[0].0].end;
    let about = sketch.point(corner);
    let arm = (sketch.point(leaning) - about).length();
    sketch.settle_around(
        leaning,
        about + DVec2::from_angle(20_f64.to_radians()) * arm,
        SCALE,
    );

    let lies_at = direction(&sketch, sides[0]);
    assert!(
        (lies_at - 20.0).abs() < A_HAIR,
        "the square was to come out leaning at 20°, and it lies at {lies_at}°",
    );
    assert!(
        !sketch.is_fully_constrained(SCALE),
        "a shape leaning at no stated angle is not finished, whatever holds it still",
    );

    let adrift = sketch.freedom(SCALE).degrees_of_freedom;
    sketch.set_dimension(
        DimensionTarget::AxisAngle {
            segment: sides[0],
            axis: SketchAxis::U,
        },
        20.0,
        false,
    );
    sketch.resolve_keeping_places(SCALE);

    assert_eq!(
        sketch.freedom(SCALE).degrees_of_freedom,
        adrift - 1,
        "the angle is what the drawing was missing: holding the way round while it settles never said it",
    );
}

#[test]
fn two_squares_drawn_apart_each_keep_their_own_way_round() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let nailed = a_square(&mut sketch, DVec2::new(100.0, 100.0));
    let adrift = a_square(&mut sketch, DVec2::new(-200.0, 40.0));
    let corner = sketch.segments()[nailed[0].0].start;
    sketch.add_constraint(Constraint::Fixed {
        element: Element::Point(corner),
    });
    sketch.solve(SCALE);

    retype_the_height(&mut sketch, nailed[3]);
    retype_the_height(&mut sketch, adrift[3]);

    assert!(
        direction(&sketch, nailed[0]).abs() < A_HAIR,
        "the nailed square was left at {}°",
        direction(&sketch, nailed[0]),
    );
    assert!(
        direction(&sketch, adrift[0]).abs() < A_HAIR,
        "the one beside it, holding nothing, was left at {}°",
        direction(&sketch, adrift[0]),
    );
}

#[test]
fn an_angle_typed_turns_a_square_held_by_one_fixed_point() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let sides = a_square(&mut sketch, DVec2::new(100.0, 100.0));
    let corner = sketch.segments()[sides[0].0].start;
    sketch.add_constraint(Constraint::Fixed {
        element: Element::Point(corner),
    });
    sketch.solve(SCALE);
    let stays = sketch.point(corner);

    sketch.set_dimension(
        DimensionTarget::AxisAngle {
            segment: sides[0],
            axis: SketchAxis::U,
        },
        30.0,
        false,
    );
    sketch.resolve_keeping_places(SCALE);

    assert!(
        (direction(&sketch, sides[0]) - 30.0).abs() < A_HAIR,
        "the angle typed was to turn it to 30°, and it lies at {}°",
        direction(&sketch, sides[0]),
    );
    assert!(
        (sketch.point(corner) - stays).length() < 1e-6,
        "it turned about the corner holding it",
    );
    assert!(
        (sketch.segment_length(sides[0]) - 50.0).abs() < A_SETTLED_LENGTH,
        "and no size nobody typed changed: {}",
        sketch.segment_length(sides[0]),
    );
}

#[test]
fn a_rule_laid_turns_a_leaning_square_held_by_one_fixed_point() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let sides = a_square(&mut sketch, DVec2::new(100.0, 100.0));
    let corner = sketch.segments()[sides[0].0].start;
    sketch.add_constraint(Constraint::Fixed {
        element: Element::Point(corner),
    });
    sketch.solve(SCALE);

    let leaning = sketch.segments()[sides[0].0].end;
    let about = sketch.point(corner);
    let arm = (sketch.point(leaning) - about).length();
    sketch.settle_around(
        leaning,
        about + DVec2::from_angle(25_f64.to_radians()) * arm,
        SCALE,
    );
    assert!(
        (direction(&sketch, sides[0]) - 25.0).abs() < A_HAIR,
        "it was to be leaning at 25° before the rule",
    );

    sketch.add_constraint(Constraint::AxisParallel {
        segment: sides[0],
        axis: SketchAxis::U,
    });
    sketch.resolve_keeping_places(SCALE);

    assert!(
        direction(&sketch, sides[0]).abs() < A_HAIR,
        "the rule laid was to bring it back square, and it lies at {}°",
        direction(&sketch, sides[0]),
    );
}

/// A square 50 wide from the origin, three right angles and two sizes
/// typed, and a free trait above it with its length typed, laid parallel to
/// the square's top.
fn a_square_and_a_parallel_trait() -> (Sketch, [SegmentId; 4], SegmentId) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corners = [
        Sketch::ORIGIN,
        sketch.add_point(DVec2::new(50.0, 0.0)),
        sketch.add_point(DVec2::new(50.0, 50.0)),
        sketch.add_point(DVec2::new(0.0, 50.0)),
    ];
    let sides: [SegmentId; 4] =
        std::array::from_fn(|side| sketch.add_segment(corners[side], corners[(side + 1) % 4]));
    for pair in 0..3 {
        sketch.add_constraint(Constraint::Perpendicular {
            first: sides[pair],
            second: sides[pair + 1],
        });
    }
    sketch.set_dimension(DimensionTarget::Length(sides[0]), 50.0, false);
    sketch.set_dimension(DimensionTarget::Length(sides[3]), 50.0, false);
    let [start, end] =
        [DVec2::new(10.0, 90.0), DVec2::new(90.0, 110.0)].map(|at| sketch.add_point(at));
    let free = sketch.add_segment(start, end);
    sketch.set_dimension(DimensionTarget::Length(free), 80.0, false);
    sketch.solve(SCALE);
    sketch.lay_rule(
        Constraint::Parallel {
            first: free,
            second: sides[2],
        },
        SCALE,
    );
    (sketch, sides, free)
}

fn corners_of(sketch: &Sketch, sides: [SegmentId; 4]) -> Vec<PointId> {
    sides
        .iter()
        .map(|side| sketch.segments()[side.0].start)
        .collect()
}

#[test]
fn a_square_tied_by_a_parallel_to_a_free_trait_is_still_settled() {
    let (sketch, sides, _) = a_square_and_a_parallel_trait();

    let settled = sketch.settled_points(SCALE);
    for corner in corners_of(&sketch, sides) {
        assert!(settled[corner.0], "corner {corner:?} is not settled");
    }
}

#[test]
fn a_square_tied_by_a_parallel_to_a_free_trait_keeps_its_way_up_through_gestures() {
    let (mut sketch, sides, free) = a_square_and_a_parallel_trait();
    let end = sketch.segments()[free.0].end;

    for length in [120.0, 40.0, 95.0] {
        sketch.set_dimension(DimensionTarget::Length(free), length, false);
        sketch.land_value(DimensionTarget::Length(free), SCALE);
    }
    for height in [80.0, 30.0] {
        sketch.set_dimension(DimensionTarget::Length(sides[3]), height, false);
        sketch.land_value(DimensionTarget::Length(sides[3]), SCALE);
    }
    let to = sketch.point(end) + DVec2::new(30.0, -20.0);
    sketch.settle_around(end, to, SCALE);

    let drift = direction(&sketch, sides[0]);
    assert!(
        drift.abs() < A_HAIR,
        "the bottom still lies along the axis, not at {drift} degrees"
    );
    let top = direction(&sketch, sides[2]).rem_euclid(180.0);
    let along = direction(&sketch, free).rem_euclid(180.0);
    assert!(
        (top - along).abs() < A_HAIR || (180.0 - (top - along).abs()) < A_HAIR,
        "and the trait is still parallel to the top: {top} and {along}"
    );
}

#[test]
fn a_leaning_square_tied_by_a_parallel_to_a_free_trait_is_not_settled() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let a = Sketch::ORIGIN;
    let b = sketch.add_point(DVec2::new(40.0, 30.0));
    let c = sketch.add_point(DVec2::new(10.0, 70.0));
    let d = sketch.add_point(DVec2::new(-30.0, 40.0));
    let sides = [
        sketch.add_segment(a, b),
        sketch.add_segment(b, c),
        sketch.add_segment(c, d),
        sketch.add_segment(d, a),
    ];
    for pair in 0..3 {
        sketch.add_constraint(Constraint::Perpendicular {
            first: sides[pair],
            second: sides[pair + 1],
        });
    }
    sketch.set_dimension(DimensionTarget::Length(sides[0]), 50.0, false);
    sketch.set_dimension(DimensionTarget::Length(sides[1]), 50.0, false);
    let [start, end] =
        [DVec2::new(20.0, 120.0), DVec2::new(90.0, 110.0)].map(|at| sketch.add_point(at));
    let free = sketch.add_segment(start, end);
    sketch.solve(SCALE);
    sketch.lay_rule(
        Constraint::Parallel {
            first: free,
            second: sides[2],
        },
        SCALE,
    );

    let settled = sketch.settled_points(SCALE);
    assert!(
        !settled[b.0],
        "a leaning square says nothing of its way up, parallel or not"
    );
}
