//! Taking a stretch out of an ellipse, between two of the points sitting on
//! it.
//!
//! What a cut leaves is the same ellipse with a stretch taken away: it keeps
//! its axes, its rules and its values, and every tool that reads an ellipse
//! reads what is left without being told. A cut in the middle of a stretch
//! already drawn leaves two, and the second is an ellipse of its own standing
//! on the very same axes — two pieces of one curve, held on it by the axes
//! they share rather than by a rule anybody could take away.

use std::f64::consts::TAU;

use glam::DVec2;

use super::{NO_LENGTH, ON_THE_TRAIT};

/// How far round the turn a place may stand past the end of a stretch and
/// still be that end. A turn rather than a length: what is compared is how far
/// round the curve two places stand, and the two ends of a stretch are read
/// off points the drawing already holds on it.
const ROUND_THE_CURVE: f64 = 1e-9;
use crate::ellipse::{Ellipse, EllipseId};
use crate::sketch::{Element, PointId, Sketch};

/// What a cut left standing of an ellipse: the pieces still drawn, the one the
/// ellipse itself became first.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct EllipseTrimmed {
    pub pieces: Vec<EllipseId>,
}

impl Sketch {
    /// The points sitting on the stretch of an ellipse that is drawn, each
    /// with how far along that stretch it sits, in order.
    ///
    /// Never the sketch origin, which every drawing owns and nobody placed,
    /// nor the centre, which is not on the curve at all. The ends of the
    /// stretch come first and last, since a cut may take the piece that ends
    /// at either of them.
    fn sitting_along_the_ellipse(&self, id: EllipseId) -> Vec<(f64, PointId)> {
        let Some(ellipse) = self.ellipses().get(id.0).copied() else {
            return Vec::new();
        };
        let drawn = self.ellipse_draft(id);
        if drawn.first.length() < NO_LENGTH || drawn.second < NO_LENGTH {
            return Vec::new();
        }
        let (from, sweep) = self.ellipse_run(id);

        let mut sitting: Vec<(f64, PointId)> = self
            .live_points()
            .filter_map(|(point, place)| {
                let on_the_curve = !self.is_origin(point)
                    && point != ellipse.center
                    && drawn.distance(place) <= ON_THE_TRAIT;
                if !on_the_curve {
                    return None;
                }
                let along = (drawn.turn_on(place) - from).rem_euclid(TAU);
                (along <= sweep + ROUND_THE_CURVE).then_some((along, point))
            })
            .collect();
        sitting.sort_by(|first, second| first.0.total_cmp(&second.0));
        sitting
    }

    /// The stretch of an ellipse a click falls in: the two points it runs
    /// between, the first being the one the stretch leaves the way the curve
    /// runs.
    ///
    /// Nothing when the curve carries fewer than two points to run between,
    /// which is the one case where a cut takes the whole of what is drawn.
    pub fn ellipse_stretch_at(&self, id: EllipseId, at: DVec2) -> Option<(PointId, PointId)> {
        let sitting = self.sitting_along_the_ellipse(id);
        if sitting.len() < 2 {
            return None;
        }
        let drawn = self.ellipse_draft(id);
        let (from, sweep) = self.ellipse_run(id);
        // Read where the click lands on what is drawn, as the click itself was
        // read: past an end that is the end, not the far side of the curve.
        let along = (drawn.turn_on(self.place_on_ellipse(id, at)) - from).rem_euclid(TAU);

        let opens = sitting.iter().rposition(|(at, _)| *at <= along);
        match self.ellipse_ends(id) {
            // Nothing cut it yet: the curve closes on itself, so past the last
            // point the stretch the click fell in is the one running round to
            // the first.
            None => {
                let opens = opens.unwrap_or(sitting.len() - 1);
                Some((sitting[opens].1, sitting[(opens + 1) % sitting.len()].1))
            }
            // A stretch already cut has ends, and a click past the last point
            // or before the first falls outside it.
            Some(_) => {
                let opens = opens?;
                let closes = sitting.get(opens + 1)?;
                (along <= sweep).then_some((sitting[opens].1, closes.1))
            }
        }
    }

