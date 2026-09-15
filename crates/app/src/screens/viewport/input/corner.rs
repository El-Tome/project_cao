use cao_part::Operation;
use cao_sketch::{Chamfer, ChamferMode, LockedInput, SegmentId, ToolState};
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
    let Some(picked) = sketch.nearest_segment(cursor, snap) else {
        return false;
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
pub(super) fn typed(locked: LockedInput, mode: ChamferMode, rounding: bool) -> Option<Chamfer> {
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
pub(super) fn in_units(asked: Chamfer, millimeters_per_unit: f64) -> Chamfer {
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
        ToolState::Corner { sides } => match sides.as_slice() {
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
            typed(locked(Some(4.0), None), ChamferMode::Equal, false),
            Some(Chamfer::Equal(4.0)),
            "a second value would say nothing the first does not"
        );
    }

    #[test]
    fn a_mode_asking_two_values_waits_until_both_are_typed() {
        assert_eq!(
            typed(locked(Some(4.0), None), ChamferMode::Sided, false),
            None
        );
        assert_eq!(
            typed(locked(None, Some(30.0)), ChamferMode::Angled, false),
            None
        );
        assert_eq!(
            typed(locked(Some(4.0), Some(30.0)), ChamferMode::Angled, false),
            Some(Chamfer::Angled {
                along: 4.0,
                degrees: 30.0
            })
        );
    }

    #[test]
    fn a_fillet_asks_for_one_value_whatever_the_chamfer_beside_it_is_set_to() {
        assert_eq!(
            typed(locked(Some(5.0), None), ChamferMode::Sided, true),
            Some(Chamfer::Equal(5.0)),
            "a radius is a radius, and the chamfer's second field says nothing about it"
        );
    }

    #[test]
    fn a_corner_is_held_only_once_both_its_sides_have_been_clicked() {
        assert_eq!(corner_held(&ToolState::None), None);
        assert_eq!(
            corner_held(&ToolState::Corner {
                sides: vec![SegmentId(1)]
            }),
            None,
            "one side is half a corner, and Enter has nothing to cut"
        );
        assert_eq!(
            corner_held(&ToolState::Corner {
                sides: vec![SegmentId(1), SegmentId(2)]
            }),
            Some((SegmentId(1), SegmentId(2)))
        );
    }
}
