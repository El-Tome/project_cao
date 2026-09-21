//! What an operation says about raising matter from a drawing: which way it
//! goes, what it is swept around, and which of the faces it leaves a later
//! drawing was laid on.

use cao_sketch::{SegmentId, SketchAxis};
use glam::DVec3;
use serde::{Deserialize, Serialize};

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

/// The face of the part a drawing was laid on.
///
/// The face number comes out of the replay, which hands the same numbers to
/// the same part however its sizes change — so a drawing finds its face again
/// after the part has grown.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct FaceAnchor {
    pub face: usize,
    /// Which way was up on screen when the drawing was started. Kept rather
    /// than worked out again, so that a drawing reopened months later finds
    /// the axes it was drawn with.
    pub up: DVec3,
}
