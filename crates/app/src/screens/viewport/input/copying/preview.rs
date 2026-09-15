use cao_sketch::{Preview, Repeats, Sketch, ToolState};
use glam::DVec2;

use crate::screens::sketch::{SketchEditor, Tool};

use super::{axis_at, filled, turned};

/// What a tool that lays copies would leave, shown before the click that names
/// where the copies go.
///
/// Nothing until the selection is closed and the values are typed: a pattern
/// with no step has no copies to show.
pub(crate) fn previewed(
    sketch: &Sketch,
    editor: &SketchEditor,
    cursor: DVec2,
    snap: f64,
    scale: f64,
) -> Option<Preview> {
    let ToolState::Copying {
        held,
        naming_the_target: true,
    } = &editor.tool_state
    else {
        return None;
    };
    if held.is_empty() {
        return None;
    }

    match editor.tool {
        Tool::Mirror => {
            let axis = axis_at(sketch, cursor, snap)?;
            sketch.preview(|trial| trial.mirror(held, axis))
        }
        Tool::CircularPattern => {
            let centre = sketch.nearest_point(cursor, snap)?;
            let (degrees, count) = turned(editor.live.locked())?;
            sketch.preview(|trial| trial.pattern_around(held, centre, degrees, count))
        }
        Tool::RectangularPattern => {
            let direction = axis_at(sketch, cursor, snap)?;
            let live = &editor.live;
            let (along, across) =
                filled([live.typed(0), live.typed(1), live.typed(2), live.typed(3)])?;
            let in_units = |run: Repeats| Repeats {
                step: run.step / scale,
                count: run.count,
            };
            sketch.preview(|trial| {
                trial.pattern_along(held, direction, in_units(along), in_units(across))
            })
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use cao_sketch::{Element, WorkPlane};

    use super::*;

    /// A trait standing well clear of both sketch axes, and the axis it is
    /// mirrored across running through the origin.
    fn a_trait_off_to_one_side() -> (Sketch, Element) {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let low = sketch.add_point(DVec2::new(4.0, 5.0));
        let high = sketch.add_point(DVec2::new(8.0, 5.0));
        let drawn = sketch.add_segment(low, high);
        (sketch, Element::Segment(drawn))
    }

    fn holding(tool: Tool, held: Vec<Element>) -> SketchEditor {
        SketchEditor {
            tool,
            tool_state: ToolState::Copying {
                held,
                naming_the_target: true,
            },
            ..Default::default()
        }
    }

    #[test]
    fn a_mirror_shows_the_copies_the_axis_under_the_cursor_would_lay() {
        let (sketch, drawn) = a_trait_off_to_one_side();
        let editor = holding(Tool::Mirror, vec![drawn]);
        let over_the_u_axis = DVec2::new(20.0, 0.0);

        let shown = previewed(&sketch, &editor, over_the_u_axis, 0.5, 1.0)
            .expect("an axis under the cursor to mirror across");

        let copies = shown
            .laid
            .iter()
            .filter(|laid| matches!(laid, Element::Segment(_)))
            .count();
        assert_eq!(copies, 1, "the one trait held is copied once");
        assert_eq!(
            sketch.segments().len(),
            1,
            "the drawing itself gains nothing until the click"
        );
    }

    #[test]
    fn a_selection_still_being_gathered_shows_nothing() {
        let (sketch, drawn) = a_trait_off_to_one_side();
        let editor = SketchEditor {
            tool: Tool::Mirror,
            tool_state: ToolState::Copying {
                held: vec![drawn],
                naming_the_target: false,
            },
            ..Default::default()
        };

        assert!(
            previewed(&sketch, &editor, DVec2::new(20.0, 0.0), 0.5, 1.0).is_none(),
            "a click still adding to the selection is not a click that lays anything"
        );
    }

    #[test]
    fn a_circular_pattern_shows_every_copy_its_count_asks_for_bar_the_original() {
        let (mut sketch, drawn) = a_trait_off_to_one_side();
        let centre = sketch.add_point(DVec2::ZERO);
        let mut editor = holding(Tool::CircularPattern, vec![drawn]);
        editor.live.field(0).locked = Some(90.0);
        editor.live.field(1).locked = Some(4.0);

        let shown = previewed(&sketch, &editor, sketch.point(centre), 0.5, 1.0)
            .expect("a point under the cursor to turn about");

        let copies = shown
            .laid
            .iter()
            .filter(|laid| matches!(laid, Element::Segment(_)))
            .count();
        assert_eq!(copies, 3, "four standing in the end is three laid");
    }

    #[test]
    fn a_pattern_with_its_values_still_untyped_shows_nothing() {
        let (mut sketch, drawn) = a_trait_off_to_one_side();
        sketch.add_point(DVec2::ZERO);
        let editor = holding(Tool::CircularPattern, vec![drawn]);

        assert!(
            previewed(&sketch, &editor, DVec2::ZERO, 0.5, 1.0).is_none(),
            "a pattern with no angle and no count has no copies to show"
        );
    }

    #[test]
    fn a_step_typed_in_millimetres_lands_the_copies_where_the_drawing_measures() {
        let (sketch, drawn) = a_trait_off_to_one_side();
        let mut editor = holding(Tool::RectangularPattern, vec![drawn]);
        for (rank, value) in [(0, 20.0), (1, 2.0), (2, 0.0), (3, 1.0)] {
            editor.live.field(rank).locked = Some(value);
        }

        let shown = previewed(&sketch, &editor, DVec2::new(20.0, 0.0), 0.5, 2.0)
            .expect("an axis under the cursor to run along");

        let [Element::Point(first), ..] = shown.laid.as_slice() else {
            panic!(
                "the copy stands on corners of its own, got {:?}",
                shown.laid
            );
        };
        let moved = shown.sketch.point(*first) - DVec2::new(4.0, 5.0);
        assert!(
            (moved.x - 10.0).abs() <= 1e-9 && moved.y.abs() <= 1e-9,
            "twenty millimetres at two per unit is ten units along the axis, got {moved}"
        );
    }
}
