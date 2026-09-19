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

/// What became of the curves one cut replaced: each curve it took out, with
/// the curves standing in its place — none at all when the cut took the whole
/// of it.
///
/// A cut that replaces a curve by pieces of itself is not a curve lost, and
/// what stood on the area it bounded must not be. Only a cut knows which piece
/// came out of which curve, so only a cut can say.
pub type Became = Vec<(CurveId, Vec<CurveId>)>;

/// Which closed area of a drawing something stands on.
///
/// `bounds` is the name: every curve that bordered the area when it was
/// pointed at, once each and in order. `inside` is the place pointed at, and
/// is read to tell apart areas the same curves bound — the two halves of a
/// circle a chord cuts are both bounded by that circle and that chord.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Area {
    pub bounds: Vec<CurveId>,
    pub inside: DVec2,
}

impl Area {
    /// The name of an area, as it stands, with the place that was pointed at.
    pub fn of(region: &Region, inside: DVec2) -> Self {
        Self {
            bounds: region.bounds().to_vec(),
            inside,
        }
    }

    /// The name read against a drawing nothing has been cut out of since it
    /// was written, where every border is still the one curve it was.
    pub fn uncut(&self) -> Standing {
        Standing {
            borders: self.bounds.iter().map(|curve| vec![*curve]).collect(),
            inside: self.inside,
        }
    }

    /// Which of these areas the name answers to, nothing having been cut
    /// since it was written.
    pub fn found_in(&self, regions: &[Region]) -> Option<usize> {
        self.uncut().found_in(regions)
    }
}

/// A name read against the drawing as it stands now: for every curve that
/// bordered the area, the curves standing in that border's place.
///
/// One border, one entry — even where a cut left several curves along it. An
/// area answers to the name when it is bounded by **at least one** curve out
/// of every entry: dividing a trait two areas share puts one piece on each
/// side of the boundary, and each area keeps the piece that is still its own
/// border. Demanding all of them would lose both.
#[derive(Clone, Debug, PartialEq)]
pub struct Standing {
    borders: Vec<Vec<CurveId>>,
    inside: DVec2,
}

impl Standing {
    pub fn new(borders: Vec<Vec<CurveId>>, inside: DVec2) -> Self {
        Self { borders, inside }
    }

    /// Which of these areas the name answers to, and nothing when the drawing
    /// no longer encloses it.
    ///
    /// An area may be bounded by more than the name asks for: rounding a
    /// corner puts a curve in the boundary that descends from no curve the
    /// name could have held, and an area nothing happened to must not be lost
    /// over it.
    ///
    /// Of the areas that answer, the one holding the place pointed at wins —
    /// the innermost, where a shape is drawn inside another. That comes first
    /// and not last: a chord laid across a circle a chord already cuts leaves
    /// the far half answering to the name exactly while the half that was
    /// clicked answers with one curve to spare, and the matter belongs where
    /// the user pointed. Only when no answering area holds that place — the
    /// drawing having moved out from under it, which is the whole point of
    /// naming — does the closest fit take it.
    pub fn found_in(&self, regions: &[Region]) -> Option<usize> {
        if self.borders.is_empty() {
            return None;
        }
        let answering: Vec<usize> = regions
            .iter()
            .enumerate()
            .filter(|(_, region)| {
                self.borders
                    .iter()
                    .all(|border| border.iter().any(|curve| region.bounds().contains(curve)))
            })
            .map(|(rank, _)| rank)
            .collect();

        answering
            .iter()
            .copied()
            .filter(|rank| regions[*rank].contains(self.inside))
            .max_by_key(|rank| regions[*rank].depth)
            .or_else(|| {
                answering
                    .iter()
                    .copied()
                    .min_by_key(|rank| regions[*rank].bounds().len())
            })
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
