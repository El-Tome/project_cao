//! A line held tangent to an arc is held against the whole circle the arc is a
//! piece of, so a drag along itself may leave the touch past the line's end.
//! Compacting the history lays the rule again on the drawing as it stands, and
//! must not bring the line, or the arc, back to where the touch would fall on
//! the line: nothing the user left where it is moves.
//!
//! Found while fixing the same defect for a fillet in #651, where
//! `a_compacted_part_whose_line_was_dragged_past_the_touch_moves_nothing`
//! holds it.

use cao_part::{Operation, PartDocument, PointRef};
use cao_sketch::{
    ArcId, Constraint, Element, Rule, RuleIntent, RulePick, SegmentId, Sketch, WorkPlane,
    rule_intent,
};
use glam::DVec2;

/// How near two places must stand to be the same place after a solve.
const SAME: f64 = 1e-6;

const ARC: ArcId = ArcId(0);
const LINE: SegmentId = SegmentId(0);

/// An arc of radius 20 about (50, 50), running below its centre from 200° to
/// 340°, and a free line under it from (35, 25) to (65, 25), held tangent to
/// it, the line or the arc clicked first.
fn a_line_held_tangent_to_an_arc(line_first: bool) -> PartDocument {
    let mut document = PartDocument::new("Plaque", "2026-11-03T09:00:00Z".parse().expect("a date"));
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    let centre = DVec2::new(50.0, 50.0);
    let around = |degrees: f64| centre + DVec2::from_angle(degrees.to_radians()) * 20.0;
    document.apply(Operation::AddArc {
        sketch: 0,
        center: PointRef::New(centre),
        start: PointRef::New(around(200.0)),
        end: PointRef::New(around(340.0)),
        construction: false,
    });
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::new(35.0, 25.0)),
        end: PointRef::New(DVec2::new(65.0, 25.0)),
        construction: false,
    });
    let (line, arc) = (
        RulePick::Element(Element::Segment(LINE)),
        RulePick::Element(Element::Arc(ARC)),
    );
    let picks = match line_first {
        true => [line, arc],
        false => [arc, line],
    };
    let Some(RuleIntent::Constrain(rule)) =
        rule_intent(Rule::Tangent, &picks, &document.sketches()[0])
    else {
        panic!("a line and an arc make no tangency");
    };
    let outcome = document.apply(Operation::Constrain {
        sketch: 0,
        constraint: rule,
    });
    assert!(outcome.is_none(), "the tangency was refused: {outcome:?}");
    document
}

/// How far the line, taken whole, runs from the arc's centre, and how far the
/// touch falls past the nearer end of the line.
fn reach_and_past(sketch: &Sketch) -> (f64, f64) {
    let centre = sketch.point(sketch.arc(ARC).center);
    let (start, end) = sketch.endpoints(LINE);
    let run = end - start;
    let reach = run.normalize().perp_dot(centre - start).abs();
    let along = (centre - start).dot(run) / run.length_squared();
    (reach, (along - along.clamp(0.0, 1.0)).abs() * run.length())
}

/// The places a point stood before, where none stands now.
fn left_empty(before: &Sketch, after: &Sketch) -> Vec<DVec2> {
    before
        .live_points()
        .map(|(_, place)| place)
        .filter(|place| after.nearest_point(*place, SAME).is_none())
        .collect()
}

#[test]
fn a_compacted_part_whose_line_was_dragged_past_an_arc_s_touch_moves_nothing() {
    let mut moved = Vec::new();
    for line_first in [false, true] {
        let case = match line_first {
            true => "the line clicked first",
            false => "the arc clicked first",
        };
        let mut document = a_line_held_tangent_to_an_arc(line_first);
        let held = document.sketches()[0].segments()[LINE.0];
        document.apply(Operation::MoveMany {
            sketch: 0,
            points: vec![held.start, held.end],
            by: DVec2::new(30.0, 0.0),
        });
        let before = document.sketches()[0].clone();
        let (reach, past) = reach_and_past(&before);
        let arc = before.arc(ARC);
        let radius = before.point(arc.center).distance(before.point(arc.start));
        assert!(
            (reach - radius).abs() < SAME,
            "{case}: the line runs {reach} from the arc's centre, its radius is {radius}"
        );
        assert!(
            past > 1.0,
            "{case}: the line was to be dragged along itself past the touch"
        );

        document.compact_history();

        let after = &document.sketches()[0];
        assert!(
            after
                .constraints()
                .iter()
                .any(|rule| matches!(rule, Constraint::ArcTangent { .. })),
            "{case}: compacted, the line is no longer held tangent to the arc"
        );
        assert_eq!(
            after.live_points().count(),
            before.live_points().count(),
            "{case}: compacted, points came or went"
        );
        let gone = left_empty(&before, after);
        if !gone.is_empty() {
            moved.push(format!("{case}: the points at {gone:?}"));
        }
    }
    assert!(moved.is_empty(), "compacted, moved: {moved:?}");
}
