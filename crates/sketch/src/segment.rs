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
mod tests;
