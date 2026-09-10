//! The sketch model: 2D geometry drawn on a work plane, the rules that apply a
//! typed value to it, and the solver that keeps every value true at once. No
//! rendering and no UI, so the same model backs any front-end and can be saved
//! straight into a part file.

mod aim;
mod annotation;
mod chain;
mod circling;
mod constraints;
pub mod construct;
mod crossing;
mod dimensioning;
mod element;
mod equation;
mod erased;
mod independence;
mod length;
mod measuring;
mod picking;
mod plane;
mod regions;
mod rigid;
mod rule_intent;
mod rule_marks;
pub mod segment;
mod settled;
mod shape_dimensions;
mod sketch;
mod snap;
mod solver;
mod tangency;
mod tool;

pub use aim::{Aim, ChainAnchor, LockedInput, rectangle_corner};
pub use annotation::{AnnotationMetrics, Placement};
pub use chain::{ChainClick, chain_click};
pub use circling::{CircleProgress, Found, circle_from, circle_progress, rim_of};
pub use constraints::{Constraint, Dimension, DimensionTarget, Freedom, SketchAxis};
pub use construct::CircleMode;
pub use dimensioning::axis_under;
pub use length::LengthOutcome;
pub use measuring::{DimensionMode, DimensionPick, DimensionPicks, measure_pick};
pub use picking::Selection;
pub use plane::{PlaneKind, WorkPlane};
pub use regions::Region;
pub use rule_intent::{Rule, RuleIntent, RulePick, rule_intent};
pub use shape_dimensions::{line_dimensions, rectangle_dimensions};
pub use sketch::{Circle, CircleId, Element, PointId, Segment, SegmentId, Sketch};
pub use snap::{Snap, SnapSettings};
pub use solver::SolveOutcome;
pub use tool::{SelectState, ToolState};
