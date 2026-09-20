use cao_part::Operation;
use cao_sketch::{Chamfer, ChamferMode, LockedInput, SegmentId, Sketch, ToolState};
use glam::DVec2;

use crate::screens::sketch::Tool;

mod preview;
use crate::screens::viewport::SketchContext;
use crate::wording::outcome;
pub(crate) use preview::previewed;

/// One click of the chamfer or the fillet tool: the first names a side of the
/// corner, the second names the other and cuts it, once the values the tool
/// asks for have been typed.
pub(crate) fn corner(
    context: &mut SketchContext<'_>,
    index: usize,
    cursor: DVec2,
    snap: f64,
) -> bool {
    let Some(sketch) = context.document.sketches().get(index) else {
        return false;
    };
    let picked = match clicked(sketch, cursor, snap, takes_a_point(context)) {
        CornerClick::Corner(first, second) => {
            // The click that names a whole corner is the first click of that
            // corner, so it opens the fields the way naming a first side does.
            // Without it there is nowhere to type, and the tool asks for a
            // value the keyboard cannot reach.
            context.editor.live.open();
            return cut(context, index, first, second);
        }
        CornerClick::Crowded => {
            context.editor.message = Some(context.lang.t("sketch.corner_has_too_many_traits"));
            return true;
        }
        CornerClick::Side(side) => side,
        CornerClick::Nothing => return false,
    };

    let held = match &context.editor.tool_state {
        ToolState::Corner { sides } => sides.first().copied(),
        _ => None,
    };
    let Some(first) = held.filter(|first| *first != picked) else {
        context.editor.tool_state = ToolState::Corner {
            sides: vec![picked],
        };
        context.editor.live.open();
        context.editor.message = Some(context.lang.t("sketch.click_the_other_side"));
        return true;
    };

    cut(context, index, first, picked)
}

/// Cuts or rounds the corner, or says why it cannot be.
///
/// The sides are kept when only the values are missing, so typing them and
/// pressing Enter finishes what the two clicks already said.
pub(crate) fn cut(
    context: &mut SketchContext<'_>,
    index: usize,
    first: SegmentId,
    second: SegmentId,
) -> bool {
    let rounding = context.editor.tool == Tool::Fillet;
    let mode = context.editor.chamfer_mode;
    let Some(sketch) = context.document.sketches().get(index) else {
        return false;
    };
    if sketch.corner_points(first, second).is_none() {
        context.editor.tool_state = ToolState::Corner { sides: vec![first] };
        context.editor.message = Some(context.lang.t("sketch.chamfer_needs_a_corner"));
        return false;
    }

    let Some(asked) = typed(context.editor.live.locked(), mode, rounding) else {
        context.editor.tool_state = ToolState::Corner {
            sides: vec![first, second],
        };
        context.editor.message = Some(match rounding {
            true => context.lang.t("sketch.fillet_asks_a_radius"),
            false => asks_for(context, mode),
        });
        return false;
    };

    let Some(operation) = fits(context, index, first, second, asked, rounding) else {
        return false;
    };
    let applied = context.document.apply(operation);
    context.editor.tool_state = ToolState::None;
    context.editor.live.clear();
    context.editor.message = outcome::message(context.lang, applied)
        .or_else(|| Some(context.lang.t("sketch.click_a_corner")));
    true
}

/// The operation to record, once the drawing says the corner can take it.
fn fits(
    context: &mut SketchContext<'_>,
    index: usize,
    first: SegmentId,
    second: SegmentId,
    asked: Chamfer,
    rounding: bool,
) -> Option<Operation> {
    let scale = context.document.scale();
    let sketch = context.document.sketches().get(index)?;
    let Chamfer::Equal(radius) = asked else {
        return sketch
            .chamfer_fits(first, second, in_units(asked, scale))
            .then_some(Operation::Chamfer {
                sketch: index,
                first,
                second,
                mode: asked,
            })
            .or_else(|| {
                context.editor.message = Some(context.lang.t("sketch.chamfer_too_long"));
                None
            });
    };
    match rounding {
        true => sketch
            .fillet_fits(first, second, radius / scale)
            .then_some(Operation::Fillet {
                sketch: index,
                first,
                second,
                radius,
            })
            .or_else(|| {
                context.editor.message = Some(context.lang.t("sketch.fillet_too_big"));
                None
            }),
        false => sketch
            .chamfer_fits(first, second, in_units(asked, scale))
            .then_some(Operation::Chamfer {
                sketch: index,
                first,
                second,
                mode: asked,
            })
            .or_else(|| {
                context.editor.message = Some(context.lang.t("sketch.chamfer_too_long"));
                None
            }),
    }
}

