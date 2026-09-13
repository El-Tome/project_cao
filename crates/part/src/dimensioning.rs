//! What a typed value does to a part, and what the part measures back.
//!
//! The first value a part is given fixes what its drawing is worth in
//! millimetres; every later one is a rule the geometry gives way to. Replaying
//! the history is [`crate::state`]'s business, and this is the one step of it
//! that answers back.

use cao_sketch::{DimensionTarget, LengthOutcome};

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
        sketch.set_dimension(target, value, false);

        // A radius stands on its own; everything else moves the points, so the
        // whole system is re-solved to keep the earlier values true.
        Some(DimensionOutcome::Geometry(sketch.resolve(scale)))
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
mod tests {
    use cao_sketch::{CircleId, DimensionTarget, SegmentId, WorkPlane};
    use glam::DVec2;

    use super::*;
    use crate::history::{Operation, PointRef};
    use crate::state::PartState;

    #[test]
    fn a_circle_takes_its_radius_from_the_scale() {
        let mut state = PartState::default();
        state.apply(&Operation::CreateSketch {
            plane: WorkPlane::XY,
        });
        state.apply(&Operation::AddCircle {
            sketch: 0,
            center: PointRef::New(DVec2::ZERO),
            radius: 4.0,
            rim: Vec::new(),
            construction: false,
        });

        // First value in the part: it sets the scale rather than resizing.
        let outcome = state.apply(&Operation::SetDimension {
            sketch: 0,
            target: DimensionTarget::Radius(CircleId(0)),
            value: 20.0,
            placement: None,
        });
        assert_eq!(
            outcome,
            Some(DimensionOutcome::ScaleDefined {
                millimeters_per_unit: 5.0
            })
        );
        assert!((state.sketches[0].circle(CircleId(0)).radius - 4.0).abs() < 1e-4);
    }

    /// An angle cannot set the scale: degrees say nothing about size.
    #[test]
    fn an_angle_never_defines_the_scale() {
        let mut state = PartState::default();
        state.apply(&Operation::CreateSketch {
            plane: WorkPlane::XY,
        });
        state.apply(&Operation::AddSegment {
            sketch: 0,
            start: PointRef::New(DVec2::ZERO),
            end: PointRef::New(DVec2::new(10.0, 0.0)),
            construction: false,
        });
        state.apply(&Operation::AddSegment {
            sketch: 0,
            start: PointRef::Existing(cao_sketch::PointId(1)),
            end: PointRef::New(DVec2::new(0.0, 10.0)),
            construction: false,
        });

        let outcome = state.apply(&Operation::SetDimension {
            sketch: 0,
            target: DimensionTarget::Angle {
                first: SegmentId(0),
                second: SegmentId(1),
            },
            value: 45.0,
            placement: None,
        });

        assert!(matches!(outcome, Some(DimensionOutcome::Geometry(_))));
        assert!(!state.has_scale());
        let measured = state.sketches[0]
            .angle_between(SegmentId(0), SegmentId(1))
            .expect("the segments meet");
        assert!((measured - 45.0).abs() < 1e-2, "got {measured}°");
    }

    /// A rectangle with one corner on the sketch origin, nothing about it said
    /// yet.
    fn a_rectangle_on_the_origin() -> PartState {
        let mut state = PartState::default();
        state.apply(&Operation::CreateSketch {
            plane: WorkPlane::XY,
        });
        state.apply(&Operation::AddRectangle {
            sketch: 0,
            corner: PointRef::Existing(cao_sketch::Sketch::ORIGIN),
            opposite: PointRef::New(DVec2::new(70.0, 30.0)),
            construction: false,
        });
        state
    }

