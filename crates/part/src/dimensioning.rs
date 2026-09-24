//! What a typed value does to a part, and what the part measures back.
//!
//! The first value a part is given fixes what its drawing is worth in
//! millimetres; every later one is a rule the geometry gives way to. Replaying
//! the history is [`crate::state`]'s business, and this is the one step of it
//! that answers back.

use cao_sketch::{DimensionTarget, LengthOutcome};
use glam::DVec2;

use crate::broken::Broken;
use crate::formula::Formula;
use crate::outcome::Outcome;
use crate::replay::SetBy;
use crate::state::PartState;

/// What applying a typed length did.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DimensionOutcome {
    /// The part had no dimension yet, so this one set its scale instead of
    /// moving anything: the drawing keeps its shape and gains a real size.
    ScaleDefined { millimeters_per_unit: f64 },
    /// The scale was already fixed, so the geometry moved to match.
    Geometry(LengthOutcome),
    /// The shape was already fully determined, so this value drives nothing.
    /// It is kept as a readout: it shows what the geometry measures, and
    /// changing it would mean nothing.
    Reference,
}

impl PartState {
    /// Applies a value as it was written: worked out against the variables,
    /// set on the drawing, and remembered there as what it was written from —
    /// so that it can be shown and edited as that, and carried with the value
    /// wherever a cut hands it on. Which step of the replay set it goes with
    /// it, which is how the replay sees it leave the drawing.
    pub(crate) fn apply_written_dimension(
        &mut self,
        index: usize,
        target: DimensionTarget,
        written: &Formula,
        placement: Option<DVec2>,
    ) -> Option<Outcome> {
        let broken = Broken::Dimension {
            sketch: index,
            target,
        };
        let (set, values) = self.next_value_set();
        let value = written.value(values).filter(|value| target.takes(*value));
        let outcome = value.and_then(|value| self.apply_dimension(index, target, value));
        match self.remember_as(index, target, outcome, written.note(), Some(set)) {
            true => self.held(broken),
            false => self.broke(broken),
        }
        if let (Some(offset), Some(drawing)) = (placement, self.sketches.get_mut(index)) {
            drawing.offset_dimension(target, offset);
        }
        outcome.map(Outcome::Dimension)
    }

    /// Remembers on the drawing what a value was written from, and which step
    /// of the replay set it when that was the variables, once it holds; and
    /// says whether it did. A readout reports what the drawing measures, and
    /// so was written from nothing; a value refused left the drawing as it
    /// was.
    pub(crate) fn remember_as(
        &mut self,
        index: usize,
        target: DimensionTarget,
        outcome: Option<DimensionOutcome>,
        note: Option<String>,
        set: Option<SetBy>,
    ) -> bool {
        let note = match outcome {
            Some(DimensionOutcome::ScaleDefined { .. })
            | Some(DimensionOutcome::Geometry(LengthOutcome::Exact)) => note,
            Some(DimensionOutcome::Reference) => None,
            _ => return false,
        };
        if let Some(drawing) = self.sketches.get_mut(index) {
            drawing.mark_dimension(target, note.as_ref().and(set));
            drawing.write_dimension_as(target, note);
        }
        true
    }

    /// Applies a length typed by the user, in millimetres.
    ///
    /// The very first one defines what the drawing measures: nothing moves, the
    /// part simply learns how many millimetres a world unit is worth. Every
    /// later one is a constraint, and the geometry gives way instead.
    pub(crate) fn apply_dimension(
        &mut self,
        index: usize,
        target: DimensionTarget,
        value: f64,
    ) -> Option<DimensionOutcome> {
        if value <= 0.0 && !matches!(target, DimensionTarget::AxisAngle { .. }) {
            return None;
        }
        let scale = self.scale();
        let measured = self.measured(index, target)?;

        // Nothing here is left to determine, so this value cannot drive the
        // shape. It is kept as a readout rather than refused: seeing a length
        // is useful even when setting it is not.
        if self.sketches.get(index)?.would_be_redundant(target, scale) {
            self.sketches[index].set_dimension(target, measured, true);
            return Some(DimensionOutcome::Reference);
        }

        // A length in millimetres is what gives the drawing its size; an angle
        // says nothing about scale, so it can never be the one to set it.
        if !self.has_scale()
            && let Some(units) = self.length_in_units(index, target)
        {
            self.sketches[index].set_dimension(target, value, false);
            let millimeters_per_unit = value / units;
            self.millimeters_per_unit = Some(millimeters_per_unit);
            // Nothing to solve: the value was chosen to match what is already
            // drawn, which is the whole point of letting it set the scale.
            return Some(DimensionOutcome::ScaleDefined {
                millimeters_per_unit,
            });
        }

        let sketch = self.sketches.get_mut(index)?;
        let before = sketch.clone();
        sketch.set_dimension(target, value, false);

        // A radius stands on its own; everything else moves the points, so the
        // whole system is re-solved to keep the earlier values true. A value
        // that cannot be fully honoured is refused rather than half-applied:
        // the drawing goes back to what it was, the same way a drag the
        // solver cannot satisfy already reverts in `settle_around_all`.
        let outcome = sketch.resolve(scale);
        if outcome != LengthOutcome::Exact {
            self.sketches[index] = before;
        }
        Some(DimensionOutcome::Geometry(outcome))
    }

