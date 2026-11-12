use cao_part::history::Operation;
use cao_part::{PartDocument, VariableChange, Variables};

use crate::lang::Catalogue;
use crate::wording::history::moved::moved;
use crate::wording::history::sized;
use crate::wording::history::values::{
    between, chosen_axis, point_label, revolution_axis, rounded,
};
use crate::wording::{constraints, dimension};

/// The line shown when a history entry of that part is unfolded, a sketch
/// named by the number the part gave it.
///
/// Every value is rounded here, before it reaches a key: a language file that
/// carried `{:.1}` would be a language file only a developer could write.
pub fn detail_in(lang: &Catalogue, document: &PartDocument, operation: &Operation) -> String {
    let number = |sketch| crate::wording::sketch::number(document, sketch);
    unfolded(lang, document.variables(), operation, &number)
}

fn unfolded(
    lang: &Catalogue,
    variables: &Variables,
    operation: &Operation,
    number: &dyn Fn(usize) -> u32,
) -> String {
    match operation {
        Operation::Gesture(done) => done
            .first()
            .map(|first| unfolded(lang, variables, first, number))
            .unwrap_or_default(),
        Operation::Variable(
            VariableChange::Added { name, formula } | VariableChange::Edited { name, formula, .. },
        ) => lang.t_with(
            "history.detail.variable",
            &[
                ("name", name),
                ("formula", &sized::formula(lang, variables, formula)),
            ],
        ),
        Operation::Variable(VariableChange::Erased { variable }) => variables
            .get(*variable)
            .map(|found| found.name().to_string())
            .unwrap_or_default(),
        Operation::CreateSketch { plane, .. } | Operation::CreateSketchOnSection { plane, .. } => {
            let normal = plane.normal();
            lang.t_with(
                "history.detail.plane",
                &[
                    ("x", &rounded(normal.x, 0)),
                    ("y", &rounded(normal.y, 0)),
                    ("z", &rounded(normal.z, 0)),
                ],
            )
        }
        Operation::AddPoint {
            sketch, position, ..
        } => lang.t_with(
            "history.detail.point",
            &[
                ("sketch", &number(*sketch).to_string()),
                ("x", &dimension::short(position.x)),
                ("y", &dimension::short(position.y)),
            ],
        ),
        Operation::AddSegment {
            sketch, start, end, ..
        } => between(lang, number(*sketch), start, end),
        Operation::AddSymmetricSegment {
            sketch,
            middle,
            end,
            ..
        } => lang.t_with(
            "history.detail.symmetric_segment",
            &[
                ("sketch", &number(*sketch).to_string()),
                ("middle", &point_label(lang, middle)),
                ("end", &point_label(lang, end)),
            ],
        ),
        Operation::AddRectangle {
            sketch,
            corner,
            opposite,
            ..
        } => between(lang, number(*sketch), corner, opposite),
        Operation::AddCircle { sketch, radius, .. } => lang.t_with(
            "history.detail.circle",
            &[
                ("sketch", &number(*sketch).to_string()),
                ("radius", &dimension::short(*radius)),
            ],
        ),
        Operation::AddArc {
            sketch,
            center,
            start,
            end,
            ..
        } => lang.t_with(
            "history.detail.arc",
            &[
                ("sketch", &number(*sketch).to_string()),
                ("center", &point_label(lang, center)),
                ("start", &point_label(lang, start)),
                ("end", &point_label(lang, end)),
            ],
        ),
        Operation::MovePoint { .. }
        | Operation::ResizeCircle { .. }
        | Operation::ResizeArc { .. }
        | Operation::ResizeEllipse { .. }
        | Operation::MoveMany { .. }
        | Operation::MoveSegment { .. }
        | Operation::TurnShape { .. }
        | Operation::MoveDimension { .. } => moved(lang, operation, number),
        Operation::AddEllipse {
            sketch,
            center,
            first,
            ..
        } => lang.t_with(
            "history.detail.ellipse",
            &[
                ("sketch", &number(*sketch).to_string()),
                ("center", &point_label(lang, center)),
                ("end", &point_label(lang, &first[1])),
            ],
        ),
        Operation::Extrude { sketch, areas, .. } => lang.t_with(
            "history.detail.extrusion",
            &[
                ("sketch", &number(*sketch).to_string()),
                ("count", &areas.len().to_string()),
            ],
        ),
        Operation::Constrain { sketch, constraint } => lang.t_with(
            "history.detail.rule",
            &[
                ("sketch", &number(*sketch).to_string()),
                ("rule", &constraints::label(lang, *constraint)),
            ],
        ),
        Operation::EraseMany {
            sketch,
            elements,
            dimensions,
            constraints,
        } => lang.t_with(
            "history.detail.erased",
            &[
                ("sketch", &number(*sketch).to_string()),
                ("elements", &elements.len().to_string()),
                ("dimensions", &dimensions.len().to_string()),
                ("constraints", &constraints.len().to_string()),
            ],
        ),
        Operation::Trim {
            sketch,
            segment,
            from,
            to,
        } => lang.t_with(
            "history.detail.trimmed",
            &[
                ("sketch", &number(*sketch).to_string()),
                ("segment", &segment.0.to_string()),
                ("from", &from.0.to_string()),
                ("to", &to.0.to_string()),
            ],
        ),
        Operation::Chamfer {
            sketch,
            corners,
            mode,
            ..
        } => sized::chamfer(lang, variables, number(*sketch), corners.len(), mode),
        Operation::Fillet {
            sketch,
            corners,
            radius,
            ..
        } => sized::fillet(lang, variables, number(*sketch), corners.len(), radius),
        Operation::CutChange(change) => super::cut_change::detail(lang, variables, change, number),
        Operation::EditMatter { to, .. } => super::label(lang, variables, to),
        Operation::SaveSection(_)
        | Operation::EditSection { .. }
        | Operation::DeleteSection { .. } => String::new(),
        Operation::Mirror {
            sketch,
            elements,
            axis,
        } => lang.t_with(
            "history.detail.mirrored",
            &[
                ("sketch", &number(*sketch).to_string()),
                ("elements", &elements.len().to_string()),
                ("axis", &chosen_axis(lang, *axis)),
            ],
        ),
        Operation::CircularPattern {
            sketch,
            elements,
            centre,
            degrees,
            count,
        } => sized::circular_pattern(
            lang,
            variables,
            (number(*sketch), elements.len()),
            *centre,
            degrees,
            count,
        ),
        Operation::RectangularPattern {
            sketch,
            elements,
            direction,
            along,
            across,
        } => sized::rectangular_pattern(
            lang,
            variables,
            (number(*sketch), elements.len()),
            *direction,
            along,
            across,
        ),
        Operation::TrimArc {
            sketch,
            arc,
            from,
            to,
        } => lang.t_with(
            "history.detail.arc_trimmed",
            &[
                ("sketch", &number(*sketch).to_string()),
                ("arc", &arc.0.to_string()),
                ("from", &from.0.to_string()),
                ("to", &to.0.to_string()),
            ],
        ),
        Operation::TrimEllipse {
            sketch,
            ellipse,
            between: Some((from, to)),
        } => lang.t_with(
            "history.detail.ellipse_trimmed",
            &[
                ("sketch", &number(*sketch).to_string()),
                ("ellipse", &ellipse.0.to_string()),
                ("from", &from.0.to_string()),
                ("to", &to.0.to_string()),
            ],
        ),
        Operation::TrimEllipse {
            sketch,
            ellipse,
            between: None,
        } => lang.t_with(
            "history.detail.ellipse_taken_whole",
            &[
                ("sketch", &number(*sketch).to_string()),
                ("ellipse", &ellipse.0.to_string()),
            ],
        ),
        Operation::TrimCircle {
            sketch,
            circle,
            between: Some((from, to)),
        } => lang.t_with(
            "history.detail.circle_trimmed",
            &[
                ("sketch", &number(*sketch).to_string()),
                ("circle", &circle.0.to_string()),
                ("from", &from.0.to_string()),
                ("to", &to.0.to_string()),
            ],
        ),
        Operation::TrimCircle {
            sketch,
            circle,
            between: None,
        } => lang.t_with(
            "history.detail.circle_taken_whole",
            &[
                ("sketch", &number(*sketch).to_string()),
                ("circle", &circle.0.to_string()),
            ],
        ),
        Operation::Split {
            sketch,
            segments,
            arcs,
            at,
        } => lang.t_with(
            "history.detail.split",
            &[
                ("sketch", &number(*sketch).to_string()),
                ("count", &(segments.len() + arcs.len()).to_string()),
                ("x", &dimension::short(at.x)),
                ("y", &dimension::short(at.y)),
            ],
        ),
        Operation::MergePoints {
            sketch,
            kept,
            dropped,
        } => lang.t_with(
            "history.detail.points_merged",
            &[
                ("sketch", &number(*sketch).to_string()),
                ("kept", &kept.0.to_string()),
                ("dropped", &dropped.0.to_string()),
            ],
        ),
        Operation::Revolve {
            sketch,
            areas,
            axis,
            ..
        } => lang.t_with(
            "history.detail.revolution",
            &[
                ("sketch", &number(*sketch).to_string()),
                ("count", &areas.len().to_string()),
                ("axis", &revolution_axis(lang, *axis)),
            ],
        ),
        Operation::SetDimension { sketch, target, .. } => lang.t_with(
            "history.detail.dimension",
            &[
                ("sketch", &number(*sketch).to_string()),
                ("spans", &dimension::spans(lang, target)),
            ],
        ),
    }
}

#[cfg(test)]
mod tests;
