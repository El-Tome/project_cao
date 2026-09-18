// `architecture.rs`'s reader-text scan slices each file at its own first
// "#[cfg(test)]" literal to tell shipped prose from test code. This file is
// nothing but test code, reached only through `#[cfg(test)] mod tests;` in
// `compaction.rs`, but carries no such literal of its own — so an unrelated
// import wears it here to keep the scan from reading this file's own
// assertion messages as sentences owed to a reader.
use cao_sketch::DimensionTarget;
#[cfg(test)]
use cao_sketch::WorkPlane;
use glam::DVec2;

use super::*;
use crate::history::Step;

#[test]
fn a_history_with_nothing_undone_compacts_to_itself_in_shape() {
    let mut history = History::default();
    history.push(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    history.push(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::ZERO),
        end: PointRef::New(DVec2::new(10.0, 0.0)),
        construction: false,
    });

    let compacted = compact(&history);
    let before = PartState::rebuild(&history);
    let after = PartState::rebuild(&compacted);
    assert_eq!(
        before.sketches[0].live_points().count(),
        after.sketches[0].live_points().count()
    );
    assert_eq!(
        before.sketches[0].live_segments().count(),
        after.sketches[0].live_segments().count()
    );
}

fn volume(mesh: &cao_solid::Mesh) -> f64 {
    mesh.triangles()
        .iter()
        .map(|[a, b, c]| a.dot(b.cross(*c)) / 6.0)
        .sum()
}

fn dragged_dimensioned_and_extruded() -> History {
    let mut history = History::default();
    history.push(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    history.push(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::ZERO),
        opposite: PointRef::New(DVec2::new(10.0, 10.0)),
        construction: false,
    });
    history.push(Operation::MovePoint {
        sketch: 0,
        point: PointId(2),
        position: DVec2::new(11.0, 9.0),
        merged_into: None,
    });
    history.push(Operation::MovePoint {
        sketch: 0,
        point: PointId(2),
        position: DVec2::new(12.0, 8.0),
        merged_into: None,
    });
    history.push(Operation::MoveMany {
        sketch: 0,
        points: vec![PointId(1), PointId(2), PointId(3), PointId(4)],
        by: DVec2::new(1.0, 0.0),
    });
    history.push(Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Length(SegmentId(0)),
        value: 30.0,
        placement: None,
    });
    history.push(Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Length(SegmentId(0)),
        value: 50.0,
        placement: None,
    });
    history.push(Operation::Extrude {
        sketch: 0,
        picks: vec![DVec2::new(5.0, 5.0)],
        distance: 4.0,
        mode: crate::history::ExtrusionMode::Add,
    });
    history
}

#[test]
fn a_dragged_dimensioned_part_compacts_to_the_same_body_with_fewer_steps() {
    let history = dragged_dimensioned_and_extruded();
    let before = PartState::rebuild(&history);

    let compacted = compact(&history);
    let after = PartState::rebuild(&compacted);

    assert!(
        (volume(&after.body) - volume(&before.body)).abs() < 1e-2,
        "before {} after {}",
        volume(&before.body),
        volume(&after.body)
    );
    assert!(
        compacted.operations().len() < history.operations().len(),
        "compacted to {} steps from {}",
        compacted.operations().len(),
        history.operations().len()
    );
}

#[test]
fn compacting_a_second_time_changes_nothing_more() {
    let history = dragged_dimensioned_and_extruded();
    let once = compact(&history);
    let twice = compact(&once);

    assert_eq!(once.operations().len(), twice.operations().len());
    assert_eq!(once.operations(), twice.operations());
}

#[test]
fn compacting_drops_the_redo_tail() {
    let mut history = dragged_dimensioned_and_extruded();
    history.undo();
    assert!(history.can_redo());

    let compacted = compact(&history);

    assert!(!compacted.can_redo());
    assert_eq!(compacted.applied(), compacted.operations().len());
}

#[test]
fn two_independent_dimensions_keep_their_value_and_driven_state() {
    let mut history = History::default();
    history.push(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    history.push(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::ZERO),
        opposite: PointRef::New(DVec2::new(10.0, 10.0)),
        construction: false,
    });
    history.push(Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Length(SegmentId(0)),
        value: 40.0,
        placement: None,
    });
    history.push(Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Length(SegmentId(1)),
        value: 20.0,
        placement: None,
    });

    let before = PartState::rebuild(&history);
    let compacted = compact(&history);
    let after = PartState::rebuild(&compacted);

    let before_dims = before.sketches[0].dimensions();
    let after_dims = after.sketches[0].dimensions();
    assert_eq!(before_dims.len(), after_dims.len());
    for (b, a) in before_dims.iter().zip(after_dims.iter()) {
        assert_eq!(b.driven, a.driven, "target {:?}", b.target);
        assert!((b.value - a.value).abs() < 1e-6);
    }
}

