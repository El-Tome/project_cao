//! What the angle of a trait drawn on from another one is read against: the
//! corner the two make.
//!
//! A trait carried on from another is turned from it, and the number the user
//! has in mind is the corner's — 90 for a square one, 180 straight on — as the
//! dimension laid there reads it. Which side of the trait before it leaves on
//! is the cursor's to say. A trait drawn on from no trait reads its angle
//! against the horizontal.

use glam::DVec2;

use crate::aim::ChainAnchor;
use crate::sketch::{SegmentId, Sketch};

impl Sketch {
    /// The trait a new one is drawn on from: the one drawn just before it in
    /// the chain, or else the only trait ending where it starts.
    ///
    /// Where several traits end, choosing one of them would be arbitrary, and
    /// a reference that changed as the cursor moved would change the number
    /// shown under the user's eyes: the angle is read against the horizontal.
    pub fn drawn_on_from(
        &self,
        anchor: ChainAnchor,
        previous: Option<SegmentId>,
    ) -> Option<SegmentId> {
        let ChainAnchor::Point(at) = anchor else {
            return None;
        };
        let mut ending = self
            .live_segments()
            .filter(|(_, segment)| segment.start == at || segment.end == at)
            .map(|(id, _)| id);
        if let Some(previous) = previous {
            return ending.any(|id| id == previous).then_some(previous);
        }
        let only = ending.next()?;
        ending.next().is_none().then_some(only)
    }

    /// Where the corner is, and the way back from it along the trait drawn on
    /// from.
    fn corner_drawn_on(
        &self,
        anchor: ChainAnchor,
        previous: Option<SegmentId>,
    ) -> Option<(DVec2, DVec2)> {
        let on = self.drawn_on_from(anchor, previous)?;
        let ChainAnchor::Point(at) = anchor else {
            return None;
        };
        let drawn = self.segments()[on.0];
        let other = match drawn.start == at {
            true => drawn.end,
            false => drawn.start,
        };
        let corner = self.point(at);
        Some((corner, (self.point(other) - corner).try_normalize()?))
    }

    /// The way a trait leaves the corner when `degrees` are typed for it: that
    /// far round from the trait before, on the side `towards` lies. The sign
    /// typed is the cursor's to say, and is not read.
    pub(crate) fn turned_at_the_corner(
        &self,
        anchor: ChainAnchor,
        previous: Option<SegmentId>,
        degrees: f64,
        towards: DVec2,
    ) -> Option<DVec2> {
        let (corner, back) = self.corner_drawn_on(anchor, previous)?;
        let side = side_of(back, towards - corner);
        Some(DVec2::from_angle(-side * degrees.abs().to_radians()).rotate(back))
    }

    /// The angle a trait from the anchor to `end` is typed as, in degrees.
    ///
    /// Drawn on from a trait, the corner it makes with it, signed by the side
    /// it leaves on: + on the left of the trait before, the way an angle turns
    /// against the horizontal, − on its right. Drawn on from no trait, its
    /// direction against the horizontal.
    pub fn angle_as_typed(
        &self,
        anchor: ChainAnchor,
        previous: Option<SegmentId>,
        end: DVec2,
    ) -> Option<f64> {
        let span = end - self.anchor_position(anchor)?;
        Some(match self.corner_drawn_on(anchor, previous) {
            Some((_, back)) => side_of(back, span) * back.angle_to(span).abs().to_degrees(),
            None => span.y.atan2(span.x).to_degrees(),
        })
    }
}

/// Which side of the trait before a direction leaves the corner on: 1 on the
/// left of the way the trait ran into the corner, -1 on its right.
fn side_of(back: DVec2, leaving: DVec2) -> f64 {
    match (-back).perp_dot(leaving) >= 0.0 {
        true => 1.0,
        false => -1.0,
    }
}

#[cfg(test)]
mod tests;
