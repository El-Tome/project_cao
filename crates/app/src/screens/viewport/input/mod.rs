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
        Tool::Constrain(rule) => constrain(context, index, rule, cursor, snap),
        Tool::Select | Tool::None => false,
    }
}

/// What the cursor is over. The order and the reaches are the drawing's own
/// rule; all this adds is where each annotation was drawn.
fn pick(
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

/// Pulls a box across the drawing and takes everything inside it.
///
/// Whole elements only: a trait counts when both its ends are in the box. Half
/// a trait cannot be deleted, so letting the box claim it would say something
/// the drawing cannot do.
fn band_select(
    context: &mut SketchContext<'_>,
    index: usize,
    to: DVec2,
    response: &egui::Response,
    adding: bool,
    pixel: f64,
) -> bool {
    // Where the box started is kept from the frame the drag began: egui lets go
    // of the press position on the very frame the button comes up, which is the
    // frame that matters here.
    let Some((from, _)) = context.editor.select_state().and_then(|state| state.band) else {
        return false;
    };
    if let Some(state) = context.editor.select_state() {
        state.band = Some((from, to));
    }
    if !response.drag_stopped() {
        return false;
    }
    if let Some(state) = context.editor.select_state() {
        state.band = None;
    }

    let Some(sketch) = context.document.sketches().get(index) else {
        return false;
    };
    let caught = sketch.inside_band(
        from,
        to,
        crate::screens::annotations::metrics(pixel, DVec2::ZERO),
    );

    if !adding && let Some(state) = context.editor.select_state() {
        state.held.clear();
    }
    for what in caught {
        if !context.editor.is_selected(what)
            && let Some(state) = context.editor.select_state()
        {
            state.held.push(what);
        }
    }
    false
}

/// Deletes what the selection tool is holding, in one step.
fn erase(context: &mut SketchContext<'_>, index: usize, selection: &[Selection]) -> bool {
    if selection.is_empty() {
        return false;
    }
    let mut elements = Vec::new();
    let mut dimensions = Vec::new();
    let mut constraints = Vec::new();
    for held in selection {
        match held {
            Selection::Element(element) => elements.push(*element),
            Selection::Dimension(target) => dimensions.push(*target),
            Selection::Rule(constraint) => constraints.push(*constraint),
        }
    }
    context.document.apply(Operation::EraseMany {
        sketch: index,
        elements,
        dimensions,
        constraints,
    });
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
fn drag_point(
    context: &mut SketchContext<'_>,
    index: usize,
    cursor: DVec2,
    pressed: DVec2,
    response: &egui::Response,
    snap: f64,
    pixel: f64,
) -> bool {
    let sketch = &context.document.sketches()[index];
    if response.drag_started() {
        // Pressing on something already picked moves the whole selection, the
        // way a desktop moves a group of icons. It comes first: what is held is
        // a deliberate choice, and it would be odd for the drag to take one
        // corner out of it instead.
        let dragged_group = grabbed_group(context, index, pressed, snap, pixel);
        let group_grabbed = !dragged_group.is_empty();
        if let Some(state) = context.editor.select_state() {
            state.dragged_group = dragged_group;
        }
        if group_grabbed {
            if let Some(state) = context.editor.select_state() {
                state.drag_origin = Some(pressed);
            }
            return false;
        }

        // A settled point cannot be dragged, and the centre of an arc carries
        // its two ends along, the way a circle's centre carries its rim.
        let settled = sketch.settled_points(context.document.scale());
        let dragged_point = sketch
            .nearest_point(pressed, snap)
            .filter(|point| !sketch.is_origin(*point))
            .filter(|point| !settled.get(point.0).copied().unwrap_or(false));
        let arc_group = dragged_point.and_then(|point| arc_centre_group(sketch, point));
        if let Some(state) = context.editor.select_state() {
            match arc_group {
                Some(group) => {
                    state.dragged_group = group;
                    state.drag_origin = Some(pressed);
                }
                None => state.dragged_point = dragged_point,
            }
        }

        if dragged_point.is_none() {
            let dragged_dimension = nearest_annotation(context, index, pressed, snap * 1.5, pixel);
            if let Some(state) = context.editor.select_state() {
                state.dragged_dimension = dragged_dimension;
                state.drag_origin = Some(pressed);
            }
        }
    }

    let group_pending = context
        .editor
        .select_state()
        .is_some_and(|state| !state.dragged_group.is_empty());
    if group_pending {
        return drag_group(context, index, cursor, response);
    }
    let dimension_pending = context
        .editor
        .select_state()
        .and_then(|state| state.dragged_dimension);
    if let Some(target) = dimension_pending {
        return drag_annotation(context, index, target, cursor, response, pixel);
    }

    let Some(point) = context
        .editor
        .select_state()
        .and_then(|state| state.dragged_point)
    else {
        return false;
    };

    // While the drag lasts the point is only *shown* at the cursor; the move is
    // recorded once, on release. Recording every frame buried the history under
    // hundreds of entries that all said the same thing.
    //
    // What is shown, though, is the whole drawing settled as if the point had
    // been let go here — the values already given pull the rest of the shape
    // along, and seeing only the point move told nothing of where it was
    // heading.
    if !response.drag_stopped() {
        let mut settling = context.document.sketches()[index].clone();
        settling.settle_around(point, cursor, context.document.scale());
        if let Some(state) = context.editor.select_state() {
            state.drag_position = Some(cursor);
            state.drag_preview = Some(settling);
        }
        return false;
    }

    if let Some(state) = context.editor.select_state() {
        state.dragged_point = None;
        state.drag_position = None;
        state.drag_preview = None;
    }
    context.document.apply(Operation::MovePoint {
        sketch: index,
        point,
        position: cursor,
    });

    // Two ends laid on top of each other are one corner, not two. The decision
    // is taken here, at the drop, and recorded: how close is close enough
    // depends on the zoom, so re-deriving it on replay could join a different
    // pair, or none.
    let sketch = &context.document.sketches()[index];
    if let Some(other) = sketch
        .nearest_point(cursor, snap)
        .filter(|other| *other != point)
    {
        context.document.apply(Operation::MergePoints {
            sketch: index,
            kept: other,
            dropped: point,
        });
    }
    true
}

/// The points a drag would carry along, when it starts on something the
/// selection tool is already holding.
///
/// Empty when the press lands anywhere else: a drag beside a selection is a
/// new box, not a move of the old one.
fn grabbed_group(
    context: &SketchContext<'_>,
    index: usize,
    pressed: DVec2,
    snap: f64,
    pixel: f64,
) -> Vec<PointId> {
    let Some(what) = pick(context, index, pressed, snap, pixel) else {
        return Vec::new();
    };
    if !context.editor.is_selected(what) {
        return Vec::new();
    }
    let Some(sketch) = context.document.sketches().get(index) else {
        return Vec::new();
    };
    sketch.points_of(context.editor.selection())
}

/// Moving a whole selection at once. Same rule as a point: shown following the
/// cursor, written once on release, as a single entry in the history.
fn drag_group(
    context: &mut SketchContext<'_>,
    index: usize,
    cursor: DVec2,
    response: &egui::Response,
) -> bool {
    let Some(origin) = context
        .editor
        .select_state()
        .and_then(|state| state.drag_origin)
    else {
        return false;
    };
    let travelled = cursor - origin;
    let points = context
        .editor
        .select_state()
        .map(|state| state.dragged_group.clone())
        .unwrap_or_default();

    if !response.drag_stopped() {
        let mut settling = context.document.sketches()[index].clone();
        let dropped: Vec<(PointId, DVec2)> = points
            .iter()
            .filter_map(|point| {
                settling
                    .points()
                    .get(point.0)
                    .map(|place| (*point, *place + travelled))
            })
            .collect();
        settling.settle_around_all(&dropped, context.document.scale());
        if let Some(state) = context.editor.select_state() {
            state.drag_position = Some(cursor);
            state.drag_preview = Some(settling);
        }
        return false;
    }

    if let Some(state) = context.editor.select_state() {
        state.dragged_group.clear();
        state.drag_origin = None;
        state.drag_position = None;
        state.drag_preview = None;
    }
    if travelled.length() < 1e-9 {
        return false;
    }
    context.document.apply(Operation::MoveMany {
        sketch: index,
        points,
        by: travelled,
    });
    true
}

/// Moving an annotation out of the way. Same rule as a point: shown following
/// the cursor, written once on release.
fn drag_annotation(
    context: &mut SketchContext<'_>,
    index: usize,
    target: DimensionTarget,
    cursor: DVec2,
    response: &egui::Response,
    pixel: f64,
) -> bool {
    let Some(origin) = context
        .editor
        .select_state()
        .and_then(|state| state.drag_origin)
    else {
        return false;
    };
    let travelled = cursor - origin;

    if !response.drag_stopped() {
        if let Some(state) = context.editor.select_state() {
            state.drag_position = Some(cursor);
        }
        return false;
    }

    // Where the annotation sits right now, whether that was recorded before or
    // is still the standing-off distance it was drawn with.
    let previous = annotation_position(context, index, target, pixel)
        .map(|placement| placement.offset)
        .unwrap_or_default();

    if let Some(state) = context.editor.select_state() {
        state.dragged_dimension = None;
        state.drag_origin = None;
        state.drag_position = None;
    }
    context.document.apply(Operation::MoveDimension {
        sketch: index,
        target,
        offset: previous + travelled,
    });
    true
}

/// Which annotation sits under the cursor. Asked straight of the sketch: it
/// is the one that knows where each of its dimensions is drawn.
pub(super) fn nearest_annotation(
    context: &SketchContext<'_>,
    index: usize,
    cursor: DVec2,
    tolerance: f64,
    pixel: f64,
) -> Option<DimensionTarget> {
    context.document.sketches().get(index)?.nearest_dimension(
        cursor,
        tolerance,
        crate::screens::annotations::metrics(pixel, DVec2::ZERO),
    )
}

/// Where an annotation's value sits right now, and the offset that would
/// record it there.
///
/// Asked straight of `sketch.place`: no colour needed just to find a
/// position, so no theme has to be made up to get one.
pub(crate) fn annotation_position(
    context: &SketchContext<'_>,
    index: usize,
    target: DimensionTarget,
    pixel: f64,
) -> Option<cao_sketch::Placement> {
    context.document.sketches().get(index)?.place(
        target,
        crate::screens::annotations::metrics(pixel, DVec2::ZERO),
    )
}

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
