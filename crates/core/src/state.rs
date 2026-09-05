use cao_sketch::{DimensionTarget, LengthOutcome, Sketch};
use glam::Vec2;

use crate::history::{History, Operation, PointRef};

/// What applying a typed length did.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DimensionOutcome {
    /// The part had no dimension yet, so this one set its scale instead of
    /// moving anything: the drawing keeps its shape and gains a real size.
    ScaleDefined { millimeters_per_unit: f32 },
    /// The scale was already fixed, so the geometry moved to match.
    Geometry(LengthOutcome),
    /// The shape was already fully determined, so this value drives nothing.
    /// It is kept as a readout: it shows what the geometry measures, and
    /// changing it would mean nothing.
    Reference,
}

/// The geometry of a part at a given point in its history.
///
/// Never saved: it is rebuilt by replaying the history, which is what keeps
/// "go back to this step" honest — there is no second copy that could drift
/// away from the list of operations.
#[derive(Debug, Clone, Default)]
pub struct PartState {
    /// Millimetres one world unit is worth, undefined until the first
    /// dimension is typed.
    pub millimeters_per_unit: Option<f32>,
    pub sketches: Vec<Sketch>,
}

impl PartState {
    pub fn rebuild(history: &History) -> Self {
        let mut state = Self::default();
        for operation in history.applied_operations() {
            state.apply(operation);
        }
        state
    }

    pub fn scale(&self) -> f32 {
        self.millimeters_per_unit.unwrap_or(1.0)
    }

    pub fn has_scale(&self) -> bool {
        self.millimeters_per_unit.is_some()
    }

    pub fn to_millimeters(&self, units: f32) -> f32 {
        units * self.scale()
    }

    /// Runs one operation. This is the only place geometry is produced, so a
    /// replay and a live edit can never disagree.
    pub fn apply(&mut self, operation: &Operation) -> Option<DimensionOutcome> {
        match operation {
            Operation::CreateSketch { plane } => {
                self.sketches.push(Sketch::new(*plane));
                None
            }
            Operation::AddPoint { sketch, position } => {
                self.sketches.get_mut(*sketch)?.add_point(*position);
                None
            }
            Operation::AddSegment { sketch, start, end } => {
                let sketch = self.sketches.get_mut(*sketch)?;
                let start = resolve(sketch, start);
                let end = resolve(sketch, end);
                if start != end {
                    sketch.add_segment(start, end);
                }
                None
            }
            Operation::AddRectangle {
                sketch,
                corner,
                opposite,
            } => {
                let sketch = self.sketches.get_mut(*sketch)?;
                // The two given corners may reuse points already drawn; the
                // other two are always new.
                let first = resolve(sketch, corner);
                let third = resolve(sketch, opposite);
                let (a, c) = (sketch.point(first), sketch.point(third));
                let second = sketch.add_point(Vec2::new(c.x, a.y));
                let fourth = sketch.add_point(Vec2::new(a.x, c.y));

                let corners = [first, second, third, fourth];
                for index in 0..4 {
                    sketch.add_segment(corners[index], corners[(index + 1) % 4]);
                }
                None
            }
            Operation::MovePoint {
                sketch,
                point,
                position,
            } => {
                let scale = self.scale();
                let sketch = self.sketches.get_mut(*sketch)?;
                sketch.move_point(*point, *position);
                // Moving a point by hand must not break the values already
                // given, so the drawing settles again around it.
                sketch.resolve(scale);
                None
            }
            Operation::AddCircle {
                sketch,
                center,
                radius,
            } => {
                let sketch = self.sketches.get_mut(*sketch)?;
                let center = resolve(sketch, center);
                sketch.add_circle(center, *radius);
                None
            }
            Operation::MoveDimension {
                sketch,
                target,
                offset,
            } => {
                self.sketches
                    .get_mut(*sketch)?
                    .offset_dimension(*target, *offset);
                None
            }
            Operation::SetDimension {
                sketch,
                target,
                value,
            } => self.apply_dimension(*sketch, *target, *value),
        }
    }

