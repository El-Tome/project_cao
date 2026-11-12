//! A panel standing over a view, always in front, dragged anywhere in it by
//! its title.

use crate::ui::theme::tokens::{TEXT_QUIET, TYPOGRAPHY_SIZES_DETAIL};

/// How wide C13 of `docs/redesign/components.md` draws a tool's panel.
const TOOL_PANEL_WIDTH: f32 = 300.0;

/// The size C13 writes a tool panel's title at, which the tokens do not carry.
const TOOL_TITLE_SIZE: f32 = 14.0;

/// What the panel's contents answered, where the panel was let go when its
/// title was dropped this frame, and whether the control beside the title
/// closed it.
pub struct Floated<R> {
    pub inner: R,
    pub let_go_at: Option<egui::Pos2>,
    pub closed: bool,
}

/// Shows the panel at the place it was last left this session, or else at
/// `left_at`, brought back inside `view` whenever it no longer fits there. The
/// title carries a control reading `close`.
pub fn floating_panel<R>(
    ctx: &egui::Context,
    id: egui::Id,
    (title, close): (&str, &str),
    (left_at, view): (egui::Pos2, egui::Rect),
    contents: impl FnOnce(&mut egui::Ui) -> R,
) -> Floated<R> {
    let heading = |ui: &mut egui::Ui| {
        ui.horizontal(|ui| {
            let title = dragged_by(ui, egui::RichText::new(title).strong());
            (title, ui.small_button(close).clicked())
        })
        .inner
    };
    floated(ctx, id, (left_at, view), heading, contents)
}

/// The same panel as C13 draws a tool's: `TOOL_PANEL_WIDTH` across, its title
/// larger, and what the tool works on — `context` — written at the right of
/// it.
pub fn tool_panel<R>(
    ctx: &egui::Context,
    id: egui::Id,
    (title, context, close): (&str, &str, &str),
    (left_at, view): (egui::Pos2, egui::Rect),
    contents: impl FnOnce(&mut egui::Ui) -> R,
) -> Floated<R> {
    let heading = |ui: &mut egui::Ui| {
        let margins = ui.style().spacing.menu_margin.sum().x;
        ui.set_width(TOOL_PANEL_WIDTH - margins);
        ui.horizontal(|ui| {
            let title = egui::RichText::new(title).size(TOOL_TITLE_SIZE).strong();
            let title = dragged_by(ui, title);
            let right = egui::Layout::right_to_left(egui::Align::Center);
            let closed = ui.with_layout(right, |ui| {
                let closed = ui.small_button(close).clicked();
                let context = egui::RichText::new(context)
                    .size(TYPOGRAPHY_SIZES_DETAIL)
                    .color(TEXT_QUIET);
                ui.label(context);
                closed
            });
            (title, closed.inner)
        })
        .inner
    };
    floated(ctx, id, (left_at, view), heading, contents)
}

/// The title a panel is dragged by.
fn dragged_by(ui: &mut egui::Ui, title: egui::RichText) -> egui::Response {
    let title = egui::Label::new(title).sense(egui::Sense::drag());
    ui.add(title).on_hover_cursor(egui::CursorIcon::Grab)
}

/// The panel, under the title row `heading` draws: the title it is dragged by,
/// and whether the control closing it was clicked.
fn floated<R>(
    ctx: &egui::Context,
    id: egui::Id,
    (left_at, view): (egui::Pos2, egui::Rect),
    heading: impl FnOnce(&mut egui::Ui) -> (egui::Response, bool),
    contents: impl FnOnce(&mut egui::Ui) -> R,
) -> Floated<R> {
    let held = ctx.data(|data| data.get_temp::<(egui::Pos2, egui::Vec2)>(id));
    let (at, size) = held.unwrap_or((left_at, egui::Vec2::ZERO));
    let at = kept_inside(at, size, view);
    let (mut moved, mut let_go, mut titled) = (egui::Vec2::ZERO, false, None);
    let mut closed = false;
    let shown = egui::Area::new(id)
        .order(egui::Order::Foreground)
        .fixed_pos(at)
        .show(ctx, |ui| {
            egui::Frame::popup(ui.style())
                .show(ui, |ui| {
                    // Rows never wrap: the panel takes the width of its widest
                    // one, frame after frame, as they change.
                    ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                    let (title, close) = heading(ui);
                    closed = close;
                    moved = title.drag_delta();
                    let_go = title.drag_stopped();
                    titled = Some(title.id);
                    contents(ui)
                })
                .inner
        });
    let size = shown.response.rect.size();
    if let Some(title) = titled {
        shown.response.labelled_by(title);
    }
    let now = at + moved;
    ctx.data_mut(|data| data.insert_temp(id, (now, size)));
    Floated {
        inner: shown.inner,
        let_go_at: let_go.then_some(now),
        closed,
    }
}

/// One line of a panel, lit while it is the active one and warned while it is
/// marked. Clicking it is the user's way of making it active.
pub fn panel_row(ui: &mut egui::Ui, said: String, active: bool, marked: bool) -> egui::Response {
    let mut text = egui::RichText::new(said);
    if marked {
        text = text.color(ui.visuals().warn_fg_color);
    }
    ui.selectable_label(active, text)
}

/// Where a panel of this size stands so as to be inside the view: the place it
/// was left, moved in as little as it takes. A panel larger than the view keeps
/// its top-left corner, and so its title, in reach.
pub fn kept_inside(at: egui::Pos2, size: egui::Vec2, view: egui::Rect) -> egui::Pos2 {
    let far = (view.max - size).max(view.min);
    at.clamp(view.min, far)
}

#[cfg(test)]
mod tests;
