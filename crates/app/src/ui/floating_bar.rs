//! A row of a view's own tools, standing over the view against its bottom
//! right corner: buttons, a menu, a separator and a reading.

use egui::{
    Align, Align2, Area, AreaState, Button, Context, FontId, Frame, Id, IntoAtoms, Label, Layout,
    Margin, Order, Popup, Pos2, Rect, Response, RichText, Sense, Stroke, TextStyle, TextWrapMode,
    Ui, WidgetText, vec2,
};

use crate::ui::icon::{Icon, IconSize};
use crate::ui::theme::tokens::{
    BORDER_CONTROL, SPACING_L, SPACING_M, SPACING_XXL, TEXT_QUIET, TEXT_SUBDUED,
    TYPOGRAPHY_SIZES_DETAIL,
};

/// The bar's measures in points, as `docs/redesign/source/part.html` draws
/// them and the tokens do not carry: its corners, its border, its padding and
/// the gap between what it holds; a button's height, its padding across and the gap
/// between its icon and its name; the size of a key; a separator's height.
const RADIUS: u8 = 8;
const STROKE: f32 = 1.0;
const PADDING: i8 = 3;
const GAP: f32 = 2.0;
const BUTTON_HEIGHT: f32 = 30.0;
const BUTTON_PADDING: f32 = 10.0;
const BUTTON_GAP: f32 = 7.0;
const KEY_SIZE: f32 = 10.0;
const SEPARATOR_HEIGHT: f32 = 18.0;

/// How far above a view's bottom edge the top of the bar stands: its margin,
/// its frame and its row. What is set in that corner stands above it.
pub const TOP_ABOVE_THE_EDGE: f32 = SPACING_XXL + 2.0 * FRAME_INSET + BUTTON_HEIGHT;

/// How far inside a row's frame what it holds starts: its border and padding.
pub const FRAME_INSET: f32 = STROKE + PADDING as f32;

/// What a menu button wears after its name to say that it opens.
const OPENS: &str = "⌄";

/// A bar standing over a view, against its bottom right corner, holding a row
/// laid left to right.
pub fn floating_bar<R>(
    ctx: &Context,
    id: Id,
    view: Rect,
    contents: impl FnOnce(&mut Ui) -> R,
) -> R {
    let corner = view.right_bottom() - vec2(SPACING_XXL, SPACING_XXL);
    floating_row(ctx, id, view, (Align2::RIGHT_BOTTOM, corner), contents)
}

/// A framed row standing over a view, the point `pivot` names set at `at` and
/// the whole kept inside the view. It sits on a layer of its own, above the
/// view: what lies under it is neither hovered nor clicked, save by a drag
/// already under way, which carries on over it.
pub fn floating_row<R>(
    ctx: &Context,
    id: Id,
    view: Rect,
    (pivot, at): (Align2, Pos2),
    contents: impl FnOnce(&mut Ui) -> R,
) -> R {
    let size_before = AreaState::load(ctx, id).and_then(|state| state.size);
    let shown = Area::new(id)
        .order(Order::Middle)
        .interactable(ctx.dragged_id().is_none())
        .pivot(pivot)
        .fixed_pos(at)
        .constrain_to(view)
        .show(ctx, |ui| {
            Frame::popup(ui.style())
                .stroke(Stroke::new(STROKE, BORDER_CONTROL))
                .corner_radius(RADIUS)
                .inner_margin(Margin::same(PADDING))
                .show(ui, |ui| {
                    ui.style_mut().wrap_mode = Some(TextWrapMode::Extend);
                    ui.spacing_mut().item_spacing.x = GAP;
                    ui.spacing_mut().button_padding.x = BUTTON_PADDING;
                    let row = vec2(ui.available_width(), BUTTON_HEIGHT);
                    ui.allocate_ui_with_layout(row, Layout::left_to_right(Align::Center), contents)
                        .inner
                })
                .inner
        });
    // Placed by the size it had the frame before, as a pill is: one that grew
    // or shrank stands off the corner until the next frame lays it again.
    if size_before != Some(shown.response.rect.size()) {
        ctx.request_repaint();
    }
    shown.inner
}

/// A button of the bar: its icon, its name, and the key bound to it in a
/// quieter ink when there is one. Framed only under the pointer — or, when it
/// switches something, while that is on.
pub fn bar_button(
    ui: &mut Ui,
    enabled: bool,
    on: Option<bool>,
    icon: Icon,
    name: &str,
    key: Option<&str>,
) -> Response {
    let mut atoms = (icon.image(IconSize::Small), name).into_atoms();
    if let Some(key) = key {
        atoms.push_right(
            RichText::new(key)
                .font(FontId::monospace(KEY_SIZE))
                .color(TEXT_QUIET),
        );
    }
    let mut button = Button::new(atoms).frame_when_inactive(on == Some(true));
    if let Some(on) = on {
        button = button.selected(on);
    }
    ui.add_enabled(enabled, dressed(button))
}

/// A button of the bar that opens a menu of `entries`, the current one marked,
/// and says which was picked, by its rank.
pub fn bar_menu(
    ui: &mut Ui,
    enabled: bool,
    icon: Icon,
    name: &str,
    entries: &[(&str, bool)],
) -> Option<usize> {
    let button = Button::new((icon.image(IconSize::Small), name, OPENS)).frame_when_inactive(false);
    let response = ui.add_enabled(enabled, dressed(button));
    let mut picked = None;
    Popup::menu(&response).show(|ui| {
        for (rank, (entry, current)) in entries.iter().enumerate() {
            if ui.add(Button::selectable(*current, *entry)).clicked() {
                picked = Some(rank);
            }
        }
    });
    picked
}

/// A thin upright line between two runs of the bar.
pub fn bar_separator(ui: &mut Ui) {
    let size = vec2(1.0 + 2.0 * SPACING_M, SEPARATOR_HEIGHT);
    let (rect, _) = ui.allocate_exact_size(size, Sense::hover());
    ui.painter().vline(
        rect.center().x,
        rect.y_range(),
        Stroke::new(1.0, BORDER_CONTROL),
    );
}

/// What the bar reads out in figures, in the monospace family and a quieter
/// ink than its buttons, the last thing on the bar. It keeps the room
/// `room_for` takes, the text against its right end, so that what stands
/// before it does not move as the text changes length.
pub fn bar_reading(ui: &mut Ui, text: &str, room_for: &str) -> Response {
    let laid = |text: &str| {
        let read = RichText::new(text)
            .monospace()
            .size(TYPOGRAPHY_SIZES_DETAIL)
            .color(TEXT_SUBDUED);
        WidgetText::from(read).into_galley(
            ui,
            Some(TextWrapMode::Extend),
            f32::INFINITY,
            TextStyle::Monospace,
        )
    };
    let galley = laid(text);
    let room = laid(room_for).size().x.max(galley.size().x);
    let (slot, _) = ui.allocate_exact_size(vec2(room, galley.size().y), Sense::hover());
    let at = Align2::RIGHT_CENTER.align_size_within_rect(galley.size(), slot);
    let read = ui.put(at, Label::new(galley));
    ui.add_space(SPACING_L - GAP);
    read
}

fn dressed(button: Button<'_>) -> Button<'_> {
    button
        .image_tint_follows_text_color(true)
        .min_size(vec2(0.0, BUTTON_HEIGHT))
        .gap(BUTTON_GAP)
}
