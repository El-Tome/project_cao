//! The surfaces a new sketch can be started on: the three planes through the
//! world origin, and the face of the part under the cursor.

use cao_sketch::WorkPlane;

use super::{tint, tint_at};
use crate::screens::SketchContext;
use crate::screens::sketch::PlaneChoice;
use crate::screens::viewport::{ViewportState, plane_half_size};
use cao_render::{push_plane_outline, push_plane_quad};

pub(crate) fn push_choosable_planes(
    surfaces: &mut Vec<cao_render::Vertex>,
    lines: &mut Vec<cao_render::Vertex>,
    state: &ViewportState,
    context: &SketchContext<'_>,
) {
    let theme = &state.theme;
    let half_size = plane_half_size(state);
    // Once there is a part, the three planes step back: they are still there to
    // be picked, but they no longer hide the faces one usually wants.
    let has_body = !context.document.body().is_empty();
    let faded = if has_body { 0.35 } else { 1.0 };

    match context.editor.hovered_plane {
        Some(PlaneChoice::Face { face, .. }) => {
            push_hovered_face(surfaces, tint(theme.highlight), context, face);
        }
        Some(PlaneChoice::Curved(face)) => {
            push_hovered_face(surfaces, tint(theme.refused), context, face);
        }
        _ => {}
    }

    for (index, plane) in WorkPlane::ORIGIN_PLANES.iter().enumerate() {
        let hovered = context.editor.hovered_plane == Some(PlaneChoice::Origin(index));
        let fill = if hovered {
            tint(theme.highlight)
        } else {
            tint_at(theme.sketch_inactive, 0.12 * faded)
        };
        let outline = if hovered {
            tint_at(theme.highlight, 1.0)
        } else {
            tint_at(theme.sketch_inactive, 0.7 * faded)
        };
        push_plane_quad(
            surfaces,
            plane.origin.as_vec3(),
            plane.u.as_vec3(),
            plane.v.as_vec3(),
            half_size as f32,
            fill,
        );
        push_plane_outline(
            lines,
            plane.origin.as_vec3(),
            plane.u.as_vec3(),
            plane.v.as_vec3(),
            half_size as f32,
            outline,
            if hovered { 2.5 } else { 1.5 },
        );
    }
}

/// Lights up the face of the part under the cursor, so it is clear what a click would sketch on.
/// Lights every piece of one face at once.
///
/// A face is stored as many flat pieces, and lighting only the piece under the
/// cursor would read as picking a fragment of it. Only the fill is drawn —
/// outlining each piece would show the seams between them, which are not
/// something the user drew.
pub(crate) fn push_hovered_face(
    surfaces: &mut Vec<cao_render::Vertex>,
    fill: [f32; 4],
    context: &SketchContext<'_>,
    face: usize,
) {
    for polygon in context.document.body().pieces_of(face) {
        for [a, b, c] in polygon.triangles() {
            for corner in [a, b, c] {
                surfaces.push(cao_render::Vertex::solid(corner.as_vec3(), fill));
            }
        }
    }
}

#[cfg(test)]
mod tests;
