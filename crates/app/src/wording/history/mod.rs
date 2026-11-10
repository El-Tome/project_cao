//! How a history step reads: its name in the tree, and the line it unfolds
//! into.

mod cut_change;
mod detail;
mod moved;
mod sized;
mod values;

pub use detail::detail;

use cao_part::history::{ExtrusionMode, FaceAnchor, Operation};
use cao_part::{VariableChange, Variables};
use cao_sketch::{Constraint, DimensionTarget, Element, WorkPlane};

use crate::lang::Catalogue;
use crate::wording::{constraints, dimension, formula, plane};

/// Where a sketch was laid: a plane of the origin, a face of the part, or a
/// plane set off from the origin with no face under it.
pub fn laid_on(lang: &Catalogue, laid: &WorkPlane, on: &Option<FaceAnchor>) -> String {
    match (laid.kind(), on) {
        (cao_sketch::PlaneKind::OffOrigin, None) => lang.t("plane.offset"),
        (kind, _) => plane::label(lang, kind),
    }
}

/// The only place a history step is turned into a name.
///
/// Short, because the history tree shows one per line. A size written from
/// the variables is said as what it was written as, with the names the
/// variables go by now.
pub fn label(lang: &Catalogue, variables: &Variables, operation: &Operation) -> String {
    match operation {
        // Named by what it laid first: a gesture's headline is the thing the
        // user meant to draw, and the rest is what that thing leans on. A rule
        // laid in place of a dimension erases it first, and is named by the
        // rule.
        Operation::Gesture(done) => done
            .iter()
            .find(|operation| !matches!(operation, Operation::EraseMany { .. }))
            .or(done.first())
            .map(|first| label(lang, variables, first))
            .unwrap_or_default(),
        Operation::Variable(change) => variable(lang, variables, change),
        Operation::CreateSketch { plane, on } => {
            lang.t_with("history.sketch", &[("plane", &laid_on(lang, plane, on))])
        }
        Operation::CreateSketchOnSection { section, .. } => {
            let name = lang.t_with("part_tree.section", &[("rank", &(section + 1).to_string())]);
            lang.t_with("history.sketch", &[("plane", &name)])
        }
        Operation::SaveSection(_)
        | Operation::EditSection { .. }
        | Operation::DeleteSection { .. } => lang.t("command.label.section_view"),
        Operation::AddPoint { .. } => lang.t("history.point"),
        Operation::AddSegment { .. } => lang.t("history.segment"),
        Operation::AddSymmetricSegment { .. } => lang.t("history.symmetric_segment"),
        Operation::AddRectangle { .. } => lang.t("history.rectangle"),
        Operation::AddCircle { .. } => lang.t("history.circle"),
        Operation::AddArc { .. } => lang.t("history.arc"),
        Operation::AddEllipse { .. } => lang.t("history.ellipse"),
        Operation::MovePoint {
            merged_into: Some(_),
            ..
        } => lang.t("history.points_merged"),
        Operation::MovePoint { .. } | Operation::MoveMany { .. } => lang.t("history.move"),
        Operation::MoveSegment { .. } => lang.t("history.side_moved"),
        Operation::TurnShape { .. } => lang.t("history.turned"),
        Operation::ResizeCircle { .. }
        | Operation::ResizeArc { .. }
        | Operation::ResizeEllipse { .. } => lang.t("history.resized"),
        Operation::MoveDimension { .. } => lang.t("history.dimension_moved"),
        Operation::Constrain { constraint, .. } => constraints::label(lang, *constraint),
        Operation::EraseMany {
            elements,
            dimensions,
            constraints,
            ..
        } => erased(lang, elements, dimensions, constraints),
        Operation::MergePoints { .. } => lang.t("history.points_merged"),
        Operation::Trim { .. } => lang.t("history.trimmed"),
        Operation::TrimArc { .. } => lang.t("history.arc_trimmed"),
        Operation::TrimCircle { .. } => lang.t("history.circle_trimmed"),
        Operation::TrimEllipse { .. } => lang.t("history.ellipse_trimmed"),
        Operation::Split { .. } => lang.t("history.split"),
        Operation::Chamfer { .. } => lang.t("history.chamfer"),
        Operation::Fillet { .. } => lang.t("history.fillet"),
        Operation::CutChange(change) => cut_change::label(lang, change),
        Operation::Mirror { .. } => lang.t("history.mirrored"),
        Operation::CircularPattern { .. } => lang.t("history.circular_pattern"),
        Operation::RectangularPattern { .. } => lang.t("history.rectangular_pattern"),
        Operation::Revolve { angle, mode, .. } => lang.t_with(
            match mode {
                ExtrusionMode::Add => "history.revolution",
                ExtrusionMode::Cut => "history.revolution_cut",
            },
            &[(
                "angle",
                &formula::sized(lang, variables, angle, |turn| turn.to_string()),
            )],
        ),
        Operation::Extrude { distance, mode, .. } => lang.t_with(
            match mode {
                ExtrusionMode::Add => "history.extrusion",
                ExtrusionMode::Cut => "history.extrusion_cut",
            },
            &[(
                "distance",
                &formula::sized(lang, variables, distance, |travel| travel.to_string()),
            )],
        ),
        Operation::SetDimension { target, value, .. } => dimension::label(
            lang,
            target,
            &formula::sized(lang, variables, value, dimension::short),
        ),
    }
}

/// A change to the variables, named by the variable. An erased one keeps the
/// name it last went by.
fn variable(lang: &Catalogue, variables: &Variables, change: &VariableChange) -> String {
    match change {
        VariableChange::Added { name, .. } => {
            lang.t_with("history.variable_added", &[("name", name)])
        }
        VariableChange::Edited { name, formula, .. } => lang.t_with(
            "history.variable_edited",
            &[("name", name), ("formula", &variables.written(formula))],
        ),
        VariableChange::Erased { variable } => lang.t_with(
            "history.variable_erased",
            &[(
                "name",
                variables.get(*variable).map_or("", |found| found.name()),
            )],
        ),
    }
}

/// One thing taken away is named; several are only counted, since a line of
/// the tree has no room to list them.
fn erased(
    lang: &Catalogue,
    elements: &[Element],
    dimensions: &[DimensionTarget],
    rules: &[Constraint],
) -> String {
    match (elements, dimensions, rules) {
        ([Element::Point(_)], [], []) => lang.t("history.point_erased"),
        ([Element::Segment(_)], [], []) => lang.t("history.segment_erased"),
        ([Element::Circle(_)], [], []) => lang.t("history.circle_erased"),
        ([Element::Arc(_)], [], []) => lang.t("history.arc_erased"),
        ([Element::Ellipse(_)], [], []) => lang.t("history.ellipse_erased"),
        ([], [_], []) => lang.t("history.dimension_erased"),
        ([], [], [rule]) => constraints::erased_label(lang, *rule),
        _ => lang.t_with(
            "history.many_erased",
            &[(
                "count",
                &(elements.len() + dimensions.len() + rules.len()).to_string(),
            )],
        ),
    }
}

#[cfg(test)]
mod tests;
