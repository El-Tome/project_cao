//! What the rows of the copy tools' panel say, and what the drawing says the
//! panel's current step asks for.

use cao_prefs::Command;

use crate::lang::Catalogue;
use crate::wording::command;

/// What the target slot of the panel names, depending on the tool.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Target {
    Axis,
    Centre,
    Direction,
}

/// What a numbered step of the circular pattern asks for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Asked {
    Elements,
    Centre,
    Count,
    Rotation,
}

/// The message over the drawing while a step of the circular pattern is
/// current: the tool, the step's number, and what it asks for.
pub fn circular_step(lang: &Catalogue, number: usize, asked: Asked) -> String {
    let asked = lang.t(match asked {
        Asked::Elements => "copy_panel.asks_elements",
        Asked::Centre => "copy_panel.asks_centre",
        Asked::Count => "copy_panel.asks_count",
        Asked::Rotation => "copy_panel.asks_rotation",
    });
    let tool = command::label(lang, Command::ToolCircularPattern);
    lang.t_with(
        "copy_panel.step_asks",
        &[
            ("tool", &tool),
            ("number", &number.to_string()),
            ("asked", &asked),
        ],
    )
}

/// The elements row: how many are held.
pub fn elements(lang: &Catalogue, count: usize) -> String {
    lang.t_with("copy_panel.elements", &[("count", &count.to_string())])
}

/// The target row, chosen or still to be clicked in the drawing.
pub fn target(lang: &Catalogue, target: Target, chosen: bool) -> String {
    lang.t(match (target, chosen) {
        (Target::Axis, true) => "copy_panel.axis_chosen",
        (Target::Axis, false) => "copy_panel.axis_to_click",
        (Target::Centre, true) => "copy_panel.centre_chosen",
        (Target::Centre, false) => "copy_panel.centre_to_click",
        (Target::Direction, true) => "copy_panel.direction_chosen",
        (Target::Direction, false) => "copy_panel.direction_to_click",
    })
}

#[cfg(test)]
mod tests;
