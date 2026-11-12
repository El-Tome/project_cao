//! A chip: one chosen element, or the invitation to click one in the view.

use std::f32::consts::{FRAC_PI_2, PI};

use egui::{
    Align2, Color32, FontId, Pos2, Rect, Response, Sense, Shape, Stroke, StrokeKind, Ui,
    WidgetInfo, WidgetType, pos2, vec2,
};

use crate::ui::theme::tokens::{
    ACCENT_AMBER_BACKGROUND, ACCENT_AMBER_BORDER, ACCENT_AMBER_TEXT, BORDER_STRONG, RADIUS_PILL,
    TEXT_SUBDUED, TYPOGRAPHY_SIZES_DETAIL,
};

/// How tall a chip stands: C14's 24 px.
const HEIGHT: f32 = 24.0;

/// What a chip keeps clear on either side of its label, as
/// `docs/redesign/source/part.html` draws it.
const PADDING: f32 = 8.0;

/// The dashes of the invitation's border, and the gaps between them.
const DASH: f32 = 3.0;
const GAP: f32 = 3.0;

/// How many straight pieces each round end of the dashed border is drawn in.
const ARC_PIECES: usize = 12;

/// What a chip stands for, as C14 draws it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChipKind {
    /// An element chosen.
    Selection,
    /// The place of the next element, still to be clicked in the view.
    Invitation,
}

/// How a chip is painted.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChipLook {
    pub fill: Color32,
    pub ink: Color32,
    pub border: Color32,
    pub dashed: bool,
}

impl ChipKind {
    pub fn look(self) -> ChipLook {
        match self {
            Self::Selection => ChipLook {
                fill: ACCENT_AMBER_BACKGROUND,
                ink: ACCENT_AMBER_TEXT,
                border: ACCENT_AMBER_BORDER,
                dashed: false,
            },
            Self::Invitation => ChipLook {
                fill: Color32::TRANSPARENT,
                ink: TEXT_SUBDUED,
                border: BORDER_STRONG,
                dashed: true,
            },
        }
    }
}

/// `label` on a pill in the look of its kind. A click on it is handed back.
pub fn chip(ui: &mut Ui, label: &str, kind: ChipKind) -> Response {
    let look = kind.look();
    let font = FontId::proportional(TYPOGRAPHY_SIZES_DETAIL);
    let written = ui
        .painter()
        .layout_no_wrap(label.to_owned(), font, look.ink);
    let size = vec2(written.size().x + 2.0 * PADDING, HEIGHT);
    let (rect, response) = ui.allocate_exact_size(size, Sense::click());
    response.widget_info(|| WidgetInfo::labeled(WidgetType::Button, ui.is_enabled(), label));

    let painter = ui.painter();
    let border = Stroke::new(1.0, look.border);
    let solid = match look.dashed {
        true => Stroke::NONE,
        false => border,
    };
    painter.rect(rect, RADIUS_PILL, look.fill, solid, StrokeKind::Inside);
    if look.dashed {
        painter.extend(Shape::dashed_line(
            &pill_outline(rect.shrink(border.width / 2.0)),
            border,
            DASH,
            GAP,
        ));
    }
    let at = Align2::CENTER_CENTER.align_size_within_rect(written.size(), rect);
    painter.galley(at.min, written, look.ink);
    response
}

/// The outline of a pill standing in `rect`, round at both ends, as points
/// a dashed line can follow all the way round.
fn pill_outline(rect: Rect) -> Vec<Pos2> {
    let radius = rect.height() / 2.0;
    let ends = [
        (rect.right() - radius, -FRAC_PI_2),
        (rect.left() + radius, FRAC_PI_2),
    ];
    let mut outline: Vec<Pos2> = ends
        .into_iter()
        .flat_map(|(x, from)| {
            (0..=ARC_PIECES).map(move |piece| {
                let angle = from + PI * piece as f32 / ARC_PIECES as f32;
                pos2(
                    x + radius * angle.cos(),
                    rect.center().y + radius * angle.sin(),
                )
            })
        })
        .collect();
    if let Some(&start) = outline.first() {
        outline.push(start);
    }
    outline
}

#[cfg(test)]
mod tests;
