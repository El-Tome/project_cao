//! One row of a tree: C10 of `docs/redesign/components.md`. The eye in a
//! column of its own on the left, then the row itself indented by its depth —
//! a chevron, its name — and on the right its badge and its value.

use egui::{
    Align, Color32, FontId, Id, Label, Layout, Pos2, Rect, Response, RichText, Sense, Stroke, Ui,
    UiBuilder, WidgetInfo, WidgetType, pos2, vec2,
};

use crate::ui::badge::{Level, badge, badge_width};
use crate::ui::eye::eye;
use crate::ui::theme::tokens::{
    ACCENT_AMBER_BACKGROUND, ACCENT_AMBER_TEXT, BACKGROUND_HOVER, BORDER_GROUP, RADIUS_BUTTON,
    SPACING_M, SPACING_S, TEXT_PRIMARY, TEXT_QUIET, TYPOGRAPHY_SIZES_BUTTON,
    TYPOGRAPHY_SIZES_MONO_VALUE,
};

/// C10's measures in points, which the tokens do not carry: a row's height,
/// how far each depth stands in, the chevron's side and the box it sits in.
pub const HEIGHT: f32 = 28.0;
pub const INDENT: f32 = 26.0;
const CHEVRON: f32 = 10.0;
const CHEVRON_BOX: f32 = 18.0;
/// Room for C11's eye, 22 points wide, on every row whether it has one or not,
/// so that every row of a depth starts on one edge.
const EYE_COLUMN: f32 = 22.0 + SPACING_S;

