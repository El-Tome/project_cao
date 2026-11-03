//! A point dragged and dropped at the middle of a trait carries the midpoint
//! rule there, replayed from the step the drop wrote.
//!
//! Closes #615.
//! - a point dropped on the middle of a trait follows the middle when the trait
//!   changes length — `a_point_dropped_at_the_middle_follows_it_when_the_trait_stretches`
//! - a point held elsewhere keeps its other holds —
//!   `a_point_held_elsewhere_keeps_its_hold_when_dropped_at_a_middle`
//! - the midpoint rule replaces the hold on the trait —
//!   `a_point_slid_to_the_middle_of_its_trait_trades_its_hold_for_the_middle`
//! - the drop and its rule are one step of the history —
//!   `one_undo_takes_back_the_drop_and_its_midpoint_rule`

use cao_part::{Operation, PartDocument, PointRef};
use cao_sketch::{Constraint, PointId, SegmentId, Support, WorkPlane};
use chrono::{DateTime, Utc};
use glam::DVec2;

fn at(text: &str) -> DateTime<Utc> {
    text.parse().expect("a date")
}

const THE_MIDDLE: DVec2 = DVec2::new(30.0, 10.0);

/// One trait lying flat, from (10, 10) to (50, 10), and a point standing at
/// `point`, held by `on`.
fn a_flat_trait_and_a_point(point: DVec2, on: Vec<Support>) -> (PartDocument, PointId) {
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
    document.apply(Operation::AddPoint {
        sketch: 0,
        position: point,
        on,
    });
    let point = PointId(document.sketches()[0].points().len() - 1);
    (document, point)
}

fn rules_on(document: &PartDocument, point: PointId) -> Vec<Constraint> {
    document.sketches()[0]
        .constraints()
        .iter()
        .copied()
        .filter(|rule| match *rule {
            Constraint::Midpoint { point: held, .. } => held == point,
            rule => Support::held_by(rule).is_some_and(|(held, _)| held == point),
        })
        .collect()
}

fn drop_at_the_middle(document: &mut PartDocument, point: PointId) {
    document.apply(Operation::MovePoint {
        sketch: 0,
        point,
        position: THE_MIDDLE,
        merged_into: None,
        on: vec![Support::Middle(SegmentId(0))],
        let_go: false,
    });
}

#[test]
fn a_point_dropped_at_the_middle_follows_it_when_the_trait_stretches() {
    let (mut document, point) = a_flat_trait_and_a_point(DVec2::new(30.0, 40.0), Vec::new());
    drop_at_the_middle(&mut document, point);

    document.apply(Operation::MovePoint {
        sketch: 0,
        point: PointId(1),
        position: DVec2::new(90.0, 10.0),
        merged_into: None,
        on: Vec::new(),
        let_go: false,
    });

    let drawing = &document.sketches()[0];
    let (start, end) = drawing.endpoints(SegmentId(0));
    let middle = (start + end) * 0.5;
    assert!(
        drawing.point(point).distance(middle) < 1e-6,
        "the trait was stretched and its middle moved to {middle}, the point stayed at {}",
        drawing.point(point)
    );
}

#[test]
fn a_point_slid_to_the_middle_of_its_trait_trades_its_hold_for_the_middle() {
    let (mut document, point) =
        a_flat_trait_and_a_point(DVec2::new(20.0, 10.0), vec![Support::Segment(SegmentId(0))]);

    drop_at_the_middle(&mut document, point);

    assert_eq!(
        rules_on(&document, point),
        vec![Support::Middle(SegmentId(0)).holding(point)]
    );
}

#[test]
fn a_point_held_elsewhere_keeps_its_hold_when_dropped_at_a_middle() {
    let mut document = PartDocument::new("Test", at("2026-01-02T09:00:00Z"));
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    for (start, end) in [((10.0, 10.0), (50.0, 10.0)), ((30.0, 0.0), (30.0, 60.0))] {
        document.apply(Operation::AddSegment {
            sketch: 0,
            start: PointRef::New(DVec2::from(start)),
            end: PointRef::New(DVec2::from(end)),
            construction: false,
        });
    }
    document.apply(Operation::AddPoint {
        sketch: 0,
        position: DVec2::new(30.0, 50.0),
        on: vec![Support::Segment(SegmentId(1))],
    });
    let point = PointId(document.sketches()[0].points().len() - 1);

    drop_at_the_middle(&mut document, point);

    assert_eq!(
        rules_on(&document, point),
        vec![
            Support::Segment(SegmentId(1)).holding(point),
            Support::Middle(SegmentId(0)).holding(point)
        ]
    );
}

#[test]
fn one_undo_takes_back_the_drop_and_its_midpoint_rule() {
    let (mut document, point) = a_flat_trait_and_a_point(DVec2::new(30.0, 40.0), Vec::new());
    let rules = document.sketches()[0].constraints().len();

    drop_at_the_middle(&mut document, point);
    document.undo();

    let drawing = &document.sketches()[0];
    assert_eq!(drawing.constraints().len(), rules, "the rule went back");
    assert!(
        drawing.point(point).distance(DVec2::new(30.0, 40.0)) < 1e-9,
        "and the point with it, in the same step"
    );
}
