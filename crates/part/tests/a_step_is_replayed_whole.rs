//! What the design replays, and in what order.
//!
//! Closes #364.
//! - dragging a corner of a sketch an extrusion stands on changes the matter —
//!   `a_corner_dragged_long_afterwards_rebuilds_the_matter`
//! - the order things were typed still drives undo —
//!   `undo_walks_back_the_order_things_were_typed`
//! - an operation is recorded under the step it edits —
//!   `an_operation_belongs_to_the_step_it_edits`
//! - a design reopened is grouped the way it is replayed — no test: it is
//!   `regroup`, held beside the history by
//!   `a_design_read_back_is_grouped_by_what_each_operation_edits`
//! - going back to a step in the history panel still shows the part without
//!   what follows — no test: `rewind_to` moves the same cursor undo does, and
//!   `undo_walks_back_the_order_things_were_typed` holds that cursor

use cao_part::history::{ExtrusionMode, Operation, PointRef};
use cao_part::{History, PartState};
use cao_sketch::{PointId, WorkPlane};
use cao_solid::Mesh;
use glam::DVec2;

fn volume(mesh: &Mesh) -> f64 {
    mesh.triangles()
        .iter()
        .map(|[a, b, c]| a.dot(b.cross(*c)) / 6.0)
        .sum()
}

/// A rectangle raised into matter, a second sketch started afterwards, and
/// only then a corner of the first rectangle dragged out.
fn drawn_then_edited() -> History {
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
    history.push(Operation::Extrude {
        sketch: 0,
        picks: vec![DVec2::new(5.0, 5.0)],
        distance: 2.0,
        mode: ExtrusionMode::Add,
    });
    history.push(Operation::CreateSketch {
        plane: WorkPlane::XZ,
        on: None,
    });
    history.push(Operation::AddPoint {
        sketch: 1,
        position: DVec2::new(1.0, 1.0),
    });
    // The two far corners, dragged out long after the extrusion, leaving a
    // rectangle 10 by 20 where there was one 10 by 10.
    for (point, position) in [(2, DVec2::new(10.0, 20.0)), (4, DVec2::new(0.0, 20.0))] {
        history.push(Operation::MovePoint {
            sketch: 0,
            point: PointId(point),
            position,
            merged_into: None,
        });
    }
    history
}

#[test]
fn a_corner_dragged_long_afterwards_rebuilds_the_matter() {
    let state = PartState::rebuild(&drawn_then_edited());

    assert!(
        (volume(&state.body) - 400.0).abs() < 1.0,
        "the rectangle was dragged out to 10 by 20 and raised 2, which is 400 \
         of matter — the body holds {}",
        volume(&state.body),
    );
}

#[test]
fn an_operation_belongs_to_the_step_it_edits() {
    let history = drawn_then_edited();

    let sketch = &history.steps()[0];
    assert_eq!(
        sketch.operations().len(),
        4,
        "the first sketch holds its own opening, its rectangle and the two \
         corners dragged out later: {:?}",
        sketch.operations(),
    );
    assert_eq!(
        history.steps()[2].operations().len(),
        2,
        "the second sketch holds its opening and its point, and not the move \
         that was typed while it was open",
    );
}

#[test]
fn undo_walks_back_the_order_things_were_typed() {
    let mut history = drawn_then_edited();

    history.undo();

    assert!(
        (volume(&PartState::rebuild(&history).body) - 300.0).abs() < 1.0,
        "undoing takes back the last thing typed — the second corner — and the \
         drawing is a quadrilateral of 150 raised 2",
    );
}
