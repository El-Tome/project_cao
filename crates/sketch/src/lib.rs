//! The sketch model: 2D geometry drawn on a work plane, the rules that apply a
//! typed value to it, and the solver that keeps every value true at once. No
//! rendering and no UI, so the same model backs any front-end and can be saved
//! straight into a part file.

mod aim;
mod constraints;
pub mod construct;
mod dimensioning;
mod equation;
mod independence;
mod picking;
mod plane;
mod regions;
mod rigid;
mod rule_intent;
mod rule_marks;
pub mod segment;
mod settled;
mod sketch;
mod snap;
mod solver;

pub use aim::{Aim, ChainAnchor, LockedInput, rectangle_corner};
pub use constraints::{Constraint, Dimension, DimensionTarget, Freedom, SketchAxis};
pub use dimensioning::axis_under;
pub use picking::Selection;
pub use plane::{PlaneKind, WorkPlane};
pub use regions::Region;
pub use rule_intent::{Rule, RuleIntent, RulePick, rule_intent};
pub use sketch::{Circle, CircleId, Element, LengthOutcome, PointId, Segment, SegmentId, Sketch};
pub use snap::{Snap, SnapSettings};
pub use solver::SolveOutcome;
