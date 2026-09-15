use std::f64::consts::TAU;

use serde::{Deserialize, Serialize};

use crate::sketch::{Element, PointId, SegmentId, Sketch};

/// Under this a side gives nothing to a chamfer, and the cut would run from
/// the corner to itself.
const NOTHING_TAKEN: f64 = 1e-9;

/// Which of the three ways of saying a chamfer the tool is asking for. The
/// values themselves arrive separately, as they are typed.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChamferMode {
    /// One distance, taken the same way along both sides.
    #[default]
    Equal,
    /// A distance along the first side, and the angle the cut leaves it at.
    Angled,
    /// A distance of its own along each side.
    Sided,
}

impl ChamferMode {
    /// How many values the mode needs typed.
    pub fn wants(self) -> usize {
        match self {
            Self::Equal => 1,
            Self::Angled | Self::Sided => 2,
        }
    }

    /// The chamfer the mode makes of the values typed for it.
    pub fn of(self, first: f64, second: f64) -> Chamfer {
        match self {
            Self::Equal => Chamfer::Equal(first),
            Self::Angled => Chamfer::Angled {
                along: first,
                degrees: second,
            },
            Self::Sided => Chamfer::Sided { first, second },
        }
    }
}

/// How much of each side of a corner a chamfer takes.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Chamfer {
    /// One distance, taken the same way along both sides.
    Equal(f64),
    /// A distance along the first side, and the angle the cut leaves it at.
    Angled { along: f64, degrees: f64 },
    /// A distance of its own along each side.
    Sided { first: f64, second: f64 },
}

/// What a chamfer left behind: the straight cut laid across the corner, what
/// is left of the two sides, and what the cut cost.
#[derive(Clone, Debug, PartialEq)]
pub struct Chamfered {
    pub cut: SegmentId,
    pub pieces: Vec<SegmentId>,
    pub rules_dropped: usize,
    pub values_dropped: usize,
}

impl Sketch {
    /// Cuts the corner two traits share with a straight line, pulling each of
    /// them back from the corner by what the mode asks of it.
    pub fn chamfer(
        &mut self,
        first: SegmentId,
        second: SegmentId,
        mode: Chamfer,
    ) -> Option<Chamfered> {
        let (pivot, far_first, far_second, back_first, back_second) =
            self.corner_for(first, second, mode)?;

        let mut cut = self.clone();
        let at = cut.point(pivot);
        let along = |far, by: f64| at + (self.point(far) - at).normalize_or_zero() * by;
        let (starts_at, ends_at) = (along(far_first, back_first), along(far_second, back_second));
        let start = cut.add_point(starts_at);
        let end = cut.add_point(ends_at);

        let mut chamfered = Chamfered {
            cut: SegmentId(0),
            pieces: Vec::new(),
            rules_dropped: 0,
            values_dropped: 0,
        };
        for (side, back_to) in [(first, start), (second, end)] {
            let trimmed = cut.trim(side, pivot, back_to)?;
            chamfered.pieces.extend(trimmed.pieces);
            chamfered.rules_dropped += trimmed.rules_dropped;
            chamfered.values_dropped += trimmed.values_dropped;
        }
        chamfered.cut = cut.add_segment(start, end);
        if cut.nothing_leans_on(pivot) {
            cut.erase(Element::Point(pivot));
        }

        *self = cut;
        Some(chamfered)
    }

    /// Whether a chamfer can be cut at all: the two traits meet, and neither
    /// side is asked for more than it has to give.
    pub fn chamfer_fits(&self, first: SegmentId, second: SegmentId, mode: Chamfer) -> bool {
        self.corner_for(first, second, mode).is_some()
    }

    /// The corner two traits share and how far back along each side the mode
    /// pulls it, or nothing when there is no chamfer to cut.
    fn corner_for(
        &self,
        first: SegmentId,
        second: SegmentId,
        mode: Chamfer,
    ) -> Option<(PointId, PointId, PointId, f64, f64)> {
        let (pivot, far_first, far_second) = self.shared_corner(first, second)?;
        let (back_first, back_second) = self.taken_by(mode, pivot, far_first, far_second)?;
        for (far, back) in [(far_first, back_first), (far_second, back_second)] {
            if back <= NOTHING_TAKEN || back >= self.point(pivot).distance(self.point(far)) {
                return None;
            }
        }
        Some((pivot, far_first, far_second, back_first, back_second))
    }

    /// How far back along each side the mode asks the corner to be pulled.
    ///
    /// An angle is read at the point the cut meets the first side, between the
    /// cut and the stretch of that side still running to the corner — the
    /// angle of the triangle the cut closes. The other two are the corner's own
    /// and what is left of half a turn, so the sine rule gives the second
    /// distance.
    fn taken_by(
        &self,
        mode: Chamfer,
        pivot: PointId,
        far_first: PointId,
        far_second: PointId,
    ) -> Option<(f64, f64)> {
        match mode {
            Chamfer::Equal(reach) => Some((reach, reach)),
            Chamfer::Sided { first, second } => Some((first, second)),
            Chamfer::Angled { along, degrees } => {
                let opening = degrees.to_radians();
                let corner = self.opening_at(pivot, far_first, far_second);
                let closing = (corner + opening).sin();
                match closing.abs() <= NOTHING_TAKEN {
                    true => None,
                    false => Some((along, along * opening.sin() / closing)),
                }
            }
        }
    }

    /// Whether the drawing has anything left standing on a point. A corner cut
    /// off leaves its own point behind, holding nothing up.
    pub(crate) fn nothing_leans_on(&self, point: PointId) -> bool {
        !self
            .live_segments()
            .any(|(_, segment)| segment.start == point || segment.end == point)
            && !self
                .live_circles()
                .any(|(_, circle)| circle.center == point)
            && self.arcs_leaning_on(point).is_empty()
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
}

#[cfg(test)]
mod tests;
