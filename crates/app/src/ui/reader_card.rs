//! The step reader's card, P7 of `docs/redesign/part-screen.md`: ◀ and ▶ at
//! the ends of a row of boxes, which scrolls sideways once it outgrows the
//! card, to the box read to whenever that one changes; under the row a line
//! and a counter, then the legend of the alerts' colours. Plain values in,
//! what was clicked out.

use egui::style::ScrollStyle;
use egui::text::TextWrapping;
use egui::{
    Align, Align2, Color32, Frame, Id, Label, Layout, Margin, Response, RichText, ScrollArea,
    Sense, Stroke, TextStyle, TextWrapMode, Ui, Vec2, WidgetInfo, WidgetText, WidgetType, vec2,
};

use crate::ui::icon::{self, Icon, IconSize};
use crate::ui::step_box::{ARROW, BOX, DOT, StepBox, step_arrow, step_box};
use crate::ui::theme::tokens::{
    BACKGROUND_CARD, BACKGROUND_HOVER, BORDER_GROUP, BORDER_STRONG, RADIUS_BUTTON, RADIUS_CARD,
    SPACING_L, SPACING_M, SPACING_S, TEXT_QUIET, TEXT_SUBDUED, TEXT_TITLE, TYPOGRAPHY_SIZES_BUTTON,
    TYPOGRAPHY_SIZES_DETAIL,
};

/// What stands between two things of the row, in points.
const GAP: f32 = 2.0;
/// The width of ◀ and ▶.
const END: f32 = 18.0;
/// How many lines the line under the row may take before it is cut short.
const LINE_ROWS: usize = 2;
/// The thickness of the row's scroll bar.
const BAR: f32 = 3.0;

/// What the card shows.
pub struct ReaderCard<'a> {
    pub boxes: &'a [StepBox<'a>],
    /// One between each two boxes: whether the box it leads to is played.
    pub arrows: &'a [bool],
    /// What ◀ and ▶ answer to.
    pub previous: &'a str,
    pub next: &'a str,
    /// The line under the row, the ink it is written in, and the counter on
    /// its right.
    pub line: (&'a str, Color32, &'a str),
    /// Each colour an alert gives a box, with its word.
    pub legend: &'a [(Color32, &'a str)],
}

/// What was clicked on the card.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Asked {
    Nothing,
    Box(usize),
    Previous,
    Next,
}

pub fn reader_card(ui: &mut Ui, card: &ReaderCard<'_>) -> Asked {
    Frame::new()
        .fill(BACKGROUND_CARD)
        .stroke(Stroke::new(1.0, BORDER_GROUP))
        .corner_radius(RADIUS_CARD)
        .inner_margin(Margin::same(SPACING_L as i8))
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing.y = SPACING_S;
            let asked = row(ui, card);
            line(ui, card.line);
            legend(ui, card.legend);
            asked
        })
        .inner
}

/// ◀, the boxes in the middle of what room is left, then ▶. The row is
/// brought to the box read to on the first frame and whenever that box
/// changes, and left where the hand scrolled it otherwise.
fn row(ui: &mut Ui, card: &ReaderCard<'_>) -> Asked {
    let mut asked = Asked::Nothing;
    let current = card.boxes.iter().position(|step| step.current);
    let remembered = Id::new("step_reader_row").with("current");
    let moved = ui.data(|data| data.get_temp::<Option<usize>>(remembered)) != Some(current);
    ui.data_mut(|data| data.insert_temp(remembered, current));
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = GAP;
        quiet_scroll_bar(ui);
        if end(ui, icon::STEP_PREVIOUS, card.previous).clicked() {
            asked = Asked::Previous;
        }
        let room = (ui.available_width() - END - GAP).max(0.0);
        let count = card.boxes.len() as f32;
        let wide = count * BOX.x + (count - 1.0).max(0.0) * (ARROW + 2.0 * GAP) + DOT / 2.0;
        ScrollArea::horizontal()
            .id_salt("step_reader_row")
            .max_width(room)
            .auto_shrink([false, true])
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.allocate_exact_size(vec2(0.0, BOX.y + DOT), Sense::hover());
                    ui.add_space(((room - wide) / 2.0 - GAP).max(0.0));
                    for (rank, step) in card.boxes.iter().enumerate() {
                        if rank > 0 {
                            step_arrow(ui, card.arrows.get(rank - 1).copied().unwrap_or(false));
                        }
                        let response = step_box(ui, step);
                        if moved && step.current {
                            response.scroll_to_me(None);
                        }
                        if response.clicked() {
                            asked = Asked::Box(rank);
                        }
                    }
                    ui.add_space(DOT / 2.0);
                });
            });
        if end(ui, icon::STEP_NEXT, card.next).clicked() {
            asked = Asked::Next;
        }
    });
    asked
}

