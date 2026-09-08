use glam::DVec2;
use serde::{Deserialize, Serialize};

use crate::sketch::{CircleId, Element, PointId, SegmentId};

/// One of the sketch's own axes, usable as the fixed reference of an angle.
///
/// Without it a drawing can always be spun about its anchor: pinning a point
/// takes away the two ways it can slide, never the way it can turn.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SketchAxis {
    /// The sketch's horizontal axis.
    U,
    /// The sketch's vertical axis.
    V,
}

impl SketchAxis {
    pub fn direction(self) -> DVec2 {
        match self {
            Self::U => DVec2::X,
            Self::V => DVec2::Y,
        }
    }
}

/// What a dimension measures.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
    /// Distance from a point to the line a segment lies on, taken square to
    /// that line — as if a perpendicular segment ran from the point down to it.
    PointToSegment { point: PointId, segment: SegmentId },
    /// The gap between two points along one of the sketch's axes: the width of
    /// a slanted trait rather than its length, or its height.
    Projected {
        from: PointId,
        to: PointId,
        axis: SketchAxis,
    },
    /// Radius of a circle, taken from its centre out to the rim.
    Radius(CircleId),
    /// Diameter of a circle, right across it. What a single click on a circle
    /// means: it is the size a hole is drilled to and the size a round bar is
    /// turned to, and a radius is what one asks for on purpose.
    Diameter(CircleId),
}

impl DimensionTarget {
    /// The same target with its pair put in a fixed order.
    ///
    /// Clicking two segments one way round and the other way round means the
    /// same angle; without this they are two different targets, and the drawing
    /// ends up carrying the same dimension twice.
    pub fn normalised(self) -> Self {
        match self {
            Self::Distance { from, to } if to.0 < from.0 => Self::Distance { from: to, to: from },
            Self::Angle { first, second } if second.0 < first.0 => Self::Angle {
                first: second,
                second: first,
            },
            Self::Projected { from, to, axis } if to.0 < from.0 => Self::Projected {
                from: to,
                to: from,
                axis,
            },
            other => other,
        }
    }
}

/// A rule with no number to it.
///
/// A dimension says how big something is; a constraint says how two things
/// stand to each other. Both take freedom away from the drawing and both are
/// counted the same way when working out what is still loose — they are kept
/// apart only because one carries a value the user types and the other does
/// not.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Constraint {
    /// Two traits meeting at a right angle, without saying which way up.
    Perpendicular {
        first: SegmentId,
        second: SegmentId,
    },
    Parallel {
        first: SegmentId,
        second: SegmentId,
    },
    /// Two traits of the same length.
    Equal {
        first: SegmentId,
        second: SegmentId,
    },
    /// Two circles of the same radius.
    EqualRadius {
        first: CircleId,
        second: CircleId,
    },
    /// A point held on the line a trait lies on, wherever the trait goes.
    OnSegment {
        point: PointId,
        segment: SegmentId,
    },
    /// Two traits lying on one and the same line.
    Collinear {
        first: SegmentId,
        second: SegmentId,
    },
    /// A circle brushing a line: the line grazes it and no more.
    Tangent {
        circle: CircleId,
        segment: SegmentId,
        /// The point where the two touch, kept as a real point of the drawing
        /// so it can be grabbed, dimensioned and snapped to. It is held both on
        /// the line and square under the centre, which is what keeps it at the
        /// contact instead of sliding along the line.
        #[serde(default)]
        at: Option<PointId>,
    },
    /// A point held on a circle's rim, wherever the circle goes and whatever
    /// size it takes. This is what makes the points clicked to draw a circle
    /// into handles: dragging one resizes the circle instead of leaving a
    /// stray point behind.
    OnCircle {
        point: PointId,
        circle: CircleId,
    },
    /// A point held halfway along a trait.
    Midpoint {
        point: PointId,
        segment: SegmentId,
    },
    /// A trait lying on one of the sketch's own axes.
    AxisCollinear {
        segment: SegmentId,
        axis: SketchAxis,
    },
    /// Something that stays where it is put. Only its place is held: a fixed
    /// circle keeps its centre, not its radius.
    Fixed {
        element: Element,
    },
}

impl Constraint {
    /// The same rule with its pair in a fixed order, so the two ways of
    /// clicking it are one rule.
    pub fn normalised(self) -> Self {
        match self {
            Self::Perpendicular { first, second } if second.0 < first.0 => Self::Perpendicular {
                first: second,
                second: first,
            },
            Self::Parallel { first, second } if second.0 < first.0 => Self::Parallel {
                first: second,
                second: first,
            },
            Self::Equal { first, second } if second.0 < first.0 => Self::Equal {
                first: second,
                second: first,
            },
            Self::EqualRadius { first, second } if second.0 < first.0 => Self::EqualRadius {
                first: second,
                second: first,
            },
            Self::Collinear { first, second } if second.0 < first.0 => Self::Collinear {
                first: second,
                second: first,
            },
            other => other,
        }
    }
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
    pub value: f64,
    pub driven: bool,
    /// Where the annotation sits, in sketch units, measured from what it
    /// annotates. `None` while it has never been placed, in which case it falls
    /// back to a distance in pixels.
    ///
    /// In sketch units rather than pixels because a dimension put somewhere is
    /// expected to stay there: a placement in pixels slides back over the
    /// drawing as soon as one zooms out.
    #[serde(default)]
    pub offset: Option<DVec2>,
}

impl Dimension {
    pub fn is_angle(&self) -> bool {
        matches!(
            self.target,
            DimensionTarget::Angle { .. } | DimensionTarget::AxisAngle { .. }
        )
    }

    /// Whether this dimension has been put somewhere by hand.
    pub fn is_placed(&self) -> bool {
        self.offset.is_some()
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
