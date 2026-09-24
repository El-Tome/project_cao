//! The drawing itself: its lines, its circles, its arcs, the areas it encloses
//! and the points that hold them — as recorded, or as a tool is about to
//! change them.

use cao_prefs::theme::Theme;
use cao_sketch::{Element, Going, PointId, Preview, Selection, Sketch};
use glam::DVec2;

use super::curves::{push_arc_at, push_circle_at, push_ellipse_at, push_line};
use super::marks::push_point_markers;
use super::preview::push_preview;
use super::trim::{LOUDER, push_going};
use super::{emphasis, live_offset, tint, tint_at};
use crate::screens::SketchContext;
use crate::screens::viewport::input::{copying_shows, corner_shows};
use crate::screens::viewport::{PICK_PIXELS, ViewScale};

/// Tints the areas the drawing encloses, so a closed contour reads as a face
/// rather than four separate lines.
///
/// A shape drawn inside another is tinted more heavily: without that, an
/// outline and the pocket in it wash into one another and the eye cannot tell which is which.
pub(crate) fn push_regions(
    surfaces: &mut Vec<cao_render::Vertex>,
    sketch: &Sketch,
    theme: &Theme,
    active: bool,
) {
    for region in sketch.regions() {
        // Deeper areas take more of the tint, which is what tells a shape
        // drawn inside another from the one it sits in.
        let shade = theme.region_fill.a * (1.0 + 0.6 * region.depth.min(4) as f32);
        let color = if active {
            tint_at(theme.region_fill, shade)
        } else {
            tint_at(theme.sketch_inactive, shade * 0.6)
        };
        for [a, b, c] in region.triangles {
            for corner in [a, b, c] {
                surfaces.push(cao_render::Vertex::solid(
                    sketch.plane.to_world(corner).as_vec3(),
                    color,
                ));
            }
        }
    }
}

/// Colours saying how settled the drawing is: a shape that still has freedom
/// left is drawn one way, one that is fully determined another. It is the
/// quickest possible answer to "is my part pinned down yet?".
pub(crate) fn sketch_colors(theme: &Theme, active: bool, constrained: bool) -> ([f32; 4], f32) {
    match (active, constrained) {
        (true, false) => (tint(theme.sketch_free), theme.sketch_width),
        (true, true) => (tint(theme.sketch_settled), theme.sketch_width),
        (false, _) => (tint(theme.sketch_inactive), theme.sketch_width * 0.6),
    }
}

/// Where a point is shown: at the cursor while it is being dragged, at its recorded place otherwise.
pub(crate) fn shown_position(
    sketch: &Sketch,
    point: PointId,
    context: &SketchContext<'_>,
) -> DVec2 {
    // The settled preview already has the point where the cursor put it, and
    // everything else where it followed; moving it again would put it twice.
    if context.editor.drag_preview().is_some() {
        return sketch.point(point);
    }
    match (
        context.editor.dragged_point(),
        context.editor.drag_position(),
    ) {
        (Some(dragged), Some(position)) if dragged == point && !sketch.is_origin(point) => position,
        _ => sketch.point(point),
    }
}

#[allow(clippy::too_many_arguments)]
/// What the tool in hand would lay if it were clicked right now, read from the
/// very call the click commits.
pub(crate) fn what_would_be_laid(
    sketch: &Sketch,
    context: &SketchContext<'_>,
    scale: ViewScale,
) -> Option<Preview> {
    let cursor = context.editor.cursor?;
    let snap = scale.world_size_of(PICK_PIXELS);
    let units = context.document.scale();
    corner_shows(sketch, context.editor, cursor, snap, units)
        .or_else(|| copying_shows(sketch, context.editor, cursor, snap, units))
}

/// A drawing as this frame paints it: the one on record, or what a tool is
/// offering in its place, with the pieces that are only offered named.
pub(crate) struct Shown<'a> {
    pub(crate) sketch: &'a Sketch,
    pub(crate) laid: &'a [Element],
    /// What the tool in hand would take away, drawn over the rest rather than
    /// in place of it: what is aimed at has to stay visible.
    pub(crate) going: Option<&'a Going>,
    pub(crate) active: bool,
}

