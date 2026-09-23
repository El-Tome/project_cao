//! One step of what was done to a part: what it names, and what it asks
//! for. Replaying the list of them is what produces the geometry.

use cao_sketch::{
    ArcId, Area, Chamfer, ChosenAxis, CircleId, Constraint, Corner, DimensionTarget, Element,
    EllipseId, PointId, Repeats, SegmentId, Support, WorkPlane,
};
use glam::DVec2;
use serde::{Deserialize, Serialize};

mod point_ref;
mod raising;
pub use point_ref::PointRef;
pub use raising::{ExtrusionMode, FaceAnchor, RevolutionAxis};

impl Operation {
    /// The sketch this operation edits, when it edits one.
    ///
    /// It is what says which step the operation belongs to, and so when it is
    /// replayed: a corner of the first sketch dragged long after an extrusion
    /// was raised from it is still the first sketch's business, and is played
    /// before that extrusion.
    ///
    /// The three that open a step answer `None` — they are a step rather than
    /// something recorded under one, and `Extrude` and `Revolve` name the
    /// sketch they stand on rather than one they change.
    ///
    /// The match has no wildcard arm, so an operation added later has to say
    /// where it belongs instead of quietly landing wherever the list ends.
    pub(crate) fn edits(&self) -> Option<usize> {
        match self {
            Self::CreateSketch { .. } | Self::Extrude { .. } | Self::Revolve { .. } => None,
            Self::AddPoint { sketch, .. }
            | Self::AddSegment { sketch, .. }
            | Self::AddSymmetricSegment { sketch, .. }
            | Self::AddRectangle { sketch, .. }
            | Self::AddCircle { sketch, .. }
            | Self::AddArc { sketch, .. }
            | Self::AddEllipse { sketch, .. }
            | Self::MovePoint { sketch, .. }
            | Self::ResizeCircle { sketch, .. }
            | Self::ResizeArc { sketch, .. }
            | Self::ResizeEllipse { sketch, .. }
            | Self::MoveMany { sketch, .. }
            | Self::MoveDimension { sketch, .. }
            | Self::SetDimension { sketch, .. }
            | Self::MergePoints { sketch, .. }
            | Self::Constrain { sketch, .. }
            | Self::EraseMany { sketch, .. }
            | Self::Trim { sketch, .. }
            | Self::TrimArc { sketch, .. }
            | Self::TrimCircle { sketch, .. }
            | Self::TrimEllipse { sketch, .. }
            | Self::Split { sketch, .. }
            | Self::Chamfer { sketch, .. }
            | Self::Fillet { sketch, .. }
            | Self::Mirror { sketch, .. }
            | Self::CircularPattern { sketch, .. }
            | Self::RectangularPattern { sketch, .. } => Some(*sketch),
        }
    }
}

/// One step of the part's history. Replaying the list from the start rebuilds
/// the whole part, which is what makes rolling back to any point possible.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Operation {
    CreateSketch {
        /// Where the drawing was laid. On a face it is worked out again at
        /// every replay from `on`, and this is what is left to fall back on
        /// when the face is gone.
        plane: WorkPlane,
        /// The face of the part it was started on, when it was started on one.
        /// A drawing laid on a face travels with it; one laid on a plane of
        /// the origin is held by nothing and never moves.
        on: Option<FaceAnchor>,
    },
    AddPoint {
        sketch: usize,
        position: DVec2,
        /// What the point was laid on, and is held by.
        #[serde(default)]
        on: Vec<Support>,
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
    /// An ellipse, as its centre and the two ends of each of its axes. The axes
    /// are laid with it as construction traits, in the same step.
    ///
    /// `drawn` is the stretch of it left after a cut, as the two points it runs
    /// between. Nothing while the whole curve is drawn, which is how one is
    /// laid; compaction writes it out so that an arc of ellipse comes back as
    /// the one it is rather than as the whole curve.
    AddEllipse {
        sketch: usize,
        center: PointRef,
        first: [PointRef; 2],
        second: [PointRef; 2],
        #[serde(default)]
        construction: bool,
        #[serde(default)]
        drawn: Option<[PointRef; 2]>,
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
        /// What the point was dropped on, and is held by from now on. A point
        /// dropped on a trait is held there exactly as one born on it is.
        #[serde(default)]
        on: Vec<Support>,
        /// Whether the drag pulled the point off whatever held it, which is
        /// what the let-go key asks for.
        #[serde(default)]
        let_go: bool,
    },
    /// A circle drawn to a new size by hand, about the centre it already has.
    ///
    /// The size, not a value: a shape pushed about with the mouse says how big
    /// it is now, never how big it must stay. Recorded as a dimension instead,
    /// every rough drag would leave a number to delete afterwards.
    ResizeCircle {
        sketch: usize,
        circle: CircleId,
        /// How far the rim now stands from the centre, in world units.
        reach: f64,
    },
    /// The same for an arc: its two ends travel out to the new reach, keeping
    /// the sweep the curve was drawn with. An arc holds no size of its own —
    /// it is read off the end it starts at — so this is where its two ends go.
    ResizeArc {
        sketch: usize,
        arc: ArcId,
        reach: f64,
    },
    /// Dragging an ellipse by its curve: scaled whole about its centre, until
    /// its first axis reaches `reach` from it.
    ResizeEllipse {
        sketch: usize,
        ellipse: EllipseId,
        reach: f64,
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
        /// Each chosen area, named by the curves that bounded it at the
        /// moment it was clicked.
        ///
        /// Not by its rank, which would move the instant another shape is
        /// drawn, and no longer by the place clicked alone: pull the drawing
        /// far enough and no area passes under that place any more, and the
        /// step would quietly raise nothing. The place is kept inside the
        /// name, and read only to tell apart two areas the same curves bound.
        areas: Vec<Area>,
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
    /// The same, on a circle: the stretch running counter-clockwise from the
    /// first point round to the second goes, and what is left comes back as an
    /// arc of that very circle.
    ///
    /// Nothing to cut between — a circle carrying fewer than two points — and
    /// the whole round goes, since the stretch "between two points" is then the
    /// round itself.
    TrimCircle {
        sketch: usize,
        circle: CircleId,
        between: Option<(PointId, PointId)>,
    },
    /// Takes a stretch out of an ellipse, which leaves the same ellipse with
    /// that stretch gone — and a second piece of it when the stretch came out
    /// of the middle of one already cut.
    TrimEllipse {
        sketch: usize,
        ellipse: EllipseId,
        between: Option<(PointId, PointId)>,
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
    /// Every corner the one gesture named, cut with the same values. They are
    /// one operation so that undo takes back the gesture rather than a quarter
    /// of it, and so the history reads as the one thing that was done.
    Chamfer {
        sketch: usize,
        corners: Vec<Corner>,
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
    /// Rounds every corner the one gesture named into a curve tangent to both
    /// its traits, for the reason `Chamfer` carries several.
    Fillet {
        sketch: usize,
        corners: Vec<Corner>,
        /// Millimetres, like every other length the user types.
        radius: f64,
    },
    /// Sweeps closed areas of a sketch around an axis lying in its plane.
    Revolve {
        sketch: usize,
        areas: Vec<Area>,
        axis: RevolutionAxis,
        /// Degrees. Negative turns the other way.
        angle: f64,
        mode: ExtrusionMode,
    },
}
