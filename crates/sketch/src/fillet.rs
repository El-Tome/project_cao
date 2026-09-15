use std::f64::consts::{PI, TAU};

use crate::arc::ArcId;
use crate::constraints::Constraint;
use crate::sketch::{Element, PointId, SegmentId, Sketch};

/// Under this a fillet takes nothing off either side, and there is no curve to
/// put where the corner was.
const NOTHING_ROUNDED: f64 = 1e-9;

/// What a fillet left behind: the curve now standing where the corner was,
/// what is left of the two sides, and what rounding it cost.
#[derive(Clone, Debug, PartialEq)]
pub struct Rounded {
    pub arc: ArcId,
    pub pieces: Vec<SegmentId>,
    pub rules_dropped: usize,
    pub values_dropped: usize,
}

impl Sketch {
    /// Rounds the corner two traits share into a curve of the radius given,
    /// tangent to both, pulling each side back to where the curve meets it.
    pub fn fillet(&mut self, first: SegmentId, second: SegmentId, radius: f64) -> Option<Rounded> {
        let (pivot, far_first, far_second, back, opening) =
            self.rounded_corner(first, second, radius)?;

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
            pieces: Vec::new(),
            rules_dropped: 0,
            values_dropped: 0,
        };
        for (side, back_to) in [(first, touches_first), (second, touches_second)] {
            let trimmed = rounded.trim(side, pivot, back_to)?;
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
            let Some(piece) = piece_on(&rounded, &cut.pieces, touches) else {
                continue;
            };
            rounded.add_constraint(Constraint::ArcTangent {
                arc: cut.arc,
                segment: piece,
                at: Some(touches),
            });
        }

        if rounded.nothing_leans_on(pivot) {
            rounded.erase(Element::Point(pivot));
        }

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

/// Which of the pieces left of the two sides runs into this point.
fn piece_on(sketch: &Sketch, pieces: &[SegmentId], point: PointId) -> Option<SegmentId> {
    pieces.iter().copied().find(|piece| {
        let side = sketch.segments()[piece.0];
        side.start == point || side.end == point
    })
}

#[cfg(test)]
mod tests;
