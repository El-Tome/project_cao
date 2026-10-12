//! What holds a trait to one of the sketch's own axes: lying on it, or merely
//! running the way it runs.

use glam::DVec2;

use crate::constraints::{Constraint, DimensionTarget, SketchAxis};
use crate::equation::Equation;
use crate::sketch::{SegmentId, Sketch};

impl Sketch {
    /// A trait laid on one of the sketch's own axes: both its ends have to sit
    /// on that line, which is two statements.
    pub(super) fn on_axis_equations(
        &self,
        segment: SegmentId,
        axis: SketchAxis,
        into: &mut Vec<Equation>,
    ) {
        let Some(line) = self.segments().get(segment.0).copied() else {
            return;
        };
        let direction = axis.direction();
        let normal = DVec2::new(-direction.y, direction.x);

        for point in [line.start, line.end] {
            let mut equation = Equation::new(self.variables());
            equation.error = self.point(point).dot(normal);
            equation.add(point, normal);
            into.push(equation);
        }
    }

    /// A trait running the way an axis runs: its two ends stand the same
    /// distance off that axis, whatever that distance is.
    ///
    /// One equation where lying *on* the axis takes two — which is exactly the
    /// freedom the two differ by: an arm held this way can still be slid
    /// sideways, and only its direction is settled.
    pub(super) fn along_axis_equation(&self, rule: Constraint, into: &mut Vec<Equation>) {
        let Some((segment, axis)) = rule.along_an_axis() else {
            return;
        };
        let Some(line) = self.segments().get(segment.0).copied() else {
            return;
        };
        let direction = axis.direction();
        let normal = DVec2::new(-direction.y, direction.x);

        let mut equation = Equation::new(self.variables());
        equation.error = (self.point(line.end) - self.point(line.start)).dot(normal);
        equation.add(line.end, normal);
        equation.add(line.start, -normal);
        into.push(equation);
    }

    /// The angle between a segment and a fixed direction of the sketch.
    ///
    /// Unlike an angle between two segments, this one has something immovable
    /// to lean on, so it is what finally stops a drawing from spinning about
    /// its anchor.
    ///
    /// Read between the two arms the dimension names, the trait's running
    /// towards whichever of its ends it was asked to.
    pub(super) fn axis_angle_equation(
        &self,
        target: DimensionTarget,
        degrees: f64,
    ) -> Option<Equation> {
        let DimensionTarget::AxisAngle {
            segment,
            segment_toward,
            ..
        } = target
        else {
            return None;
        };
        let (_, reference, span) = self.axis_arms(target)?;
        let (tip, tail) = self.arm_ends(segment, segment_toward)?;
        let length = span.length_squared();
        if length < 1e-12 {
            return None;
        }

        let signed = reference.perp_dot(span).atan2(reference.dot(span));
        let sign = if signed < 0.0 { -1.0 } else { 1.0 };
        let turn = DVec2::new(-span.y, span.x) / length;

        let mut equation = Equation::new(self.variables());
        equation.error = signed.abs() - degrees.to_radians();
        equation.angular = true;
        equation.add(tip, turn * sign);
        equation.add(tail, -turn * sign);
        Some(equation)
    }
}