pub(crate) fn push_sketch(
    out: &mut Vec<cao_render::Vertex>,
    surfaces: &mut Vec<cao_render::Vertex>,
    shown: &Shown<'_>,
    theme: &Theme,
    scale: ViewScale,
    context: &SketchContext<'_>,
) {
    let Shown {
        sketch,
        laid,
        going,
        active,
    } = *shown;
    push_regions(surfaces, sketch, theme, active);

    // Per element, not one verdict for the whole drawing: a contour can be
    // nailed down while its neighbour is still floating, and that is exactly
    // what tells the user what is left to do.
    let settled = sketch.settled_points(context.document.scale());
    let holds = |point: PointId| settled.get(point.0).copied().unwrap_or(false);

    for (id, segment) in sketch.live_segments() {
        let held = holds(segment.start) && holds(segment.end);
        let (color, width) = match sketch.is_held(Element::Segment(id)) {
            true => (tint(theme.fixed), theme.sketch_width),
            false => sketch_colors(theme, active, held),
        };
        let (color, width) = emphasis::mark(
            context,
            theme,
            Selection::Element(Element::Segment(id)),
            laid,
            color,
            width,
        );
        let start = sketch
            .plane
            .to_world(shown_position(sketch, segment.start, context));
        let end = sketch
            .plane
            .to_world(shown_position(sketch, segment.end, context));
        push_line(out, start, end, color, width, segment.construction, scale);
    }

    for (id, circle) in sketch.live_circles() {
        let (color, width) = match sketch.is_held(Element::Circle(id)) {
            true => (tint(theme.fixed), theme.sketch_width),
            false => sketch_colors(theme, active, holds(circle.center)),
        };
        let (color, width) = emphasis::mark(
            context,
            theme,
            Selection::Element(Element::Circle(id)),
            laid,
            color,
            width,
        );
        push_circle_at(
            out,
            sketch,
            sketch.point(circle.center),
            circle.radius,
            color,
            width,
            circle.construction,
            scale,
        );
    }

    for (id, arc) in sketch.live_arcs() {
        let (color, width) = match sketch.is_held(Element::Arc(id)) {
            true => (tint(theme.fixed), theme.sketch_width),
            false => sketch_colors(theme, active, holds(arc.center)),
        };
        let (color, width) = emphasis::mark(
            context,
            theme,
            Selection::Element(Element::Arc(id)),
            laid,
            color,
            width,
        );
        push_arc_at(
            out,
            sketch,
            sketch.arc_draft(id),
            color,
            width,
            arc.construction,
            scale,
        );
    }

    for (id, oval) in sketch.live_ellipses() {
        let stands = sketch.ellipse_points(id).into_iter().all(holds);
        let (color, width) = match sketch.is_held(Element::Ellipse(id)) {
            true => (tint(theme.fixed), theme.sketch_width),
            false => sketch_colors(theme, active, stands),
        };
        let (color, width) = emphasis::mark(
            context,
            theme,
            Selection::Element(Element::Ellipse(id)),
            laid,
            color,
            width,
        );
        push_ellipse_at(
            out,
            sketch,
            sketch.ellipse_polyline(id),
            color,
            width,
            oval.construction,
            scale,
        );
    }

    if !active {
        return;
    }

    push_point_markers(out, sketch, theme, scale, &settled, laid, context);
    emphasis::push_picked_axes(out, sketch, theme, context.editor.rule_picks());

    // Every dimension is drawn where it applies, with extension lines, arrows
    // and arcs, so the drawing says what holds it rather than just carrying a number.
    for dimension in sketch.dimensions() {
        let mut style = if dimension.driven {
            crate::screens::annotations::Style::driven(theme)
        } else {
            crate::screens::annotations::Style::driving(theme)
        };
        let target = Selection::Dimension(dimension.target);
        if context.editor.is_selected(target) || context.editor.hovered == Some(target) {
            style.color = tint_at(theme.highlight, 1.0);
            style.width *= 2.0;
        }
        // Last, so that a value both pointed at and about to go reads as going:
        // of the two things to say, that is the one the user cannot undo.
        if going.is_some_and(|going| going.values.contains(&dimension.target)) {
            style.color = tint(theme.going);
            style.width *= LOUDER;
        }
        crate::screens::annotations::push(
            out,
            sketch,
            dimension.target,
            &style,
            scale.units_per_pixel,
            live_offset(context, dimension.target),
        );
    }

    push_preview(out, sketch, theme, scale, context);

    if let Some(going) = going {
        push_going(out, sketch, going, theme, scale);
    }

    if let Some(cao_sketch::DimensionTarget::Length(selected)) = context.editor.selected()
        && selected.0 < sketch.segments().len()
    {
        let (start, end) = sketch.endpoints(selected);
        let highlight = tint_at(theme.highlight, 1.0);
        out.push(cao_render::Vertex::line(
            sketch.plane.to_world(start).as_vec3(),
            highlight,
            4.0,
        ));
        out.push(cao_render::Vertex::line(
            sketch.plane.to_world(end).as_vec3(),
            highlight,
            4.0,
        ));
    }
}

#[cfg(test)]
mod tests;