/// What a row shows. A row folds when it has `unfolded`, and is drawn open or
/// shut by it.
#[derive(Clone, Copy, Default)]
pub struct TreeRow<'a> {
    pub name: &'a str,
    pub depth: usize,
    pub unfolded: Option<bool>,
    pub selected: bool,
    /// What it names is hidden.
    pub dimmed: bool,
    /// The colour the name is written in, other than the plain text's.
    pub ink: Option<Color32>,
    pub value: Option<&'a str>,
    pub badge: Option<(&'a str, Level)>,
    /// Whether the eye is open, and what a click on it does.
    pub eye: Option<(bool, &'a str)>,
}

/// The row as the hand found it, and whether its eye was clicked.
pub struct Touched {
    pub row: Response,
    pub eye: bool,
}

pub fn tree_row(ui: &mut Ui, row: TreeRow<'_>) -> Touched {
    let (line, spot) = ui.allocate_exact_size(vec2(ui.available_width(), HEIGHT), Sense::hover());
    let mut inside = ui.new_child(UiBuilder::new().id_salt(spot.id).max_rect(line));
    let start = line.left() + EYE_COLUMN + row.depth as f32 * INDENT;
    for depth in 0..row.depth {
        let x = line.left() + EYE_COLUMN + depth as f32 * INDENT + CHEVRON_BOX / 2.0;
        inside
            .painter()
            .vline(x, line.y_range(), Stroke::new(1.0, BORDER_GROUP));
    }

    let mut end = line.right() - SPACING_M;
    if let Some(value) = row.value {
        let text = RichText::new(value)
            .monospace()
            .size(TYPOGRAPHY_SIZES_MONO_VALUE)
            .color(TEXT_QUIET);
        end = beside(&mut inside, line, end, text);
    }
    if let Some((said, level)) = row.badge {
        let width = badge_width(&inside, said);
        let room = Rect::from_x_y_ranges(end - width..=end, line.y_range());
        inside.scope_builder(
            UiBuilder::new()
                .max_rect(room)
                .layout(Layout::right_to_left(Align::Center)),
            |ui| badge(ui, said, level),
        );
        end = room.left() - SPACING_M;
    }

    let area = Rect::from_x_y_ranges(start..=end.max(start), line.y_range());
    let response = ui.interact(area, spot.id.with("row"), Sense::click());
    let name = row.name.to_string();
    response.widget_info(|| {
        WidgetInfo::selected(WidgetType::Button, ui.is_enabled(), row.selected, &name)
    });
    paint(&inside, &row, area, &response);

    let mut eye_clicked = false;
    if let Some((lit, said)) = row.eye {
        let column = Rect::from_x_y_ranges(line.left()..=line.left() + EYE_COLUMN, line.y_range());
        inside.scope_builder(
            UiBuilder::new()
                .max_rect(column)
                .layout(Layout::left_to_right(Align::Center)),
            |ui| eye_clicked = eye(ui, lit, said.to_string()),
        );
    }
    Touched {
        row: response,
        eye: eye_clicked,
    }
}

/// A quiet line at a row's depth, standing for no element.
pub fn tree_note(ui: &mut Ui, depth: usize, said: &str) {
    ui.horizontal(|ui| {
        ui.add_space(EYE_COLUMN + depth as f32 * INDENT + CHEVRON_BOX);
        ui.weak(said);
    });
}

/// Lays `text` against `end`, on the right of the line, and says where it
/// starts.
fn beside(ui: &mut Ui, line: Rect, end: f32, text: RichText) -> f32 {
    let width = ui
        .painter()
        .layout_no_wrap(
            text.text().to_string(),
            FontId::monospace(TYPOGRAPHY_SIZES_MONO_VALUE),
            TEXT_QUIET,
        )
        .size()
        .x;
    let room = Rect::from_x_y_ranges(end - width..=end, line.y_range());
    ui.put(room, Label::new(text));
    room.left() - SPACING_M
}

fn paint(ui: &Ui, row: &TreeRow<'_>, area: Rect, response: &Response) {
    let painter = ui.painter().with_clip_rect(area);
    let fill = match (row.selected, response.hovered()) {
        (true, _) => Some(ACCENT_AMBER_BACKGROUND),
        (false, true) => Some(BACKGROUND_HOVER),
        (false, false) => None,
    };
    if let Some(fill) = fill {
        painter.rect_filled(area, RADIUS_BUTTON, fill);
    }
    let ink = match (row.selected, row.dimmed) {
        (true, _) => ACCENT_AMBER_TEXT,
        (false, true) => TEXT_QUIET,
        (false, false) => row.ink.unwrap_or(TEXT_PRIMARY),
    };
    let middle = area.center().y;
    if let Some(open) = row.unfolded {
        chevron(
            &painter,
            pos2(area.left() + CHEVRON_BOX / 2.0, middle),
            open,
        );
    }
    painter.text(
        pos2(area.left() + CHEVRON_BOX, middle),
        egui::Align2::LEFT_CENTER,
        row.name,
        FontId::proportional(TYPOGRAPHY_SIZES_BUTTON),
        ink,
    );
}

fn chevron(painter: &egui::Painter, at: Pos2, open: bool) {
    let half = CHEVRON / 2.0;
    let quarter = CHEVRON / 4.0;
    let points = match open {
        true => [
            at + vec2(-half, -quarter),
            at + vec2(0.0, quarter),
            at + vec2(half, -quarter),
        ],
        false => [
            at + vec2(-quarter, -half),
            at + vec2(quarter, 0.0),
            at + vec2(-quarter, half),
        ],
    };
    painter.line(points.to_vec(), Stroke::new(1.5, TEXT_QUIET));
}

/// A row that folds, and whether it stands open: kept between frames under
/// `id`, turned by a click, and held open while `held`.
pub fn folding_row(ui: &mut Ui, id: Id, held: bool, row: TreeRow<'_>) -> (Touched, bool) {
    let mut open = ui.data(|data| data.get_temp::<bool>(id)).unwrap_or(false);
    let touched = tree_row(
        ui,
        TreeRow {
            unfolded: Some(held || open),
            ..row
        },
    );
    if touched.row.clicked() && !held {
        open = !open;
        ui.data_mut(|data| data.insert_temp(id, open));
    }
    (touched, held || open)
}
