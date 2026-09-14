//! A typed value that a dimension already fixed rules out: refused rather
//! than bent to whatever the solver could reach.

use cao_part::{DimensionOutcome, Operation, Outcome, PartState, PointRef};
use cao_sketch::{ArcId, DimensionTarget, LengthOutcome, Sketch, WorkPlane};
use glam::DVec2;

#[test]
fn a_radius_a_fixed_chord_rules_out_is_refused_rather_than_bent_to_fit() {
    let mut state = PartState::default();
    state.apply(&Operation::CreateSketch {
        plane: WorkPlane::XY,
    });
    state.apply(&Operation::AddArc {
        sketch: 0,
        center: PointRef::New(DVec2::new(40.0, 30.0)),
        start: PointRef::Existing(Sketch::ORIGIN),
        end: PointRef::New(DVec2::new(80.0, 0.0)),
        construction: false,
    });
    state.apply(&Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Distance {
            from: Sketch::ORIGIN,
            to: cao_sketch::PointId(2),
        },
        value: 80.0,
        placement: None,
    });
    let before = state.sketches[0].clone();

    // No arc through both ends of an 80 mm chord can be smaller than a
    // 40 mm semicircle, so 5 mm cannot be honoured.
    let outcome = state.apply(&Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::ArcRadius(ArcId(0)),
        value: 5.0,
        placement: None,
    });

    assert_eq!(
        outcome,
        Some(Outcome::Dimension(DimensionOutcome::Geometry(
            LengthOutcome::BestEffort
        )))
    );
    assert!(
        state.sketches[0]
            .dimension_of(DimensionTarget::ArcRadius(ArcId(0)))
            .is_none(),
        "the refused value is not kept"
    );
    assert!(
        state.sketches[0]
            .points()
            .iter()
            .zip(before.points())
            .all(|(now, was)| now.distance(*was) < 1e-9),
        "nothing about the drawing moved"
    );
}
