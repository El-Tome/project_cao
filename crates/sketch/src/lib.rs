//! The sketch model: 2D geometry drawn on a work plane, the rules that apply a
//! typed value to it, and the solver that keeps every value true at once. No
//! rendering and no UI, so the same model backs any front-end and can be saved
//! straight into a part file.

mod constraints;
pub mod construct;
mod dimensioning;
mod independence;
mod plane;
mod regions;
pub mod segment;
mod sketch;
mod solver;

pub use constraints::{Constraint, Dimension, DimensionTarget, Freedom, SketchAxis};
pub use dimensioning::axis_under;
pub use plane::WorkPlane;
pub use regions::Region;
pub use sketch::{Circle, CircleId, Element, LengthOutcome, PointId, Segment, SegmentId, Sketch};
pub use solver::SolveOutcome;
