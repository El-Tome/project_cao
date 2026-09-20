//! A point laid on a trait, an arc, a circle or an axis is held there, and
//! stays held through a replay, an undo and a compaction.
//!
//! Closes #342.
//! - a point born on a trait, an arc, a circle or an axis is held on it —
//!   `a_point_laid_on_a_trait_follows_it_when_the_trait_moves`
//! - whatever the tool that laid it down —
//!   `a_line_started_on_a_circle_keeps_its_end_on_the_rim`
//! - a point born where two of them cross is held on both —
//!   `a_point_laid_where_a_trait_and_a_circle_cross_is_held_by_both`
//! - and cannot move on its own, following the crossing instead — no test: it
//!   is `Sketch::slide`, held beside the rule by
//!   `a_point_held_at_a_crossing_stays_there_however_it_is_pulled` and by
//!   `a_point_held_where_two_traits_cross_follows_it_when_one_of_them_moves`
//! - a point dropped there by a drag is held the same way —
//!   `a_point_dropped_on_a_trait_by_a_drag_is_held_there`
//! - dragging with the let-go key pulls the point off, and dropping it with
//!   the key still down does not hold it again —
//!   `a_point_dragged_with_the_let_go_key_comes_off_what_held_it`
//! - undo takes back the point and the rule holding it in one step —
//!   `one_undo_takes_back_both_the_point_and_the_rule_that_holds_it`
//! - compacting the history carries the new rules along —
//!   `compacting_the_history_carries_what_holds_a_point`
//! - a held point dragged slides along what holds it and cannot leave it — no
//!   test: it is `Sketch::slide`, held beside the rule it reads by
//!   `a_point_held_on_a_trait_slides_along_it_when_it_is_pulled_off`
//! - what holds a point moves or resizes, and the point follows without being
//!   deformed by it — no test: it is the solver's, held by
//!   `a_trait_dragged_carries_the_point_it_holds_without_being_bent_by_it`
//! - the rule shows as a mark on the point and is deleted from there like any
//!   other — no test: `rule_marks` answers for the two new rules, and what
//!   draws the mark is the canvas, which has no net (docs/code-map.md)
//! - merging with an existing end point still works — no test: `dropping_a_point`
//!   holds it, and a drop that joins another point records no hold
//! - the trim tool still carries what it carried, arcs included —
//!   `what_a_cut_keeps` and `what_a_cut_leaves_of_an_arc` hold it — no test:
//!   they are `cao_sketch`'s, beside the cut they read
//! - the let-go key is one the view is not turned with — no test: it is
//!   `the_key_that_pulls_a_point_off_is_not_one_the_view_is_turned_with`, in
//!   `cao_prefs`, where the navigation presets are

use cao_part::{Operation, PartDocument, PointRef};
use cao_sketch::{CircleId, DimensionTarget, PointId, SegmentId, Support, WorkPlane};
use chrono::{DateTime, Utc};
use glam::DVec2;

fn at(text: &str) -> DateTime<Utc> {
    text.parse().expect("a date")
}

/// A trait lying flat, and a circle crossing it.
fn a_trait_and_a_circle() -> PartDocument {
    let mut document = PartDocument::new("Test", at("2026-01-02T09:00:00Z"));
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::new(0.0, 20.0)),
        end: PointRef::New(DVec2::new(40.0, 20.0)),
        construction: false,
    });
    document.apply(Operation::AddCircle {
        sketch: 0,
        center: PointRef::New(DVec2::new(10.0, 20.0)),
        radius: 5.0,
        rim: Vec::new(),
        construction: false,
    });
    document
}

#[test]
fn a_point_laid_on_a_trait_follows_it_when_the_trait_moves() {
    let mut document = a_trait_and_a_circle();
    document.apply(Operation::AddPoint {
        sketch: 0,
        position: DVec2::new(30.0, 20.0),
        on: vec![Support::Segment(SegmentId(0))],
    });
    let point = PointId(document.sketches()[0].points().len() - 1);

    document.apply(Operation::MoveMany {
        sketch: 0,
        points: vec![PointId(1), PointId(2)],
        by: DVec2::new(0.0, 10.0),
    });

    let drawing = &document.sketches()[0];
    let (start, end) = drawing.endpoints(SegmentId(0));
    let place = drawing.point(point);
    let off = (end - start).perp_dot(place - start).abs() / (end - start).length();
    assert!(
        off < 1e-6,
        "the trait was moved to {start}..{end} and left the point at {place}",
    );
}

