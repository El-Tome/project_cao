use egui::text::{LayoutJob, TextFormat};
use egui::{
    Align2, Area, AreaState, Color32, Context, FontId, Frame, Id, Label, Margin, Order, Rect,
    Response, RichText, Sense, Stroke, StrokeKind, TextStyle, TextWrapMode, Ui, WidgetText, pos2,
    vec2,
};

use crate::ui::theme::tokens::{
    ACCENT_AMBER, BACKGROUND_FLOATING, BACKGROUND_HOVER, BORDER_CONTROL, BORDER_GROUP,
    RADIUS_BUTTON, RADIUS_PILL, SPACING_L, SPACING_M, SPACING_S, SPACING_XXL, STATE_ERROR,
    STATE_ERROR_BACKGROUND, STATE_ERROR_TEXT, TEXT_BUTTON, TEXT_TITLE, TYPOGRAPHY_SIZES_BUTTON,
    TYPOGRAPHY_SIZES_DETAIL,
};

/// How tall a message stands, its dot how wide, and how far the dot stands
/// from the text, in points: `docs/redesign/source/part.html`, which the
/// tokens do not carry.
pub const MESSAGE_HEIGHT: f32 = 28.0;
const DOT: f32 = 7.0;
const DOT_GAP: f32 = 9.0;

/// A short run of text on a rounded patch of its own, set apart from what
/// stands beside it on a bar.
pub fn pill(ui: &mut Ui, text: &str) -> Response {
    Frame::new()
        .fill(BACKGROUND_HOVER)
        .corner_radius(RADIUS_BUTTON)
        .inner_margin(Margin::symmetric(SPACING_L as i8, SPACING_S as i8))
        .show(ui, |ui| {
            ui.label(
                RichText::new(text)
                    .color(TEXT_BUTTON)
                    .size(TYPOGRAPHY_SIZES_DETAIL),
            )
        })
        .inner
}

/// A pill standing over a view, its middle on the view's and its top a little
/// below the view's, holding a row laid left to right. It sits on a layer of
/// its own, above the view: what lies under it is neither hovered nor clicked.
pub fn floating_pill<R>(
    ctx: &Context,
    id: Id,
    view: Rect,
    contents: impl FnOnce(&mut Ui) -> R,
) -> R {
    let size_before = AreaState::load(ctx, id).and_then(|state| state.size);
    let shown = Area::new(id)
        .order(Order::Middle)
        .pivot(Align2::CENTER_TOP)
        .fixed_pos(pos2(view.center().x, view.top() + SPACING_XXL))
        .constrain_to(view)
        .show(ctx, |ui| {
            Frame::popup(ui.style())
                .stroke(Stroke::new(1.0, BORDER_CONTROL))
                .corner_radius(RADIUS_PILL)
                .inner_margin(Margin {
                    left: SPACING_XXL as i8,
                    right: SPACING_M as i8,
                    top: SPACING_S as i8,
                    bottom: SPACING_S as i8,
                })
                .show(ui, |ui| {
                    ui.style_mut().wrap_mode = Some(TextWrapMode::Extend);
                    ui.spacing_mut().item_spacing.x = SPACING_XXL;
                    ui.horizontal(contents).inner
                })
                .inner
        });
    // The pill is placed by the size it had the frame before: one that grew or
    // shrank stands off the middle until the next frame lays it again.
    if size_before != Some(shown.response.rect.size()) {
        ctx.request_repaint();
    }
    shown.inner
}

/// A name in the strongest ink, then what it stands on after a middle dot, as
/// a single text.
pub fn name_and_detail(ui: &mut Ui, name: &str, detail: &str) -> Response {
    let font = FontId::proportional(TYPOGRAPHY_SIZES_BUTTON);
    let mut job = LayoutJob::default();
    job.append(name, 0.0, TextFormat::simple(font.clone(), TEXT_TITLE));
    job.append(
        &format!(" · {detail}"),
        0.0,
        TextFormat::simple(font, TEXT_BUTTON),
    );
    ui.label(job)
}

/// What a message is said as, which colours its dot, its patch and its text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tone {
    /// What a gesture answers: a hint, a refusal, a confirmation.
    Said,
    /// What went wrong reaching the disk.
    Failed,
}

impl Tone {
    fn colours(self) -> (Color32, Color32, Color32, Color32) {
        match self {
            Self::Said => (ACCENT_AMBER, BACKGROUND_FLOATING, BORDER_GROUP, TEXT_BUTTON),
            Self::Failed => (
                STATE_ERROR,
                STATE_ERROR_BACKGROUND,
                STATE_ERROR,
                STATE_ERROR_TEXT,
            ),
        }
    }
}

/// A message on a patch with round ends, a dot before it, its middle at
/// `middle` on the row being laid — or pushed right of what the row already
/// holds, never over it, and cut short with the whole of it on hover when the
/// row has no room left for it. The text stays a label reading all of it, for
/// whoever reads the screen without seeing it.
pub fn message_pill(ui: &mut Ui, middle: f32, text: &str, tone: Tone) -> Response {
    let (dot, fill, border, ink) = tone.colours();
    let row = ui.max_rect();
    let from = ui.cursor().left();
    let around = SPACING_XXL + DOT + DOT_GAP + SPACING_XXL;
    let said = RichText::new(text).size(TYPOGRAPHY_SIZES_BUTTON).color(ink);
    let galley = WidgetText::from(said).into_galley(
        ui,
        Some(TextWrapMode::Truncate),
        (row.right() - from - around).max(0.0),
        TextStyle::Body,
    );
    let size = vec2(around + galley.size().x, MESSAGE_HEIGHT);
    let left = (middle - size.x / 2.0).min(row.right() - size.x).max(from);
    let patch = Rect::from_min_size(pos2(left, row.center().y - size.y / 2.0), size);

    let response = ui.allocate_rect(patch, Sense::hover());
    let painter = ui.painter();
    painter.rect(
        patch,
        RADIUS_PILL,
        fill,
        Stroke::new(1.0, border),
        StrokeKind::Inside,
    );
    let dot_at = pos2(patch.left() + SPACING_XXL + DOT / 2.0, patch.center().y);
    painter.circle_filled(dot_at, DOT / 2.0, dot);
    let text_at = Align2::LEFT_CENTER.align_size_within_rect(
        galley.size(),
        Rect::from_min_max(pos2(dot_at.x + DOT / 2.0 + DOT_GAP, patch.top()), patch.max),
    );
    ui.put(text_at, Label::new(galley));
    response
}
