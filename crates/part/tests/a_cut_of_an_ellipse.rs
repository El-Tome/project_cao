//! What a cut of an ellipse costs the history.
//!
//! Closes #414.
//! - a cut is one step of the history, undone in one go —
//!   `one_undo_gives_the_whole_ellipse_back`
//! - the arc of ellipse survives compaction, with the stretch it was left —
//!   `compacting_keeps_the_stretch_the_cut_left`
//! - and is read back from the file as the arc it is —
//!   `a_drawing_read_back_holds_the_stretch_of_its_ellipse`
//! - a cut in the middle of a stretch leaves two pieces, replayed as two —
//!   `a_cut_in_the_middle_is_replayed_as_the_two_pieces_it_left`
//! - what a cut leaves, and what every tool makes of it, is the drawing's —
//!   no test: it is held beside the geometry, in the sketch's own
//!   what_a_cut_leaves_of_an_ellipse

use cao_part::{Operation, PartDocument, PointRef};
use cao_sketch::{EllipseId, PointId, WorkPlane};
use chrono::{DateTime, Utc};
use glam::DVec2;

fn at(text: &str) -> DateTime<Utc> {
    text.parse().expect("a date")
}

/// A part holding one ellipse about the origin, sixty wide and forty high,
/// with the four points its axes give it.
fn a_document_with_an_ellipse() -> PartDocument {
    let mut document = PartDocument::new("Test", at("2026-09-23T09:00:00Z"));
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddEllipse {
        sketch: 0,
        center: PointRef::New(DVec2::ZERO),
        first: [
            PointRef::New(DVec2::new(-30.0, 0.0)),
            PointRef::New(DVec2::new(30.0, 0.0)),
        ],
        second: [
            PointRef::New(DVec2::new(0.0, -20.0)),
            PointRef::New(DVec2::new(0.0, 20.0)),
        ],
        construction: false,
        drawn: None,
    });
    document
}

/// The points of that fixture by the rank the drawing gives them: the origin
/// is nought and the centre one, so the four the axes give it follow.
const WEST: PointId = PointId(2);
const EAST: PointId = PointId(3);
const SOUTH: PointId = PointId(4);
const NORTH: PointId = PointId(5);

fn how_far_it_runs(document: &PartDocument, id: EllipseId) -> f64 {
    document.sketches()[0]
        .ellipse_polyline(id)
        .windows(2)
        .map(|pair| pair[0].distance(pair[1]))
        .sum()
}

#[test]
fn one_undo_gives_the_whole_ellipse_back() {
    let mut document = a_document_with_an_ellipse();
    let whole = how_far_it_runs(&document, EllipseId(0));
    document.apply(Operation::TrimEllipse {
        sketch: 0,
        ellipse: EllipseId(0),
        between: Some((EAST, NORTH)),
    });
    let cut = how_far_it_runs(&document, EllipseId(0));
    assert!(cut < whole * 0.8, "a quarter went: {cut} of {whole}");

    document.undo();

    let back = how_far_it_runs(&document, EllipseId(0));
    assert!((back - whole).abs() < 1e-6, "{back} of {whole}");
    assert_eq!(document.sketches()[0].ellipse_ends(EllipseId(0)), None);
}

#[test]
fn compacting_keeps_the_stretch_the_cut_left() {
    let mut document = a_document_with_an_ellipse();
    document.apply(Operation::TrimEllipse {
        sketch: 0,
        ellipse: EllipseId(0),
        between: Some((EAST, NORTH)),
    });
    let before = how_far_it_runs(&document, EllipseId(0));

    document.compact_history();

    let after = how_far_it_runs(&document, EllipseId(0));
    assert!((after - before).abs() < 1e-6, "{after} of {before}");
    let drawing = &document.sketches()[0];
    assert!(
        drawing.ellipse_ends(EllipseId(0)).is_some(),
        "it comes back an arc of ellipse, not the whole curve",
    );
    assert_eq!(
        drawing.live_segments().count(),
        2,
        "and its axes, laid once"
    );
}

#[test]
fn a_drawing_read_back_holds_the_stretch_of_its_ellipse() {
    let mut document = a_document_with_an_ellipse();
    document.apply(Operation::TrimEllipse {
        sketch: 0,
        ellipse: EllipseId(0),
        between: Some((EAST, NORTH)),
    });
    let drawing = &document.sketches()[0];

    let written = serde_json::to_string(drawing).expect("written");
    let read: cao_sketch::Sketch = serde_json::from_str(&written).expect("read back");

    assert_eq!(
        read.ellipse_ends(EllipseId(0)),
        drawing.ellipse_ends(EllipseId(0)),
    );
}

#[test]
fn a_cut_in_the_middle_is_replayed_as_the_two_pieces_it_left() {
    let mut document = a_document_with_an_ellipse();
    document.apply(Operation::TrimEllipse {
        sketch: 0,
        ellipse: EllipseId(0),
        between: Some((EAST, NORTH)),
    });

    document.apply(Operation::TrimEllipse {
        sketch: 0,
        ellipse: EllipseId(0),
        between: Some((WEST, SOUTH)),
    });

    let drawing = &document.sketches()[0];
    assert_eq!(
        drawing.live_ellipses().count(),
        2,
        "two pieces of one curve"
    );
    assert_eq!(
        drawing.ellipse_ends(EllipseId(0)),
        Some((NORTH, WEST)),
        "the ellipse itself is the first of them",
    );
    assert_eq!(drawing.ellipse_ends(EllipseId(1)), Some((SOUTH, EAST)));
    assert_eq!(
        drawing.live_segments().count(),
        2,
        "both stand on the one pair of axes",
    );
}
