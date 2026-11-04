//! A value typed into a dimension already on the drawing, and what the part
//! makes of it.

use cao_part::{DimensionOutcome, Outcome, PartDocument, history::Operation};
use cao_sketch::{DimensionTarget, LengthOutcome, ToolState};

use super::SketchEditor;

/// A dimension already on the drawing, opened for editing: its value, typed
/// into the field beside it.
pub struct DimensionEdit {
    pub target: DimensionTarget,
    pub input: String,
    /// Set when the dimension has just been opened, so its field takes the
    /// keyboard on its own: reaching it with Tab means walking through every
    /// button of the toolbar first.
    pub focus: bool,
    /// Set when placing this very dimension is what gave the part its scale,
    /// at what it read: a value typed straight into it is placed instead.
    pub placed_the_scale: bool,
}

impl DimensionEdit {
    /// Marks this dimension as the one that gave the part its scale, its field
    /// showing what its label shows rather than the unround value it holds.
    pub(crate) fn placed_the_scale_at(&mut self, held: f64) {
        self.placed_the_scale = true;
        self.input = crate::wording::dimension::short(held);
    }
}

/// `placement: None` leaves the annotation where it was put down, not reset;
/// a value taking the place of the one a dimension was placed with says where
/// that was, since the step that said it is taken back.
pub(crate) fn apply_dimension_value(
    document: &mut PartDocument,
    editor: &mut SketchEditor,
    index: usize,
    target: DimensionTarget,
    lang: &crate::lang::Catalogue,
) -> bool {
    let Some(typed) = editor.editing.as_ref().map(|editing| editing.input.clone()) else {
        return false;
    };
    let (written, value) = match document.variables().size_of(&typed) {
        Ok(read) => read,
        Err(wrong) => {
            editor.message = Some(crate::wording::formula::unusable(lang, &wrong));
            return false;
        }
    };
    let rules = editor.sketching.exact_angles_lay_rules;
    // Only a number typed as such: a value written from a variable would lose
    // its link to it, and stays a dimension.
    if rules
        && written.as_number().is_some()
        && let Some(in_place) = document.sketches()[index].rule_in_place_of(target, value)
    {
        end_the_gesture(editor);
        let rule = in_place.rule;
        if !document.lay_in_place_of(index, target, in_place) {
            editor.message = Some(crate::wording::constraints::impossible(lang));
            return false;
        }
        editor.message = Some(lang.t_with(
            "sketch.rule_in_place_of_dimension",
            &[("rule", &crate::wording::constraints::label(lang, rule))],
        ));
        return true;
    }
    if !target.takes(value) {
        editor.message = Some(lang.t(if rules {
            "sketch.angle_would_lay_parallel_but_exactly"
        } else {
            "sketch.angle_would_lay_parallel"
        }));
        return false;
    }

    end_the_gesture(editor);

    // The same value twice must not repeat an identical step in the history —
    // though the same number written from a variable, or no longer from one,
    // is a step: it changes what the value follows. The dimension that set the
    // scale opened on less than it holds, and left as it opened is the same.
    let placed_the_scale = editor
        .editing
        .as_ref()
        .is_some_and(|editing| editing.placed_the_scale);
    let left_as_opened =
        |held: f64| placed_the_scale && typed == crate::wording::dimension::short(held);
    if document.sketches()[index]
        .dimension_of(target)
        .is_some_and(|dimension| {
            (same_value(dimension.value, value) || left_as_opened(dimension.value))
                && dimension.written == written.note()
        })
    {
        editor.message = None;
        return false;
    }

    let placement = take_back_the_first_placing(document, editor, index, target, value);
    match document.apply(Operation::SetDimension {
        sketch: index,
        target,
        value: written,
        placement,
    }) {
        Some(Outcome::Dimension(DimensionOutcome::ScaleDefined {
            millimeters_per_unit: mm,
        })) => {
            editor.message = Some(lang.t_with("sketch.scale_set", &[("mm", &format!("{mm:.4}"))]));
            true
        }
        Some(Outcome::Dimension(DimensionOutcome::Geometry(LengthOutcome::Exact))) => {
            editor.message = None;
            true
        }
        Some(Outcome::Dimension(DimensionOutcome::Geometry(LengthOutcome::BestEffort))) => {
            editor.message = Some(crate::wording::dimension::conflict_warning(lang));
            false
        }
        Some(Outcome::Dimension(DimensionOutcome::Reference)) => {
            editor.message = Some(crate::wording::dimension::redundant_warning(lang));
            true
        }
        _ => {
            editor.message = Some(lang.t("sketch.no_dimension_here"));
            false
        }
    }
}

/// Whether two values are one to the user: far under the tenth of a micron they
/// are shown to, and over the noise of working one out from the drawing.
pub(crate) fn same_value(first: f64, second: f64) -> bool {
    (first - second).abs() <= 1e-9 * first.abs().max(second.abs()).max(1.0)
}

/// What a field opens on: the value as it is shown, unless that is not it.
pub(crate) fn opened_on(value: f64) -> String {
    let shown = crate::wording::dimension::short(value);
    match shown.parse().is_ok_and(|read| same_value(read, value)) {
        true => shown,
        false => value.to_string(),
    }
}

/// A value typed ends the gesture: the trait it was typed on stops being the
/// first half of an angle the next click would complete.
fn end_the_gesture(editor: &mut SketchEditor) {
    if let ToolState::Dimension { picks, .. } = &mut editor.tool_state {
        picks.first_angle_segment = None;
    }
}

/// Placing the part's first dimension gave it a scale of one unit to the
/// millimetre, at what the dimension read. A length typed straight into it is
/// the value it is placed with instead: the step that placed it is taken back,
/// so that this one is what says what a unit is worth, and nothing moves. Says
/// where the dimension was put down, for it to stay there.
fn take_back_the_first_placing(
    document: &mut PartDocument,
    editor: &SketchEditor,
    index: usize,
    target: DimensionTarget,
    value: f64,
) -> Option<glam::DVec2> {
    let placed = editor
        .editing
        .as_ref()
        .is_some_and(|editing| editing.placed_the_scale);
    let last = document.history.applied_operations().last();
    let placing = matches!(last, Some(Operation::SetDimension { sketch, target: set, .. })
        if *sketch == index && *set == target);
    if !placed || !placing || value <= 0.0 {
        return None;
    }
    let offset = document.sketches()[index]
        .dimension_of(target)
        .and_then(|dimension| dimension.offset);
    document.undo();
    offset
}

#[cfg(test)]
mod tests;
