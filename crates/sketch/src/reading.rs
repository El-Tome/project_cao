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
    ///
    /// `None` when the target names something the drawing no longer has. Every
    /// arm checks: a measure outlives the geometry under it for exactly as long
    /// as it takes the next frame to notice, and a panic in that window would
    /// take the application with it.
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
            // A projected gap is the reach along one axis and nothing along
            // the other, so that is what it says. Pairing it with the offsets
            // of the whole run would put a span beside two numbers it is not
            // the hypotenuse of.
            DimensionTarget::Projected { from, to, axis } => {
                let span = self.projected_gap(from, to, axis)?;
                Reading::Gap {
                    span,
                    offsets: axis.direction().abs() * span,
                }
            }
            DimensionTarget::Radius(circle) | DimensionTarget::Diameter(circle) => Reading::Round {
                radius: self.circles().get(circle.0)?.radius,
            },
            DimensionTarget::ArcRadius(arc) => Reading::Round {
                radius: (arc.0 < self.arcs().len()).then(|| self.arc_radius(arc))?,
            },
            DimensionTarget::Angle { first, second } => {
                self.segments().get(first.0)?;
                self.segments().get(second.0)?;
                Reading::Opening {
                    degrees: self.angle_between(first, second)?,
                }
            }
            DimensionTarget::AngleBetween { .. } => Reading::Opening {
                degrees: self.opening(target)?,
            },
            DimensionTarget::AxisAngle { segment, axis } => {
                self.segments().get(segment.0)?;
                Reading::Opening {
                    degrees: self.angle_with_axis(segment, axis)?,
                }
            }
            DimensionTarget::ArcSweep(arc) => Reading::Opening {
                degrees: (arc.0 < self.arcs().len()).then(|| self.arc_sweep(arc).to_degrees())?,
            },
        })
    }

    /// The two places a straight run was read between, which is what the
    /// triangle showing it is drawn on. `None` for a circle or an angle, which
    /// are not runs and have no two axes to come apart into.
    pub fn run_of(&self, target: DimensionTarget) -> Option<(DVec2, DVec2)> {
        match target {
            DimensionTarget::Length(segment) => {
                self.segments().get(segment.0)?;
                Some(self.endpoints(segment))
            }
            DimensionTarget::Distance { from, to } => {
                self.points().get(from.0)?;
                self.points().get(to.0)?;
                Some((self.point(from), self.point(to)))
            }
            DimensionTarget::PointToSegment { point, segment } => {
                Some((self.point(point), self.foot_on_segment(point, segment)?))
            }
            DimensionTarget::Projected { from, to, .. } => {
                self.points().get(from.0)?;
                self.points().get(to.0)?;
                Some((self.point(from), self.point(to)))
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests;
