//! The sketch model: 2D geometry drawn on a work plane, the rules that apply a
//! typed value to it, and the solver that keeps every value true at once. No
//! rendering and no UI, so the same model backs any front-end and can be saved
//! straight into a part file.

mod aim;
mod annotation;
mod arc;
mod arc_annotation;
mod arc_dimensions;
mod arc_placing;
mod arc_regions;
mod arc_rules;
mod arcing;
mod chain;
mod circle_edges;
mod circling;
mod constraints;
pub mod construct;
mod crossing;
mod dimensioning;
mod edges;
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
mod symmetric;
mod tangency;
mod tool;
mod trimming;

pub use aim::{Aim, ChainAnchor, LockedInput, rectangle_corner};
pub use annotation::{AnnotationMetrics, Placement};
pub use arc::{Arc, ArcId};
pub use arc_dimensions::arc_dimensions;
pub use arc_placing::{
    ArcMode, aimed as arc_aimed, angle_reference as arc_angle_reference, arc_from,
};
pub use arcing::{ArcDraft, places_along, steps_along, sweep_of};
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
pub use shape_dimensions::{line_dimensions, rectangle_dimensions, symmetric_segment_dimensions};
pub use sketch::{Circle, CircleId, Element, PointId, Segment, SegmentId, Sketch};
pub use snap::{Snap, SnapSettings};
pub use solver::SolveOutcome;
pub use symmetric::{SymmetricClick, symmetric_click};
pub use tool::{SelectState, ToolState};
pub use trimming::Trimmed;
pub use trimming::arc::ArcTrimmed;
