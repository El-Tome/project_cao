use glam::DVec2;
use serde::{Deserialize, Serialize};

use crate::axis::ChosenAxis;
use crate::duplicating::Duplicated;
use crate::element::Element;
use crate::sketch::{PointId, Sketch};

/// One direction of a rectangular pattern: how far apart the copies stand, and
/// how many of them there are with the original among them.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Repeats {
    pub step: f64,
    pub count: usize,
}

impl Repeats {
    /// A direction is walked at least once: the original stands in it whatever
    /// was asked for.
    fn at_least_once(self) -> Self {
        Self {
            count: self.count.max(1),
            ..self
        }
    }

    fn piles_up(self) -> bool {
        self.count > 1 && self.step.abs() <= NO_STEP
    }
}

/// Below this a step leaves the copy on the one before it, and the pattern is a
/// pile nobody can tell apart.
const NO_STEP: f64 = 1e-9;

impl Sketch {
    /// Repeats what is held around a point of the drawing, one step further
    /// round for each copy.
    ///
    /// The count is how many stand there in the end, the original among them,
    /// so a count of four lays three. Nothing when there is nothing to lay: a
    /// count of one, or a step that would put every copy on top of the last.
    pub fn pattern_around(
        &mut self,
        of: &[Element],
        centre: PointId,
        degrees: f64,
        count: usize,
    ) -> Option<Duplicated> {
        let at = *self.points().get(centre.0)?;
        if self.is_erased_point(centre) || count < 2 {
            return None;
        }
        let step = degrees.to_radians();
        if step.abs() <= NO_STEP {
            return None;
        }

        let mut made = Duplicated::default();
        for rank in 1..count {
            let turn = DVec2::from_angle(step * rank as f64);
            let laid = self.duplicate(of, |place| at + turn.rotate(place - at));
            made.points.extend(laid.points);
            made.segments.extend(laid.segments);
            made.circles.extend(laid.circles);
            made.arcs.extend(laid.arcs);
        }
        Some(made)
    }

    /// Repeats what is held in rows square to a direction of the drawing: so
    /// far along it, so far across it, so many each way.
    pub fn pattern_along(
        &mut self,
        of: &[Element],
        direction: ChosenAxis,
        along: Repeats,
        across: Repeats,
    ) -> Option<Duplicated> {
        let (_, forward) = self.axis_line(direction)?;
        let (along, across) = (along.at_least_once(), across.at_least_once());
        if along.count * across.count < 2 || along.piles_up() || across.piles_up() {
            return None;
        }
        let sideways = forward.perp();

        let mut made = Duplicated::default();
        for row in 0..across.count {
            for column in 0..along.count {
                if (row, column) == (0, 0) {
                    continue;
                }
                let offset =
                    forward * along.step * column as f64 + sideways * across.step * row as f64;
                let laid = self.duplicate(of, |place| place + offset);
                made.points.extend(laid.points);
                made.segments.extend(laid.segments);
                made.circles.extend(laid.circles);
                made.arcs.extend(laid.arcs);
            }
        }
        Some(made)
    }
}

#[cfg(test)]
mod tests;
