use cao_part::history::Operation;

use crate::lang::Catalogue;
use crate::wording::history::values::{
    between, chamfer, chosen_axis, point_label, revolution_axis, rounded,
};
use crate::wording::{constraints, dimension};

/// The line shown when a history entry is unfolded.
///
/// Every value is rounded here, before it reaches a key: a language file that
/// carried `{:.1}` would be a language file only a developer could write.
pub fn detail(lang: &Catalogue, operation: &Operation) -> String {
    match operation {
        Operation::CreateSketch { plane, .. } => {
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
                ("sketch", &sketch.to_string()),
                ("x", &rounded(position.x, 1)),
                ("y", &rounded(position.y, 1)),
            ],
        ),
        Operation::AddSegment {
            sketch, start, end, ..
        } => between(lang, *sketch, start, end),
        Operation::AddSymmetricSegment {
            sketch,
            middle,
            end,
            ..
        } => lang.t_with(
            "history.detail.symmetric_segment",
            &[
                ("sketch", &sketch.to_string()),
                ("middle", &point_label(lang, middle)),
                ("end", &point_label(lang, end)),
            ],
        ),
        Operation::AddRectangle {
            sketch,
            corner,
            opposite,
            ..
        } => between(lang, *sketch, corner, opposite),
        Operation::AddCircle { sketch, radius, .. } => lang.t_with(
            "history.detail.circle",
            &[
                ("sketch", &sketch.to_string()),
                ("radius", &rounded(*radius, 2)),
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
                ("sketch", &sketch.to_string()),
                ("center", &point_label(lang, center)),
                ("start", &point_label(lang, start)),
                ("end", &point_label(lang, end)),
            ],
        ),
        Operation::MovePoint {
            sketch,
            point,
            position,
            merged_into: Some(kept),
            ..
        } => lang.t_with(
            "history.detail.point_dropped_on",
            &[
                ("sketch", &sketch.to_string()),
                ("point", &point.0.to_string()),
                ("kept", &kept.0.to_string()),
                ("x", &rounded(position.x, 1)),
                ("y", &rounded(position.y, 1)),
            ],
        ),
        Operation::MovePoint {
            sketch,
            point,
            position,
            ..
        } => lang.t_with(
            "history.detail.point_moved",
            &[
                ("sketch", &sketch.to_string()),
                ("point", &point.0.to_string()),
                ("x", &rounded(position.x, 1)),
                ("y", &rounded(position.y, 1)),
            ],
        ),
        Operation::MoveMany { sketch, points, by } => lang.t_with(
            "history.detail.many_moved",
            &[
                ("sketch", &sketch.to_string()),
                ("count", &points.len().to_string()),
                ("x", &rounded(by.x, 1)),
                ("y", &rounded(by.y, 1)),
            ],
        ),
        Operation::MoveDimension { sketch, offset, .. } => lang.t_with(
            "history.detail.dimension_moved",
            &[
                ("sketch", &sketch.to_string()),
                ("x", &rounded(offset.x, 1)),
                ("y", &rounded(offset.y, 1)),
            ],
        ),
        Operation::Extrude { sketch, areas, .. } => lang.t_with(
            "history.detail.extrusion",
            &[
                ("sketch", &sketch.to_string()),
                ("count", &areas.len().to_string()),
            ],
        ),
        Operation::Constrain { sketch, constraint } => lang.t_with(
            "history.detail.rule",
            &[
                ("sketch", &sketch.to_string()),
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
                ("sketch", &sketch.to_string()),
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
                ("sketch", &sketch.to_string()),
                ("segment", &segment.0.to_string()),
                ("from", &from.0.to_string()),
                ("to", &to.0.to_string()),
            ],
        ),
        Operation::Chamfer {
            sketch,
            first,
            second,
            mode,
        } => lang.t_with(
            "history.detail.chamfer",
            &[
                ("sketch", &sketch.to_string()),
                ("first", &first.0.to_string()),
                ("second", &second.0.to_string()),
                ("mode", &chamfer(lang, *mode)),
            ],
        ),
        Operation::Fillet {
            sketch,
            first,
            second,
            radius,
        } => lang.t_with(
            "history.detail.fillet",
            &[
                ("sketch", &sketch.to_string()),
                ("first", &first.0.to_string()),
                ("second", &second.0.to_string()),
                ("radius", &format!("{radius:.3}")),
            ],
        ),
        Operation::Mirror {
            sketch,
            elements,
            axis,
        } => lang.t_with(
            "history.detail.mirrored",
            &[
                ("sketch", &sketch.to_string()),
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
        } => lang.t_with(
            "history.detail.circular_pattern",
            &[
                ("sketch", &sketch.to_string()),
                ("elements", &elements.len().to_string()),
                ("centre", &centre.0.to_string()),
                ("degrees", &rounded(*degrees, 3)),
                ("count", &count.to_string()),
            ],
        ),
        Operation::RectangularPattern {
            sketch,
            elements,
            direction,
            along,
            across,
        } => lang.t_with(
            "history.detail.rectangular_pattern",
            &[
                ("sketch", &sketch.to_string()),
                ("elements", &elements.len().to_string()),
                ("direction", &chosen_axis(lang, *direction)),
                ("along", &along.count.to_string()),
                ("along_step", &rounded(along.step, 3)),
                ("across", &across.count.to_string()),
                ("across_step", &rounded(across.step, 3)),
            ],
        ),
        Operation::TrimArc {
            sketch,
            arc,
            from,
            to,
        } => lang.t_with(
            "history.detail.arc_trimmed",
            &[
                ("sketch", &sketch.to_string()),
                ("arc", &arc.0.to_string()),
                ("from", &from.0.to_string()),
                ("to", &to.0.to_string()),
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
                ("sketch", &sketch.to_string()),
                ("count", &(segments.len() + arcs.len()).to_string()),
                ("x", &format!("{:.3}", at.x)),
                ("y", &format!("{:.3}", at.y)),
            ],
        ),
        Operation::MergePoints {
            sketch,
            kept,
            dropped,
        } => lang.t_with(
            "history.detail.points_merged",
            &[
                ("sketch", &sketch.to_string()),
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
                ("sketch", &sketch.to_string()),
                ("count", &areas.len().to_string()),
                ("axis", &revolution_axis(lang, *axis)),
            ],
        ),
        Operation::SetDimension { sketch, target, .. } => lang.t_with(
            "history.detail.dimension",
            &[
                ("sketch", &sketch.to_string()),
                ("spans", &dimension::spans(lang, target)),
            ],
        ),
    }
}

#[cfg(test)]
mod tests;
