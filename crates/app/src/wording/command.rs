use cao_prefs::{Command, CommandFamily};

use crate::lang::Catalogue;

/// The only place a `Command` is turned into a name.
///
/// The palette, the toolbar and the shortcut list all read from here, so a
/// command is called the same thing wherever the user meets it.
pub fn label(lang: &Catalogue, command: Command) -> String {
    lang.t(match command {
        Command::NewSketch => "command.label.new_sketch",
        Command::FinishSketch => "command.label.finish_sketch",
        Command::RecenterOnSketch => "command.label.recenter_on_sketch",
        Command::Undo => "command.label.undo",
        Command::Redo => "command.label.redo",
        Command::ToolSelect => "command.label.tool_select",
        Command::ToolLine => "command.label.tool_line",
        Command::ToolRectangle => "command.label.tool_rectangle",
        Command::ToolCircle => "command.label.tool_circle",
        Command::ToolPoint => "command.label.tool_point",
        Command::ToolDimension => "command.label.tool_dimension",
        Command::ToggleConstruction => "command.label.toggle_construction",
        Command::CircleCenter => "command.label.circle_center",
        Command::CircleTwoPoints => "command.label.circle_two_points",
        Command::CircleThreePoints => "command.label.circle_three_points",
        Command::CircleTwoTangents => "command.label.circle_two_tangents",
        Command::CircleThreeTangents => "command.label.circle_three_tangents",
        Command::DimensionAuto => "command.label.dimension_auto",
        Command::DimensionPointToPoint => "command.label.dimension_point_to_point",
        Command::DimensionLength => "command.label.dimension_length",
        Command::DimensionAngle => "command.label.dimension_angle",
        Command::DimensionRadius => "command.label.dimension_radius",
        Command::RulePerpendicular => "command.label.rule_perpendicular",
        Command::RuleParallel => "command.label.rule_parallel",
        Command::RuleEqual => "command.label.rule_equal",
        Command::RuleCoincident => "command.label.rule_coincident",
        Command::RuleCollinear => "command.label.rule_collinear",
        Command::RuleTangent => "command.label.rule_tangent",
        Command::RuleMidpoint => "command.label.rule_midpoint",
        Command::RuleFixed => "command.label.rule_fixed",
        Command::RuleConcentric => "command.label.rule_concentric",
        Command::ExtrusionAdd => "command.label.extrusion_add",
        Command::ExtrusionCut => "command.label.extrusion_cut",
        Command::ExtrusionStraight => "command.label.extrusion_straight",
        Command::ExtrusionRevolution => "command.label.extrusion_revolution",
        Command::ExtrusionApply => "command.label.extrusion_apply",
        Command::ExtrusionCancel => "command.label.extrusion_cancel",
        Command::ToggleHistory => "command.label.toggle_history",
        Command::ToggleToolbarDocked => "command.label.toggle_toolbar_docked",
        Command::OpenSettings => "command.label.open_settings",
        Command::BackToMenu => "command.label.back_to_menu",
    })
}

