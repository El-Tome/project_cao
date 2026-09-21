//! What the two corner tools share.
//!
//! A chamfer and a fillet differ only in what they put where the corner was —
//! a straight cut or a curve. Everything around that is one job: reading how
//! wide the corner stands, pulling each side back, leaving the corner's own
//! point behind held on both sides, and carrying across the lengths the sides
//! were given. Neither tool is a kind of the other, so none of it belongs in
//! either file.

use std::f64::consts::TAU;

use serde::{Deserialize, Serialize};

use crate::constraints::{Dimension, DimensionTarget};
use crate::holding::Support;
use crate::sketch::{PointId, SegmentId, Sketch};

/// How a corner was named, as the history records it.
///
/// Not the two traits it stood between at the time: cutting one corner of a
/// shape trims the traits its neighbours lean on, and a neighbour recorded by
/// those traits would name curves that no longer exist by the time its own turn
/// came. Rounding the four corners of a plate is exactly that case, and it is
/// the one the tool was made for.
///
/// A point is enough wherever two traits meet, and it survives every cut, since
/// the corner's own point is what a chamfer and a fillet now leave behind. The
/// two traits are recorded only where a point cannot say which corner is meant.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Corner {
    /// The point two traits meet at. Which two is read from the drawing at the
    /// moment the cut is made.
    At(PointId),
    /// The two traits themselves, the one named first before the other — for a
    /// point too crowded to name a corner on its own, and for the chamfer modes
    /// that measure from the side named first.
    ///
    /// Two corners of one shape share a trait, so cutting the first leaves this
    /// one naming a curve that is gone. What became of each trait is followed
    /// through the cuts rather than trusted as a number, which is the part
    /// layer's business — [`Sketch::sides_of`] answers only for a drawing
    /// nothing has cut since.
    Between(SegmentId, SegmentId),
}

impl Sketch {
    /// The two traits a corner stands between, as the drawing has them now.
    ///
    /// Nothing when the corner is no longer there to cut: a trait erased by an
    /// earlier cut, or a point that stopped being a corner.
    pub fn sides_of(&self, corner: Corner) -> Option<(SegmentId, SegmentId)> {
        match corner {
            Corner::At(point) => self.corner_at(point),
            Corner::Between(first, second) => {
                self.shared_point(first, second).map(|_| (first, second))
            }
        }
    }

    /// The point two traits meet at, for naming the corner they make.
    pub fn shared_point(&self, first: SegmentId, second: SegmentId) -> Option<PointId> {
        let (pivot, _, _) = self.shared_corner(first, second)?;
        Some(pivot)
    }

    /// The point a corner stands at, whichever way it was named. What a tool
    /// shows as taken, and what the other trait of a half-named corner is
    /// looked up from.
    pub fn corner_point(&self, corner: Corner) -> Option<PointId> {
        match corner {
            Corner::At(point) => Some(point),
            Corner::Between(first, second) => self.shared_point(first, second),
        }
    }

    /// The other trait of the corner a side was named at, when the click that
    /// should have named it landed on the corner itself.
    ///
    /// Both traits pass through that point, so the nearest is whichever the
    /// drawing holds first — the same one every time, and the corner never
    /// completes. The side already named is the one thing that settles it:
    /// with exactly two traits meeting there, the other is the only choice
    /// left, which is not a guess.
    pub fn other_side_at(&self, point: PointId, named: SegmentId) -> Option<SegmentId> {
        let (first, second) = self.corner_at(point)?;
        match (first == named, second == named) {
            (true, false) => Some(second),
            (false, true) => Some(first),
            _ => None,
        }
    }

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

    /// The corner a point makes: the two traits that meet there, when exactly
    /// two do and nothing else leans on it.
    ///
    /// This is what lets a corner be named by one click instead of two. Three
    /// traits leave no saying which two the cut is meant for, and a curve
    /// running in is neither one of them nor nothing — a corner is still two
    /// straight traits. Both cases are refused here rather than guessed at, so
    /// the tool can ask for the two traits by name.
    ///
    /// Only the traits the shape is drawn with count. A corner already cut
    /// keeps its point, with the two stretches the cut laid back in still
    /// meeting there — read as a corner, they let the same corner be cut a
    /// second time, which crosses the drawing over itself. A corner of
    /// construction traits is still named one trait at a time.
    pub fn corner_at(&self, point: PointId) -> Option<(SegmentId, SegmentId)> {
        if !self.arcs_leaning_on(point).is_empty()
            || self.live_circles().any(|(_, round)| round.center == point)
        {
            return None;
        }
        match self.traits_at(point).as_slice() {
            [first, second] => Some((*first, *second)),
            _ => None,
        }
    }

    /// Every trait that runs into a point, in the order the drawing holds
    /// them. How many there are is what tells a tool whether the point names a
    /// corner on its own, or whether it has to ask which two traits are meant.
    pub fn traits_at(&self, point: PointId) -> Vec<SegmentId> {
        self.live_segments()
            .filter(|(_, side)| !side.construction)
            .filter(|(_, side)| side.start == point || side.end == point)
            .map(|(id, _)| id)
            .collect()
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

#[cfg(test)]
mod tests;
