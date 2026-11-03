//! A circular pattern lays no copy that comes back round onto the first, and
//! says so when a variable asks it for more than a full turn.
//!
//! Closes #522.
//! - the count is capped at the full turn for 90°, 180°, 270° and 72°, and left
//!   free for an angle that never comes back round — no test: held in
//!   `cao_sketch` by `a_turn_comes_back_round_onto_the_original_after_so_many_copies`
//!   and `a_turn_that_never_comes_back_round_within_a_turn_of_copies_keeps_no_cap`
//! - a count typed above the cap is brought down with the line saying why — no
//!   test: held in `cao_app` by `a_count_typed_above_a_full_turn_is_brought_down_to_it`
//! - a replay whose variable lowers the cap lays the cap, keeps the count
//!   written, and the step says why —
//!   `a_variable_that_turns_the_pattern_onto_itself_lays_a_full_turn_and_says_so`;
//!   the yellow is the history panel's, which has no net
//! - moving the variable back lays every copy again, and the step says nothing
//!   — `moving_the_variable_back_lays_every_copy_again`

use cao_part::{Formula, Operation, PartDocument, PointRef, VariableChange, VariableId};
use cao_sketch::{Element, SegmentId, Sketch, WorkPlane};
use glam::DVec2;

/// `step` at 60°, a trait turned six times about the origin by it.
fn six_turned_by_a_variable() -> PartDocument {
    let mut document = PartDocument::new("Rosace", "2026-01-02T09:00:00Z".parse().expect("a date"));
    document
        .change_variable(VariableChange::Added {
            name: "step".to_string(),
            formula: Formula::Number(60.0),
        })
        .expect("an angle");
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::new(10.0, 0.0)),
        end: PointRef::New(DVec2::new(20.0, 0.0)),
        construction: false,
    });
    let degrees = document.variables().read("step").expect("it reads");
    document.apply(Operation::CircularPattern {
        sketch: 0,
        elements: vec![Element::Segment(SegmentId(0))],
        centre: Sketch::ORIGIN,
        degrees,
        count: 6.0.into(),
    });
    document
}

fn step_to(document: &mut PartDocument, degrees: f64) {
    document
        .change_variable(VariableChange::Edited {
            variable: VariableId(0),
            name: "step".to_string(),
            formula: Formula::Number(degrees),
        })
        .expect("a pattern turning onto itself does not refuse the change");
}

fn traits(document: &PartDocument) -> usize {
    document.sketches()[0].live_segments().count()
}

/// The pattern is the fourth step: the variable, the sketch, the trait, the
/// pattern.
const THE_PATTERN: u32 = 4;

#[test]
fn a_variable_that_turns_the_pattern_onto_itself_lays_a_full_turn_and_says_so() {
    let mut document = six_turned_by_a_variable();
    assert_eq!(traits(&document), 6);

    step_to(&mut document, 90.0);

    assert_eq!(
        traits(&document),
        4,
        "four at 90°, the fifth would land on the first"
    );
    let capped = document
        .capped_at(THE_PATTERN)
        .expect("the step says it lays fewer");
    assert_eq!((capped.asked, capped.laid), (6, 4));
    assert!(
        document
            .history
            .operations()
            .iter()
            .any(|operation| matches!(
                operation,
                Operation::CircularPattern { count, .. } if *count == Formula::Number(6.0)
            )),
        "the count written in the step stays as written"
    );
}

#[test]
fn moving_the_variable_back_lays_every_copy_again() {
    let mut document = six_turned_by_a_variable();
    step_to(&mut document, 90.0);

    step_to(&mut document, 60.0);

    assert_eq!(traits(&document), 6);
    assert!(document.capped_at(THE_PATTERN).is_none());
}
