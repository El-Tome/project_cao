//! What holds a trait to one of the sketch's own axes: lying on it, or merely
//! running the way it runs.

use glam::DVec2;

use crate::constraints::SketchAxis;
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
    pub(super) fn along_axis_equation(
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

        let mut equation = Equation::new(self.variables());
        equation.error = (self.point(line.end) - self.point(line.start)).dot(normal);
        equation.add(line.end, normal);
        equation.add(line.start, -normal);
        into.push(equation);
    }
}
