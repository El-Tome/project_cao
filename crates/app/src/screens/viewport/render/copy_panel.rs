//! The panel of the mirror and the two patterns: its slots top to bottom, the
//! active one shown as such, and the one `Entrée` found empty marked.

use cao_prefs::Command;

use super::super::input::copying::enter_in_the_panel;
use super::super::input::count_brought_down;
use super::live_fields::value_field;
use crate::screens::SketchContext;
use crate::screens::sketch::{CopySlot, CopyValue, Tool};
use crate::ui::floating_panel::{floating_panel, panel_row};
use crate::wording::copy_panel::{self as said, Target};

/// What a gesture in the panel asked for this frame.
#[derive(Default)]
struct Asked {
    activate: Option<CopySlot>,
    enter: bool,
    cancel: bool,
}

/// Draws the panel while a copy tool is in hand, and carries out what was done
/// in it: whether a copy was laid, and where the panel was let go this frame.
pub(crate) fn paint_copy_panel(
    ui: &egui::Ui,
    left_at: Option<[f32; 2]>,
    view: egui::Rect,
    context: &mut SketchContext<'_>,
) -> (bool, Option<[f32; 2]>) {
    let Some(active) = context.editor.copy_panel().map(|panel| panel.active()) else {
        return (false, None);
    };
    let command = match context.editor.tool {
        Tool::CircularPattern => Command::ToolCircularPattern,
        Tool::RectangularPattern => Command::ToolRectangularPattern,
        _ => Command::ToolMirror,
    };
    let title = crate::wording::command::label(context.lang, command);
    let left_at = left_at.map_or(view.min + egui::vec2(16.0, 16.0), |[x, y]| egui::pos2(x, y));
    let id = egui::Id::new("copy_panel");
    let arrived = ui.ctx().data_mut(|data| {
        let before = data.get_temp::<CopySlot>(id.with("active"));
        data.insert_temp(id.with("active"), active);
        before != Some(active)
    });
    let close = context.lang.t("panel.close");
    let floated = floating_panel(ui.ctx(), id, (&title, &close), (left_at, view), |ui| {
        slots(ui, context, arrived)
    });
    let let_go_at = floated.let_go_at.map(|at| [at.x, at.y]);

    let asked = floated.inner;
    if asked.cancel || floated.closed {
        context.editor.cancel_the_copy();
        return (false, let_go_at);
    }
    if let (Some(slot), Some(panel)) = (asked.activate, context.editor.copy_panel_mut()) {
        panel.activate(slot);
    }
    let bare_enter = ui.input(|input| input.key_pressed(egui::Key::Enter));
    let laid = (asked.enter || bare_enter) && enter_in_the_panel(context);
    (laid, let_go_at)
}

fn slots(ui: &mut egui::Ui, context: &mut SketchContext<'_>, arrived: bool) -> Asked {
    let mut asked = Asked::default();
    let (lang, tool) = (context.lang, context.editor.tool);
    let Some(panel) = context.editor.copy_panel() else {
        return asked;
    };
    let (active, marked, kinds) = (panel.active(), panel.marked(), panel.value_kinds());
    let shown = |slot: CopySlot, said: String| Row {
        slot,
        said,
        active: active == slot,
        marked: marked == Some(slot),
    };

    let target = match tool {
        Tool::CircularPattern => Target::Centre,
        Tool::RectangularPattern => Target::Direction,
        _ => Target::Axis,
    };
    let chosen = panel.target().is_some();
    for row in [
        shown(
            CopySlot::Elements,
            said::elements(lang, panel.elements().len()),
        ),
        shown(CopySlot::Target, said::target(lang, target, chosen)),
    ] {
        asked.activate = row.show(ui).or(asked.activate);
    }

    let variables = context.document.variables();
    let live = &mut context.editor.live;
    for (rank, kind) in kinds.iter().enumerate() {
        let here = CopySlot::Value(rank);
        let (named, unit) = match kind {
            CopyValue::StepInMillimetres => ("copy_panel.step", "mm"),
            CopyValue::StepInDegrees => ("copy_panel.step", "°"),
            CopyValue::Count => ("copy_panel.count", "×"),
        };
        let named = lang.t(named);
        ui.horizontal(|ui| {
            asked.activate = shown(here, named).show(ui).or(asked.activate);
            let id = egui::Id::new(("copy_panel_value", rank));
            let focus = arrived && active == here;
            let field = value_field(ui, id, &mut live.field(rank).text, "", focus, variables);
            let response = field.response.response;
            ui.label(unit);
            if response.gained_focus() {
                asked.activate = Some(here);
            }
            if response.changed() {
                live.take(rank, variables, None);
                ui.ctx().request_repaint();
            }
            asked.enter |= response.lost_focus()
                && ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Enter));
        });
    }
    if tool == Tool::CircularPattern
        && let Some(largest) = count_brought_down(live, variables)
    {
        ui.weak(crate::wording::pattern::count_brought_down(lang, largest));
    }
    if let Some(panel) = context.editor.copy_panel.as_mut() {
        panel.unmark_if_filled(&context.editor.live);
    }

    ui.horizontal(|ui| {
        let validate = shown(CopySlot::Validate, lang.t("copy_panel.validate"));
        if validate.show(ui).is_some() {
            asked.activate = Some(CopySlot::Validate);
            asked.enter = true;
        }
        asked.cancel = ui.button(lang.t("copy_panel.cancel")).clicked();
    });
    asked
}

/// One slot of the panel, as a line that makes it active when clicked.
struct Row {
    slot: CopySlot,
    said: String,
    active: bool,
    marked: bool,
}

impl Row {
    fn show(self, ui: &mut egui::Ui) -> Option<CopySlot> {
        let clicked = panel_row(ui, self.said, self.active, self.marked).clicked();
        clicked.then_some(self.slot)
    }
}
