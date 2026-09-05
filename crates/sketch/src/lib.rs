//! The sketch model: 2D geometry drawn on a work plane, and the rules that
//! apply a typed value to it. No rendering and no UI, so the same model backs
//! any front-end and can be saved straight into a part file.

mod constraints;
mod plane;
mod sketch;

pub use constraints::{Dimension, DimensionTarget, Freedom};
pub use plane::WorkPlane;
pub use sketch::{Circle, CircleId, LengthOutcome, PointId, Segment, SegmentId, Sketch};
