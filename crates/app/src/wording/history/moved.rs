//! The line a step unfolds into when it moved something already drawn: a
//! point, a selection, a side, a shape turned, an annotation, or a curve drawn
//! to another size.

use cao_part::history::Operation;

use crate::lang::Catalogue;
use crate::wording::history::values::rounded;

/// The unfolded line of a step that moved something, and nothing for any
/// other step.
pub(super) fn moved(lang: &Catalogue, operation: &Operation) -> String {
    match operation {
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
        Operation::ResizeCircle {
            sketch,
            circle,
            reach,
        } => lang.t_with(
            "history.detail.circle_resized",
            &[
                ("sketch", &sketch.to_string()),
                ("circle", &circle.0.to_string()),
                ("reach", &rounded(*reach, 1)),
            ],
        ),
        Operation::ResizeEllipse {
            sketch,
            ellipse,
            reach,
        } => lang.t_with(
            "history.detail.ellipse_resized",
            &[
                ("sketch", &sketch.to_string()),
                ("ellipse", &ellipse.0.to_string()),
                ("reach", &rounded(*reach, 1)),
            ],
        ),
        Operation::ResizeArc { sketch, arc, reach } => lang.t_with(
            "history.detail.arc_resized",
            &[
                ("sketch", &sketch.to_string()),
                ("arc", &arc.0.to_string()),
                ("reach", &rounded(*reach, 1)),
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
        Operation::MoveSegment {
            sketch,
            segment,
            by,
        } => lang.t_with(
            "history.detail.side_moved",
            &[
                ("sketch", &sketch.to_string()),
                ("segment", &segment.0.to_string()),
                ("x", &rounded(by.x, 1)),
                ("y", &rounded(by.y, 1)),
            ],
        ),
        Operation::TurnShape {
            sketch,
            points,
            about,
            angle,
        } => lang.t_with(
            "history.detail.turned",
            &[
                ("sketch", &sketch.to_string()),
                ("count", &points.len().to_string()),
                ("angle", &rounded(angle.to_degrees(), 1)),
                ("x", &rounded(about.x, 1)),
                ("y", &rounded(about.y, 1)),
            ],
        ),
        _ => String::new(),
    }
}