    /// Applies a length typed by the user, in millimetres.
    ///
    /// The very first one defines what the drawing measures: nothing moves, the
    /// part simply learns how many millimetres a world unit is worth. Every
    /// later one is a constraint, and the geometry gives way instead.
    fn apply_dimension(
        &mut self,
        index: usize,
        target: DimensionTarget,
        value: f32,
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
        if let DimensionTarget::Radius(circle) = target {
            sketch.set_circle_radius(circle, value / scale);
            return Some(DimensionOutcome::Geometry(cao_sketch::LengthOutcome::Exact));
        }
        Some(DimensionOutcome::Geometry(sketch.resolve(scale)))
    }

    /// The length a dimension refers to, in world units, or `None` for an angle
    /// which has no length at all.
    fn length_in_units(&self, index: usize, target: DimensionTarget) -> Option<f32> {
        let sketch = self.sketches.get(index)?;
        let units = match target {
            DimensionTarget::Length(segment) => sketch.segment_length(segment),
            DimensionTarget::Distance { from, to } => sketch.point(from).distance(sketch.point(to)),
            DimensionTarget::Radius(circle) => sketch.circle(circle).radius,
            DimensionTarget::Angle { .. } | DimensionTarget::AxisAngle { .. } => return None,
        };
        (units > 1e-6).then_some(units)
    }

    /// What the geometry actually measures right now: millimetres for a length
    /// or a radius, degrees for an angle. This is what a readout shows, so it
    /// stays true however the drawing moves afterwards.
    pub fn measured(&self, index: usize, target: DimensionTarget) -> Option<f32> {
        let sketch = self.sketches.get(index)?;
        match target {
            DimensionTarget::Length(segment) => (segment.0 < sketch.segments().len())
                .then(|| self.to_millimeters(sketch.segment_length(segment))),
            DimensionTarget::Distance { from, to } => (from.0 < sketch.points().len()
                && to.0 < sketch.points().len())
            .then(|| self.to_millimeters(sketch.point(from).distance(sketch.point(to)))),
            DimensionTarget::Radius(circle) => (circle.0 < sketch.circles().len())
                .then(|| self.to_millimeters(sketch.circle(circle).radius)),
            DimensionTarget::Angle { first, second } => sketch.angle_between(first, second),
            DimensionTarget::AxisAngle { segment, axis } => sketch.angle_with_axis(segment, axis),
        }
    }
}

fn resolve(sketch: &mut Sketch, point: &PointRef) -> cao_sketch::PointId {
    match point {
        PointRef::Existing(id) => *id,
        PointRef::New(position) => sketch.add_point(*position),
    }
}

#[cfg(test)]
mod tests {
    use cao_sketch::{SegmentId, WorkPlane};
    use glam::Vec2;

    use super::*;

    fn chain_history() -> History {
        let mut history = History::default();
        history.push(Operation::CreateSketch {
            plane: WorkPlane::XY,
        });
        history.push(Operation::AddSegment {
            sketch: 0,
            start: PointRef::New(Vec2::ZERO),
            end: PointRef::New(Vec2::new(2.0, 0.0)),
        });
        history.push(Operation::AddSegment {
            sketch: 0,
            // Point 0 is the sketch origin, so the corner just drawn is 2.
            start: PointRef::Existing(cao_sketch::PointId(2)),
            end: PointRef::New(Vec2::new(2.0, 1.0)),
        });
        history
    }

    #[test]
    fn replaying_a_history_builds_the_drawing() {
        let state = PartState::rebuild(&chain_history());
        assert_eq!(state.sketches.len(), 1);
        assert_eq!(state.sketches[0].segments().len(), 2);
        assert_eq!(
            state.sketches[0].points().len(),
            4,
            "the origin, plus three drawn points with the corner shared"
        );
    }

