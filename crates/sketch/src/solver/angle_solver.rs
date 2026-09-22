//! The equation an angle between two traits that share no end asks of the
//! solver, kept apart so it does not crowd out `solver.rs`.

use glam::DVec2;

use crate::constraints::{DimensionTarget, Toward};
use crate::equation::Equation;
use crate::sketch::{PointId, SegmentId, Sketch};

impl Sketch {
    /// Two traits held at an angle to each other, with no shared end to turn
    /// them about.
    ///
    /// The same reckoning as a corner's, with each arm read off its own trait
    /// rather than out from a common pivot: an arm is the trait's direction,
    /// run the way the dimension names, so turning it moves both of the
    /// trait's ends, in opposite senses.
    pub(super) fn angle_between_equation(
        &self,
        target: DimensionTarget,
        degrees: f64,
    ) -> Option<Equation> {
        let DimensionTarget::AngleBetween {
            first,
            first_toward,
            second,
            second_toward,
        } = target
        else {
            return None;
        };
        let (head_first, tail_first) = self.arm_ends(first, first_toward)?;
        let (head_second, tail_second) = self.arm_ends(second, second_toward)?;
        let a = self.point(head_first) - self.point(tail_first);
        let b = self.point(head_second) - self.point(tail_second);
        let (length_a, length_b) = (a.length_squared(), b.length_squared());
        if length_a < 1e-12 || length_b < 1e-12 {
            return None;
        }

        let signed = a.perp_dot(b).atan2(a.dot(b));
        let sign = if signed < 0.0 { -1.0 } else { 1.0 };

        // How the angle answers a push square to each arm, scaled by how long
        // that arm is — the same derivative a corner's arms have.
        let across_a = DVec2::new(-a.y, a.x) / length_a;
        let across_b = DVec2::new(-b.y, b.x) / length_b;

        let mut equation = Equation::new(self.variables());
        equation.error = signed.abs() - degrees.to_radians();
        equation.angular = true;
        equation.add(head_first, -across_a * sign);
        equation.add(tail_first, across_a * sign);
        equation.add(head_second, across_b * sign);
        equation.add(tail_second, -across_b * sign);
        Some(equation)
    }

    /// The end an arm runs towards along its trait, and the end it runs from.
    fn arm_ends(&self, segment: SegmentId, toward: Toward) -> Option<(PointId, PointId)> {
        let drawn = *self.segments().get(segment.0)?;
        Some(match toward {
            Toward::End => (drawn.end, drawn.start),
            Toward::Start => (drawn.start, drawn.end),
        })
    }
}