#[test]
fn a_point_laid_where_a_trait_and_a_circle_cross_is_held_by_both() {
    let mut document = a_trait_and_a_circle();
    document.apply(Operation::AddPoint {
        sketch: 0,
        position: DVec2::new(15.0, 20.0),
        on: vec![Support::Segment(SegmentId(0)), Support::Circle(CircleId(0))],
    });
    let point = PointId(document.sketches()[0].points().len() - 1);

    let holds = document.sketches()[0].holds_on(point);

    assert_eq!(
        holds,
        vec![Support::Segment(SegmentId(0)), Support::Circle(CircleId(0)),],
    );
}

#[test]
fn a_line_started_on_a_circle_keeps_its_end_on_the_rim() {
    let mut document = a_trait_and_a_circle();
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::Held {
            at: DVec2::new(15.0, 20.0),
            on: vec![Support::Circle(CircleId(0))],
        },
        end: PointRef::New(DVec2::new(30.0, 35.0)),
        construction: false,
    });
    let on_rim = PointId(document.sketches()[0].points().len() - 2);

    document.apply(Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Diameter(CircleId(0)),
        value: 16.0,
        placement: None,
    });

    let drawing = &document.sketches()[0];
    let circle = drawing.circle(CircleId(0));
    let reach = drawing.point(on_rim).distance(drawing.point(circle.center));
    assert!(
        (reach - circle.radius).abs() < 1e-6,
        "the circle was resized to {} and the end of the line stands {reach} out",
        circle.radius,
    );
}

#[test]
fn a_point_dropped_on_a_trait_by_a_drag_is_held_there() {
    let mut document = a_trait_and_a_circle();
    document.apply(Operation::AddPoint {
        sketch: 0,
        position: DVec2::new(30.0, 35.0),
        on: Vec::new(),
    });
    let point = PointId(document.sketches()[0].points().len() - 1);

    document.apply(Operation::MovePoint {
        sketch: 0,
        point,
        position: DVec2::new(30.0, 20.0),
        merged_into: None,
        on: vec![Support::Segment(SegmentId(0))],
        let_go: false,
    });

    assert_eq!(
        document.sketches()[0].holds_on(point),
        vec![Support::Segment(SegmentId(0))],
        "a point dropped on a trait is held there, as one born on it is",
    );
}

#[test]
fn a_point_dragged_with_the_let_go_key_comes_off_what_held_it() {
    let mut document = a_trait_and_a_circle();
    document.apply(Operation::AddPoint {
        sketch: 0,
        position: DVec2::new(30.0, 20.0),
        on: vec![Support::Segment(SegmentId(0))],
    });
    let point = PointId(document.sketches()[0].points().len() - 1);

    document.apply(Operation::MovePoint {
        sketch: 0,
        point,
        position: DVec2::new(30.0, 35.0),
        merged_into: None,
        on: Vec::new(),
        let_go: true,
    });

    assert!(
        document.sketches()[0].holds_on(point).is_empty(),
        "the key pulled it off the trait",
    );
    let place = document.sketches()[0].point(point);
    assert!(
        place.distance(DVec2::new(30.0, 35.0)) < 1e-9,
        "and it stayed where it was dropped, at {place}",
    );
}

#[test]
fn one_undo_takes_back_both_the_point_and_the_rule_that_holds_it() {
    let mut document = a_trait_and_a_circle();
    let before = document.sketches()[0].constraints().len();

    document.apply(Operation::AddPoint {
        sketch: 0,
        position: DVec2::new(30.0, 20.0),
        on: vec![Support::Segment(SegmentId(0))],
    });
    document.undo();

    assert_eq!(
        document.sketches()[0].constraints().len(),
        before,
        "the rule went back with the point, in one step",
    );
}

#[test]
fn compacting_the_history_carries_what_holds_a_point() {
    let mut document = a_trait_and_a_circle();
    document.apply(Operation::AddPoint {
        sketch: 0,
        position: DVec2::new(30.0, 20.0),
        on: vec![Support::Segment(SegmentId(0))],
    });
    let held = |document: &PartDocument| {
        let drawing = &document.sketches()[0];
        drawing
            .live_points()
            .filter(|(id, _)| !drawing.holds_on(*id).is_empty())
            .count()
    };
    let before = held(&document);

    document.compact_history();

    assert_eq!(before, 1, "one point is held before compaction");
    assert_eq!(held(&document), before, "and the same one after it");
}
