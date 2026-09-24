//! What part · feature.rs is held to.

use super::*;
use crate::history::{Operation, PointRef};
use cao_sketch::WorkPlane;
use glam::DVec2;

fn drawn(operations: impl IntoIterator<Item = Operation>) -> History {
    let mut history = History::default();
    for operation in operations {
        history.push(operation);
    }
    history
}

fn sketch() -> Operation {
    Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    }
}

fn segment(sketch: usize) -> Operation {
    Operation::AddSegment {
        sketch,
        start: PointRef::New(DVec2::ZERO),
        end: PointRef::New(DVec2::X),
        construction: false,
    }
}

fn extrude(sketch: usize) -> Operation {
    Operation::Extrude {
        sketch,
        areas: Vec::new(),
        distance: 10.0.into(),
        mode: crate::history::ExtrusionMode::Add,
    }
}

#[test]
fn an_extrusion_between_two_sketches_does_not_shift_the_second_one() {
    let history = drawn([sketch(), segment(0), extrude(0), sketch(), segment(1)]);

    let features = Feature::all(&history);

    assert_eq!(features.len(), 3);
    assert_eq!(features[0].sketch, Some(0));
    assert_eq!(features[1].sketch, None, "an extrusion opens no sketch");
    assert_eq!(features[2].sketch, Some(1));
}

#[test]
fn the_steps_that_follow_a_feature_are_grouped_under_it() {
    let history = drawn([sketch(), segment(0), segment(0), extrude(0)]);

    let features = Feature::all(&history);

    assert_eq!(features.len(), 2);
    assert_eq!((features[0].start, features[0].end), (0, 3));
    assert_eq!((features[1].start, features[1].end), (3, 4));
}

#[test]
fn a_history_with_nothing_in_it_has_no_feature() {
    assert!(Feature::all(&History::default()).is_empty());
}

#[test]
fn a_stroke_that_would_belong_to_no_step_is_not_recorded() {
    let history = drawn([segment(0), segment(0)]);

    assert!(
        Feature::all(&history).is_empty(),
        "every operation is written in a step's folder, so one that names \
         a sketch the part does not have has nowhere to go — and the \
         geometry makes nothing of it either",
    );
    assert!(history.operations().is_empty());
}

/// An edit to an earlier sketch is replayed with that sketch, and still
/// reads in the journal where it was typed — at the end, under the last
/// feature opened, rather than pulling the features after it into the
/// first one.
#[test]
fn an_edit_to_an_earlier_sketch_reads_where_it_was_typed() {
    let history = drawn([
        sketch(),
        segment(0),
        Operation::Extrude {
            sketch: 0,
            areas: Vec::new(),
            distance: 1.0.into(),
            mode: crate::history::ExtrusionMode::Add,
        },
        sketch(),
        segment(0),
    ]);

    let features = Feature::all(&history);

    assert_eq!(features.len(), 3);
    assert_eq!((features[0].start, features[0].end), (0, 2));
    assert_eq!((features[1].start, features[1].end), (2, 3));
    assert_eq!(
        (features[2].start, features[2].end),
        (3, 5),
        "the edit typed last reads under the feature that was open",
    );
}

fn a_variable(name: &str) -> Operation {
    Operation::Variable(crate::variables::VariableChange::Added {
        name: name.to_string(),
        formula: crate::formula::Formula::Number(1.0),
    })
}

#[test]
fn a_variable_made_before_anything_is_drawn_has_a_line_of_its_own() {
    let history = drawn([a_variable("width"), a_variable("height"), sketch()]);

    assert_eq!(
        Feature::all(&history),
        vec![
            Feature {
                start: 0,
                end: 2,
                sketch: None
            },
            Feature {
                start: 2,
                end: 3,
                sketch: Some(0)
            },
        ],
    );
}

#[test]
fn a_variable_changed_while_drawing_shows_where_it_was_changed() {
    let history = drawn([sketch(), segment(0), a_variable("width"), segment(0)]);

    assert_eq!(
        Feature::all(&history),
        vec![Feature {
            start: 0,
            end: 4,
            sketch: Some(0)
        }],
    );
}
