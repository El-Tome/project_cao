//! A rule with the point that goes replaced by the point that stays.

use crate::constraints::Constraint;
use crate::sketch::{Element, PointId};

/// The same rule, every point it names handed over. Every kind is named, so
/// that a kind added later is not forgotten here.
pub(super) fn handed_over(rule: Constraint, one: impl Fn(PointId) -> PointId) -> Constraint {
    match rule {
        Constraint::OnSegment {
            point,
            segment,
            from,
        } => Constraint::OnSegment {
            point: one(point),
            segment,
            from,
        },
        Constraint::OnCircle {
            point,
            circle,
            from,
        } => Constraint::OnCircle {
            point: one(point),
            circle,
            from,
        },
        Constraint::OnArc { point, arc, from } => Constraint::OnArc {
            point: one(point),
            arc,
            from,
        },
        Constraint::OnEllipse {
            point,
            ellipse,
            from,
        } => Constraint::OnEllipse {
            point: one(point),
            ellipse,
            from,
        },
        Constraint::OnAxis { point, axis } => Constraint::OnAxis {
            point: one(point),
            axis,
        },
        Constraint::Midpoint { point, segment } => Constraint::Midpoint {
            point: one(point),
            segment,
        },
        Constraint::Fixed {
            element: Element::Point(point),
        } => Constraint::Fixed {
            element: Element::Point(one(point)),
        },
        Constraint::Tangent {
            circle,
            segment,
            at,
            from,
        } => Constraint::Tangent {
            circle,
            segment,
            at: at.map(&one),
            from,
        },
        Constraint::ArcTangent {
            arc,
            segment,
            at,
            from,
        } => Constraint::ArcTangent {
            arc,
            segment,
            at: at.map(&one),
            from,
        },
        Constraint::EllipseTangent {
            ellipse,
            segment,
            at,
            from,
        } => Constraint::EllipseTangent {
            ellipse,
            segment,
            at: at.map(&one),
            from,
        },
        Constraint::Fixed { .. }
        | Constraint::Perpendicular { .. }
        | Constraint::Parallel { .. }
        | Constraint::Equal { .. }
        | Constraint::EqualRadius { .. }
        | Constraint::EqualRadiusArc { .. }
        | Constraint::EqualRadiusArcCircle { .. }
        | Constraint::Collinear { .. }
        | Constraint::AxisCollinear { .. }
        | Constraint::AxisParallel { .. }
        | Constraint::AxisPerpendicular { .. } => rule,
    }
}
