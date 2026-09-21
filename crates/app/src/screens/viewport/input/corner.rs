use cao_part::Operation;
use cao_sketch::{
    Chamfer, ChamferMode, Corner, LockedInput, PointId, SegmentId, Sketch, ToolState,
};
use glam::DVec2;

use crate::screens::sketch::Tool;

mod preview;
use crate::screens::viewport::SketchContext;
use crate::wording::outcome;
pub(crate) use preview::previewed;

/// One click of the chamfer or the fillet tool.
///
/// A click on a corner's point takes that corner, and a second click on it
/// drops it again. A click on a side names half a corner, which the next click
/// on the other side completes. Either way the values typed apply to every
/// corner taken, and Enter lays them all.
pub(crate) fn corner(
    context: &mut SketchContext<'_>,
    index: usize,
    cursor: DVec2,
    snap: f64,
) -> bool {
    let Some(sketch) = context.document.sketches().get(index) else {
        return false;
    };
    let (mut taken, half) = held(&context.editor.tool_state);

    match clicked(sketch, cursor, snap, takes_a_point(context)) {
        CornerClick::Corner(corner) => {
            toggle(&mut taken, corner);
            wait_for_values(context, taken, None);
        }
        CornerClick::Side(side) => {
            // A click on the corner itself finds whichever trait the drawing
            // holds first, and the same one every time. With one already
            // named, the other is the only choice left.
            let side = match half.filter(|first| *first == side) {
                Some(named) => sketch
                    .nearest_point(cursor, snap)
                    .and_then(|point| sketch.other_side_at(point, named))
                    .unwrap_or(side),
                None => side,
            };
            let Some(first) = half.filter(|first| *first != side) else {
                wait_for_values(context, taken, Some(side));
                context.editor.message = Some(context.lang.t("sketch.click_the_other_side"));
                return true;
            };
            // Two traits that do not meet is a different failure from a
            // corner too tight, and the count of corners turned away would
            // describe it wrongly.
            if sketch.shared_point(first, side).is_none() {
                keep_typing(context);
                context.editor.message = Some(context.lang.t("sketch.chamfer_needs_a_corner"));
                return true;
            }
            let corner = Corner::Between(first, side);
            // Named this way a corner carries its own first side, so a mode
            // that measures from one gathers corners like any other. What it
            // cannot do is take a corner by its point, which would leave it no
            // first side at all.
            toggle(&mut taken, corner);
            wait_for_values(context, taken, None);
        }
        CornerClick::Crowded => {
            keep_typing(context);
            context.editor.message = Some(context.lang.t("sketch.corner_has_too_many_traits"));
            return true;
        }
        CornerClick::Nothing => {
            keep_typing(context);
            return false;
        }
    }

    // A click never lays. The values typed are meant for every corner the
    // gesture names, and a click that laid them would close the gesture the
    // moment a value was in the field — leaving no way to add the next corner.
    let (taken, half) = held(&context.editor.tool_state);
    if half.is_none() {
        context.editor.message = Some(context.lang.t_with(
            "sketch.corners_taken",
            &[("count", &taken.len().to_string())],
        ));
    }
    true
}

/// The points of the corners the tool is holding, so the drawing can show what
/// a click has taken. A gesture that gathers several needs to say which.
pub(crate) fn corners_taken(sketch: &Sketch, state: &ToolState) -> Vec<PointId> {
    let ToolState::Corner { taken, .. } = state else {
        return Vec::new();
    };
    taken
        .iter()
        .filter_map(|corner| sketch.corner_point(*corner))
        .collect()
}

/// Takes a corner, or drops it when it was already taken.
///
/// Clicking twice is how a corner is changed one's mind about: there is no
/// second gesture for putting one back, and a list that only grows makes a
/// slip of the mouse cost the whole selection.
fn toggle(taken: &mut Vec<Corner>, corner: Corner) {
    match taken.iter().position(|already| *already == corner) {
        Some(rank) => {
            taken.remove(rank);
        }
        None => taken.push(corner),
    }
}

/// What the tool is holding: the corners taken, and half a corner if a side is
/// waiting for its other.
fn held(state: &ToolState) -> (Vec<Corner>, Option<SegmentId>) {
    match state {
        ToolState::Corner { taken, half } => (taken.clone(), *half),
        _ => (Vec::new(), None),
    }
}

