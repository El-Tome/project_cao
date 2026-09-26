//! What a drag takes hold of and what it moves: a point, a whole selection,
//! or the annotation of a dimension.

use cao_part::Operation;
use cao_prefs::Modifier;
use cao_sketch::{PointId, Pulled, Sketch, SnapSettings};
use glam::DVec2;

use crate::screens::SketchContext;

use super::annotation_drag::{drag_annotation, nearest_annotation};
use super::selection_drag::{drag_group, grabbed_group};
use super::sides::drag_side;
use super::{drag_curve, dropped_on};

/// What a drag needs beyond where the cursor is: how far a click reaches, what
/// a pixel is worth in the drawing, whether the key that pulls a point off
/// what holds it is down, and whether one that gathers a selection is.
///
/// And the hand's own places, before any magnet: a side or a curve pulled is
/// measured from where it was pressed, and the magnets would put the press,
/// and the first few pixels of the pull, back on the very curve pressed on.
/// The grid is there to pull what turns onto it.
#[derive(Clone, Copy)]
pub(super) struct Gesture {
    pub(super) snap: f64,
    pub(super) pixel: f64,
    pub(super) letting_go: bool,
    pub(super) adding: bool,
    pub(super) raw_cursor: DVec2,
    pub(super) raw_pressed: DVec2,
    pub(super) magnets: SnapSettings,
    /// Whether the camera holds the gesture: an orbit or a pan started on
    /// the drawing moves the view, and takes hold of nothing in it.
    pub(super) navigating: bool,
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
    // Whatever an earlier gesture left behind is dropped as soon as no drag is
    // under way — one let go of over the cube or outside the canvas never saw
    // its release, and would otherwise go on following the hand as a ghost —
    // and again before a new one decides what it takes.
    let still = !response.dragged() && !response.drag_stopped();
    if (still || response.drag_started())
        && let Some(state) = context.editor.select_state()
    {
        *state = cao_sketch::SelectState {
            held: std::mem::take(&mut state.held),
            ..Default::default()
        };
    }
    if still {
        return false;
    }
    if response.drag_started() {
        if gesture.navigating || !response.drag_started_by(egui::PointerButton::Primary) {
            return false;
        }
        // Pressing on something already picked moves the whole selection, the
        // way a desktop moves a group of icons. It comes first: what is held is
        // a deliberate choice, and it would be odd for the drag to take one
        // corner out of it instead. A lone corner picked and pressed on is
        // dragged as the corner it is, stretching its shape like any other.
        let mut dragged_group = grabbed_group(context, index, pressed, snap, pixel);
        if dragged_group.len() == 1
            && sketch.nearest_point(pressed, snap) == dragged_group.first().copied()
        {
            dragged_group.clear();
        }
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

        // A settled point cannot be dragged, and the centre of an arc or of an
        // ellipse carries the curve along, the way a circle's centre carries
        // its rim.
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
        let arc_group = dragged_point.and_then(|point| centre_group(sketch, point));
        let carried = arc_group.is_some();
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
        // What the drag may do is read once, here, off the drawing as the
        // press found it — the drawing the drop will be replayed on.
        if let Some(point) = dragged_point.filter(|_| !carried) {
            let mut reading = sketch.clone();
            if gesture.letting_go {
                reading.let_go(point);
            }
            let pull = reading.pull(point, context.document.scale());
            if let Some(state) = context.editor.select_state() {
                state.pull = Some(pull);
            }
        }

        if dragged_point.is_none() {
            let dragged_dimension = nearest_annotation(context, index, pressed, snap * 1.5, pixel);
            // A press that took hold of no point and no annotation may still
            // have landed on a side or a curve, whichever is nearer: pulled
            // across, a side travels and a curve is drawn to another size;
            // slid along, either turns its shape. A press gathering a
            // selection still pulls a box.
            let pulled = match (dragged_dimension.is_none(), gesture.adding) {
                (true, false) => sketch.pulled_at(pressed, snap, context.document.scale()),
                (true, true) => sketch.curve_at(pressed, snap).map(Pulled::Curve),
                (false, _) => None,
            };
            if let Some(state) = context.editor.select_state() {
                state.dragged_dimension = dragged_dimension;
                state.dragged_curve = match pulled {
                    Some(Pulled::Curve(curve)) => Some(curve),
                    _ => None,
                };
                state.dragged_side = match pulled {
                    Some(Pulled::Side(side)) => Some(side),
                    _ => None,
                };
                state.drag_origin = Some(pressed);
                state.grabbed_at = Some(gesture.raw_pressed);
            }
        }
        // A drag that grabbed nothing pulls a box instead, the way a desktop
        // does.
        if let Some(state) = context.editor.select_state()
            && state.grabbed_nothing()
        {
            state.band = Some((pressed, cursor));
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
    let curve_pending = context
        .editor
        .select_state()
        .and_then(|state| state.dragged_curve);
    if let Some(curve) = curve_pending {
        return drag_curve(context, index, curve, cursor, response, gesture);
    }
    let side_pending = context
        .editor
        .select_state()
        .and_then(|state| state.dragged_side);
    if let Some(side) = side_pending {
        return drag_side(context, index, side, response, gesture);
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
    // anywhere at all — which is the whole of what a crossing means. A shape
    // that can only turn is turned towards the hand itself, its end pulled
    // onto the grid, rather than towards the cursor the magnets moved.
    let scale = context.document.scale();
    let Some(pull) = context
        .editor
        .select_state()
        .and_then(|state| state.pull.clone())
    else {
        return false;
    };
    if pull.turns() {
        context.editor.snap = None;
    }
    let sketch = &context.document.sketches()[index];
    let landing = pull.landing(
        sketch,
        gesture.raw_cursor,
        cursor,
        letting_go,
        &gesture.magnets,
    );
    let mut settling = sketch.clone();
    if letting_go {
        settling.let_go(point);
    }
    settling.settle_pulled(&pull, landing, scale);

    if !response.drag_stopped() {
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
        state.pull = None;
    }
    // A drag the drawing could not take at all writes nothing: a step that
    // changes nothing would be what the next undo takes back.
    let sketch = &context.document.sketches()[index];
    // Two ends laid on top of each other are one corner, not two. The decision
    // is taken here, at the drop, and recorded: how close is close enough
    // depends on the zoom, so re-deriving it on replay could join a different
    // pair, or none. A point the shape could not take all the way is joined
    // to nothing, and held by nothing: what the hand is over is not where the
    // point is.
    //
    // It rides in the same step as the drag, because dropping a corner on
    // another is one gesture and one undo has to take the whole of it back.
    let arrived = pull.arrived(&settling, landing);
    let merged_into = pull.joined_to(sketch, &settling, landing, snap);
    // What the drop leaves the point held by. A point dropped on a trait is
    // held there exactly as one born on it is — but only a point nothing held
    // already: one sliding along its own trait would otherwise catch on the
    // first crossing it went over. Joining another point holds nothing: that
    // is a merge, and it is the other point that stands there afterwards.
    // What it is held on is read off the drawing the drag settled, the one
    // shown and the one the replay rebuilds: a trait the settling moved away
    // no longer runs through the place, and a hold on it would not be met.
    let free = sketch.holds_on(point).is_empty();
    let on = match letting_go || merged_into.is_some() || !free || !arrived {
        true => Vec::new(),
        false => dropped_on(&settling, point, landing),
    };
    // A drag that changed nothing — the drawing refused it, and it neither
    // joined, held nor let go of anything — writes nothing: a step that changes
    // nothing would be what the next undo takes back.
    if !letting_go && merged_into.is_none() && on.is_empty() && !reshaped(sketch, &settling) {
        return false;
    }
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

/// Whether a settling moved anything the drawing had: a point, or the size
/// of a circle.
pub(super) fn reshaped(before: &Sketch, after: &Sketch) -> bool {
    before.points() != after.points()
        || before
            .circles()
            .iter()
            .zip(after.circles())
            .any(|(one, other)| one.radius != other.radius)
}

/// The points a drag on `point` would carry along, when it is the centre of
/// an arc or of an ellipse: the centre and the curve's own points move as one,
/// the way a circle's rim follows a dragged centre. `None` for a point that is
/// nobody's centre, so a plain drag of it stays a plain drag.
fn centre_group(sketch: &Sketch, point: PointId) -> Option<Vec<PointId>> {
    let mut ends = sketch.arc_ends_around(point);
    ends.extend(sketch.ellipse_ends_around(point));
    (!ends.is_empty()).then(|| {
        let mut group = vec![point];
        group.extend(ends);
        group
    })
}