    /// Takes the stretch running from the first named point round to the
    /// second out of an ellipse, and leaves what is left of it drawn.
    ///
    /// The order of the two says which of the two stretches between them goes,
    /// as it does for a circle: the one running the way the curve runs.
    ///
    /// Named nothing to cut between — a curve carrying fewer than two points —
    /// the stretch "between two points" is the whole of what is drawn, and the
    /// ellipse is taken away entire.
    pub fn trim_ellipse(
        &mut self,
        id: EllipseId,
        between: Option<(PointId, PointId)>,
    ) -> Option<EllipseTrimmed> {
        if self.is_erased_ellipse(id) {
            return None;
        }
        let ellipse = *self.ellipses().get(id.0)?;
        let Some((from, to)) = between else {
            self.erase(Element::Ellipse(id));
            return Some(EllipseTrimmed::default());
        };
        if !self.is_a_stretch_of_the_ellipse(id, from, to) {
            return None;
        }

        let kept = self.what_is_left(id, from, to);
        let mut pieces = Vec::new();
        for (rank, (opens, closes)) in kept.iter().enumerate() {
            match rank {
                0 => {
                    self.ellipses[id.0].drawn = Some((*opens, *closes));
                    pieces.push(id);
                }
                // A cut in the middle leaves a second piece of the same curve,
                // standing on the very same axes.
                _ => {
                    self.ellipses.push(Ellipse {
                        drawn: Some((*opens, *closes)),
                        ..ellipse
                    });
                    pieces.push(EllipseId(self.ellipses.len() - 1));
                }
            }
        }
        if pieces.is_empty() {
            self.erase(Element::Ellipse(id));
        }
        Some(EllipseTrimmed { pieces })
    }

    /// The stretches left drawn once the one between the two points is taken
    /// away, each as the two points it runs between.
    fn what_is_left(&self, id: EllipseId, from: PointId, to: PointId) -> Vec<(PointId, PointId)> {
        let Some((opens, closes)) = self.ellipse_ends(id) else {
            // A whole curve is left as the one stretch running the other way
            // round, from the far point back to the near one.
            return vec![(to, from)];
        };
        let mut left = Vec::new();
        if opens != from {
            left.push((opens, from));
        }
        if to != closes {
            left.push((to, closes));
        }
        left
    }

    /// Whether a stretch named between two points is one this ellipse can give
    /// up: two points that are not the same one, both still drawn, and both
    /// sitting on the stretch that is drawn.
    ///
    /// Asked again here rather than trusted from the click, because the two
    /// points are recorded once and replayed afterwards: an earlier step
    /// edited since can have carried them off the curve.
    /// The two are asked for in the order the curve runs as well: a pair that
    /// has crossed over names no stretch of what is drawn, and a replay that
    /// took one at its word conjured a second piece out of a cut.
    fn is_a_stretch_of_the_ellipse(&self, id: EllipseId, from: PointId, to: PointId) -> bool {
        let sitting = self.sitting_along_the_ellipse(id);
        let along = |point: PointId| sitting.iter().find(|(_, sits)| *sits == point);
        let (Some(near), Some(far)) = (along(from), along(to)) else {
            return false;
        };
        from != to && (self.ellipse_ends(id).is_none() || near.0 <= far.0)
    }

    /// Whether another ellipse is drawn on the same axes: a piece of the very
    /// curve this one is a piece of, which a cut in the middle left behind.
    pub(crate) fn shares_its_axes(&self, id: EllipseId) -> bool {
        let Some(ellipse) = self.ellipses().get(id.0) else {
            return false;
        };
        self.live_ellipses()
            .any(|(other, held)| other != id && held.first == ellipse.first)
    }
}

#[cfg(test)]
mod tests;
