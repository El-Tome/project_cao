use serde::{Deserialize, Serialize};

/// Everything the user can ask the application to do.
///
/// One list for both the toolbar and the keyboard: a button and a shortcut are
/// two ways of asking for the same thing, and keeping them apart would mean
/// adding every new action twice and letting them drift.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Command {
    NewSketch,
    FinishSketch,
    RecenterOnSketch,
    Undo,
    Redo,
    CompactHistory,

    ToolSelect,
    ToolLine,
    ToolLineSymmetric,
    ToolRectangle,
    ToolCircle,
    ToolArc,
    ToolPoint,
    ToolDimension,
    ToolTrim,
    ToolSplit,
    ToolChamfer,
    ToolFillet,
    ToolMirror,
    ToolCircularPattern,
    ToolRectangularPattern,
    ToggleConstruction,

    CircleCenter,
    CircleTwoPoints,
    CircleThreePoints,
    CircleTwoTangents,
    CircleThreeTangents,

    ArcByCenter,
    ArcByEnds,

    ChamferEqual,
    ChamferAngled,
    ChamferSided,

    DimensionAuto,
    DimensionPointToPoint,
    DimensionLength,
    DimensionAngle,
    DimensionRadius,

    RulePerpendicular,
    RuleParallel,
    RuleEqual,
    RuleCoincident,
    RuleCollinear,
    RuleTangent,
    RuleMidpoint,
    RuleFixed,
    RuleConcentric,

    ExtrusionAdd,
    ExtrusionCut,
    ExtrusionStraight,
    ExtrusionRevolution,
    ExtrusionApply,
    ExtrusionCancel,

    ToggleHistory,
    ToggleExplorer,
    ToggleToolbarDocked,
    OpenSettings,
    BackToMenu,
}

/// The group a command is listed under.
///
/// A case rather than a heading: the settings screen compares one family to
/// the next to know where to start a new run, and comparing two sentences for
/// equality is how a translation quietly breaks the grouping.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CommandFamily {
    Sketch,
    Editing,
    DrawingTools,
    Circles,
    Arcs,
    Chamfers,
    Dimensions,
    Constraints,
    Extrusion,
    Window,
}

impl Command {
    /// Every command, in the order the settings screen offers them.
    pub const ALL: [Self; 55] = [
        Self::NewSketch,
        Self::FinishSketch,
        Self::RecenterOnSketch,
        Self::Undo,
        Self::Redo,
        Self::CompactHistory,
        Self::ToolSelect,
        Self::ToolLine,
        Self::ToolLineSymmetric,
        Self::ToolRectangle,
        Self::ToolCircle,
        Self::ToolArc,
        Self::ToolPoint,
        Self::ToolDimension,
        Self::ToolTrim,
        Self::ToolSplit,
        Self::ToolChamfer,
        Self::ToolFillet,
        Self::ToolMirror,
        Self::ToolCircularPattern,
        Self::ToolRectangularPattern,
        Self::ToggleConstruction,
        Self::CircleCenter,
        Self::CircleTwoPoints,
        Self::CircleThreePoints,
        Self::CircleTwoTangents,
        Self::CircleThreeTangents,
        Self::ArcByCenter,
        Self::ArcByEnds,
        Self::ChamferEqual,
        Self::ChamferAngled,
        Self::ChamferSided,
        Self::DimensionAuto,
        Self::DimensionPointToPoint,
        Self::DimensionLength,
        Self::DimensionAngle,
        Self::DimensionRadius,
        Self::RulePerpendicular,
        Self::RuleParallel,
        Self::RuleEqual,
        Self::RuleCoincident,
        Self::RuleCollinear,
        Self::RuleTangent,
        Self::RuleMidpoint,
        Self::RuleFixed,
        Self::RuleConcentric,
        Self::ExtrusionAdd,
        Self::ExtrusionCut,
        Self::ExtrusionStraight,
        Self::ExtrusionRevolution,
        Self::ExtrusionApply,
        Self::ExtrusionCancel,
        Self::ToggleHistory,
        Self::ToggleExplorer,
        Self::ToggleToolbarDocked,
    ];

    /// Which family the command belongs to, used to group the palette in the
    /// settings screen.
    pub fn family(self) -> CommandFamily {
        match self {
            Self::NewSketch | Self::FinishSketch | Self::RecenterOnSketch => CommandFamily::Sketch,
            Self::Undo | Self::Redo | Self::CompactHistory => CommandFamily::Editing,
            Self::ToolSelect
            | Self::ToolLine
            | Self::ToolLineSymmetric
            | Self::ToolRectangle
            | Self::ToolCircle
            | Self::ToolArc
            | Self::ToolPoint
            | Self::ToolDimension
            | Self::ToolTrim
            | Self::ToolSplit
            | Self::ToolChamfer
            | Self::ToolFillet
            | Self::ToolMirror
            | Self::ToolCircularPattern
            | Self::ToolRectangularPattern
            | Self::ToggleConstruction => CommandFamily::DrawingTools,
            Self::CircleCenter
            | Self::CircleTwoPoints
            | Self::CircleThreePoints
            | Self::CircleTwoTangents
            | Self::CircleThreeTangents => CommandFamily::Circles,
            Self::ArcByCenter | Self::ArcByEnds => CommandFamily::Arcs,
            Self::ChamferEqual | Self::ChamferAngled | Self::ChamferSided => {
                CommandFamily::Chamfers
            }
            Self::DimensionAuto
            | Self::DimensionPointToPoint
            | Self::DimensionLength
            | Self::DimensionAngle
            | Self::DimensionRadius => CommandFamily::Dimensions,
            Self::RulePerpendicular
            | Self::RuleParallel
            | Self::RuleEqual
            | Self::RuleCoincident
            | Self::RuleCollinear
            | Self::RuleTangent
            | Self::RuleMidpoint
            | Self::RuleFixed
            | Self::RuleConcentric => CommandFamily::Constraints,
            Self::ExtrusionAdd
            | Self::ExtrusionCut
            | Self::ExtrusionStraight
            | Self::ExtrusionRevolution
            | Self::ExtrusionApply
            | Self::ExtrusionCancel => CommandFamily::Extrusion,
            Self::ToggleHistory
            | Self::ToggleExplorer
            | Self::ToggleToolbarDocked
            | Self::OpenSettings
            | Self::BackToMenu => CommandFamily::Window,
        }
    }
}

#[cfg(test)]
mod tests;
