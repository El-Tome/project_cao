//! A trait made to run the way of one of the sketch's axes, or square to it:
//! the shape it belongs to turns whole, as a hand would turn it.

use std::f64::consts::{FRAC_PI_2, PI};

use glam::DVec2;

use super::{PointId, Sketch};
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
        if angle.abs() < 1e-12 {
            return;
        }
        let shape = self.shape_through(&[line.start, line.end]);
        let Some(about) = self.pivot_onto_the_axis(&shape) else {
            return;
        };
        let turn = DVec2::from_angle(angle);
        for point in shape {
            let place = about + turn.rotate(self.point(point) - about);
            self.move_point(point, place);
        }
    }

    fn pivot_onto_the_axis(&self, shape: &[PointId]) -> Option<DVec2> {
        let stays = self.points_that_stay();
        let mut held = shape
            .iter()
            .filter(|point| stays[point.0])
            .map(|point| self.point(*point));
        if let Some(first) = held.next() {
            return held
                .all(|place| place.distance(first) < 1e-9)
                .then_some(first);
        }
        self.nearest_the_origin(shape)
            .map(|point| self.point(point))
    }
}

/// The turn, taken the short way: a trait runs the way of a line whichever end
/// leads, so no turn needs to be more than a quarter.
fn quarter_turn_at_most(angle: f64) -> f64 {
    let angle = angle.rem_euclid(PI);
    if angle > FRAC_PI_2 { angle - PI } else { angle }
}
