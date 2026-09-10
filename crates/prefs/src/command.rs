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

    ToolSelect,
    ToolLine,
    ToolRectangle,
    ToolCircle,
    ToolPoint,
    ToolDimension,
    ToggleConstruction,

    CircleCenter,
    CircleTwoPoints,
    CircleThreePoints,
    CircleTwoTangents,
    CircleThreeTangents,

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
    Dimensions,
    Constraints,
    Extrusion,
    Window,
}

impl Command {
    /// Every command, in the order the settings screen offers them.
    pub const ALL: [Self; 39] = [
        Self::NewSketch,
        Self::FinishSketch,
        Self::RecenterOnSketch,
        Self::Undo,
        Self::Redo,
        Self::ToolSelect,
        Self::ToolLine,
        Self::ToolRectangle,
        Self::ToolCircle,
        Self::ToolPoint,
        Self::ToolDimension,
        Self::ToggleConstruction,
        Self::CircleCenter,
        Self::CircleTwoPoints,
        Self::CircleThreePoints,
        Self::CircleTwoTangents,
        Self::CircleThreeTangents,
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
        Self::ToggleToolbarDocked,
    ];

    /// Which family the command belongs to, used to group the palette in the
    /// settings screen.
    pub fn family(self) -> CommandFamily {
        match self {
            Self::NewSketch | Self::FinishSketch | Self::RecenterOnSketch => CommandFamily::Sketch,
            Self::Undo | Self::Redo => CommandFamily::Editing,
            Self::ToolSelect
            | Self::ToolLine
            | Self::ToolRectangle
            | Self::ToolCircle
            | Self::ToolPoint
            | Self::ToolDimension
            | Self::ToggleConstruction => CommandFamily::DrawingTools,
            Self::CircleCenter
            | Self::CircleTwoPoints
            | Self::CircleThreePoints
            | Self::CircleTwoTangents
            | Self::CircleThreeTangents => CommandFamily::Circles,
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
            | Self::ToggleToolbarDocked
            | Self::OpenSettings
            | Self::BackToMenu => CommandFamily::Window,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_family_comes_back_once_the_palette_has_left_it() {
        let mut headed: Vec<CommandFamily> = Vec::new();
        let mut current: Option<CommandFamily> = None;

        for command in Command::ALL {
            let family = command.family();
            if current == Some(family) {
                continue;
            }
            assert!(
                !headed.contains(&family),
                "{family:?} comes back after {current:?}: the palette heads a family \
                 where it changes, so a family listed in two runs shows twice and the \
                 user reads the same heading over a different half",
            );
            headed.push(family);
            current = Some(family);
        }
    }
}
