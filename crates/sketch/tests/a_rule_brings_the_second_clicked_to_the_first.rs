//! Closes #451.
//! - a perpendicular laid in both orders leaves the first trait clicked where
//!   it was — `a_perpendicular_leaves_the_first_trait_clicked_where_it_was`
//! - a parallel and a collinear do the same —
//!   `a_parallel_leaves_the_first_trait_clicked_where_it_was`,
//!   `a_collinear_leaves_the_first_trait_clicked_where_it_was`
//! - a tangent between a trait and a circle brings the second clicked to the
//!   first — `a_tangent_brings_the_second_clicked_to_the_first`
//! - a point laid on a trait comes onto it in both orders —
//!   `a_point_laid_on_a_trait_comes_onto_it_whichever_was_clicked_first`
//! - **Equal** keeps the first trait's length, as today —
//!   `an_equality_keeps_the_length_of_the_first_trait_clicked`
//! - an angle typed between two free traits, in both orders, turns only the
//!   second trait clicked — `an_angle_typed_turns_only_the_second_trait_clicked`
//!
//! Beside the criteria, what the issue says around them: two circles made equal
//! (`two_circles_made_equal_leave_the_first_clicked_as_it_was`), an angle where
//! two traits cross
//! (`an_angle_typed_where_two_traits_cross_turns_only_the_second_clicked`), the
//! rank of #445 deciding before the order of the clicks
//! (`a_fixed_trait_stays_although_it_was_clicked_second`), an angle retyped
//! turning the same trait
//! (`an_angle_retyped_the_other_way_round_turns_the_same_trait`), and the
//! reference held only while the rule lands
//! (`a_length_typed_later_on_the_second_trait_reaches_the_first`,
//! `an_end_of_the_second_of_two_equal_traits_dragged_stretches_the_first_with_it`).

use cao_sketch::{
    Constraint, DimensionTarget, Element, LaidFrom, LengthOutcome, PointId, Rule, RuleIntent,
    RulePick, SegmentId, Sketch, Toward, WorkPlane, rule_intent,
};
use glam::DVec2;

const SCALE: f64 = 1.0;
const CLOSE: f64 = 1e-6;
/// What the solver promises of a value it had to reach by moving both sides of
/// a rule: a hundred-thousandth of the drawing's size, which is a hundred
/// units across here.
const SETTLED: f64 = 1e-5 * 100.0;

/// Two traits hanging off nothing, leaning at an angle no rule names: one from
/// (10, 10) to (110, 30), the other from (10, 90) to (60, 180). Neither
/// touches the origin, so nothing but the rule laid decides which gives way.
fn two_free_traits() -> Sketch {
    let mut sketch = Sketch::new(WorkPlane::XY);
    for place in [
        DVec2::new(10.0, 10.0),
        DVec2::new(110.0, 30.0),
        DVec2::new(10.0, 90.0),
        DVec2::new(60.0, 180.0),
    ] {
        sketch.add_point(place);
    }
    sketch.add_segment(PointId(1), PointId(2));
    sketch.add_segment(PointId(3), PointId(4));
    sketch
}

/// Two traits meeting at a corner, 60° or so apart.
fn a_corner() -> Sketch {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corner = sketch.add_point(DVec2::new(10.0, 10.0));
    let far_one = sketch.add_point(DVec2::new(110.0, 10.0));
    let far_other = sketch.add_point(DVec2::new(60.0, 100.0));
    sketch.add_segment(corner, far_one);
    sketch.add_segment(corner, far_other);
    sketch
}

fn ends(sketch: &Sketch, segment: usize) -> [DVec2; 2] {
    let drawn = sketch.segments()[segment];
    [sketch.point(drawn.start), sketch.point(drawn.end)]
}

/// How far the trait moved, taken as the worse of its two ends.
fn moved(sketch: &Sketch, segment: usize, from: [DVec2; 2]) -> f64 {
    let now = ends(sketch, segment);
    now[0].distance(from[0]).max(now[1].distance(from[1]))
}

const BOTH_ORDERS: [[usize; 2]; 2] = [[0, 1], [1, 0]];