    /// Rewinding must give exactly the state that existed at that step: this is
    /// what both undo and the history tree rely on.
    #[test]
    fn rewinding_reproduces_the_earlier_state() {
        let mut history = chain_history();
        let after_first_segment = {
            let mut shorter = history.clone();
            shorter.rewind_to(2);
            PartState::rebuild(&shorter)
        };

        history.undo();
        let undone = PartState::rebuild(&history);

        assert_eq!(undone.sketches[0].segments().len(), 1);
        assert_eq!(
            undone.sketches[0].points().len(),
            after_first_segment.sketches[0].points().len()
        );
        assert_eq!(
            undone.sketches[0].points(),
            after_first_segment.sketches[0].points()
        );
    }

    /// A live edit and a replay must produce the same thing, or the drawing
    /// would silently change the next time the part is opened.
    #[test]
    fn applying_live_matches_replaying() {
        let mut history = chain_history();
        history.push(Operation::SetDimension {
            sketch: 0,
            target: DimensionTarget::Length(SegmentId(0)),
            value: 100.0,
        });

        let mut live = PartState::default();
        for operation in history.applied_operations() {
            live.apply(operation);
        }
        let replayed = PartState::rebuild(&history);

        assert_eq!(live.millimeters_per_unit, replayed.millimeters_per_unit);
        assert_eq!(live.sketches[0].points(), replayed.sketches[0].points());
    }

    #[test]
    fn the_first_dimension_sets_the_scale_without_moving_anything() {
        let mut state = PartState::rebuild(&chain_history());
        let before = state.sketches[0].points().to_vec();

        let outcome = state.apply(&Operation::SetDimension {
            sketch: 0,
            target: DimensionTarget::Length(SegmentId(0)),
            value: 100.0,
        });

        assert_eq!(
            outcome,
            Some(DimensionOutcome::ScaleDefined {
                millimeters_per_unit: 50.0
            })
        );
        assert_eq!(state.sketches[0].points(), before.as_slice());
    }

    #[test]
    fn later_dimensions_move_the_geometry() {
        let mut state = PartState::rebuild(&chain_history());
        state.apply(&Operation::SetDimension {
            sketch: 0,
            target: DimensionTarget::Length(SegmentId(0)),
            value: 100.0,
        });

        let outcome = state.apply(&Operation::SetDimension {
            sketch: 0,
            target: DimensionTarget::Length(SegmentId(1)),
            value: 100.0,
        });

        assert_eq!(
            outcome,
            Some(DimensionOutcome::Geometry(LengthOutcome::Exact))
        );
        let length = state.to_millimeters(state.sketches[0].segment_length(SegmentId(1)));
        assert!((length - 100.0).abs() < 1e-2, "got {length} mm");
    }

    #[test]
    fn an_operation_on_a_missing_sketch_is_ignored() {
        let mut state = PartState::default();
        assert_eq!(
            state.apply(&Operation::AddSegment {
                sketch: 3,
                start: PointRef::New(Vec2::ZERO),
                end: PointRef::New(Vec2::X),
            }),
            None
        );
        assert!(state.sketches.is_empty());
    }
}

#[cfg(test)]
mod extra_tests {
    use cao_sketch::{CircleId, DimensionTarget, SegmentId, WorkPlane};
    use glam::Vec2;

    use super::*;

    #[test]
    fn a_rectangle_is_one_step_with_four_sides() {
        let mut state = PartState::default();
        state.apply(&Operation::CreateSketch {
            plane: WorkPlane::XY,
        });
        state.apply(&Operation::AddRectangle {
            sketch: 0,
            corner: PointRef::New(Vec2::ZERO),
            opposite: PointRef::New(Vec2::new(40.0, 20.0)),
        });

        let sketch = &state.sketches[0];
        // The origin, plus the four corners.
        assert_eq!(sketch.points().len(), 5);
        assert_eq!(sketch.segments().len(), 4);
        assert!((sketch.segment_length(SegmentId(0)) - 40.0).abs() < 1e-4);
        assert!((sketch.segment_length(SegmentId(1)) - 20.0).abs() < 1e-4);
    }

