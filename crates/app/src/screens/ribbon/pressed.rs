//! Which command is the one in force — the tool in hand, the way it works,
//! the toggle on — so that its button shows pressed.

use cao_prefs::Command;
use cao_sketch::Rule;

use super::state::Context;
use crate::screens::extrusion::Shape;
use crate::screens::sketch::{ArcMode, ChamferMode, CircleMode, DimensionMode, EllipseMode, Tool};

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
        Command::ToolMeasure => tool == Tool::Measure,
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
        Command::EllipseByCentre => {
            tool == Tool::Ellipse && state.editor.ellipse_mode == EllipseMode::ByCentre
        }
        Command::EllipseByEnds => {
            tool == Tool::Ellipse && state.editor.ellipse_mode == EllipseMode::ByEnds
        }
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
        Command::SectionView => state.section_on,
        _ => false,
    }
}

fn mode_is(state: &Context<'_>, mode: CircleMode) -> bool {
    state.editor.circle_mode == mode
}
