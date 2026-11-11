//! One box of the step reader, and the small arrow between two: the icon of a
//! step on a small square, solid while the step is played and dashed once it
//! is not, in its alert's colours, with a dot in its corner that stays seen on
//! a dashed box. A box answers to a name of its own for whoever cannot see it,
//! and shows its step's name on hover.

use egui::{
    Align2, Color32, Painter, Rect, Response, Sense, Shape, Stroke, StrokeKind, Ui, Vec2,
    WidgetInfo, WidgetType, vec2,
};

use crate::ui::icon::{Icon, IconSize};
use crate::ui::theme::tokens::{
    ACCENT_AMBER, BACKGROUND_FIELD, BACKGROUND_HOVER, BORDER_STRONG, RADIUS_BUTTON, TEXT_DISABLED,
    TEXT_PRIMARY, TEXT_QUIET,
};

/// The measures in points the tokens do not carry: P7's box, the arrow's
/// room, the alert's dot across, and the dashes of a box not played.
pub const BOX: Vec2 = vec2(22.0, 24.0);
pub const ARROW: f32 = 9.0;
pub const DOT: f32 = 7.0;
const DASH: f32 = 3.0;
const DASH_GAP: f32 = 2.0;

/// What a box shows. The box read to stands out; `alert` is the patch and the
/// ink of its step's alert, when it has one.
pub struct StepBox<'a> {
    pub icon: Icon,
    /// What it answers to for whoever cannot see it.
    pub called: &'a str,
    /// Its step's name, shown on hover.
    pub name: &'a str,
    pub applied: bool,
    pub current: bool,
    pub alert: Option<(Color32, Color32)>,
}

pub fn step_box(ui: &mut Ui, step: &StepBox<'_>) -> Response {
    let (rect, response) = ui.allocate_exact_size(BOX, Sense::click());
    response.widget_info(|| {
        WidgetInfo::selected(
            WidgetType::Button,
            ui.is_enabled(),
            step.current,
            step.called,
        )
    });
    let (patch, ink) = (
        step.alert.map(|(patch, _)| patch),
        step.alert.map(|(_, ink)| ink),
    );
    let painter = ui.painter();
    let fill = match (response.hovered(), step.applied) {
        (true, _) => BACKGROUND_HOVER,
        (false, true) => patch.unwrap_or(BACKGROUND_FIELD),
        (false, false) => Color32::TRANSPARENT,
    };
    painter.rect_filled(rect, RADIUS_BUTTON, fill);
    match (step.current, step.applied) {
        (true, _) => {
            let stroke = Stroke::new(1.5, ACCENT_AMBER);
            painter.rect_stroke(rect, RADIUS_BUTTON, stroke, StrokeKind::Inside);
        }
        (false, true) => {
            let stroke = Stroke::new(1.0, ink.unwrap_or(BORDER_STRONG));
            painter.rect_stroke(rect, RADIUS_BUTTON, stroke, StrokeKind::Inside);
        }
        (false, false) => dashed(painter, rect.shrink(0.5), BORDER_STRONG),
    }
    let drawn_in = match step.applied {
        true => ink.unwrap_or(TEXT_PRIMARY),
        false => TEXT_DISABLED,
    };
    let side = Vec2::splat(IconSize::Row.points());
    step.icon
        .image(IconSize::Row)
        .tint(drawn_in)
        .paint_at(ui, Align2::CENTER_CENTER.align_size_within_rect(side, rect));
    if let Some(ink) = ink {
        let corner = rect.right_top() + vec2(-1.0, 1.0);
        ui.painter().circle_filled(corner, DOT / 2.0, ink);
    }
    response.on_hover_text(step.name)
}

/// The arrow leading to the next box, quiet while that box is played and
/// darker once it is not.
pub fn step_arrow(ui: &mut Ui, applied: bool) {
    let (rect, _) = ui.allocate_exact_size(vec2(ARROW, BOX.y), Sense::hover());
    let ink = match applied {
        true => TEXT_QUIET,
        false => BORDER_STRONG,
    };
    let stroke = Stroke::new(1.2, ink);
    let (from, to) = (rect.left_center(), rect.right_center());
    let painter = ui.painter();
    painter.line_segment([from, to], stroke);
    painter.line_segment([to, to + vec2(-3.0, -3.0)], stroke);
    painter.line_segment([to, to + vec2(-3.0, 3.0)], stroke);
}

/// The outline of `rect` in dashes, square at its corners.
fn dashed(painter: &Painter, rect: Rect, ink: Color32) {
    let around = [
        rect.left_top(),
        rect.right_top(),
        rect.right_bottom(),
        rect.left_bottom(),
        rect.left_top(),
    ];
    painter.extend(Shape::dashed_line(
        &around,
        Stroke::new(1.0, ink),
        DASH,
        DASH_GAP,
    ));
}
