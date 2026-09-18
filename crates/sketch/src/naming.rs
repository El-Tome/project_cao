//! Naming a closed area of the drawing by the curves that bound it.
//!
//! A place stored inside an area says nothing once the drawing has moved:
//! pull the shape far enough and no area passes under it any more, and
//! whatever stood on that area quietly stands on nothing. The curves bounding
//! it are a name it cannot drift out of — they are only ever appended and
//! never reused, so the name holds under exactly the edits that must not break
//! it, and breaks under exactly the ones that deserve saying so.
//!
//! The name is worked out from the outline every time the drawing is read, and
//! never stored beside it: an area therefore has a name the instant it exists.

use glam::DVec2;
use serde::{Deserialize, Serialize};

use crate::arc::ArcId;
use crate::regions::Region;
use crate::sketch::{CircleId, SegmentId};

/// One curve of the drawing, whichever kind it was drawn as.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum CurveId {
    Segment(SegmentId),
    Arc(ArcId),
    Circle(CircleId),
}

/// Which closed area of a drawing something stands on.
///
/// `bounds` is the name: every curve that bordered the area when it was
/// pointed at, once each and in order. `inside` is the place pointed at, and
/// is read only to tell apart two areas the same curves bound — the two halves
/// of a circle a chord cuts are both bounded by that circle and that chord.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Area {
    pub bounds: Vec<CurveId>,
    pub inside: DVec2,
}

/// What became of the curves one cut replaced: each curve it took out, with
/// the curves standing in its place — none at all when the cut took the whole
/// of it.
///
/// A cut that replaces a curve by pieces of itself is not a curve lost, and
/// what stood on the area it bounded must not be. Only a cut knows which piece
/// came out of which curve, so only a cut can say.
pub type Became = Vec<(CurveId, Vec<CurveId>)>;

impl Area {
    /// The name of an area, as it stands, with the place that was pointed at.
    pub fn of(region: &Region, inside: DVec2) -> Self {
        Self {
            bounds: region.bounds().to_vec(),
            inside,
        }
    }

    /// Which of these areas the name answers to, and nothing when the drawing
    /// no longer encloses it.
    ///
    /// An area answers to the name when it is bounded by every curve the name
    /// holds. It may be bounded by more: rounding a corner puts a curve in the
    /// boundary that descends from no curve the name could have held, and an
    /// area nothing happened to must not be lost over it. Fewest of those
    /// extras wins, and the place pointed at settles a draw.
    pub fn found_in(&self, regions: &[Region]) -> Option<usize> {
        if self.bounds.is_empty() {
            return None;
        }
        let mut answering: Vec<(usize, usize)> = regions
            .iter()
            .enumerate()
            .filter(|(_, region)| {
                self.bounds
                    .iter()
                    .all(|curve| region.bounds().contains(curve))
            })
            .map(|(rank, region)| {
                (
                    region.bounds().len().saturating_sub(self.bounds.len()),
                    rank,
                )
            })
            .collect();
        answering.sort_unstable();

        let fewest = answering.first()?.0;
        let drawn: Vec<usize> = answering
            .iter()
            .take_while(|(extras, _)| *extras == fewest)
            .map(|(_, rank)| *rank)
            .collect();
        drawn
            .iter()
            .copied()
            .find(|rank| regions[*rank].contains(self.inside))
            .or_else(|| drawn.first().copied())
    }
}

/// Which of these areas a place falls in: the innermost, since a shape drawn
/// inside another means the small one and not the one it sits in.
pub fn area_under(regions: &[Region], place: DVec2) -> Option<usize> {
    regions
        .iter()
        .enumerate()
        .filter(|(_, region)| region.contains(place))
        .max_by_key(|(_, region)| region.depth)
        .map(|(rank, _)| rank)
}

#[cfg(test)]
mod tests;
