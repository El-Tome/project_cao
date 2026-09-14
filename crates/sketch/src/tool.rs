//! What a tool remembers between one click and the next.
//!
//! A flat struct standing in for every tool at once means a rectangle carries
//! fields that only ever mean something to a circle, and resetting between
//! shapes clears two dozen of them one by one. A variant per tool means only
//! the click just taken decides which fields exist at all.

use glam::DVec2;

use crate::aim::ChainAnchor;
use crate::constraints::DimensionTarget;
use crate::measuring::DimensionPicks;
use crate::picking::Selection;
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
    Dimension {
        placing: Option<DimensionTarget>,
        picks: DimensionPicks,
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
mod tests {
    use super::*;
    use crate::sketch::Element;

    #[test]
    fn escape_undoes_the_shape_in_progress_before_it_gives_the_tool_back() {
        let mid_rectangle = ToolState::Rectangle {
            start: DVec2::new(1.0, 2.0),
        };
        assert!(
            mid_rectangle.is_busy(),
            "a rectangle with one corner placed is a shape to lose, not a tool to give up"
        );

        assert!(
            !ToolState::None.is_busy(),
            "nothing recorded leaves the tool itself as the only thing escape can give back"
        );
    }

    #[test]
    fn a_selection_held_by_the_select_tool_is_also_something_escape_would_undo_first() {
        let mut holding = SelectState::default();
        holding
            .held
            .push(Selection::Element(Element::Point(PointId(0))));

        assert!(ToolState::Select(Box::new(holding)).is_busy());
        assert!(!ToolState::Select(Box::default()).is_busy());
    }

    #[test]
    fn a_circle_or_a_rule_half_picked_is_busy_even_though_the_old_flat_check_missed_it() {
        // The flat `SketchEditor` this replaces checked `chain`, `pending_start`,
        // `placing`, `selected`, `selection`, and the three dimension picks for
        // whether Escape should hand the tool back — but never `circle_points`,
        // `circle_segments`, `rule_picks` or `band`. A circle two clicks into a
        // three-point construction, or a rule with one trait already picked,
        // was silently kicked back to Select. A variant that only exists once
        // something is recorded cannot make that mistake.
        let half_circle = ToolState::Circle {
            points: vec![DVec2::new(1.0, 1.0)],
            segments: Vec::new(),
        };
        let half_rule = ToolState::Constrain {
            picks: vec![RulePick::Element(Element::Point(PointId(0)))],
        };

        assert!(half_circle.is_busy());
        assert!(half_rule.is_busy());
    }

    #[test]
    fn a_rule_half_laid_down_hands_back_what_it_has_been_pointed_at() {
        let trait_picked = RulePick::Element(Element::Segment(SegmentId(2)));
        let half_rule = ToolState::Constrain {
            picks: vec![trait_picked],
        };

        assert_eq!(half_rule.rule_picks(), [trait_picked]);
        assert!(
            ToolState::None.rule_picks().is_empty(),
            "a tool laying no rule down has been shown nothing to draw differently"
        );
    }
}
