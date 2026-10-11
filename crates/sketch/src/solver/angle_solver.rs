//! The equations an angle between two traits asks of the solver, at a corner
//! or where they share no end, kept apart so they do not crowd out
//! `solver.rs`.

use glam::DVec2;

use crate::constraints::{DimensionTarget, Toward};
use crate::equation::Equation;
use crate::sketch::{PointId, SegmentId, Sketch};

impl Sketch {
    /// The angle at the corner two segments share, between the two arms it
    /// names: each trait's own way out of the corner, or its prolongation's.
    ///
    /// The wanted value keeps the sign the corner currently has, so asking for
    /// 30° on a corner that opens one way does not flip it to the other. An arm
    /// run along a prolongation turns with its trait all the same: only the
    /// angle read between the arms is another, not how a point moving turns it.
    pub(super) fn angle_equation(&self, target: DimensionTarget, degrees: f64) -> Option<Equation> {
        let DimensionTarget::Angle { first, second, .. } = target else {
            return None;
        };
        let (pivot, far_first, far_second) = self.shared_corner(first, second)?;
        let (_, one, other) = self.corner_arms(target)?;
        let a = self.point(far_first) - self.point(pivot);
        let b = self.point(far_second) - self.point(pivot);
        let (length_a, length_b) = (a.length_squared(), b.length_squared());
        if length_a < 1e-12 || length_b < 1e-12 {
            return None;
        }

        let signed = one.perp_dot(other).atan2(one.dot(other));
        let sign = if signed < 0.0 { -1.0 } else { 1.0 };

        // Turning a point about the pivot changes the angle by the component
        // perpendicular to its arm, scaled by how far out it sits.
        let from_first = DVec2::new(-a.y, a.x) / length_a;
        let from_second = DVec2::new(-b.y, b.x) / length_b;

        let mut equation = Equation::new(self.variables());
        equation.error = signed.abs() - degrees.to_radians();
        equation.angular = true;
        equation.add(far_second, from_second * sign);
        equation.add(far_first, -from_first * sign);
        equation.add(pivot, (from_first - from_second) * sign);
        Some(equation)
    }

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
    pub(super) fn arm_ends(
        &self,
        segment: SegmentId,
        toward: Toward,
    ) -> Option<(PointId, PointId)> {
        let drawn = *self.segments().get(segment.0)?;
        Some(match toward {
            Toward::End => (drawn.end, drawn.start),
            Toward::Start => (drawn.start, drawn.end),
        })
    }
}
