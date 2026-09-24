//! A value written from the variables follows them while it stands on the
//! drawing, and keeps what it came to once it leaves — whichever way it
//! leaves.
//!
//! Closes #175.
//! - changing a variable changes every size written from it, and only those
//!   still written from it: a value that left the drawing keeps what it came
//!   to as it left, and one typed again and refused is still written from
//!   it — `a_value_whose_trait_is_erased_keeps_what_it_came_to`,
//!   `a_radius_a_cut_carried_then_erased_keeps_what_it_came_to`,
//!   `a_value_typed_again_and_refused_goes_on_following_its_variable`,
//!   `compacting_a_sketch_laid_again_as_drawn_keeps_an_erased_value_where_it_was`,
//!   `compacting_keeps_a_value_set_second_in_a_gesture_where_it_was_once_erased`;
//!   and a value gone is not held against a change it could not have taken —
//!   `an_erased_value_is_not_held_against_a_change_it_could_not_have_taken`,
//!   `a_value_gone_with_its_trait_is_not_held_against_a_change_it_could_not_have_taken`

use cao_part::history::{Operation, PointRef};
use cao_part::{Formula, PartDocument, VariableChange, VariableId};
use cao_sketch::{
    ArcId, CircleId, DimensionTarget, Element, PointId, SegmentId, Sketch, WorkPlane,
};
use glam::DVec2;

const TOLERANCE: f64 = 1e-6;

fn at_nine() -> chrono::DateTime<chrono::Utc> {
    "2026-01-02T09:00:00Z".parse().expect("a date")
}

fn written(document: &PartDocument, text: &str) -> Formula {
    document
        .variables()
        .read(text)
        .expect("a formula that reads")
}

fn added(name: &str, value: f64) -> VariableChange {
    VariableChange::Added {
        name: name.to_string(),
        formula: Formula::Number(value),
    }
}

fn edited(variable: usize, name: &str, value: f64) -> VariableChange {
    VariableChange::Edited {
        variable: VariableId(variable),
        name: name.to_string(),
        formula: Formula::Number(value),
    }
}

/// A part whose first variable is `a` at 50, with a sketch opened, a trait
/// 100 long along X giving it its scale, and another from the end of that
/// one up towards (100, 40).
fn a_corner() -> PartDocument {
    let mut document = PartDocument::new("Plate", at_nine());
    document
        .change_variable(added("a", 50.0))
        .expect("a length");
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::new(0.0, 0.0)),
        end: PointRef::New(DVec2::new(100.0, 0.0)),
        construction: false,
    });
    document.apply(Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Length(SegmentId(0)),
        value: 100.0.into(),
        placement: None,
    });
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::Existing(PointId(2)),
        end: PointRef::New(DVec2::new(100.0, 40.0)),
        construction: false,
    });
    document
}

const SIDE: DimensionTarget = DimensionTarget::Length(SegmentId(1));

#[test]
fn a_value_whose_trait_is_erased_keeps_what_it_came_to() {
    let mut document = a_corner();
    let from_a = written(&document, "a");
    document.apply(Operation::SetDimension {
        sketch: 0,
        target: SIDE,
        value: from_a,
        placement: None,
    });
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::Existing(PointId(3)),
        end: PointRef::New(DVec2::new(0.0, 40.0)),
        construction: false,
    });
    document.apply(Operation::EraseMany {
        sketch: 0,
        elements: vec![Element::Segment(SegmentId(1))],
        dimensions: Vec::new(),
        constraints: Vec::new(),
    });
    let corner = document.sketches()[0].point(PointId(3));

    document
        .change_variable(edited(0, "a", 80.0))
        .expect("nothing on the drawing is written from a any more");

    let now = document.sketches()[0].point(PointId(3));
    assert!(
        (now - corner).length() < TOLERANCE,
        "the corner the erased trait's value had set moved with a: {corner:?} -> {now:?}"
    );
}

