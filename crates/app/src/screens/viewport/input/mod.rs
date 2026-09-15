//! What one click of a tool does: picking a plane, drawing a line, a
//! rectangle, a circle or an arc, measuring a dimension, laying down a rule,
//! dragging a point or a selection.
//!
//! What is painted from the state this leaves behind lives in
//! [`super::render`].

use cao_part::history::{Operation, PointRef};
use cao_sketch::{
    Aim, ChainAnchor, DimensionTarget, PointId, Rule, RuleIntent, SegmentId, Selection, ToolState,
    rule_intent,
};
use glam::DVec2;

use crate::screens::sketch::Tool;
use crate::wording::constraints;
use crate::wording::outcome;

use super::{PICK_PIXELS, SketchContext, ViewScale, ViewportState, plane_half_size, to_ndc};

mod arcs;
pub(crate) use arcs::{aimed as arc_aimed, arc_centre_group, arc_preview, draw_arc};

mod circles;
pub(crate) use circles::{circle_from, draw_circle};

mod constrain;
use constrain::nearest_rule_pick;

mod rectangle;
use rectangle::dimension_the_rectangle;

mod planes;
use planes::plane_under;

mod symmetric_line;
pub(crate) use symmetric_line::draw_symmetric_line_point;

mod split;
use split::split;

mod trim;
use trim::trim;

mod measure;
pub(crate) use measure::measure_preview;
use measure::{edit_dimension, measure};

mod dragging;
pub(crate) use dragging::annotation_position;
use dragging::{drag_point, nearest_annotation};

mod selecting;
use selecting::{band_select, erase};

mod mirror;
pub(crate) use mirror::{hold_is_done, mirror};

mod corner;
pub(crate) use corner::{corner, corner_held, cut as cut_the_corner};

