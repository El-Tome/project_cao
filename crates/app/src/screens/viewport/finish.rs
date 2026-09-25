//! What Enter does to the shape in hand, once the live fields have had their
//! say.

use cao_sketch::ToolState;

use super::input::{
    CornerEnter, corner_on_enter, cut_the_corner, draw_arc, draw_circle, draw_ellipse,
    draw_line_point, draw_symmetric_line_point, hold_is_done, rectangle_corner, two_click_shape,
};
use super::render::paint_live_input;
use super::values::refused_for_what_is_typed;
use super::{PICK_PIXELS, ViewScale};
use crate::screens::SketchContext;
use crate::screens::sketch::Tool;

/// Enter finishes the shape from the keyboard, without having to find the canvas again with the
/// mouse. What the shape ends at follows the same reading as a click would: the line's aim, or the
/// rectangle's corner once what was typed has had its say.
pub(crate) fn advance_on_enter(
    ui: &mut egui::Ui,
    sketch: &mut SketchContext<'_>,
    scale: ViewScale,
) -> bool {
    // While what is held is gathered there is no field for Enter to land in,
    // so the key is read here rather than handed on by one. Once it is done, a
    // pattern shows the values it is laid with, and the click that names
    // where lays it: Enter only settles the field it was pressed in.
    if let ToolState::Copying {
        naming_the_target, ..
    } = sketch.editor.tool_state
    {
        if naming_the_target {
            paint_live_input(ui, sketch);
            return false;
        }
        return ui.input(|input| input.key_pressed(egui::Key::Enter)) && hold_is_done(sketch);
    }

    let drawing = matches!(
        sketch.editor.tool_state,
        ToolState::Line { .. }
            | ToolState::SymmetricLine { .. }
            | ToolState::Rectangle { .. }
            | ToolState::Circle { .. }
            | ToolState::Arc { .. }
            | ToolState::Ellipse { .. }
            | ToolState::Corner { .. }
    );
    if !drawing || !paint_live_input(ui, sketch) || refused_for_what_is_typed(sketch) {
        return false;
    }
    let Some(index) = sketch.editor.active_sketch() else {
        return false;
    };
    // The corner tools answer for their own key. Handed on, it reached a tool
    // that put its own state where the corner's was, and the side already
    // clicked was lost.
    if matches!(sketch.editor.tool, Tool::Chamfer | Tool::Fillet) {
        return match corner_on_enter(&sketch.editor.tool_state) {
            CornerEnter::Cut => cut_the_corner(sketch, index),
            CornerEnter::Waiting(say) => {
                sketch.editor.message = Some(sketch.lang.t(say));
                false
            }
        };
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
        Tool::Ellipse => draw_ellipse(sketch, index, cursor, snap, scale.units_per_pixel),
        _ => two_click_shape(sketch, index, cursor, snap, scale.units_per_pixel),
    }
}