    /// Two sides, the three right angles a rectangle needs, and the direction
    /// of one side against an axis — everything it takes to leave nothing
    /// about it still to determine.
    fn give_it_everything_it_needs(state: &mut PartState) {
        for (target, value) in [
            (DimensionTarget::Length(SegmentId(0)), 70.0),
            (DimensionTarget::Length(SegmentId(1)), 30.0),
            (
                DimensionTarget::Angle {
                    first: SegmentId(0),
                    second: SegmentId(1),
                },
                90.0,
            ),
            (
                DimensionTarget::Angle {
                    first: SegmentId(1),
                    second: SegmentId(2),
                },
                90.0,
            ),
            (
                DimensionTarget::Angle {
                    first: SegmentId(2),
                    second: SegmentId(3),
                },
                90.0,
            ),
            (
                DimensionTarget::AxisAngle {
                    segment: SegmentId(0),
                    axis: cao_sketch::SketchAxis::U,
                },
                0.0,
            ),
        ] {
            state.apply(&Operation::SetDimension {
                sketch: 0,
                target,
                value,
                placement: None,
            });
        }
    }

    /// The rectangle from the report: it must end up fully constrained — which
    /// needs the angle taken against an axis, since nothing else stops it
    /// turning about its corner.
    #[test]
    fn a_rectangle_on_the_axes_can_be_fully_constrained() {
        let mut state = a_rectangle_on_the_origin();
        assert!(
            !state.sketches[0].is_fully_constrained(1.0),
            "nothing given yet"
        );

        give_it_everything_it_needs(&mut state);

        let sketch = &state.sketches[0];
        assert_eq!(
            sketch.freedom(state.scale()).degrees_of_freedom,
            0,
            "the rectangle should have nothing left to determine"
        );
        assert!(sketch.is_fully_constrained(state.scale()));
    }

    /// Once it is settled, a further angle on the same rectangle adds nothing.
    #[test]
    fn a_further_angle_on_a_settled_rectangle_is_redundant() {
        let mut state = a_rectangle_on_the_origin();
        give_it_everything_it_needs(&mut state);

        // The fourth corner follows from the other three.
        let outcome = state.apply(&Operation::SetDimension {
            sketch: 0,
            target: DimensionTarget::Angle {
                first: SegmentId(3),
                second: SegmentId(0),
            },
            value: 90.0,
            placement: None,
        });

        assert_eq!(outcome, Some(DimensionOutcome::Reference));
    }

    /// A value on an already-settled shape becomes a readout, and the readout
    /// shows what the geometry measures rather than what was typed.
    #[test]
    fn a_redundant_dimension_becomes_a_readout() {
        let mut state = PartState::default();
        state.apply(&Operation::CreateSketch {
            plane: WorkPlane::XY,
        });
        // Hung off the sketch origin, so only the far point can still move.
        state.apply(&Operation::AddSegment {
            sketch: 0,
            start: PointRef::Existing(cao_sketch::Sketch::ORIGIN),
            end: PointRef::New(DVec2::new(10.0, 0.0)),
            construction: false,
        });
        state.apply(&Operation::SetDimension {
            sketch: 0,
            target: DimensionTarget::Length(SegmentId(0)),
            value: 100.0,
            placement: None,
        });
        state.apply(&Operation::SetDimension {
            sketch: 0,
            target: DimensionTarget::AxisAngle {
                segment: SegmentId(0),
                axis: cao_sketch::SketchAxis::U,
            },
            value: 0.0,
            placement: None,
        });

        // Changing a value that already drives something is not redundant, so
        // the redundant one has to be a target that has never been set.
        state.apply(&Operation::AddSegment {
            sketch: 0,
            start: PointRef::Existing(cao_sketch::Sketch::ORIGIN),
            end: PointRef::Existing(cao_sketch::PointId(1)),
            construction: false,
        });

        let outcome = state.apply(&Operation::SetDimension {
            sketch: 0,
            target: DimensionTarget::Length(SegmentId(1)),
            value: 999.0,
            placement: None,
        });

        assert_eq!(outcome, Some(DimensionOutcome::Reference));
        let stored = state.sketches[0]
            .dimension_of(DimensionTarget::Length(SegmentId(1)))
            .expect("a readout was placed");
        assert!(stored.driven);
        assert!(
            (stored.value - 100.0).abs() < 1e-2,
            "a readout shows the measurement, not the typed value: {}",
            stored.value
        );
    }
}