    #[test]
    fn a_circle_takes_its_radius_from_the_scale() {
        let mut state = PartState::default();
        state.apply(&Operation::CreateSketch {
            plane: WorkPlane::XY,
        });
        state.apply(&Operation::AddCircle {
            sketch: 0,
            center: PointRef::New(Vec2::ZERO),
            radius: 4.0,
        });

        // First value in the part: it sets the scale rather than resizing.
        let outcome = state.apply(&Operation::SetDimension {
            sketch: 0,
            target: DimensionTarget::Radius(CircleId(0)),
            value: 20.0,
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
            start: PointRef::New(Vec2::ZERO),
            end: PointRef::New(Vec2::new(10.0, 0.0)),
        });
        state.apply(&Operation::AddSegment {
            sketch: 0,
            start: PointRef::Existing(cao_sketch::PointId(1)),
            end: PointRef::New(Vec2::new(0.0, 10.0)),
        });

        let outcome = state.apply(&Operation::SetDimension {
            sketch: 0,
            target: DimensionTarget::Angle {
                first: SegmentId(0),
                second: SegmentId(1),
            },
            value: 45.0,
        });

        assert!(matches!(outcome, Some(DimensionOutcome::Geometry(_))));
        assert!(!state.has_scale());
        let measured = state.sketches[0]
            .angle_between(SegmentId(0), SegmentId(1))
            .expect("the segments meet");
        assert!((measured - 45.0).abs() < 1e-2, "got {measured}°");
    }

    /// The rectangle from the report: one corner on the origin, sides along the
    /// axes, three right angles and two lengths. It must end up fully
    /// constrained — which needs the angle taken against an axis, since
    /// nothing else stops it turning about its corner.
    #[test]
    fn a_rectangle_on_the_axes_can_be_fully_constrained() {
        let mut state = PartState::default();
        state.apply(&Operation::CreateSketch {
            plane: WorkPlane::XY,
        });
        state.apply(&Operation::AddRectangle {
            sketch: 0,
            corner: PointRef::Existing(cao_sketch::Sketch::ORIGIN),
            opposite: PointRef::New(Vec2::new(70.0, 30.0)),
        });

        let sketch = &state.sketches[0];
        assert!(!sketch.is_fully_constrained(1.0), "nothing given yet");

        // Two sides, the three right angles a rectangle needs, and the
        // direction of one side against an axis.
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
            });
        }

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
        let mut state = PartState::default();
        state.apply(&Operation::CreateSketch {
            plane: WorkPlane::XY,
        });
        state.apply(&Operation::AddRectangle {
            sketch: 0,
            corner: PointRef::Existing(cao_sketch::Sketch::ORIGIN),
            opposite: PointRef::New(Vec2::new(70.0, 30.0)),
        });
        // Two sides, the three right angles a rectangle needs, and the
        // direction of one side against an axis.
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
            });
        }

        // The fourth corner follows from the other three.
        let outcome = state.apply(&Operation::SetDimension {
            sketch: 0,
            target: DimensionTarget::Angle {
                first: SegmentId(3),
                second: SegmentId(0),
            },
            value: 90.0,
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
            end: PointRef::New(Vec2::new(10.0, 0.0)),
        });
        state.apply(&Operation::SetDimension {
            sketch: 0,
            target: DimensionTarget::Length(SegmentId(0)),
            value: 100.0,
        });
        state.apply(&Operation::SetDimension {
            sketch: 0,
            target: DimensionTarget::AxisAngle {
                segment: SegmentId(0),
                axis: cao_sketch::SketchAxis::U,
            },
            value: 0.0,
        });

        // Changing a value that already drives something is not redundant, so
        // the redundant one has to be a target that has never been set.
        state.apply(&Operation::AddSegment {
            sketch: 0,
            start: PointRef::Existing(cao_sketch::Sketch::ORIGIN),
            end: PointRef::Existing(cao_sketch::PointId(1)),
        });

        let outcome = state.apply(&Operation::SetDimension {
            sketch: 0,
            target: DimensionTarget::Length(SegmentId(1)),
            value: 999.0,
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
