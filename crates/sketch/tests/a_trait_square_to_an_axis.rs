//! Closes #463.
//! - « Parallèle » and « Perpendiculaire » take a sketch axis as one of their
//!   two picks, in both orders —
//!   `parallel_and_perpendicular_take_an_axis_whichever_is_clicked_first`
//! - a free trait made parallel or perpendicular to an axis turns to it, the
//!   axis staying — `a_free_trait_turns_onto_the_axis_about_its_end_nearest_the_origin`
//! - a rectangle side made parallel to an axis turns the whole rectangle, which
//!   keeps its right angles —
//!   `a_rectangle_side_made_square_to_an_axis_turns_the_whole_rectangle`
//! - the rule holds afterwards: a drag does not turn the trait off it —
//!   `a_corner_dragged_afterwards_leaves_the_side_the_way_of_the_axis`
//!
//! Beside the criteria, what the issue says must not break: a trait already
//! running the way of the axis
//! (`a_trait_already_the_way_of_the_axis_takes_the_rule_and_does_not_move`), a
//! shape drawn from the origin turning about it
//! (`a_rectangle_drawn_from_the_origin_turns_about_the_origin`), a fixed corner
//! being what the shape turns about
//! (`a_fixed_corner_is_what_the_shape_turns_about`), and « Colinéaire » with an
//! axis as it was (`collinear_with_an_axis_is_what_it_was`).

use cao_sketch::{
    Constraint, Element, LengthOutcome, PointId, Rule, RuleIntent, RulePick, SegmentId, Sketch,
    SketchAxis, WorkPlane, rule_intent,
};
use glam::DVec2;

const SCALE: f64 = 1.0;
const CLOSE: f64 = 1e-6;
/// What the solver leaves of a rule it had to reach by moving points: a
/// hundred-thousandth of the drawing's size, a hundred units across here.
const SETTLED: f64 = 1e-3;

/// How the tool lays a rectangle: four traits, three right angles, nothing
/// saying which way up. This one leans by 0.3 rad, its first corner at
/// `corner`.
fn leaning_rectangle(corner: DVec2) -> (Sketch, [PointId; 4], [SegmentId; 4]) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let along = DVec2::from_angle(0.3);
    let across = along.perp();
    let corners = [
        corner,
        corner + along * 100.0,
        corner + along * 100.0 + across * 50.0,
        corner + across * 50.0,
    ];
    let points = corners.map(|place| sketch.add_point(place));
    let sides: [SegmentId; 4] =
        std::array::from_fn(|rank| sketch.add_segment(points[rank], points[(rank + 1) % 4]));
    for rank in 0..3 {
        sketch.add_constraint(Constraint::Perpendicular {
            first: sides[rank],
            second: sides[rank + 1],
        });
    }
    (sketch, points, sides)
}

fn direction(sketch: &Sketch, side: SegmentId) -> DVec2 {
    let drawn = sketch.segments()[side.0];
    (sketch.point(drawn.end) - sketch.point(drawn.start)).normalize()
}

/// How far a trait's direction stands off an axis's, either way along it.
fn off(found: DVec2, wanted: DVec2) -> f64 {
    found.perp_dot(wanted).abs()
}

fn square_to(axis: SketchAxis, square: bool) -> DVec2 {
    match square {
        true => axis.direction().perp(),
        false => axis.direction(),
    }
}

fn rule(segment: SegmentId, axis: SketchAxis, square: bool) -> Constraint {
    match square {
        true => Constraint::AxisPerpendicular { segment, axis },
        false => Constraint::AxisParallel { segment, axis },
    }
}

const EVERY_RULE: [(SketchAxis, bool); 4] = [
    (SketchAxis::U, false),
    (SketchAxis::U, true),
    (SketchAxis::V, false),
    (SketchAxis::V, true),
];

#[test]
fn parallel_and_perpendicular_take_an_axis_whichever_is_clicked_first() {
    let (sketch, _, sides) = leaning_rectangle(DVec2::new(40.0, 30.0));
    let side = RulePick::Element(Element::Segment(sides[0]));
    for axis in [SketchAxis::U, SketchAxis::V] {
        for picks in [[side, RulePick::Axis(axis)], [RulePick::Axis(axis), side]] {
            assert_eq!(
                rule_intent(Rule::Parallel, &picks, &sketch),
                Some(RuleIntent::Constrain(Constraint::AxisParallel {
                    segment: sides[0],
                    axis
                })),
                "parallel, {picks:?}"
            );
            assert_eq!(
                rule_intent(Rule::Perpendicular, &picks, &sketch),
                Some(RuleIntent::Constrain(Constraint::AxisPerpendicular {
                    segment: sides[0],
                    axis
                })),
                "perpendicular, {picks:?}"
            );
        }
    }
}

#[test]
fn a_free_trait_turns_onto_the_axis_about_its_end_nearest_the_origin() {
    for (axis, square) in EVERY_RULE {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let near = sketch.add_point(DVec2::new(30.0, 20.0));
        let far = sketch.add_point(DVec2::new(110.0, 60.0));
        let trait_ = sketch.add_segment(near, far);
        let length = sketch.segment_length(trait_);

        let outcome = sketch.lay_rule(rule(trait_, axis, square), SCALE);

        assert_eq!(outcome, LengthOutcome::Exact, "{axis:?} {square}");
        assert!(
            off(direction(&sketch, trait_), square_to(axis, square)) < CLOSE,
            "{axis:?} {square}: the trait did not turn to the axis"
        );
        assert!(
            sketch.point(near).distance(DVec2::new(30.0, 20.0)) < CLOSE,
            "{axis:?} {square}: the end nearest the origin moved"
        );
        assert!(
            (sketch.segment_length(trait_) - length).abs() < CLOSE,
            "{axis:?} {square}: the trait changed length"
        );
        assert!(
            sketch.point(Sketch::ORIGIN).length() < CLOSE,
            "the origin, and its axes, moved"
        );
    }
}

