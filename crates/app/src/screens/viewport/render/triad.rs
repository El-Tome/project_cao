//! The X/Y/Z triad built into the orientation cube: three axes starting from
//! one of its corners and running along its edges, past their far corners. An
//! axis behind the cube is not drawn.

use cao_render::{CubeFace, cube};
use glam::Vec3;

use super::overlays::tint_to_color;
use crate::screens::viewport::{ViewportState, cube_labels};

/// Each axis, its letter, and the two faces its edge separates.
const EDGES: [(Vec3, &str, [CubeFace; 2]); 3] = [
    (Vec3::X, "X", [CubeFace::MinusY, CubeFace::MinusZ]),
    (Vec3::Y, "Y", [CubeFace::MinusX, CubeFace::MinusZ]),
    (Vec3::Z, "Z", [CubeFace::MinusX, CubeFace::MinusY]),
];

/// The corner of the cube the three axes start from.
const CORNER: Vec3 = Vec3::splat(-0.5);

/// How far a line goes past the far corner of its edge, as a share of the edge:
/// more would leave the square at some views.
const PAST_THE_FAR_CORNER: f32 = 0.2;

/// How far past edge-on a face has to turn to be seen, as the dot of its normal
/// with the view: a face exactly edge-on is not, or two axes would be drawn on
/// one line at the views along an edge of the cube.
const FACE_SEEN_PAST: f32 = 1e-3;

/// How close to 1 the dot of an axis with the view has to come for the axis to
/// be end-on.
const AXIS_END_ON_WITHIN: f32 = 1e-3;

const LINE_WIDTH: f32 = 2.0;

/// How large a letter is written against the face names.
const LETTER_OVER_NAME: f32 = 1.2;

/// How far past the tip of its line a letter's middle stands, in font sizes,
/// along the axis in cube space: the direction on screen spins as an axis comes
/// end-on, and the letter would spin with it.
const LETTER_GAP: f32 = 0.7;

fn is_end_on(axis: Vec3, forward: Vec3) -> bool {
    axis.dot(forward).abs() >= 1.0 - AXIS_END_ON_WITHIN
}

/// Never end-on; otherwise when a face its edge bounds is strictly seen, or
/// when another axis is end-on, the view then being square onto a plane this
/// axis lies in.
fn is_shown(axis: Vec3, faces: [CubeFace; 2], forward: Vec3) -> bool {
    !is_end_on(axis, forward)
        && (EDGES.iter().any(|(other, _, _)| is_end_on(*other, forward))
            || faces
                .iter()
                .any(|face| face.normal().dot(forward) < -FACE_SEEN_PAST))
}

/// Paints the triad with `painter`: the line of each axis shown, and its letter
/// held inside the square. A letter larger than the square is dropped and its
/// line stays.
pub(super) fn paint(
    painter: &egui::Painter,
    state: &ViewportState,
    cube_rect: egui::Rect,
    font_size: f32,
) {
    let (camera, theme) = (&state.camera, &state.theme);
    let forward = camera.forward();
    let view_projection = cube::view_projection(camera.rotation());
    let on_the_square = |point| cube_labels::on_the_square(view_projection, cube_rect, point);
    let letter_size = font_size * LETTER_OVER_NAME;
    let letter_gap = letter_size * LETTER_GAP / cube_labels::points_per_unit(cube_rect);

    for ((axis, name, faces), colour) in
        EDGES
            .into_iter()
            .zip([theme.axis_x, theme.axis_y, theme.axis_z])
    {
        if !is_shown(axis, faces, forward) {
            continue;
        }
        let ink = tint_to_color(colour);
        let reach = 1.0 + PAST_THE_FAR_CORNER;
        let line = [on_the_square(CORNER), on_the_square(CORNER + axis * reach)];
        painter.line_segment(line, egui::Stroke::new(LINE_WIDTH, ink));

        let font = egui::FontId::proportional(letter_size);
        let letter = painter.layout_no_wrap(name.to_string(), font, ink);
        let inked = letter.mesh_bounds;
        if inked.size().max_elem() > cube_rect.width() {
            continue;
        }
        let room = inked.size() * 0.5;
        let middle = on_the_square(CORNER + axis * (reach + letter_gap))
            .clamp(cube_rect.min + room, cube_rect.max - room);
        painter.galley(middle - inked.center().to_vec2(), letter, ink);
    }
}
