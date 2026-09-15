use glam::DVec2;

use crate::duplicating::Duplicated;
use crate::element::Element;
use crate::sketch::{PointId, Sketch};

/// Below this a step turns the copy onto the one before it, and the pattern is
/// a pile nobody can tell apart.
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
}

#[cfg(test)]
mod tests;
