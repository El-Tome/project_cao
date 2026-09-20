//! The equations a rule holding a point asks of the solver, kept apart from
//! the rest so they do not crowd out `solver.rs`.

use glam::DVec2;

use crate::arc::ArcId;
use crate::constraints::{Constraint, SketchAxis};
use crate::equation::{Equation, Row};
use crate::holding::Support;
use crate::sketch::{CircleId, PointId, Sketch};

impl Sketch {
    /// What a rule holding a point asks of the drawing.
    ///
    /// The whole truth, the curve's own coordinates included: this is the
    /// system the drawing is *read* from — what is settled, what is still
    /// free, whether a value would say anything new — and a row that left the
    /// curve out would answer those questions about a drawing nobody drew.
    ///
    /// Which of the two gives is another question, and it is asked at the
    /// moment of the correction: see [`Sketch::held_alone`].
    pub(super) fn hold_equations(&self, constraint: Constraint, into: &mut Vec<Equation>) {
        match constraint {
            Constraint::OnSegment { point, segment } => {
                into.extend(self.on_line_equation(point, segment, 0.0))
            }
            Constraint::OnCircle { point, circle } => into.extend(self.rim_equation(point, circle)),
            Constraint::OnArc { point, arc } => into.extend(self.on_arc_equation(point, arc)),
            Constraint::OnAxis { point, axis } => into.extend(self.on_axis_equation(point, axis)),
            _ => {}
        }
    }

    /// The point a row of the system is to correct on its own, when that row
    /// is a rule holding one.
    ///
    /// A point laid on a curve follows it and does not reshape what it was
    /// laid on: the step the solver takes for that row moves the point and
    /// leaves the curve where it is. Without that, dragging one end of a
    /// trait a point sits on pulls the far end out of true — the correction
    /// is shared out, as it is for a rule between two traits, and a hold is
    /// not that kind of rule.
    ///
    /// Two points do not follow on their own, and for them the curve is what
    /// is left to move: one that cannot give at all — pinned, or held under
    /// the cursor — and one that something else is already pulling on. A rim
    /// point at the end of a trait whose length is typed is the second: hold
    /// the circle still there and neither the length nor the rim can be
    /// satisfied, where letting the circle grow settles both.
    ///
    /// Only the step is trimmed. The row itself keeps the curve's own
    /// coordinates, since it is also what the drawing is read from.
    pub(super) fn held_alone(
        &self,
        index: usize,
        pinned: &[bool],
        pulled: &[bool],
    ) -> Option<PointId> {
        let point = self.held_by_a_rule(index)?;
        let follows = !pinned.get(point.0).copied().unwrap_or(false)
            && !pulled.get(point.0).copied().unwrap_or(false);
        follows.then_some(point)
    }

    /// The point a row holds, when the row is a rule holding one.
    fn held_by_a_rule(&self, index: usize) -> Option<PointId> {
        let Row::Rule(at) = self.row(index) else {
            return None;
        };
        Support::held_by(*self.constraints().get(at)?).map(|(point, _)| point)
    }

    /// Which points something other than a rule holding them pulls on.
    ///
    /// Read once per sweep rather than per correction: which rows speak of
    /// which points does not change while the drawing settles, only how far
    /// off they are.
    pub(super) fn pulled_elsewhere(&self, millimeters_per_unit: f64, pinned: &[bool]) -> Vec<bool> {
        let mut pulled = vec![false; self.points().len()];
        let mut entry: Vec<Equation> = Vec::new();
        for index in 0..self.equation_count() {
            if self.held_by_a_rule(index).is_some() {
                continue;
            }
            self.any_equation(index, millimeters_per_unit, pinned, &mut entry);
            for equation in entry.drain(..) {
                for (point, pulled) in pulled.iter_mut().enumerate() {
                    *pulled |= equation.gradient[point * 2].abs() > 1e-12
                        || equation.gradient[point * 2 + 1].abs() > 1e-12;
                }
                equation.recycle();
            }
        }
        pulled
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
