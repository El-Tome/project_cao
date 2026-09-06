use cao_sketch::{DimensionTarget, Element, PointId, SegmentId, SketchAxis, WorkPlane};
use glam::DVec2;
use serde::{Deserialize, Serialize};

/// Which point an operation refers to.
///
/// Resolved when the user clicks, never re-derived on replay: snapping depends
/// on the zoom level at the time, so re-running it later could join different
/// points and rebuild a different drawing.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum PointRef {
    Existing(PointId),
    New(DVec2),
}

/// What an extrusion does to the part.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExtrusionMode {
    /// Adds the prism to the part.
    Add,
    /// Takes the prism out of it.
    Cut,
}

impl ExtrusionMode {
    pub fn label(self) -> &'static str {
        match self {
            Self::Add => "Ajout de matière",
            Self::Cut => "Enlèvement de matière",
        }
    }

    pub fn hint(self) -> &'static str {
        match self {
            Self::Add => "Sélectionner des aires fermées, donner une hauteur",
            Self::Cut => "Sélectionner des aires fermées, donner une profondeur",
        }
    }
}

/// What a face is swept around.
///
/// Either one of the sketch's own axes, or a line the user drew. A drawn line
/// is named by its rank in the sketch, which is stable: segments are only ever
/// appended.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RevolutionAxis {
    Sketch(SketchAxis),
    Segment(SegmentId),
}

impl RevolutionAxis {
    pub fn label(self) -> String {
        match self {
            Self::Sketch(axis) => axis.label().to_string(),
            Self::Segment(segment) => format!("trait {}", segment.0),
        }
    }
}

/// One step of the part's history. Replaying the list from the start rebuilds
/// the whole part, which is what makes rolling back to any point possible.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Operation {
    CreateSketch {
        plane: WorkPlane,
    },
    AddPoint {
        sketch: usize,
        position: DVec2,
    },
    AddSegment {
        sketch: usize,
        start: PointRef,
        end: PointRef,
    },
    /// Four corners and four sides in one step, so the history reads as one
    /// rectangle rather than four unrelated lines.
    AddRectangle {
        sketch: usize,
        corner: PointRef,
        opposite: PointRef,
    },
    AddCircle {
        sketch: usize,
        center: PointRef,
        radius: f64,
    },
    /// Dragging a point to a new place.
    MovePoint {
        sketch: usize,
        point: PointId,
        position: DVec2,
    },
    /// Dragging an annotation away from where it sits by default.
    MoveDimension {
        sketch: usize,
        target: DimensionTarget,
        offset: DVec2,
    },
    SetDimension {
        sketch: usize,
        target: DimensionTarget,
        /// Millimetres for a length or radius, degrees for an angle.
        value: f64,
        /// Where the annotation goes, in sketch units. Carried by the same
        /// step rather than a `MoveDimension` of its own: a dimension put down
        /// somewhere is one action, and reading "Cote 60 mm" then "Cote
        /// déplacée" for every single click says nothing extra.
        #[serde(default)]
        placement: Option<DVec2>,
    },
    /// Turns closed areas of a sketch into matter, or takes matter away.
    Extrude {
        sketch: usize,
        /// One position inside each chosen area, in the sketch's own
        /// coordinates.
        ///
        /// The areas are named by a point rather than by their rank: a rank
        /// would move the moment another shape is drawn, and the extrusion
        /// would silently start applying to a different part of the drawing.
        picks: Vec<DVec2>,
        /// Millimetres. Negative goes the other way along the plane.
        distance: f64,
        mode: ExtrusionMode,
    },
    /// Makes two points one, once they have been laid on top of each other.
    ///
    /// Recorded rather than worked out again on replay: which points are close
    /// enough depends on the zoom at the time, so re-deriving it later could
    /// join a different pair — or none.
    MergePoints {
        sketch: usize,
        kept: PointId,
        dropped: PointId,
    },
    /// Deletes a piece of a sketch, and whatever leaned on it.
    Erase {
        sketch: usize,
        element: Element,
    },
    /// Deletes a dimension without touching the geometry it measured.
    EraseDimension {
        sketch: usize,
        target: DimensionTarget,
    },
    /// Deletes everything that was selected, in one step.
    ///
    /// `Erase` and `EraseDimension` above are what a single deletion used to
    /// be, and are kept so that parts drawn before this still replay.
    EraseMany {
        sketch: usize,
        elements: Vec<Element>,
        dimensions: Vec<DimensionTarget>,
    },
    /// Sweeps closed areas of a sketch around an axis lying in its plane.
    Revolve {
        sketch: usize,
        picks: Vec<DVec2>,
        axis: RevolutionAxis,
        /// Degrees. Negative turns the other way.
        angle: f64,
        mode: ExtrusionMode,
    },
}

