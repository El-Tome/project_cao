//! What sketch · edges/border.rs is held to.

use super::*;

/// An ellipse whose two axes reach alike is a circle, whichever tool drew it
/// and whichever way its first axis was laid.
///
/// Held here rather than on a drawing: a circle laid over a round ellipse
/// takes the crossing hunt seconds to read, which is no test for the gate.
#[test]
fn a_run_of_a_round_ellipse_is_the_piece_of_circle_it_lies_along() {
    let places = [DVec2::new(2.0, 0.0), DVec2::new(0.0, 2.0)];
    let round = EllipseDraft {
        centre: DVec2::ZERO,
        first: DVec2::new(0.0, -2.0),
        second: 2.0,
    };
    let mut borders = Borders::default();
    assert_eq!(
        borders.along_curved(Bend::Round(DVec2::ZERO), (0, 1), &places, 0),
        None
    );
    assert_eq!(
        borders.along_curved(Bend::Oval(round), (0, 1), &places, 2),
        Some(0),
        "the run lies along the piece of circle from end to end, and is that border"
    );
}
