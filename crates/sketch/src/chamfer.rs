use serde::{Deserialize, Serialize};

use crate::constraints::DimensionTarget;
use crate::naming::{Became, CurveId};
use crate::sketch::{PointId, SegmentId, Sketch};

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
    /// The two points the cut runs between: the one on the first side, then the
    /// one on the second. Which is which is what lets a distance typed for one
    /// side be written against that side.
    across: [PointId; 2],
    /// The point left standing where the corner was, held on the line of each
    /// side so it follows them to their new crossing. It is what the typed
    /// distances are measured from, and erasing it takes them with it.
    corner: PointId,
    /// The stretch the cut took off the first side, laid back in as
    /// construction. Only an angled chamfer has one: an angle needs two traits
    /// to be read between, and the one the angle was measured from is the one
    /// the cut took away. A distance needs no trait, only the corner.
    extension: Option<SegmentId>,
    pub pieces: Vec<SegmentId>,
    /// Which pieces came out of which side, which `pieces` runs together. The
    /// straight cut laid across the corner is in neither: it stands where the
    /// corner was and descends from no curve.
    pub became: Became,
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
        let lengths = self.lengths_of([first, second]);

        let mut cut = self.clone();
        let at = cut.point(pivot);
        let along = |far, by: f64| at + (self.point(far) - at).normalize_or_zero() * by;
        let (starts_at, ends_at) = (along(far_first, back_first), along(far_second, back_second));
        let start = cut.add_point(starts_at);
        let end = cut.add_point(ends_at);

        let mut chamfered = Chamfered {
            cut: SegmentId(0),
            across: [start, end],
            corner: pivot,
            extension: None,
            pieces: Vec::new(),
            became: Became::new(),
            rules_dropped: 0,
            values_dropped: 0,
        };
        for (side, back_to) in [(first, start), (second, end)] {
            let trimmed = cut.trim(side, pivot, back_to)?;
            chamfered.became.push((
                CurveId::Segment(side),
                trimmed
                    .pieces
                    .iter()
                    .copied()
                    .map(CurveId::Segment)
                    .collect(),
            ));
            chamfered.pieces.extend(trimmed.pieces);
            chamfered.rules_dropped += trimmed.rules_dropped;
            chamfered.values_dropped += trimmed.values_dropped;
        }
        chamfered.cut = cut.add_segment(start, end);
        if matches!(mode, Chamfer::Angled { .. }) {
            chamfered.extension = Some(cut.add_construction_segment(pivot, start));
        }
        cut.hold_corner(pivot, &chamfered.pieces, [start, end]);
        let saved = cut.rehang(lengths, pivot, [far_first, far_second]);
        chamfered.values_dropped = chamfered.values_dropped.saturating_sub(saved);

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
}

impl Chamfered {
    /// Each value the mode was given, paired with what should carry it in the
    /// drawing — the whole of what a chamfer writes down, and nothing else.
    ///
    /// A distance is read from the corner the cut took away, which is why that
    /// point is kept: measured from there, `20 x 20` reads back as `20` and
    /// `20` rather than as one slanted `28.28` at `45` degrees, and the
    /// two-distance mode can be read back at all. An angle needs two traits to
    /// lie between, so it is read against the stretch of the first side the cut
    /// removed, laid back in as construction alongside it.
    ///
    /// The angle the two sides themselves stand at is nobody's typed value:
    /// writing it down would add a rule no one asked for, and over-constrain a
    /// drawing already dimensioned.
    ///
    /// The values come back in whatever unit the mode was given in, since this
    /// only says which target carries which of them. The sketch is cut in its
    /// own units and dimensions are recorded in millimetres, so the caller
    /// holding the millimetres is the one to ask.
    pub fn typed(&self, mode: Chamfer) -> Vec<(DimensionTarget, f64)> {
        let [on_first, on_second] = self.across;
        let from_corner = |to| DimensionTarget::Distance {
            from: self.corner,
            to,
        };
        match mode {
            Chamfer::Equal(reach) => vec![
                (from_corner(on_first), reach),
                (from_corner(on_second), reach),
            ],
            Chamfer::Sided { first, second } => vec![
                (from_corner(on_first), first),
                (from_corner(on_second), second),
            ],
            // `chamfer` lays the stretch for this mode and no other, so the
            // two are one decision; `an_angled_chamfer_writes_both_the_values_it_was_given`
            // is what holds them together.
            Chamfer::Angled { along, degrees } => match self.extension {
                Some(extension) => vec![
                    (from_corner(on_first), along),
                    (
                        DimensionTarget::Angle {
                            first: extension,
                            second: self.cut,
                        },
                        degrees,
                    ),
                ],
                None => vec![(from_corner(on_first), along)],
            },
        }
    }
}

#[cfg(test)]
mod tests;
