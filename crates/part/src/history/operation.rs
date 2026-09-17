//! One step of what was done to a part: what it names, and what it asks
//! for. Replaying the list of them is what produces the geometry.

use cao_sketch::{
    ArcId, Chamfer, ChosenAxis, Constraint, DimensionTarget, Element, PointId, Repeats, SegmentId,
    SketchAxis, WorkPlane,
};
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
        /// Helps build the drawing without becoming part of it: excluded from
        /// area.
        #[serde(default)]
        construction: bool,
    },
    /// A trait growing equally on both sides of `middle`, one step holding the
    /// segment and the constraint that keeps `middle` at its centre — so the
    /// history reads as one symmetric line rather than a segment and a rule
    /// added by hand afterwards.
    AddSymmetricSegment {
        sketch: usize,
        middle: PointRef,
        end: PointRef,
        #[serde(default)]
        construction: bool,
    },
    /// Four corners and four sides in one step, so the history reads as one
    /// rectangle rather than four unrelated lines.
    AddRectangle {
        sketch: usize,
        corner: PointRef,
        opposite: PointRef,
        #[serde(default)]
        construction: bool,
    },
    AddCircle {
        sketch: usize,
        center: PointRef,
        radius: f64,
        /// The places clicked on the rim, kept as real points held on the
        /// circle. They are what a circle drawn by its points can be grabbed
        /// by: dragging one resizes the circle rather than leaving a stray
        /// point behind.
        #[serde(default)]
        rim: Vec<PointRef>,
        #[serde(default)]
        construction: bool,
    },
    /// A piece of a circle, as the three places it stands on. No radius: it is
    /// read off the end the curve starts at, so a rebuild cannot produce an
    /// arc whose ends disagree with the size it was saved under.
    AddArc {
        sketch: usize,
        center: PointRef,
        start: PointRef,
        end: PointRef,
        #[serde(default)]
        construction: bool,
    },
    /// Dragging a point to a new place, and the corner it was laid on top of
    /// when it landed on one.
    ///
    /// The joining is carried by the same step rather than a `MergePoints` of
    /// its own: dropping a corner on another is one gesture of the user, and
    /// one undo has to take the whole of it back. Leaving them apart left a
    /// drawing that read closed and was not, its area gone, and two undos to
    /// get back to one drag.
    ///
    /// Which corner it lands on is decided at the drop and recorded, never
    /// worked out again on replay: how close is close enough depends on the
    /// zoom at the time, so re-deriving it later could join a different pair,
    /// or none.
    MovePoint {
        sketch: usize,
        point: PointId,
        position: DVec2,
        #[serde(default)]
        merged_into: Option<PointId>,
    },
    /// Dragging a whole selection: every point named moves by the same step,
    /// so the shapes travel together instead of being pulled apart.
    MoveMany {
        sketch: usize,
        points: Vec<PointId>,
        by: DVec2,
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
    /// Lays down a rule with no value: perpendicular, parallel, equal…
    Constrain {
        sketch: usize,
        constraint: Constraint,
    },
    /// Deletes everything that was selected, in one step.
    EraseMany {
        sketch: usize,
        elements: Vec<Element>,
        dimensions: Vec<DimensionTarget>,
        #[serde(default)]
        constraints: Vec<Constraint>,
    },
    /// Takes a stretch out of a trait, between two of the points sitting on
    /// it. What is left of the trait stays, as traits of its own.
    ///
    /// The two points are recorded rather than worked out again on replay:
    /// which points count as sitting on a trait depends on the reach the
    /// cursor had at the time, so re-deriving it later could cut elsewhere.
    Trim {
        sketch: usize,
        segment: SegmentId,
        from: PointId,
        to: PointId,
    },
    /// The same, on a curve: what is left of the arc stays as arcs around the
    /// centre it already turned about.
    TrimArc {
        sketch: usize,
        arc: ArcId,
        from: PointId,
        to: PointId,
    },
    /// Drops a point where curves cross and cuts each of them in two there.
    ///
    /// The curves are recorded rather than worked out again on replay, for the
    /// reason `Trim` records its points: which curves a click finds depends on
    /// the reach the cursor had at the time, and re-deriving it later could
    /// divide elsewhere.
    ///
    /// The arcs came later than the traits, so a division a past release wrote
    /// names none and divides traits alone, exactly as it did then.
    Split {
        sketch: usize,
        segments: Vec<SegmentId>,
        #[serde(default)]
        arcs: Vec<ArcId>,
        at: DVec2,
    },
    /// Cuts the corner two traits share with a straight line, pulling each of
    /// them back from it by what the mode asks.
    ///
    /// The two traits are recorded rather than worked out again on replay, for
    /// the reason `Trim` records its points.
    Chamfer {
        sketch: usize,
        first: SegmentId,
        second: SegmentId,
        mode: Chamfer,
    },
    /// Lays a second copy of what was selected on the other side of an axis.
    ///
    /// The elements are recorded rather than worked out again on replay, for
    /// the reason `Trim` records its points: what a click took hold of depends
    /// on the reach the cursor had at the time.
    Mirror {
        sketch: usize,
        elements: Vec<Element>,
        axis: ChosenAxis,
    },
    /// Repeats what was selected around a point of the drawing, one step
    /// further round for each copy. The count is how many stand there in the
    /// end, the original among them.
    CircularPattern {
        sketch: usize,
        elements: Vec<Element>,
        centre: PointId,
        /// Degrees between one copy and the next.
        degrees: f64,
        count: usize,
    },
    /// Repeats what was selected in rows square to a direction of the drawing.
    /// Each count is how many stand there in the end along its own direction,
    /// the original among them.
    RectangularPattern {
        sketch: usize,
        elements: Vec<Element>,
        direction: ChosenAxis,
        /// Along the direction, its step in millimetres like every other
        /// length the user types.
        along: Repeats,
        /// Square to it, the same.
        across: Repeats,
    },
    /// Rounds the corner two traits share into a curve tangent to both.
    Fillet {
        sketch: usize,
        first: SegmentId,
        second: SegmentId,
        /// Millimetres, like every other length the user types.
        radius: f64,
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
