//! What a drag takes hold of and what it moves: a point, a whole selection,
//! or the annotation of a dimension.

use cao_part::Operation;
use cao_prefs::Modifier;
use cao_sketch::{DimensionTarget, PointId};
use glam::DVec2;

use crate::screens::viewport::SketchContext;

use super::{arc_centre_group, dropped_on, pick};

/// What a drag needs beyond where the cursor is: how far a click reaches, what
/// a pixel is worth in the drawing, and whether the key that pulls a point off
/// what holds it is down.
#[derive(Clone, Copy)]
pub(super) struct Gesture {
    pub(super) snap: f64,
    pub(super) pixel: f64,
    pub(super) letting_go: bool,
}

/// Whether the key that pulls a point off what holds it is down.
pub(super) fn letting_go(ui: &egui::Ui, modifier: Modifier) -> bool {
    ui.input(|input| match modifier {
        Modifier::Command => input.modifiers.command,
        Modifier::Shift => input.modifiers.shift,
        Modifier::Alt => input.modifiers.alt,
    })
}

pub(super) fn drag_point(
    context: &mut SketchContext<'_>,
    index: usize,
    cursor: DVec2,
    pressed: DVec2,
    response: &egui::Response,
    gesture: Gesture,
) -> bool {
    let (snap, pixel) = (gesture.snap, gesture.pixel);
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
        //
        // A point held where two things cross reads as settled, and the key
        // that pulls it off is the one way to move it: without this it could
        // only ever be freed by deleting the marks that hold it.
        let settled = sketch.settled_points(context.document.scale());
        let dragged_point = sketch
            .nearest_point(pressed, snap)
            .filter(|point| !sketch.is_origin(*point))
            .filter(|point| {
                let holds = sketch.holds_on(*point);
                let stuck = settled.get(point.0).copied().unwrap_or(false) || holds.len() > 1;
                !stuck || (gesture.letting_go && !holds.is_empty())
            });
        let arc_group = dragged_point.and_then(|point| arc_centre_group(sketch, point));
        if let Some(state) = context.editor.select_state() {
            // Read where the gesture starts and kept for the whole of it, as
            // what is grabbed already is: letting the key decide again every
            // frame would have the drop read a key released a frame early as
            // a drag that never asked for anything.
            state.letting_go = gesture.letting_go;
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
    let letting_go = context
        .editor
        .select_state()
        .is_some_and(|state| state.letting_go);

    // Where the point actually goes: along whatever holds it, unless the key
    // that pulls it off is down. Held where two things cross, it does not go
    // anywhere at all — which is the whole of what a crossing means.
    let landing = match letting_go {
        true => cursor,
        false => context.document.sketches()[index].slide(point, cursor),
    };

    if !response.drag_stopped() {
        let mut settling = context.document.sketches()[index].clone();
        if letting_go {
            settling.let_go(point);
        }
        settling.settle_around(point, landing, context.document.scale());
        if let Some(state) = context.editor.select_state() {
            state.drag_position = Some(landing);
            state.drag_preview = Some(settling);
        }
        return false;
    }

    if let Some(state) = context.editor.select_state() {
        state.dragged_point = None;
        state.drag_position = None;
        state.drag_preview = None;
        state.letting_go = false;
    }
    // Two ends laid on top of each other are one corner, not two. The decision
    // is taken here, at the drop, and recorded: how close is close enough
    // depends on the zoom, so re-deriving it on replay could join a different
    // pair, or none.
    //
    // It rides in the same step as the drag, because dropping a corner on
    // another is one gesture and one undo has to take the whole of it back.
    let sketch = &context.document.sketches()[index];
    let merged_into = sketch
        .nearest_point(landing, snap)
        .filter(|other| *other != point);
    // What the drop leaves the point held by. A point dropped on a trait is
    // held there exactly as one born on it is — but only a point nothing held
    // already: one sliding along its own trait would otherwise catch on the
    // first crossing it went over. Joining another point holds nothing: that
    // is a merge, and it is the other point that stands there afterwards.
    let free = sketch.holds_on(point).is_empty();
    let on = match letting_go || merged_into.is_some() || !free {
        true => Vec::new(),
        false => dropped_on(sketch, point, landing),
    };
    context.document.apply(Operation::MovePoint {
        sketch: index,
        point,
        position: landing,
        merged_into,
        on,
        let_go: letting_go,
    });
    true
}

/// The points a drag would carry along, when it starts on something the
/// selection tool is already holding.
///
/// Empty when the press lands anywhere else: a drag beside a selection is a
/// new box, not a move of the old one.
pub(super) fn grabbed_group(
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
pub(super) fn drag_group(
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
pub(super) fn drag_annotation(
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
