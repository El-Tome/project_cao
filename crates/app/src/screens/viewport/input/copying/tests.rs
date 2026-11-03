//! What app · screens/viewport/input/copying.rs is held to.
//!
//! Closes #175.
//! - the steps and counts of the patterns keep the formula typed —
//!   `a_count_written_from_a_variable_is_kept_as_written_when_it_comes_out_whole`
//! - a count that comes out as anything but a positive whole number is
//!   refused — `a_count_written_from_a_variable_that_does_not_come_out_whole_is_refused`
//!
//! Closes #522.
//! - a count typed above the cap is brought down —
//!   `a_count_typed_above_a_full_turn_is_brought_down_to_it`
//! - with the line saying why — no test: its words are held in
//!   `wording/pattern`, and the fields it is written under have no net
//! - changing the angle with a count already typed brings it down the same way
//!   — `an_angle_typed_after_the_count_brings_the_count_down_too`
//! - an angle that never comes back round leaves the count alone —
//!   `an_angle_that_never_comes_back_round_leaves_the_count_as_typed`

use cao_sketch::{SketchAxis, WorkPlane};

use super::*;

/// A trait standing well clear of both sketch axes.
fn a_drawing() -> (Sketch, cao_sketch::SegmentId) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let low = sketch.add_point(DVec2::new(4.0, -5.0));
    let high = sketch.add_point(DVec2::new(4.0, 5.0));
    let trait_ = sketch.add_segment(low, high);
    (sketch, trait_)
}

#[test]
fn a_click_on_a_trait_names_that_trait_as_the_axis() {
    let (sketch, trait_) = a_drawing();

    assert_eq!(
        axis_at(&sketch, DVec2::new(4.0, 1.0), 0.5),
        Some(ChosenAxis::Trait(trait_))
    );
}

#[test]
fn a_click_on_nothing_but_a_sketch_axis_names_that_axis() {
    let (sketch, _) = a_drawing();

    assert_eq!(
        axis_at(&sketch, DVec2::new(20.0, 0.0), 0.5),
        Some(ChosenAxis::Sketch(SketchAxis::U))
    );
}

#[test]
fn a_trait_wins_over_the_axis_it_is_lying_on() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let west = sketch.add_point(DVec2::new(-5.0, 0.0));
    let east = sketch.add_point(DVec2::new(5.0, 0.0));
    let along = sketch.add_segment(west, east);

    assert_eq!(
        axis_at(&sketch, DVec2::new(1.0, 0.0), 0.5),
        Some(ChosenAxis::Trait(along)),
        "the trait is the smaller target, and the one the user drew"
    );
}

#[test]
fn a_click_far_from_anything_names_no_axis() {
    let (sketch, _) = a_drawing();

    assert_eq!(axis_at(&sketch, DVec2::new(20.0, 20.0), 0.5), None);
}

#[test]
fn a_pattern_waits_for_both_its_values() {
    let some = |first, second| LockedInput { first, second };

    assert_eq!(turned(some(Some(30.0), None)), None);
    assert_eq!(turned(some(None, Some(6.0))), None);
    assert_eq!(turned(some(Some(30.0), Some(6.0))), Some((30.0, 6)));
}

#[test]
fn a_grid_waits_for_all_four_of_its_values() {
    let full = [Some(20.0), Some(4.0), Some(15.0), Some(3.0)];

    assert_eq!(
        filled(full),
        Some((
            Repeats {
                step: 20.0,
                count: 4
            },
            Repeats {
                step: 15.0,
                count: 3
            }
        ))
    );
    for rank in 0..4 {
        let mut missing = full;
        missing[rank] = None;
        assert_eq!(filled(missing), None, "value {rank} left empty");
    }
}

#[test]
fn a_grid_of_one_by_one_is_no_pattern_at_all() {
    assert_eq!(filled([Some(20.0), Some(1.0), Some(15.0), Some(1.0)]), None);
    assert!(
        filled([Some(20.0), Some(4.0), Some(15.0), Some(1.0)]).is_some(),
        "a single row is a pattern like any other"
    );
}

#[test]
fn a_count_under_two_is_no_pattern_at_all() {
    let some = |first, second| LockedInput { first, second };

    assert_eq!(turned(some(Some(30.0), Some(1.0))), None);
    assert_eq!(
        turned(some(Some(30.0), Some(1.6))),
        Some((30.0, 2)),
        "a count is a whole number of copies, so what was typed is rounded to one"
    );
}

fn a_count_written_as(text: &str) -> Option<(Formula, f64)> {
    let mut variables = cao_part::Variables::default();
    variables.change(&cao_part::VariableChange::Added {
        name: "holes".to_string(),
        formula: Formula::Number(8.0),
    });
    variables.size_of(text).ok()
}

#[test]
fn a_count_written_from_a_variable_is_kept_as_written_when_it_comes_out_whole() {
    let typed = a_count_written_as("holes / 2");

    assert_eq!(
        count_as_typed(typed.clone(), 4),
        typed.map(|(written, _)| written)
    );
}

#[test]
fn a_count_written_from_a_variable_that_does_not_come_out_whole_is_refused() {
    assert_eq!(count_as_typed(a_count_written_as("holes / 3"), 3), None);
}

#[test]
fn a_count_typed_as_a_plain_number_is_rounded_as_it_always_was() {
    assert_eq!(
        count_as_typed(Some((Formula::Number(4.4), 4.4)), 4),
        Some(Formula::Number(4.0))
    );
}

fn typed(degrees: &str, count: &str) -> crate::screens::sketch::LiveInput {
    let mut live = crate::screens::sketch::LiveInput::default();
    for (rank, text) in [(0, degrees), (1, count)] {
        live.field(rank).text = text.to_string();
        live.take(rank, &cao_part::Variables::default(), None);
    }
    live
}

#[test]
fn a_count_typed_above_a_full_turn_is_brought_down_to_it() {
    let mut live = typed("90", "6");

    assert_eq!(
        count_brought_down(&mut live, &cao_part::Variables::default()),
        Some(4)
    );
    assert_eq!(live.typed(1), Some(4.0));
    assert_eq!(live.field(1).text, "4");
}

#[test]
fn an_angle_typed_after_the_count_brings_the_count_down_too() {
    let mut live = typed("", "6");
    assert_eq!(
        count_brought_down(&mut live, &cao_part::Variables::default()),
        None
    );

    live.field(0).text = "72".to_string();
    live.take(0, &cao_part::Variables::default(), None);

    assert_eq!(
        count_brought_down(&mut live, &cao_part::Variables::default()),
        Some(5)
    );
    assert_eq!(live.typed(1), Some(5.0));
}

#[test]
fn an_angle_that_never_comes_back_round_leaves_the_count_as_typed() {
    let mut live = typed("7.3", "60");

    assert_eq!(
        count_brought_down(&mut live, &cao_part::Variables::default()),
        None
    );
    assert_eq!(live.typed(1), Some(60.0));
}
