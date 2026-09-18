//! A cut landing on the place a circle brushes a trait.
//!
//! The contact point is the one thing `Sketch::erase` takes with a tangency,
//! so a trim that stops on it used to leave the surviving piece standing on a
//! point the drawing had erased — which compaction then quietly repaired,
//! changing the drawing.

use cao_part::history::{Operation, PointRef};
use cao_part::{History, PartState, compact};
use cao_sketch::{CircleId, Constraint, PointId, SegmentId, WorkPlane};
use glam::DVec2;

fn a_circle_brushing_a_trait() -> History {
    let mut history = History::default();
    history.push(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    history.push(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::new(2.0, 4.0)),
        end: PointRef::New(DVec2::new(12.0, 4.0)),
        construction: false,
    });
    history.push(Operation::AddCircle {
        sketch: 0,
        center: PointRef::New(DVec2::new(7.0, 7.0)),
        radius: 3.0,
        rim: Vec::new(),
        construction: false,
    });
    history.push(Operation::Constrain {
        sketch: 0,
        constraint: Constraint::Tangent {
            circle: CircleId(0),
            segment: SegmentId(0),
            at: None,
        },
    });
    history
}

fn contact_of(state: &PartState) -> PointId {
    state.sketches[0]
        .constraints()
        .iter()
        .find_map(|rule| match rule {
            Constraint::Tangent {
                at: Some(point), ..
            } => Some(*point),
            _ => None,
        })
        .expect("a tangency keeps a point where the two touch")
}

fn drawing_of(state: &PartState) -> (usize, Vec<(DVec2, DVec2)>, usize) {
    let sketch = &state.sketches[0];
    (
        sketch.live_points().count(),
        sketch
            .live_segments()
            .map(|(id, _)| sketch.endpoints(id))
            .collect(),
        sketch.live_circles().count(),
    )
}

fn trimmed_to_the_contact() -> History {
    let mut history = a_circle_brushing_a_trait();
    let contact = contact_of(&PartState::rebuild(&history));
    history.push(Operation::Trim {
        sketch: 0,
        segment: SegmentId(0),
        from: PointId(1),
        to: contact,
    });
    history
}

#[test]
fn a_piece_cut_to_the_contact_stands_on_a_point_the_drawing_still_has() {
    let history = trimmed_to_the_contact();
    let state = PartState::rebuild(&history);
    let sketch = &state.sketches[0];

    for (id, piece) in sketch.live_segments() {
        assert!(
            !sketch.is_erased_point(piece.start) && !sketch.is_erased_point(piece.end),
            "{id:?} stands on a point that is gone",
        );
    }
}

#[test]
fn compacting_that_history_leaves_the_drawing_exactly_as_it_was() {
    let history = trimmed_to_the_contact();

    assert_eq!(
        drawing_of(&PartState::rebuild(&history)),
        drawing_of(&PartState::rebuild(&compact(&history))),
    );
}