/// A value as it reads in the history: a dimension taken from the drawing
/// itself is a full float, and "Cote 60.878967 mm" is unreadable.
fn short(value: f64) -> String {
    let text = format!("{value:.2}");
    match text.contains('.') {
        true => text.trim_end_matches('0').trim_end_matches('.').to_string(),
        false => text,
    }
}

impl Operation {
    /// Short name for the history tree.
    pub fn label(&self) -> String {
        match self {
            Self::CreateSketch { plane } => format!("Esquisse — {}", plane.label()),
            Self::AddPoint { .. } => "Point".to_string(),
            Self::AddSegment { .. } => "Trait".to_string(),
            Self::AddRectangle { .. } => "Rectangle".to_string(),
            Self::AddCircle { .. } => "Cercle".to_string(),
            Self::MovePoint { .. } => "Déplacement".to_string(),
            Self::MoveDimension { .. } => "Cote déplacée".to_string(),
            Self::Erase { element, .. } => match element {
                Element::Point(_) => "Point supprimé".to_string(),
                Element::Segment(_) => "Trait supprimé".to_string(),
                Element::Circle(_) => "Cercle supprimé".to_string(),
            },
            Self::EraseDimension { .. } => "Cote supprimée".to_string(),
            Self::EraseMany {
                elements,
                dimensions,
                ..
            } => match (elements.as_slice(), dimensions.as_slice()) {
                ([Element::Point(_)], []) => "Point supprimé".to_string(),
                ([Element::Segment(_)], []) => "Trait supprimé".to_string(),
                ([Element::Circle(_)], []) => "Cercle supprimé".to_string(),
                ([], [_]) => "Cote supprimée".to_string(),
                _ => format!("{} éléments supprimés", elements.len() + dimensions.len()),
            },
            Self::MergePoints { .. } => "Sommets fusionnés".to_string(),
            Self::Revolve { angle, mode, .. } => {
                let verb = match mode {
                    ExtrusionMode::Add => "Révolution",
                    ExtrusionMode::Cut => "Révolution creusée",
                };
                format!("{verb} {angle}°")
            }
            Self::Extrude {
                distance, mode, ..
            } => {
                let verb = match mode {
                    ExtrusionMode::Add => "Extrusion",
                    ExtrusionMode::Cut => "Enlèvement",
                };
                format!("{verb} {distance} mm")
            }
            Self::SetDimension { target, value, .. } => {
                let value = short(*value);
                match target {
                    DimensionTarget::Angle { .. } => format!("Angle {value}°"),
                    DimensionTarget::AxisAngle { axis, .. } => {
                        format!("Angle {value}° / {}", axis.label())
                    }
                    DimensionTarget::Radius(_) => format!("Rayon {value} mm"),
                    DimensionTarget::Projected { axis, .. } => match axis {
                        cao_sketch::SketchAxis::U => format!("Largeur {value} mm"),
                        cao_sketch::SketchAxis::V => format!("Hauteur {value} mm"),
                    },
                    DimensionTarget::PointToSegment { .. }
                    | DimensionTarget::Length(_)
                    | DimensionTarget::Distance { .. } => format!("Cote {value} mm"),
                }
            }
        }
    }