/// The rectangle turned whole: its sides keep their lengths and their right
/// angles, and `stays` is where it was.
fn assert_turned_whole(sketch: &Sketch, sides: [SegmentId; 4], stays: PointId, at: DVec2) {
    for (rank, side) in sides.iter().enumerate() {
        let wanted = if rank % 2 == 0 { 100.0 } else { 50.0 };
        let length = sketch.segment_length(*side);
        assert!(
            (length - wanted).abs() < SETTLED,
            "side {rank} went from {wanted} to {length}"
        );
        let next = sides[(rank + 1) % 4];
        let corner = direction(sketch, *side).dot(direction(sketch, next));
        assert!(corner.abs() < SETTLED, "corner {rank} is no longer square");
    }
    assert!(
        sketch.point(stays).distance(at) < SETTLED,
        "the corner it turns about moved to {}",
        sketch.point(stays)
    );
}

#[test]
fn a_rectangle_side_made_square_to_an_axis_turns_the_whole_rectangle() {
    let corner = DVec2::new(40.0, 30.0);
    for side in 0..2 {
        for (axis, square) in EVERY_RULE {
            let (mut sketch, points, sides) = leaning_rectangle(corner);

            let outcome = sketch.lay_rule(rule(sides[side], axis, square), SCALE);

            assert_eq!(
                outcome,
                LengthOutcome::Exact,
                "side {side}, {axis:?} {square}"
            );
            assert!(
                off(direction(&sketch, sides[side]), square_to(axis, square)) < SETTLED,
                "side {side}, {axis:?} {square}: the side is not the axis's way"
            );
            assert_turned_whole(&sketch, sides, points[0], corner);
        }
    }
}

#[test]
fn a_corner_dragged_afterwards_leaves_the_side_the_way_of_the_axis() {
    for (axis, square) in EVERY_RULE {
        let (mut sketch, points, sides) = leaning_rectangle(DVec2::new(40.0, 30.0));
        sketch.lay_rule(rule(sides[0], axis, square), SCALE);

        let dropped = sketch.point(points[2]) + DVec2::new(35.0, -20.0);
        sketch.settle_around(points[2], dropped, SCALE);

        assert!(
            off(direction(&sketch, sides[0]), square_to(axis, square)) < SETTLED,
            "{axis:?} {square}: the drag turned the side off the axis"
        );
    }
}

#[test]
fn a_trait_already_the_way_of_the_axis_takes_the_rule_and_does_not_move() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let places = [DVec2::new(30.0, 20.0), DVec2::new(30.0, 90.0)];
    let ends = places.map(|place| sketch.add_point(place));
    let upright = sketch.add_segment(ends[0], ends[1]);

    let laid = Constraint::AxisPerpendicular {
        segment: upright,
        axis: SketchAxis::U,
    };
    assert_eq!(sketch.lay_rule(laid, SCALE), LengthOutcome::Exact);

    assert!(
        sketch.constraints().contains(&laid),
        "the rule was not kept"
    );
    for (end, place) in ends.iter().zip(places) {
        assert!(sketch.point(*end).distance(place) < CLOSE, "an end moved");
    }
}

#[test]
fn a_rectangle_drawn_from_the_origin_turns_about_the_origin() {
    let (mut sketch, points, sides) = leaning_rectangle(DVec2::ZERO);
    sketch.merge_points(Sketch::ORIGIN, points[0]);

    sketch.lay_rule(rule(sides[1], SketchAxis::U, true), SCALE);

    assert!(
        off(direction(&sketch, sides[1]), DVec2::Y) < SETTLED,
        "the side did not stand upright"
    );
    assert_turned_whole(&sketch, sides, Sketch::ORIGIN, DVec2::ZERO);
}

#[test]
fn a_fixed_corner_is_what_the_shape_turns_about() {
    let (mut sketch, points, sides) = leaning_rectangle(DVec2::new(40.0, 30.0));
    let fixed = sketch.point(points[2]);
    sketch.add_constraint(Constraint::Fixed {
        element: Element::Point(points[2]),
    });

    sketch.lay_rule(rule(sides[0], SketchAxis::V, false), SCALE);

    assert!(
        off(direction(&sketch, sides[0]), DVec2::Y) < SETTLED,
        "the side did not stand upright"
    );
    assert_turned_whole(&sketch, sides, points[2], fixed);
}

#[test]
fn collinear_with_an_axis_is_what_it_was() {
    let (sketch, _, sides) = leaning_rectangle(DVec2::new(40.0, 30.0));
    let picks = [
        RulePick::Element(Element::Segment(sides[0])),
        RulePick::Axis(SketchAxis::U),
    ];
    assert_eq!(
        rule_intent(Rule::Collinear, &picks, &sketch),
        Some(RuleIntent::Constrain(Constraint::AxisCollinear {
            segment: sides[0],
            axis: SketchAxis::U
        }))
    );
}
