//! One row of a tree: C10 of `docs/redesign/components.md`. The eye in a
//! column of its own on the left, then the row itself indented by its depth —
//! a chevron, an icon, its name — and on the right its badge and its value.

use std::ops::RangeInclusive;

use egui::{
    Align, Color32, FontId, Id, Label, Layout, Pos2, Rect, Response, RichText, Sense, Stroke, Ui,
    UiBuilder, WidgetInfo, WidgetType, pos2, vec2,
};

use crate::ui::badge::{badge, badge_width};
use crate::ui::eye::{eye, visibility};
use crate::ui::icon::{Icon, IconSize};
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
    /// What a badge says, and its patch and ink.
    pub badge: Option<(&'a str, (Color32, Color32))>,
    /// The button in the eye's column, when the row has one.
    pub eye: Option<Eye<'a>>,
    /// A picture before the name, in its own colour.
    pub icon: Option<(Icon, Color32)>,
    /// A click on it asks nothing: it reads as text, not as a button.
    pub inert: bool,
}

/// What stands in a row's eye column: C11, in one of its two forms.
#[derive(Clone, Copy)]
pub enum Eye<'a> {
    /// Shown or not: whether the eye is open, and what a click on it does.
    Open(bool, &'a str),
    /// Shown more ways than two: the picture of its state, and what a click
    /// on it does. Lit unless the row is dimmed.
    Pictured(Icon, &'a str),
}

/// The row as the hand found it, and whether the button in its eye's column
/// was clicked.
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

    let named = inside
        .painter()
        .layout_no_wrap(row.name.to_string(), name_font(), TEXT_PRIMARY)
        .size()
        .x;
    let mut kept = start + CHEVRON_BOX + pictured(&row) + named + SPACING_M;
    let badge_width = row
        .badge
        .map(|(said, _)| badge_width(&inside, said) + SPACING_M);
    kept += badge_width.unwrap_or(0.0);
    let mut end = line.right() - SPACING_M;
    if let Some(value) = row.value {
        end = beside(&mut inside, line, kept..=end, value);
    }
    if let Some((said, colours)) = row.badge {
        let room = Rect::from_x_y_ranges(end - badge_width.unwrap_or(0.0)..=end, line.y_range());
        inside.scope_builder(
            UiBuilder::new()
                .max_rect(room)
                .layout(Layout::right_to_left(Align::Center)),
            |ui| badge(ui, said, colours),
        );
        end = room.left();
    }

    let area = Rect::from_x_y_ranges(start..=end.max(start), line.y_range());
    let sense = if row.inert {
        Sense::hover()
    } else {
        Sense::click()
    };
    let response = ui.interact(area, spot.id.with("row"), sense);
    let name = row.name.to_string();
    response.widget_info(|| match row.inert {
        true => WidgetInfo::labeled(WidgetType::Label, ui.is_enabled(), &name),
        false => WidgetInfo::selected(WidgetType::Button, ui.is_enabled(), row.selected, &name),
    });
    paint(&inside, &row, area, &response);

    let mut eye_clicked = false;
    if let Some(shown) = row.eye {
        let column = Rect::from_x_y_ranges(line.left()..=line.left() + EYE_COLUMN, line.y_range());
        let in_the_column = UiBuilder::new()
            .max_rect(column)
            .layout(Layout::left_to_right(Align::Center));
        inside.scope_builder(in_the_column, |ui| {
            eye_clicked = match shown {
                Eye::Open(lit, said) => eye(ui, lit, said.to_string()),
                Eye::Pictured(drawn, said) => visibility(ui, !row.dimmed, drawn, said.to_string()),
            };
        });
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

/// Lays a value against the right of `room`, cut short rather than past its
/// left, and says where the room before it ends.
fn beside(ui: &mut Ui, line: Rect, room: RangeInclusive<f32>, value: &str) -> f32 {
    let text = RichText::new(value)
        .monospace()
        .size(TYPOGRAPHY_SIZES_MONO_VALUE)
        .color(TEXT_QUIET);
    let width = ui
        .painter()
        .layout_no_wrap(
            value.to_string(),
            FontId::monospace(TYPOGRAPHY_SIZES_MONO_VALUE),
            TEXT_QUIET,
        )
        .size()
        .x;
    let left = (room.end() - width).max(*room.start()).min(*room.end());
    let laid = Rect::from_x_y_ranges(left..=*room.end(), line.y_range());
    ui.scope_builder(
        UiBuilder::new()
            .max_rect(laid)
            .layout(Layout::right_to_left(Align::Center)),
        |ui| ui.add(Label::new(text).truncate().selectable(false)),
    );
    left - SPACING_M
}

fn paint(ui: &Ui, row: &TreeRow<'_>, area: Rect, response: &Response) {
    let painter = ui.painter().with_clip_rect(area);
    let fill = match (row.selected, response.hovered() && !row.inert) {
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
    if let Some((drawn, color)) = row.icon {
        let side = IconSize::Small.points();
        let at = Rect::from_min_size(
            pos2(area.left() + CHEVRON_BOX, middle - side / 2.0),
            vec2(side, side),
        );
        drawn.image(IconSize::Small).tint(color).paint_at(ui, at);
    }
    painter.text(
        pos2(area.left() + CHEVRON_BOX + pictured(row), middle),
        egui::Align2::LEFT_CENTER,
        row.name,
        name_font(),
        ink,
    );
}

/// How far the name stands past the chevron to leave room for the icon.
fn pictured(row: &TreeRow<'_>) -> f32 {
    row.icon
        .map_or(0.0, |_| IconSize::Small.points() + SPACING_S)
}

fn name_font() -> FontId {
    FontId::proportional(TYPOGRAPHY_SIZES_BUTTON)
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
