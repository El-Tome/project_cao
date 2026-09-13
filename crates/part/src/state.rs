use cao_sketch::Sketch;
use cao_solid::Mesh;
use glam::DVec2;

use crate::dimensioning::DimensionOutcome;
use crate::history::{History, Operation, PointRef};

/// The geometry of a part at a given point in its history.
///
/// Never saved: it is rebuilt by replaying the history, which is what keeps
/// "go back to this step" honest — there is no second copy that could drift
/// away from the list of operations.
#[derive(Debug, Clone, Default)]
pub struct PartState {
    /// Millimetres one world unit is worth, undefined until the first
    /// dimension is typed.
    pub millimeters_per_unit: Option<f64>,
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

    pub fn scale(&self) -> f64 {
        self.millimeters_per_unit.unwrap_or(1.0)
    }

    pub fn has_scale(&self) -> bool {
        self.millimeters_per_unit.is_some()
    }

    pub fn to_millimeters(&self, units: f64) -> f64 {
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
            Operation::AddSegment {
                sketch,
                start,
                end,
                construction,
            } => {
                let sketch = self.sketches.get_mut(*sketch)?;
                let start = resolve(sketch, start);
                let end = resolve(sketch, end);
                if start != end {
                    if *construction {
                        sketch.add_construction_segment(start, end);
                    } else {
                        sketch.add_segment(start, end);
                    }
                }
                None
            }
            Operation::AddSymmetricSegment {
                sketch,
                middle,
                end,
                construction,
            } => {
                let sketch = self.sketches.get_mut(*sketch)?;
                let middle = resolve(sketch, middle);
                let end = resolve(sketch, end);
                let mirrored = sketch.point(middle) * 2.0 - sketch.point(end);
                if mirrored.distance(sketch.point(end)) > 1e-9 {
                    let start = sketch.add_point(mirrored);
                    let segment = if *construction {
                        sketch.add_construction_segment(start, end)
                    } else {
                        sketch.add_segment(start, end)
                    };
                    sketch.add_constraint(cao_sketch::Constraint::Midpoint {
                        point: middle,
                        segment,
                    });
                }
                None
            }
            Operation::AddRectangle {
                sketch,
                corner,
                opposite,
                construction,
            } => {
                let sketch = self.sketches.get_mut(*sketch)?;
                // The two given corners may reuse points already drawn; the
                // other two are always new.
                let first = resolve(sketch, corner);
                let third = resolve(sketch, opposite);
                let (a, c) = (sketch.point(first), sketch.point(third));
                let second = sketch.add_point(DVec2::new(c.x, a.y));
                let fourth = sketch.add_point(DVec2::new(a.x, c.y));

                let corners = [first, second, third, fourth];
                for index in 0..4 {
                    let (from, to) = (corners[index], corners[(index + 1) % 4]);
                    if *construction {
                        sketch.add_construction_segment(from, to);
                    } else {
                        sketch.add_segment(from, to);
                    }
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
                // Moving a point by hand must not break the values already
                // given, so the drawing settles again around it — around it,
                // the point itself staying exactly where it was dropped.
                sketch.settle_around(*point, *position, scale);
                None
            }
            Operation::AddCircle {
                sketch,
                center,
                radius,
                rim,
                construction,
            } => {
                let sketch = self.sketches.get_mut(*sketch)?;
                let center = resolve(sketch, center);
                let circle = if *construction {
                    sketch.add_construction_circle(center, *radius)
                } else {
                    sketch.add_circle(center, *radius)
                };
                for place in rim {
                    let point = resolve(sketch, place);
                    sketch.add_constraint(cao_sketch::Constraint::OnCircle { point, circle });
                }
                None
            }
            Operation::MoveMany { sketch, points, by } => {
                let scale = self.scale();
                let sketch = self.sketches.get_mut(*sketch)?;
                // Every point of the selection travels by the same step and is
                // held there: a shape moved as a block keeps its shape, and the
                // rest of the drawing settles around it.
                let dropped: Vec<(cao_sketch::PointId, DVec2)> = points
                    .iter()
                    .filter_map(|point| {
                        sketch
                            .points()
                            .get(point.0)
                            .map(|place| (*point, *place + *by))
                    })
                    .collect();
                sketch.settle_around_all(&dropped, scale);
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
                placement,
            } => {
                let outcome = self.apply_dimension(*sketch, *target, *value);
                if let (Some(offset), Some(drawing)) = (placement, self.sketches.get_mut(*sketch)) {
                    drawing.offset_dimension(*target, *offset);
                }
                outcome
            }
            Operation::Extrude {
                sketch,
                picks,
                distance,
                mode,
            } => {
                self.extrude(*sketch, picks, *distance, *mode);
                None
            }
            Operation::MergePoints {
                sketch,
                kept,
                dropped,
            } => {
                let scale = self.scale();
                let sketch = self.sketches.get_mut(*sketch)?;
                sketch.merge_points(*kept, *dropped);
                sketch.resolve(scale);
                None
            }
            Operation::Constrain { sketch, constraint } => {
                let scale = self.scale();
                let sketch = self.sketches.get_mut(*sketch)?;
                sketch.add_constraint(*constraint);
                sketch.resolve(scale);
                None
            }
            Operation::EraseMany {
                sketch,
                elements,
                dimensions,
                constraints,
            } => {
                let scale = self.scale();
                let sketch = self.sketches.get_mut(*sketch)?;
                for rule in constraints {
                    sketch.erase_constraint(*rule);
                }
                for target in dimensions {
                    sketch.erase_dimension(*target);
                }
                for element in elements {
                    sketch.erase(*element);
                }
                // What is left may have room to move again, so it settles into
                // whatever the remaining values still ask of it.
                sketch.resolve(scale);
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
}

fn resolve(sketch: &mut Sketch, point: &PointRef) -> cao_sketch::PointId {
    match point {
        PointRef::Existing(id) => *id,
        PointRef::New(position) => sketch.add_point(*position),
    }
}

#[cfg(test)]
mod tests {
    use cao_sketch::{DimensionTarget, LengthOutcome, SegmentId, WorkPlane};
    use glam::DVec2;

    use super::*;

    fn chain_history() -> History {
        let mut history = History::default();
        history.push(Operation::CreateSketch {
            plane: WorkPlane::XY,
        });
        history.push(Operation::AddSegment {
            sketch: 0,
            start: PointRef::New(DVec2::ZERO),
            end: PointRef::New(DVec2::new(2.0, 0.0)),
            construction: false,
        });
        history.push(Operation::AddSegment {
            sketch: 0,
            // Point 0 is the sketch origin, so the corner just drawn is 2.
            start: PointRef::Existing(cao_sketch::PointId(2)),
            end: PointRef::New(DVec2::new(2.0, 1.0)),
            construction: false,
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
            placement: None,
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
            placement: None,
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
            placement: None,
        });

        let outcome = state.apply(&Operation::SetDimension {
            sketch: 0,
            target: DimensionTarget::Length(SegmentId(1)),
            value: 100.0,
            placement: None,
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
                start: PointRef::New(DVec2::ZERO),
                end: PointRef::New(DVec2::X),
                construction: false,
            }),
            None
        );
        assert!(state.sketches.is_empty());
    }
}

#[cfg(test)]
mod drawn_shapes {
    use cao_sketch::{SegmentId, WorkPlane};
    use glam::DVec2;

    use super::*;

    #[test]
    fn a_rectangle_is_one_step_with_four_sides() {
        let mut state = PartState::default();
        state.apply(&Operation::CreateSketch {
            plane: WorkPlane::XY,
        });
        state.apply(&Operation::AddRectangle {
            sketch: 0,
            corner: PointRef::New(DVec2::ZERO),
            opposite: PointRef::New(DVec2::new(40.0, 20.0)),
            construction: false,
        });

        let sketch = &state.sketches[0];
        // The origin, plus the four corners.
        assert_eq!(sketch.points().len(), 5);
        assert_eq!(sketch.segments().len(), 4);
        assert!((sketch.segment_length(SegmentId(0)) - 40.0).abs() < 1e-4);
        assert!((sketch.segment_length(SegmentId(1)) - 20.0).abs() < 1e-4);
    }

    #[test]
    fn a_construction_rectangle_flags_all_four_sides_in_the_one_step_that_drew_them() {
        let mut history = History::default();
        history.push(Operation::CreateSketch {
            plane: WorkPlane::XY,
        });
        history.push(Operation::AddRectangle {
            sketch: 0,
            corner: PointRef::New(DVec2::ZERO),
            opposite: PointRef::New(DVec2::new(40.0, 20.0)),
            construction: true,
        });

        let state = PartState::rebuild(&history);
        let sketch = &state.sketches[0];
        assert!(sketch.segments().iter().all(|segment| segment.construction));

        history.undo();
        let after_undo = PartState::rebuild(&history);
        assert!(after_undo.sketches[0].segments().is_empty());
    }

    #[test]
    fn a_symmetric_segment_is_one_step_holding_its_middle() {
        let mut state = PartState::default();
        state.apply(&Operation::CreateSketch {
            plane: WorkPlane::XY,
        });
        state.apply(&Operation::AddSymmetricSegment {
            sketch: 0,
            middle: PointRef::New(DVec2::new(10.0, 0.0)),
            end: PointRef::New(DVec2::new(40.0, 0.0)),
            construction: false,
        });

        let sketch = &state.sketches[0];
        // The origin, the middle, the end, and its mirror image.
        assert_eq!(sketch.points().len(), 4);
        assert_eq!(sketch.segments().len(), 1);
        assert!(
            (sketch.segment_length(SegmentId(0)) - 60.0).abs() < 1e-4,
            "30 each way"
        );
        assert!(
            sketch.constraints().iter().any(|constraint| matches!(
                constraint,
                cao_sketch::Constraint::Midpoint { segment, .. } if *segment == SegmentId(0)
            )),
            "the middle point is held at the segment's midpoint",
        );
    }
}

#[cfg(test)]
mod extrusion_tests {
    use cao_sketch::{DimensionTarget, WorkPlane};
    use glam::{DVec2, DVec3};

    use super::*;
    use crate::history::ExtrusionMode;

    fn volume(mesh: &Mesh) -> f64 {
        mesh.triangles()
            .iter()
            .map(|[a, b, c]| a.dot(b.cross(*c)) / 6.0)
            .sum()
    }

    fn rectangle(history: &mut History, min: DVec2, max: DVec2) {
        history.push(Operation::AddRectangle {
            sketch: 0,
            corner: PointRef::New(min),
            opposite: PointRef::New(max),
            construction: false,
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
        rectangle(&mut history, DVec2::ZERO, DVec2::new(10.0, 20.0));
        history.push(Operation::Extrude {
            sketch: 0,
            picks: vec![DVec2::new(5.0, 10.0)],
            distance: 4.0,
            mode: ExtrusionMode::Add,
        });

        let state = PartState::rebuild(&history);
        assert!((volume(&state.body) - 800.0).abs() < 1.0);
        let (min, max) = state.body.bounds().expect("a volume");
        assert!((max.z - min.z - 4.0).abs() < 1e-3, "the height");
    }

    /// The clicked area is found again by the point, not by its rank: drawing
    /// something else afterwards must not move the extrusion.
    #[test]
    fn an_extrusion_still_names_the_same_area_after_another_shape_is_drawn() {
        let mut history = sketch_history();
        rectangle(&mut history, DVec2::ZERO, DVec2::new(10.0, 10.0));
        rectangle(&mut history, DVec2::new(40.0, 40.0), DVec2::new(50.0, 60.0));
        history.push(Operation::Extrude {
            sketch: 0,
            picks: vec![DVec2::new(45.0, 50.0)],
            distance: 2.0,
            mode: ExtrusionMode::Add,
        });

        let state = PartState::rebuild(&history);
        assert!((volume(&state.body) - 400.0).abs() < 1.0, "the second area");
        let (min, _) = state.body.bounds().expect("a volume");
        assert!(min.x > 39.0, "in the right place: {min}");
    }

    /// Two circles one inside the other make a tube: the tool does not fill the
    /// middle.
    #[test]
    fn two_circles_extrude_to_a_tube() {
        let mut history = sketch_history();
        history.push(Operation::AddCircle {
            sketch: 0,
            center: PointRef::New(DVec2::ZERO),
            radius: 10.0,
            rim: Vec::new(),
            construction: false,
        });
        history.push(Operation::AddCircle {
            sketch: 0,
            center: PointRef::Existing(cao_sketch::PointId(1)),
            radius: 6.0,
            rim: Vec::new(),
            construction: false,
        });
        history.push(Operation::Extrude {
            sketch: 0,
            picks: vec![DVec2::new(8.0, 0.0)],
            distance: 5.0,
            mode: ExtrusionMode::Add,
        });

        let state = PartState::rebuild(&history);
        let expected = std::f64::consts::PI * (100.0 - 36.0) * 5.0;
        let made = volume(&state.body);
        assert!(
            (made - expected).abs() / expected < 0.03,
            "{made} / {expected}"
        );
    }

    /// A pocket: the second sketch digs into the block of the first.
    #[test]
    fn a_cut_takes_matter_away() {
        let mut history = sketch_history();
        rectangle(&mut history, DVec2::ZERO, DVec2::new(10.0, 10.0));
        history.push(Operation::Extrude {
            sketch: 0,
            picks: vec![DVec2::new(5.0, 5.0)],
            distance: 10.0,
            mode: ExtrusionMode::Add,
        });

        history.push(Operation::CreateSketch {
            plane: WorkPlane::XY,
        });
        history.push(Operation::AddRectangle {
            sketch: 1,
            corner: PointRef::New(DVec2::new(2.0, 2.0)),
            opposite: PointRef::New(DVec2::new(4.0, 4.0)),
            construction: false,
        });
        history.push(Operation::Extrude {
            sketch: 1,
            picks: vec![DVec2::new(3.0, 3.0)],
            distance: 4.0,
            mode: ExtrusionMode::Cut,
        });

        let state = PartState::rebuild(&history);
        let made = volume(&state.body);
        assert!((made - (1000.0 - 16.0)).abs() < 2.0, "{made}");
    }

    /// A shape inside another leaves the middle empty from the very first
    /// extrusion: the tube case, in rectangles.
    #[test]
    fn a_shape_inside_another_is_already_hollow() {
        let mut history = sketch_history();
        rectangle(&mut history, DVec2::ZERO, DVec2::new(10.0, 10.0));
        rectangle(&mut history, DVec2::new(2.0, 2.0), DVec2::new(4.0, 4.0));
        history.push(Operation::Extrude {
            sketch: 0,
            picks: vec![DVec2::new(1.0, 1.0)],
            distance: 10.0,
            mode: ExtrusionMode::Add,
        });

        let made = volume(&PartState::rebuild(&history).body);
        assert!((made - (1000.0 - 40.0)).abs() < 2.0, "{made}");
    }

    /// A full revolution around an axis of the sketch.
    #[test]
    fn a_revolution_sweeps_an_area_around_an_axis() {
        let mut history = sketch_history();
        rectangle(&mut history, DVec2::new(3.0, 0.0), DVec2::new(5.0, 2.0));
        history.push(Operation::Revolve {
            sketch: 0,
            picks: vec![DVec2::new(4.0, 1.0)],
            axis: crate::history::RevolutionAxis::Sketch(cao_sketch::SketchAxis::V),
            angle: 360.0,
            mode: ExtrusionMode::Add,
        });

        let made = volume(&PartState::rebuild(&history).body);
        let expected = std::f64::consts::TAU * 4.0 * 4.0;
        assert!(
            (made - expected).abs() / expected < 0.02,
            "{made} / {expected}"
        );
    }

    /// L'axe peut être un trait qu'on a tracé soi-même.
    #[test]
    fn a_revolution_can_turn_around_a_drawn_line() {
        let mut history = sketch_history();
        history.push(Operation::AddSegment {
            sketch: 0,
            start: PointRef::New(DVec2::new(0.0, -10.0)),
            end: PointRef::New(DVec2::new(0.0, 10.0)),
            construction: false,
        });
        rectangle(&mut history, DVec2::new(3.0, 0.0), DVec2::new(5.0, 2.0));
        history.push(Operation::Revolve {
            sketch: 0,
            picks: vec![DVec2::new(4.0, 1.0)],
            axis: crate::history::RevolutionAxis::Segment(cao_sketch::SegmentId(0)),
            angle: 360.0,
            mode: ExtrusionMode::Add,
        });

        let made = volume(&PartState::rebuild(&history).body);
        let expected = std::f64::consts::TAU * 4.0 * 4.0;
        assert!(
            (made - expected).abs() / expected < 0.02,
            "{made} / {expected}"
        );
    }

    /// A profile astride the axis would pass through itself: nothing is
    /// produced, rather than a volume turned inside out.
    #[test]
    fn a_revolution_across_its_axis_makes_nothing() {
        let mut history = sketch_history();
        rectangle(&mut history, DVec2::new(-4.0, 0.0), DVec2::new(5.0, 2.0));
        history.push(Operation::Revolve {
            sketch: 0,
            picks: vec![DVec2::new(1.0, 1.0)],
            axis: crate::history::RevolutionAxis::Sketch(cao_sketch::SketchAxis::V),
            angle: 360.0,
            mode: ExtrusionMode::Add,
        });

        assert!(PartState::rebuild(&history).body.is_empty());
    }

    /// The hang met in use, with its real measurements: a cylinder of
    /// revolution thirty units in radius, then a sketch laid on its top face
    /// and dug into.
    ///
    /// At that distance from the origin a triangle came out of its own plane in
    /// `f64`, and the partition of space cut it again without end.
    #[test]
    fn cutting_into_a_revolved_part_from_its_own_face() {
        let mut history = sketch_history();
        rectangle(
            &mut history,
            DVec2::new(-30.0, -7.5),
            DVec2::new(0.0, -52.5),
        );
        history.push(Operation::Revolve {
            sketch: 0,
            picks: vec![DVec2::new(-14.8, -26.5)],
            axis: crate::history::RevolutionAxis::Sketch(cao_sketch::SketchAxis::V),
            angle: 360.0,
            mode: ExtrusionMode::Add,
        });

        // The plane as the click on the face gives it: its normal and its axes
        // are not exactly aligned, which is part of the case.
        let face = WorkPlane {
            origin: DVec3::new(9.729663e-07, -7.5000005, -1.2972885e-06),
            u: DVec3::new(-1.0, -1.2972883e-07, 0.0),
            v: DVec3::new(2.2439427e-14, -1.7297178e-07, 1.0),
        };
        history.push(Operation::CreateSketch { plane: face });
        history.push(Operation::AddRectangle {
            sketch: 1,
            corner: PointRef::New(DVec2::new(-25.0, 32.5)),
            opposite: PointRef::New(DVec2::new(12.5, -7.5)),
            construction: false,
        });
        history.push(Operation::Extrude {
            sketch: 1,
            picks: vec![DVec2::new(-6.0, 12.0)],
            distance: -10.0,
            mode: ExtrusionMode::Cut,
        });

        let state = PartState::rebuild(&history);
        assert!(!state.body.is_empty());
        assert!(volume(&state.body) > 0.0);
    }

    /// Two superimposed vertices become one, and the replay does it again
    /// identically.
    #[test]
    fn merging_two_points_replays() {
        let mut history = sketch_history();
        history.push(Operation::AddSegment {
            sketch: 0,
            start: PointRef::New(DVec2::new(-10.0, 0.0)),
            end: PointRef::New(DVec2::new(0.0, 10.0)),
            construction: false,
        });
        history.push(Operation::AddSegment {
            sketch: 0,
            start: PointRef::New(DVec2::new(0.0, 10.0)),
            end: PointRef::New(DVec2::new(10.0, 0.0)),
            construction: false,
        });

        let before = PartState::rebuild(&history);
        assert_eq!(before.sketches[0].drawn_points().count(), 4);

        history.push(Operation::MergePoints {
            sketch: 0,
            kept: cao_sketch::PointId(2),
            dropped: cao_sketch::PointId(3),
        });

        let state = PartState::rebuild(&history);
        assert_eq!(state.sketches[0].drawn_points().count(), 3);
        assert_eq!(
            state.sketches[0].live_segments().count(),
            2,
            "both segments still hold the shared corner"
        );
    }

    /// A selection deleted as a block is one single step, undone by one single
    /// undo.
    #[test]
    fn a_whole_selection_goes_in_one_step() {
        let mut history = sketch_history();
        rectangle(&mut history, DVec2::ZERO, DVec2::new(10.0, 10.0));
        history.push(Operation::SetDimension {
            sketch: 0,
            target: DimensionTarget::Length(cao_sketch::SegmentId(0)),
            value: 10.0,
            placement: None,
        });
        let drawn = PartState::rebuild(&history).sketches[0]
            .live_segments()
            .count();
        assert_eq!(drawn, 4);

        history.push(Operation::EraseMany {
            sketch: 0,
            elements: vec![
                cao_sketch::Element::Segment(cao_sketch::SegmentId(0)),
                cao_sketch::Element::Segment(cao_sketch::SegmentId(2)),
            ],
            dimensions: vec![DimensionTarget::Length(cao_sketch::SegmentId(0))],
            constraints: Vec::new(),
        });

        let state = PartState::rebuild(&history);
        assert_eq!(state.sketches[0].live_segments().count(), 2);
        assert!(state.sketches[0].dimensions().is_empty());

        history.undo();
        let back = PartState::rebuild(&history);
        assert_eq!(back.sketches[0].live_segments().count(), 4);
        assert_eq!(back.sketches[0].dimensions().len(), 1);
    }

    /// Deleting and replaying must give the same part: that is what allows
    /// stepping back over a deletion.
    #[test]
    fn a_deletion_replays_like_any_other_step() {
        let mut history = sketch_history();
        rectangle(&mut history, DVec2::ZERO, DVec2::new(10.0, 10.0));
        history.push(Operation::Extrude {
            sketch: 0,
            picks: vec![DVec2::new(5.0, 5.0)],
            distance: 4.0,
            mode: ExtrusionMode::Add,
        });
        let before = volume(&PartState::rebuild(&history).body);
        assert!(before > 0.0);

        history.push(Operation::EraseMany {
            sketch: 0,
            elements: vec![cao_sketch::Element::Segment(cao_sketch::SegmentId(0))],
            dimensions: Vec::new(),
            constraints: Vec::new(),
        });

        let state = PartState::rebuild(&history);
        assert!(state.sketches[0].regions().is_empty(), "the area is open");
        assert!(
            (volume(&state.body) - before).abs() < 1.0,
            "the volume already made stays"
        );

        // And the cursor brought back before the deletion returns the outline.
        history.undo();
        assert_eq!(PartState::rebuild(&history).sketches[0].regions().len(), 1);
    }

    /// A sketch that is not entirely constrained extrudes all the same.
    #[test]
    fn an_extrusion_does_not_wait_for_a_settled_sketch() {
        let mut history = sketch_history();
        rectangle(&mut history, DVec2::new(3.0, 3.0), DVec2::new(9.0, 9.0));
        history.push(Operation::Extrude {
            sketch: 0,
            picks: vec![DVec2::new(5.0, 5.0)],
            distance: 1.0,
            mode: ExtrusionMode::Add,
        });

        let state = PartState::rebuild(&history);
        assert!(!state.sketches[0].is_fully_constrained(1.0));
        assert!(!state.body.is_empty());
    }

    /// A dimension changes the scale: an extrusion of 10 mm stays 10 mm.
    #[test]
    fn an_extrusion_is_given_in_millimetres() {
        let mut history = sketch_history();
        rectangle(&mut history, DVec2::ZERO, DVec2::new(1.0, 1.0));
        history.push(Operation::SetDimension {
            sketch: 0,
            target: cao_sketch::DimensionTarget::Length(cao_sketch::SegmentId(0)),
            value: 50.0,
            placement: None,
        });
        history.push(Operation::Extrude {
            sketch: 0,
            picks: vec![DVec2::new(0.5, 0.5)],
            distance: 25.0,
            mode: ExtrusionMode::Add,
        });

        let state = PartState::rebuild(&history);
        let (min, max) = state.body.bounds().expect("a volume");
        let height_millimetres = (max.z - min.z) * state.scale();
        assert!(
            (height_millimetres - 25.0).abs() < 1e-3,
            "{height_millimetres}"
        );
        let _ = DVec3::ZERO;
    }
}
