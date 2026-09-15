use cao_sketch::{Chamfer, Preview, Sketch, ToolState};
use glam::DVec2;

use crate::screens::sketch::{SketchEditor, Tool};

use super::{in_units, typed};

/// What a corner would become, shown before the click that commits it.
///
/// The two sides are the ones already clicked, or the one clicked and the one
/// under the cursor. Nothing until the values are typed: a corner with no
/// radius has no curve to show.
pub(crate) fn previewed(
    sketch: &Sketch,
    editor: &SketchEditor,
    cursor: DVec2,
    snap: f64,
    scale: f64,
) -> Option<Preview> {
    if !matches!(editor.tool, Tool::Chamfer | Tool::Fillet) {
        return None;
    }
    let ToolState::Corner { sides } = &editor.tool_state else {
        return None;
    };
    let (first, second) = match sides.as_slice() {
        [first, second] => (*first, *second),
        [first] => (
            *first,
            sketch
                .nearest_segment(cursor, snap)
                .filter(|under| under != first)?,
        ),
        _ => return None,
    };

    let rounding = editor.tool == Tool::Fillet;
    let asked = in_units(
        typed(editor.live.locked(), editor.chamfer_mode, rounding)?,
        scale,
    );
    match (rounding, asked) {
        (true, Chamfer::Equal(radius)) => {
            sketch.preview(|trial| trial.fillet(first, second, radius))
        }
        (true, _) => None,
        (false, _) => sketch.preview(|trial| trial.chamfer(first, second, asked)),
    }
}

#[cfg(test)]
mod tests {
    use cao_sketch::{ChamferMode, Element, SegmentId, WorkPlane};

    use super::*;

    /// A right angle, one side east and one north, ten units each.
    fn a_right_angle() -> (Sketch, SegmentId, SegmentId) {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let pivot = sketch.add_point(DVec2::new(2.0, 1.0));
        let east = sketch.add_point(DVec2::new(12.0, 1.0));
        let north = sketch.add_point(DVec2::new(2.0, 11.0));
        let along = sketch.add_segment(pivot, east);
        let up = sketch.add_segment(pivot, north);
        (sketch, along, up)
    }

    fn rounding(sides: Vec<SegmentId>, radius: Option<f64>) -> SketchEditor {
        let mut editor = SketchEditor {
            tool: Tool::Fillet,
            tool_state: ToolState::Corner { sides },
            ..Default::default()
        };
        editor.live.field(0).locked = radius;
        editor
    }

    #[test]
    fn a_radius_typed_on_a_corner_already_clicked_shows_the_curve_before_enter() {
        let (sketch, along, up) = a_right_angle();
        let editor = rounding(vec![along, up], Some(3.0));

        let shown = previewed(&sketch, &editor, DVec2::ZERO, 0.5, 1.0)
            .expect("a corner with both sides and a radius has a curve to show");

        assert!(
            matches!(shown.laid.as_slice(), [Element::Arc(_)]),
            "the curve the click would lay is what is shown, got {:?}",
            shown.laid
        );
        assert!(
            sketch.arcs().is_empty(),
            "the drawing itself is not rounded until the click"
        );
    }

    #[test]
    fn the_side_under_the_cursor_stands_in_for_the_second_click() {
        let (sketch, along, _) = a_right_angle();
        let editor = rounding(vec![along], Some(3.0));
        let over_the_north_side = DVec2::new(2.0, 6.0);

        assert!(
            previewed(&sketch, &editor, over_the_north_side, 0.5, 1.0).is_some(),
            "hovering the other side says which corner, so the curve can be shown"
        );
        assert!(
            previewed(&sketch, &editor, DVec2::new(40.0, 40.0), 0.5, 1.0).is_none(),
            "a cursor over nothing names no corner"
        );
    }

    #[test]
    fn a_corner_with_no_value_typed_for_it_shows_nothing() {
        let (sketch, along, up) = a_right_angle();
        let editor = rounding(vec![along, up], None);

        assert!(
            previewed(&sketch, &editor, DVec2::ZERO, 0.5, 1.0).is_none(),
            "a corner with no radius has no curve to show"
        );
    }

    #[test]
    fn a_chamfer_shows_the_straight_cut_its_two_distances_would_leave() {
        let (sketch, along, up) = a_right_angle();
        let mut editor = SketchEditor {
            tool: Tool::Chamfer,
            chamfer_mode: ChamferMode::Sided,
            tool_state: ToolState::Corner {
                sides: vec![along, up],
            },
            ..Default::default()
        };
        editor.live.field(0).locked = Some(2.0);
        editor.live.field(1).locked = Some(4.0);

        let shown = previewed(&sketch, &editor, DVec2::ZERO, 0.5, 1.0)
            .expect("a corner with both distances typed");

        assert!(
            matches!(shown.laid.as_slice(), [Element::Segment(_)]),
            "the cut is what is shown, got {:?}",
            shown.laid
        );
    }

    #[test]
    fn a_value_typed_in_millimetres_is_shown_at_the_size_the_drawing_measures() {
        let (sketch, along, up) = a_right_angle();
        let editor = rounding(vec![along, up], Some(6.0));

        let shown = previewed(&sketch, &editor, DVec2::ZERO, 0.5, 2.0)
            .expect("a corner that can be rounded at three units");

        let [Element::Arc(arc)] = shown.laid.as_slice() else {
            panic!("the curve is what is shown, got {:?}", shown.laid);
        };
        let reach = shown.sketch.arc_radius(*arc);
        assert!(
            (reach - 3.0).abs() <= 1e-9,
            "six millimetres at two per unit is three units, got {reach}"
        );
    }
}
