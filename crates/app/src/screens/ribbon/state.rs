use crate::lang::Catalogue;
use crate::screens::extrusion::{ExtrusionState, Shape};
use crate::screens::sketch::{CircleMode, DimensionMode, SketchEditor, Tool};
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
    pub history_open: bool,
}

impl Ribbon {
    pub fn new() -> Self {
        Self {
            tab: 0,
            history_open: true,
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

/// Whether the command is the one currently in force, so its button shows as
/// pressed.
pub(super) fn active(command: Command, state: &Context<'_>) -> bool {
    let tool = state.editor.tool;
    let mode = state.editor.dimension_mode;
    match command {
        Command::ToolSelect => tool == Tool::Select,
        Command::ToolLine => tool == Tool::Line,
        Command::ToolRectangle => tool == Tool::Rectangle,
        Command::ToolCircle => tool == Tool::Circle,
        Command::ToolPoint => tool == Tool::Point,
        Command::ToolDimension => tool == Tool::Dimension,
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
        Command::FinishSketch | Command::RecenterOnSketch => drawing,
        Command::ToolSelect
        | Command::ToolLine
        | Command::ToolRectangle
        | Command::ToolCircle
        | Command::ToolPoint
        | Command::ToolDimension => drawing,
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
mod tests {
    use super::*;
    use cao_sketch::WorkPlane;
    use chrono::Utc;

    fn a_part() -> PartDocument {
        PartDocument::new("part", Utc::now())
    }

    fn drawing_on_a_sketch() -> SketchEditor {
        let mut editor = SketchEditor::default();
        editor.begin_editing(0, WorkPlane::XY);
        editor
    }

    #[test]
    fn a_drawing_tool_is_offered_only_while_a_sketch_is_open() {
        let part = a_part();
        let extrusion = ExtrusionState::default();

        assert!(!is_enabled(
            Command::ToolLine,
            &part,
            &SketchEditor::default(),
            &extrusion
        ));
        assert!(is_enabled(
            Command::ToolLine,
            &part,
            &drawing_on_a_sketch(),
            &extrusion
        ));
    }

    #[test]
    fn a_fresh_part_has_nothing_to_undo_and_nothing_to_redo() {
        let part = a_part();
        let editor = SketchEditor::default();
        let extrusion = ExtrusionState::default();

        assert!(!is_enabled(Command::Undo, &part, &editor, &extrusion));
        assert!(!is_enabled(Command::Redo, &part, &editor, &extrusion));
    }

    #[test]
    fn an_extrusion_is_applied_only_once_it_holds_every_value_it_needs() {
        let part = a_part();
        let editor = SketchEditor::default();
        let ready = ExtrusionState::default();

        assert_eq!(
            is_enabled(Command::ExtrusionApply, &part, &editor, &ready),
            ready.is_ready(),
        );
    }

    #[test]
    fn the_tool_in_hand_is_the_one_shown_as_pressed() {
        let part = a_part();
        let editor = drawing_on_a_sketch();
        let extrusion = ExtrusionState::default();
        let settings = Settings::default();
        let lang = Catalogue::french();
        let state = Context {
            settings: &settings,
            document: &part,
            editor: &editor,
            extrusion: &extrusion,
            lang: &lang,
        };

        assert!(active(Command::ToolLine, &state));
        assert!(!active(Command::ToolCircle, &state));
    }
}
