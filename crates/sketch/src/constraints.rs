use glam::Vec2;
use serde::{Deserialize, Serialize};

use crate::sketch::{CircleId, SegmentId};

/// What a dimension measures.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DimensionTarget {
    /// Length of a segment.
    Length(SegmentId),
    /// Angle at the point two segments share.
    Angle { first: SegmentId, second: SegmentId },
    /// Radius of a circle.
    Radius(CircleId),
}

/// A value the user has fixed.
///
/// A *driving* dimension moves the geometry. A *driven* one only reports what
/// the geometry already measures: it is what you get when the shape is already
/// fully determined and a further constraint would be redundant.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Dimension {
    pub target: DimensionTarget,
    /// Millimetres for a length or a radius, degrees for an angle.
    pub value: f32,
    pub driven: bool,
}

impl Dimension {
    pub fn is_angle(&self) -> bool {
        matches!(self.target, DimensionTarget::Angle { .. })
    }

    /// How the value reads on screen.
    pub fn unit_suffix(&self) -> &'static str {
        if self.is_angle() { "°" } else { "mm" }
    }
}

/// How much freedom is left in one connected piece of a drawing.
///
/// This is a **count**, not a rank analysis: it compares the number of
/// coordinates against the number of values fixed. It cannot tell that two
/// constraints say the same thing in different words, so a drawing it calls
/// fully constrained may still be under-determined in an unusual arrangement.
/// It is enough to colour the drawing and to warn about the obvious redundancy,
/// and it is deliberately not presented as more than that.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Freedom {
    pub degrees_of_freedom: i32,
}

impl Freedom {
    pub fn fully_constrained(self) -> bool {
        self.degrees_of_freedom <= 0
    }
}

/// Points sitting on the sketch origin are pinned there, which is how a drawing
/// gets rid of its last freedom to slide around.
pub const ANCHOR_TOLERANCE: f32 = 1e-4;

pub fn is_anchor(position: Vec2) -> bool {
    position.length() <= ANCHOR_TOLERANCE
}

/// Union-find over point indices, used to group a drawing into the pieces that
/// move independently of each other.
pub(crate) struct Components {
    parent: Vec<usize>,
}

impl Components {
    pub fn new(count: usize) -> Self {
        Self {
            parent: (0..count).collect(),
        }
    }

    pub fn find(&mut self, mut index: usize) -> usize {
        while self.parent[index] != index {
            self.parent[index] = self.parent[self.parent[index]];
            index = self.parent[index];
        }
        index
    }

    pub fn union(&mut self, a: usize, b: usize) {
        let (a, b) = (self.find(a), self.find(b));
        if a != b {
            self.parent[b] = a;
        }
    }
}
