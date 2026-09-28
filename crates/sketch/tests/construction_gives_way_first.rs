//! Closes #452.
//! - a parallel laid between a profile trait and a free construction trait
//!   turns the construction trait, in both orders of clicks —
//!   `a_parallel_turns_the_construction_trait_whichever_was_clicked_first`
//! - a profile point dragged moves a construction trait tied to it before any
//!   other point of the profile —
//!   `a_corner_dragged_stretches_the_profile_as_if_the_construction_were_not_there`,
//!   `a_profile_that_pivots_turns_about_its_own_corner_not_a_construction_end`,
//!   `a_construction_trait_running_on_from_a_corner_follows_it`
//! - a value typed on a construction trait still holds —
//!   `a_length_typed_on_a_construction_trait_holds_while_it_turns`,
//!   `a_length_typed_on_a_construction_trait_holds_through_a_drag`
//!
//! Beside the criteria, what the issue says around them: a perpendicular turns
//! the construction as a parallel does
//! (`a_perpendicular_turns_the_construction_trait_whichever_was_clicked_first`),
//! a tangency brings a construction trait to a profile circle
//! (`a_tangency_brings_the_construction_trait_to_the_profile_circle`), an angle
//! typed between the two turns the construction
//! (`an_angle_typed_turns_the_construction_trait_whichever_was_clicked_first`),
//! and a construction trait that cannot turn lets the profile give
//! (`a_fixed_construction_trait_lets_the_profile_trait_turn`).

use cao_sketch::{
    Constraint, DimensionTarget, Element, LaidFrom, LengthOutcome, PointId, SegmentId, Sketch,
    WorkPlane,
};
use glam::DVec2;

const SCALE: f64 = 1.0;
const CLOSE: f64 = 1e-6;
/// Close enough for points a drag placed: the solver stops at a
/// hundred-thousandth of the drawing's size.
const SETTLED: f64 = 1e-3;
/// The same promise for a drawing a construction trait runs on to three
/// hundred units across.
const SETTLED_ACROSS_THE_RUN_ON: f64 = 1e-5 * 300.0;

/// A profile trait and a construction trait hanging off nothing, leaning at
/// an angle no rule names.
fn a_profile_trait_and_a_construction_trait() -> (Sketch, SegmentId, SegmentId) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let places = [
        DVec2::new(10.0, 10.0),
        DVec2::new(110.0, 30.0),
        DVec2::new(10.0, 90.0),
        DVec2::new(60.0, 180.0),
    ]
    .map(|place| sketch.add_point(place));
    let profile = sketch.add_segment(places[0], places[1]);
    let construction = sketch.add_construction_segment(places[2], places[3]);
    (sketch, profile, construction)
}

fn ends(sketch: &Sketch, segment: SegmentId) -> [DVec2; 2] {
    let (start, end) = sketch.endpoints(segment);
    [start, end]
}

fn moved(sketch: &Sketch, segment: SegmentId, from: [DVec2; 2]) -> f64 {
    let now = ends(sketch, segment);
    now[0].distance(from[0]).max(now[1].distance(from[1]))
}

/// Lays a rule between the two traits, clicked each way round, and asserts
/// the profile trait stayed and the construction trait came to it.
fn assert_the_construction_gives(rule: impl Fn(SegmentId, SegmentId) -> Constraint) {
    for construction_first in [true, false] {
        let (mut sketch, profile, construction) = a_profile_trait_and_a_construction_trait();
        let before = (ends(&sketch, profile), ends(&sketch, construction));
        let laid = match construction_first {
            true => rule(construction, profile),
            false => rule(profile, construction),
        };

        let outcome = sketch.lay_rule(laid, SCALE);

        assert_eq!(outcome, LengthOutcome::Exact, "the rule did not land");
        assert!(
            moved(&sketch, profile, before.0) < CLOSE,
            "construction first: {construction_first}, the profile trait moved"
        );
        assert!(
            moved(&sketch, construction, before.1) > CLOSE,
            "construction first: {construction_first}, the construction trait stayed"
        );
    }
}

#[test]
fn a_parallel_turns_the_construction_trait_whichever_was_clicked_first() {
    assert_the_construction_gives(|first, second| Constraint::Parallel { first, second });
}

#[test]
fn a_perpendicular_turns_the_construction_trait_whichever_was_clicked_first() {
    assert_the_construction_gives(|first, second| Constraint::Perpendicular { first, second });
}

