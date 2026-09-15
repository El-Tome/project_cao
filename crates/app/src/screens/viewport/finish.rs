//! What Enter does to the shape in hand, once the live fields have had their
//! say.

use cao_sketch::ToolState;

use super::input::{
    corner_held, cut_the_corner, draw_arc, draw_circle, draw_line_point, draw_symmetric_line_point,
    rectangle_corner, two_click_shape,
};
use super::render::paint_live_input;
use super::{PICK_PIXELS, SketchContext, ViewScale};
use crate::screens::sketch::Tool;

/// Enter finishes the shape from the keyboard, without having to find the canvas again with the
/// mouse. What the shape ends at follows the same reading as a click would: the line's aim, or the
/// rectangle's corner once what was typed has had its say.
pub(crate) fn advance_on_enter(
    ui: &mut egui::Ui,
    sketch: &mut SketchContext<'_>,
    scale: ViewScale,
) -> bool {
    let drawing = matches!(
        sketch.editor.tool_state,
        ToolState::Line { .. }
            | ToolState::SymmetricLine { .. }
            | ToolState::Rectangle { .. }
            | ToolState::Circle { .. }
            | ToolState::Arc { .. }
            | ToolState::Corner { .. }
    );
    if !drawing || !paint_live_input(ui, sketch) {
        return false;
    }
    let Some(index) = sketch.editor.active_sketch() else {
        return false;
    };
    if let Some((first, second)) = corner_held(&sketch.editor.tool_state) {
        return cut_the_corner(sketch, index, first, second);
    }
    let raw_cursor = sketch.editor.cursor.unwrap_or_default();
    let aim = sketch.editor.aimed;
    let cursor = match sketch.editor.tool {
        Tool::Line => aim.map_or(raw_cursor, |aimed| aimed.position),
        Tool::Rectangle => rectangle_corner(sketch, raw_cursor),
        _ => raw_cursor,
    };
    let snap = scale.world_size_of(PICK_PIXELS);
    match sketch.editor.tool {
        Tool::Line => draw_line_point(sketch, index, cursor, snap, scale.units_per_pixel),
        Tool::LineSymmetric => {
            draw_symmetric_line_point(sketch, index, cursor, snap, scale.units_per_pixel)
        }
        Tool::Circle => draw_circle(sketch, index, cursor, snap, scale.units_per_pixel),
        Tool::Arc => draw_arc(sketch, index, cursor, snap, scale.units_per_pixel),
        _ => two_click_shape(sketch, index, cursor, snap, scale.units_per_pixel),
    }
}
