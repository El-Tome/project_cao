//! The value each dimension carries, written over the canvas by egui, and the
//! field that edits the one in hand. The lines and arrows an annotation is made
//! of are pushed into the scene by [`crate::screens::annotations`]; only the
//! text is here.

use cao_sketch::{DimensionTarget, Going};

use crate::screens::sketch::apply_dimension_value;

use super::super::{PICK_PIXELS, ViewportState};
use super::live_fields::value_field;
use super::overlays::{tint_to_color as to_color_of, to_screen};
use super::{live_offset, pending_annotation};
use crate::screens::SketchContext;

/// Each dimension is drawn where it applies, with the value it stands for.
/// A readout shows what the geometry measures rather than a stored number, so
/// it stays true however the drawing moves.
pub(crate) fn paint_dimension_labels(
    ui: &egui::Ui,
    state: &ViewportState,
    rect: egui::Rect,
    context: &SketchContext<'_>,
    going: Option<&Going>,
) {
    let Some(index) = context.editor.active_sketch() else {
        return;
    };
    // Mid-drag the annotations are drawn from the settled preview, so their
    // values have to be read from the same drawing — otherwise the numbers stay
    // behind while the lines they belong to move away.
    let Some(sketch) = context
        .editor
        .drag_preview()
        .or_else(|| context.document.sketches().get(index))
    else {
        return;
    };
    let view_projection = state
        .camera
        .view_projection(rect.width() / rect.height().max(1.0));
    let painter = ui.painter_at(rect);

    let pixel = state
        .camera
        .world_units_per_pixel(rect.height() * ui.ctx().pixels_per_point()) as f64;

    for dimension in sketch.dimensions() {
        // Asking the annotation where its value belongs keeps the text on the
        // dimension line instead of floating near the geometry.
        let mut ignored = Vec::new();
        let style = crate::screens::annotations::Style::driving(&state.theme);
        let Some(text_at) = crate::screens::annotations::push(
            &mut ignored,
            sketch,
            dimension.target,
            &style,
            pixel,
            live_offset(context, dimension.target),
        ) else {
            continue;
        };
        let Some(position) = to_screen(sketch.plane.to_world(text_at), view_projection, rect)
        else {
            continue;
        };
        let value = if dimension.driven {
            context
                .document
                .measured(index, dimension.target)
                .unwrap_or(dimension.value)
        } else {
            dimension.value
        };
        let text = if dimension.is_angle() {
            format!("{value:.1}°")
        } else {
            state.config.unit.format(value)
        };
        let color = if dimension.driven {
            egui::Color32::from_gray(170)
        } else {
            egui::Color32::from_rgb(250, 220, 120)
        };
        let color = match going.is_some_and(|going| going.values.contains(&dimension.target)) {
            true => to_color_of(state.theme.going),
            false => color,
        };
        painter.text(
            position,
            egui::Align2::CENTER_CENTER,
            if dimension.driven {
                format!("({text})")
            } else {
                text
            },
            egui::FontId::proportional(13.0),
            color,
        );
    }

    // The dimension being placed carries its value with it: a bare pair of
    // arrows says nothing about what is being measured.
    if context.editor.placing().is_some()
        && let Some(cursor) = context.editor.cursor
        && let Some((target, nudge)) =
            pending_annotation(context, index, cursor, pixel * PICK_PIXELS, pixel)
        && let Some(value) = context.document.measured(index, target)
        && let Some(text_at) = crate::screens::annotations::push(
            &mut Vec::new(),
            sketch,
            target,
            &crate::screens::annotations::Style::driving(&state.theme),
            pixel,
            nudge,
        )
        && let Some(position) = to_screen(sketch.plane.to_world(text_at), view_projection, rect)
    {
        painter.text(
            position,
            egui::Align2::CENTER_CENTER,
            if target.is_angle() {
                format!("{value:.1}°")
            } else {
                state.config.unit.format(value)
            },
            egui::FontId::proportional(13.0),
            egui::Color32::from_rgb(250, 220, 120),
        );
    }
}

/// The value of the dimension in hand, written right where that dimension is.
///
/// It used to sit in the title bar, an arm's length from the drawing: the eyes
/// had to leave the shape being measured to find the number belonging to it.
/// Returns true when a value was applied.
pub(crate) fn paint_dimension_field(
    ui: &mut egui::Ui,
    state: &ViewportState,
    rect: egui::Rect,
    context: &mut SketchContext<'_>,
) -> bool {
    let (Some(index), Some(target)) = (context.editor.active_sketch(), context.editor.selected())
    else {
        return false;
    };
    let pixel = state
        .camera
        .world_units_per_pixel(rect.height() * ui.ctx().pixels_per_point()) as f64;
    let Some(at) = annotation_screen_position(state, rect, context, index, target, pixel) else {
        return false;
    };

    let driven = context.document.sketches()[index]
        .dimension_of(target)
        .is_some_and(|dimension| dimension.driven);
    let angle = target.is_angle();

    let mut applied = false;
    let lang = context.lang;
    egui::Area::new(egui::Id::new("dimension_field"))
        .fixed_pos(at + egui::vec2(16.0, 12.0))
        .order(egui::Order::Foreground)
        .show(ui.ctx(), |ui| {
            egui::Frame::popup(ui.style()).show(ui, |ui| {
                ui.horizontal(|ui| {
                    if driven {
                        // A readout cannot be edited: changing it would mean
                        // nothing, since it reports the geometry rather than deciding it.
                        let measured = context.document.measured(index, target).unwrap_or_default();
                        let unit = if angle { "°" } else { "mm" };
                        let value = format!("{measured:.2}");
                        let holes = [("value", value.as_str()), ("unit", unit)];
                        ui.weak(lang.t_with("viewport.read_only", &holes));
                        return;
                    }
                    let Some(editing) = context.editor.editing.as_mut() else {
                        return;
                    };
                    let hint = if angle {
                        lang.t("viewport.degrees")
                    } else {
                        "mm".to_string()
                    };
                    let focus = std::mem::take(&mut editing.focus);
                    let field = value_field(ui, &mut editing.input, &hint, focus);
                    ui.weak(&hint); // the field opens pre-filled, so its own hint_text never draws
                    // Enter is eaten here: the field has just given the keyboard
                    // back, so the same press would otherwise also fire the
                    // shortcut bound to it — and end the sketch.
                    let submitted = field.lost_focus()
                        && ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Enter));
                    let apply = ui.button("✔").on_hover_text(lang.t("viewport.apply"));
                    applied = apply.clicked() || submitted;
                });
            });
        });

    if !applied || !apply_dimension_value(context.document, context.editor, index, target, lang) {
        return false;
    }
    context.editor.editing = None;
    true
}

/// Where an annotation writes its value, on screen.
fn annotation_screen_position(
    state: &ViewportState,
    rect: egui::Rect,
    context: &SketchContext<'_>,
    index: usize,
    target: DimensionTarget,
    pixel: f64,
) -> Option<egui::Pos2> {
    let sketch = context.document.sketches().get(index)?;
    let mut ignored = Vec::new();
    let text_at = crate::screens::annotations::push(
        &mut ignored,
        sketch,
        target,
        &crate::screens::annotations::Style::driving(&state.theme),
        pixel,
        live_offset(context, target),
    )?;
    to_screen(
        sketch.plane.to_world(text_at),
        state
            .camera
            .view_projection(rect.width() / rect.height().max(1.0)),
        rect,
    )
}
