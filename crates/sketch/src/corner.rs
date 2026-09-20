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

    /// What the corner was already worth before the cut takes a piece of it
    /// away: the length each side was given, and the angle they stood at.
    ///
    /// A trim drops both. A length measured a trait that is gone, and what is
    /// left of it is shorter; an angle is read between two traits sharing a
    /// point, and the pieces no longer touch. Both are still there to be read
    /// once the corner is put back — which is what [`Sketch::rehang`] does with
    /// them. Read before the cut, since the cut is what takes them away.
    pub(crate) fn values_at(&self, sides: [SegmentId; 2]) -> CornerValues {
        CornerValues {
            lengths: sides.map(|side| self.dimension_of(DimensionTarget::Length(side)).copied()),
            opening: self
                .dimension_of(
                    DimensionTarget::Angle {
                        first: sides[0],
                        second: sides[1],
                    }
                    .normalised(),
                )
                .copied(),
        }
    }

    /// Hangs those values back on the corner, and says how many were saved
    /// from the count the cut reported as lost.
    ///
    /// A length goes from the corner out to the far end, the span it always
    /// measured. The angle goes between the two stretches the cut removed,
    /// which are laid back in as construction and still meet at the corner —
    /// the only two traits left that do.
    pub(crate) fn rehang(
        &mut self,
        held: CornerValues,
        pivot: PointId,
        fars: [PointId; 2],
        stretches: [SegmentId; 2],
    ) -> usize {
        let mut saved = 0;
        for (length, far) in held.lengths.into_iter().zip(fars) {
            let Some(length) = length else {
                continue;
            };
            saved += self.rewrite(
                length,
                DimensionTarget::Distance {
                    from: pivot,
                    to: far,
                },
            );
        }
        if let Some(opening) = held.opening {
            saved += self.rewrite(
                opening,
                DimensionTarget::Angle {
                    first: stretches[0],
                    second: stretches[1],
                },
            );
        }
        saved
    }

    /// The same value, now said of something the drawing still has.
    fn rewrite(&mut self, held: Dimension, onto: DimensionTarget) -> usize {
        let target = onto.normalised();
        self.set_dimension(target, held.value, held.driven);
        if let Some(offset) = held.offset {
            self.offset_dimension(target, offset);
        }
        1
    }
}

/// What a corner was worth before it was cut off.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct CornerValues {
    /// The length each of the two sides was given, in the order they were
    /// named.
    lengths: [Option<Dimension>; 2],
    /// The angle the two sides stood at.
    opening: Option<Dimension>,
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