/// Lays a rule between the two traits of `two_free_traits`, clicked in each
/// order in turn, and says how far the first and the second clicked travelled.
fn laid(rule: impl Fn(SegmentId, SegmentId) -> Constraint) -> [(f64, f64); 2] {
    BOTH_ORDERS.map(|[first, second]| {
        let mut sketch = two_free_traits();
        let before = [ends(&sketch, 0), ends(&sketch, 1)];
        let outcome = sketch.lay_rule(rule(SegmentId(first), SegmentId(second)), SCALE);
        assert_eq!(outcome, LengthOutcome::Exact, "the rule did not land");
        (
            moved(&sketch, first, before[first]),
            moved(&sketch, second, before[second]),
        )
    })
}

fn assert_only_the_second_moved(travel: [(f64, f64); 2], rule: &str) {
    for (order, (first, second)) in travel.into_iter().enumerate() {
        assert!(
            first < CLOSE,
            "{rule}, order {order}: the first trait clicked moved by {first}"
        );
        assert!(
            second > CLOSE,
            "{rule}, order {order}: the second trait clicked did not move"
        );
    }
}

#[test]
fn a_perpendicular_leaves_the_first_trait_clicked_where_it_was() {
    assert_only_the_second_moved(
        laid(|first, second| Constraint::Perpendicular { first, second }),
        "perpendicular",
    );
}

#[test]
fn a_parallel_leaves_the_first_trait_clicked_where_it_was() {
    assert_only_the_second_moved(
        laid(|first, second| Constraint::Parallel { first, second }),
        "parallel",
    );
}

#[test]
fn a_collinear_leaves_the_first_trait_clicked_where_it_was() {
    assert_only_the_second_moved(
        laid(|first, second| Constraint::Collinear { first, second }),
        "collinear",
    );
}

#[test]
fn an_equality_keeps_the_length_of_the_first_trait_clicked() {
    for [first, second] in BOTH_ORDERS {
        let mut sketch = two_free_traits();
        let wanted = sketch.segment_length(SegmentId(first));
        sketch.lay_rule(
            Constraint::Equal {
                first: SegmentId(first),
                second: SegmentId(second),
            },
            SCALE,
        );
        assert!(
            (sketch.segment_length(SegmentId(first)) - wanted).abs() < CLOSE,
            "the first trait clicked changed length"
        );
        assert!(
            (sketch.segment_length(SegmentId(second)) - wanted).abs() < CLOSE,
            "the second trait clicked did not take the first's length"
        );
    }
}

/// What a tangency laid between a trait and a circle moved: how far the
/// trait's ends went, and how far the circle's centre and rim went.
fn touched(centre: DVec2, diameter: Option<f64>, from: LaidFrom) -> (f64, f64) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let a = sketch.add_point(DVec2::new(10.0, 10.0));
    let b = sketch.add_point(DVec2::new(210.0, 10.0));
    let segment = sketch.add_segment(a, b);
    let centre = sketch.add_point(centre);
    let circle = sketch.add_circle(centre, 30.0);
    if let Some(diameter) = diameter {
        sketch.set_dimension(DimensionTarget::Diameter(circle), diameter, false);
        sketch.resolve(SCALE);
    }

    let before = (
        ends(&sketch, 0),
        sketch.point(centre),
        sketch.circle(circle).radius,
    );
    let outcome = sketch.lay_rule(
        Constraint::Tangent {
            circle,
            segment,
            at: None,
            from,
        },
        SCALE,
    );
    assert_eq!(outcome, LengthOutcome::Exact, "the tangency did not land");

    (
        moved(&sketch, 0, before.0),
        sketch.point(centre).distance(before.1) + (sketch.circle(circle).radius - before.2).abs(),
    )
}