/// The corner closed into a triangle 100, 90 and `a`, its third side set
/// from `a`.
fn a_triangle() -> PartDocument {
    let mut document = a_corner();
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::Existing(PointId(3)),
        end: PointRef::Existing(PointId(1)),
        construction: false,
    });
    document.apply(Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Length(SegmentId(2)),
        value: 90.0.into(),
        placement: None,
    });
    let from_a = written(&document, "a");
    document.apply(Operation::SetDimension {
        sketch: 0,
        target: SIDE,
        value: from_a,
        placement: None,
    });
    document
}

#[test]
fn an_erased_value_is_not_held_against_a_change_it_could_not_have_taken() {
    let mut document = a_triangle();
    document.apply(Operation::EraseMany {
        sketch: 0,
        elements: Vec::new(),
        dimensions: vec![SIDE],
        constraints: Vec::new(),
    });
    let corner = document.sketches()[0].point(PointId(3));

    document
        .change_variable(edited(0, "a", 500.0))
        .expect("no triangle 100, 90 and 500 exists, and none is asked for any more");

    let now = document.sketches()[0].point(PointId(3));
    assert!(
        (now - corner).length() < TOLERANCE,
        "the triangle kept the side it had when its value was erased: {corner:?} -> {now:?}"
    );
}

#[test]
fn a_value_gone_with_its_trait_is_not_held_against_a_change_it_could_not_have_taken() {
    let mut document = a_triangle();
    document.apply(Operation::EraseMany {
        sketch: 0,
        elements: vec![Element::Segment(SegmentId(1))],
        dimensions: Vec::new(),
        constraints: Vec::new(),
    });
    let corner = document.sketches()[0].point(PointId(3));

    document
        .change_variable(edited(0, "a", 500.0))
        .expect("the side measured by a is gone");

    let now = document.sketches()[0].point(PointId(3));
    assert!(
        (now - corner).length() < TOLERANCE,
        "the corner the erased side had set moved: {corner:?} -> {now:?}"
    );
}

#[test]
fn a_value_typed_again_and_refused_goes_on_following_its_variable() {
    let mut document = a_triangle();
    document.apply(Operation::SetDimension {
        sketch: 0,
        target: SIDE,
        value: 500.0.into(),
        placement: None,
    });

    document
        .change_variable(edited(0, "a", 60.0))
        .expect("a triangle 100, 90 and 60 holds");

    assert_eq!(document.formula_of(0, SIDE), Some("a".to_string()));
    let side = document.measured(0, SIDE).expect("the side is there");
    assert!(
        (side - 60.0).abs() < TOLERANCE,
        "a side that no triangle could give 500 still says a, and measures {side}"
    );
}

#[test]
fn a_radius_a_cut_carried_then_erased_keeps_what_it_came_to() {
    let mut document = PartDocument::new("Came", at_nine());
    document
        .change_variable(added("bore", 20.0))
        .expect("a bore");
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::new(30.0, 0.0)),
        end: PointRef::New(DVec2::new(60.0, 0.0)),
        construction: false,
    });
    document.apply(Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Length(SegmentId(0)),
        value: 30.0.into(),
        placement: None,
    });
    document.apply(Operation::AddCircle {
        sketch: 0,
        center: PointRef::Existing(Sketch::ORIGIN),
        radius: 10.0,
        rim: vec![
            PointRef::New(DVec2::new(10.0, 0.0)),
            PointRef::New(DVec2::new(0.0, 10.0)),
        ],
        construction: false,
    });
    let bore = written(&document, "bore");
    document.apply(Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Diameter(CircleId(0)),
        value: bore,
        placement: None,
    });
    document.apply(Operation::TrimCircle {
        sketch: 0,
        circle: CircleId(0),
        between: Some((PointId(3), PointId(4))),
    });
    let radius = DimensionTarget::ArcRadius(ArcId(0));
    assert_eq!(document.formula_of(0, radius), Some("bore / 2".to_string()));
    document.apply(Operation::EraseMany {
        sketch: 0,
        elements: Vec::new(),
        dimensions: vec![radius],
        constraints: Vec::new(),
    });

    document
        .change_variable(edited(0, "bore", 30.0))
        .expect("nothing on the drawing is written from bore any more");

    let now = document.sketches()[0].arc_radius(ArcId(0)) * document.scale();
    assert!(
        (now - 10.0).abs() < TOLERANCE,
        "the arc kept half the bore it had when its radius was erased: {now}"
    );
}

