use cao_part::Operation;
use cao_sketch::{Chamfer, ChamferMode, LockedInput, SegmentId, ToolState};
use glam::DVec2;

use crate::screens::viewport::SketchContext;
use crate::wording::outcome;

/// One click of the chamfer tool: the first names a side of the corner, the
/// second names the other and cuts it, once the values the mode asks for have
/// been typed.
pub(crate) fn chamfer(
    context: &mut SketchContext<'_>,
    index: usize,
    cursor: DVec2,
    snap: f64,
) -> bool {
    let Some(sketch) = context.document.sketches().get(index) else {
        return false;
    };
    let Some(picked) = sketch.nearest_segment(cursor, snap) else {
        return false;
    };

    let held = match &context.editor.tool_state {
        ToolState::Chamfer { sides } => sides.first().copied(),
        _ => None,
    };
    let Some(first) = held.filter(|first| *first != picked) else {
        context.editor.tool_state = ToolState::Chamfer {
            sides: vec![picked],
        };
        context.editor.live.open();
        context.editor.message = Some(context.lang.t("sketch.click_the_other_side"));
        return true;
    };

    cut(context, index, first, picked)
}

/// Cuts the corner, or says why it cannot be.
///
/// The sides are kept when only the values are missing, so typing them and
/// pressing Enter finishes what the two clicks already said.
pub(crate) fn cut(
    context: &mut SketchContext<'_>,
    index: usize,
    first: SegmentId,
    second: SegmentId,
) -> bool {
    let mode = context.editor.chamfer_mode;
    let Some(sketch) = context.document.sketches().get(index) else {
        return false;
    };
    if sketch.corner_points(first, second).is_none() {
        context.editor.tool_state = ToolState::Chamfer { sides: vec![first] };
        context.editor.message = Some(context.lang.t("sketch.chamfer_needs_a_corner"));
        return false;
    }

    let Some(asked) = typed(context.editor.live.locked(), mode) else {
        context.editor.tool_state = ToolState::Chamfer {
            sides: vec![first, second],
        };
        context.editor.message = Some(asks_for(context, mode));
        return false;
    };
    if !sketch.chamfer_fits(first, second, in_units(asked, context.document.scale())) {
        context.editor.message = Some(context.lang.t("sketch.chamfer_too_long"));
        return false;
    }

    let applied = context.document.apply(Operation::Chamfer {
        sketch: index,
        first,
        second,
        mode: asked,
    });
    context.editor.tool_state = ToolState::None;
    context.editor.live.clear();
    context.editor.message = outcome::message(context.lang, applied)
        .or_else(|| Some(context.lang.t("sketch.click_a_corner")));
    true
}

/// The values the mode needs, once they have all been typed.
fn typed(locked: LockedInput, mode: ChamferMode) -> Option<Chamfer> {
    let first = locked.first?;
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

/// The sides of the corner the two clicks named, when both have been taken.
pub(crate) fn corner_held(state: &ToolState) -> Option<(SegmentId, SegmentId)> {
    match state {
        ToolState::Chamfer { sides } => match sides.as_slice() {
            [first, second] => Some((*first, *second)),
            _ => None,
        },
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn locked(first: Option<f64>, second: Option<f64>) -> LockedInput {
        LockedInput { first, second }
    }

    #[test]
    fn the_equal_mode_takes_the_one_value_typed_for_both_sides() {
        assert_eq!(
            typed(locked(Some(4.0), None), ChamferMode::Equal),
            Some(Chamfer::Equal(4.0)),
            "a second value would say nothing the first does not"
        );
    }

    #[test]
    fn a_mode_asking_two_values_waits_until_both_are_typed() {
        assert_eq!(typed(locked(Some(4.0), None), ChamferMode::Sided), None);
        assert_eq!(typed(locked(None, Some(30.0)), ChamferMode::Angled), None);
        assert_eq!(
            typed(locked(Some(4.0), Some(30.0)), ChamferMode::Angled),
            Some(Chamfer::Angled {
                along: 4.0,
                degrees: 30.0
            })
        );
    }

    #[test]
    fn a_corner_is_held_only_once_both_its_sides_have_been_clicked() {
        assert_eq!(corner_held(&ToolState::None), None);
        assert_eq!(
            corner_held(&ToolState::Chamfer {
                sides: vec![SegmentId(1)]
            }),
            None,
            "one side is half a corner, and Enter has nothing to cut"
        );
        assert_eq!(
            corner_held(&ToolState::Chamfer {
                sides: vec![SegmentId(1), SegmentId(2)]
            }),
            Some((SegmentId(1), SegmentId(2)))
        );
    }
}