/// The values the tool needs, once they have all been typed. A fillet asks for
/// a radius and nothing else, whichever mode the chamfer beside it is in.
fn typed(locked: LockedInput, mode: ChamferMode, rounding: bool) -> Option<Chamfer> {
    let first = locked.first?;
    if rounding {
        return Some(Chamfer::Equal(first));
    }
    match mode.wants() {
        1 => Some(mode.of(first, first)),
        _ => Some(mode.of(first, locked.second?)),
    }
}

/// A chamfer as the drawing measures it, for asking whether it fits: the
/// values are typed in millimetres, and the sketch works in its own units.
fn in_units(asked: Chamfer, millimeters_per_unit: f64) -> Chamfer {
    match asked {
        Chamfer::Equal(reach) => Chamfer::Equal(reach / millimeters_per_unit),
        Chamfer::Sided { first, second } => Chamfer::Sided {
            first: first / millimeters_per_unit,
            second: second / millimeters_per_unit,
        },
        Chamfer::Angled { along, degrees } => Chamfer::Angled {
            along: along / millimeters_per_unit,
            degrees,
        },
    }
}

fn asks_for(context: &SketchContext<'_>, mode: ChamferMode) -> String {
    context.lang.t(match mode {
        ChamferMode::Equal => "sketch.chamfer_asks_a_distance",
        ChamferMode::Angled => "sketch.chamfer_asks_a_distance_and_an_angle",
        ChamferMode::Sided => "sketch.chamfer_asks_two_distances",
    })
}

/// What one click names, for a tool that cuts corners.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CornerClick {
    /// A point where exactly two traits meet: one click names the whole
    /// corner.
    Corner(SegmentId, SegmentId),
    /// A point too crowded to name a corner on its own.
    Crowded,
    /// A side of a corner, waiting for the other to be clicked.
    Side(SegmentId),
    /// Nothing the tool can use.
    Nothing,
}

/// What the click under the cursor names.
///
/// A point is read before a trait, because a point is the smaller target and
/// the one the user aimed at when they hit it. `by_point` is false for the
/// chamfer modes that need a first side named: a corner taken by its point has
/// no first side to give them.
fn clicked(sketch: &Sketch, cursor: DVec2, snap: f64, by_point: bool) -> CornerClick {
    if by_point && let Some(point) = sketch.nearest_point(cursor, snap) {
        match sketch.corner_at(point) {
            Some((first, second)) => return CornerClick::Corner(first, second),
            // Only a crowd is worth explaining. A point where one trait simply
            // ends is not a corner anybody was promised, and the trait under
            // the cursor is still worth naming.
            None if sketch.traits_at(point).len() > 2 => return CornerClick::Crowded,
            None => {}
        }
    }
    match sketch.nearest_segment(cursor, snap) {
        Some(side) => CornerClick::Side(side),
        None => CornerClick::Nothing,
    }
}

/// Whether the tool names a corner by one click on its point.
///
/// The chamfer in distance and angle, or in two distances, measures from the
/// side named first — so it always takes a side and then the other, one corner
/// at a time. A fillet has no such side, and neither has the chamfer in equal
/// distances.
fn takes_a_point(context: &SketchContext<'_>) -> bool {
    context.editor.tool == Tool::Fillet || context.editor.chamfer_mode == ChamferMode::Equal
}

/// What Enter has to work with, for a tool that cuts corners.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CornerEnter {
    /// Both sides are named: the key finishes what the clicks began.
    Cut(SegmentId, SegmentId),
    /// Not yet — what is still missing, as the key of the sentence that says
    /// so.
    Waiting(&'static str),
}

/// What Enter does for the chamfer and the fillet, whether or not there is a
/// corner to cut.
///
/// A key that lands on nothing used to be handed on to the next tool in the
/// list, and that tool put its own state where the corner's was: the side
/// already clicked was gone, and the only way on was to click it again. The
/// corner tools answer for their own key now, and say what they are waiting
/// for.
pub(crate) fn on_enter(state: &ToolState) -> CornerEnter {
    match corner_held(state) {
        Some((first, second)) => CornerEnter::Cut(first, second),
        None => CornerEnter::Waiting(match state {
            ToolState::Corner { sides } if !sides.is_empty() => "sketch.click_the_other_side",
            _ => "sketch.click_a_corner",
        }),
    }
}

/// The sides of the corner the two clicks named, when both have been taken.
fn corner_held(state: &ToolState) -> Option<(SegmentId, SegmentId)> {
    match state {
        ToolState::Corner { sides } => match sides.as_slice() {
            [first, second] => Some((*first, *second)),
            _ => None,
        },
        _ => None,
    }
}

#[cfg(test)]
mod tests;