#[test]
fn compacting_a_sketch_laid_again_as_drawn_keeps_an_erased_value_where_it_was() {
    let mut document = a_corner();
    document
        .change_variable(added("copies", 3.0))
        .expect("a count");
    let from_a = written(&document, "a");
    document.apply(Operation::SetDimension {
        sketch: 0,
        target: SIDE,
        value: from_a,
        placement: None,
    });
    document.apply(Operation::EraseMany {
        sketch: 0,
        elements: Vec::new(),
        dimensions: vec![SIDE],
        constraints: Vec::new(),
    });
    document
        .change_variable(edited(0, "a", 70.0))
        .expect("nothing on the drawing is written from a any more");
    let count = written(&document, "copies");
    document.apply(Operation::CircularPattern {
        sketch: 0,
        elements: vec![Element::Segment(SegmentId(0))],
        centre: Sketch::ORIGIN,
        degrees: 90.0.into(),
        count,
    });
    let length = |document: &PartDocument| {
        document.sketches()[0].segment_length(SegmentId(1)) * document.scale()
    };

    document.compact_history();
    let compacted = length(&document);
    document
        .change_variable(edited(0, "a", 90.0))
        .expect("still nothing written from a");

    assert!(
        (compacted - 50.0).abs() < TOLERANCE && (length(&document) - 50.0).abs() < TOLERANCE,
        "the trait kept the 50 it had when its value was erased: {compacted}, then {}",
        length(&document)
    );
}

#[test]
fn compacting_keeps_a_value_set_second_in_a_gesture_where_it_was_once_erased() {
    let mut document = PartDocument::new("Plate", at_nine());
    document
        .change_variable(added("a", 50.0))
        .expect("a length");
    document
        .change_variable(added("copies", 3.0))
        .expect("a count");
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::new(0.0, 0.0)),
        end: PointRef::New(DVec2::new(100.0, 0.0)),
        construction: false,
    });
    document.apply(Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Length(SegmentId(0)),
        value: 100.0.into(),
        placement: None,
    });
    let from_a = written(&document, "a");
    document.apply(Operation::Gesture(vec![
        Operation::AddSegment {
            sketch: 0,
            start: PointRef::Existing(PointId(2)),
            end: PointRef::New(DVec2::new(100.0, 40.0)),
            construction: false,
        },
        Operation::SetDimension {
            sketch: 0,
            target: DimensionTarget::Length(SegmentId(0)),
            value: 100.0.into(),
            placement: None,
        },
        Operation::SetDimension {
            sketch: 0,
            target: SIDE,
            value: from_a,
            placement: None,
        },
    ]));
    document.apply(Operation::EraseMany {
        sketch: 0,
        elements: Vec::new(),
        dimensions: vec![SIDE],
        constraints: Vec::new(),
    });
    document
        .change_variable(edited(0, "a", 70.0))
        .expect("nothing on the drawing is written from a any more");
    let count = written(&document, "copies");
    document.apply(Operation::CircularPattern {
        sketch: 0,
        elements: vec![Element::Segment(SegmentId(0))],
        centre: Sketch::ORIGIN,
        degrees: 90.0.into(),
        count,
    });
    let length = |document: &PartDocument| {
        document.sketches()[0].segment_length(SegmentId(1)) * document.scale()
    };

    document.compact_history();
    let compacted = length(&document);
    document
        .change_variable(edited(0, "a", 90.0))
        .expect("still nothing written from a");

    assert!(
        (compacted - 50.0).abs() < TOLERANCE && (length(&document) - 50.0).abs() < TOLERANCE,
        "the side kept the 50 it had when its value, the second of its gesture, \
         was erased: {compacted}, then {}",
        length(&document)
    );
}
