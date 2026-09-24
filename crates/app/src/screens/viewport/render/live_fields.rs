//! The values a shape is drawn to, shown beside the cursor and editable on the
//! spot: a length and an angle for a line, the two steps and the two counts of
//! a rectangular pattern.

use cao_sketch::{ChamferMode, ToolState};

use crate::screens::sketch::{LiveField, LiveInput, Tool};
use crate::ui::formula_field::formula_field;

use super::super::input::rectangle_corner;
use super::{arc, circle, ellipse, symmetric_line};
use crate::screens::SketchContext;

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
    let (labels, measured): ([&'static str; 4], [f64; 4]) = match context.editor.tool {
        Tool::Circle => two(circle::live_fields(context, index, cursor, scale)?),
        Tool::Arc => two(arc::live_fields(context, cursor)?),
        Tool::Ellipse => two(ellipse::live_fields(context, raw_cursor)?),
        Tool::Line => {
            let from = sketch.anchor_position(context.editor.chain()?)?;
            let span = cursor - from;
            two((
                ["mm", "°"],
                [span.length() * scale, span.y.atan2(span.x).to_degrees()],
            ))
        }
        Tool::Rectangle => {
            let start = context.editor.pending_start()?;
            let far = rectangle_corner(context, raw_cursor);
            let span = far - start;
            two((["mm", "mm"], [span.x.abs() * scale, span.y.abs() * scale]))
        }
        Tool::LineSymmetric => two(symmetric_line::live_fields(context, sketch, raw_cursor)?),
        Tool::CircularPattern => match context.editor.tool_state {
            // The step between one copy and the next, and how many stand there
            // in the end. Nothing is read off the cursor: a pattern is only
            // ever what is typed.
            ToolState::Copying {
                naming_the_target: true,
                ..
            } => two((["°", "×"], [0.0; 2])),
            _ => return None,
        },
        Tool::RectangularPattern => match context.editor.tool_state {
            ToolState::Copying {
                naming_the_target: true,
                ..
            } => (["mm", "×", "mm", "×"], [0.0; 4]),
            _ => return None,
        },
        Tool::Chamfer | Tool::Fillet => match context.editor.tool_state {
            // Nothing is read off the cursor: a corner tool is only ever what
            // is typed, so the fields stand empty until they are.
            ToolState::Corner { .. } => two((corner_units(context.editor), [0.0; 2])),
            _ => return None,
        },
        _ => return None,
    };

    // Hung off the pointer itself, down and to the right: anchored on the
    // drawing, the fields ended up under the cursor, and a cursor over them is
    // a cursor no longer over the canvas — the shape stopped following it.
    let at = ui.ctx().pointer_latest_pos()?;

    let variables = context.document.variables();
    let live = &mut context.editor.live;
    let mut validated = false;
    let focus = std::mem::take(&mut live.focus);
    egui::Area::new(egui::Id::new("live_input"))
        .fixed_pos(at + egui::vec2(20.0, 20.0))
        .order(egui::Order::Foreground)
        .show(ui.ctx(), |ui| {
            egui::Frame::popup(ui.style()).show(ui, |ui| {
                ui.horizontal(|ui| {
                    for (rank, label) in labels.into_iter().enumerate() {
                        if label.is_empty() {
                            continue;
                        }
                        let first = rank == 0;
                        validated |= live_field(
                            ui,
                            (label, measured[rank]),
                            live,
                            rank,
                            focus && first,
                            variables,
                        );
                    }
                });
            });
        });
    Some(validated)
}

/// A tool that shows two values, among the four there is room for. The ones it
/// leaves out are drawn by nobody: an empty label is a field that is not there.
fn two((labels, measured): ([&'static str; 2], [f64; 2])) -> ([&'static str; 4], [f64; 4]) {
    (
        [labels[0], labels[1], "", ""],
        [measured[0], measured[1], 0.0, 0.0],
    )
}

/// A number field that takes the keyboard on demand, whole value selected.
///
/// Selecting it matters: the field arrives holding the value already there, and
/// without it the first keystroke lands after it — 40 typed over 60.88 read
/// 60.8840. That holds for a field reached with Tab as much as for the one the
/// keyboard opens on, so the value goes whole there too; a field reached with a
/// click keeps the place the click named.
pub(super) fn value_field(
    ui: &mut egui::Ui,
    id: egui::Id,
    text: &mut String,
    hint: &str,
    focus: bool,
    variables: &cao_part::Variables,
) -> egui::text_edit::TextEditOutput {
    let offers = || crate::screens::variables::offered(variables);
    let output = formula_field(
        ui,
        id,
        text,
        (72.0, hint),
        &offers,
        crate::screens::variables::NAMING,
    );
    let response = output.response.response.clone();
    let reached_by_keyboard =
        focus || (response.gained_focus() && !response.is_pointer_button_down_on());
    if focus {
        response.request_focus();
    }
    if reached_by_keyboard {
        let mut state = output.state.clone();
        state
            .cursor
            .set_char_range(Some(egui::text::CCursorRange::two(
                egui::text::CCursor::new(0),
                egui::text::CCursor::new(text.chars().count()),
            )));
        state.store(ui.ctx(), response.id);
    }
    output
}

/// One of the two fields. Returns true when Enter was pressed in it.
///
/// An untouched field stays empty and shows what the cursor is doing as a hint:
/// keeping the readout in the field itself meant the first keystroke landed
/// after it, and "40" typed over "0.000" read 0.00040.
fn live_field(
    ui: &mut egui::Ui,
    (suffix, measured): (&str, f64),
    live: &mut LiveInput,
    rank: usize,
    focus: bool,
    variables: &cao_part::Variables,
) -> bool {
    let id = field_id(rank);
    let field: &mut LiveField = live.field(rank);
    // The keyboard goes to the first field as soon as the fields appear: the
    // value is the next thing the user types, and Tab from the canvas walks
    // through the whole toolbar to get here.
    let response = value_field(
        ui,
        id,
        &mut field.text,
        &format!("{measured:.2}"),
        focus,
        variables,
    )
    .response
    .response;
    ui.label(suffix);

    // Typing is what turns a readout into a decision. Emptying the field takes the decision back.
    if response.changed() {
        field.take(variables);
        // The canvas this frame was already built from the value as it stood before this keystroke.
        ui.ctx().request_repaint();
    }
    // Enter is consumed rather than merely read: the field has just given the
    // keyboard back, so the shortcut bound to that key would fire too.
    response.lost_focus()
        && ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Enter))
}

fn field_id(rank: usize) -> egui::Id {
    egui::Id::new(("live_field", rank))
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
