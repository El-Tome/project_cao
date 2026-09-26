//! How far a point stands from the line of a trait.

use glam::DVec2;

use crate::equation::Equation;
use crate::sketch::{PointId, SegmentId, Sketch};

impl Sketch {
    /// The distance from a point to the line two other points define.
    ///
    /// The distance is the cross product of the line's span with the reach to
    /// the point, over that span's length; everything below is that quotient
    /// differentiated, which is why the line's own ends move too — a drawing
    /// where only the point could answer would tilt the line instead.
    pub(super) fn point_to_segment_equation(
        &self,
        point: PointId,
        segment: SegmentId,
        target: f64,
    ) -> Option<Equation> {
        let segment = *self.segments().get(segment.0)?;
        let (a, b) = (self.point(segment.start), self.point(segment.end));
        let span = b - a;
        let length = span.length();
        if length < 1e-9 {
            return None;
        }
        let reach = self.point(point) - a;
        let cross = span.perp_dot(reach);
        let distance = cross / length;
        if distance.abs() < 1e-9 {
            return None;
        }

        let d_cross_point = DVec2::new(-span.y, span.x);
        let d_cross_start = DVec2::new(span.y - reach.y, reach.x - span.x);
        let d_cross_end = DVec2::new(reach.y, -reach.x);
        let unit = span / length;
        let gradient = |d_cross: DVec2, d_length: DVec2| (d_cross - d_length * distance) / length;

        let sign = if distance < 0.0 { -1.0 } else { 1.0 };
        let mut equation = Equation::new(self.variables());
        equation.error = distance.abs() - target;
        equation.add(point, gradient(d_cross_point, DVec2::ZERO) * sign);
        equation.add(segment.start, gradient(d_cross_start, -unit) * sign);
        equation.add(segment.end, gradient(d_cross_end, unit) * sign);
        Some(equation)
    }
}
