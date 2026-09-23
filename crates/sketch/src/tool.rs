//! What a tool remembers between one click and the next.
//!
//! A flat struct standing in for every tool at once means a rectangle carries
//! fields that only ever mean something to a circle, and resetting between
//! shapes clears two dozen of them one by one. A variant per tool means only
//! the click just taken decides which fields exist at all.

use glam::DVec2;

use crate::aim::{ChainAnchor, LockedInput};
use crate::constraints::DimensionTarget;
use crate::corner::Corner;
use crate::element::Element;
use crate::measuring::DimensionPicks;
use crate::picking::Selection;
use crate::resizing::Curved;
use crate::rule_intent::RulePick;
use crate::sketch::{PointId, SegmentId, Sketch};

/// What the selection tool is holding, and what a drag in progress has taken
/// hold of.
#[derive(Clone, Debug, Default)]
pub struct SelectState {
    pub held: Vec<Selection>,
    pub dragged_point: Option<PointId>,
    pub dragged_group: Vec<PointId>,
    pub dragged_dimension: Option<DimensionTarget>,
    pub drag_origin: Option<DVec2>,
    pub drag_position: Option<DVec2>,
    pub drag_preview: Option<Sketch>,
    pub band: Option<(DVec2, DVec2)>,
    /// The curve a drag is drawing to another size, about the centre it
    /// already has.
    pub dragged_curve: Option<Curved>,
    /// Whether the drag under way is pulling its point off what holds it.
    /// Read where the gesture starts, like what it grabbed, and kept for the
    /// whole of it.
    pub letting_go: bool,
}

/// What a measure has taken hold of.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Measured {
    /// A run, a circle or an angle, named the way a dimension names one — the
    /// measure tool shares the dimension tool's aim, so it shares its names.
    Of(DimensionTarget),
    /// A closed area, named by a place inside it.
    ///
    /// The place rather than the area's own name, which is the curves bounding
    /// it: that name is built to outlive an edit, and a measure is built not
    /// to. It is gone the moment the drawing moves, so the simplest thing that
    /// finds the area again in the very next frame is the right one.
    Inside(DVec2),
}

/// How far a shape, a dimension or a rule being drawn has gotten.
///
/// Only one of these is ever true at once — the tool in hand decides which —
/// so a click only ever has to read the fields that concern it, and resetting
/// between shapes is one assignment rather than clearing two dozen fields one
/// by one.
#[derive(Clone, Debug, Default)]
pub enum ToolState {
    /// Nothing recorded: the next click starts a fresh shape, or nothing at
    /// all when the tool in hand is `Select`.
    #[default]
    None,
    Line {
        anchor: ChainAnchor,
        previous: Option<SegmentId>,
    },
    SymmetricLine {
        middle: ChainAnchor,
    },
    Rectangle {
        start: DVec2,
    },
    Circle {
        points: Vec<DVec2>,
        segments: Vec<SegmentId>,
    },
    Arc {
        places: Vec<DVec2>,
        /// Whether a value was typed for the first leg — the radius for
        /// `ByCenter`, the distance between the two ends for `ByEnds` — kept
        /// here because the live field that held it is cleared and reused for
        /// the second leg before the arc is settled enough to be dimensioned.
        first_typed: bool,
    },
    Ellipse {
        places: Vec<DVec2>,
        /// What was typed for the first axis — its width and its angle — kept
        /// here because the live fields are cleared and reused for the second
        /// before the ellipse is drawn and can be dimensioned.
        first_typed: LockedInput,
    },
    Dimension {
        placing: Option<DimensionTarget>,
        picks: DimensionPicks,
    },
    /// What the measure tool is half-way through picking, and what it is
    /// showing right now.
    ///
    /// `showing` is the whole of a measure: it is never written anywhere else,
    /// which is what makes a measure leave no trace. Clearing this state is
    /// clearing the measure, and `reset_pending` already does it on Escape and
    /// on every change of tool.
    Measure {
        picks: DimensionPicks,
        showing: Option<Measured>,
    },
    /// What a tool that lays copies is holding, and whether the next click
    /// names the one thing it still needs — an axis, a centre, a direction —
    /// rather than adding to what is held.
    Copying {
        held: Vec<Element>,
        naming_the_target: bool,
    },
    /// What the chamfer or fillet tool has been shown so far. The two gestures
    /// are the same; only what is laid across the corner differs.
    Corner {
        /// The corners taken, each to be cut with the values typed. Clicking a
        /// corner already taken drops it again.
        taken: Vec<Corner>,
        /// A side named whose other side has not been clicked yet — half a
        /// corner, which is nothing to cut on its own.
        half: Option<SegmentId>,
    },
    Constrain {
        picks: Vec<RulePick>,
    },
    Select(Box<SelectState>),
}

impl ToolState {
    /// True while something is recorded that Escape should undo before it
    /// gives the tool itself back to Select.
    pub fn is_busy(&self) -> bool {
        match self {
            Self::None => false,
            Self::Select(state) => !state.held.is_empty(),
            _ => true,
        }
    }

    /// What the rule being laid down has already been pointed at, and nothing
    /// at all when the tool in hand is laying no rule down.
    pub fn rule_picks(&self) -> &[RulePick] {
        match self {
            Self::Constrain { picks } => picks,
            _ => &[],
        }
    }
}

#[cfg(test)]
mod tests;
