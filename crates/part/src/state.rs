use std::collections::{BTreeMap, BTreeSet};

use cao_sketch::Sketch;
use cao_solid::Mesh;
use glam::DVec2;
use serde::{Deserialize, Serialize};

use crate::descent::Descent;
use crate::history::{History, Operation};
use crate::outcome::Outcome;

/// The geometry of a part at a given point in its history.
///
/// Rebuilt by replaying the history, which is what keeps "go back to this
/// step" honest. What a part file holds of it is a cache and says which design
/// it was rebuilt from, so that the list of operations stays the one thing the
/// geometry can be read from.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PartState {
    /// Millimetres one world unit is worth, undefined until the first
    /// dimension is typed.
    pub millimeters_per_unit: Option<f64>,
    pub sketches: Vec<Sketch>,
    /// The drawings that were pointed at a corner the part no longer has.
    /// Their points stay where they were drawn, loose, and the interface says
    /// so rather than letting them quietly catch the corner next door.
    #[serde(default)]
    pub(crate) unanchored: BTreeSet<usize>,
    /// The drawings whose face the part no longer has. They keep the plane
    /// they last had, and the interface says so rather than letting them
    /// quietly catch another face.
    #[serde(default)]
    pub adrift: BTreeSet<usize>,
    /// What each drawing's curves became, as the replay cut them. It is what
    /// lets a step of matter find the area it was raised from after a corner
    /// of that area has been rounded or a border of it divided.
    ///
    /// Scratch for the length of a replay, and kept out of the cache the part
    /// file holds: the history already says what was cut, and a cache that
    /// carries a second copy of something the history says is a cache that
    /// can disagree with it.
    #[serde(skip)]
    pub(crate) descent: BTreeMap<usize, Descent>,
    /// The matter of the part, as one surface. Extrusions add to it or take
    /// from it; there is a single body rather than a pile of separate lumps,
    /// so that a pocket cut in a block really is a hole in the block.
    pub body: Mesh,
}

impl PartState {
    pub fn rebuild(history: &History) -> Self {
        let mut state = Self::default();
        // Step by step, each step whole — not in the order things were typed.
        // A corner of a sketch dragged long after an extrusion was raised from
        // it is played with that sketch, so the extrusion is raised again.
        for operation in history.replay_order() {
            state.apply(operation);
        }
        state
    }

    /// Whether a drawing was pointed at a corner the part no longer has.
    pub fn is_unanchored(&self, sketch: usize) -> bool {
        self.unanchored.contains(&sketch)
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
            Operation::CreateSketch { plane, on } => {
                let plane = self.plane_for(*plane, on);
                self.sketches.push(Sketch::new(plane));
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
                let start = self.point_for(*sketch, start)?;
                let end = self.point_for(*sketch, end)?;
                let sketch = self.sketches.get_mut(*sketch)?;
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
                let middle = self.point_for(*sketch, middle)?;
                let end = self.point_for(*sketch, end)?;
                let sketch = self.sketches.get_mut(*sketch)?;
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
                // The two given corners may reuse points already drawn; the
                // other two are always new.
                let first = self.point_for(*sketch, corner)?;
                let third = self.point_for(*sketch, opposite)?;
                let sketch = self.sketches.get_mut(*sketch)?;
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
                merged_into,
            } => {
                let scale = self.scale();
                let sketch = self.sketches.get_mut(*sketch)?;
                // Moving a point by hand must not break the values already
                // given, so the drawing settles again around it — around it,
                // the point itself staying exactly where it was dropped.
                sketch.settle_around(*point, *position, scale);
                if let Some(kept) = merged_into {
                    sketch.merge_points(*kept, *point);
                    sketch.resolve(scale);
                }
                None
            }
            Operation::AddCircle {
                sketch,
                center,
                radius,
                rim,
                construction,
            } => {
                let index = *sketch;
                let center = self.point_for(index, center)?;
                let drawing = self.sketches.get_mut(index)?;
                let circle = if *construction {
                    drawing.add_construction_circle(center, *radius)
                } else {
                    drawing.add_circle(center, *radius)
                };
                for place in rim {
                    let point = self.point_for(index, place)?;
                    self.sketches
                        .get_mut(index)?
                        .add_constraint(cao_sketch::Constraint::OnCircle { point, circle });
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
                let index = *sketch;
                let center = self.point_for(index, center)?;
                let start = self.point_for(index, start)?;
                let end = self.point_for(index, end)?;
                let sketch = self.sketches.get_mut(index)?;
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
                areas,
                distance,
                mode,
            } => {
                self.extrude(*sketch, areas, *distance, *mode);
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
            } => self.trim(*sketch, *segment, *from, *to),
            Operation::TrimArc {
                sketch,
                arc,
                from,
                to,
            } => self.trim_arc(*sketch, *arc, *from, *to),
            Operation::Split {
                sketch,
                segments,
                arcs,
                at,
            } => self.split(*sketch, segments, arcs, *at),
            Operation::Chamfer {
                sketch,
                first,
                second,
                mode,
            } => self.chamfer(*sketch, *first, *second, *mode),
            Operation::Fillet {
                sketch,
                first,
                second,
                radius,
            } => self.fillet(*sketch, *first, *second, *radius),
            Operation::Mirror {
                sketch,
                elements,
                axis,
            } => self.mirror(*sketch, elements, *axis),
            Operation::CircularPattern {
                sketch,
                elements,
                centre,
                degrees,
                count,
            } => self.pattern_around(*sketch, elements, *centre, *degrees, *count),
            Operation::RectangularPattern {
                sketch,
                elements,
                direction,
                along,
                across,
            } => self.pattern_along(*sketch, elements, *direction, *along, *across),
            Operation::Revolve {
                sketch,
                areas,
                axis,
                angle,
                mode,
            } => {
                self.revolve(*sketch, areas, *axis, *angle, *mode);
                None
            }
        }
    }
}
