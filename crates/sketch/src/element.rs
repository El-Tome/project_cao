use serde::{Deserialize, Serialize};

use crate::sketch::{CircleId, PointId, SegmentId};

/// One thing a sketch is made of, for deleting it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Element {
    Point(PointId),
    Segment(SegmentId),
    Circle(CircleId),
}
