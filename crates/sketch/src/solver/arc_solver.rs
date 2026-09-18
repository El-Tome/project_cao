//! The equations an arc's own rules and dimensions ask of the solver, kept
//! apart from the rest so they do not crowd out `solver.rs`.

use glam::DVec2;

use crate::arc::ArcId;
use crate::constraints::{Constraint, Dimension, DimensionTarget};
use crate::equation::Equation;
use crate::sketch::{CircleId, PointId, SegmentId, Sketch};

impl Sketch {
    /// Two circles, an arc and a circle, or two arcs, held to the same
    /// radius — whichever pair a circle's own `EqualRadius` now reaches.
    pub(super) fn equal_radius_equations(&self, constraint: Constraint) -> Vec<Equation> {
        match constraint {
            Constraint::EqualRadius { first, second } => self
                .equal_radius_circles(first, second)
                .into_iter()
                .collect(),
            Constraint::EqualRadiusArc { first, second } => self
                .equal_radius_arc_equation(first, second)
                .into_iter()
                .collect(),
            _ => Vec::new(),
        }
    }

    fn equal_radius_circles(&self, first: CircleId, second: CircleId) -> Option<Equation> {
        let (one, other) = (self.radius_column(first)?, self.radius_column(second)?);
        let mut equation = Equation::new(self.variables());
        equation.error = self.circles()[second.0].radius - self.circles()[first.0].radius;
        equation.add_radius(other, 1.0);
        equation.add_radius(one, -1.0);
        Some(equation)
    }

    /// Either of the two dimensions an arc alone can carry, `scale` already
    /// resolved to what a millimetre is worth.
    pub(super) fn arc_dimension_equation(
        &self,
        dimension: Dimension,
        scale: f64,
    ) -> Option<Equation> {
        match dimension.target {
            DimensionTarget::ArcRadius(arc) => {
                self.arc_radius_equation(arc, dimension.value / scale)
            }
            DimensionTarget::ArcSweep(arc) => self.arc_sweep_equation(arc, dimension.value),
            _ => None,
        }
    }

    /// An arc brushing a line: the same shape as a circle's tangency, with
    /// the radius read off the arc's start instead of a column of its own.
    pub(super) fn arc_tangent_equations(
        &self,
        arc: ArcId,
        segment: SegmentId,
        at: Option<PointId>,
    ) -> Vec<Equation> {
        let mut equations = Vec::new();
        let Some(drawn) = self.arcs().get(arc.0).copied() else {
            return equations;
        };
        let radius = self.arc_radius(arc);
        let Some(mut equation) = self.on_line_equation(drawn.center, segment, radius) else {
            return equations;
        };
        // Growing the arc — moving its start out from the centre — closes
        // the gap just as surely as moving the centre does.
        let reach = self.point(drawn.start) - self.point(drawn.center);
        let length = reach.length();
        if length > 1e-9 {
            let unit = reach / length;
            let sign = -self.side_of(drawn.center, segment);
            equation.add(drawn.start, unit * sign);
            equation.add(drawn.center, -unit * sign);
        }
        equations.push(equation);
        if let Some(contact) = self.live_point(at) {
            equations.extend(self.on_line_equation(contact, segment, 0.0));
            equations.extend(self.foot_equation(contact, drawn.center, segment));
        }
        equations
    }

    /// Two arcs held to the same reach from their own centres.
    pub(super) fn equal_radius_arc_equation(
        &self,
        first: ArcId,
        second: ArcId,
    ) -> Option<Equation> {
        let (one, other) = (*self.arcs().get(first.0)?, *self.arcs().get(second.0)?);
        let (one_reach, other_reach) = (
            self.point(one.start) - self.point(one.center),
            self.point(other.start) - self.point(other.center),
        );
        let (one_length, other_length) = (one_reach.length(), other_reach.length());
        if one_length < 1e-9 || other_length < 1e-9 {
            return None;
        }
        let (one_unit, other_unit) = (one_reach / one_length, other_reach / other_length);

        let mut equation = Equation::new(self.variables());
        equation.error = other_length - one_length;
        equation.add(other.start, other_unit);
        equation.add(other.center, -other_unit);
        equation.add(one.start, -one_unit);
        equation.add(one.center, one_unit);
        Some(equation)
    }

    /// An arc told how big to be: the same length equation its own roundness
    /// already reads the radius off, since an arc keeps no size of its own.
    pub(super) fn arc_radius_equation(&self, id: ArcId, target: f64) -> Option<Equation> {
        let arc = *self.arcs().get(id.0)?;
        self.length_equation(arc.center, arc.start, target)
    }

    /// An arc told how far round it should run.
    ///
    /// Unlike the angle between two segments, an arc's sweep is never folded
    /// to the smaller way round: a fillet can honestly ask for 300 degrees.
    pub(super) fn arc_sweep_equation(&self, id: ArcId, degrees: f64) -> Option<Equation> {
        let arc = *self.arcs().get(id.0)?;
        let centre = self.point(arc.center);
        let (from, to) = (self.point(arc.start) - centre, self.point(arc.end) - centre);
        let (length_from, length_to) = (from.length_squared(), to.length_squared());
        if length_from < 1e-12 || length_to < 1e-12 {
            return None;
        }
        let sweep = (to.to_angle() - from.to_angle()).rem_euclid(std::f64::consts::TAU);

        let from_start = DVec2::new(-from.y, from.x) / length_from;
        let from_end = DVec2::new(-to.y, to.x) / length_to;

        let mut equation = Equation::new(self.variables());
        equation.error = sweep - degrees.to_radians();
        equation.angular = true;
        equation.add(arc.end, from_end);
        equation.add(arc.start, -from_start);
        equation.add(arc.center, from_start - from_end);
        Some(equation)
    }
}

#[cfg(test)]
mod tests;
