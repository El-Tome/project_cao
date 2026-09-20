//! What app · screens/viewport/input/corner.rs is held to.
//!
//! Closes #315.
//! - `Entrée` that cannot lay anything says why, instead of doing nothing —
//!   `enter_with_half_a_corner_asks_for_the_other_side_rather_than_dropping_it`,
//!   `enter_with_no_side_clicked_asks_for_a_corner`,
//!   `enter_with_both_sides_named_cuts_them`

use super::*;

fn locked(first: Option<f64>, second: Option<f64>) -> LockedInput {
    LockedInput { first, second }
}

#[test]
fn the_equal_mode_takes_the_one_value_typed_for_both_sides() {
    assert_eq!(
        typed(locked(Some(4.0), None), ChamferMode::Equal, false),
        Some(Chamfer::Equal(4.0)),
        "a second value would say nothing the first does not"
    );
}

#[test]
fn a_mode_asking_two_values_waits_until_both_are_typed() {
    assert_eq!(
        typed(locked(Some(4.0), None), ChamferMode::Sided, false),
        None
    );
    assert_eq!(
        typed(locked(None, Some(30.0)), ChamferMode::Angled, false),
        None
    );
    assert_eq!(
        typed(locked(Some(4.0), Some(30.0)), ChamferMode::Angled, false),
        Some(Chamfer::Angled {
            along: 4.0,
            degrees: 30.0
        })
    );
}

#[test]
fn a_fillet_asks_for_one_value_whatever_the_chamfer_beside_it_is_set_to() {
    assert_eq!(
        typed(locked(Some(5.0), None), ChamferMode::Sided, true),
        Some(Chamfer::Equal(5.0)),
        "a radius is a radius, and the chamfer's second field says nothing about it"
    );
}

#[test]
fn a_corner_is_held_only_once_both_its_sides_have_been_clicked() {
    assert_eq!(corner_held(&ToolState::None), None);
    assert_eq!(
        corner_held(&ToolState::Corner {
            sides: vec![SegmentId(1)]
        }),
        None,
        "one side is half a corner, and Enter has nothing to cut"
    );
    assert_eq!(
        corner_held(&ToolState::Corner {
            sides: vec![SegmentId(1), SegmentId(2)]
        }),
        Some((SegmentId(1), SegmentId(2)))
    );
}

#[test]
fn enter_with_half_a_corner_asks_for_the_other_side_rather_than_dropping_it() {
    assert_eq!(
        on_enter(&ToolState::Corner {
            sides: vec![SegmentId(1)]
        }),
        CornerEnter::Waiting("sketch.click_the_other_side"),
        "Enter used to fall through to the rectangle, which put its own state \
         where the corner's was — so the side already clicked was lost and had \
         to be clicked again"
    );
}

#[test]
fn enter_with_no_side_clicked_asks_for_a_corner() {
    assert_eq!(
        on_enter(&ToolState::None),
        CornerEnter::Waiting("sketch.click_a_corner")
    );
}

#[test]
fn enter_with_both_sides_named_cuts_them() {
    assert_eq!(
        on_enter(&ToolState::Corner {
            sides: vec![SegmentId(1), SegmentId(2)]
        }),
        CornerEnter::Cut(SegmentId(1), SegmentId(2))
    );
}