/// The row's scroll bar, shown once the boxes outgrow it: thin and solid, in
/// a strip of its own under the boxes rather than floating over them, and in
/// the card's quiet colours rather than the text's.
fn quiet_scroll_bar(ui: &mut Ui) {
    let scroll = &mut ui.spacing_mut().scroll;
    *scroll = ScrollStyle {
        bar_width: BAR,
        bar_inner_margin: GAP,
        content_margin: scroll.content_margin,
        ..ScrollStyle::solid()
    };
    let visuals = ui.visuals_mut();
    visuals.extreme_bg_color = BACKGROUND_CARD;
    visuals.widgets.inactive.bg_fill = BORDER_STRONG;
    visuals.widgets.hovered.bg_fill = TEXT_QUIET;
    visuals.widgets.active.bg_fill = TEXT_QUIET;
}

/// ◀ or ▶: its icon alone, called `name`.
fn end(ui: &mut Ui, icon: Icon, name: &str) -> Response {
    let (rect, response) = ui.allocate_exact_size(vec2(END, BOX.y), Sense::click());
    response.widget_info(|| WidgetInfo::labeled(WidgetType::Button, ui.is_enabled(), name));
    let ink = match response.hovered() {
        true => {
            ui.painter()
                .rect_filled(rect, RADIUS_BUTTON, BACKGROUND_HOVER);
            TEXT_TITLE
        }
        false => TEXT_SUBDUED,
    };
    let side = Vec2::splat(IconSize::Row.points());
    icon.image(IconSize::Row)
        .tint(ink)
        .paint_at(ui, Align2::CENTER_CENTER.align_size_within_rect(side, rect));
    response.on_hover_text(name)
}

/// The line, wrapped on two lines at most and cut short past them, but read
/// whole by whoever reads the screen without seeing it; the counter on the
/// right of its first line.
fn line(ui: &mut Ui, (text, ink, counter): (&str, Color32, &str)) {
    ui.horizontal_top(|ui| {
        let counter = WidgetText::from(RichText::new(counter).color(TEXT_QUIET)).into_galley(
            ui,
            Some(TextWrapMode::Extend),
            f32::INFINITY,
            TextStyle::Monospace,
        );
        let room = (ui.available_width() - counter.size().x - ui.spacing().item_spacing.x).max(0.0);
        ui.allocate_ui_with_layout(vec2(room, 0.0), Layout::top_down(Align::Min), |ui| {
            ui.set_min_width(room);
            let wrapping = TextWrapping {
                max_rows: LINE_ROWS,
                ..TextWrapping::wrap_at_width(room)
            };
            let said = RichText::new(text).size(TYPOGRAPHY_SIZES_BUTTON).color(ink);
            let galley = WidgetText::from(said).into_galley_impl(
                ui.ctx(),
                ui.style(),
                wrapping,
                TextStyle::Body.into(),
                ui.text_valign(),
            );
            ui.add(Label::new(galley).selectable(false));
        });
        ui.add(Label::new(counter).selectable(false));
    });
}

/// Each colour's dot, its word beside it.
fn legend(ui: &mut Ui, legend: &[(Color32, &str)]) {
    ui.horizontal(|ui| {
        for (dot, word) in legend {
            let (rect, _) = ui.allocate_exact_size(Vec2::splat(DOT), Sense::hover());
            ui.painter().circle_filled(rect.center(), DOT / 2.0, *dot);
            let said = RichText::new(*word)
                .size(TYPOGRAPHY_SIZES_DETAIL)
                .color(TEXT_QUIET);
            ui.add(Label::new(said).selectable(false));
            ui.add_space(SPACING_M);
        }
    });
}
