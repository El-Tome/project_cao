//! What app · screens/sketch/live_input.rs is held to.
//!
//! Closes #423.
//! - on a part with no scale yet, a length typed while a shape is drawn is
//!   worth the length the shape had on screen when it was typed, and keeps
//!   that size while its digits come —
//!   `a_length_typed_on_a_part_with_no_scale_is_worth_what_it_was_typed_over`,
//!   `the_length_stays_worth_what_it_was_first_typed_over_while_its_digits_come`
//! - a second size typed on the same shape says nothing of the scale, and one
//!   stage of a shape says it for the next —
//!   `a_second_length_typed_on_the_shape_says_nothing_of_the_scale`,
//!   `what_one_stage_of_a_shape_said_holds_for_the_next`
//! - an angle never sets the scale — `an_angle_says_nothing_of_the_scale`
//! - nothing moves when the length that said it is taken back, while another
//!   length is still typed — `a_length_taken_back_hands_the_scale_to_a_length_still_typed`

use super::*;

/// What the field of that rank holds once `text` is typed into it, over a shape
/// that measured `over` units along it.
fn type_into(live: &mut LiveInput, rank: usize, text: &str, over: Option<f64>) {
    live.field(rank).text = text.to_string();
    live.take(rank, &Variables::default(), over);
}

fn assert_close(scale: f64, expected: f64) {
    assert!(
        (scale - expected).abs() < 1e-12,
        "a unit should be worth {expected} mm, and is worth {scale}",
    );
}

#[test]
fn a_length_typed_on_a_part_with_no_scale_is_worth_what_it_was_typed_over() {
    let mut live = LiveInput::default();
    live.open();

    type_into(&mut live, 0, "100", Some(24.0));

    assert_close(live.scale(None), 100.0 / 24.0);
}

#[test]
fn the_length_stays_worth_what_it_was_first_typed_over_while_its_digits_come() {
    let mut live = LiveInput::default();
    live.open();

    type_into(&mut live, 0, "1", Some(24.0));
    type_into(&mut live, 0, "100", Some(0.24));

    assert_close(live.scale(None), 100.0 / 24.0);
}

#[test]
fn a_part_that_has_a_scale_keeps_it_whatever_is_typed() {
    let mut live = LiveInput::default();
    live.open();

    type_into(&mut live, 0, "100", Some(24.0));

    assert_close(live.scale(Some(2.5)), 2.5);
}

#[test]
fn a_second_length_typed_on_the_shape_says_nothing_of_the_scale() {
    let mut live = LiveInput::default();
    live.open();

    type_into(&mut live, 0, "100", Some(24.0));
    type_into(&mut live, 1, "50", Some(20.0));

    assert_close(live.scale(None), 100.0 / 24.0);
}

#[test]
fn an_angle_says_nothing_of_the_scale() {
    let mut live = LiveInput::default();
    live.open();

    type_into(&mut live, 1, "30", None);

    assert_close(live.scale(None), 1.0);
}

#[test]
fn a_length_taken_back_hands_the_scale_to_a_length_still_typed() {
    let mut live = LiveInput::default();
    live.open();
    type_into(&mut live, 0, "100", Some(24.0));
    type_into(&mut live, 1, "50", Some(20.0));

    type_into(&mut live, 0, "", Some(30.0));

    assert_close(live.scale(None), 100.0 / 24.0);
}

#[test]
fn with_no_length_left_to_say_it_the_next_one_typed_does() {
    let mut live = LiveInput::default();
    live.open();
    type_into(&mut live, 0, "100", Some(24.0));
    type_into(&mut live, 0, "", Some(30.0));
    assert_close(live.scale(None), 1.0);

    type_into(&mut live, 1, "50", Some(20.0));

    assert_close(live.scale(None), 50.0 / 20.0);
}

#[test]
fn what_one_stage_of_a_shape_said_holds_for_the_next() {
    let mut live = LiveInput::default();
    live.open();
    type_into(&mut live, 0, "100", Some(24.0));

    live.open_for_the_next_stage();
    type_into(&mut live, 0, "50", Some(20.0));

    assert_close(live.scale(None), 100.0 / 24.0);
}

#[test]
fn a_fresh_shape_has_said_nothing_yet() {
    let mut live = LiveInput::default();
    live.open();
    type_into(&mut live, 0, "100", Some(24.0));

    live.open();

    assert_close(live.scale(None), 1.0);
}

#[test]
fn a_length_of_nothing_says_nothing_of_the_scale() {
    let mut live = LiveInput::default();
    live.open();

    type_into(&mut live, 0, "0", Some(24.0));

    assert_close(live.scale(None), 1.0);
}

#[test]
fn a_length_typed_over_nothing_on_screen_says_nothing_of_the_scale() {
    let mut live = LiveInput::default();
    live.open();

    type_into(&mut live, 0, "100", Some(0.0));

    assert_close(live.scale(None), 1.0);
}
