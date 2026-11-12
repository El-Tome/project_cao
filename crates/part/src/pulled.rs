//! A point or a selection dragged, a side moved across or a shape turned by
//! hand.
//!
//! The geometry is the drawing's own — `cao_sketch` decides what the rest of
//! it does. What is here is the replay: which side, how far; which points,
//! about where, how far round.

use cao_sketch::{PointId, SegmentId, Support};
use glam::DVec2;

use crate::outcome::Outcome;
use crate::state::{PartState, hold};

impl PartState {
    pub(crate) fn move_point(
        &mut self,
        sketch: usize,
        point: PointId,
        position: DVec2,
        merged_into: Option<PointId>,
        on: &[Support],
        let_go: bool,
    ) -> Option<Outcome> {
        let scale = self.scale();
        let sketch = self.sketches.get_mut(sketch)?;
        if let_go {
            sketch.let_go(point);
        }
        // Moving a point by hand must not break the values already
        // given, so the drawing settles again around it, as the pull
        // allows: stretching first, turning only when it cannot. It is
        // the drawing the gesture was shown on, read and settled the
        // same way; the drop's holds come after, already met at the
        // place the point landed — laid before, they would pull on a
        // shape still turning towards it.
        let pull = sketch.pull(point, scale);
        sketch.settle_pulled(&pull, position, scale);
        hold(sketch, point, on);
        // A drop the drawing cannot take joins nothing, as one that
        // stopped short of the hand does.
        if let Some(kept) = merged_into {
            sketch.join_points(kept, point, scale);
        }
        None
    }

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