/// The one line of help shown when the pointer rests on the command.
pub fn hint(lang: &Catalogue, command: Command) -> String {
    lang.t(match command {
        Command::NewSketch => "command.hint.new_sketch",
        Command::FinishSketch => "command.hint.finish_sketch",
        Command::RecenterOnSketch => "command.hint.recenter_on_sketch",
        Command::Undo => "command.hint.undo",
        Command::Redo => "command.hint.redo",
        Command::ToolSelect => "command.hint.tool_select",
        Command::ToolLine => "command.hint.tool_line",
        Command::ToolRectangle => "command.hint.tool_rectangle",
        Command::ToolCircle => "command.hint.tool_circle",
        Command::ToolPoint => "command.hint.tool_point",
        Command::ToolDimension => "command.hint.tool_dimension",
        Command::ToggleConstruction => "command.hint.toggle_construction",
        Command::CircleCenter => "command.hint.circle_center",
        Command::CircleTwoPoints => "command.hint.circle_two_points",
        Command::CircleThreePoints => "command.hint.circle_three_points",
        Command::CircleTwoTangents => "command.hint.circle_two_tangents",
        Command::CircleThreeTangents => "command.hint.circle_three_tangents",
        Command::DimensionAuto => "command.hint.dimension_auto",
        Command::DimensionPointToPoint => "command.hint.dimension_point_to_point",
        Command::DimensionLength => "command.hint.dimension_length",
        Command::DimensionAngle => "command.hint.dimension_angle",
        Command::DimensionRadius => "command.hint.dimension_radius",
        Command::RulePerpendicular => "command.hint.rule_perpendicular",
        Command::RuleParallel => "command.hint.rule_parallel",
        Command::RuleEqual => "command.hint.rule_equal",
        Command::RuleCoincident => "command.hint.rule_coincident",
        Command::RuleCollinear => "command.hint.rule_collinear",
        Command::RuleTangent => "command.hint.rule_tangent",
        Command::RuleMidpoint => "command.hint.rule_midpoint",
        Command::RuleFixed => "command.hint.rule_fixed",
        Command::RuleConcentric => "command.hint.rule_concentric",
        Command::ExtrusionAdd => "command.hint.extrusion_add",
        Command::ExtrusionCut => "command.hint.extrusion_cut",
        Command::ExtrusionStraight => "command.hint.extrusion_straight",
        Command::ExtrusionRevolution => "command.hint.extrusion_revolution",
        Command::ExtrusionApply => "command.hint.extrusion_apply",
        Command::ExtrusionCancel => "command.hint.extrusion_cancel",
        Command::ToggleHistory => "command.hint.toggle_history",
        Command::ToggleToolbarDocked => "command.hint.toggle_toolbar_docked",
        Command::OpenSettings => "command.hint.open_settings",
        Command::BackToMenu => "command.hint.back_to_menu",
    })
}

/// The heading a run of the palette is listed under.
///
/// The settings screen starts a new run where the family changes, so a family
/// is written once, above the commands that belong to it.
pub fn family_heading(lang: &Catalogue, family: CommandFamily) -> String {
    lang.t(match family {
        CommandFamily::Sketch => "command.family.sketch",
        CommandFamily::Editing => "command.family.editing",
        CommandFamily::DrawingTools => "command.family.drawing_tools",
        CommandFamily::Circles => "command.family.circles",
        CommandFamily::Dimensions => "command.family.dimensions",
        CommandFamily::Constraints => "command.family.constraints",
        CommandFamily::Extrusion => "command.family.extrusion",
        CommandFamily::Window => "command.family.window",
    })
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;

    #[test]
    fn no_two_commands_of_the_palette_read_the_same() {
        let lang = Catalogue::french();
        let mut seen: BTreeMap<String, Command> = BTreeMap::new();

        for command in Command::ALL {
            assert!(
                !label(&lang, command).is_empty(),
                "{command:?} has no name, so its button is a blank",
            );
            if let Some(taken) = seen.insert(label(&lang, command), command) {
                panic!(
                    "{taken:?} and {command:?} both read {:?}: the palette offers two \
                     entries a user cannot tell apart",
                    label(&lang, command),
                );
            }
        }
    }

    #[test]
    fn resting_on_a_command_says_more_than_its_button_already_shows() {
        let lang = Catalogue::french();

        for command in Command::ALL {
            assert!(
                !hint(&lang, command).is_empty(),
                "{command:?} has nothing to say on hover",
            );
            assert_ne!(
                hint(&lang, command),
                label(&lang, command),
                "{command:?} repeats its own name on hover instead of helping",
            );
        }
    }

    #[test]
    fn no_two_families_of_the_palette_are_headed_the_same() {
        let lang = Catalogue::french();
        let mut seen: BTreeMap<String, CommandFamily> = BTreeMap::new();

        for command in Command::ALL {
            let family = command.family();
            assert!(
                !family_heading(&lang, family).is_empty(),
                "{family:?} heads its run of the palette with a blank",
            );
            if let Some(taken) = seen.insert(family_heading(&lang, family), family) {
                assert_eq!(
                    taken,
                    family,
                    "{taken:?} and {family:?} both head {:?}: the palette reads as \
                     one family cut in two",
                    family_heading(&lang, family),
                );
            }
        }
    }
}
