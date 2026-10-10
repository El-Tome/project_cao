//! The straight line a click names: a trait of the drawing, or one of the
//! sketch's own two axes. What the mirror lays a copy across, and what a
//! rectangular pattern runs along.

use glam::DVec2;
use serde::{Deserialize, Serialize};

use crate::constraints::SketchAxis;
use crate::sketch::{SegmentId, Sketch};

/// Below this a trait runs nowhere, and names no direction.
const NO_DIRECTION: f64 = 1e-9;

/// A straight line of the drawing, named by a click.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChosenAxis {
    /// A trait of the drawing, which follows it when it moves.
    Trait(SegmentId),
    /// One of the sketch's own two axes.
    Sketch(SketchAxis),
}

impl Sketch {
    /// A point the axis runs through, and the direction it runs in.
    pub(crate) fn axis_line(&self, axis: ChosenAxis) -> Option<(DVec2, DVec2)> {
        match axis {
            ChosenAxis::Sketch(SketchAxis::U) => Some((DVec2::ZERO, DVec2::X)),
            ChosenAxis::Sketch(SketchAxis::V) => Some((DVec2::ZERO, DVec2::Y)),
            ChosenAxis::Trait(id) => {
                if self.is_erased_segment(id) || id.0 >= self.segments().len() {
                    return None;
                }
                let (start, end) = self.endpoints(id);
                let along = end - start;
                (along.length() > NO_DIRECTION).then(|| (start, along.normalize()))
            }
        }
    }
}
