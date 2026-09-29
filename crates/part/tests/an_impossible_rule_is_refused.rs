//! A rule the drawing cannot hold is refused, as a value is, rather than kept
//! untrue.
//!
//! Closes #467.
//! - a rule contradicting a value is refused: the drawing is as it was, and the
//!   history has no new step —
//!   `a_rule_contradicting_a_value_is_refused_and_leaves_no_step`
//! - the message reads « Règle impossible : elle contredit le dessin » — no
//!   test: the sentence is the interface's, held beside it in
//!   `crates/app/src/wording/outcome/tests.rs`
//! - a rule only fixed points stand in the way of still lands —
//!   `a_rule_only_fixed_points_stand_in_the_way_of_still_lands`
//! - a rule the drawing can take still lands, and is recorded —
//!   `a_rule_the_drawing_can_take_lands_and_is_recorded`

use cao_part::{Operation, Outcome, PartDocument, PointRef};
use cao_sketch::{Constraint, DimensionTarget, Element, SegmentId, SketchAxis, WorkPlane};
use glam::DVec2;

/// Two traits apart, 50 and 80 long, both typed.
fn two_typed_traits() -> PartDocument {
    let mut document = PartDocument::new("part", chrono::Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    for (start, end) in [
        (DVec2::new(10.0, 10.0), DVec2::new(60.0, 10.0)),
        (DVec2::new(10.0, 40.0), DVec2::new(90.0, 40.0)),
    ] {
        document.apply(Operation::AddSegment {
            sketch: 0,
            start: PointRef::New(start),
            end: PointRef::New(end),
            construction: false,
        });
    }
    for (side, value) in [(SegmentId(0), 50.0), (SegmentId(1), 80.0)] {
        document.apply(Operation::SetDimension {
            sketch: 0,
            target: DimensionTarget::Length(side),
            value: value.into(),
            placement: None,
        });
    }
    document
}

fn steps(document: &PartDocument) -> usize {
    document.history.applied_operations().len()
}

#[test]
fn a_rule_contradicting_a_value_is_refused_and_leaves_no_step() {
    let mut document = two_typed_traits();
    let before = document.sketches()[0].clone();
    let recorded = steps(&document);
    let equal = Constraint::Equal {
        first: SegmentId(0),
        second: SegmentId(1),
    };

    let outcome = document.apply(Operation::Constrain {
        sketch: 0,
        constraint: equal,
    });

    assert_eq!(outcome, Some(Outcome::RuleRefused));
    let sketch = &document.sketches()[0];
    assert!(!sketch.carries(equal), "the rule is not kept");
    assert!(
        sketch
            .points()
            .iter()
            .zip(before.points())
            .all(|(now, was)| now.distance(*was) < 1e-9),
        "nothing about the drawing moved"
    );
    assert_eq!(steps(&document), recorded, "the history has no new step");
}

#[test]
fn a_rule_only_fixed_points_stand_in_the_way_of_still_lands() {
    let mut document = two_typed_traits();
    document.apply(Operation::Constrain {
        sketch: 0,
        constraint: Constraint::Fixed {
            element: Element::Segment(SegmentId(0)),
        },
    });
    let along = Constraint::AxisCollinear {
        segment: SegmentId(0),
        axis: SketchAxis::U,
    };

    let outcome = document.apply(Operation::Constrain {
        sketch: 0,
        constraint: along,
    });

    assert_eq!(outcome, None, "nothing to say: the rule holds");
    let sketch = &document.sketches()[0];
    assert!(sketch.carries(along));
    let (start, end) = sketch.endpoints(SegmentId(0));
    assert!(
        start.y.abs() < 1e-3 && end.y.abs() < 1e-3,
        "the fixed trait came onto the axis: {start} to {end}"
    );
}

#[test]
fn a_rule_the_drawing_can_take_lands_and_is_recorded() {
    let mut document = two_typed_traits();
    let recorded = steps(&document);
    let parallel = Constraint::Parallel {
        first: SegmentId(0),
        second: SegmentId(1),
    };

    let outcome = document.apply(Operation::Constrain {
        sketch: 0,
        constraint: parallel,
    });

    assert_eq!(outcome, None);
    assert!(document.sketches()[0].carries(parallel));
    assert_eq!(steps(&document), recorded + 1, "the rule is a step");
}
