//! What a pattern's fields stand on when they appear.
//!
//! A value there to be typed over: the first thing anyone types is then a
//! number being corrected rather than one guessed at.

use cao_sketch::{Element, ToolState};

use crate::screens::sketch::Tool;
use crate::screens::viewport::SketchContext;

/// A circular pattern owes nothing to what is held: the quarter turn four times
/// round is the pattern anyone draws when they draw one at all.
const A_QUARTER_TURN: f64 = 90.0;
const FOUR_TIMES_ROUND: f64 = 4.0;

/// Both counts of a grid open on the original and one copy — the smallest
/// pattern that lays anything.
const THE_ORIGINAL_AND_ONE: f64 = 2.0;

/// Opens the fields the tool in hand asks for, each on the value it opens on.
///
/// A grid steps by how wide what is held stands, in the millimetres its field
/// is labelled with. The click that names the direction comes after this, so the
/// extent along it cannot be read yet, and the widest way round is the one
/// number no direction can make too small.
///
/// The mirror asks for no values and opens nothing.
pub(super) fn fields_open_on(context: &mut SketchContext<'_>) {
    let ToolState::Copying { held, .. } = &context.editor.tool_state else {
        return;
    };
    let values = match context.editor.tool {
        Tool::CircularPattern => [Some(A_QUARTER_TURN), Some(FOUR_TIMES_ROUND), None, None],
        Tool::RectangularPattern => {
            let span = widest_span(context, held);
            [
                span,
                Some(THE_ORIGINAL_AND_ONE),
                span,
                Some(THE_ORIGINAL_AND_ONE),
            ]
        }
        _ => return,
    };
    context.editor.live.open_on(&values);
}

/// How wide what is held stands, when there is a drawing to read it from.
fn widest_span(context: &SketchContext<'_>, held: &[Element]) -> Option<f64> {
    let index = context.editor.active_sketch()?;
    let span = context.document.sketches().get(index)?.widest_span(held)?;
    Some(context.document.to_millimeters(span))
}

#[cfg(test)]
mod tests {
    use cao_part::{Operation, PartDocument, PointRef};
    use cao_sketch::{CircleId, DimensionTarget, WorkPlane};
    use chrono::Utc;
    use glam::DVec2;

    use super::*;
    use crate::lang::Catalogue;
    use crate::screens::extrusion::ExtrusionState;
    use crate::screens::sketch::{SketchEditor, SketchPhase, Tool};

    /// A drawing holding one circle four units across, and nothing else.
    fn a_circle_four_units_across() -> PartDocument {
        let mut document = PartDocument::new("part", Utc::now());
        document.apply(Operation::CreateSketch {
            plane: WorkPlane::XY,
            on: None,
        });
        document.apply(Operation::AddCircle {
            sketch: 0,
            center: PointRef::New(DVec2::new(10.0, 10.0)),
            radius: 2.0,
            rim: Vec::new(),
            construction: false,
        });
        document
    }

    /// The four fields as `Entrée` leaves them, that circle held.
    fn fields_opened_by(tool: Tool, document: &mut PartDocument) -> Vec<Option<f64>> {
        let mut editor = SketchEditor {
            phase: SketchPhase::Editing(0),
            tool,
            tool_state: ToolState::Copying {
                held: vec![Element::Circle(CircleId(0))],
                naming_the_target: false,
            },
            ..Default::default()
        };
        let mut extrusion = ExtrusionState::default();
        let lang = Catalogue::french();
        let mut context = SketchContext {
            document,
            editor: &mut editor,
            extrusion: &mut extrusion,
            lang: &lang,
        };

        super::super::hold_is_done(&mut context);
        (0..4).map(|rank| context.editor.live.typed(rank)).collect()
    }

    #[test]
    fn a_grid_opens_on_a_step_the_hold_fits_in_and_one_copy_each_way() {
        assert_eq!(
            fields_opened_by(Tool::RectangularPattern, &mut a_circle_four_units_across()),
            vec![Some(4.0), Some(2.0), Some(4.0), Some(2.0)]
        );
    }

    #[test]
    fn a_circular_pattern_opens_on_a_quarter_turn_four_times_round() {
        assert_eq!(
            fields_opened_by(Tool::CircularPattern, &mut a_circle_four_units_across()),
            vec![Some(90.0), Some(4.0), None, None]
        );
    }

    #[test]
    fn the_mirror_opens_no_fields_at_all() {
        assert_eq!(
            fields_opened_by(Tool::Mirror, &mut a_circle_four_units_across()),
            vec![None; 4]
        );
    }

    #[test]
    fn a_step_opens_in_the_millimetres_its_field_is_labelled_with() {
        let mut document = a_circle_four_units_across();
        document.apply(Operation::SetDimension {
            sketch: 0,
            target: DimensionTarget::Diameter(CircleId(0)),
            value: 20.0,
            placement: None,
        });

        let opened = fields_opened_by(Tool::RectangularPattern, &mut document);

        assert_eq!(
            opened[0],
            Some(20.0),
            "five millimetres to the unit, and the circle four units across"
        );
    }
}
