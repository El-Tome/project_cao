//! What the measure tool reads off the drawing.
//!
//! A measure is not a dimension: it holds the drawing to nothing, it takes no
//! typed value, and it is gone the moment anything moves. What it does share
//! is the dimension tool's aim — `measure_pick` already knows what a click
//! means — so a measure names its subject with the same `DimensionTarget`,
//! and only the reading and the drawing of it are its own.
//!
//! That sharing is worth more than it looks. `Sketch::place` turns a target
//! into the shape an annotation is drawn as, whatever the target: the two
//! genuinely awkward angles — traits that lie apart, a trait against an axis —
//! are already solved there. A measure that named its subject any other way
//! would have to solve them a second time, and the second answer would drift
//! from the first.

use glam::DVec2;

use crate::constraints::DimensionTarget;
use crate::sketch::Sketch;

/// What a measure says, in the drawing's own units. Angles are degrees, which
/// need no scale.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Reading {
    /// A straight run: how long it is, and how far it reaches along each of
    /// the sketch's two axes.
    Gap { span: f64, offsets: DVec2 },
    /// A circle or an arc. The diameter is twice the radius, and the label
    /// says both — a measure costs nothing, so it need not choose.
    Round { radius: f64 },
    /// How wide two traits stand, in degrees.
    Opening { degrees: f64 },
}

impl Reading {
    /// The same reading in millimetres. An opening is already in degrees and
    /// is handed back untouched.
    pub fn scaled(self, millimeters_per_unit: f64) -> Self {
        match self {
            Self::Gap { span, offsets } => Self::Gap {
                span: span * millimeters_per_unit,
                offsets: offsets * millimeters_per_unit,
            },
            Self::Round { radius } => Self::Round {
                radius: radius * millimeters_per_unit,
            },
            Self::Opening { degrees } => Self::Opening { degrees },
        }
    }
}

impl Sketch {
    /// What the measure tool reads off a target, in the drawing's own units.
    ///
    /// The values are the ones the dimension tool would show for the same
    /// target — there is one geometry and it answers once — with the reach
    /// along each axis added, which is the half a dimension has no room for.
    pub fn read(&self, target: DimensionTarget) -> Option<Reading> {
        let gap = |from: DVec2, to: DVec2| Reading::Gap {
            span: from.distance(to),
            offsets: (to - from).abs(),
        };
        Some(match target {
            DimensionTarget::Length(segment) => {
                self.segments().get(segment.0)?;
                let (start, end) = self.endpoints(segment);
                gap(start, end)
            }
            DimensionTarget::Distance { from, to } => {
                self.points().get(from.0)?;
                self.points().get(to.0)?;
                gap(self.point(from), self.point(to))
            }
            DimensionTarget::PointToSegment { point, segment } => {
                let foot = self.foot_on_segment(point, segment)?;
                gap(self.point(point), foot)
            }
            DimensionTarget::Projected { from, to, axis } => Reading::Gap {
                span: self.projected_gap(from, to, axis)?,
                offsets: (self.point(to) - self.point(from)).abs(),
            },
            DimensionTarget::Radius(circle) | DimensionTarget::Diameter(circle) => Reading::Round {
                radius: self.circles().get(circle.0)?.radius,
            },
            DimensionTarget::ArcRadius(arc) => Reading::Round {
                radius: (arc.0 < self.arcs().len()).then(|| self.arc_radius(arc))?,
            },
            DimensionTarget::Angle { first, second } => Reading::Opening {
                degrees: self.angle_between(first, second)?,
            },
            DimensionTarget::AngleBetween { .. } => Reading::Opening {
                degrees: self.opening(target)?,
            },
            DimensionTarget::AxisAngle { segment, axis } => Reading::Opening {
                degrees: self.angle_with_axis(segment, axis)?,
            },
            DimensionTarget::ArcSweep(arc) => Reading::Opening {
                degrees: (arc.0 < self.arcs().len()).then(|| self.arc_sweep(arc).to_degrees())?,
            },
        })
    }
}

#[cfg(test)]
mod tests;
