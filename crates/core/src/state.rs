use cao_sketch::{DimensionTarget, LengthOutcome, Sketch};
use cao_solid::Mesh;
use glam::Vec2;

use crate::history::{ExtrusionMode, History, Operation, PointRef, RevolutionAxis};

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
    /// The matter of the part, as one surface. Extrusions add to it or take
    /// from it; there is a single body rather than a pile of separate lumps,
    /// so that a pocket cut in a block really is a hole in the block.
    pub body: Mesh,
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
            Operation::Extrude {
                sketch,
                picks,
                distance,
                mode,
            } => {
                self.extrude(*sketch, picks, *distance, *mode);
                None
            }
            Operation::Revolve {
                sketch,
                picks,
                axis,
                angle,
                mode,
            } => {
                self.revolve(*sketch, picks, *axis, *angle, *mode);
                None
            }
        }
    }

    /// Sweeps the chosen areas around an axis of the sketch and joins the
    /// result to the part, or takes it out.
    fn revolve(
        &mut self,
        index: usize,
        picks: &[Vec2],
        axis: RevolutionAxis,
        degrees: f32,
        mode: ExtrusionMode,
    ) {
        let Some(sketch) = self.sketches.get(index) else {
            return;
        };
        let Some((axis_origin, axis_direction)) = axis_in_sketch(sketch, axis) else {
            return;
        };

        let plane = sketch.plane;
        let turn = degrees.to_radians();
        let regions = sketch.regions();

        let mut tool = Mesh::default();
        for pick in picks {
            let Some(region) = regions
                .iter()
                .filter(|region| region.contains(*pick))
                .max_by_key(|region| region.depth)
            else {
                continue;
            };
            let Some(piece) = cao_solid::revolution(
                &region.outline,
                &region.holes,
                &region.face_triangles(),
                |point| plane.to_world(point),
                axis_origin,
                axis_direction,
                turn,
            ) else {
                continue;
            };
            tool = tool.union(&piece);
        }

        self.combine(tool, mode);
    }

    /// Joins a tool to the part, or takes it out.
    fn combine(&mut self, tool: Mesh, mode: ExtrusionMode) {
        if tool.is_empty() {
            return;
        }
        self.body = match mode {
            ExtrusionMode::Add => self.body.union(&tool),
            ExtrusionMode::Cut => self.body.difference(&tool),
        };
    }

    /// Turns the chosen areas of a sketch into a prism and joins it to the
    /// part, or takes it out.
    ///
    /// Every area is turned into matter first and the lot is applied in one go:
    /// two areas extruded together must behave as one shape, not as two that
    /// happen to be cut one after the other.
    fn extrude(&mut self, index: usize, picks: &[Vec2], distance: f32, mode: ExtrusionMode) {
        let scale = self.scale();
        let Some(sketch) = self.sketches.get(index) else {
            return;
        };
        if distance.abs() < 1e-6 {
            return;
        }

        let plane = sketch.plane;
        let travel = plane.normal() * (distance / scale);
        let regions = sketch.regions();

        let mut tool = Mesh::default();
        for pick in picks {
            // The area is found again by the point that was clicked, so the
            // extrusion still means the same thing after the drawing changes.
            let Some(region) = regions
                .iter()
                .filter(|region| region.contains(*pick))
                .max_by_key(|region| region.depth)
            else {
                continue;
            };
            let piece = cao_solid::prism(
                &region.outline,
                &region.holes,
                &region.face_triangles(),
                |point| plane.to_world(point),
                travel,
            );
            tool = tool.union(&piece);
        }

        self.combine(tool, mode);
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

/// Where a revolution's axis lies, in the sketch's own coordinates.
fn axis_in_sketch(sketch: &Sketch, axis: RevolutionAxis) -> Option<(Vec2, Vec2)> {
    match axis {
        RevolutionAxis::Sketch(axis) => Some((Vec2::ZERO, axis.direction())),
        RevolutionAxis::Segment(segment) => {
            if segment.0 >= sketch.segments().len() {
                return None;
            }
            let (start, end) = sketch.endpoints(segment);
            ((end - start).length() > 1e-6).then_some((start, end - start))
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

#[cfg(test)]
mod extrusion_tests {
    use cao_sketch::WorkPlane;
    use glam::{Vec2, Vec3};

    use super::*;
    use crate::history::ExtrusionMode;

    fn volume(mesh: &Mesh) -> f32 {
        mesh.triangles()
            .iter()
            .map(|[a, b, c]| a.dot(b.cross(*c)) / 6.0)
            .sum()
    }

    fn rectangle(history: &mut History, min: Vec2, max: Vec2) {
        history.push(Operation::AddRectangle {
            sketch: 0,
            corner: PointRef::New(min),
            opposite: PointRef::New(max),
        });
    }

    fn sketch_history() -> History {
        let mut history = History::default();
        history.push(Operation::CreateSketch {
            plane: WorkPlane::XY,
        });
        history
    }

    #[test]
    fn an_extrusion_turns_an_area_into_matter() {
        let mut history = sketch_history();
        rectangle(&mut history, Vec2::ZERO, Vec2::new(10.0, 20.0));
        history.push(Operation::Extrude {
            sketch: 0,
            picks: vec![Vec2::new(5.0, 10.0)],
            distance: 4.0,
            mode: ExtrusionMode::Add,
        });

        let state = PartState::rebuild(&history);
        assert!((volume(&state.body) - 800.0).abs() < 1.0);
        let (min, max) = state.body.bounds().expect("un volume");
        assert!((max.z - min.z - 4.0).abs() < 1e-3, "hauteur");
    }

    /// L'aire cliquée est retrouvée par le point, pas par son rang : dessiner
    /// autre chose ensuite ne doit pas déplacer l'extrusion.
    #[test]
    fn an_extrusion_still_names_the_same_area_after_another_shape_is_drawn() {
        let mut history = sketch_history();
        rectangle(&mut history, Vec2::ZERO, Vec2::new(10.0, 10.0));
        rectangle(&mut history, Vec2::new(40.0, 40.0), Vec2::new(50.0, 60.0));
        history.push(Operation::Extrude {
            sketch: 0,
            picks: vec![Vec2::new(45.0, 50.0)],
            distance: 2.0,
            mode: ExtrusionMode::Add,
        });

        let state = PartState::rebuild(&history);
        assert!((volume(&state.body) - 400.0).abs() < 1.0, "la seconde aire");
        let (min, _) = state.body.bounds().expect("un volume");
        assert!(min.x > 39.0, "au bon endroit : {min}");
    }

    /// Deux cercles l'un dans l'autre font un tube : l'outil ne remplit pas le
    /// milieu.
    #[test]
    fn two_circles_extrude_to_a_tube() {
        let mut history = sketch_history();
        history.push(Operation::AddCircle {
            sketch: 0,
            center: PointRef::New(Vec2::ZERO),
            radius: 10.0,
        });
        history.push(Operation::AddCircle {
            sketch: 0,
            center: PointRef::Existing(cao_sketch::PointId(1)),
            radius: 6.0,
        });
        history.push(Operation::Extrude {
            sketch: 0,
            picks: vec![Vec2::new(8.0, 0.0)],
            distance: 5.0,
            mode: ExtrusionMode::Add,
        });

        let state = PartState::rebuild(&history);
        let expected = std::f32::consts::PI * (100.0 - 36.0) * 5.0;
        let made = volume(&state.body);
        assert!((made - expected).abs() / expected < 0.03, "{made} / {expected}");
    }

    /// Une poche : la seconde esquisse creuse le bloc de la première.
    #[test]
    fn a_cut_takes_matter_away() {
        let mut history = sketch_history();
        rectangle(&mut history, Vec2::ZERO, Vec2::new(10.0, 10.0));
        history.push(Operation::Extrude {
            sketch: 0,
            picks: vec![Vec2::new(5.0, 5.0)],
            distance: 10.0,
            mode: ExtrusionMode::Add,
        });

        history.push(Operation::CreateSketch {
            plane: WorkPlane::XY,
        });
        history.push(Operation::AddRectangle {
            sketch: 1,
            corner: PointRef::New(Vec2::new(2.0, 2.0)),
            opposite: PointRef::New(Vec2::new(4.0, 4.0)),
        });
        history.push(Operation::Extrude {
            sketch: 1,
            picks: vec![Vec2::new(3.0, 3.0)],
            distance: 4.0,
            mode: ExtrusionMode::Cut,
        });

        let state = PartState::rebuild(&history);
        let made = volume(&state.body);
        assert!((made - (1000.0 - 16.0)).abs() < 2.0, "{made}");
    }

    /// Une forme dans une autre laisse le milieu vide dès la première
    /// extrusion : c'est le cas du tube, en rectangles.
    #[test]
    fn a_shape_inside_another_is_already_hollow() {
        let mut history = sketch_history();
        rectangle(&mut history, Vec2::ZERO, Vec2::new(10.0, 10.0));
        rectangle(&mut history, Vec2::new(2.0, 2.0), Vec2::new(4.0, 4.0));
        history.push(Operation::Extrude {
            sketch: 0,
            picks: vec![Vec2::new(1.0, 1.0)],
            distance: 10.0,
            mode: ExtrusionMode::Add,
        });

        let made = volume(&PartState::rebuild(&history).body);
        assert!((made - (1000.0 - 40.0)).abs() < 2.0, "{made}");
    }

    /// Une révolution complète autour d'un axe de l'esquisse.
    #[test]
    fn a_revolution_sweeps_an_area_around_an_axis() {
        let mut history = sketch_history();
        rectangle(&mut history, Vec2::new(3.0, 0.0), Vec2::new(5.0, 2.0));
        history.push(Operation::Revolve {
            sketch: 0,
            picks: vec![Vec2::new(4.0, 1.0)],
            axis: crate::history::RevolutionAxis::Sketch(cao_sketch::SketchAxis::V),
            angle: 360.0,
            mode: ExtrusionMode::Add,
        });

        let made = volume(&PartState::rebuild(&history).body);
        let expected = std::f32::consts::TAU * 4.0 * 4.0;
        assert!((made - expected).abs() / expected < 0.02, "{made} / {expected}");
    }

    /// L'axe peut être un trait qu'on a tracé soi-même.
    #[test]
    fn a_revolution_can_turn_around_a_drawn_line() {
        let mut history = sketch_history();
        history.push(Operation::AddSegment {
            sketch: 0,
            start: PointRef::New(Vec2::new(0.0, -10.0)),
            end: PointRef::New(Vec2::new(0.0, 10.0)),
        });
        rectangle(&mut history, Vec2::new(3.0, 0.0), Vec2::new(5.0, 2.0));
        history.push(Operation::Revolve {
            sketch: 0,
            picks: vec![Vec2::new(4.0, 1.0)],
            axis: crate::history::RevolutionAxis::Segment(cao_sketch::SegmentId(0)),
            angle: 360.0,
            mode: ExtrusionMode::Add,
        });

        let made = volume(&PartState::rebuild(&history).body);
        let expected = std::f32::consts::TAU * 4.0 * 4.0;
        assert!((made - expected).abs() / expected < 0.02, "{made} / {expected}");
    }

    /// Un profil à cheval sur l'axe passerait à travers lui-même : rien n'est
    /// produit plutôt qu'un volume retourné.
    #[test]
    fn a_revolution_across_its_axis_makes_nothing() {
        let mut history = sketch_history();
        rectangle(&mut history, Vec2::new(-4.0, 0.0), Vec2::new(5.0, 2.0));
        history.push(Operation::Revolve {
            sketch: 0,
            picks: vec![Vec2::new(1.0, 1.0)],
            axis: crate::history::RevolutionAxis::Sketch(cao_sketch::SketchAxis::V),
            angle: 360.0,
            mode: ExtrusionMode::Add,
        });

        assert!(PartState::rebuild(&history).body.is_empty());
    }

    /// Le blocage rencontré à l'usage, avec ses vraies mesures : un cylindre de
    /// révolution de trente unités de rayon, puis une esquisse posée sur sa
    /// face du dessus et creusée dedans.
    ///
    /// À cette distance de l'origine, un triangle sortait de son propre plan en
    /// `f32`, et la partition de l'espace le recoupait sans fin.
    #[test]
    fn cutting_into_a_revolved_part_from_its_own_face() {
        let mut history = sketch_history();
        rectangle(&mut history, Vec2::new(-30.0, -7.5), Vec2::new(0.0, -52.5));
        history.push(Operation::Revolve {
            sketch: 0,
            picks: vec![Vec2::new(-14.8, -26.5)],
            axis: crate::history::RevolutionAxis::Sketch(cao_sketch::SketchAxis::V),
            angle: 360.0,
            mode: ExtrusionMode::Add,
        });

        // Le plan tel que le donne le clic sur la face : sa normale et ses axes
        // ne sont pas exactement alignés, ce qui fait partie du cas.
        let face = WorkPlane {
            origin: Vec3::new(9.729663e-07, -7.5000005, -1.2972885e-06),
            u: Vec3::new(-1.0, -1.2972883e-07, 0.0),
            v: Vec3::new(2.2439427e-14, -1.7297178e-07, 1.0),
        };
        history.push(Operation::CreateSketch { plane: face });
        history.push(Operation::AddRectangle {
            sketch: 1,
            corner: PointRef::New(Vec2::new(-25.0, 32.5)),
            opposite: PointRef::New(Vec2::new(12.5, -7.5)),
        });
        history.push(Operation::Extrude {
            sketch: 1,
            picks: vec![Vec2::new(-6.0, 12.0)],
            distance: -10.0,
            mode: ExtrusionMode::Cut,
        });

        let state = PartState::rebuild(&history);
        assert!(!state.body.is_empty());
        assert!(volume(&state.body) > 0.0);
    }

    /// Une esquisse pas entièrement contrainte s'extrude quand même.
    #[test]
    fn an_extrusion_does_not_wait_for_a_settled_sketch() {
        let mut history = sketch_history();
        rectangle(&mut history, Vec2::new(3.0, 3.0), Vec2::new(9.0, 9.0));
        history.push(Operation::Extrude {
            sketch: 0,
            picks: vec![Vec2::new(5.0, 5.0)],
            distance: 1.0,
            mode: ExtrusionMode::Add,
        });

        let state = PartState::rebuild(&history);
        assert!(!state.sketches[0].is_fully_constrained(1.0));
        assert!(!state.body.is_empty());
    }

    /// Une cote change l'échelle : une extrusion de 10 mm reste 10 mm.
    #[test]
    fn an_extrusion_is_given_in_millimetres() {
        let mut history = sketch_history();
        rectangle(&mut history, Vec2::ZERO, Vec2::new(1.0, 1.0));
        history.push(Operation::SetDimension {
            sketch: 0,
            target: cao_sketch::DimensionTarget::Length(cao_sketch::SegmentId(0)),
            value: 50.0,
        });
        history.push(Operation::Extrude {
            sketch: 0,
            picks: vec![Vec2::new(0.5, 0.5)],
            distance: 25.0,
            mode: ExtrusionMode::Add,
        });

        let state = PartState::rebuild(&history);
        let (min, max) = state.body.bounds().expect("un volume");
        let height_millimetres = (max.z - min.z) * state.scale();
        assert!((height_millimetres - 25.0).abs() < 1e-3, "{height_millimetres}");
        let _ = Vec3::ZERO;
    }
}
