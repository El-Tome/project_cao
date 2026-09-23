use crate::lang::Catalogue;
use crate::screens::extrusion::{ExtrusionState, Shape};
use crate::screens::sketch::{ArcMode, ChamferMode, CircleMode, DimensionMode, SketchEditor, Tool};
use cao_part::PartDocument;
use cao_prefs::{Command, Settings};
use cao_sketch::Rule;

/// The toolbar, drawn from the arrangement the user has set.
///
/// Nothing about which buttons exist or where they sit is decided here: the
/// tree comes from the settings, and this only knows how to draw a tree. That
/// is what lets the arrangement be changed, saved and handed to somebody else.
#[derive(Default)]
pub struct Ribbon {
    /// Which top-level group is open, by rank.
    pub tab: usize,
    /// The tree of what the part is made of, open by default. The run of
    /// operations that made it is the other panel, and it waits to be asked
    /// for.
    pub part_tree_open: bool,
    pub history_open: bool,
    /// Whether the history panel is asking to confirm a compaction. Held here
    /// rather than only in the panel's own frame, so the warning survives to
    /// the next one instead of closing the moment the mouse moves.
    pub history_compact_confirm: bool,
}

impl Ribbon {
    pub fn new() -> Self {
        Self {
            tab: 0,
            part_tree_open: true,
            history_open: false,
            history_compact_confirm: false,
        }
    }
}

/// What the toolbar needs to know to draw a button in the right state.
pub(super) struct Context<'a> {
    pub(super) settings: &'a Settings,
    pub(super) document: &'a PartDocument,
    pub(super) editor: &'a SketchEditor,
    pub(super) extrusion: &'a ExtrusionState,
    pub(super) lang: &'a Catalogue,
}

/// Whether the chamfer tool is in hand and saying what it takes this way.
fn chamfering(state: &Context<'_>, mode: ChamferMode) -> bool {
    state.editor.tool == Tool::Chamfer && state.editor.chamfer_mode == mode
}

/// Whether the command is the one currently in force, so its button shows as
/// pressed.
pub(super) fn active(command: Command, state: &Context<'_>) -> bool {
    let tool = state.editor.tool;
    let mode = state.editor.dimension_mode;
    match command {
        Command::ToolSelect => tool == Tool::Select,
        Command::ToolLine => tool == Tool::Line,
        Command::ToolLineSymmetric => tool == Tool::LineSymmetric,
        Command::ToolRectangle => tool == Tool::Rectangle,
        Command::ToolCircle => tool == Tool::Circle,
        Command::ToolArc => tool == Tool::Arc,
        Command::ToolEllipse => tool == Tool::Ellipse,
        Command::ToolPoint => tool == Tool::Point,
        Command::ToolDimension => tool == Tool::Dimension,
        Command::ToolTrim => tool == Tool::Trim,
        Command::ToolSplit => tool == Tool::Split,
        Command::ToolChamfer => tool == Tool::Chamfer,
        Command::ToolFillet => tool == Tool::Fillet,
        Command::ToolMirror => tool == Tool::Mirror,
        Command::ToolCircularPattern => tool == Tool::CircularPattern,
        Command::ToolRectangularPattern => tool == Tool::RectangularPattern,
        Command::ToggleConstruction => state.editor.construction,
        Command::CircleCenter => tool == Tool::Circle && mode_is(state, CircleMode::Center),
        Command::CircleTwoPoints => tool == Tool::Circle && mode_is(state, CircleMode::TwoPoints),
        Command::CircleThreePoints => {
            tool == Tool::Circle && mode_is(state, CircleMode::ThreePoints)
        }
        Command::CircleTwoTangents => {
            tool == Tool::Circle && mode_is(state, CircleMode::TwoTangents)
        }
        Command::CircleThreeTangents => {
            tool == Tool::Circle && mode_is(state, CircleMode::ThreeTangents)
        }
        Command::ArcByCenter => tool == Tool::Arc && state.editor.arc_mode == ArcMode::ByCenter,
        Command::ArcByEnds => tool == Tool::Arc && state.editor.arc_mode == ArcMode::ByEnds,
        Command::ChamferEqual => chamfering(state, ChamferMode::Equal),
        Command::ChamferAngled => chamfering(state, ChamferMode::Angled),
        Command::ChamferSided => chamfering(state, ChamferMode::Sided),
        Command::RulePerpendicular => tool == Tool::Constrain(Rule::Perpendicular),
        Command::RuleParallel => tool == Tool::Constrain(Rule::Parallel),
        Command::RuleEqual => tool == Tool::Constrain(Rule::Equal),
        Command::RuleCoincident => tool == Tool::Constrain(Rule::Coincident),
        Command::RuleCollinear => tool == Tool::Constrain(Rule::Collinear),
        Command::RuleTangent => tool == Tool::Constrain(Rule::Tangent),
        Command::RuleMidpoint => tool == Tool::Constrain(Rule::Midpoint),
        Command::RuleFixed => tool == Tool::Constrain(Rule::Fixed),
        Command::RuleConcentric => tool == Tool::Constrain(Rule::Concentric),
        Command::DimensionAuto => mode == DimensionMode::Auto,
        Command::DimensionPointToPoint => mode == DimensionMode::PointToPoint,
        Command::DimensionLength => mode == DimensionMode::Length,
        Command::DimensionAngle => mode == DimensionMode::Angle,
        Command::DimensionRadius => mode == DimensionMode::Radius,
        Command::ExtrusionAdd => state.extrusion.mode == Some(cao_part::ExtrusionMode::Add),
        Command::ExtrusionCut => state.extrusion.mode == Some(cao_part::ExtrusionMode::Cut),
        Command::ExtrusionStraight => state.extrusion.shape == Shape::Straight,
        Command::ExtrusionRevolution => state.extrusion.shape == Shape::Revolution,
        _ => false,
    }
}

