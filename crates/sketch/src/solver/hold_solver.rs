//! The equations a rule holding a point asks of the solver, kept apart from
//! the rest so they do not crowd out `solver.rs`.

use glam::DVec2;

use crate::arc::ArcId;
use crate::constraints::{Constraint, SketchAxis};
use crate::equation::Equation;
use crate::sketch::{CircleId, PointId, Sketch};

impl Sketch {
    /// What a rule holding a point asks of the drawing.
    ///
    /// The correction falls on the point alone: a point laid on a curve
    /// follows it, and does not reshape what it was laid on. Unless the point
    /// itself cannot give — pinned, or held under the cursor — and then the
    /// curve is what is left to move, which is how a circle is still resized
    /// by dragging a point of its rim.
    pub(super) fn hold_equations(
        &self,
        constraint: Constraint,
        pinned: &[bool],
        into: &mut Vec<Equation>,
    ) {
        let written = into.len();
        let held = match constraint {
            Constraint::OnSegment { point, segment } => {
                into.extend(self.on_line_equation(point, segment, 0.0));
                point
            }
            Constraint::OnCircle { point, circle } => {
                into.extend(self.rim_equation(point, circle));
                point
            }
            Constraint::OnArc { point, arc } => {
                into.extend(self.on_arc_equation(point, arc));
                point
            }
            Constraint::OnAxis { point, axis } => {
                into.extend(self.on_axis_equation(point, axis));
                point
            }
            _ => return,
        };
        if pinned.get(held.0).copied().unwrap_or(false) {
            return;
        }
        for equation in &mut into[written..] {
            equation.hold_to(held);
        }
    }

    /// A point held on one of the sketch's own axes: no distance at all
    /// across it.
    fn on_axis_equation(&self, point: PointId, axis: SketchAxis) -> Option<Equation> {
        let place = *self.points().get(point.0)?;
        let along = axis.direction();
        let across = DVec2::new(-along.y, along.x);

        let mut equation = Equation::new(self.variables());
        equation.error = place.dot(across);
        equation.add(point, across);
        Some(equation)
    }

    /// A point held on a circle's rim. The size gives as readily as the place:
    /// a rim held still is how a circle is resized by hand.
    fn rim_equation(&self, point: PointId, circle: CircleId) -> Option<Equation> {
        let round = *self.circles().get(circle.0)?;
        if point.0 >= self.points().len() {
            return None;
        }
        let reach = self.point(point) - self.point(round.center);
        let length = reach.length();
        if length < 1e-9 {
            return None;
        }
        let unit = reach / length;

        let mut equation = Equation::new(self.variables());
        equation.error = length - round.radius;
        equation.add(point, unit);
        equation.add(round.center, -unit);
        if let Some(column) = self.radius_column(circle) {
            equation.add_radius(column, -1.0);
        }
        Some(equation)
    }

    /// A point held on the circle an arc is a piece of: as far from the centre
    /// as the end the curve starts at, which is where an arc keeps its size.
    pub(crate) fn on_arc_equation(&self, point: PointId, arc: ArcId) -> Option<Equation> {
        let curve = *self.arcs().get(arc.0)?;
        if point.0 >= self.points().len() {
            return None;
        }
        let centre = self.point(curve.center);
        let (out, along) = (self.point(point) - centre, self.point(curve.start) - centre);
        let (reach, radius) = (out.length(), along.length());
        if reach < 1e-9 || radius < 1e-9 {
            return None;
        }
        let (out, along) = (out / reach, along / radius);

        let mut equation = Equation::new(self.variables());
        equation.error = reach - radius;
        equation.add(point, out);
        equation.add(curve.start, -along);
        equation.add(curve.center, along - out);
        Some(equation)
    }
}
