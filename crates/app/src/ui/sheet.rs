//! A sheet hung under the button that opens it, in front of everything else:
//! the button opens and shuts it, and its cross, Escape, or a click away from
//! it shut it.

use egui::{
    Align, Area, Button, Context, Frame, Id, Key, Layout, Margin, Modifiers, Order, Response,
    RichText, Ui, pos2,
};

use crate::ui::formula_field::keeps_escape;
use crate::ui::icon::{self, IconSize};
use crate::ui::theme::tokens::{RADIUS_CARD, SPACING_L, SPACING_S, TEXT_TITLE};

/// Draws the sheet `width` wide while `open` says so, its right edge on the
/// button's and its top just under it, headed by `heading` with a cross named
/// `close` on its right. An Escape that shuts it is taken from whatever is
/// drawn after it. Nothing comes back while it is shut.
pub fn sheet_under<R>(
    ctx: &Context,
    id: Id,
    (button, open): (&Response, &mut bool),
    (width, heading, close): (f32, &str, &str),
    contents: impl FnOnce(&mut Ui) -> R,
) -> Option<R> {
    let was_open = *open;
    if button.clicked() {
        *open = !*open;
    }
    let escaped = *open
        && !keeps_escape(ctx)
        && ctx.input_mut(|input| input.consume_key(Modifiers::NONE, Key::Escape));
    if !*open || escaped {
        *open = false;
        return None;
    }
    let mut crossed = false;
    let shown = Area::new(id)
        .order(Order::Foreground)
        .fixed_pos(pos2(
            button.rect.right() - width,
            button.rect.bottom() + SPACING_S,
        ))
        .show(ctx, |ui| {
            let frame = Frame::popup(ui.style())
                .corner_radius(RADIUS_CARD)
                .inner_margin(Margin::same(SPACING_L as i8));
            let inside = width - frame.total_margin().sum().x;
            frame
                .show(ui, |ui| {
                    ui.set_width(inside);
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(heading).color(TEXT_TITLE));
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            let cross = icon::CLOSE.image(IconSize::Row).alt_text(close);
                            let cross = Button::new(cross)
                                .frame_when_inactive(false)
                                .image_tint_follows_text_color(true);
                            crossed = ui.add(cross).clicked();
                        });
                    });
                    contents(ui)
                })
                .inner
        });
    // The click that opened it lands away from it, and must not shut it again.
    if crossed || (was_open && shown.response.clicked_elsewhere()) {
        *open = false;
    }
    Some(shown.inner)
}