#[test]
fn a_merge_and_a_deletion_leave_only_what_is_still_drawn() {
    let mut history = History::default();
    history.push(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    history.push(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::new(-10.0, 0.0)),
        end: PointRef::New(DVec2::new(0.0, 10.0)),
        construction: false,
    });
    history.push(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::new(0.0, 10.0)),
        end: PointRef::New(DVec2::new(10.0, 0.0)),
        construction: false,
    });
    history.push(Operation::MergePoints {
        sketch: 0,
        kept: PointId(2),
        dropped: PointId(3),
    });
    history.push(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::new(20.0, 20.0)),
        end: PointRef::New(DVec2::new(30.0, 20.0)),
        construction: false,
    });
    history.push(Operation::EraseMany {
        sketch: 0,
        elements: vec![cao_sketch::Element::Segment(SegmentId(2))],
        dimensions: Vec::new(),
        constraints: Vec::new(),
    });

    let before = PartState::rebuild(&history);
    let compacted = compact(&history);
    let after = PartState::rebuild(&compacted);

    assert_eq!(
        before.sketches[0].live_segments().count(),
        after.sketches[0].live_segments().count()
    );
    assert_eq!(
        before.sketches[0].drawn_points().count(),
        after.sketches[0].drawn_points().count()
    );
    assert!(
        !compacted.operations().iter().any(|op| matches!(
            op,
            Operation::MergePoints { .. } | Operation::EraseMany { .. }
        )),
        "the trace of how the shape was reached does not survive",
    );
}

#[test]
fn a_revolution_around_a_segment_erased_afterwards_still_compacts() {
    let mut history = History::default();
    history.push(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    history.push(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::new(0.0, -10.0)),
        end: PointRef::New(DVec2::new(0.0, 10.0)),
        construction: true,
    });
    history.push(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::new(3.0, 0.0)),
        opposite: PointRef::New(DVec2::new(5.0, 2.0)),
        construction: false,
    });
    history.push(Operation::Revolve {
        sketch: 0,
        picks: vec![DVec2::new(4.0, 1.0)],
        axis: RevolutionAxis::Segment(SegmentId(0)),
        angle: 360.0,
        mode: crate::history::ExtrusionMode::Add,
    });
    // Tidying up an axis line once the feature that needed it has run is
    // ordinary use, not a special case.
    history.push(Operation::EraseMany {
        sketch: 0,
        elements: vec![cao_sketch::Element::Segment(SegmentId(0))],
        dimensions: Vec::new(),
        constraints: Vec::new(),
    });

    let before = volume(&PartState::rebuild(&history).body);
    let compacted = compact(&history);
    let after = volume(&PartState::rebuild(&compacted).body);

    assert!((after - before).abs() / before < 0.02, "{after} / {before}");
}

/// Drawing a circle keeps the place clicked on its rim as a point of the
/// drawing, held there by an `OnCircle` constraint added in the very same
/// step — not a separate one, the way `cao_sketch::rim_of` describes it for
/// every circle mode.
#[test]
fn a_circles_rim_point_stays_bundled_in_its_own_step_after_compacting() {
    let mut history = History::default();
    history.push(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    history.push(Operation::AddCircle {
        sketch: 0,
        center: PointRef::New(DVec2::ZERO),
        radius: 5.0,
        rim: vec![PointRef::New(DVec2::new(5.0, 0.0))],
        construction: false,
    });

    let compacted = compact(&history);

    assert_eq!(
        compacted.operations().len(),
        history.operations().len(),
        "the rim point and its constraint came back as their own steps \
         instead of staying inside the circle's",
    );
    let after = PartState::rebuild(&compacted);
    let sketch = &after.sketches[0];
    assert_eq!(
        sketch.drawn_points().count(),
        2,
        "the centre and the rim point"
    );
    let rim_point = PointId(2);
    assert!(
        sketch
            .constraints()
            .contains(&cao_sketch::Constraint::OnCircle {
                point: rim_point,
                circle: cao_sketch::CircleId(0),
            }),
        "the rim point no longer holds the circle: {:?}",
        sketch.constraints()
    );
}

#[test]
fn a_compacted_design_is_still_one_step_per_folder_and_reuses_no_operation_number() {
    let history = dragged_dimensioned_and_extruded();
    let numbers: Vec<u32> = history
        .steps()
        .iter()
        .flat_map(|step| step.operations().to_vec())
        .collect();

    let compacted = compact(&history);

    let counted: usize = compacted.steps().iter().map(Step::len).sum();
    assert_eq!(
        counted,
        compacted.operations().len(),
        "every operation is written in the folder of the step that holds it",
    );
    let after: Vec<u32> = compacted
        .steps()
        .iter()
        .flat_map(|step| step.operations().to_vec())
        .collect();
    assert!(
        after.iter().all(|number| !numbers.contains(number)),
        "a number that named an operation before the rewrite has come back \
         naming another: {numbers:?} against {after:?}",
    );
}
