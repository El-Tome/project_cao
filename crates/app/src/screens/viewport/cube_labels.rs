//! Sizing for the orientation cube's face labels.
//!
//! A face's on-screen shape is a parallelogram — the affine image of a
//! square — so it stays a face-sized target for the label no matter how the
//! camera is turned, and no matter how long the word for that face is in the
//! language showing.

use cao_render::CubeFace;
use cao_render::cube;

/// The smallest a label is still drawn at; below this it is dropped instead.
pub(super) const MINIMUM_READABLE_FONT: f32 = 8.0;

const REFERENCE_FONT_SIZE: f32 = 64.0;

/// Where a face's label goes, and how large it can be without spilling past
/// that face — `None` when nothing readable fits, in which case the face is
/// better left bare than drawn over its neighbour.
pub(super) fn face_label(
    painter: &egui::Painter,
    view_projection: glam::Mat4,
    cube_rect: egui::Rect,
    face: CubeFace,
    text: &str,
    preferred: f32,
) -> Option<(egui::Pos2, f32)> {
    let project = |point: glam::Vec3| {
        let clip = view_projection * point.extend(1.0);
        egui::pos2(
            cube_rect.center().x + clip.x / clip.w * cube_rect.width() * 0.5,
            cube_rect.center().y - clip.y / clip.w * cube_rect.height() * 0.5,
        )
    };
    let corners = cube::corners(face).map(project);
    let half_extent = painter
        .layout_no_wrap(
            text.to_string(),
            egui::FontId::proportional(REFERENCE_FONT_SIZE),
            egui::Color32::WHITE,
        )
        .size()
        * 0.5;
    let font_size = fitting_font_size(
        (corners[1] - corners[0]) * 0.5,
        (corners[3] - corners[0]) * 0.5,
        half_extent,
        REFERENCE_FONT_SIZE,
        preferred,
        MINIMUM_READABLE_FONT,
    )?;
    Some((project(cube::face_center(face)), font_size))
}

/// The largest font size, at or below `preferred`, whose text — measured at
/// `reference_size` giving `half_extent_at_reference` — stays inside the
/// face's projected shape. `None` when even `minimum_readable` would not fit,
/// which is when the label is better dropped than left spilling onto a
/// neighbouring face.
fn fitting_font_size(
    half_edge_a: egui::Vec2,
    half_edge_b: egui::Vec2,
    half_extent_at_reference: egui::Vec2,
    reference_size: f32,
    preferred: f32,
    minimum_readable: f32,
) -> Option<f32> {
    let scale = inscribed_scale(half_edge_a, half_edge_b, half_extent_at_reference);
    let size = (reference_size * scale * FIT_MARGIN).min(preferred);
    (size >= minimum_readable).then_some(size)
}

/// Room left between the fitted text and the face's own edge.
const FIT_MARGIN: f32 = 0.85;

/// The largest `t` for which a rectangle of half-extents `t * corner`,
/// centred where `a` and `b` meet, stays inside the parallelogram spanned by
/// `a` and `b` around that same centre.
///
/// The parallelogram is `{s*a + t*b : |s| <= 1, |t| <= 1}`, a convex set
/// centred on the origin, so it holds the rectangle's four corners — the two
/// points passed in and their negations — exactly when it holds those two:
/// the rest follow by symmetry and convexity.
fn inscribed_scale(a: egui::Vec2, b: egui::Vec2, corner: egui::Vec2) -> f32 {
    let determinant = a.x * b.y - a.y * b.x;
    if determinant.abs() < 1e-6 {
        return 0.0;
    }
    let scale_to_edge = |point: egui::Vec2| -> f32 {
        let s = (point.x * b.y - point.y * b.x) / determinant;
        let t = (a.x * point.y - a.y * point.x) / determinant;
        let reach = s.abs().max(t.abs());
        if reach < 1e-6 {
            f32::INFINITY
        } else {
            1.0 / reach
        }
    };
    scale_to_edge(corner).min(scale_to_edge(egui::vec2(corner.x, -corner.y)))
}

#[cfg(test)]
mod tests;
