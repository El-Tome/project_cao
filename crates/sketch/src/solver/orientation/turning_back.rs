//! A group turned back the way it was pointing once the drawing has settled,
//! unless turning it back breaks what the settle made true.

use glam::DVec2;

use super::WayRound;
use crate::sketch::{PointId, Sketch};
use crate::solver::TOLERANCE;

impl Sketch {
    /// Turns each group back the way it was pointing. A rigid turn about the
    /// point the group turns about leaves every dimension of it free to turn
    /// exactly as it found it — that is what "free to turn" means — so this
    /// straightens the drawing without touching what it measures.
    ///
    /// That point stays for good, so where it was read before the settle is
    /// where it still is.
    ///
    /// Free to turn is read to the first order, and a rule tying the group to
    /// a point that stays can stand so that the first order sees nothing — a
    /// point held on an arc, in line with the arc's centre and the origin. A
    /// turn that leaves untrue a drawing the settle had made true is not kept:
    /// the drawing settled is worth more than the way up it was drawn.
    pub(in crate::solver) fn hold_orientations(
        &mut self,
        held: &[WayRound],
        millimeters_per_unit: f64,
        scale: f64,
    ) {
        if held.is_empty() {
            return;
        }
        let pinned = self.pinned_points();
        let mut settled = None;

        for way in held {
            let span = self.point(way.to) - self.point(way.from);
            if span.length() < 1e-6 {
                continue;
            }
            let drift = wrap(span.to_angle() - way.angle);
            if drift.abs() < 1e-6 {
                continue;
            }
            let settled = *settled
                .get_or_insert_with(|| self.worst_error(millimeters_per_unit, scale) < TOLERANCE);
            let before = self.points().to_vec();
            let turn = DVec2::from_angle(-drift);
            for index in way.members.iter().copied() {
                if pinned[index] {
                    continue;
                }
                let moved = way.about + turn.rotate(self.point(PointId(index)) - way.about);
                self.place_point(PointId(index), moved);
            }
            if settled && self.worst_error(millimeters_per_unit, scale) >= TOLERANCE {
                for index in way.members.iter().copied() {
                    self.place_point(PointId(index), before[index]);
                }
            }
        }
    }
}

/// An angle brought back into [-pi, pi], so a drift either side of a turn reads
/// as the small angle it is.
fn wrap(mut angle: f64) -> f64 {
    while angle > std::f64::consts::PI {
        angle -= std::f64::consts::TAU;
    }
    while angle < -std::f64::consts::PI {
        angle += std::f64::consts::TAU;
    }
    angle
}
