//! What the rows of the copy tools' panel say.

use crate::lang::Catalogue;

/// What the target slot of the panel names, depending on the tool.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Target {
    Axis,
    Centre,
    Direction,
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
