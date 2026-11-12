//! The circular pattern's panel, as P13 of `docs/redesign/part-screen.md`
//! draws it: four numbered steps — the elements to copy, the centre, the
//! number and the step, the direction — then « Annuler » and « Valider ↵ ».

use cao_sketch::PointId;

use super::super::super::input::count_brought_down;
use super::super::live_fields::{taken_whole_when_reached, value_field};
use super::Asked;
use crate::lang::Catalogue;
use crate::screens::SketchContext;
use crate::screens::sketch::{CopyPanel, CopySlot, CopyStep, CopyTarget, Rotation, StepState};
use crate::ui::chip::{ChipKind, chip};
use crate::ui::number_field::number_field;
use crate::ui::primary_button::primary_button;
use crate::ui::quiet_link::quiet_link;
use crate::ui::secondary_button::secondary_button;
use crate::ui::step_dot::{Progress, step_dot};
use crate::ui::theme::tokens::{SPACING_M, SPACING_XXL};
use crate::wording::copy_panel as said;

/// The ranks of the pattern's two values among the panel's fields.
const STEP: usize = 0;
const COUNT: usize = 1;

/// The fewest copies − leaves in the field: the original and one more.
const LEAST_COUNT: f64 = 2.0;

/// How tall C15 stands the panel's two buttons.
const BUTTON_HEIGHT: f32 = 32.0;

/// What the panel holds, read before anything of it is drawn.
struct Held {
    steps: Vec<(CopyStep, StepState)>,
    active: CopySlot,
    marked: Option<CopySlot>,
    picked: Vec<String>,
    centre: Option<String>,
    rotation: Rotation,
}

impl Held {
    fn of(panel: &CopyPanel, context: &SketchContext<'_>) -> Self {
        let centre = match panel.target() {
            Some(CopyTarget::Centre(point)) => centre_at(context, point),
            _ => None,
        };
        Self {
            steps: panel.steps(&context.editor.live),
            active: panel.active(),
            marked: panel.marked(),
            picked: panel
                .picked()
                .map(|element| said::element(context.lang, element))
                .collect(),
            centre,
            rotation: panel.rotation(),
        }
    }
}

/// Draws the four steps and the two buttons, and says what was asked in them.
pub(super) fn steps(ui: &mut egui::Ui, context: &mut SketchContext<'_>, arrived: bool) -> Asked {
    let mut asked = Asked::default();
    let Some(panel) = context.editor.copy_panel() else {
        return asked;
    };
    let held = Held::of(panel, context);
    let lang = context.lang;

    ui.spacing_mut().item_spacing.y = SPACING_XXL;
    for (rank, (step, state)) in held.steps.iter().copied().enumerate() {
        ui.horizontal_top(|ui| {
            if step_dot(ui, rank + 1, progress(state)).clicked() {
                asked.activate = Some(first_slot(step));
            }
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing = egui::vec2(SPACING_M, SPACING_M);
                match step {
                    CopyStep::Elements => elements(ui, lang, &held, &mut asked),
                    CopyStep::Centre => centre(ui, lang, &held, &mut asked),
                    CopyStep::CountAndStep => {
                        count_and_step(ui, context, (&held, arrived), &mut asked);
                    }
                    CopyStep::Rotation => rotation(ui, lang, &held, &mut asked),
                }
            });
        });
    }
    if let Some(panel) = context.editor.copy_panel.as_mut() {
        panel.unmark_if_filled(&context.editor.live);
    }
    footer(ui, lang, &mut asked);
    asked
}

fn progress(state: StepState) -> Progress {
    match state {
        StepState::Done => Progress::Done,
        StepState::Current => Progress::Current,
        StepState::ToCome => Progress::ToCome,
    }
}

/// The slot a click on a step's dot makes active: the first it asks for.
fn first_slot(step: CopyStep) -> CopySlot {
    match step {
        CopyStep::Elements => CopySlot::Elements,
        CopyStep::Centre => CopySlot::Target,
        CopyStep::CountAndStep => CopySlot::Value(STEP),
        CopyStep::Rotation => CopySlot::Rotation,
    }
}

/// The centre chosen, by where it stands on the sketch.
fn centre_at(context: &SketchContext<'_>, point: PointId) -> Option<String> {
    let index = context.editor.active_sketch()?;
    let at = *context
        .document
        .sketches()
        .get(index)?
        .points()
        .get(point.0)?;
    let millimetres = |units| context.document.to_millimeters(units);
    Some(said::centre_at(
        context.lang,
        (millimetres(at.x), millimetres(at.y)),
    ))
}

/// A step's title, warned while `Entrée` found its step empty. Says whether
/// it was clicked.
fn title(ui: &mut egui::Ui, said: String, marked: bool) -> bool {
    let warn = ui.visuals().warn_fg_color;
    let shown = match marked {
        true => ui.colored_label(warn, said),
        false => ui.label(said),
    };
    shown.interact(egui::Sense::click()).clicked()
}

