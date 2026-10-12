use cao_sketch::{Constraint, DimensionTarget, InPlace, SegmentId, SketchAxis, WorkPlane};
use glam::DVec2;

use super::*;
use crate::history::PointRef;

/// Two traits from the origin, one along the horizontal and one at 60°, with
/// the angle between them laid at 60.
fn a_corner() -> (PartDocument, DimensionTarget) {
    let mut document = PartDocument::new("part", "2026-01-02T09:00:00Z".parse().expect("a date"));
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    for end in [
        DVec2::new(10.0, 0.0),
        DVec2::from_angle(std::f64::consts::FRAC_PI_3) * 10.0,
    ] {
        document.apply(Operation::AddSegment {
            sketch: 0,
            start: PointRef::Existing(cao_sketch::Sketch::ORIGIN),
            end: PointRef::New(end),
            construction: false,
        });
    }
    let target = DimensionTarget::corner(SegmentId(0), SegmentId(1));
    document.apply(Operation::SetDimension {
        sketch: 0,
        target,
        value: 60.0.into(),
        placement: None,
    });
    (document, target)
}

fn square() -> InPlace {
    InPlace {
        rule: Constraint::Perpendicular {
            first: SegmentId(0),
            second: SegmentId(1),
        },
        erased: Vec::new(),
    }
}

#[test]
fn the_rule_takes_the_dimensions_place_in_one_step_and_one_undo_gives_it_back() {
    let (mut document, target) = a_corner();
    let steps = document.history.applied();

    assert!(document.lay_in_place_of(0, target, square()));

    let sketch = &document.sketches()[0];
    assert!(sketch.dimension_of(target).is_none(), "the dimension goes");
    assert!(sketch.constraints().contains(&square().rule));
    assert_eq!(document.history.applied(), steps + 1, "one step");

    document.undo();
    let sketch = &document.sketches()[0];
    assert!(
        sketch.constraints().is_empty(),
        "{:?}",
        sketch.constraints()
    );
    assert!(
        sketch
            .dimension_of(target)
            .is_some_and(|it| (it.value - 60.0).abs() < 1e-9),
        "the dimension comes back as it was",
    );
}

#[test]
fn a_rule_that_cannot_hold_is_refused_and_the_dimension_stays() {
    let (mut document, _) = a_corner();
    document.undo();
    for constraint in [
        Constraint::AxisParallel {
            segment: SegmentId(0),
            axis: SketchAxis::U,
        },
        Constraint::AxisParallel {
            segment: SegmentId(1),
            axis: SketchAxis::V,
        },
    ] {
        document.apply(Operation::Constrain {
            sketch: 0,
            constraint,
        });
    }
    let target = DimensionTarget::Length(SegmentId(1));
    document.apply(Operation::SetDimension {
        sketch: 0,
        target,
        value: 10.0.into(),
        placement: None,
    });
    let steps = document.history.applied();
    let parallel = InPlace {
        rule: Constraint::Parallel {
            first: SegmentId(0),
            second: SegmentId(1),
        },
        erased: Vec::new(),
    };

    assert!(!document.lay_in_place_of(0, target, parallel));

    assert_eq!(document.history.applied(), steps, "nothing is recorded");
    assert!(!document.history.can_redo(), "not even an erase taken back");
    assert!(document.sketches()[0].dimension_of(target).is_some());
}
