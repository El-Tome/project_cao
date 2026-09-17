use cao_part::{Operation, PartDocument, PointRef};
use cao_sketch::{PointId, WorkPlane};
use chrono::{DateTime, Utc};
use glam::DVec2;

fn at(text: &str) -> DateTime<Utc> {
    text.parse().expect("a date")
}

/// Two traits standing apart, four corners between them.
fn two_traits() -> PartDocument {
    let mut document = PartDocument::new("Test", at("2026-01-02T09:00:00Z"));
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
    });
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::ZERO),
        end: PointRef::New(DVec2::new(10.0, 0.0)),
        construction: false,
    });
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::new(0.0, 10.0)),
        end: PointRef::New(DVec2::new(10.0, 10.0)),
        construction: false,
    });
    document
}

fn corners(document: &PartDocument) -> Vec<(PointId, DVec2)> {
    document.sketches()[0].live_points().collect()
}

#[test]
fn a_corner_dropped_on_another_becomes_one_corner_with_it() {
    let mut document = two_traits();
    let before = corners(&document).len();

    document.apply(Operation::MovePoint {
        sketch: 0,
        point: PointId(3),
        position: DVec2::ZERO,
        merged_into: Some(PointId(1)),
    });

    assert_eq!(
        corners(&document).len(),
        before - 1,
        "two ends laid on top of each other are one corner, not two",
    );
}

#[test]
fn one_undo_takes_back_both_the_drop_and_the_joining() {
    let mut document = two_traits();
    let before = corners(&document);

    document.apply(Operation::MovePoint {
        sketch: 0,
        point: PointId(3),
        position: DVec2::ZERO,
        merged_into: Some(PointId(1)),
    });
    document.undo();

    assert_eq!(
        corners(&document),
        before,
        "dropping a corner on another is one gesture of the user, so one undo \
         puts the drawing back the way it was before the drag",
    );
}

#[test]
fn a_drop_that_joins_nothing_moves_the_corner_and_no_more() {
    let mut document = two_traits();
    let before = corners(&document).len();

    document.apply(Operation::MovePoint {
        sketch: 0,
        point: PointId(3),
        position: DVec2::new(4.0, 7.0),
        merged_into: None,
    });

    let after = corners(&document);
    assert_eq!(after.len(), before);
    assert_eq!(
        after
            .iter()
            .find(|(id, _)| *id == PointId(3))
            .expect("the corner that was dragged")
            .1,
        DVec2::new(4.0, 7.0),
    );
}