    /// The line shown when a history entry is unfolded.
    pub fn detail(&self) -> String {
        match self {
            Self::CreateSketch { plane } => format!(
                "Plan d'origine ({:.0}, {:.0}, {:.0})",
                plane.normal().x,
                plane.normal().y,
                plane.normal().z
            ),
            Self::AddPoint { sketch, position } => {
                format!("Esquisse {sketch} · ({:.1}, {:.1})", position.x, position.y)
            }
            Self::AddSegment { sketch, start, end } => {
                format!(
                    "Esquisse {sketch} · {} → {}",
                    point_label(start),
                    point_label(end)
                )
            }
            Self::AddRectangle {
                sketch,
                corner,
                opposite,
            } => format!(
                "Esquisse {sketch} · {} → {}",
                point_label(corner),
                point_label(opposite)
            ),
            Self::AddCircle { sketch, radius, .. } => {
                format!("Esquisse {sketch} · rayon {radius:.2}")
            }
            Self::MovePoint {
                sketch,
                point,
                position,
            } => format!(
                "Esquisse {sketch} · point {} vers ({:.1}, {:.1})",
                point.0, position.x, position.y
            ),
            Self::MoveDimension { sketch, offset, .. } => format!(
                "Esquisse {sketch} · décalage ({:.1}, {:.1})",
                offset.x, offset.y
            ),
            Self::Extrude { sketch, picks, .. } => {
                format!("Esquisse {sketch} · {} aire(s)", picks.len())
            }
            Self::Erase { sketch, element } => match element {
                Element::Point(point) => format!("Esquisse {sketch} · point {}", point.0),
                Element::Segment(segment) => format!("Esquisse {sketch} · trait {}", segment.0),
                Element::Circle(circle) => format!("Esquisse {sketch} · cercle {}", circle.0),
            },
            Self::EraseMany {
                sketch,
                elements,
                dimensions,
            } => format!(
                "Esquisse {sketch} · {} tracé(s) et {} cote(s)",
                elements.len(),
                dimensions.len()
            ),
            Self::EraseDimension { sketch, .. } => format!("Esquisse {sketch}"),
            Self::MergePoints {
                sketch,
                kept,
                dropped,
            } => format!("Esquisse {sketch} · points {} et {}", kept.0, dropped.0),
            Self::Revolve {
                sketch, picks, axis, ..
            } => format!(
                "Esquisse {sketch} · {} aire(s) autour de {}",
                picks.len(),
                axis.label()
            ),
            Self::SetDimension { sketch, target, .. } => match target {
                DimensionTarget::Distance { from, to } => {
                    format!("Esquisse {sketch} · points {} et {}", from.0, to.0)
                }
                DimensionTarget::Length(segment) => {
                    format!("Esquisse {sketch} · trait {}", segment.0)
                }
                DimensionTarget::Angle { first, second } => {
                    format!("Esquisse {sketch} · traits {} et {}", first.0, second.0)
                }
                DimensionTarget::AxisAngle { segment, axis } => {
                    format!("Esquisse {sketch} · trait {} / {}", segment.0, axis.label())
                }
                DimensionTarget::PointToSegment { point, segment } => {
                    format!("Esquisse {sketch} · point {} au trait {}", point.0, segment.0)
                }
                DimensionTarget::Projected { from, to, axis } => format!(
                    "Esquisse {sketch} · points {} et {} sur l'{}",
                    from.0,
                    to.0,
                    axis.label()
                ),
                DimensionTarget::Radius(circle) => {
                    format!("Esquisse {sketch} · cercle {}", circle.0)
                }
            },
        }
    }

    /// True for the operations that open a new feature in the tree, and under
    /// which the following ones are grouped.
    pub fn starts_feature(&self) -> bool {
        matches!(
            self,
            Self::CreateSketch { .. } | Self::Extrude { .. } | Self::Revolve { .. }
        )
    }
}

fn point_label(point: &PointRef) -> String {
    match point {
        PointRef::Existing(id) => format!("point {}", id.0),
        PointRef::New(position) => format!("({:.1}, {:.1})", position.x, position.y),
    }
}