fn elements(ui: &mut egui::Ui, lang: &Catalogue, held: &Held, asked: &mut Asked) {
    let marked = held.marked == Some(CopySlot::Elements);
    let mut clicked = title(ui, lang.t("copy_panel.elements_to_copy"), marked);
    ui.horizontal_wrapped(|ui| {
        for name in &held.picked {
            clicked |= chip(ui, name, ChipKind::Selection).clicked();
        }
        let invited = lang.t("copy_panel.click_in_the_view");
        clicked |= chip(ui, &invited, ChipKind::Invitation).clicked();
    });
    if clicked {
        asked.activate = Some(CopySlot::Elements);
    }
}

fn centre(ui: &mut egui::Ui, lang: &Catalogue, held: &Held, asked: &mut Asked) {
    let marked = held.marked == Some(CopySlot::Target);
    let mut clicked = title(ui, lang.t("copy_panel.centre"), marked);
    ui.horizontal(|ui| match &held.centre {
        Some(at) => {
            clicked |= chip(ui, at, ChipKind::Selection).clicked();
            clicked |= quiet_link(ui, &lang.t("copy_panel.change")).clicked();
        }
        None => {
            let invited = lang.t("copy_panel.click_in_the_view");
            clicked |= chip(ui, &invited, ChipKind::Invitation).clicked();
        }
    });
    if clicked {
        asked.activate = Some(CopySlot::Target);
    }
}

/// « Nombre » with its − and +, and « Pas » with what it comes to, side by
/// side; under them, what a count brought down to a full turn says.
fn count_and_step(
    ui: &mut egui::Ui,
    context: &mut SketchContext<'_>,
    (held, arrived): (&Held, bool),
    asked: &mut Asked,
) {
    let (lang, variables) = (context.lang, context.document.variables());
    let live = &mut context.editor.live;
    let reached = |rank| arrived && held.active == CopySlot::Value(rank);
    ui.columns(2, |columns| {
        let [count, step] = columns else {
            return;
        };
        if title(count, lang.t("copy_panel.count"), false) {
            asked.activate = Some(CopySlot::Value(COUNT));
        }
        let current = held.active == CopySlot::Value(COUNT);
        let typed = &mut live.field(COUNT).text;
        let field = number_field(count, typed, current, LEAST_COUNT);
        taken_whole_when_reached(count.ctx(), &field, typed, reached(COUNT));
        typed_into(count, &field, COUNT, asked);
        if field.changed() {
            live.take(COUNT, variables, None);
        }

        if title(step, lang.t("copy_panel.step"), false) {
            asked.activate = Some(CopySlot::Value(STEP));
        }
        step.horizontal(|ui| {
            let id = egui::Id::new(("copy_panel_value", STEP));
            let typed = &mut live.field(STEP).text;
            let field = value_field(ui, id, typed, "", reached(STEP), variables);
            let field = field.response.response;
            typed_into(ui, &field, STEP, asked);
            if field.changed() {
                live.take(STEP, variables, None);
            }
            if let Some(degrees) = live.typed(STEP) {
                ui.weak(crate::wording::dimension::degrees(degrees));
            }
        });
    });
    if let Some(largest) = count_brought_down(live, variables) {
        ui.weak(crate::wording::pattern::count_brought_down(lang, largest));
    }
}

/// What a field of the panel asks for: its slot once it is reached or typed
/// into, and `Entrée` when the keyboard leaves it on that key.
fn typed_into(ui: &egui::Ui, field: &egui::Response, rank: usize, asked: &mut Asked) {
    if field.gained_focus() || field.changed() {
        asked.activate = Some(CopySlot::Value(rank));
    }
    if field.changed() {
        ui.ctx().request_repaint();
    }
    asked.enter |= field.lost_focus()
        && ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Enter));
}

fn rotation(ui: &mut egui::Ui, lang: &Catalogue, held: &Held, asked: &mut Asked) {
    if title(ui, lang.t("copy_panel.rotation"), false) {
        asked.activate = Some(CopySlot::Rotation);
    }
    ui.horizontal(|ui| {
        for (way, key) in [
            (Rotation::CounterClockwise, "copy_panel.counter_clockwise"),
            (Rotation::Clockwise, "copy_panel.clockwise"),
        ] {
            if ui
                .selectable_label(held.rotation == way, lang.t(key))
                .clicked()
            {
                asked.rotation = Some(way);
                asked.activate = Some(CopySlot::Rotation);
            }
        }
    });
}

/// « Annuler » and « Valider ↵ », at the right of the panel's foot.
fn footer(ui: &mut egui::Ui, lang: &Catalogue, asked: &mut Asked) {
    let right = egui::Layout::right_to_left(egui::Align::Center);
    ui.with_layout(right, |ui| {
        ui.spacing_mut().interact_size.y = BUTTON_HEIGHT;
        if primary_button(ui, &lang.t("copy_panel.validate_enter")).clicked() {
            asked.activate = Some(CopySlot::Validate);
            asked.enter = true;
        }
        asked.cancel = secondary_button(ui, true, &lang.t("copy_panel.cancel")).clicked();
    });
}
