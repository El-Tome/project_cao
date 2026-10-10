//! Whether a value would say anything the drawing does not already hold.

use super::Sketch;
use crate::constraints::DimensionTarget;
use crate::independence::is_dependent;

impl Sketch {
    /// Whether a value on this target would say anything new.
    ///
    /// A constraint is redundant when its equation is a combination of those
    /// already there — exactly the case of a triangle's third side once its
    /// other sides and angles are fixed. Counting constraints could never see
    /// that; comparing their directions can. A fixed point decides nothing.
    pub fn would_be_redundant(&self, target: DimensionTarget, millimeters_per_unit: f64) -> bool {
        if self.dimension_of(target).is_some() {
            return false;
        }

        let existing = self.equations_pinned_by(millimeters_per_unit, &self.only_the_origin());
        let Some(candidate) = self.candidate_equation(target, millimeters_per_unit) else {
            return false;
        };
        is_dependent(&existing, &candidate)
    }

    /// The equation a not-yet-placed dimension would contribute, taken at the
    /// value the geometry already has so only its direction matters.
    fn candidate_equation(
        &self,
        target: DimensionTarget,
        millimeters_per_unit: f64,
    ) -> Option<crate::equation::Equation> {
        let value = match target {
            DimensionTarget::Length(segment) => {
                (segment.0 < self.segments.len()).then(|| self.segment_length(segment))?
                    * millimeters_per_unit.max(1e-9)
            }
            DimensionTarget::Distance { from, to } => {
                let (a, b) = (self.points.get(from.0)?, self.points.get(to.0)?);
                a.distance(*b) * millimeters_per_unit.max(1e-9)
            }
            DimensionTarget::Angle { first, second } => self.angle_between(first, second)?,
            DimensionTarget::AngleBetween { .. } | DimensionTarget::AxisAngle { .. } => {
                self.opening(target)?
            }
            DimensionTarget::PointToSegment { point, segment } => {
                self.point_to_segment(point, segment)? * millimeters_per_unit.max(1e-9)
            }
            DimensionTarget::Projected { from, to, axis } => {
                self.projected_gap(from, to, axis)? * millimeters_per_unit.max(1e-9)
            }
            DimensionTarget::Radius(circle) => {
                self.circles.get(circle.0)?.radius * millimeters_per_unit.max(1e-9)
            }
            DimensionTarget::Diameter(circle) => {
                self.circles.get(circle.0)?.radius * 2.0 * millimeters_per_unit.max(1e-9)
            }
            DimensionTarget::ArcRadius(arc) => self.arc_radius_value(arc, millimeters_per_unit)?,
            DimensionTarget::ArcSweep(arc) => self.arc_sweep_value(arc)?,
        };

        let mut probe = self.clone();
        probe.dimensions.clear();
        probe.set_dimension(target, value, false);
        let origin = probe.only_the_origin();
        probe
            .equations_pinned_by(millimeters_per_unit, &origin)
            .into_iter()
            .next()
    }
}