#[test]
fn a_tangent_brings_the_second_clicked_to_the_first() {
    let over_the_trait = DVec2::new(100.0, 90.0);
    let beside_its_end = DVec2::new(260.0, 90.0);
    for centre in [over_the_trait, beside_its_end] {
        for diameter in [None, Some(60.0)] {
            for from in [LaidFrom::Curve, LaidFrom::Trait] {
                let (trait_moved, circle_moved) = touched(centre, diameter, from);
                let (stayed, came) = match from {
                    LaidFrom::Trait => (trait_moved, circle_moved),
                    _ => (circle_moved, trait_moved),
                };
                let case = format!("{from:?} first, centre at {centre}, diameter {diameter:?}");
                assert!(
                    stayed < CLOSE,
                    "{case}: the first clicked moved by {stayed}"
                );
                assert!(
                    came > CLOSE,
                    "{case}: the second clicked stayed where it was"
                );
            }
        }
    }
}

#[test]
fn a_point_laid_on_a_trait_comes_onto_it_whichever_was_clicked_first() {
    for point_first in [true, false] {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let a = sketch.add_point(DVec2::new(10.0, 10.0));
        let b = sketch.add_point(DVec2::new(210.0, 10.0));
        let segment = sketch.add_segment(a, b);
        let loose = sketch.add_point(DVec2::new(100.0, 90.0));
        let (point, line) = (
            RulePick::Element(Element::Point(loose)),
            RulePick::Element(Element::Segment(segment)),
        );
        let picks = match point_first {
            true => [point, line],
            false => [line, point],
        };
        let Some(RuleIntent::Constrain(rule)) = rule_intent(Rule::Coincident, &picks, &sketch)
        else {
            panic!("a point and a trait make no coincidence");
        };

        let before = ends(&sketch, 0);
        sketch.lay_rule(rule, SCALE);

        assert!(
            moved(&sketch, 0, before) < CLOSE,
            "point first: {point_first}, the trait moved to meet the point"
        );
        let span = sketch.point(b) - sketch.point(a);
        let across = span.perp_dot(sketch.point(loose) - sketch.point(a)) / span.length();
        assert!(
            across.abs() < CLOSE,
            "point first: {point_first}, the point is {across} off the trait"
        );
    }
}

#[test]
fn two_circles_made_equal_leave_the_first_clicked_as_it_was() {
    for [first, second] in BOTH_ORDERS {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let circles = [
            (DVec2::new(40.0, 40.0), 20.0),
            (DVec2::new(140.0, 60.0), 35.0),
        ]
        .map(|(centre, radius)| {
            let centre = sketch.add_point(centre);
            sketch.add_circle(centre, radius)
        });
        let before = circles.map(|circle| sketch.circle(circle));

        sketch.lay_rule(
            Constraint::EqualRadius {
                first: circles[first],
                second: circles[second],
            },
            SCALE,
        );

        let now = circles.map(|circle| sketch.circle(circle));
        assert!(
            (now[first].radius - before[first].radius).abs() < CLOSE,
            "the first circle clicked changed size"
        );
        assert!(
            (now[second].radius - before[first].radius).abs() < CLOSE,
            "the second circle clicked did not take the first's size"
        );
    }
}

#[test]
fn a_fixed_trait_stays_although_it_was_clicked_second() {
    let mut sketch = two_free_traits();
    let before = [ends(&sketch, 0), ends(&sketch, 1)];
    sketch.add_constraint(Constraint::Fixed {
        element: Element::Segment(SegmentId(1)),
    });
    let outcome = sketch.lay_rule(
        Constraint::Perpendicular {
            first: SegmentId(0),
            second: SegmentId(1),
        },
        SCALE,
    );

    assert_eq!(outcome, LengthOutcome::Exact, "the rule was refused");
    assert!(
        moved(&sketch, 1, before[1]) < CLOSE,
        "the fixed trait moved although nothing may move it"
    );
    assert!(
        moved(&sketch, 0, before[0]) > CLOSE,
        "nothing gave way, so the rule cannot hold"
    );
}

fn type_the_angle(sketch: &mut Sketch, first: usize, second: usize, degrees: f64) {
    let target = DimensionTarget::corner(SegmentId(first), SegmentId(second));
    sketch.set_dimension(target, degrees, false);
    let outcome = sketch.land_value(target, SCALE);
    assert_eq!(outcome, LengthOutcome::Exact, "the angle did not land");
}