pub(crate) fn handle_sketch_input(
    ui: &egui::Ui,
    state: &mut ViewportState,
    response: &egui::Response,
    rect: egui::Rect,
    scale: ViewScale,
    context: &mut SketchContext<'_>,
) -> bool {
    let Some(pointer) = response.hover_pos() else {
        context.editor.hovered_plane = None;
        context.editor.cursor = None;
        return false;
    };
    // The camera reasons in f32, the drawing in f64: the ray crosses over here,
    // once, rather than at every call that follows.
    let (origin, direction) = state
        .camera
        .ray(to_ndc(pointer, rect), rect.width() / rect.height());
    let (origin, direction) = (origin.as_dvec3(), direction.as_dvec3());

    if context.editor.is_choosing_plane() {
        context.editor.hovered_plane = plane_under(state, context, origin, direction);

        if response.clicked()
            && let Some(choice) = context.editor.hovered_plane
        {
            let plane = choice.plane();
            context.document.apply(Operation::CreateSketch { plane });
            let sketch = context.document.sketches().len() - 1;
            context.editor.begin_editing(sketch, plane);
            // A fresh sketch has nothing to frame yet, so we show a patch of
            // plane big enough to draw in, centred where the click landed. On a
            // face of the part that matters: the plane's own origin is the world
            // origin projected onto it, which can be nowhere near the face.
            let center = plane
                .ray_intersection(origin, direction)
                .map(|local| plane.to_world(local))
                .unwrap_or(plane.origin);
            state.look_at_plane(plane, center, DEFAULT_SKETCH_RADIUS);
            return true;
        }
        return false;
    }

    let Some(index) = context.editor.active_sketch() else {
        return false;
    };
    let Some(plane) = context
        .document
        .sketches()
        .get(index)
        .map(|sketch| sketch.plane)
    else {
        return false;
    };
    let Some(cursor) = plane.ray_intersection(origin, direction) else {
        return false;
    };

    // Snapping to an existing point is what lets a contour actually close.
    let snap = scale.world_size_of(PICK_PIXELS);
    let magnets = scale.snapping(&state.config);
    let (cursor, snapped_to) = context.document.sketches()[index].magnetise(cursor, &magnets);
    context.editor.snap = snapped_to;

    context.editor.cursor = Some(cursor);
    context.editor.hovered_point = context.document.sketches()[index].nearest_point(cursor, snap);
    // Previewing a click's target only where a click takes hold of existing
    // geometry — a drawing tool placing a fresh point keeps its plain cursor.
    context.editor.hovered = matches!(context.editor.tool, Tool::Select | Tool::Trim)
        .then(|| pick(context, index, cursor, snap, scale.units_per_pixel))
        .flatten();

    // Worked out once a frame and shown as the preview, so that what is drawn
    // on screen is exactly what a click would record.
    context.editor.aimed = match context.editor.tool_state {
        ToolState::Line { anchor, previous } => Some(aim(context, index, anchor, previous, cursor)),
        _ => None,
    };

    // Dragging a point is a gesture, not a click, so it comes before the
    // click-based tools.
    if context.editor.tool == Tool::Select {
        // What is grabbed is decided where the button went down, not where the
        // cursor is when egui calls it a drag: by then it has already travelled
        // the few pixels of the drag threshold, which was enough to miss the
        // very point being aimed at.
        let pressed = ui
            .input(|input| input.pointer.press_origin())
            .and_then(|position| {
                let (origin, direction) = state
                    .camera
                    .ray(to_ndc(position, rect), rect.width() / rect.height());
                plane.ray_intersection(origin.as_dvec3(), direction.as_dvec3())
            })
            .map(|position| {
                context.document.sketches()[index]
                    .magnetise(position, &magnets)
                    .0
            })
            .unwrap_or(cursor);

        let adding = ui.input(|input| input.modifiers.command || input.modifiers.shift);
        if response.clicked() {
            let picked = pick(context, index, cursor, snap, scale.units_per_pixel);
            match (picked, adding) {
                // Holding the modifier gathers things up one by one, which is
                // how one picks out three traits that no box can enclose alone.
                (Some(what), true) => context.editor.toggle(what),
                (Some(what), false) => {
                    if let ToolState::Select(select) = &mut context.editor.tool_state {
                        select.held = vec![what];
                    }
                }
                (None, false) => {
                    if let ToolState::Select(select) = &mut context.editor.tool_state {
                        select.held.clear();
                    }
                }
                (None, true) => {}
            }
            // A dimension picked with the selection tool opens its value too:
            // reaching for the dimension tool again to change a number one is
            // already pointing at is a step for nothing.
            match picked {
                Some(Selection::Dimension(target)) if !adding => {
                    edit_dimension(context, index, target)
                }
                _ => context.editor.select(None, None),
            }
        }

        if !context.editor.selection().is_empty()
            && ui.input(|input| {
                input.key_pressed(egui::Key::Delete) || input.key_pressed(egui::Key::Backspace)
            })
        {
            let held = match &mut context.editor.tool_state {
                ToolState::Select(select) => std::mem::take(&mut select.held),
                _ => Vec::new(),
            };
            return erase(context, index, &held);
        }

        // A drag that grabbed nothing pulls a box instead, the way a desktop
        // does. Who grabs is settled at the start of the gesture and holds for
        // the whole of it: deciding again every frame would swap gestures
        // mid-drag, as soon as the cursor happened to pass over a point.
        if response.drag_started() {
            let changed = drag_point(
                context,
                index,
                cursor,
                pressed,
                response,
                snap,
                scale.units_per_pixel,
            );
            let nothing_grabbed = matches!(
                &context.editor.tool_state,
                ToolState::Select(select)
                    if select.dragged_point.is_none()
                        && select.dragged_dimension.is_none()
                        && select.dragged_group.is_empty()
            );
            if nothing_grabbed && let ToolState::Select(select) = &mut context.editor.tool_state {
                select.band = Some((pressed, cursor));
            }
            return changed;
        }
        let has_band = matches!(&context.editor.tool_state, ToolState::Select(select) if select.band.is_some());
        if has_band {
            return band_select(
                context,
                index,
                cursor,
                response,
                adding,
                scale.units_per_pixel,
            );
        }

        return drag_point(
            context,
            index,
            cursor,
            pressed,
            response,
            snap,
            scale.units_per_pixel,
        );
    }

    if !response.clicked() {
        return false;
    }

    match context.editor.tool {
        Tool::Line => draw_line_point(context, index, cursor, snap, scale.units_per_pixel),
        Tool::LineSymmetric => {
            draw_symmetric_line_point(context, index, cursor, snap, scale.units_per_pixel)
        }
        Tool::Point => {
            context.document.apply(Operation::AddPoint {
                sketch: index,
                position: cursor,
            });
            true
        }
        Tool::Rectangle => {
            let corner = rectangle_corner(context, cursor);
            two_click_shape(context, index, corner, snap, scale.units_per_pixel)
        }
        Tool::Circle => draw_circle(context, index, cursor, snap, scale.units_per_pixel),
        Tool::Arc => draw_arc(context, index, cursor, snap, scale.units_per_pixel),
        Tool::Dimension => measure(context, index, cursor, snap, scale.units_per_pixel),
        Tool::Trim => trim(context, index, cursor, snap),
        Tool::Split => split(context, index, cursor, snap),
        Tool::Chamfer | Tool::Fillet => corner(context, index, cursor, snap),
        Tool::Mirror => mirror(context, index, cursor, snap, scale.units_per_pixel),
        Tool::Constrain(rule) => constrain(context, index, rule, cursor, snap),
        Tool::Select | Tool::None => false,
    }
}

