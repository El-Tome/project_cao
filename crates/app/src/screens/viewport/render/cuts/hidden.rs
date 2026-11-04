//! The stretch of a side — trait or curve — a cut hides up to its sharp
//! corner, as the canvas strokes it: thin, faint and dashed, lit with its cut.

use cao_prefs::theme::Theme;
use cao_sketch::{CutSide, PointId};

use super::super::tint_at;
use super::{CutCanvas, HIDDEN, LINKED, Stroke};

impl CutCanvas<'_> {
    /// The stroke of the stretch of a side a cut hides up to its corner: the
    /// side's own, or lit with the cut while it is held or pointed at, and
    /// faintly with the cuts laid with it.
    pub(super) fn hidden_at(
        &self,
        corner: PointId,
        side: CutSide,
        stroke: Stroke,
        theme: &Theme,
    ) -> Stroke {
        let lit = match (
            self.lit_ends.contains(&(corner, side)),
            self.linked_ends.contains(&(corner, side)),
        ) {
            (true, _) => 1.0,
            (false, true) => LINKED,
            (false, false) => return stroke,
        };
        Stroke {
            color: tint_at(theme.highlight, lit),
            ..stroke
        }
    }
}

/// How the stretch a cut hides up to its sharp corner is drawn: thin, faint
/// and dashed, the corner still there to be read.
pub(super) fn faint(stroke: Stroke) -> Stroke {
    let [red, green, blue, alpha] = stroke.color;
    Stroke {
        color: [red, green, blue, alpha * HIDDEN],
        width: stroke.width * 0.6,
        construction: true,
    }
}
