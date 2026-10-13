//! A trait made to run the way of one of the sketch's axes, or square to it:
//! the shape it belongs to turns whole, as a hand would turn it.

use super::Sketch;
use super::turned_whole::quarter_turn_at_most;
use crate::constraints::Constraint;

impl Sketch {
    /// Turns the shape `rule` speaks of until its trait runs the way the rule
    /// asks, before the solver sees it. Left to the solver, the correction
    /// spreads over every free point and a rectangle comes out bent, or
    /// folded flat when the side named stood nearly square to the axis.
    ///
    /// The shape turns about the one place that holds it — the origin, a
    /// fixed point — or, held by nothing, about its point nearest the origin,
    /// as a value typed ranks them. Held in two places, it cannot turn whole,
    /// and is left to the solver.
    pub(super) fn turn_onto_the_axis(&mut self, rule: Constraint) {
        // Lying on the axis is not a turn: the trait is carried onto it.
        if matches!(rule, Constraint::AxisCollinear { .. }) {
            return;
        }
        let Some((segment, axis)) = rule.along_an_axis() else {
            return;
        };
        let wanted = axis.direction();
        let Some(line) = self.segments().get(segment.0).copied() else {
            return;
        };
        let running = self.point(line.end) - self.point(line.start);
        if running.length_squared() < 1e-18 {
            return;
        }
        let angle = quarter_turn_at_most(running.angle_to(wanted));
        let stops = self.held_in_direction(None);
        self.turn_whole(segment, &stops, angle, None);
    }
}
