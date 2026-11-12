//! The icon a command's button wears, in the ribbon or on the view bar, looked
//! up by what it does.

use cao_prefs::Command;
use cao_render::MatterInFront;

use crate::ui::icon::{self, Icon};

/// The picture the redesign drew for the command, worn wherever its button is.
pub(crate) fn of(command: Command) -> Icon {
    match command {
        Command::NewSketch => icon::NEW_SKETCH,
        Command::FinishSketch => icon::FINISH,
        Command::Reframe => icon::REFRAME,
        Command::OriginView => icon::HOME_VIEW,
        Command::NextMatterInFront => icon::MATTER_IN_FRONT_HIDDEN,
        Command::SectionView => icon::SECTION_VIEW,
        Command::Undo => icon::UNDO,
        Command::Redo => icon::REDO,
        Command::CompactHistory => icon::COMPACT_HISTORY,
        Command::SelectAll | Command::ToolSelect => icon::SELECT,
        Command::ToolLine => icon::LINE,
        Command::ToolLineSymmetric => icon::LINE_SYMMETRIC,
        Command::ToolRectangle => icon::RECTANGLE,
        Command::ToolCircle | Command::CircleCenter => icon::CIRCLE_CENTRE,
        Command::ToolArc | Command::ArcByCenter => icon::ARC_BY_CENTRE,
        Command::ToolEllipse | Command::EllipseByCentre => icon::ELLIPSE_BY_CENTRE,
        Command::ToolPoint => icon::POINT,
        Command::ToolDimension => icon::DIMENSION,
        Command::ToolMeasure => icon::MEASURE,
        Command::ToolTrim => icon::TRIM,
        Command::ToolSplit => icon::SPLIT,
        Command::ToolChamfer => icon::CHAMFER,
        Command::ToolFillet => icon::FILLET,
        Command::ToolMirror => icon::MIRROR,
        Command::ToolCircularPattern => icon::CIRCULAR_PATTERN,
        Command::ToolRectangularPattern => icon::RECTANGULAR_PATTERN,
        Command::ToggleConstruction => icon::CONSTRUCTION,
        Command::SwapChamferSides => icon::SWAP_CHAMFER_SIDES,
        Command::CircleTwoPoints => icon::CIRCLE_TWO_POINTS,
        Command::CircleThreePoints => icon::CIRCLE_THREE_POINTS,
        Command::CircleTwoTangents => icon::CIRCLE_TWO_TANGENTS,
        Command::CircleThreeTangents => icon::CIRCLE_THREE_TANGENTS,
        Command::ArcByEnds => icon::ARC_BY_ENDS,
        Command::EllipseByEnds => icon::ELLIPSE_BY_ENDS,
        Command::ChamferEqual => icon::CHAMFER_EQUAL,
        Command::ChamferAngled => icon::CHAMFER_ANGLED,
        Command::ChamferSided => icon::CHAMFER_SIDED,
        Command::DimensionAuto => icon::DIMENSION_AUTO,
        Command::DimensionPointToPoint => icon::DIMENSION_POINT_TO_POINT,
        Command::DimensionLength => icon::DIMENSION_LENGTH,
        Command::DimensionAngle => icon::DIMENSION_ANGLE,
        Command::DimensionRadius => icon::DIMENSION_RADIUS,
        Command::RulePerpendicular => icon::PERPENDICULAR,
        Command::RuleParallel => icon::PARALLEL,
        Command::RuleEqual => icon::EQUAL,
        Command::RuleCoincident => icon::COINCIDENT,
        Command::RuleCollinear => icon::COLLINEAR,
        Command::RuleTangent => icon::TANGENT,
        Command::RuleMidpoint => icon::MIDPOINT,
        Command::RuleFixed => icon::FIXED,
        Command::RuleConcentric => icon::CONCENTRIC,
        Command::ExtrusionAdd => icon::EXTRUSION_ADD,
        Command::ExtrusionCut => icon::EXTRUSION_CUT,
        Command::ExtrusionStraight => icon::EXTRUSION_STRAIGHT,
        Command::ExtrusionRevolution => icon::REVOLUTION,
        Command::ExtrusionApply | Command::BlendApply => icon::EXTRUSION_APPLY,
        Command::ExtrusionCancel => icon::EXTRUSION_CANCEL,
        Command::ExportStlCoarse => icon::STL_COARSE,
        Command::ExportStlNormal => icon::STL_NORMAL,
        Command::ExportStlFine => icon::STL_FINE,
        Command::ExportStep => icon::EXPORT_STEP,
        Command::ToggleHistory => icon::HISTORY,
        Command::ToggleToolbarDocked => icon::DOCK_TOOLBAR,
        Command::OpenSettings => icon::PREFERENCES,
        Command::BackToMenu => icon::HOME,
    }
}

/// What the button stepping the matter in front of the sketch wears: the one
/// of its three icons the redesign gives the state it is in.
pub(crate) fn matter_in_front(state: MatterInFront) -> Icon {
    match state {
        MatterInFront::Hidden => icon::MATTER_IN_FRONT_HIDDEN,
        MatterInFront::SeeThrough => icon::MATTER_IN_FRONT_SEE_THROUGH,
        MatterInFront::Opaque => icon::MATTER_IN_FRONT_OPAQUE,
    }
}

#[cfg(test)]
mod tests;