#[test]
fn a_fixed_construction_trait_lets_the_profile_trait_turn() {
    let (mut sketch, profile, construction) = a_profile_trait_and_a_construction_trait();
    let before = (ends(&sketch, profile), ends(&sketch, construction));
    sketch.add_constraint(Constraint::Fixed {
        element: Element::Segment(construction),
    });

    let outcome = sketch.lay_rule(
        Constraint::Parallel {
            first: profile,
            second: construction,
        },
        SCALE,
    );

    assert_eq!(outcome, LengthOutcome::Exact, "the rule was refused");
    assert!(
        moved(&sketch, construction, before.1) < CLOSE,
        "the fixed construction trait moved"
    );
    assert!(
        moved(&sketch, profile, before.0) > CLOSE,
        "nothing gave way, so the rule cannot hold"
    );
}

#[test]
fn a_length_typed_on_a_construction_trait_holds_while_it_turns() {
    let (mut sketch, profile, construction) = a_profile_trait_and_a_construction_trait();
    sketch.set_dimension(DimensionTarget::Length(construction), 80.0, false);
    sketch.resolve(SCALE);
    let before = ends(&sketch, profile);

    let outcome = sketch.lay_rule(
        Constraint::Parallel {
            first: construction,
            second: profile,
        },
        SCALE,
    );

    assert_eq!(outcome, LengthOutcome::Exact, "the rule did not land");
    assert!(
        moved(&sketch, profile, before) < CLOSE,
        "the profile trait moved"
    );
    assert!(
        (sketch.segment_length(construction) - 80.0).abs() < CLOSE,
        "the construction trait's typed length gave: {}",
        sketch.segment_length(construction)
    );
}

#[test]
fn an_angle_typed_turns_the_construction_trait_whichever_was_clicked_first() {
    for construction_first in [true, false] {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let corner = sketch.add_point(DVec2::new(10.0, 10.0));
        let far_one = sketch.add_point(DVec2::new(110.0, 10.0));
        let far_other = sketch.add_point(DVec2::new(60.0, 100.0));
        let profile = sketch.add_segment(corner, far_one);
        let construction = sketch.add_construction_segment(corner, far_other);
        let before = (ends(&sketch, profile), ends(&sketch, construction));
        let (first, second) = match construction_first {
            true => (construction, profile),
            false => (profile, construction),
        };
        let target = DimensionTarget::Angle { first, second };

        sketch.set_dimension(target, 45.0, false);
        let outcome = sketch.land_value(target, SCALE);

        assert_eq!(outcome, LengthOutcome::Exact, "the angle did not land");
        assert!(
            moved(&sketch, profile, before.0) < CLOSE,
            "construction first: {construction_first}, the profile trait turned"
        );
        assert!(
            moved(&sketch, construction, before.1) > CLOSE,
            "construction first: {construction_first}, the construction trait stayed"
        );
    }
}

#[test]
fn a_tangency_brings_the_construction_trait_to_the_profile_circle() {
    let over_the_trait = DVec2::new(100.0, 90.0);
    let beside_its_end = DVec2::new(260.0, 90.0);
    for centre in [over_the_trait, beside_its_end] {
        for from in [LaidFrom::Trait, LaidFrom::Curve] {
            let mut sketch = Sketch::new(WorkPlane::XY);
            let a = sketch.add_point(DVec2::new(10.0, 10.0));
            let b = sketch.add_point(DVec2::new(210.0, 10.0));
            let segment = sketch.add_construction_segment(a, b);
            let centre = sketch.add_point(centre);
            let circle = sketch.add_circle(centre, 30.0);
            let before = (sketch.point(centre), sketch.circle(circle).radius);

            let outcome = sketch.lay_rule(
                Constraint::Tangent {
                    circle,
                    segment,
                    at: None,
                    from,
                },
                SCALE,
            );

            let case = format!("{from:?} first, centre at {}", before.0);
            assert_eq!(outcome, LengthOutcome::Exact, "{case}: it did not land");
            assert!(
                sketch.point(centre).distance(before.0) < CLOSE,
                "{case}: the profile circle moved"
            );
            assert!(
                (sketch.circle(circle).radius - before.1).abs() < CLOSE,
                "{case}: the profile circle changed size"
            );
        }
    }
}

/// A rectangle as the tool lays it: four traits, three right angles.
fn upright(sketch: &mut Sketch) -> ([PointId; 4], [SegmentId; 4]) {
    let points = [
        DVec2::new(20.0, 20.0),
        DVec2::new(120.0, 20.0),
        DVec2::new(120.0, 70.0),
        DVec2::new(20.0, 70.0),
    ]
    .map(|place| sketch.add_point(place));
    let sides: [SegmentId; 4] =
        std::array::from_fn(|rank| sketch.add_segment(points[rank], points[(rank + 1) % 4]));
    for corner in 0..3 {
        sketch.add_constraint(Constraint::Perpendicular {
            first: sides[corner],
            second: sides[corner + 1],
        });
    }
    (points, sides)
}

