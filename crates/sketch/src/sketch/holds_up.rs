//! Whether a rule still speaks of a drawing that has it.
//!
//! Erasing a piece of the drawing takes with it every rule that named it: a
//! rule left behind would hold a number against something nobody can see, and
//! the solver would go on satisfying it. What each kind of rule needs is one
//! question — is what it names still there — asked of the kinds it names.

use super::{CircleId, Constraint, Element, PointId, SegmentId, Sketch};

impl Sketch {
    /// Whether everything a rule speaks of is still drawn.
    pub(super) fn holds_up(&self, constraint: Constraint) -> bool {
        let segment = |id: SegmentId| id.0 < self.segments.len() && !self.is_erased_segment(id);
        let circle = |id: CircleId| id.0 < self.circles.len() && !self.is_erased_circle(id);
        let point = |id: PointId| id.0 < self.points.len() && !self.is_erased_point(id);
        match constraint {
            Constraint::Perpendicular { first, second }
            | Constraint::Parallel { first, second }
            | Constraint::Equal { first, second }
            | Constraint::Collinear { first, second } => {
                first != second && segment(first) && segment(second)
            }
            Constraint::EqualRadius { first, second } => {
                first != second && circle(first) && circle(second)
            }
            Constraint::EqualRadiusArc { .. }
            | Constraint::EqualRadiusArcCircle { .. }
            | Constraint::ArcTangent { .. } => self.arc_rule_holds_up(constraint),
            Constraint::EllipseTangent {
                ellipse,
                segment: line,
                ..
            } => {
                ellipse.0 < self.ellipses().len()
                    && !self.is_erased_ellipse(ellipse)
                    && segment(line)
            }
            Constraint::OnSegment { .. }
            | Constraint::OnCircle { .. }
            | Constraint::OnArc { .. }
            | Constraint::OnEllipse { .. }
            | Constraint::OnAxis { .. } => self.hold_holds_up(constraint),
            Constraint::Midpoint {
                point: held,
                segment: on,
            } => point(held) && segment(on),
            Constraint::Tangent {
                circle: round,
                segment: line,
                ..
            } => circle(round) && segment(line),
            Constraint::AxisCollinear { segment: on, .. } => segment(on),
            Constraint::Fixed { element } => match element {
                Element::Point(held) => point(held),
                Element::Segment(held) => segment(held),
                Element::Circle(held) => circle(held),
                Element::Arc(held) => held.0 < self.arcs.len() && !self.is_erased_arc(held),
                Element::Ellipse(held) => !self.is_erased_ellipse(held),
            },
        }
    }
}
