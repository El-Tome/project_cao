//! The small X/Y/Z triad in a corner of the orientation cube's square: the
//! world's axes as the camera sees them, each ended by its letter.

use glam::Vec3;

use super::overlays::tint_to_color;
use crate::screens::viewport::ViewportState;

/// How far in from the square's top left corner the triad's origin stands, on
/// both sides, and how long an axis square to the view is drawn, each as a
/// share of the square's side. The corner is the one with the most room beside
/// the cube, and the first is no less than the second so that a line stays
/// inside the square whichever way it points.
const INSET: f32 = 0.16;
const LENGTH: f32 = 0.15;

const LINE_WIDTH: f32 = 1.5;

/// How far past the tip of its line a letter's middle stands, in font sizes.
const LETTER_GAP: f32 = 0.7;

/// Paints the triad with `painter`. A letter that would leave the square is
/// held inside it, the corner leaving little room at the smallest cube, and
/// dropped when it is larger than the square.
pub(super) fn paint(
    painter: &egui::Painter,
    state: &ViewportState,
    cube_rect: egui::Rect,
    font_size: f32,
) {
    let (camera, theme) = (&state.camera, &state.theme);
    let side = cube_rect.width();
    let origin = cube_rect.left_top() + egui::Vec2::splat(INSET * side);

    for (axis, name, colour) in [
        (Vec3::X, "X", theme.axis_x),
        (Vec3::Y, "Y", theme.axis_y),
        (Vec3::Z, "Z", theme.axis_z),
    ] {
        let ink = tint_to_color(colour);
        let tip =
            origin + egui::vec2(camera.right().dot(axis), -camera.up().dot(axis)) * LENGTH * side;
        painter.line_segment([origin, tip], egui::Stroke::new(LINE_WIDTH, ink));

        let letter =
            painter.layout_no_wrap(name.to_string(), egui::FontId::proportional(font_size), ink);
        let middle = tip + (tip - origin).normalized() * font_size * LETTER_GAP;
        let size = letter.size();
        if size.max_elem() > side {
            continue;
        }
        let at = (middle - size * 0.5).clamp(cube_rect.min, cube_rect.max - size);
        painter.galley(at, letter, ink);
    }
}
