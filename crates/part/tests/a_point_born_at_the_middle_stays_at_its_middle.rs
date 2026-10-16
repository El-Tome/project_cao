//! A point born at the middle of a trait carries the midpoint rule, and the
//! rule survives a replay, an undo and a compaction.
//!
//! Closes #480.
//! - a point born at the middle of a trait carries the midpoint rule and
//!   follows the middle when the trait changes length —
//!   `a_point_born_at_the_middle_follows_it_when_the_trait_stretches`
//! - whatever tool made it — no test: every tool reads the place through
//!   `born_at`, held beside it in `cao_app` by
//!   `a_point_born_at_the_middle_of_a_trait_is_held_at_its_middle`
//! - a line started from the middle of a side stays at its middle —
//!   `a_line_started_at_the_middle_keeps_its_start_there`
//! - the midpoint rule replaces the hold on the trait —
//!   `the_midpoint_rule_replaces_the_hold_on_the_trait`
//! - a point born anywhere else along a trait is held on it, as today — no
//!   test: `a_place_elsewhere_along_a_trait_lands_on_the_trait_alone` holds it
//!   in `cao_sketch`, beside the reading
//! - a point born at a crossing that is the middle of one trait is at the
//!   middle of that one and on the other — no test:
//!   `a_crossing_at_the_middle_of_one_trait_lands_at_that_middle_and_on_the_other`,
//!   in `cao_sketch` too
//! - a point the drawing already has, reused by a tool, gets nothing new — no
//!   test: `point_ref_at` answers an existing point before `born_at` is asked,
//!   as it always has
//! - a point dragged and dropped on the middle is only held on the trait — no
//!   test: `a_point_dropped_at_the_middle_by_a_drag_is_held_on_the_trait_alone`
//!   holds `supports_for` in `cao_sketch`
//! - the symmetric line keeps the midpoint rule on its own segment, laid
//!   once — no test: `a_symmetric_segment_is_one_step_holding_its_middle`
//!   already holds it, in `drawn_shapes.rs` beside the operation
//! - the point and its rule are one step of the history —
//!   `one_undo_takes_back_both_the_point_and_its_midpoint_rule` and
//!   `compacting_the_history_carries_the_midpoint_rule`
//! - the "1/2" mark shows on the point straight away — no test: `rule_marks`
//!   already marks a midpoint rule, and what draws the mark is the canvas,
//!   which has no net

use cao_part::{Operation, PartDocument, PointRef};
use cao_sketch::{PointId, SegmentId, Support, WorkPlane};
use chrono::{DateTime, Utc};
use glam::DVec2;

fn at(text: &str) -> DateTime<Utc> {
    text.parse().expect("a date")
}

/// One trait lying flat, from (10, 10) to (50, 10).
fn a_flat_trait() -> PartDocument {
    let mut document = PartDocument::new("Test", at("2026-01-02T09:00:00Z"));
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::new(10.0, 10.0)),
        end: PointRef::New(DVec2::new(50.0, 10.0)),
        construction: false,
    });
    document
}

fn middle_of(document: &PartDocument, side: SegmentId) -> DVec2 {
    let drawing = &document.sketches()[0];
    let (start, end) = drawing.endpoints(side);
    (start + end) * 0.5
}

#[test]
fn a_point_born_at_the_middle_follows_it_when_the_trait_stretches() {
    let mut document = a_flat_trait();
    document.apply(Operation::AddPoint {
        sketch: 0,
        position: DVec2::new(30.0, 10.0),
        on: vec![Support::Middle(SegmentId(0))],
    });
    let point = PointId(document.sketches()[0].points().len() - 1);

    document.apply(Operation::MovePoint {
        sketch: 0,
        point: PointId(1),
        position: DVec2::new(90.0, 10.0),
        merged_into: None,
        on: Vec::new(),
        let_go: false,
    });

    let middle = middle_of(&document, SegmentId(0));
    let place = document.sketches()[0].point(point);
    assert!(
        place.distance(middle) < 1e-6,
        "the trait was stretched and its middle moved to {middle}, the point stayed at {place}",
    );
}

#[test]
fn the_midpoint_rule_replaces_the_hold_on_the_trait() {
    let mut document = a_flat_trait();
    document.apply(Operation::AddPoint {
        sketch: 0,
        position: DVec2::new(30.0, 10.0),
        on: vec![Support::Middle(SegmentId(0))],
    });
    let point = PointId(document.sketches()[0].points().len() - 1);

    let drawing = &document.sketches()[0];
    let holds = |rule: &cao_sketch::Constraint| match *rule {
        cao_sketch::Constraint::Midpoint { point: held, .. } => (held == point).then_some("middle"),
        cao_sketch::Constraint::OnSegment { point: held, .. } => (held == point).then_some("on"),
        _ => None,
    };
    let rules: Vec<_> = drawing.constraints().iter().filter_map(holds).collect();
    assert_eq!(rules, vec!["middle"], "the middle is already on the trait");
}

#[test]
fn a_line_started_at_the_middle_keeps_its_start_there() {
    let mut document = a_flat_trait();
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::Held {
            at: DVec2::new(30.0, 10.0),
            on: vec![Support::Middle(SegmentId(0))],
        },
        end: PointRef::New(DVec2::new(30.0, 40.0)),
        construction: false,
    });
    let foot = PointId(document.sketches()[0].points().len() - 2);

    document.apply(Operation::MovePoint {
        sketch: 0,
        point: PointId(0),
        position: DVec2::new(-30.0, 10.0),
        merged_into: None,
        on: Vec::new(),
        let_go: false,
    });

    let middle = middle_of(&document, SegmentId(0));
    let place = document.sketches()[0].point(foot);
    assert!(
        place.distance(middle) < 1e-6,
        "the side was stretched and its middle moved to {middle}, the line starts at {place}",
    );
}

#[test]
fn one_undo_takes_back_both_the_point_and_its_midpoint_rule() {
    let mut document = a_flat_trait();
    let points = document.sketches()[0].points().len();
    let rules = document.sketches()[0].constraints().len();

    document.apply(Operation::AddPoint {
        sketch: 0,
        position: DVec2::new(30.0, 10.0),
        on: vec![Support::Middle(SegmentId(0))],
    });
    document.undo();

    let drawing = &document.sketches()[0];
    assert_eq!(drawing.points().len(), points, "the point went back");
    assert_eq!(
        drawing.constraints().len(),
        rules,
        "and its rule went back with it, in the same step",
    );
}

#[test]
fn compacting_the_history_carries_the_midpoint_rule() {
    let mut document = a_flat_trait();
    document.apply(Operation::AddPoint {
        sketch: 0,
        position: DVec2::new(30.0, 10.0),
        on: vec![Support::Middle(SegmentId(0))],
    });
    let held = |document: &PartDocument| {
        document.sketches()[0]
            .constraints()
            .iter()
            .filter(|rule| matches!(rule, cao_sketch::Constraint::Midpoint { .. }))
            .count()
    };
    let before = held(&document);

    document.compact_history();

    assert_eq!(before, 1, "one midpoint rule before compaction");
    assert_eq!(held(&document), before, "and the same one after it");
}
