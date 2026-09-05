//! The sketch model: 2D geometry drawn on a work plane, and the rule that
//! applies a typed length to it. No rendering and no UI, so the same model
//! backs any front-end and can be saved straight into a part file.

mod plane;
mod sketch;

pub use plane::WorkPlane;
pub use sketch::{Dimension, LengthOutcome, PointId, Segment, SegmentId, Sketch};
