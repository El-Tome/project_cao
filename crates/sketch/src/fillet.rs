use std::f64::consts::{PI, TAU};

use crate::arc::ArcId;
use crate::constraints::{Constraint, DimensionTarget};
use crate::corner::piece_running_into;
use crate::naming::{Became, CurveId};
use crate::sketch::{PointId, SegmentId, Sketch};

/// Under this a fillet takes nothing off either side, and there is no curve to
/// put where the corner was.
const NOTHING_ROUNDED: f64 = 1e-9;

/// What a fillet left behind: the curve now standing where the corner was,
/// what is left of the two sides, and what rounding it cost.
#[derive(Clone, Debug, PartialEq)]
pub struct Rounded {
    pub arc: ArcId,
    /// The point left standing where the corner was, held on the line of each
    /// side, the same as a chamfer leaves. The radius and the tangencies
    /// already hold the curve; this is what a distance can later be measured
    /// from.
    corner: PointId,
    /// The stretch the curve took off each side, laid back in as construction,
    /// the same as a chamfer leaves. What they are for is said there.
    stretches: [SegmentId; 2],
    pub pieces: Vec<SegmentId>,
    /// Which pieces came out of which side, which `pieces` runs together. The
    /// curve now standing where the corner was is in neither: it descends from
    /// no curve.
    pub became: Became,
    pub rules_dropped: usize,
    pub values_dropped: usize,
}

impl Sketch {
    /// Rounds the corner two traits share into a curve of the radius given,
    /// tangent to both, pulling each side back to where the curve meets it.
    pub fn fillet(&mut self, first: SegmentId, second: SegmentId, radius: f64) -> Option<Rounded> {
        let (pivot, far_first, far_second, back, opening) =
            self.rounded_corner(first, second, radius)?;
        let held = self.values_at([first, second]);

        let at = self.point(pivot);
        let towards = |far| (self.point(far) - at).normalize_or_zero();
        let (out_first, out_second) = (towards(far_first), towards(far_second));

        let mut rounded = self.clone();
        let touches_first = rounded.add_point(at + out_first * back);
        let touches_second = rounded.add_point(at + out_second * back);
        let inwards = (out_first + out_second).normalize();
        let middle = rounded.add_point(at + inwards * (radius / (opening * 0.5).sin()));

        let mut cut = Rounded {
            arc: ArcId(0),
            corner: pivot,
            stretches: [SegmentId(0); 2],
            pieces: Vec::new(),
            became: Became::new(),
            rules_dropped: 0,
            values_dropped: 0,
        };
        for (side, back_to) in [(first, touches_first), (second, touches_second)] {
            let trimmed = rounded.trim(side, pivot, back_to)?;
            cut.became.push((
                CurveId::Segment(side),
                trimmed
                    .pieces
                    .iter()
                    .copied()
                    .map(CurveId::Segment)
                    .collect(),
            ));
            cut.pieces.extend(trimmed.pieces);
            cut.rules_dropped += trimmed.rules_dropped;
            cut.values_dropped += trimmed.values_dropped;
        }

        let centre = rounded.point(middle);
        let turns = (rounded.point(touches_second) - centre).to_angle()
            - (rounded.point(touches_first) - centre).to_angle();
        let (start, end) = match turns.rem_euclid(TAU) <= PI {
            true => (touches_first, touches_second),
            false => (touches_second, touches_first),
        };
        cut.arc = rounded.add_arc(middle, start, end);

        for touches in [touches_first, touches_second] {
            let Some(piece) = piece_running_into(&rounded, &cut.pieces, touches) else {
                continue;
            };
            rounded.add_constraint(Constraint::ArcTangent {
                arc: cut.arc,
                segment: piece,
                at: Some(touches),
            });
        }

        cut.stretches = [
            rounded.add_construction_segment(pivot, touches_first),
            rounded.add_construction_segment(pivot, touches_second),
        ];
        rounded.hold_corner(pivot, &cut.pieces, [touches_first, touches_second]);
        let saved = rounded.rehang(held, pivot, [far_first, far_second], cut.stretches);
        cut.values_dropped = cut.values_dropped.saturating_sub(saved);

        *self = rounded;
        Some(cut)
    }
    /// Whether a curve of this radius fits in the corner two traits share.
    pub fn fillet_fits(&self, first: SegmentId, second: SegmentId, radius: f64) -> bool {
        self.rounded_corner(first, second, radius).is_some()
    }

    /// The corner two traits share, how far back the curve pulls each side, and
    /// how wide the corner stands open — or nothing when no curve of that
    /// radius fits between them.
    fn rounded_corner(
        &self,
        first: SegmentId,
        second: SegmentId,
        radius: f64,
    ) -> Option<(PointId, PointId, PointId, f64, f64)> {
        let (pivot, far_first, far_second) = self.shared_corner(first, second)?;
        let opening = self.opening_at(pivot, far_first, far_second);
        let back = radius / (opening * 0.5).tan();
        if radius <= NOTHING_ROUNDED || back <= NOTHING_ROUNDED || !back.is_finite() {
            return None;
        }
        let at = self.point(pivot);
        for far in [far_first, far_second] {
            if back >= at.distance(self.point(far)) {
                return None;
            }
        }
        Some((pivot, far_first, far_second, back, opening))
    }
}

impl Rounded {
    /// The one value a fillet is given, paired with what carries it. The two
    /// tangencies hold the rest, so the radius is the whole of what a fillet
    /// writes down.
    pub fn typed(&self, radius: f64) -> Vec<(DimensionTarget, f64)> {
        vec![(DimensionTarget::ArcRadius(self.arc), radius)]
    }
}

#[cfg(test)]
mod tests;
