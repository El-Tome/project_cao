//! A curve seen in a surface's parameters: `(s, t)` on a plane, `(θ, h)` on a
//! cylinder, with `θ` unwrapped along the trace so that it never jumps by a
//! turn in the middle of one.

use glam::DVec2;

use super::curve::Meet;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Trace {
    /// Straight from `from` to `to`: a line on a plane, a ruling or a circle of
    /// a cylinder on that cylinder.
    Segment { from: DVec2, to: DVec2 },
    /// Round `center` at `radius`, from the angle `start` turning by `sweep`,
    /// signed as the turn goes: a circle on a plane square to its axis.
    Round {
        center: DVec2,
        radius: f64,
        start: f64,
        sweep: f64,
    },
    /// The curve two perpendicular cylinders meet along, seen on the first of
    /// them or on the second, between two of its parameters.
    Graph {
        meet: Meet,
        on_first: bool,
        from: f64,
        to: f64,
    },
}

impl Trace {
    /// The point at `u`, from nought at the trace's start to one at its end,
    /// with its first and second derivatives with respect to `u`.
    pub fn at(&self, u: f64) -> [DVec2; 3] {
        match *self {
            Trace::Segment { from, to } => [from + (to - from) * u, to - from, DVec2::ZERO],
            Trace::Round {
                center,
                radius,
                start,
                sweep,
            } => {
                let angle = start + sweep * u;
                let radial = DVec2::from_angle(angle);
                [
                    center + radial * radius,
                    radial.perp() * radius * sweep,
                    -radial * radius * sweep * sweep,
                ]
            }
            Trace::Graph {
                meet,
                on_first,
                from,
                to,
            } => {
                let span = to - from;
                let [point, first, second] = meet.seen_on(on_first, from + span * u);
                [point, first * span, second * span * span]
            }
        }
    }

    pub fn start(&self) -> DVec2 {
        self.at(0.0)[0]
    }

    pub fn end(&self) -> DVec2 {
        self.at(1.0)[0]
    }

    /// The same trace run the other way.
    pub fn reversed(&self) -> Trace {
        match *self {
            Trace::Segment { from, to } => Trace::Segment { from: to, to: from },
            Trace::Round {
                center,
                radius,
                start,
                sweep,
            } => Trace::Round {
                center,
                radius,
                start: start + sweep,
                sweep: -sweep,
            },
            Trace::Graph {
                meet,
                on_first,
                from,
                to,
            } => Trace::Graph {
                meet,
                on_first,
                from: to,
                to: from,
            },
        }
    }
}

#[cfg(test)]
mod tests;
