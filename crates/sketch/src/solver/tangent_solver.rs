//! What a circle brushing a line asks of the solver, kept apart so it does not
//! crowd out `solver.rs`.

use crate::equation::Equation;
use crate::sketch::{CircleId, PointId, SegmentId, Sketch};

impl Sketch {
    /// A circle brushing a line, and the point where the two touch when the
    /// drawing keeps one.
    pub(super) fn circle_tangent_equations(
        &self,
        circle: CircleId,
        segment: SegmentId,
        at: Option<PointId>,
        into: &mut Vec<Equation>,
    ) {
        let Some(round) = self.circles().get(circle.0).copied() else {
            return;
        };
        let Some(mut equation) = self.on_line_equation(round.center, segment, round.radius) else {
            return;
        };
        // Growing the circle closes the gap just as surely as moving
        // it does, so the size is part of the answer — outwards or
        // inwards according to the side of the line the circle is on.
        if let Some(column) = self.radius_column(circle) {
            equation.add_radius(column, -self.side_of(round.center, segment));
        }
        into.push(equation);
        // Where the two touch is a point of the drawing, and it is not
        // free: it lies on the line, square under the centre. Without
        // that second half it would slide along the line, since sliding
        // a point along a circle it touches changes nothing at all to
        // first order.
        if let Some(contact) = self.live_point(at) {
            into.extend(self.on_line_equation(contact, segment, 0.0));
            into.extend(self.foot_equation(contact, round.center, segment));
        }
    }
}