    /// The length a dimension refers to, in world units, or `None` for an angle
    /// which has no length at all.
    fn length_in_units(&self, index: usize, target: DimensionTarget) -> Option<f64> {
        let sketch = self.sketches.get(index)?;
        let units = match target {
            DimensionTarget::Length(segment) => {
                (segment.0 < sketch.segments().len()).then(|| sketch.segment_length(segment))?
            }
            DimensionTarget::Distance { from, to } => {
                let points = sketch.points();
                points.get(from.0)?.distance(*points.get(to.0)?)
            }
            DimensionTarget::Radius(circle) => sketch.circles().get(circle.0)?.radius,
            DimensionTarget::Diameter(circle) => sketch.circles().get(circle.0)?.radius * 2.0,
            DimensionTarget::PointToSegment { point, segment } => {
                sketch.point_to_segment(point, segment)?
            }
            DimensionTarget::Projected { from, to, axis } => {
                sketch.projected_gap(from, to, axis)?
            }
            DimensionTarget::ArcRadius(arc) => {
                (arc.0 < sketch.arcs().len()).then(|| sketch.arc_radius(arc))?
            }
            DimensionTarget::Angle { .. }
            | DimensionTarget::AngleBetween { .. }
            | DimensionTarget::AxisAngle { .. }
            | DimensionTarget::ArcSweep(_) => return None,
        };
        (units > 1e-6).then_some(units)
    }

    /// What the geometry actually measures right now: millimetres for a length
    /// or a radius, degrees for an angle. This is what a readout shows, so it
    /// stays true however the drawing moves afterwards.
    pub fn measured(&self, index: usize, target: DimensionTarget) -> Option<f64> {
        let sketch = self.sketches.get(index)?;
        match target {
            DimensionTarget::Length(segment) => (segment.0 < sketch.segments().len())
                .then(|| self.to_millimeters(sketch.segment_length(segment))),
            DimensionTarget::Distance { from, to } => (from.0 < sketch.points().len()
                && to.0 < sketch.points().len())
            .then(|| self.to_millimeters(sketch.point(from).distance(sketch.point(to)))),
            DimensionTarget::Radius(circle) => (circle.0 < sketch.circles().len())
                .then(|| self.to_millimeters(sketch.circle(circle).radius)),
            DimensionTarget::Diameter(circle) => (circle.0 < sketch.circles().len())
                .then(|| self.to_millimeters(sketch.circle(circle).radius * 2.0)),
            DimensionTarget::Angle { first, second } => sketch.angle_between(first, second),
            DimensionTarget::AngleBetween { .. } => sketch.opening(target),
            DimensionTarget::AxisAngle { segment, axis } => sketch.angle_with_axis(segment, axis),
            DimensionTarget::PointToSegment { point, segment } => sketch
                .point_to_segment(point, segment)
                .map(|units| self.to_millimeters(units)),
            DimensionTarget::Projected { from, to, axis } => sketch
                .projected_gap(from, to, axis)
                .map(|units| self.to_millimeters(units)),
            DimensionTarget::ArcRadius(arc) => {
                (arc.0 < sketch.arcs().len()).then(|| self.to_millimeters(sketch.arc_radius(arc)))
            }
            DimensionTarget::ArcSweep(arc) => {
                (arc.0 < sketch.arcs().len()).then(|| sketch.arc_sweep(arc).to_degrees())
            }
        }
    }
}

#[cfg(test)]
mod tests;
