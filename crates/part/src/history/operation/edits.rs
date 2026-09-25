//! Which sketch an operation edits, which is what files it under a step of
//! the design.

use super::Operation;

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
    /// sketch they stand on rather than one they change. A change to the
    /// variables answers `None` too: it belongs to the part, not to a step.
    ///
    /// The match has no wildcard arm, so an operation added later has to say
    /// where it belongs instead of quietly landing wherever the list ends.
    pub(crate) fn edits(&self) -> Option<usize> {
        match self {
            Self::CreateSketch { .. }
            | Self::Extrude { .. }
            | Self::Revolve { .. }
            | Self::Variable(_) => None,
            Self::Gesture(done) => done.iter().find_map(Self::edits),
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
            | Self::MoveSegment { sketch, .. }
            | Self::TurnShape { sketch, .. }
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
