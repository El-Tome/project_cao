//! What the two corner tools share.
//!
//! A chamfer and a fillet differ only in what they put where the corner was —
//! a straight cut or a curve. Everything around that is one job: reading how
//! wide the corner stands, pulling each side back, leaving the corner's own
//! point behind held on both sides, and carrying across the lengths the sides
//! were given. Neither tool is a kind of the other, so none of it belongs in
//! either file.

use std::f64::consts::TAU;

use crate::constraints::{Dimension, DimensionTarget};
use crate::holding::Support;
use crate::sketch::{PointId, SegmentId, Sketch};

impl Sketch {
    /// How wide a corner stands open, the shorter way round.
    pub(crate) fn opening_at(
        &self,
        pivot: PointId,
        far_first: PointId,
        far_second: PointId,
    ) -> f64 {
        let at = self.point(pivot);
        let turn = ((self.point(far_second) - at).to_angle()
            - (self.point(far_first) - at).to_angle())
        .rem_euclid(TAU);
        turn.min(TAU - turn)
    }

    /// Leaves the corner's own point standing where the corner was, held on the
    /// line each side now lies on.
    ///
    /// Two holds and no more: a point held on two crossing lines stands at
    /// their crossing, which is both what keeps the corner where the corner
    /// was and what carries it along when a side is turned or slid. It is also
    /// why a drag cannot take it anywhere of its own — see [`Sketch::slide`].
    pub(crate) fn hold_corner(&mut self, pivot: PointId, pieces: &[SegmentId], ends: [PointId; 2]) {
        for end in ends {
            let Some(piece) = piece_running_into(self, pieces, end) else {
                continue;
            };
            self.add_constraint(Support::Segment(piece).holding(pivot));
        }
    }

    /// The length each of the two sides was given, to be hung back on the
    /// corner once the cut has taken a piece of that side away.
    ///
    /// A trim drops a length: the trait it measured is gone, and what is left
    /// of it is shorter. Here the corner survives the cut, so the same length
    /// is still there to be read — from the corner out to the far end, which is
    /// the span it always measured. Read before the cut, since the cut is what
    /// takes it away.
    pub(crate) fn lengths_of(&self, sides: [SegmentId; 2]) -> [Option<Dimension>; 2] {
        sides.map(|side| self.dimension_of(DimensionTarget::Length(side)).copied())
    }

    /// Hangs those lengths back on the corner, and says how many were saved
    /// from the count the cut reported as lost.
    pub(crate) fn rehang(
        &mut self,
        held: [Option<Dimension>; 2],
        pivot: PointId,
        fars: [PointId; 2],
    ) -> usize {
        let mut saved = 0;
        for (held, far) in held.into_iter().zip(fars) {
            let Some(held) = held else {
                continue;
            };
            let target = DimensionTarget::Distance {
                from: pivot,
                to: far,
            }
            .normalised();
            self.set_dimension(target, held.value, held.driven);
            if let Some(offset) = held.offset {
                self.offset_dimension(target, offset);
            }
            saved += 1;
        }
        saved
    }
}

/// Which of the pieces left of the two sides runs into this point.
pub(crate) fn piece_running_into(
    sketch: &Sketch,
    pieces: &[SegmentId],
    point: PointId,
) -> Option<SegmentId> {
    pieces.iter().copied().find(|piece| {
        let side = sketch.segments()[piece.0];
        side.start == point || side.end == point
    })
}
