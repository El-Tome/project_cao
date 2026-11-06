//! A selection dragged, a side moved across or a shape turned by hand.
//!
//! The geometry is the drawing's own — `cao_sketch` decides what the rest of
//! it does. What is here is the replay: which side, how far; which points,
//! about where, how far round.

use cao_sketch::{PointId, SegmentId};
use glam::DVec2;

use crate::outcome::Outcome;
use crate::state::PartState;

impl PartState {
    pub(crate) fn move_many(
        &mut self,
        sketch: usize,
        points: &[PointId],
        by: DVec2,
    ) -> Option<Outcome> {
        let scale = self.scale();
        let sketch = self.sketches.get_mut(sketch)?;
        // Every point of the selection travels by the same step and is held
        // there: a shape moved as a block keeps its shape, and the rest of the
        // drawing settles around it.
        let dropped: Vec<(PointId, DVec2)> = points
            .iter()
            .filter_map(|point| {
                sketch
                    .points()
                    .get(point.0)
                    .map(|place| (*point, *place + by))
            })
            .collect();
        sketch.settle_around_all(&dropped, scale);
        None
    }

    pub(crate) fn move_segment(
        &mut self,
        sketch: usize,
        segment: SegmentId,
        by: DVec2,
    ) -> Option<Outcome> {
        let scale = self.scale();
        self.sketches.get_mut(sketch)?.move_side(segment, by, scale);
        None
    }

    pub(crate) fn turn_shape(
        &mut self,
        sketch: usize,
        points: &[PointId],
        about: DVec2,
        angle: f64,
    ) -> Option<Outcome> {
        let scale = self.scale();
        self.sketches
            .get_mut(sketch)?
            .turn_shape(points, about, angle, scale);
        None
    }
}
