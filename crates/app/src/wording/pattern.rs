//! What the user reads of a circular pattern a full turn holds fewer copies
//! of than were asked for.

use cao_part::Capped;

use crate::lang::Catalogue;

/// The line under the count, once it is brought down to the full turn.
pub fn count_brought_down(lang: &Catalogue, largest: usize) -> String {
    lang.t_with(
        "sketch.pattern_count_capped",
        &[
            ("count", &largest.to_string()),
            ("next", &(largest + 1).to_string()),
        ],
    )
}

/// What a step laying a full turn short of its count says of itself.
pub fn step_capped(lang: &Catalogue, capped: Capped) -> String {
    lang.t_with(
        "part_tree.history.pattern_capped",
        &[
            ("laid", &capped.laid.to_string()),
            ("degrees", &capped.degrees.to_string()),
            ("asked", &capped.asked.to_string()),
        ],
    )
}

#[cfg(test)]
mod tests;
