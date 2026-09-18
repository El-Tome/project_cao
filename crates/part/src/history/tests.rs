use cao_sketch::WorkPlane;
use glam::DVec2;

use super::*;

fn create_sketch() -> Operation {
    Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    }
}

fn extrude(sketch: usize) -> Operation {
    Operation::Extrude {
        sketch,
        picks: Vec::new(),
        distance: 10.0,
        mode: ExtrusionMode::Add,
    }
}

#[test]
fn a_sketch_and_the_strokes_on_it_are_one_step_of_the_design() {
    let mut history = History::default();
    history.push(create_sketch());
    history.push(segment_op(0));
    history.push(segment_op(0));

    let steps = history.steps();

    assert_eq!(steps.len(), 1);
    assert_eq!(steps[0].kind(), StepKind::Sketch);
    assert_eq!(steps[0].operations(), [1, 2, 3]);
}

#[test]
fn an_extrusion_opens_a_step_of_its_own() {
    let mut history = History::default();
    history.push(create_sketch());
    history.push(segment_op(0));
    history.push(create_sketch());
    history.push(extrude(1));

    let steps = history.steps();

    assert_eq!(steps.len(), 3);
    assert_eq!(steps[2].kind(), StepKind::Extrusion);
    assert_eq!(steps[2].operations(), [4]);
}

fn segment_op(sketch: usize) -> Operation {
    Operation::AddSegment {
        sketch,
        start: PointRef::New(DVec2::ZERO),
        end: PointRef::New(DVec2::X),
        construction: false,
    }
}

#[test]
fn undo_and_redo_walk_the_list() {
    let mut history = History::default();
    history.push(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    history.push(segment_op(0));
    assert_eq!(history.applied(), 2);
    assert!(!history.can_redo());

    assert!(history.undo());
    assert_eq!(history.applied(), 1);
    assert!(history.can_redo());
    assert_eq!(history.operations().len(), 2, "the tail is kept for redo");

    assert!(history.redo());
    assert_eq!(history.applied(), 2);
    assert!(!history.redo());
}

#[test]
fn undoing_past_the_start_does_nothing() {
    let mut history = History::default();
    assert!(!history.undo());
    assert!(!history.can_undo());
}

/// Drawing something new after an undo abandons the branch that was undone.
#[test]
fn a_new_operation_drops_what_was_undone() {
    let mut history = History::default();
    history.push(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    history.push(segment_op(0));
    history.undo();

    history.push(segment_op(1));

    assert_eq!(history.operations().len(), 2);
    assert_eq!(history.applied(), 2);
    assert!(!history.can_redo());
    assert_eq!(history.operations()[1], segment_op(1));
}

#[test]
fn rewinding_keeps_everything_for_redo() {
    let mut history = History::default();
    history.push(create_sketch());
    for _ in 0..4 {
        history.push(segment_op(0));
    }

    history.rewind_to(2);

    assert_eq!(history.applied(), 2);
    assert_eq!(history.applied_operations().len(), 2);
    assert_eq!(history.operations().len(), 5);
    assert!(history.can_redo());
}

#[test]
fn a_number_handed_out_is_never_handed_out_again() {
    let mut history = History::default();
    history.push(create_sketch());
    history.push(segment_op(0));
    history.undo();
    history.undo();

    history.push(create_sketch());

    let steps = history.steps();
    assert_eq!(steps.len(), 1);
    assert_eq!(
        steps[0].operations(),
        [3],
        "undo walks the operations in the order they were done, so a number \
         handed to one cannot come back naming another",
    );
}

#[test]
fn drawing_again_after_an_undo_leaves_the_step_it_lands_in_shorter() {
    let mut history = History::default();
    history.push(create_sketch());
    history.push(segment_op(0));
    history.push(segment_op(0));
    history.push(extrude(0));
    history.undo();
    history.undo();

    history.push(segment_op(0));

    let steps = history.steps();
    assert_eq!(steps.len(), 1, "the extrusion was abandoned with its step");
    assert_eq!(steps[0].operations(), [1, 2, 5]);
    assert_eq!(history.operations().len(), 3);
}

#[test]
fn a_design_put_back_together_from_its_index_is_the_one_that_was_taken_apart() {
    let mut history = History::default();
    history.push(create_sketch());
    history.push(segment_op(0));
    history.push(extrude(0));

    let index = history.index();
    let operations: Vec<Operation> = (0..index.steps.len())
        .flat_map(|step| history.operations_of(step).to_vec())
        .collect();

    assert_eq!(History::restore(index, operations), Some(history));
}

#[test]
fn a_step_holding_other_operations_than_the_index_says_is_refused() {
    let mut history = History::default();
    history.push(create_sketch());
    history.push(segment_op(0));

    assert_eq!(
        History::restore(history.index(), vec![create_sketch()]),
        None,
        "a design nobody can vouch for opens on nothing rather than on \
         half a part",
    );
}

#[test]
fn rewinding_past_the_end_is_clamped() {
    let mut history = History::default();
    history.push(create_sketch());
    history.rewind_to(99);
    assert_eq!(history.applied(), 1);
}
