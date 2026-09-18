//! Which closed areas of a sketch an extrusion is being offered, and which
//! of them a click takes.

use crate::screens::viewport::{SketchContext, ViewScale, ViewportState, to_ndc};

/// Choosing which closed areas of a sketch become matter.
///
/// An area is named by a point inside it rather than by its rank, so the choice
/// still means the same thing after the drawing changes. Clicking an area
/// already chosen takes it back out.
pub(crate) fn pick_areas(
    state: &ViewportState,
    response: &egui::Response,
    rect: egui::Rect,
    scale: ViewScale,
    context: &mut SketchContext<'_>,
) {
    context.extrusion.hovered = None;

    let Some(index) = context.extrusion.sketch else {
        return;
    };
    let Some(sketch) = context.document.sketches().get(index) else {
        return;
    };
    let Some(pointer) = response.hover_pos() else {
        return;
    };

    let (origin, direction) = state
        .camera
        .ray(to_ndc(pointer, rect), rect.width() / rect.height());
    let Some(cursor) = sketch
        .plane
        .ray_intersection(origin.as_dvec3(), direction.as_dvec3())
    else {
        return;
    };

    // A line is a much smaller target than an area, so it is offered first:
    // that is how a drawn line becomes the axis a revolution turns around.
    if context.extrusion.is_revolving()
        && response.clicked()
        && let Some(segment) = sketch.nearest_segment(cursor, scale.world_size_of(8.0))
    {
        context.extrusion.axis = cao_part::RevolutionAxis::Segment(segment);
        return;
    }

    let regions = sketch.regions();
    let Some(under) = cao_sketch::area_under(&regions, cursor) else {
        return;
    };
    context.extrusion.hovered = Some(under);

    if !response.clicked() {
        return;
    }
    let already = context
        .extrusion
        .picks
        .iter()
        .position(|pick| regions[under].contains(*pick));
    match already {
        Some(position) => {
            context.extrusion.picks.remove(position);
        }
        None => context.extrusion.picks.push(cursor),
    }
}
