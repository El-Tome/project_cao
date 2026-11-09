use egui::{Align2, FontId, Response, Ui, Vec2, vec2};

use super::{ink, key_ink, pressable, undo_the_fade, written_key};
use crate::ui::icon::{Icon, IconSize};
use crate::ui::theme::tokens::TYPOGRAPHY_SIZES_RIBBON_BUTTON;

/// What three rows of 20 leave of the 64 a large button stands in.
const ROW_GAP: f32 = 2.0;

/// How many rows of small buttons a column of the ribbon holds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rows {
    Two,
    Three,
}

impl Rows {
    fn count(self) -> usize {
        match self {
            Self::Two => 2,
            Self::Three => 3,
        }
    }

    fn button_height(self) -> f32 {
        match self {
            Self::Two => 26.0,
            Self::Three => 20.0,
        }
    }
}

/// A small button of the ribbon: its icon left of its name, its key on the
/// right of the column it stands in.
pub fn small_button(
    ui: &mut Ui,
    enabled: bool,
    on: bool,
    icon: Icon,
    name: &str,
    key: Option<&str>,
    rows: Rows,
) -> Response {
    ui.add_enabled(enabled, move |ui: &mut Ui| {
        undo_the_fade(ui);
        let ink = ink(ui.is_enabled(), on);
        let font = FontId::proportional(TYPOGRAPHY_SIZES_RIBBON_BUTTON);
        let written_name = ui.painter().layout_no_wrap(name.to_owned(), font, ink);
        let written_key = written_key(ui, key);

        let side = IconSize::Small.points();
        let padding = ui.spacing().button_padding.x;
        let gap = ui.spacing().icon_spacing;
        let key_room = written_key.as_ref().map_or(0.0, |key| gap + key.size().x);
        let wanted = 2.0 * padding + side + gap + written_name.size().x + key_room;

        let size = vec2(width_in_its_column(ui, wanted), rows.button_height());
        let (rect, response) = pressable(ui, size, on, name, key);

        let inside = rect.shrink2(vec2(padding, 0.0));
        let icon_rect = Align2::LEFT_CENTER.align_size_within_rect(Vec2::splat(side), inside);
        icon.image(IconSize::Small)
            .tint(ink)
            .paint_at(ui, icon_rect);
        let name_at = Align2::LEFT_CENTER.align_size_within_rect(
            written_name.size(),
            inside.with_min_x(icon_rect.right() + gap),
        );
        ui.painter().galley(name_at.min, written_name, ink);
        if let Some(key) = written_key {
            let at = Align2::RIGHT_CENTER.align_size_within_rect(key.size(), inside);
            ui.painter().galley(at.min, key, key_ink(ui.is_enabled()));
        }

        response
    })
}

/// Lays `count` cells in columns of `rows`, top to bottom then the next column.
pub fn small_grid(ui: &mut Ui, rows: Rows, count: usize, mut cell: impl FnMut(&mut Ui, usize)) {
    ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.y = ROW_GAP;
        for first in (0..count).step_by(rows.count()) {
            ui.vertical(|ui| {
                start_the_column(ui);
                for rank in first..(first + rows.count()).min(count) {
                    cell(ui, rank);
                }
            });
        }
    });
}

/// A column's width as its buttons asked for it, kept apart from the width
/// they were given so that a column can narrow when a long name goes away.
#[derive(Clone, Copy, Default)]
struct ColumnWidth {
    last_frame: f32,
    widest_so_far: f32,
}

fn column_key(ui: &Ui) -> egui::Id {
    ui.id().with("small_column")
}

fn start_the_column(ui: &Ui) {
    let key = column_key(ui);
    ui.data_mut(|data| {
        let width = data.get_temp_mut_or_default::<ColumnWidth>(key);
        width.last_frame = width.widest_so_far;
        width.widest_so_far = 0.0;
    });
}

/// The width of the widest button of the column last frame, which is only
/// known once they have all been asked: the keys of a column end together.
fn width_in_its_column(ui: &Ui, wanted: f32) -> f32 {
    let key = column_key(ui);
    ui.data_mut(|data| {
        let width = data.get_temp_mut_or_default::<ColumnWidth>(key);
        width.widest_so_far = width.widest_so_far.max(wanted);
        wanted.max(width.last_frame)
    })
}