/// What the cursor is over. The order and the reaches are the drawing's own
/// rule; all this adds is where each annotation was drawn.
pub(super) fn pick(
    context: &SketchContext<'_>,
    index: usize,
    cursor: DVec2,
    snap: f64,
    pixel: f64,
) -> Option<Selection> {
    let sketch = context.document.sketches().get(index)?;
    sketch.pick(
        cursor,
        snap,
        crate::screens::annotations::metrics(pixel, DVec2::ZERO),
    )
}

/// Points the constraint tool at something, and lays the rule down as soon as
/// it has been shown enough.
///
/// The order of the clicks does not matter: a point and a trait make the same
/// coincidence whichever comes first. What matters is what was clicked, so the
/// rule is built from the kinds gathered rather than from their order.
fn constrain(
    context: &mut SketchContext<'_>,
    index: usize,
    rule: Rule,
    cursor: DVec2,
    snap: f64,
) -> bool {
    let Some(sketch) = context.document.sketches().get(index) else {
        return false;
    };
    let Some(picked) = nearest_rule_pick(sketch, cursor, snap, rule) else {
        context.editor.message = Some(context.lang.t("sketch.nothing_to_constrain"));
        return false;
    };
    let mut picks = match &context.editor.tool_state {
        ToolState::Constrain { picks } => picks.clone(),
        _ => Vec::new(),
    };
    if picks.contains(&picked) {
        return false;
    }
    picks.push(picked);
    if picks.len() < rule.arity() {
        context.editor.tool_state = ToolState::Constrain { picks };
        context.editor.message = Some(constraints::rule_asks_for(context.lang, rule));
        return false;
    }

    context.editor.tool_state = ToolState::None;
    let Some(intent) = rule_intent(rule, &picks, sketch) else {
        let asks = constraints::rule_asks_for(context.lang, rule);
        context.editor.message = Some(
            context
                .lang
                .t_with("sketch.rule_refused", &[("asks", &asks)]),
        );
        return false;
    };
    if let RuleIntent::Constrain(constraint) = intent
        && sketch.constraints().contains(&constraint.normalised())
    {
        // The drawing already carries it; recording the step again would fill
        // the history with entries that change nothing.
        context.editor.message = Some(constraints::already_there_label(context.lang, rule));
        return false;
    }
    let operation = match intent {
        RuleIntent::Constrain(constraint) => Operation::Constrain {
            sketch: index,
            constraint,
        },
        RuleIntent::Merge { kept, dropped } => Operation::MergePoints {
            sketch: index,
            kept,
            dropped,
        },
    };
    context.document.apply(operation);
    context.editor.message = Some(constraints::rule_asks_for(context.lang, rule));
    true
}

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
    // The innermost area wins: inside a shape drawn within another, the click
    // means the small one, not the one it sits in.
    let Some(under) = regions
        .iter()
        .enumerate()
        .filter(|(_, region)| region.contains(cursor))
        .max_by_key(|(_, region)| region.depth)
        .map(|(index, _)| index)
    else {
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

/// Moving a point by hand. The drawing settles around it afterwards, so the
/// values already given stay true.
#[allow(clippy::too_many_arguments)]
/// A point already there, or a new one where the cursor is.
pub(super) fn point_ref_at(
    context: &SketchContext<'_>,
    index: usize,
    position: DVec2,
    snap: f64,
) -> PointRef {
    match context.document.sketches()[index].nearest_point(position, snap) {
        Some(point) => PointRef::Existing(point),
        None => PointRef::New(position),
    }
}

/// One click of the rectangle tool: the first remembers a corner, the second
/// draws it opposite.
pub(crate) fn two_click_shape(
    context: &mut SketchContext<'_>,
    index: usize,
    cursor: DVec2,
    snap: f64,
    pixel: f64,
) -> bool {
    let ToolState::Rectangle { start } = context.editor.tool_state else {
        context.editor.tool_state = ToolState::Rectangle { start: cursor };
        context.editor.live.open();
        return false;
    };
    // A shape with no extent is a stray click, not a drawing.
    if start.distance(cursor) < 1e-6 {
        return false;
    }
    context.editor.tool_state = ToolState::None;

    // Corners reuse a point already drawn when one is under the cursor, so
    // shapes hang together instead of stacking points on top of each other.
    // Nothing forces the user to place those points first.
    let corner = point_ref_at(context, index, start, snap);
    let opposite = point_ref_at(context, index, cursor, snap);
    context.document.apply(Operation::AddRectangle {
        sketch: index,
        corner,
        opposite,
        construction: context.editor.construction,
    });
    dimension_the_rectangle(context, index, pixel);
    context.editor.live.clear();
    true
}

pub(crate) fn rectangle_corner(context: &SketchContext<'_>, cursor: DVec2) -> DVec2 {
    let ToolState::Rectangle { start } = context.editor.tool_state else {
        return cursor;
    };
    cao_sketch::rectangle_corner(
        start,
        cursor,
        context.editor.live.locked(),
        context.document.scale(),
    )
}

/// The second half a dimension in hand can still take: another segment makes it
/// an angle, a point makes it a distance to a line, an axis a direction.
///
/// Without this, clicking a segment could only ever mean its length, and an
/// angle between two traits had to be asked for through the Mesurer row — which
/// is exactly what one expects the smart dimension to do on its own.
pub(crate) fn refine(
    context: &SketchContext<'_>,
    index: usize,
    target: DimensionTarget,
    cursor: DVec2,
    snap: f64,
) -> Option<DimensionTarget> {
    context
        .document
        .sketches()
        .get(index)?
        .refine(target, cursor, snap)
}

/// One click of the line tool. The first click only remembers where the chain
/// starts; the second turns the pair into a segment in the history.
pub(crate) fn draw_line_point(
    context: &mut SketchContext<'_>,
    index: usize,
    cursor: DVec2,
    snap: f64,
    pixel: f64,
) -> bool {
    let sketch = &context.document.sketches()[index];
    let (anchor, previous) = match context.editor.tool_state {
        ToolState::Line { anchor, previous } => (Some(anchor), previous),
        _ => (None, None),
    };
    let locked = context.editor.live.locked();
    let scale = context.document.scale();

    match cao_sketch::chain_click(sketch, anchor, previous, cursor, snap, locked, scale) {
        cao_sketch::ChainClick::Started(anchor) => {
            context.editor.tool_state = ToolState::Line {
                anchor,
                previous: None,
            };
            context.editor.live.open();
            false
        }
        cao_sketch::ChainClick::Ignored => false,
        cao_sketch::ChainClick::Drew { start, end, aimed } => {
            let point_ref = |anchor: ChainAnchor| match anchor {
                ChainAnchor::Point(id) => PointRef::Existing(id),
                ChainAnchor::Pending(position) => PointRef::New(position),
            };
            context.document.apply(Operation::AddSegment {
                sketch: index,
                start: point_ref(start),
                end: point_ref(end),
                construction: context.editor.construction,
            });

            // The far end of the segment just drawn becomes the next anchor.
            // A point created by the operation is the last one in the sketch.
            let sketch = &context.document.sketches()[index];
            let drawn = SegmentId(sketch.segments().len().saturating_sub(1));
            let next_anchor = ChainAnchor::Point(match end {
                ChainAnchor::Point(id) => id,
                ChainAnchor::Pending(_) => PointId(sketch.points().len().saturating_sub(1)),
            });

            dimension_the_line(context, index, drawn, aimed, pixel);
            context.editor.tool_state = ToolState::Line {
                anchor: next_anchor,
                previous: Some(drawn),
            };
            context.editor.live.open();
            true
        }
    }
}

/// Applies to the cursor everything the user has already decided. The rules are
/// the drawing's; what this adds is the state the tool is holding.
fn aim(
    context: &SketchContext<'_>,
    index: usize,
    anchor: ChainAnchor,
    previous: Option<SegmentId>,
    cursor: DVec2,
) -> Aim {
    let nowhere = Aim {
        position: cursor,
        square_with: None,
    };
    let Some(sketch) = context.document.sketches().get(index) else {
        return nowhere;
    };
    sketch.aim(
        anchor,
        previous,
        cursor,
        context.editor.live.locked(),
        context.document.scale(),
    )
}

/// Places on the line just drawn whatever the user typed, and the right angle
/// they aimed at.
///
/// A value that would say nothing is left out: the drawing already holds it,
/// and a second copy could only be redundant.
fn dimension_the_line(
    context: &mut SketchContext<'_>,
    index: usize,
    segment: SegmentId,
    aimed: Aim,
    pixel: f64,
) {
    let locked = context.editor.live.locked();
    let scale = context.document.scale();
    let wanted = cao_sketch::line_dimensions(
        &context.document.sketches()[index],
        segment,
        locked,
        aimed.square_with,
        scale,
    );

    for (target, value) in wanted {
        // Pinned down where it is drawn, in sketch units: left to stand off by
        // a distance in pixels, an annotation slides back over the drawing as
        // soon as one zooms out.
        let applied = context.document.apply(Operation::SetDimension {
            sketch: index,
            target,
            value,
            placement: annotation_position(context, index, target, pixel)
                .map(|placement| placement.offset),
        });
        if let Some(message) = outcome::message(context.lang, applied) {
            context.editor.message = Some(message);
        }
    }
}

/// How much of the plane to show when a sketch has no geometry to frame yet.
pub const DEFAULT_SKETCH_RADIUS: f64 = 100.0;
