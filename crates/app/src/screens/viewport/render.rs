//! What ends up painted: the wgpu scene (grid, axes, the sketch, the solid),
//! and everything egui draws on top of it — dimensions, rule marks, the band,
//! the live fields, the orientation cube's labels.
//!
//! What a click decides lives in [`super::input`].

use cao_prefs::theme::{Background, Rgba, Theme};
use cao_render::camera::CubeZone;
use cao_render::{
    AxisStyle, BackgroundShape, GridStyle, SceneFrame, ViewportRect, cube, push_axes, push_grid,
    push_plane_outline, push_plane_quad, srgb,
};
use cao_sketch::{
    ChainAnchor, DimensionTarget, Element, PointId, Selection, Sketch, Snap, WorkPlane,
};
use glam::{DVec2, DVec3};

use crate::screens::sketch::{DimensionMode, LiveField, PlaneChoice, Tool, apply_dimension_value};
use crate::wording::constraints;

use super::input::{annotation_position, circle_from, measure_preview, rectangle_corner, refine};
use super::{
    PICK_PIXELS, SketchContext, ViewMode, ViewScale, ViewportState, corner_origin, plane_half_size,
};

/// A colour from the theme, turned into the space the shader blends in.
fn tint(color: cao_prefs::theme::Rgba) -> [f32; 4] {
    srgb(color.r, color.g, color.b, color.a)
}

/// The same, with the opacity replaced.
fn tint_at(color: cao_prefs::theme::Rgba, alpha: f32) -> [f32; 4] {
    tint(color.with_alpha(alpha))
}

/// Where a world point lands on screen, or `None` when it is behind the camera.
fn to_screen(point: DVec3, view_projection: glam::Mat4, rect: egui::Rect) -> Option<egui::Pos2> {
    let clip = view_projection * point.as_vec3().extend(1.0);
    if clip.w <= 1e-6 {
        return None;
    }
    let ndc = clip.truncate() / clip.w;
    Some(egui::pos2(
        rect.center().x + ndc.x * rect.width() * 0.5,
        rect.center().y - ndc.y * rect.height() * 0.5,
    ))
}

pub(crate) fn build_frame(
    state: &ViewportState,
    rect: egui::Rect,
    cube_rect: egui::Rect,
    scale: ViewScale,
    context: &SketchContext<'_>,
) -> SceneFrame {
    let camera = &state.camera;
    let pixels_per_point = scale.height_px / rect.height();

    let theme = &state.theme;
    let mut lines = Vec::new();
    let mut world_lines = Vec::new();
    let mut surfaces = Vec::new();

    let mut background = Vec::new();
    let (shape, sample) = background_shape(&theme.background);
    cao_render::push_background(&mut background, shape, sample);

    // The grid is only drawn once the view has actually landed on the plane.
    // Mid-animation the view is oblique, and a grid of finite size seen at an
    // angle reads as a disc floating in the middle of the screen.
    if let (ViewMode::Plane(plane), None) = (state.mode, &state.transition) {
        // It also has to reach past the corners of the screen, or its outer
        // fade shows up as that same disc.
        let half_extent = (scale.units_per_pixel * scale.diagonal_px as f64 * 1.5) as f32;
        let normal = plane.normal().as_vec3();
        let origin = plane.origin.as_vec3();
        let center = camera.target() - normal * (camera.target() - origin).dot(normal);
        push_grid(
            &mut world_lines,
            plane.u.as_vec3(),
            plane.v.as_vec3(),
            center,
            scale.step as f32,
            half_extent,
            &GridStyle {
                minor: tint(theme.grid_minor),
                major: tint(theme.grid_major),
                minor_width: theme.grid_minor_width,
                major_width: theme.grid_major_width,
                major_every: theme.grid_major_every.max(1),
                ..GridStyle::default()
            },
        );
    }

    let facing = match state.mode {
        ViewMode::Plane(plane) => Some(plane.normal().as_vec3()),
        ViewMode::Free => None,
    };
    push_axes(
        &mut world_lines,
        camera.distance() * 50.0,
        &AxisStyle {
            x: tint(theme.axis_x),
            y: tint(theme.axis_y),
            z: tint(theme.axis_z),
            width: theme.axis_width,
        },
        facing,
    );

    if context.editor.is_choosing_plane() {
        push_choosable_planes(&mut surfaces, &mut lines, state, context);
    }

    for (index, sketch) in context.document.sketches().iter().enumerate() {
        let active = context.editor.active_sketch() == Some(index);
        // Mid-drag, the settled preview stands in for the recorded drawing.
        let shown = match (active, context.editor.drag_preview()) {
            (true, Some(preview)) => preview,
            _ => sketch,
        };
        push_sketch(
            &mut lines,
            &mut surfaces,
            shown,
            theme,
            scale,
            active,
            context,
        );
    }

    push_chosen_areas(&mut surfaces, &mut lines, theme, context);

    let mut solids = Vec::new();
    cao_render::push_solid(
        &mut solids,
        &context
            .document
            .body()
            .triangles()
            .iter()
            .map(|corners| corners.map(|corner| corner.as_vec3()))
            .collect::<Vec<_>>(),
        tint(theme.solid),
        camera.forward(),
    );

    let mut cube_triangles = Vec::new();
    let mut cube_edges = Vec::new();
    cube::push_faces(&mut cube_triangles, state.hovered_zone);
    cube::push_edges(&mut cube_edges, camera.forward(), 1.5);

    SceneFrame {
        scene_view_projection: camera.view_projection(scale.aspect),
        scene_viewport: to_physical(rect, pixels_per_point),
        scene_background: background,
        scene_world_lines: world_lines,
        scene_surfaces: surfaces,
        scene_solids: solids,
        scene_lines: lines,
        cube_view_projection: cube::view_projection(camera.rotation()),
        cube_triangles,
        cube_edges,
        cube_viewport: to_physical(cube_rect, pixels_per_point),
    }
}

