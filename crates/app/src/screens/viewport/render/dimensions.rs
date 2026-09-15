//! The value each dimension carries, written over the canvas by egui, and the
//! field that edits the one in hand. The lines and arrows an annotation is made
//! of are pushed into the scene by [`crate::screens::annotations`]; only the
//! text is here.

use cao_sketch::{ChamferMode, DimensionTarget, ToolState};

use crate::screens::sketch::{LiveField, Tool, apply_dimension_value};

use super::super::input::rectangle_corner;
use super::super::{PICK_PIXELS, SketchContext, ViewportState};
use super::{arc, circle, live_offset, pending_annotation, symmetric_line, to_screen};

/// Each dimension is drawn where it applies, with the value it stands for.
/// A readout shows what the geometry measures rather than a stored number, so
/// it stays true however the drawing moves.
pub(crate) fn paint_dimension_labels(
    ui: &egui::Ui,
    state: &ViewportState,
    rect: egui::Rect,
    context: &SketchContext<'_>,
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
            if matches!(
                target,
                DimensionTarget::Angle { .. }
                    | DimensionTarget::AxisAngle { .. }
                    | DimensionTarget::ArcSweep(_)
            ) {
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
    let angle = matches!(
        target,
        DimensionTarget::Angle { .. }
            | DimensionTarget::AxisAngle { .. }
            | DimensionTarget::ArcSweep(_)
    );

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

/// The length and the angle of the line being drawn, editable on the spot.
///
/// Left alone they only report. Typed into, they become constraints and are
/// placed as dimensions when the line is validated — which is the whole point:
/// a line drawn to a value should not have to be measured afterwards.
///
/// Returns true when the user pressed Enter to finish the line from the
/// keyboard.
pub(crate) fn paint_live_input(ui: &mut egui::Ui, context: &mut SketchContext<'_>) -> bool {
    paint_live_fields(ui, context).unwrap_or(false)
}

/// The same, written where a missing piece simply means there is nothing to show yet.
fn paint_live_fields(ui: &mut egui::Ui, context: &mut SketchContext<'_>) -> Option<bool> {
    let index = context.editor.active_sketch()?;
    let sketch = context.document.sketches().get(index)?;
    let raw_cursor = context.editor.cursor?;
    let aim = context.editor.aimed;
    let cursor = aim.map_or(raw_cursor, |aimed| aimed.position);
    let scale = context.document.scale();

    // A line is a length and an angle, a rectangle its two sides, a circle its
    // diameter and nothing else — so its second field is left out.
    let fields: Vec<(&'static str, f64)> = match context.editor.tool {
        Tool::Circle => pair(circle::live_fields(context, index, cursor, scale)?),
        Tool::Arc => pair(arc::live_fields(context, cursor)?),
        Tool::Line => {
            let from = sketch.anchor_position(context.editor.chain()?)?;
            let span = cursor - from;
            pair((
                ["mm", "°"],
                [span.length() * scale, span.y.atan2(span.x).to_degrees()],
            ))
        }
        Tool::Rectangle => {
            let start = context.editor.pending_start()?;
            let far = rectangle_corner(context, raw_cursor);
            let span = far - start;
            pair((["mm", "mm"], [span.x.abs() * scale, span.y.abs() * scale]))
        }
        Tool::LineSymmetric => pair(symmetric_line::live_fields(context, sketch, raw_cursor)?),
        Tool::CircularPattern => match context.editor.tool_state {
            // The step between one copy and the next, and how many stand there
            // in the end. Nothing is read off the cursor: a pattern is only
            // ever what is typed.
            ToolState::Mirror {
                naming_the_axis: true,
                ..
            } => pair((["°", "×"], [0.0; 2])),
            _ => return None,
        },
        Tool::Chamfer | Tool::Fillet => match context.editor.tool_state {
            // Nothing is read off the cursor: a corner tool is only ever what
            // is typed, so the fields stand empty until they are.
            ToolState::Corner { .. } => pair((corner_units(context.editor), [0.0; 2])),
            _ => return None,
        },
        _ => return None,
    };

    // Hung off the pointer itself, down and to the right: anchored on the
    // drawing, the fields ended up under the cursor, and a cursor over them is
    // a cursor no longer over the canvas — the shape stopped following it.
    let at = ui.ctx().pointer_latest_pos()?;

    let live = &mut context.editor.live;
    let mut validated = false;
    let focus = std::mem::take(&mut live.focus);
    egui::Area::new(egui::Id::new("live_input"))
        .fixed_pos(at + egui::vec2(20.0, 20.0))
        .order(egui::Order::Foreground)
        .show(ui.ctx(), |ui| {
            egui::Frame::popup(ui.style()).show(ui, |ui| {
                ui.horizontal(|ui| {
                    for (rank, (label, measured)) in fields.into_iter().enumerate() {
                        let first = rank == 0;
                        validated |=
                            live_field(ui, label, measured, live.field(rank), focus && first);
                    }
                });
            });
        });
    Some(validated)
}

/// The two values a tool shows, ready to draw. A tool with only one of them —
/// a circle is a diameter and nothing else — leaves the second label empty.
fn pair((labels, measured): ([&'static str; 2], [f64; 2])) -> Vec<(&'static str, f64)> {
    labels
        .into_iter()
        .zip(measured)
        .filter(|(label, _)| !label.is_empty())
        .collect()
}

/// A number field that takes the keyboard on demand, whole value selected.
///
/// Selecting it matters: the field arrives holding the value already there, and
/// without it the first keystroke lands after it — 40 typed over 60.88 read
/// 60.8840.
fn value_field(ui: &mut egui::Ui, text: &mut String, hint: &str, focus: bool) -> egui::Response {
    let output = egui::TextEdit::singleline(text)
        .desired_width(72.0)
        .hint_text(hint)
        .show(ui);
    let response = output.response.response;
    if focus {
        response.request_focus();
        let mut state = output.state;
        state
            .cursor
            .set_char_range(Some(egui::text::CCursorRange::two(
                egui::text::CCursor::new(0),
                egui::text::CCursor::new(text.chars().count()),
            )));
        state.store(ui.ctx(), response.id);
    }
    response
}

/// One of the two fields. Returns true when Enter was pressed in it.
///
/// An untouched field stays empty and shows what the cursor is doing as a hint:
/// keeping the readout in the field itself meant the first keystroke landed
/// after it, and "40" typed over "0.000" read 0.00040.
fn live_field(
    ui: &mut egui::Ui,
    suffix: &str,
    measured: f64,
    field: &mut LiveField,
    focus: bool,
) -> bool {
    // The keyboard goes to the first field as soon as the fields appear: the
    // value is the next thing the user types, and Tab from the canvas walks
    // through the whole toolbar to get here.
    let response = value_field(ui, &mut field.text, &format!("{measured:.2}"), focus);
    ui.label(suffix);

    // Typing is what turns a readout into a decision. Emptying the field takes the decision back.
    if response.changed() {
        field.locked = crate::screens::sketch::LiveInput::read(&field.text);
        // The canvas this frame was already built from the value as it stood before this keystroke.
        ui.ctx().request_repaint();
    }
    // Enter is consumed rather than merely read: the field has just given the
    // keyboard back, so the shortcut bound to that key would fire too.
    response.lost_focus()
        && ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Enter))
}

/// What a corner tool's two fields are measured in, which is what says how many
/// of them there are. A fillet asks for a radius and nothing else.
fn corner_units(editor: &crate::screens::sketch::SketchEditor) -> [&'static str; 2] {
    match editor.tool {
        Tool::Fillet => ["mm", ""],
        _ => match editor.chamfer_mode {
            ChamferMode::Equal => ["mm", ""],
            ChamferMode::Angled => ["mm", "°"],
            ChamferMode::Sided => ["mm", "mm"],
        },
    }
}
