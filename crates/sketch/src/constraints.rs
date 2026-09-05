use glam::Vec2;
use serde::{Deserialize, Serialize};

use crate::sketch::{CircleId, PointId, SegmentId};

/// One of the sketch's own axes, usable as the fixed reference of an angle.
///
/// Without it a drawing can always be spun about its anchor: pinning a point
/// takes away the two ways it can slide, never the way it can turn.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SketchAxis {
    /// The sketch's horizontal axis.
    U,
    /// The sketch's vertical axis.
    V,
}

impl SketchAxis {
    pub fn direction(self) -> Vec2 {
        match self {
            Self::U => Vec2::X,
            Self::V => Vec2::Y,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::U => "axe horizontal",
            Self::V => "axe vertical",
        }
    }
}

/// What a dimension measures.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DimensionTarget {
    /// Length of a segment.
    Length(SegmentId),
    /// Straight distance between two points, which need not be joined by a
    /// segment. Measuring from the sketch origin is how a drawing gets pinned
    /// without having to sit exactly on it.
    Distance { from: PointId, to: PointId },
    /// Angle at the point two segments share.
    Angle { first: SegmentId, second: SegmentId },
    /// Angle between a segment and one of the sketch axes.
    AxisAngle {
        segment: SegmentId,
        axis: SketchAxis,
    },
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
    /// Where the annotation sits relative to where it would land on its own,
    /// in sketch units. Dragged by hand when the default place collides with
    /// the drawing.
    #[serde(default)]
    pub offset: Vec2,
}

impl Dimension {
    pub fn is_angle(&self) -> bool {
        matches!(
            self.target,
            DimensionTarget::Angle { .. } | DimensionTarget::AxisAngle { .. }
        )
    }

    /// How the value reads on screen.
    pub fn unit_suffix(&self) -> &'static str {
        if self.is_angle() { "°" } else { "mm" }
    }
}

/// How much freedom a drawing still has.
///
/// Worked out from the **rank** of the constraint system, not by counting
/// constraints: that is the only way to see that a triangle's third side
/// follows from its other sides and angles, and so adds nothing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Freedom {
    /// Coordinates that are still free to move.
    pub degrees_of_freedom: usize,
}

impl Freedom {
    pub fn fully_constrained(self) -> bool {
        self.degrees_of_freedom == 0
    }
}