fn push_choosable_planes(
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

    if let Some(PlaneChoice::Face(plane)) = context.editor.hovered_plane {
        push_hovered_face(surfaces, theme, context, plane);
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

/// Marks the areas picked for an extrusion, and the one under the cursor.
///
/// A chosen area is filled with the colour of the matter it is about to become,
/// which is the only preview needed before the height is typed.
fn push_chosen_areas(
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

        // Holes stay empty here too: what is shown filled is exactly what will
        // become matter.
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

/// Tints the areas the drawing encloses, so a closed contour reads as a face
/// rather than four separate lines.
///
/// A shape drawn inside another is tinted more heavily: without that, an
/// outline and the pocket in it wash into one another and the eye cannot tell
/// which is which.
fn push_regions(
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

/// Lights up the face of the part under the cursor, so it is clear what a click
/// would sketch on.
fn push_hovered_face(
    surfaces: &mut Vec<cao_render::Vertex>,
    theme: &Theme,
    context: &SketchContext<'_>,
    plane: WorkPlane,
) {
    let normal = plane.normal();
    let offset = plane.origin.dot(normal);
    let fill = tint(theme.highlight);

    // Every face lying on the same plane lights up together: a curved surface
    // and a cut one are both stored as many flat pieces, and lighting only the
    // piece under the cursor would read as picking a fragment of it. Only the
    // fill is drawn — outlining each piece would show the seams between them,
    // which are not something the user drew.
    for polygon in &context.document.body().polygons {
        if polygon.normal().dot(normal) < 0.999 || (polygon.plane_offset() - offset).abs() > 1e-4 {
            continue;
        }
        for [a, b, c] in polygon.triangles() {
            for corner in [a, b, c] {
                surfaces.push(cao_render::Vertex::solid(corner.as_vec3(), fill));
            }
        }
    }
}

/// Turns the background the user described into what the renderer draws.
fn background_shape(background: &Background) -> (BackgroundShape, impl Fn(f32) -> [f32; 4] + '_) {
    let shape = match background {
        Background::Solid(_) => BackgroundShape::Flat,
        Background::Linear { angle_degrees, .. } => BackgroundShape::Linear {
            angle_degrees: *angle_degrees,
        },
        Background::Radial { center, radius, .. } => BackgroundShape::Radial {
            center: *center,
            radius: *radius,
        },
    };
    (shape, move |t: f32| tint(background.sample(t)))
}

/// Colours saying how settled the drawing is: a shape that still has freedom
/// left is drawn one way, one that is fully determined another. It is the
/// quickest possible answer to "is my part pinned down yet?".
fn sketch_colors(theme: &Theme, active: bool, constrained: bool) -> ([f32; 4], f32) {
    match (active, constrained) {
        (true, false) => (tint(theme.sketch_free), theme.sketch_width),
        (true, true) => (tint(theme.sketch_settled), theme.sketch_width),
        (false, _) => (tint(theme.sketch_inactive), theme.sketch_width * 0.6),
    }
}

/// Where a point is shown: at the cursor while it is being dragged, at its
/// recorded place otherwise.
fn shown_position(sketch: &Sketch, point: PointId, context: &SketchContext<'_>) -> DVec2 {
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
fn push_sketch(
    out: &mut Vec<cao_render::Vertex>,
    surfaces: &mut Vec<cao_render::Vertex>,
    sketch: &Sketch,
    theme: &Theme,
    scale: ViewScale,
    active: bool,
    context: &SketchContext<'_>,
) {
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
        let (color, width) = mark_selected(
            context,
            theme,
            Selection::Element(Element::Segment(id)),
            color,
            width,
        );
        let start = sketch
            .plane
            .to_world(shown_position(sketch, segment.start, context));
        let end = sketch
            .plane
            .to_world(shown_position(sketch, segment.end, context));
        out.push(cao_render::Vertex::line(start.as_vec3(), color, width));
        out.push(cao_render::Vertex::line(end.as_vec3(), color, width));
    }

    for (id, circle) in sketch.live_circles() {
        let (color, width) = match sketch.is_held(Element::Circle(id)) {
            true => (tint(theme.fixed), theme.sketch_width),
            false => sketch_colors(theme, active, holds(circle.center)),
        };
        let (color, width) = mark_selected(
            context,
            theme,
            Selection::Element(Element::Circle(id)),
            color,
            width,
        );
        push_circle(out, sketch, circle.center, circle.radius, color, width);
    }

    if !active {
        return;
    }

    push_point_markers(out, sketch, theme, scale, &settled, context);

    // Every dimension is drawn where it applies, with extension lines, arrows
    // and arcs, so the drawing says what holds it rather than just carrying a
    // number.
    for dimension in sketch.dimensions() {
        let mut style = if dimension.driven {
            crate::screens::annotations::Style::driven(theme)
        } else {
            crate::screens::annotations::Style::driving(theme)
        };
        if context
            .editor
            .is_selected(Selection::Dimension(dimension.target))
        {
            style.color = tint_at(theme.highlight, 1.0);
            style.width *= 2.0;
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

/// Points are drawn as small squares kept at a constant size on screen, so they
/// stay visible and clickable at any zoom. The sketch origin gets a diamond
/// instead: it is always there, cannot be moved, and everything can be measured
/// from it, so it should not look like an ordinary point.
fn push_point_markers(
    out: &mut Vec<cao_render::Vertex>,
    sketch: &Sketch,
    theme: &Theme,
    scale: ViewScale,
    settled: &[bool],
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
        let (color, width) = mark_selected(
            context,
            theme,
            Selection::Element(Element::Point(point)),
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

/// How far an annotation has been dragged so far, before anything is recorded.
///
/// Nothing goes into the history until the button is let go, so without this
/// the annotation would sit still through the whole gesture and only jump at
/// the end — the drag would look like it had done nothing.
fn live_offset(context: &SketchContext<'_>, target: DimensionTarget) -> DVec2 {
    let editor = &context.editor;
    match (
        editor.dragged_dimension(),
        editor.drag_origin(),
        editor.drag_position(),
    ) {
        (Some(dragged), Some(origin), Some(position)) if dragged == target => position - origin,
        _ => DVec2::ZERO,
    }
}

/// Draws what the selection tool is holding differently, so it is clear what
/// pressing Suppr would take away.
fn mark_selected(
    context: &SketchContext<'_>,
    theme: &Theme,
    element: Selection,
    color: [f32; 4],
    width: f32,
) -> ([f32; 4], f32) {
    if context.editor.is_selected(element) {
        (tint_at(theme.highlight, 1.0), width * 1.8)
    } else {
        (color, width)
    }
}

/// A small square outline on the plane, which is what a point looks like.
fn push_point_marker(
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
fn push_midpoint_mark(
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
fn push_square_mark(
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

/// The annotation the dimension tool is showing in advance: the one a click
/// would choose, or the one already chosen and looking for its place.
fn pending_annotation(
    context: &SketchContext<'_>,
    index: usize,
    cursor: DVec2,
    snap: f64,
    pixel: f64,
) -> Option<(DimensionTarget, DVec2)> {
    if let Some(target) = context.editor.placing() {
        // A second entity under the cursor turns the dimension into another
        // one; it is shown where it would land, not dragged to the cursor.
        if context.editor.dimension_mode == DimensionMode::Auto
            && let Some(refined) = refine(context, index, target, cursor, snap)
        {
            return Some((refined, DVec2::ZERO));
        }
        let target = context
            .document
            .sketches()
            .get(index)
            .map_or(target, |sketch| sketch.oriented(target, cursor));
        // The preview is nudged from where the annotation stands today, not
        // moved to an absolute offset: `push` adds a nudge on top of whatever
        // the dimension already carries.
        let nudge = annotation_position(context, index, target, pixel)
            .map(|placement| cursor - placement.text_at)
            .unwrap_or_default();
        return Some((target, nudge));
    }
    let target = measure_preview(context, index, cursor, snap)?;
    Some((target, DVec2::ZERO))
}

/// The shape about to be drawn, following the cursor: placing a point blind and
/// only then seeing where it went is needlessly uncomfortable.
fn push_preview(
    out: &mut Vec<cao_render::Vertex>,
    sketch: &Sketch,
    theme: &Theme,
    scale: ViewScale,
    context: &SketchContext<'_>,
) {
    let Some(cursor) = context.editor.cursor else {
        return;
    };
    let preview = tint_at(theme.sketch_free, 0.55);

    if let Some(anchor) = context.editor.chain() {
        let from = match anchor {
            ChainAnchor::Pending(position) => position,
            ChainAnchor::Point(id) if id.0 < sketch.points().len() => sketch.point(id),
            ChainAnchor::Point(_) => cursor,
        };
        let to = context
            .editor
            .aimed
            .map(|aimed| aimed.position)
            .unwrap_or(cursor);
        push_preview_line(out, sketch, from, to, preview);

        // The little square of a right angle, drawn before it is committed to
        // so the constraint is never a surprise. Its two arms are the line
        // being drawn and the one it is squaring up against.
        if let Some(previous) = context.editor.aimed.and_then(|aimed| aimed.square_with)
            && previous.0 < sketch.segments().len()
        {
            let (start, end) = sketch.endpoints(previous);
            let arm = if start.distance(from) < end.distance(from) {
                end - start
            } else {
                start - end
            };
            push_square_mark(out, sketch, from, to - from, -arm, scale, preview);
        }
    }

    // The point tool has nothing pending, yet placing a point blind is exactly
    // as uncomfortable as the rest.
    if context.editor.tool == Tool::Point {
        push_point_marker(out, sketch, cursor, scale.world_size_of(4.0), preview, 1.5);
    }

    // The dimension a click would place, drawn faintly where it would land —
    // then, once it is chosen, the same annotation following the cursor to the
    // spot it will sit on.
    if context.editor.tool == Tool::Dimension
        && let Some(index) = context.editor.active_sketch()
        && let Some((target, nudge)) = pending_annotation(
            context,
            index,
            cursor,
            scale.world_size_of(PICK_PIXELS),
            scale.units_per_pixel,
        )
    {
        let mut style = crate::screens::annotations::Style::driving(theme);
        style.color = preview;
        style.width *= 0.9;
        crate::screens::annotations::push(
            out,
            sketch,
            target,
            &style,
            scale.units_per_pixel,
            nudge,
        );
    }

    // What the cursor has been caught by. The middle of a line is the one that
    // has to be said out loud: nothing else on screen tells you that you are
    // exactly halfway along it.
    match context.editor.snap {
        Some(Snap::Midpoint(at)) => {
            push_midpoint_mark(out, sketch, at, scale, tint_at(theme.highlight, 1.0))
        }
        Some(Snap::OnSegment(at)) => push_point_marker(
            out,
            sketch,
            at,
            scale.world_size_of(3.0),
            tint_at(theme.highlight, 1.0),
            2.0,
        ),
        _ => {}
    }

    if context.editor.tool == Tool::Circle
        && let Some(index) = context.editor.active_sketch()
        && let Some(found) = circle_from(context, index, cursor, scale.world_size_of(PICK_PIXELS))
    {
        push_circle_at(out, sketch, found.centre, found.radius, preview, 1.5);
        push_point_marker(
            out,
            sketch,
            found.centre,
            scale.world_size_of(3.0),
            preview,
            1.5,
        );
    }

    let Some(start) = context.editor.pending_start() else {
        return;
    };
    if context.editor.tool == Tool::Rectangle {
        let far = rectangle_corner(context, cursor);
        let corners = [
            start,
            DVec2::new(far.x, start.y),
            far,
            DVec2::new(start.x, far.y),
        ];
        for index in 0..4 {
            push_preview_line(
                out,
                sketch,
                corners[index],
                corners[(index + 1) % 4],
                preview,
            );
        }
    }
}

fn push_preview_line(
    out: &mut Vec<cao_render::Vertex>,
    sketch: &Sketch,
    from: DVec2,
    to: DVec2,
    color: [f32; 4],
) {
    out.push(cao_render::Vertex::line(
        sketch.plane.to_world(from).as_vec3(),
        color,
        1.5,
    ));
    out.push(cao_render::Vertex::line(
        sketch.plane.to_world(to).as_vec3(),
        color,
        1.5,
    ));
}

fn push_circle(
    out: &mut Vec<cao_render::Vertex>,
    sketch: &Sketch,
    center: PointId,
    radius: f64,
    color: [f32; 4],
    width: f32,
) {
    push_circle_at(out, sketch, sketch.point(center), radius, color, width);
}

/// Circles are drawn as a many-sided polygon: the line renderer only knows
/// about segments, and at this many sides the corners are invisible.
fn push_circle_at(
    out: &mut Vec<cao_render::Vertex>,
    sketch: &Sketch,
    center: DVec2,
    radius: f64,
    color: [f32; 4],
    width: f32,
) {
    const SIDES: usize = 96;
    if radius <= 0.0 {
        return;
    }
    let mut previous = None;
    for step in 0..=SIDES {
        let angle = step as f64 / SIDES as f64 * std::f64::consts::TAU;
        let point = center + DVec2::new(angle.cos(), angle.sin()) * radius;
        let world = sketch.plane.to_world(point);
        if let Some(previous) = previous {
            out.push(cao_render::Vertex::line(previous, color, width));
            out.push(cao_render::Vertex::line(world.as_vec3(), color, width));
        }
        previous = Some(world.as_vec3());
    }
}

fn to_physical(rect: egui::Rect, pixels_per_point: f32) -> ViewportRect {
    ViewportRect {
        x: rect.left() * pixels_per_point,
        y: rect.top() * pixels_per_point,
        width: rect.width() * pixels_per_point,
        height: rect.height() * pixels_per_point,
    }
}

/// The cube's labels are drawn by egui rather than the GPU: text needs a font
/// atlas, and egui already has one.
pub(crate) fn paint_face_labels(ui: &egui::Ui, state: &ViewportState, cube_rect: egui::Rect) {
    let painter = ui.painter_at(cube_rect);
    let view_projection = cube::view_projection(state.camera.rotation());
    let forward = state.camera.forward();
    let font = egui::FontId::proportional((state.config.cube_size * 0.11).max(8.0));

    for face in cao_render::CubeFace::ALL {
        if !cube::is_visible(face, forward) {
            continue;
        }
        let clip = view_projection * cube::face_center(face).extend(1.0);
        let position = egui::pos2(
            cube_rect.center().x + clip.x / clip.w * cube_rect.width() * 0.5,
            cube_rect.center().y - clip.y / clip.w * cube_rect.height() * 0.5,
        );
        let color = if state.hovered_zone == Some(CubeZone::Face(face)) {
            egui::Color32::WHITE
        } else {
            egui::Color32::from_gray(60)
        };
        painter.text(
            position,
            egui::Align2::CENTER_CENTER,
            face_label(face),
            font.clone(),
            color,
        );
    }
}

/// The length and the angle of the line being drawn, editable on the spot.
///
/// Left alone they only report. Typed into, they become constraints and are
/// placed as dimensions when the line is validated — which is the whole point:
/// a line drawn to a value should not have to be measured afterwards.
///
/// Returns true when the user pressed Enter to finish the line from the
/// keyboard.
pub(crate) fn paint_live_input(ui: &mut egui::Ui, context: &mut SketchContext<'_>) -> bool {
    paint_live_fields(ui, context).unwrap_or(false)
}

/// The same, written where a missing piece simply means there is nothing to
/// show yet.
fn paint_live_fields(ui: &mut egui::Ui, context: &mut SketchContext<'_>) -> Option<bool> {
    let index = context.editor.active_sketch()?;
    let sketch = context.document.sketches().get(index)?;
    let raw_cursor = context.editor.cursor?;
    let cursor = context
        .editor
        .aimed
        .map(|aimed| aimed.position)
        .unwrap_or(raw_cursor);
    let scale = context.document.scale();

    // A line is a length and an angle, a rectangle its two sides, a circle its
    // diameter and nothing else — so its second field is left out.
    let (labels, measured): ([&str; 2], [f64; 2]) = match context.editor.tool {
        Tool::Circle => {
            // Shown whether or not the picks make a circle just now: a field
            // that disappears while being typed into cannot be typed into.
            let across = circle_from(context, index, cursor, f64::MAX)
                .map(|found| found.radius * 2.0 * scale)
                .unwrap_or_default();
            (["mm", ""], [across, 0.0])
        }
        Tool::Line => {
            let from = sketch.anchor_position(context.editor.chain()?)?;
            let span = cursor - from;
            (
                ["mm", "°"],
                [span.length() * scale, span.y.atan2(span.x).to_degrees()],
            )
        }
        Tool::Rectangle => {
            let start = context.editor.pending_start()?;
            let far = rectangle_corner(context, raw_cursor);
            let span = far - start;
            (["mm", "mm"], [span.x.abs() * scale, span.y.abs() * scale])
        }
        _ => return None,
    };

    // Hung off the pointer itself, down and to the right: anchored on the
    // drawing, the fields ended up under the cursor, and a cursor over them is
    // a cursor no longer over the canvas — the shape stopped following it.
    let at = ui.ctx().pointer_latest_pos()?;

    let live = &mut context.editor.live;
    let mut validated = false;
    let focus = std::mem::take(&mut live.focus);
    egui::Area::new(egui::Id::new("live_input"))
        .fixed_pos(at + egui::vec2(20.0, 20.0))
        .order(egui::Order::Foreground)
        .show(ui.ctx(), |ui| {
            egui::Frame::popup(ui.style()).show(ui, |ui| {
                ui.horizontal(|ui| {
                    validated |= live_field(ui, labels[0], measured[0], &mut live.first, focus);
                    if !labels[1].is_empty() {
                        validated |=
                            live_field(ui, labels[1], measured[1], &mut live.second, false);
                    }
                });
            });
        });
    Some(validated)
}

/// A number field that takes the keyboard on demand, whole value selected.
///
/// Selecting it matters: the field arrives holding the value already there, and
/// without it the first keystroke lands after it — 40 typed over 60.88 read
/// 60.8840.
fn value_field(ui: &mut egui::Ui, text: &mut String, hint: &str, focus: bool) -> egui::Response {
    let output = egui::TextEdit::singleline(text)
        .desired_width(72.0)
        .hint_text(hint)
        .show(ui);
    let response = output.response.response;
    if focus {
        response.request_focus();
        let mut state = output.state;
        state
            .cursor
            .set_char_range(Some(egui::text::CCursorRange::two(
                egui::text::CCursor::new(0),
                egui::text::CCursor::new(text.chars().count()),
            )));
        state.store(ui.ctx(), response.id);
    }
    response
}

/// One of the two fields. Returns true when Enter was pressed in it.
///
/// An untouched field stays empty and shows what the cursor is doing as a hint:
/// keeping the readout in the field itself meant the first keystroke landed
/// after it, and "40" typed over "0.000" read 0.00040.
fn live_field(
    ui: &mut egui::Ui,
    suffix: &str,
    measured: f64,
    field: &mut LiveField,
    focus: bool,
) -> bool {
    // The keyboard goes to the first field as soon as the fields appear: the
    // value is the next thing the user types, and Tab from the canvas walks
    // through the whole toolbar to get here.
    let response = value_field(ui, &mut field.text, &format!("{measured:.2}"), focus);
    ui.label(suffix);

    // Typing is what turns a readout into a decision. Emptying the field takes
    // the decision back.
    if response.changed() {
        field.locked = crate::screens::sketch::LiveInput::read(&field.text);
    }
    // Enter is consumed rather than merely read: the field has just given the
    // keyboard back, so the shortcut bound to that key would fire too.
    response.lost_focus()
        && ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Enter))
}

/// The marks of the rules, written beside what they hold.
///
/// Text rather than drawn symbols: a rule has no size and no direction of its
/// own, so there is nothing to draw it *at* — only something to write next to.
pub(crate) fn paint_rule_marks(
    ui: &egui::Ui,
    state: &ViewportState,
    rect: egui::Rect,
    context: &SketchContext<'_>,
) {
    let Some(index) = context.editor.active_sketch() else {
        return;
    };
    // Mid-drag the marks are read off the settled preview, like the values of
    // the dimensions: read from the recorded drawing they would stay behind
    // and only catch up when the button came up.
    let Some(sketch) = context
        .editor
        .drag_preview()
        .or_else(|| context.document.sketches().get(index))
    else {
        return;
    };
    let view_projection = state
        .camera
        .view_projection(rect.width() / rect.height().max(1.0));
    let painter = ui.painter_at(rect);

    // Several rules can hold the same corner — a middle and a right angle, say
    // — so marks that would land on each other are set side by side.
    let mut taken: Vec<egui::Pos2> = Vec::new();
    for constraint in sketch.constraints() {
        let held = context.editor.is_selected(Selection::Rule(*constraint));
        for at in sketch.rule_marks(*constraint) {
            let Some(position) = to_screen(sketch.plane.to_world(at), view_projection, rect) else {
                continue;
            };
            let mut position = position + egui::vec2(9.0, -9.0);
            while taken
                .iter()
                .any(|held| held.distance(position) < MARK_SPACING)
            {
                position.x += MARK_SPACING;
            }
            taken.push(position);
            painter.text(
                position,
                egui::Align2::CENTER_CENTER,
                constraints::mark(*constraint),
                egui::FontId::proportional(13.0),
                match held {
                    true => tint_to_color(state.theme.highlight),
                    false => tint_to_color(state.theme.rule),
                },
            );
        }
    }
}

/// How far apart two marks are set when they would otherwise land on top of
/// each other, in points.
const MARK_SPACING: f32 = 16.0;

/// The box being pulled across the drawing./// The box being pulled across the drawing.
///
/// Drawn on the sketch's own axes rather than the screen's: seen at an angle,
/// a box that stayed square on screen would take in a different area than the
/// one it appears to cover.
pub(crate) fn paint_band(
    ui: &egui::Ui,
    state: &ViewportState,
    rect: egui::Rect,
    context: &SketchContext<'_>,
) {
    let (Some((from, to)), Some(index)) = (context.editor.band(), context.editor.active_sketch())
    else {
        return;
    };
    let Some(sketch) = context.document.sketches().get(index) else {
        return;
    };
    let view_projection = state
        .camera
        .view_projection(rect.width() / rect.height().max(1.0));

    let corners: Option<Vec<egui::Pos2>> =
        [from, DVec2::new(to.x, from.y), to, DVec2::new(from.x, to.y)]
            .into_iter()
            .map(|corner| to_screen(sketch.plane.to_world(corner), view_projection, rect))
            .collect();
    let Some(corners) = corners else {
        return;
    };

    let edge = tint_to_color(state.theme.highlight);
    ui.painter_at(rect).add(egui::Shape::convex_polygon(
        corners,
        edge.gamma_multiply(0.12),
        egui::Stroke::new(1.0, edge),
    ));
}

/// A colour of the theme as egui paints it.
fn tint_to_color(color: Rgba) -> egui::Color32 {
    let channel = |value: f32| (value.clamp(0.0, 1.0) * 255.0) as u8;
    egui::Color32::from_rgba_unmultiplied(
        channel(color.r),
        channel(color.g),
        channel(color.b),
        channel(color.a),
    )
}

/// Each dimension is drawn where it applies, with the value it stands for.
/// A readout shows what the geometry measures rather than a stored number, so
/// it stays true however the drawing moves.
pub(crate) fn paint_dimension_labels(
    ui: &egui::Ui,
    state: &ViewportState,
    rect: egui::Rect,
    context: &SketchContext<'_>,
) {
    let Some(index) = context.editor.active_sketch() else {
        return;
    };
    // Mid-drag the annotations are drawn from the settled preview, so their
    // values have to be read from the same drawing — otherwise the numbers stay
    // behind while the lines they belong to move away.
    let Some(sketch) = context
        .editor
        .drag_preview()
        .or_else(|| context.document.sketches().get(index))
    else {
        return;
    };
    let view_projection = state
        .camera
        .view_projection(rect.width() / rect.height().max(1.0));
    let painter = ui.painter_at(rect);

    let pixel = state
        .camera
        .world_units_per_pixel(rect.height() * ui.ctx().pixels_per_point()) as f64;

    for dimension in sketch.dimensions() {
        // Asking the annotation where its value belongs keeps the text on the
        // dimension line instead of floating near the geometry.
        let mut ignored = Vec::new();
        let style = crate::screens::annotations::Style::driving(&state.theme);
        let Some(text_at) = crate::screens::annotations::push(
            &mut ignored,
            sketch,
            dimension.target,
            &style,
            pixel,
            live_offset(context, dimension.target),
        ) else {
            continue;
        };
        let Some(position) = to_screen(sketch.plane.to_world(text_at), view_projection, rect)
        else {
            continue;
        };
        let value = if dimension.driven {
            context
                .document
                .measured(index, dimension.target)
                .unwrap_or(dimension.value)
        } else {
            dimension.value
        };
        let text = if dimension.is_angle() {
            format!("{value:.1}°")
        } else {
            state.config.unit.format(value)
        };
        let color = if dimension.driven {
            egui::Color32::from_gray(170)
        } else {
            egui::Color32::from_rgb(250, 220, 120)
        };
        painter.text(
            position,
            egui::Align2::CENTER_CENTER,
            if dimension.driven {
                format!("({text})")
            } else {
                text
            },
            egui::FontId::proportional(13.0),
            color,
        );
    }

    // The dimension being placed carries its value with it: a bare pair of
    // arrows says nothing about what is being measured.
    if context.editor.placing().is_some()
        && let Some(cursor) = context.editor.cursor
        && let Some((target, nudge)) =
            pending_annotation(context, index, cursor, pixel * PICK_PIXELS, pixel)
        && let Some(value) = context.document.measured(index, target)
        && let Some(text_at) = crate::screens::annotations::push(
            &mut Vec::new(),
            sketch,
            target,
            &crate::screens::annotations::Style::driving(&state.theme),
            pixel,
            nudge,
        )
        && let Some(position) = to_screen(sketch.plane.to_world(text_at), view_projection, rect)
    {
        painter.text(
            position,
            egui::Align2::CENTER_CENTER,
            if matches!(
                target,
                DimensionTarget::Angle { .. } | DimensionTarget::AxisAngle { .. }
            ) {
                format!("{value:.1}°")
            } else {
                state.config.unit.format(value)
            },
            egui::FontId::proportional(13.0),
            egui::Color32::from_rgb(250, 220, 120),
        );
    }
}

/// The value of the dimension in hand, written right where that dimension is.
///
/// It used to sit in the title bar, an arm's length from the drawing: the eyes
/// had to leave the shape being measured to find the number belonging to it.
/// Returns true when a value was applied.
pub(crate) fn paint_dimension_field(
    ui: &mut egui::Ui,
    state: &ViewportState,
    rect: egui::Rect,
    context: &mut SketchContext<'_>,
) -> bool {
    let (Some(index), Some(target)) = (context.editor.active_sketch(), context.editor.selected())
    else {
        return false;
    };
    let pixel = state
        .camera
        .world_units_per_pixel(rect.height() * ui.ctx().pixels_per_point()) as f64;
    let Some(at) = annotation_screen_position(state, rect, context, index, target, pixel) else {
        return false;
    };

    let driven = context.document.sketches()[index]
        .dimension_of(target)
        .is_some_and(|dimension| dimension.driven);
    let angle = matches!(
        target,
        DimensionTarget::Angle { .. } | DimensionTarget::AxisAngle { .. }
    );

    let mut applied = false;
    egui::Area::new(egui::Id::new("dimension_field"))
        .fixed_pos(at + egui::vec2(16.0, 12.0))
        .order(egui::Order::Foreground)
        .show(ui.ctx(), |ui| {
            egui::Frame::popup(ui.style()).show(ui, |ui| {
                ui.horizontal(|ui| {
                    if driven {
                        // A readout cannot be edited: changing it would mean
                        // nothing, since it reports the geometry rather than
                        // deciding it.
                        let measured = context.document.measured(index, target).unwrap_or_default();
                        let suffix = if angle { "°" } else { "mm" };
                        ui.weak(format!("{measured:.2} {suffix} (lecture seule)"));
                        return;
                    }
                    let Some(editing) = context.editor.editing.as_mut() else {
                        return;
                    };
                    let field = value_field(
                        ui,
                        &mut editing.input,
                        if angle { "degrés" } else { "mm" },
                        std::mem::take(&mut editing.focus),
                    );
                    // Enter is eaten here: the field has just given the keyboard
                    // back, so the same press would otherwise also fire the
                    // shortcut bound to it — and end the sketch.
                    let submitted = field.lost_focus()
                        && ui.input_mut(|input| {
                            input.consume_key(egui::Modifiers::NONE, egui::Key::Enter)
                        });
                    applied = ui.button("✔").on_hover_text("Appliquer").clicked() || submitted;
                });
            });
        });

    let applied = applied && apply_dimension_value(context.document, context.editor, index, target);
    if applied {
        context.editor.editing = None;
    }
    applied
}

/// Where an annotation writes its value, on screen.
fn annotation_screen_position(
    state: &ViewportState,
    rect: egui::Rect,
    context: &SketchContext<'_>,
    index: usize,
    target: DimensionTarget,
    pixel: f64,
) -> Option<egui::Pos2> {
    let sketch = context.document.sketches().get(index)?;
    let mut ignored = Vec::new();
    let text_at = crate::screens::annotations::push(
        &mut ignored,
        sketch,
        target,
        &crate::screens::annotations::Style::driving(&state.theme),
        pixel,
        live_offset(context, target),
    )?;
    to_screen(
        sketch.plane.to_world(text_at),
        state
            .camera
            .view_projection(rect.width() / rect.height().max(1.0)),
        rect,
    )
}

fn face_label(face: cao_render::CubeFace) -> &'static str {
    match face {
        cao_render::CubeFace::PlusX => "DROITE",
        cao_render::CubeFace::MinusX => "GAUCHE",
        cao_render::CubeFace::PlusY => "ARRIÈRE",
        cao_render::CubeFace::MinusY => "FACE",
        cao_render::CubeFace::PlusZ => "DESSUS",
        cao_render::CubeFace::MinusZ => "DESSOUS",
    }
}

/// A scale bar: one grid step long, labelled with the length it represents.
/// It answers "how big is a square, and how fast am I zooming" at a glance.
pub(crate) fn paint_ruler(
    ui: &egui::Ui,
    state: &ViewportState,
    viewport: egui::Rect,
    scale: ViewScale,
) {
    let config = &state.config;
    let length = scale.step_in_points(ui.ctx().pixels_per_point());
    let label = config.unit.format(scale.step_millimeters);

    let tick = 5.0;
    let text_height = 16.0;
    let size = egui::vec2(length, tick + text_height);
    let origin = corner_origin(viewport, config.ruler_corner, size, config.cube_margin);

    let painter = ui.painter_at(viewport);
    let color = egui::Color32::from_gray(200);
    let stroke = egui::Stroke::new(1.5, color);
    let baseline = origin.y + size.y;
    let (left, right) = (origin.x, origin.x + length);

    painter.line_segment(
        [egui::pos2(left, baseline), egui::pos2(right, baseline)],
        stroke,
    );
    for x in [left, right] {
        painter.line_segment(
            [egui::pos2(x, baseline), egui::pos2(x, baseline - tick)],
            stroke,
        );
    }
    painter.text(
        egui::pos2((left + right) * 0.5, baseline - tick - 2.0),
        egui::Align2::CENTER_BOTTOM,
        label,
        egui::FontId::proportional(12.0),
        color,
    );
}
