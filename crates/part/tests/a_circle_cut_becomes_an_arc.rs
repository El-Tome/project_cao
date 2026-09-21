//! A cut taken out of a circle, read through the door the canvas uses: one
//! step of the history, and an arc of the very circle it was taken from.
//!
//! Closes #290.
//! - a trim of a circle is one step in the history, undone in one go —
//!   `a_cut_of_a_circle_is_one_step_of_the_history_and_is_undone_in_one_go`
//! - compacting the history (#191) leaves the drawing as it was —
//!   `compacting_a_history_that_cuts_a_circle_leaves_the_drawing_as_it_was`
//! - a second click on that arc takes a further stretch of it, as #289 does —
//!   `a_second_cut_takes_a_further_stretch_of_the_arc_the_first_one_left`
//! - a tool pointed at the arc left behind answers as it would on the whole
//!   circle — no test: nothing is asked of the arc that `Arc` does not already
//!   answer, and #359 holds that reading

use cao_part::history::{Operation, PointRef};
use cao_part::{History, PartState, compact};
use cao_sketch::{CircleId, PointId, Sketch, WorkPlane};
use glam::DVec2;

const CUT: CircleId = CircleId(0);
const REACH: f64 = 10.0;

fn on_the_rim(degrees: f64) -> DVec2 {
    DVec2::from_angle(degrees.to_radians()) * REACH
}

/// A round about the sketch origin with four points sitting on it, a quarter
/// turn apart from due east.
fn a_round_with_four_points() -> History {
    let mut history = History::default();
    history.push(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    history.push(Operation::AddCircle {
        sketch: 0,
        center: PointRef::Existing(Sketch::ORIGIN),
        radius: REACH,
        rim: Vec::new(),
        construction: false,
    });
    for degrees in [0.0, 90.0, 180.0, 270.0] {
        history.push(Operation::AddPoint {
            sketch: 0,
            position: on_the_rim(degrees),
            on: Vec::new(),
        });
    }
    history
}

/// The points of `a_round_with_four_points`, in the order they were laid: due
/// east first, then a quarter turn on each time.
fn quarter(turn: usize) -> PointId {
    PointId(turn + 1)
}

fn drawn(state: &PartState) -> (usize, usize) {
    let sketch = &state.sketches[0];
    (sketch.live_circles().count(), sketch.live_arcs().count())
}

fn a_round_cut_once() -> History {
    let mut history = a_round_with_four_points();
    history.push(Operation::TrimCircle {
        sketch: 0,
        circle: CUT,
        between: Some((quarter(0), quarter(1))),
    });
    history
}

#[test]
fn a_cut_of_a_circle_is_one_step_of_the_history_and_is_undone_in_one_go() {
    let before = PartState::rebuild(&a_round_with_four_points());
    let after = PartState::rebuild(&a_round_cut_once());

    assert_eq!(drawn(&before), (1, 0), "a round, and no curve");
    assert_eq!(
        drawn(&after),
        (0, 1),
        "one step later, a curve and no round"
    );

    let mut history = a_round_cut_once();
    assert!(history.undo(), "there is a step to take back");

    assert_eq!(
        drawn(&PartState::rebuild(&history)),
        (1, 0),
        "the round is back, and one step back is all it took",
    );
}

#[test]
fn compacting_a_history_that_cuts_a_circle_leaves_the_drawing_as_it_was() {
    let history = a_round_cut_once();

    assert_eq!(
        drawn(&PartState::rebuild(&history)),
        drawn(&PartState::rebuild(&compact(&history))),
    );
}

#[test]
fn a_second_cut_takes_a_further_stretch_of_the_arc_the_first_one_left() {
    let mut history = a_round_cut_once();
    let arc = PartState::rebuild(&history).sketches[0]
        .live_arcs()
        .map(|(id, _)| id)
        .next()
        .expect("an arc of the circle stayed");
    history.push(Operation::TrimArc {
        sketch: 0,
        arc,
        from: quarter(2),
        to: quarter(3),
    });

    let state = PartState::rebuild(&history);

    assert_eq!(
        drawn(&state),
        (0, 2),
        "the curve gave up a further stretch, and both ends of it stayed",
    );
}