/// Everything done to a part, in order, with a cursor separating what is
/// applied from what can be redone.
///
/// The part's geometry is not stored: it is rebuilt by replaying this list. So
/// undo, redo and "go back to this step" are all the same operation — moving
/// the cursor — and the redo tail survives being saved and reopened.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct History {
    operations: Vec<Operation>,
    applied: usize,
}

impl History {
    pub fn operations(&self) -> &[Operation] {
        &self.operations
    }

    /// How many operations are currently in effect.
    pub fn applied(&self) -> usize {
        self.applied
    }

    pub fn applied_operations(&self) -> &[Operation] {
        &self.operations[..self.applied]
    }

    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }

    pub fn can_undo(&self) -> bool {
        self.applied > 0
    }

    pub fn can_redo(&self) -> bool {
        self.applied < self.operations.len()
    }

    /// Records a new operation. Anything that had been undone is dropped: the
    /// part has taken a different turn, and keeping the old branch would leave
    /// a redo that no longer follows from what is on screen.
    pub fn push(&mut self, operation: Operation) {
        self.operations.truncate(self.applied);
        self.operations.push(operation);
        self.applied = self.operations.len();
    }

    pub fn undo(&mut self) -> bool {
        if !self.can_undo() {
            return false;
        }
        self.applied -= 1;
        true
    }

    pub fn redo(&mut self) -> bool {
        if !self.can_redo() {
            return false;
        }
        self.applied += 1;
        true
    }

    /// Rewrites every operation in place. Only meant for bringing an older
    /// file up to date; nothing else should reach past the cursor.
    pub fn map_operations(&mut self, mut change: impl FnMut(&mut Operation)) {
        for operation in &mut self.operations {
            change(operation);
        }
    }

    /// Moves the cursor anywhere in the list, which is how the history tree
    /// puts the part back the way it was at a given step.
    pub fn rewind_to(&mut self, applied: usize) {
        self.applied = applied.min(self.operations.len());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn segment_op(sketch: usize) -> Operation {
        Operation::AddSegment {
            sketch,
            start: PointRef::New(DVec2::ZERO),
            end: PointRef::New(DVec2::X),
        }
    }

    #[test]
    fn undo_and_redo_walk_the_list() {
        let mut history = History::default();
        history.push(Operation::CreateSketch {
            plane: WorkPlane::XY,
        });
        history.push(segment_op(0));
        assert_eq!(history.applied(), 2);
        assert!(!history.can_redo());

        assert!(history.undo());
        assert_eq!(history.applied(), 1);
        assert!(history.can_redo());
        assert_eq!(history.operations().len(), 2, "the tail is kept for redo");

        assert!(history.redo());
        assert_eq!(history.applied(), 2);
        assert!(!history.redo());
    }

    #[test]
    fn undoing_past_the_start_does_nothing() {
        let mut history = History::default();
        assert!(!history.undo());
        assert!(!history.can_undo());
    }

    /// Drawing something new after an undo abandons the branch that was undone.
    #[test]
    fn a_new_operation_drops_what_was_undone() {
        let mut history = History::default();
        history.push(Operation::CreateSketch {
            plane: WorkPlane::XY,
        });
        history.push(segment_op(0));
        history.undo();

        history.push(segment_op(1));

        assert_eq!(history.operations().len(), 2);
        assert_eq!(history.applied(), 2);
        assert!(!history.can_redo());
        assert_eq!(history.operations()[1], segment_op(1));
    }

    #[test]
    fn rewinding_keeps_everything_for_redo() {
        let mut history = History::default();
        for _ in 0..5 {
            history.push(segment_op(0));
        }

        history.rewind_to(2);

        assert_eq!(history.applied(), 2);
        assert_eq!(history.applied_operations().len(), 2);
        assert_eq!(history.operations().len(), 5);
        assert!(history.can_redo());
    }

    #[test]
    fn rewinding_past_the_end_is_clamped() {
        let mut history = History::default();
        history.push(segment_op(0));
        history.rewind_to(99);
        assert_eq!(history.applied(), 1);
    }
}
