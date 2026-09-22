//! How a history step reads: its name in the tree, and the line it unfolds
//! into.

mod detail;
mod values;

pub use detail::detail;

use cao_part::history::{ExtrusionMode, Operation};
use cao_sketch::{Constraint, DimensionTarget, Element};

use crate::lang::Catalogue;
use crate::wording::{constraints, dimension, plane};

/// The only place a history step is turned into a name.
///
/// Short, because the history tree shows one per line.
pub fn label(lang: &Catalogue, operation: &Operation) -> String {
    match operation {
        Operation::CreateSketch { plane, .. } => lang.t_with(
            "history.sketch",
            &[("plane", &plane::label(lang, plane.kind()))],
        ),
        Operation::AddPoint { .. } => lang.t("history.point"),
        Operation::AddSegment { .. } => lang.t("history.segment"),
        Operation::AddSymmetricSegment { .. } => lang.t("history.symmetric_segment"),
        Operation::AddRectangle { .. } => lang.t("history.rectangle"),
        Operation::AddCircle { .. } => lang.t("history.circle"),
        Operation::AddArc { .. } => lang.t("history.arc"),
        Operation::MovePoint {
            merged_into: Some(_),
            ..
        } => lang.t("history.points_merged"),
        Operation::MovePoint { .. } | Operation::MoveMany { .. } => lang.t("history.move"),
        Operation::ResizeCircle { .. } | Operation::ResizeArc { .. } => lang.t("history.resized"),
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
        Operation::Split { .. } => lang.t("history.split"),
        Operation::Chamfer { .. } => lang.t("history.chamfer"),
        Operation::Fillet { .. } => lang.t("history.fillet"),
        Operation::Mirror { .. } => lang.t("history.mirrored"),
        Operation::CircularPattern { .. } => lang.t("history.circular_pattern"),
        Operation::RectangularPattern { .. } => lang.t("history.rectangular_pattern"),
        Operation::Revolve { angle, mode, .. } => lang.t_with(
            match mode {
                ExtrusionMode::Add => "history.revolution",
                ExtrusionMode::Cut => "history.revolution_cut",
            },
            &[("angle", &angle.to_string())],
        ),
        Operation::Extrude { distance, mode, .. } => lang.t_with(
            match mode {
                ExtrusionMode::Add => "history.extrusion",
                ExtrusionMode::Cut => "history.extrusion_cut",
            },
            &[("distance", &distance.to_string())],
        ),
        Operation::SetDimension { target, value, .. } => dimension::label(lang, target, *value),
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
