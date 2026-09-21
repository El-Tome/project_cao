//! The small marks drawn on the plane: a point, the middle of a line, a right
//! angle. Each is kept a constant size on screen, so it stays readable and
//! clickable at any zoom.

use cao_prefs::theme::Theme;
use cao_sketch::{Element, PointId, Selection, Sketch};
use glam::DVec2;

use super::{emphasis, shown_position, sketch_colors, tint, tint_at};
use crate::screens::viewport::{SketchContext, ViewScale};

/// Points are drawn as small squares kept at a constant size on screen, so they
/// stay visible and clickable at any zoom. The sketch origin gets a diamond
/// instead: it is always there, cannot be moved, and everything can be measured
/// from it, so it should not look like an ordinary point.
pub(crate) fn push_point_markers(
    out: &mut Vec<cao_render::Vertex>,
    sketch: &Sketch,
    theme: &Theme,
    scale: ViewScale,
    settled: &[bool],
    laid: &[Element],
    context: &SketchContext<'_>,
) {
    let half = scale.world_size_of(4.0);
    let highlight = tint_at(theme.highlight, 1.0);

    for index in 0..sketch.points().len() {
        let point = PointId(index);
        if sketch.is_erased_point(point) {
            continue;
        }
        let (base, _) = match sketch.is_held(Element::Point(point)) {
            true => (tint(theme.fixed), theme.sketch_width),
            false => sketch_colors(theme, true, settled.get(index).copied().unwrap_or(false)),
        };
        let center = sketch
            .plane
            .to_world(shown_position(sketch, point, context));
        let hovered = context.editor.hovered_point == Some(point)
            || context.editor.first_point() == Some(point);
        let color = if hovered { highlight } else { base };
        let width = if hovered { 2.5 } else { 1.5 };
        let (color, width) = emphasis::mark(
            context,
            theme,
            Selection::Element(Element::Point(point)),
            laid,
            color,
            width,
        );

        let (u, v) = if sketch.is_origin(point) {
            // Turned a quarter: a diamond reads differently at a glance.
            (
                (sketch.plane.u + sketch.plane.v) * std::f64::consts::FRAC_1_SQRT_2,
                (sketch.plane.v - sketch.plane.u) * std::f64::consts::FRAC_1_SQRT_2,
            )
        } else {
            (sketch.plane.u, sketch.plane.v)
        };
        let size = if sketch.is_origin(point) {
            half * 1.6
        } else {
            half
        };

        let corners = [
            center - u * size - v * size,
            center + u * size - v * size,
            center + u * size + v * size,
            center - u * size + v * size,
        ];
        for corner in 0..4 {
            out.push(cao_render::Vertex::line(
                corners[corner].as_vec3(),
                color,
                width,
            ));
            out.push(cao_render::Vertex::line(
                corners[(corner + 1) % 4].as_vec3(),
                color,
                width,
            ));
        }
    }
}

/// A small square outline on the plane, which is what a point looks like.
pub(crate) fn push_point_marker(
    out: &mut Vec<cao_render::Vertex>,
    sketch: &Sketch,
    at: DVec2,
    size: f64,
    color: [f32; 4],
    width: f32,
) {
    let center = sketch.plane.to_world(at);
    let (u, v) = (sketch.plane.u, sketch.plane.v);
    let corners = [
        center - u * size - v * size,
        center + u * size - v * size,
        center + u * size + v * size,
        center - u * size + v * size,
    ];
    for corner in 0..4 {
        out.push(cao_render::Vertex::line(
            corners[corner].as_vec3(),
            color,
            width,
        ));
        out.push(cao_render::Vertex::line(
            corners[(corner + 1) % 4].as_vec3(),
            color,
            width,
        ));
    }
}

/// The mark for the middle of a line: a small triangle, as on a drawing.
///
/// A shape of its own rather than the square used for a point — the two mean
/// different things and must not be told apart by size alone.
pub(crate) fn push_midpoint_mark(
    out: &mut Vec<cao_render::Vertex>,
    sketch: &Sketch,
    at: DVec2,
    scale: ViewScale,
    color: [f32; 4],
) {
    let size = scale.world_size_of(6.0);
    let corners = [
        at + DVec2::new(0.0, size),
        at + DVec2::new(-size, -size * 0.7),
        at + DVec2::new(size, -size * 0.7),
    ];
    for index in 0..3 {
        out.push(cao_render::Vertex::line(
            sketch.plane.to_world(corners[index]).as_vec3(),
            color,
            2.0,
        ));
        out.push(cao_render::Vertex::line(
            sketch.plane.to_world(corners[(index + 1) % 3]).as_vec3(),
            color,
            2.0,
        ));
    }
}

/// The draughtsman's mark for a right angle: a small square tucked into the
/// corner, on the inside of the two arms.
pub(crate) fn push_square_mark(
    out: &mut Vec<cao_render::Vertex>,
    sketch: &Sketch,
    corner: DVec2,
    along: DVec2,
    across: DVec2,
    scale: ViewScale,
    color: [f32; 4],
) {
    let (along, across) = (along.normalize_or_zero(), across.normalize_or_zero());
    if along == DVec2::ZERO || across == DVec2::ZERO {
        return;
    }
    // Kept the same size on screen: it marks a corner, it does not measure it.
    let side = scale.world_size_of(11.0);
    let (a, b) = (along * side, across * side);

    for (start, end) in [(a, a + b), (a + b, b)] {
        out.push(cao_render::Vertex::line(
            sketch.plane.to_world(corner + start).as_vec3(),
            color,
            2.0,
        ));
        out.push(cao_render::Vertex::line(
            sketch.plane.to_world(corner + end).as_vec3(),
            color,
            2.0,
        ));
    }
}

#[cfg(test)]
mod tests;