#[test]
fn an_angle_typed_turns_only_the_second_trait_clicked() {
    for [first, second] in BOTH_ORDERS {
        let mut sketch = a_corner();
        let before = [ends(&sketch, 0), ends(&sketch, 1)];
        type_the_angle(&mut sketch, first, second, 30.0);

        assert!(
            moved(&sketch, first, before[first]) < CLOSE,
            "order {first}{second}: the first trait clicked turned"
        );
        assert!(
            moved(&sketch, second, before[second]) > CLOSE,
            "order {first}{second}: the second trait clicked did not turn"
        );
    }
}

#[test]
fn an_angle_typed_where_two_traits_cross_turns_only_the_second_clicked() {
    for [first, second] in BOTH_ORDERS {
        let mut sketch = Sketch::new(WorkPlane::XY);
        for place in [
            DVec2::new(10.0, 60.0),
            DVec2::new(110.0, 60.0),
            DVec2::new(60.0, 10.0),
            DVec2::new(80.0, 110.0),
        ] {
            sketch.add_point(place);
        }
        sketch.add_segment(PointId(1), PointId(2));
        sketch.add_segment(PointId(3), PointId(4));
        let before = [ends(&sketch, 0), ends(&sketch, 1)];

        let target = DimensionTarget::AngleBetween {
            first: SegmentId(first),
            first_toward: Toward::End,
            second: SegmentId(second),
            second_toward: Toward::End,
        };
        sketch.set_dimension(target, 60.0, false);
        assert_eq!(sketch.land_value(target, SCALE), LengthOutcome::Exact);

        assert!(
            moved(&sketch, first, before[first]) < CLOSE,
            "order {first}{second}: the first trait clicked turned"
        );
        assert!(
            moved(&sketch, second, before[second]) > CLOSE,
            "order {first}{second}: the second trait clicked did not turn"
        );
    }
}

#[test]
fn an_end_of_the_second_of_two_equal_traits_dragged_stretches_the_first_with_it() {
    let mut sketch = two_free_traits();
    sketch.lay_rule(
        Constraint::Equal {
            first: SegmentId(0),
            second: SegmentId(1),
        },
        SCALE,
    );
    let dragged = sketch.segments()[1].end;
    let cursor = sketch.point(dragged) + DVec2::new(0.0, 30.0);

    assert_eq!(
        sketch.settle_around(dragged, cursor, SCALE),
        LengthOutcome::Exact
    );

    assert!(
        sketch.point(dragged).distance(cursor) < SETTLED,
        "the end dragged stopped short of the hand"
    );
    let lengths = [0, 1].map(|segment| sketch.segment_length(SegmentId(segment)));
    assert!(
        (lengths[0] - lengths[1]).abs() < SETTLED,
        "the two traits are no longer equal: {lengths:?}"
    );
}

#[test]
fn an_angle_retyped_the_other_way_round_turns_the_same_trait() {
    let mut sketch = a_corner();
    type_the_angle(&mut sketch, 0, 1, 30.0);
    let before = [ends(&sketch, 0), ends(&sketch, 1)];

    type_the_angle(&mut sketch, 1, 0, 45.0);

    assert_eq!(sketch.dimensions().len(), 1, "the angle was laid twice");
    assert!(
        moved(&sketch, 0, before[0]) < CLOSE,
        "retyped, the angle turned the trait it was first typed from"
    );
    assert!(
        moved(&sketch, 1, before[1]) > CLOSE,
        "retyped, the angle turned nothing"
    );
}

#[test]
fn a_length_typed_later_on_the_second_trait_reaches_the_first() {
    let mut sketch = two_free_traits();
    sketch.lay_rule(
        Constraint::Equal {
            first: SegmentId(0),
            second: SegmentId(1),
        },
        SCALE,
    );
    let target = DimensionTarget::Length(SegmentId(1));
    sketch.set_dimension(target, 50.0, false);

    assert_eq!(sketch.land_value(target, SCALE), LengthOutcome::Exact);
    let reached = sketch.segment_length(SegmentId(0));
    assert!(
        (reached - 50.0).abs() < SETTLED,
        "the first trait stayed {reached} long"
    );
}
