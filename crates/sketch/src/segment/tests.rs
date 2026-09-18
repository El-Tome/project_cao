//! What sketch · segment.rs is held to.

use super::*;

const START: DVec2 = DVec2::new(0.0, 0.0);
const END: DVec2 = DVec2::new(10.0, 0.0);

#[test]
fn a_foot_between_the_ends_overshoots_neither() {
    assert_eq!(overshot_end(START, END, DVec2::new(4.0, 3.0)), None);
    assert_eq!(overshot_end(START, END, START), None);
    assert_eq!(overshot_end(START, END, END), None);
}

#[test]
fn a_foot_past_an_end_overshoots_that_one_alone() {
    assert_eq!(
        overshot_end(START, END, DVec2::new(14.0, 2.0)),
        Some(END),
        "past the far end, the leader grows from the far end",
    );
    assert_eq!(overshot_end(START, END, DVec2::new(-3.0, 2.0)), Some(START));
}

#[test]
fn which_end_is_named_does_not_change_the_answer() {
    let foot = DVec2::new(14.0, 2.0);
    assert_eq!(
        overshot_end(START, END, foot),
        overshot_end(END, START, foot),
    );
}
