//! What an extrusion is about to turn into matter: the areas picked for it,
//! the one under the cursor, and the axis a revolution would turn around.

use cao_prefs::theme::Theme;
use cao_sketch::Sketch;
use glam::DVec2;

use super::{tint, tint_at};
use crate::screens::SketchContext;

/// Marks the areas picked for an extrusion, and the one under the cursor.
///
/// A chosen area is filled with the colour of the matter it is about to become,
/// which is the only preview needed before the height is typed.
pub(crate) fn push_chosen_areas(
    surfaces: &mut Vec<cao_render::Vertex>,
    lines: &mut Vec<cao_render::Vertex>,
    theme: &Theme,
    context: &SketchContext<'_>,
) {
    if !context.extrusion.is_active() {
        return;
    }
    let Some(sketch) = context
        .extrusion
        .sketch
        .and_then(|index| context.document.sketches().get(index))
    else {
        return;
    };

    let cutting = context.extrusion.mode == Some(cao_part::ExtrusionMode::Cut);
    let chosen = if cutting {
        tint(theme.extrusion_cut)
    } else {
        tint(theme.extrusion_add)
    };

    if context.extrusion.is_revolving() {
        push_revolution_axis(lines, sketch, theme, context);
    }

    for (index, region) in sketch.regions().iter().enumerate() {
        let picked = context
            .extrusion
            .picks
            .iter()
            .any(|pick| region.contains(*pick));
        let hovered = context.extrusion.hovered == Some(index);
        if !picked && !hovered {
            continue;
        }
        let color = if picked {
            chosen
        } else {
            tint_at(theme.highlight, theme.highlight.a * 0.5)
        };

        // Holes stay empty here too: what is shown filled is exactly what will become matter.
        for [a, b, c] in region.face_triangles() {
            for corner in [a, b, c] {
                surfaces.push(cao_render::Vertex::solid(
                    sketch.plane.to_world(corner).as_vec3(),
                    color,
                ));
            }
        }
    }
}

/// Draws the axis a revolution turns around, well past the drawing so it reads
/// as an axis rather than as one more line of the sketch.
fn push_revolution_axis(
    lines: &mut Vec<cao_render::Vertex>,
    sketch: &Sketch,
    theme: &Theme,
    context: &SketchContext<'_>,
) {
    let (origin, direction) = match context.extrusion.axis {
        cao_part::RevolutionAxis::Sketch(axis) => (DVec2::ZERO, axis.direction()),
        cao_part::RevolutionAxis::Segment(segment) => {
            if segment.0 >= sketch.segments().len() {
                return;
            }
            let (start, end) = sketch.endpoints(segment);
            (start, (end - start).normalize_or(DVec2::X))
        }
    };

    let reach = sketch
        .bounds()
        .map(|(min, max)| (max - min).length())
        .unwrap_or(1.0)
        .max(1.0);
    let color = tint_at(theme.sketch_free, 0.9);
    lines.push(cao_render::Vertex::line(
        sketch.plane.to_world(origin - direction * reach).as_vec3(),
        color,
        2.0,
    ));
    lines.push(cao_render::Vertex::line(
        sketch.plane.to_world(origin + direction * reach).as_vec3(),
        color,
        2.0,
    ));
}

#[cfg(test)]
mod tests;
