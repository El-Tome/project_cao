//! A circle, an arc or an ellipse drawn to another size by hand, about the centre it
//! already has.
//!
//! The geometry is the drawing's own — `cao_sketch` decides what a new size
//! does to the rest of it. What is here is the replay: which curve, and how
//! far out it now stands.

use cao_sketch::{ArcId, CircleId, EllipseId};

use crate::outcome::Outcome;
use crate::state::PartState;

impl PartState {
    pub(crate) fn resize_circle(
        &mut self,
        sketch: usize,
        circle: CircleId,
        reach: f64,
    ) -> Option<Outcome> {
        let scale = self.scale();
        self.sketches
            .get_mut(sketch)?
            .resize_circle(circle, reach, scale);
        None
    }

    /// An arc holds no size of its own — it is read off the end it starts at —
    /// so this walks its two ends out to the new reach.
    pub(crate) fn resize_arc(&mut self, sketch: usize, arc: ArcId, reach: f64) -> Option<Outcome> {
        let scale = self.scale();
        self.sketches.get_mut(sketch)?.resize_arc(arc, reach, scale);
        None
    }

    /// An ellipse is scaled whole about its centre, its first axis reaching
    /// that far once it is done.
    pub(crate) fn resize_ellipse(
        &mut self,
        sketch: usize,
        ellipse: EllipseId,
        reach: f64,
    ) -> Option<Outcome> {
        let scale = self.scale();
        self.sketches
            .get_mut(sketch)?
            .resize_ellipse(ellipse, reach, scale);
        None
    }
}
