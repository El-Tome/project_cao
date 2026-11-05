//! A panel standing over a view, always in front, dragged anywhere in it by
//! its title.

/// What the panel's contents answered, and where the panel was let go when its
/// title was dropped this frame.
pub struct Floated<R> {
    pub inner: R,
    pub let_go_at: Option<egui::Pos2>,
}

/// Shows the panel at the place it was last left this session, or else at
/// `left_at`, brought back inside `view` whenever it no longer fits there.
pub fn floating_panel<R>(
    ctx: &egui::Context,
    id: egui::Id,
    title: &str,
    (left_at, view): (egui::Pos2, egui::Rect),
    contents: impl FnOnce(&mut egui::Ui) -> R,
) -> Floated<R> {
    let held = ctx.data(|data| data.get_temp::<(egui::Pos2, egui::Vec2)>(id));
    let (at, size) = held.unwrap_or((left_at, egui::Vec2::ZERO));
    let at = kept_inside(at, size, view);
    let (mut moved, mut let_go, mut titled) = (egui::Vec2::ZERO, false, None);
    let shown = egui::Area::new(id)
        .order(egui::Order::Foreground)
        .fixed_pos(at)
        .show(ctx, |ui| {
            egui::Frame::popup(ui.style())
                .show(ui, |ui| {
                    // Rows never wrap: the panel takes the width of its widest
                    // one, frame after frame, as they change.
                    ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                    let title = egui::Label::new(egui::RichText::new(title).strong())
                        .sense(egui::Sense::drag());
                    let title = ui.add(title).on_hover_cursor(egui::CursorIcon::Grab);
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
