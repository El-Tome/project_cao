//! An ellipse, laid with its two axes, kept through everything a history does.
//!
//! Closes #412.
//! - an ellipse is one step in the history, undone in one go —
//!   `one_step_lays_the_ellipse_its_two_axes_and_its_five_points`,
//!   `one_undo_takes_back_the_whole_ellipse`
//! - compaction gives the same ellipse back, with what holds and measures it —
//!   `compacting_keeps_the_ellipse_what_holds_it_and_what_measures_it`
//! - saved and read back — `the_history_reads_back_the_ellipse_it_wrote`,
//!   `a_drawing_read_back_holds_its_ellipse`

use cao_part::{Operation, PartDocument, PointRef};
use cao_sketch::{Constraint, DimensionTarget, EllipseId, Support, WorkPlane};
use chrono::{DateTime, Utc};
use glam::DVec2;

fn at(text: &str) -> DateTime<Utc> {
    text.parse().expect("a date")
}

fn an_ellipse() -> Operation {
    Operation::AddEllipse {
        sketch: 0,
        center: PointRef::New(DVec2::new(50.0, 20.0)),
        first: [
            PointRef::New(DVec2::new(20.0, 20.0)),
            PointRef::New(DVec2::new(80.0, 20.0)),
        ],
        second: [
            PointRef::New(DVec2::new(50.0, 10.0)),
            PointRef::New(DVec2::new(50.0, 30.0)),
        ],
        construction: false,
    }
}

/// A sketch holding one ellipse, 60 wide and 20 high, about (50, 20).
fn a_document_with_an_ellipse() -> PartDocument {
    let mut document = PartDocument::new("Test", at("2026-09-22T09:00:00Z"));
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(an_ellipse());
    document
}

#[test]
fn one_step_lays_the_ellipse_its_two_axes_and_its_five_points() {
    let document = a_document_with_an_ellipse();
    let drawing = &document.sketches()[0];

    assert_eq!(drawing.live_ellipses().count(), 1);
    let construction: Vec<bool> = drawing
        .live_segments()
        .map(|(_, segment)| segment.construction)
        .collect();
    assert_eq!(construction, [true, true], "the two axes, as construction");
    assert_eq!(drawing.live_points().count(), 6, "the origin and five more");
}

#[test]
fn one_undo_takes_back_the_whole_ellipse() {
    let mut document = a_document_with_an_ellipse();

    document.undo();

    let drawing = &document.sketches()[0];
    assert_eq!(drawing.live_ellipses().count(), 0);
    assert_eq!(drawing.live_segments().count(), 0);
    assert_eq!(drawing.live_points().count(), 1, "the origin alone");
}

#[test]
fn compacting_keeps_the_ellipse_what_holds_it_and_what_measures_it() {
    let mut document = a_document_with_an_ellipse();
    // Drawn before the ellipse, so that compaction, which lays plain traits
    // first, hands the axes numbers other than the ones they had.
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::new(0.0, 0.0)),
        end: PointRef::New(DVec2::new(0.0, 60.0)),
        construction: false,
    });
    let first = document.sketches()[0].ellipses()[0].first;
    document.apply(Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Length(first),
        value: 80.0,
        placement: None,
    });
    let on_the_curve = document.sketches()[0].ellipse_draft(EllipseId(0)).at(0.9);
    document.apply(Operation::AddPoint {
        sketch: 0,
        position: on_the_curve,
        on: vec![Support::Ellipse(EllipseId(0))],
    });
    let before = document.sketches()[0].ellipse_draft(EllipseId(0));

    document.compact_history();

    let drawing = &document.sketches()[0];
    let after = drawing.ellipse_draft(EllipseId(0));
    assert!(
        after.centre.distance(before.centre) < 1e-6 && after.first.distance(before.first) < 1e-6,
        "{after:?} / {before:?}",
    );
    assert!((after.second - before.second).abs() < 1e-6);
    let axis = drawing.ellipses()[0].first;
    assert!(
        drawing
            .dimension_of(DimensionTarget::Length(axis))
            .is_some(),
        "the length still stands on the axis the ellipse was laid with, now {axis:?}",
    );
    assert_eq!(drawing.live_segments().count(), 3, "no axis laid twice");
    let held = drawing
        .constraints()
        .iter()
        .filter(|rule| matches!(rule, Constraint::OnEllipse { .. }))
        .count();
    assert_eq!(held, 1, "the point held on the curve is still held on it");
    assert!(
        drawing
            .live_points()
            .any(|(_, place)| place.distance(on_the_curve) < 1e-6),
        "and it is still where it was laid",
    );
}

#[test]
fn the_history_reads_back_the_ellipse_it_wrote() {
    let written = serde_json::to_string(&an_ellipse()).expect("written");

    let read: Operation = serde_json::from_str(&written).expect("read back");

    assert_eq!(
        serde_json::to_string(&read).expect("written again"),
        written
    );
}

#[test]
fn a_drawing_read_back_holds_its_ellipse() {
    let document = a_document_with_an_ellipse();
    let drawing = &document.sketches()[0];

    let written = serde_json::to_string(drawing).expect("written");
    let read: cao_sketch::Sketch = serde_json::from_str(&written).expect("read back");

    assert_eq!(read.live_ellipses().count(), 1);
    assert_eq!(
        read.ellipse_draft(EllipseId(0)),
        drawing.ellipse_draft(EllipseId(0))
    );
}
