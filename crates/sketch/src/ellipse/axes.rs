//! How far an ellipse's axes reach, and so what curve they draw.
//!
//! An axis is laid **across** the whole curve, its centre halving it, or —
//! for the half a curve the two-ends tool places — **out** from the centre to
//! where the curve stops. Both shapes are read the same way here, from the
//! centre outwards, so that nothing above this has to know which it is.

use glam::DVec2;

use super::{EllipseDraft, EllipseId, PointId, SegmentId, Sketch};

impl Sketch {
    /// The curve as its axes draw it now.
    pub fn ellipse_draft(&self, id: EllipseId) -> EllipseDraft {
        let ellipse = self.ellipses[id.0];
        EllipseDraft {
            centre: self.point(ellipse.center),
            first: self.axis_reach(ellipse.center, ellipse.first),
            second: self.axis_reach(ellipse.center, ellipse.second).length(),
        }
    }

    /// How far an axis reaches from the centre, and which way.
    ///
    /// An axis is laid **across** the whole curve, the centre halving it, or —
    /// for the half a curve the two-ends tool places — **out** from the centre
    /// to where the curve stops, since the other half of it would stand where
    /// nothing is drawn. Which of the two it is, is whether the centre is one
    /// of its own ends; either way what the curve reaches is the same thing,
    /// read from the centre outwards.
    fn axis_reach(&self, centre: PointId, axis: SegmentId) -> DVec2 {
        let Some(held) = self.segments().get(axis.0).copied() else {
            return DVec2::ZERO;
        };
        let (start, end) = self.endpoints(axis);
        match (held.start == centre, held.end == centre) {
            (true, false) => end - start,
            (false, true) => start - end,
            _ => (end - start) * 0.5,
        }
    }

    /// Whether an axis is laid out from the centre rather than across the
    /// curve, which is what a half placed by its two ends carries.
    pub(crate) fn axis_stands_on_the_centre(&self, centre: PointId, axis: SegmentId) -> bool {
        self.segments()
            .get(axis.0)
            .is_some_and(|held| held.start == centre || held.end == centre)
    }
}
