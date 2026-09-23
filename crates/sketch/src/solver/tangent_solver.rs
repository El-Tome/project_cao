//! What a curve brushing a line asks of the solver, kept apart so it does not
//! crowd out `solver.rs`.

use glam::DVec2;

use crate::constraints::Constraint;
use crate::ellipse::EllipseId;
use crate::equation::Equation;
use crate::sketch::{CircleId, PointId, SegmentId, Sketch};

impl Sketch {
    /// What one curve brushing a line asks of the drawing, whichever kind of
    /// curve it is.
    pub(super) fn tangent_equations(&self, rule: Constraint, into: &mut Vec<Equation>) {
        match rule {
            Constraint::Tangent {
                circle,
                segment,
                at,
            } => self.circle_tangent_equations(circle, segment, at, into),
            Constraint::ArcTangent { arc, segment, at } => {
                into.extend(self.arc_tangent_equations(arc, segment, at))
            }
            Constraint::EllipseTangent {
                ellipse,
                segment,
                at,
            } => self.ellipse_tangent_equations(ellipse, segment, at, into),
            _ => {}
        }
    }

    /// A circle brushing a line, and the point where the two touch when the
    /// drawing keeps one.
    fn circle_tangent_equations(
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

    /// An ellipse brushing a line, and the point where the two touch when the
    /// drawing keeps one.
    ///
    /// The same reckoning as a circle's, with the reach of the ellipse in the
    /// direction square to the line in place of a radius: how far an ellipse
    /// stands from its centre depends on which way one looks, so that reach
    /// moves when the line turns, and again when either axis is stretched.
    fn ellipse_tangent_equations(
        &self,
        ellipse: EllipseId,
        segment: SegmentId,
        at: Option<PointId>,
        into: &mut Vec<Equation>,
    ) {
        let Some(oval) = self.ellipses().get(ellipse.0).copied() else {
            return;
        };
        let Some(line) = self.segments().get(segment.0).copied() else {
            return;
        };
        let drawn = self.ellipse_draft(ellipse);
        let (from, to) = (self.point(line.start), self.point(line.end));
        let span = to - from;
        let length = span.length();
        if length < 1e-9 {
            return;
        }
        let across = span.perp() / length;
        let (along_first, along_second) =
            (across.dot(drawn.first), across.dot(drawn.second_axis()));
        let reach = along_first.hypot(along_second);
        if reach < 1e-9 {
            return;
        }
        let Some(mut equation) = self.on_line_equation(oval.center, segment, reach) else {
            return;
        };

        // The reach is no constant: it grows as an axis is stretched, and it
        // swings as the line turns. Both are part of the answer, the way a
        // circle's size is.
        let side = self.side_of(oval.center, segment);
        let (first, second) = (
            self.segments()[oval.first.0],
            self.segments()[oval.second.0],
        );
        let half = |value: f64| across * (value / (2.0 * reach)) * -side;
        equation.add(first.end, half(along_first));
        equation.add(first.start, -half(along_first));
        equation.add(second.end, half(along_second));
        equation.add(second.start, -half(along_second));

        // Turning the line turns the direction the reach is read in, which
        // moves the line's own two ends as surely as the ellipse does.
        let outward = (drawn.first * along_first + drawn.second_axis() * along_second) / reach;
        let along = span / length;
        let turned = (DVec2::new(outward.y, -outward.x) - along * outward.dot(across)) / length;
        equation.add(line.end, -turned * side);
        equation.add(line.start, turned * side);
        into.push(equation);

        // Where the two touch is a point of the drawing, and it is not free:
        // it lies on the line, and out from the centre the way the curve
        // reaches. Which of the two ways is the trait's own side of the
        // centre, and `ellipse_touching` is what knows it — `outward` alone
        // always leans the same way and would put the touch across the curve.
        if let Some(contact) = self.live_point(at)
            && let Some(touch) = self.ellipse_touching(ellipse, segment)
        {
            into.extend(self.on_line_equation(contact, segment, 0.0));
            let reaches = touch - self.point(oval.center);
            into.extend(self.reached_equation(contact, oval.center, segment, reaches));
        }
    }

    /// A tangency's contact held where the curve reaches out to the line,
    /// rather than square under the centre as a circle's is.
    ///
    /// Saying instead that it lies on the line *and* on the curve names the
    /// same place and is useless to a solver: at a tangency the two meet
    /// without crossing, so the pair of rows falls flat exactly where the
    /// answer is, and the touch settles to three decimals where this settles
    /// to ten.
    fn reached_equation(
        &self,
        point: PointId,
        centre: PointId,
        segment: SegmentId,
        outward: DVec2,
    ) -> Option<Equation> {
        let line = *self.segments().get(segment.0)?;
        if point.0 >= self.points().len() {
            return None;
        }
        let (a, b) = (self.point(line.start), self.point(line.end));
        let span = b - a;
        let length = span.length();
        if length < 1e-9 {
            return None;
        }
        let unit = span / length;
        let reach = self.point(point) - self.point(centre);
        let turning = (reach - unit * reach.dot(unit)) / length;

        let mut equation = Equation::new(self.variables());
        equation.error = reach.dot(unit) - outward.dot(unit);
        equation.add(point, unit);
        equation.add(centre, -unit);
        equation.add(line.end, turning);
        equation.add(line.start, -turning);
        Some(equation)
    }
}