fn mode_is(state: &Context<'_>, mode: CircleMode) -> bool {
    state.editor.circle_mode == mode
}

pub(super) fn enabled(command: Command, state: &Context<'_>) -> bool {
    is_enabled(command, state.document, state.editor, state.extrusion)
}

/// Whether a command can be carried out right now.
///
/// Shared with the keyboard: a shortcut for a command whose button is greyed
/// out must do nothing either, or Enter would "finish" a sketch that is not
/// open.
pub fn is_enabled(
    command: Command,
    document: &PartDocument,
    editor: &SketchEditor,
    extrusion: &ExtrusionState,
) -> bool {
    let drawing = editor.active_sketch().is_some();
    match command {
        Command::Undo => document.history.can_undo(),
        Command::Redo => document.history.can_redo(),
        Command::CompactHistory => !document.history.is_empty(),
        Command::FinishSketch | Command::RecenterOnSketch => drawing,
        Command::ToolSelect
        | Command::ToolLine
        | Command::ToolLineSymmetric
        | Command::ToolRectangle
        | Command::ToolCircle
        | Command::ToolArc
        | Command::ToolEllipse
        | Command::ToolPoint
        | Command::ToolDimension
        | Command::ToolTrim
        | Command::ToolSplit
        | Command::ToolChamfer
        | Command::ToolFillet
        | Command::ToolMirror
        | Command::ToolCircularPattern
        | Command::ToolRectangularPattern => drawing,
        Command::ChamferEqual | Command::ChamferAngled | Command::ChamferSided => drawing,
        Command::DimensionAuto
        | Command::DimensionPointToPoint
        | Command::DimensionLength
        | Command::DimensionAngle
        | Command::DimensionRadius => drawing && editor.tool == Tool::Dimension,
        Command::CircleCenter
        | Command::CircleTwoPoints
        | Command::CircleThreePoints
        | Command::CircleTwoTangents
        | Command::CircleThreeTangents => drawing,
        Command::ArcByCenter | Command::ArcByEnds => drawing,
        Command::RulePerpendicular
        | Command::RuleParallel
        | Command::RuleEqual
        | Command::RuleCoincident
        | Command::RuleCollinear
        | Command::RuleTangent
        | Command::RuleMidpoint
        | Command::RuleFixed
        | Command::RuleConcentric => drawing,
        Command::ExtrusionAdd | Command::ExtrusionCut => extrusion.sketch.is_some(),
        Command::ExtrusionStraight | Command::ExtrusionRevolution => extrusion.is_active(),
        Command::ExtrusionApply => extrusion.is_ready(),
        Command::ExtrusionCancel => extrusion.is_active(),
        _ => true,
    }
}

#[cfg(test)]
mod tests;
