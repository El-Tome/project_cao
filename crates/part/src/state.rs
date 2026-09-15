use cao_sketch::{Chamfer, Sketch};
use cao_solid::Mesh;
use glam::DVec2;

use crate::history::{History, Operation, PointRef};
use crate::outcome::Outcome;

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
    pub fn apply(&mut self, operation: &Operation) -> Option<Outcome> {
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
            Operation::AddArc {
                sketch,
                center,
                start,
                end,
                construction,
            } => {
                let sketch = self.sketches.get_mut(*sketch)?;
                let [center, start, end] = [center, start, end].map(|place| resolve(sketch, place));
                match *construction {
                    true => sketch.add_construction_arc(center, start, end),
                    false => sketch.add_arc(center, start, end),
                };
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
                outcome.map(Outcome::Dimension)
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
            Operation::Trim {
                sketch,
                segment,
                from,
                to,
            } => {
                let scale = self.scale();
                let sketch = self.sketches.get_mut(*sketch)?;
                let trimmed = sketch.trim(*segment, *from, *to)?;
                sketch.resolve(scale);
                Some(Outcome::Cut {
                    rules: trimmed.rules_dropped,
                    values: trimmed.values_dropped,
                })
            }
            Operation::TrimArc {
                sketch,
                arc,
                from,
                to,
            } => {
                let scale = self.scale();
                let sketch = self.sketches.get_mut(*sketch)?;
                let trimmed = sketch.trim_arc(*arc, *from, *to)?;
                sketch.resolve(scale);
                Some(Outcome::Cut {
                    rules: trimmed.rules_dropped,
                    values: trimmed.values_dropped,
                })
            }
            Operation::Split {
                sketch,
                segments,
                arcs,
                at,
            } => {
                let scale = self.scale();
                let sketch = self.sketches.get_mut(*sketch)?;
                let split = sketch.split(segments, arcs, *at)?;
                sketch.resolve(scale);
                Some(Outcome::Cut {
                    rules: split.rules_dropped,
                    values: split.values_dropped,
                })
            }
            Operation::Chamfer {
                sketch,
                first,
                second,
                mode,
            } => {
                let scale = self.scale();
                let sketch = self.sketches.get_mut(*sketch)?;
                let chamfered = sketch.chamfer(*first, *second, in_units(*mode, scale))?;
                sketch.resolve(scale);
                Some(Outcome::Cut {
                    rules: chamfered.rules_dropped,
                    values: chamfered.values_dropped,
                })
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

/// A chamfer as the drawing measures it. The history records millimetres, the
/// way every other length the user types is recorded; the sketch works in its
/// own units, and only the distances convert.
fn in_units(mode: Chamfer, millimeters_per_unit: f64) -> Chamfer {
    match mode {
        Chamfer::Equal(reach) => Chamfer::Equal(reach / millimeters_per_unit),
        Chamfer::Sided { first, second } => Chamfer::Sided {
            first: first / millimeters_per_unit,
            second: second / millimeters_per_unit,
        },
        Chamfer::Angled { along, degrees } => Chamfer::Angled {
            along: along / millimeters_per_unit,
            degrees,
        },
    }
}