/// A construction trait running on from `corner` along the bottom side,
/// kept in line with it by a parallel: the far end lies further from the
/// first corner than the opposite corner does.
fn with_a_construction_run_on(
    sketch: &mut Sketch,
    corner: PointId,
    sides: [SegmentId; 4],
    to: DVec2,
) -> SegmentId {
    let far = sketch.add_point(to);
    let run_on = sketch.add_construction_segment(corner, far);
    sketch.add_constraint(Constraint::Parallel {
        first: sides[0],
        second: run_on,
    });
    run_on
}

/// The profile's corners after the first one is dragged to `to`, the drawing
/// dressed by `dress` first.
fn dragged(to: DVec2, dress: impl Fn(&mut Sketch, [PointId; 4], [SegmentId; 4])) -> [DVec2; 4] {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let (points, sides) = upright(&mut sketch);
    dress(&mut sketch, points, sides);
    let outcome = sketch.settle_around(points[0], to, SCALE);
    assert_eq!(outcome, LengthOutcome::Exact, "the drag was refused");
    points.map(|point| sketch.point(point))
}

fn assert_same_profile(found: [DVec2; 4], wanted: [DVec2; 4], case: &str) {
    for (rank, (found, wanted)) in found.iter().zip(wanted).enumerate() {
        assert!(
            found.distance(wanted) < SETTLED,
            "{case}: corner {rank} is at {found}, and would be at {wanted} with no construction"
        );
    }
}

#[test]
fn a_corner_dragged_stretches_the_profile_as_if_the_construction_were_not_there() {
    let to = DVec2::new(5.0, 45.0);
    let bare = dragged(to, |_, _, _| {});
    let off_the_next_corner = dragged(to, |sketch, [_, next, _, _], sides| {
        with_a_construction_run_on(sketch, next, sides, DVec2::new(300.0, 20.0));
    });
    let off_the_corner_dragged = dragged(to, |sketch, [dragged, ..], sides| {
        with_a_construction_run_on(sketch, dragged, sides, DVec2::new(-200.0, 20.0));
    });

    assert_same_profile(off_the_next_corner, bare, "run on from the next corner");
    assert_same_profile(
        off_the_corner_dragged,
        bare,
        "run on from the corner dragged",
    );
}

#[test]
fn a_profile_that_pivots_turns_about_its_own_corner_not_a_construction_end() {
    let to = DVec2::new(10.0, 40.0);
    let typed = |sketch: &mut Sketch, sides: [SegmentId; 4]| {
        for side in [sides[0], sides[1]] {
            let length = sketch.segment_length(side);
            sketch.set_dimension(DimensionTarget::Length(side), length, false);
        }
    };
    let bare = dragged(to, |sketch, _, sides| typed(sketch, sides));
    let with_run_on = dragged(to, |sketch, [_, next, _, _], sides| {
        typed(sketch, sides);
        with_a_construction_run_on(sketch, next, sides, DVec2::new(300.0, 20.0));
    });

    assert_same_profile(with_run_on, bare, "a rectangle typed on both sides");
}

#[test]
fn a_length_typed_on_a_construction_trait_holds_through_a_drag() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let ([dragged, next, _, _], sides) = upright(&mut sketch);
    let run_on = with_a_construction_run_on(&mut sketch, next, sides, DVec2::new(300.0, 20.0));
    sketch.set_dimension(DimensionTarget::Length(run_on), 180.0, false);

    let outcome = sketch.settle_around(dragged, DVec2::new(5.0, 45.0), SCALE);

    assert_eq!(outcome, LengthOutcome::Exact, "the drag was refused");
    assert!(
        (sketch.segment_length(run_on) - 180.0).abs() < SETTLED,
        "the construction trait's typed length gave: {}",
        sketch.segment_length(run_on)
    );
}

#[test]
fn a_construction_trait_running_on_from_a_corner_follows_it() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let ([dragged, next, _, _], sides) = upright(&mut sketch);
    let run_on = with_a_construction_run_on(&mut sketch, next, sides, DVec2::new(300.0, 20.0));

    let outcome = sketch.settle_around(dragged, DVec2::new(5.0, 45.0), SCALE);

    assert_eq!(outcome, LengthOutcome::Exact, "the drag was refused");
    let (start, end) = sketch.endpoints(run_on);
    assert!(
        start.distance(DVec2::new(120.0, 45.0)) < SETTLED_ACROSS_THE_RUN_ON,
        "the construction trait no longer starts on the corner: {start}"
    );
    assert!(
        (end.y - 45.0).abs() < SETTLED_ACROSS_THE_RUN_ON,
        "the construction trait's far end stayed behind at {end}"
    );
}
