//! How a drawing settles when something is moved or a value is typed.
//!
//! Dragging a corner is not a matter of putting one point somewhere: every
//! rule the drawing carries has to hold afterwards, and the rest of the shape
//! moves to make that true. The point under the cursor is the one thing that
//! does not give, which is what makes a shape follow the mouse instead of
//! squirming away from it.
//!
//! Which points can no longer move at all, once it has settled, is
//! `settled.rs`.

use glam::DVec2;

use crate::constraints::DimensionTarget;
use crate::independence::is_dependent;
use crate::length::LengthOutcome;
use crate::solver::SolveOutcome;

use super::{PointId, Sketch};

impl Sketch {
    /// Whether a value on this target would say anything new.
    ///
    /// A constraint is redundant when its equation is a combination of those
    /// already there — exactly the case of a triangle's third side once its
    /// other sides and angles are fixed. Counting constraints could never see
    /// that; comparing their directions can.
    pub fn would_be_redundant(&self, target: DimensionTarget, millimeters_per_unit: f64) -> bool {
        if self.dimension_of(target).is_some() {
            return false;
        }

        let existing = self.equations(millimeters_per_unit);
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
            DimensionTarget::AxisAngle { segment, axis } => self.angle_with_axis(segment, axis)?,
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
        probe.equations(millimeters_per_unit).into_iter().next()
    }

    /// Puts a point where it was dropped and settles the rest of the drawing
    /// around it, that point staying exactly where it was put.
    pub fn settle_around(
        &mut self,
        point: PointId,
        position: DVec2,
        millimeters_per_unit: f64,
    ) -> LengthOutcome {
        self.settle_around_all(&[(point, position)], millimeters_per_unit)
    }

    /// The same for a whole handful of points dropped at once, which is how a
    /// selection is moved in one block.
    ///
    /// Held, they do not give: the drawing settles around them rather than
    /// pulling them back, so a shape follows the mouse instead of squirming
    /// away from it.
    ///
    /// When holding them is more than the drawing can bear — a corner dragged
    /// somewhere no tangency can reach it — the values already given win over
    /// the cursor: everything goes back and settles the ordinary way. Leaving
    /// the half-solved state was what let a circle be dragged out of shape and
    /// stay that way until the next change put it right.
    pub fn settle_around_all(
        &mut self,
        dropped: &[(PointId, DVec2)],
        millimeters_per_unit: f64,
    ) -> LengthOutcome {
        let kept = (self.points.clone(), self.circles.clone());
        let place = |sketch: &mut Self| {
            for (point, position) in dropped {
                sketch.move_point(*point, *position);
            }
        };

        place(self);
        self.held = dropped.iter().map(|(point, _)| *point).collect();
        let outcome = self.resolve(millimeters_per_unit);
        self.held.clear();
        if outcome == LengthOutcome::Exact {
            return outcome;
        }

        self.points.clone_from(&kept.0);
        self.circles.clone_from(&kept.1);
        place(self);
        let outcome = self.resolve(millimeters_per_unit);
        if !self.has_a_collapsed_trait(self.drawing_size()) && !self.has_a_flipped_tangent() {
            return outcome;
        }

        // Neither way leaves a drawing worth keeping: a trait may have collapsed,
        // or a tangency's contact slid off its segment. The gesture is refused
        // rather than the shape broken — the point simply does not go there.
        (self.points, self.circles) = kept;
        LengthOutcome::BestEffort
    }

    /// Re-satisfies every dimension at once, reporting whether it managed.
    pub fn resolve(&mut self, millimeters_per_unit: f64) -> LengthOutcome {
        match self.solve(millimeters_per_unit) {
            SolveOutcome::Solved | SolveOutcome::Nothing => LengthOutcome::Exact,
            SolveOutcome::Residual => LengthOutcome::BestEffort,
        }
    }
}
