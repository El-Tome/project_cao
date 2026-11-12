//! The line a step unfolds into when it moved something already drawn: a
//! point, a selection, a side, a shape turned, an annotation, or a curve drawn
//! to another size.

use cao_part::history::Operation;

use crate::lang::Catalogue;
use crate::wording::dimension::short;

/// The unfolded line of a step that moved something, and nothing for any
/// other step.
pub(super) fn moved(
    lang: &Catalogue,
    operation: &Operation,
    number: &dyn Fn(usize) -> u32,
) -> String {
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
                ("sketch", &number(*sketch).to_string()),
                ("point", &point.0.to_string()),
                ("kept", &kept.0.to_string()),
                ("x", &short(position.x)),
                ("y", &short(position.y)),
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
                ("sketch", &number(*sketch).to_string()),
                ("point", &point.0.to_string()),
                ("x", &short(position.x)),
                ("y", &short(position.y)),
            ],
        ),
        Operation::ResizeCircle {
            sketch,
            circle,
            reach,
        } => lang.t_with(
            "history.detail.circle_resized",
            &[
                ("sketch", &number(*sketch).to_string()),
                ("circle", &circle.0.to_string()),
                ("reach", &short(*reach)),
            ],
        ),
        Operation::ResizeEllipse {
            sketch,
            ellipse,
            reach,
        } => lang.t_with(
            "history.detail.ellipse_resized",
            &[
                ("sketch", &number(*sketch).to_string()),
                ("ellipse", &ellipse.0.to_string()),
                ("reach", &short(*reach)),
            ],
        ),
        Operation::ResizeArc { sketch, arc, reach } => lang.t_with(
            "history.detail.arc_resized",
            &[
                ("sketch", &number(*sketch).to_string()),
                ("arc", &arc.0.to_string()),
                ("reach", &short(*reach)),
            ],
        ),
        Operation::MoveMany { sketch, points, by } => lang.t_with(
            "history.detail.many_moved",
            &[
                ("sketch", &number(*sketch).to_string()),
                ("count", &points.len().to_string()),
                ("x", &short(by.x)),
                ("y", &short(by.y)),
            ],
        ),
        Operation::MoveDimension { sketch, offset, .. } => lang.t_with(
            "history.detail.dimension_moved",
            &[
                ("sketch", &number(*sketch).to_string()),
                ("x", &short(offset.x)),
                ("y", &short(offset.y)),
            ],
        ),
        Operation::MoveSegment {
            sketch,
            segment,
            by,
        } => lang.t_with(
            "history.detail.side_moved",
            &[
                ("sketch", &number(*sketch).to_string()),
                ("segment", &segment.0.to_string()),
                ("x", &short(by.x)),
                ("y", &short(by.y)),
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
                ("sketch", &number(*sketch).to_string()),
                ("count", &points.len().to_string()),
                ("angle", &short(angle.to_degrees())),
                ("x", &short(about.x)),
                ("y", &short(about.y)),
            ],
        ),
        _ => String::new(),
    }
}
