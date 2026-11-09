//! The X/Y/Z triad at the bottom left of the orientation cube's square: the
//! world's axes as the camera sees them, each ended by its letter.

use glam::Vec3;

use super::overlays::tint_to_color;
use crate::screens::viewport::ViewportState;

/// How far the triad's drawing stands in from the square's bottom left corner,
/// and how long an axis square to the view is drawn, as shares of the side.
const MARGIN: f32 = 0.02;
const LENGTH: f32 = 0.21;

const LINE_WIDTH: f32 = 2.0;

/// How large a letter is written against the face names, which it has to be
/// read beside.
const LETTER_OVER_NAME: f32 = 1.2;

/// How far past the tip of its line a letter's middle stands, in font sizes.
const LETTER_GAP: f32 = 0.6;

/// Which way from the tip of a line its letter goes, and how far as a share of
/// the gap: nearly straight along the direction the line mostly runs, which
/// keeps a slanting axis's letter off the names at the opening view, and for an
/// axis seen end-on, whose own line is too short to say, down and to the left,
/// off the two other lines. It turns smoothly from one to the other: the
/// letter's place feeds what the triad covers, which places all three lines, so
/// a letter changing sides at once would make them jump.
fn beyond(tip: egui::Vec2, letter_size: f32) -> egui::Vec2 {
    let line = tip.normalized();
    let leaned = |share: f32| share * share.abs().powi(LEANING);
    let runs = egui::vec2(leaned(line.x), leaned(line.y)).normalized();
    let seen = (tip.length() / (letter_size * LONG_ENOUGH)).min(1.0);
    egui::vec2(-1.0, 1.0).normalized() * (1.0 - seen) + runs * seen
}

/// How long a line has to be, in font sizes, to say alone where its letter
/// goes. A shorter one spins faster as the view turns, and shares the say with
/// the place of an axis seen end-on.
const LONG_ENOUGH: f32 = 1.5;

/// How strongly a letter leans towards the direction its line mostly runs: any
/// more and it swings too fast as the line turns, any less and the letter of a
/// slanting axis reaches a name at the opening view.
const LEANING: i32 = 3;

/// Paints the triad with `painter`, the lines drawn from one point.
///
/// That point is worked out from what the triad covers at this camera, so that
/// its drawing sits against the square's bottom left corner, sticking out of
/// the cube, however the axes turn. The triad stays off the names at the
/// opening view and square onto a sketch. A letter larger than the square is
/// dropped.
pub(super) fn paint(
    painter: &egui::Painter,
    state: &ViewportState,
    cube_rect: egui::Rect,
    font_size: f32,
) {
    let (camera, theme) = (&state.camera, &state.theme);
    let side = cube_rect.width();
    let letter_size = font_size * LETTER_OVER_NAME;

    let axes = [
        (Vec3::X, "X", theme.axis_x),
        (Vec3::Y, "Y", theme.axis_y),
        (Vec3::Z, "Z", theme.axis_z),
    ]
    .map(|(axis, name, colour)| {
        let ink = tint_to_color(colour);
        let tip = egui::vec2(camera.right().dot(axis), -camera.up().dot(axis)) * LENGTH * side;
        let font = egui::FontId::proportional(letter_size);
        let letter = Some(painter.layout_no_wrap(name.to_string(), font, ink))
            .filter(|letter| letter.size().max_elem() <= side)
            .map(|letter| {
                let middle = tip + beyond(tip, letter_size) * letter_size * LETTER_GAP;
                let inked = letter.mesh_bounds.size();
                (
                    letter,
                    egui::Rect::from_center_size(middle.to_pos2(), inked),
                )
            });
        (tip, letter, ink)
    });

    let covered = axes.iter().fold(
        egui::Rect::from_min_max(egui::Pos2::ZERO, egui::Pos2::ZERO),
        |covered, (tip, letter, _)| {
            let with_line =
                covered.union(egui::Rect::from_two_pos(egui::Pos2::ZERO, tip.to_pos2()));
            letter
                .as_ref()
                .map_or(with_line, |(_, place)| with_line.union(*place))
        },
    );
    let origin = cube_rect.left_bottom() + egui::vec2(MARGIN, -MARGIN) * side
        - covered.left_bottom().to_vec2();

    for (tip, letter, ink) in axes {
        painter.line_segment([origin, origin + tip], egui::Stroke::new(LINE_WIDTH, ink));
        if let Some((letter, place)) = letter {
            let at = origin + place.min.to_vec2() - letter.mesh_bounds.min.to_vec2();
            painter.galley(at, letter, ink);
        }
    }
}
