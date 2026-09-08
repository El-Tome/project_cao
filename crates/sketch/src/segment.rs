//! Where a point sits with respect to a straight segment.
//!
//! Pure geometry, kept apart from the sketch itself: the answer is the same
//! whatever is drawn from it, and it is worth testing on its own.

use glam::DVec2;

/// The end of `start..end` that `foot` lies beyond, or `None` when it lies
/// between the two.
///
/// A foot outside the segment leaves both `foot - start` and `foot - end`
/// pointing the same way, and the end it overshot is then simply the nearer of
/// the two.
pub fn overshot_end(start: DVec2, end: DVec2, foot: DVec2) -> Option<DVec2> {
    if (foot - start).dot(foot - end) <= 0.0 {
        return None;
    }
    Some(
        if foot.distance_squared(start) < foot.distance_squared(end) {
            start
        } else {
            end
        },
    )
}

#[cfg(test)]
mod tests {
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
}