/// Puts back what the click left the tool holding, with the keyboard on the
/// field. `open` rather than a nudge only when nothing was being typed yet:
/// it clears, and a value typed for the first corner is meant for the next.
fn wait_for_values(context: &mut SketchContext<'_>, taken: Vec<Corner>, half: Option<SegmentId>) {
    let fresh = context.editor.live.locked().first.is_none();
    context.editor.tool_state = ToolState::Corner { taken, half };
    match fresh {
        true => context.editor.live.open(),
        false => keep_typing(context),
    }
}

/// Cuts or rounds every corner taken, or says what is still missing.
///
/// What is held is kept when only the values are missing, so typing them and
/// pressing Enter finishes what the clicks already said.
pub(crate) fn cut(context: &mut SketchContext<'_>, index: usize) -> bool {
    let rounding = context.editor.tool == Tool::Fillet;
    let mode = context.editor.chamfer_mode;
    let (taken, _) = held(&context.editor.tool_state);
    if taken.is_empty() {
        context.editor.message = Some(context.lang.t("sketch.click_a_corner"));
        return false;
    }

    let Some(asked) = typed(context.editor.live.locked(), mode, rounding) else {
        keep_typing(context);
        context.editor.message = Some(match rounding {
            true => context.lang.t("sketch.fillet_asks_a_radius"),
            false => asks_for(context, mode),
        });
        return false;
    };

    let applied = context
        .document
        .apply(laying(index, taken, asked, rounding));
    context.editor.tool_state = ToolState::None;
    context.editor.live.clear();
    context.editor.message = outcome::message(context.lang, applied)
        .or_else(|| Some(context.lang.t("sketch.click_a_corner")));
    true
}

/// The operation that lays every corner taken. Whether each of them fits is
/// the drawing's business, and a corner too tight is counted and said rather
/// than stopping the rest.
fn laying(index: usize, taken: Vec<Corner>, asked: Chamfer, rounding: bool) -> Operation {
    match (rounding, asked) {
        (true, Chamfer::Equal(radius)) => Operation::Fillet {
            sketch: index,
            corners: taken,
            radius,
        },
        _ => Operation::Chamfer {
            sketch: index,
            corners: taken,
            mode: asked,
        },
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
    Corner(Corner),
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
            Some(_) => return CornerClick::Corner(Corner::At(point)),
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

/// Puts the keyboard back on the value field after a click in the canvas.
///
/// The flag is read once and taken, so a field asks for the keyboard the frame
/// it appears and never again. A click on the drawing gives the canvas its own
/// focus, and the field the tool is still waiting on goes quiet — what is typed
/// next lands nowhere. Only the keyboard moves: what has already been typed
/// stays, which is why this is not `open`.
fn keep_typing(context: &mut SketchContext<'_>) {
    context.editor.live.focus = true;
}

/// What the tool asks for the moment it is picked, or its mode changed.
///
/// A tool that takes a corner by its point says so; one that needs a first
/// side named asks for a side and then the other. The three used to say the
/// same sentence, which stopped being true the day one click could name a
/// whole corner.
pub(crate) fn picks_with(tool: Tool, mode: ChamferMode) -> &'static str {
    match (tool, mode) {
        (Tool::Fillet, _) => "sketch.click_a_corner_to_round",
        (_, ChamferMode::Equal) => "sketch.click_a_corner_to_cut",
        _ => "sketch.click_one_side_then_the_other",
    }
}

/// Whether the tool names a corner by one click on its point.
///
/// The chamfer in distance and angle, or in two distances, measures from the
/// side named first, and a corner taken by its point has none to give. It
/// still gathers as many corners as it is shown — each one names its own first
/// side, which is the whole of what those modes need. A fillet has no such
/// side, and neither has the chamfer in equal distances.
fn takes_a_point(context: &SketchContext<'_>) -> bool {
    context.editor.tool == Tool::Fillet || context.editor.chamfer_mode == ChamferMode::Equal
}

/// What Enter has to work with, for a tool that cuts corners.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CornerEnter {
    /// At least one corner is taken: the key finishes what the clicks began.
    Cut,
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
    match state {
        ToolState::Corner { taken, .. } if !taken.is_empty() => CornerEnter::Cut,
        ToolState::Corner { half: Some(_), .. } => {
            CornerEnter::Waiting("sketch.click_the_other_side")
        }
        _ => CornerEnter::Waiting("sketch.click_a_corner"),
    }
}

#[cfg(test)]
mod tests;
