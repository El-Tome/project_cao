use std::collections::{BTreeMap, BTreeSet};

use cao_sketch::{Sketch, Support};
use cao_solid::Mesh;
use glam::DVec2;
use serde::{Deserialize, Serialize};

use crate::descent::Descent;
use crate::history::{History, Operation, PointRef};
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
            Operation::AddPoint {
                sketch,
                position,
                on,
            } => {
                let drawing = self.sketches.get_mut(*sketch)?;
                let point = drawing.add_point(*position);
                hold(drawing, point, on);
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
                merged_into,
                on,
                let_go,
            } => {
                let scale = self.scale();
                let sketch = self.sketches.get_mut(*sketch)?;
                // What the drag decided about what holds the point, before it
                // moves: a rule laid on the place it is dropped at is already
                // satisfied there, and the settling has one thing less to do.
                if *let_go {
                    sketch.let_go(*point);
                }
                hold(sketch, *point, on);
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
            Operation::ResizeCircle {
                sketch,
                circle,
                reach,
            } => self.resize_circle(*sketch, *circle, *reach),
            Operation::ResizeArc { sketch, arc, reach } => self.resize_arc(*sketch, *arc, *reach),
            Operation::AddCircle {
                sketch,
                center,
                radius,
                rim,
                construction,
            } => self.add_circle(*sketch, center, *radius, rim, *construction),
            Operation::AddArc {
                sketch,
                center,
                start,
                end,
                construction,
            } => self.add_arc(*sketch, [center, start, end], *construction),
            Operation::AddEllipse {
                sketch,
                center,
                first,
                second,
                construction,
            } => self.add_ellipse(*sketch, center, first, second, *construction),
            Operation::ResizeEllipse {
                sketch,
                ellipse,
                reach,
            } => self.resize_ellipse(*sketch, *ellipse, *reach),
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
            Operation::TrimCircle {
                sketch,
                circle,
                between,
            } => self.trim_circle(*sketch, *circle, *between),
            Operation::Split {
                sketch,
                segments,
                arcs,
                at,
            } => self.split(*sketch, segments, arcs, *at),
            Operation::Chamfer {
                sketch,
                corners,
                mode,
            } => self.chamfer(*sketch, corners, *mode),
            Operation::Fillet {
                sketch,
                corners,
                radius,
            } => self.fillet(*sketch, corners, *radius),
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

pub(crate) fn resolve(sketch: &mut Sketch, point: &PointRef) -> cao_sketch::PointId {
    match point {
        PointRef::Existing(id) => *id,
        PointRef::New(position) => sketch.add_point(*position),
        PointRef::Held { at, on } => {
            let laid = sketch.add_point(*at);
            hold(sketch, laid, on);
            laid
        }
    }
}

/// Holds a point on everything it was laid on. The rules are already true
/// where it stands, so nothing moves until the drawing is next settled.
fn hold(sketch: &mut Sketch, point: cao_sketch::PointId, on: &[Support]) {
    for support in on {
        sketch.add_constraint(support.holding(point));
    }
}
