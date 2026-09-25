//! A side moved across or a shape turned by hand.
//!
//! The geometry is the drawing's own — `cao_sketch` decides what the rest of
//! it does. What is here is the replay: which side, how far; which points,
//! about where, how far round.

use cao_sketch::{PointId, SegmentId};
use glam::DVec2;

use crate::outcome::Outcome;
use crate::state::PartState;

impl PartState {
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
