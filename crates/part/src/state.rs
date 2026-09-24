use std::collections::{BTreeMap, BTreeSet};
use std::ops::Range;

use cao_sketch::{Sketch, Support};
use cao_solid::Mesh;
use glam::DVec2;
use serde::{Deserialize, Serialize};

use crate::broken::Broken;
use crate::descent::Descent;
use crate::history::{History, Operation, PointRef};
use crate::outcome::Outcome;
use crate::replay::Replay;
use crate::variables::Variables;

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
    /// The part's variables as the history in effect leaves them, and what
    /// each comes to. Played before any step, so that every size written from
    /// them reads the table as it stands now — which is what makes a size
    /// follow its variable. Kept out of the cache for the reason `descent` is.
    #[serde(skip)]
    pub(crate) variables: Variables,
    #[serde(skip)]
    pub(crate) values: Vec<f64>,
    /// The sizes the replay could not honour, and the number of the operation
    /// being replayed, which is what names a step whose size does not hold.
    #[serde(skip)]
    pub(crate) broken: Vec<Broken>,
    #[serde(skip)]
    pub(crate) replaying: u32,
    /// What the replay noted on the way: what a change to the variables is
    /// held to, and the values it froze.
    #[serde(skip)]
    pub(crate) replay: Replay,
    /// The matter of the part, as one surface. Extrusions add to it or take
    /// from it; there is a single body rather than a pile of separate lumps,
    /// so that a pocket cut in a block really is a hole in the block.
    pub body: Mesh,
    /// The numbers of the faces each step of matter made, in the order the
    /// steps replay. Kept with the body it numbers, in the cache as well.
    #[serde(default)]
    pub(crate) made: Vec<Range<usize>>,
}

impl PartState {
    /// Plays the table of variables out of the history, and works out what
    /// each comes to.
    pub(crate) fn read_variables(&mut self, history: &History) {
        let mut variables = Variables::default();
        for change in history.variable_changes() {
            variables.change(change);
        }
        self.values = variables.values();
        self.variables = variables;
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
            // The last word wins: a gesture whose parts each have something to
            // say says the one the user acted on last.
            Operation::Gesture(done) => done.iter().filter_map(|one| self.apply(one)).last(),
            Operation::Variable(change) => {
                self.variables.change(change);
                self.values = self.variables.values();
                None
            }
            Operation::CreateSketch { plane, on } => {
                let plane = self.plane_for(*plane, on);
                self.sketches.push(Sketch::new(plane));
                None
            }
            Operation::AddPoint {
                sketch,
                position,
                on,
            } => self.add_point(*sketch, *position, on),
            Operation::AddSegment {
                sketch,
                start,
                end,
                construction,
            } => self.add_segment(*sketch, start, end, *construction),
            Operation::AddSymmetricSegment {
                sketch,
                middle,
                end,
                construction,
            } => self.add_symmetric_segment(*sketch, middle, end, *construction),
            Operation::AddRectangle {
                sketch,
                corner,
                opposite,
                construction,
            } => self.add_rectangle(*sketch, corner, opposite, *construction),
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
                drawn,
            } => self.add_ellipse(*sketch, center, first, second, *construction, drawn),
            Operation::TrimEllipse {
                sketch,
                ellipse,
                between,
            } => self.trim_ellipse(*sketch, *ellipse, *between),
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
            } => self.apply_written_dimension(*sketch, *target, value, *placement),
            Operation::Extrude {
                sketch,
                areas,
                distance,
                mode,
            } => {
                self.raising(|state| state.extrude(*sketch, areas, distance, *mode));
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
            } => self.chamfer(*sketch, corners, mode),
            Operation::Fillet {
                sketch,
                corners,
                radius,
            } => self.fillet(*sketch, corners, radius),
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
            } => self.pattern_around(*sketch, elements, *centre, degrees, count),
            Operation::RectangularPattern {
                sketch,
                elements,
                direction,
                along,
                across,
            } => self.pattern_along(*sketch, elements, *direction, along, across),
            Operation::Revolve {
                sketch,
                areas,
                axis,
                angle,
                mode,
            } => {
                self.raising(|state| state.revolve(*sketch, areas, *axis, angle, *mode));
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
pub(crate) fn hold(sketch: &mut Sketch, point: cao_sketch::PointId, on: &[Support]) {
    for support in on {
        sketch.add_constraint(support.holding(point));
    }
}
