//! The angle between a trait and one of the sketch's own axes.
//!
//! A line running the way the axis runs, through the middle of the trait, cuts
//! the plane into four quarters with the trait: two of them acute, two obtuse.
//! Which way each arm runs out from that middle — along the axis, along the
//! trait — says which quarter a dimension measures, and where it is put down
//! is what chooses it. The way the trait was drawn says nothing.

use glam::DVec2;
use serde::{Deserialize, Serialize};

use crate::angle_between::RUN_THE_SAME_WAY;
use crate::constraints::{DimensionTarget, SketchAxis, Toward};
use crate::sketch::{SegmentId, Sketch};

/// Which way along a sketch axis one arm of an angle runs.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AxisToward {
    /// The way the axis runs: right along the horizontal, up the vertical.
    Positive,
    /// Back the other way.
    Negative,
}

impl AxisToward {
    fn of(side: f64) -> Self {
        match side >= 0.0 {
            true => Self::Positive,
            false => Self::Negative,
        }
    }
}

impl Sketch {
    /// A new angle between a trait and an axis: the acute one, until where it
    /// is put down turns it to face another quarter. Or the one the drawing
    /// already carries between them.
    pub fn angle_to_axis(&self, segment: SegmentId, axis: SketchAxis) -> DimensionTarget {
        if let Some(laid) = self.angle_already_to_axis(segment, axis) {
            return laid;
        }
        let axis_toward = match self.segments().get(segment.0) {
            Some(_) => AxisToward::of(self.arm(segment, Toward::End).dot(axis.direction())),
            None => AxisToward::Positive,
        };
        DimensionTarget::AxisAngle {
            segment,
            segment_toward: Toward::End,
            axis,
            axis_toward,
        }
    }

    /// The angle a dimension already reads between this trait and this axis,
    /// whichever quarter it measures.
    ///
    /// A trait and an axis carry one angle between them at most: the other
    /// quarters are that one or what it leaves of a half turn, and a second
    /// one laid beside it would sooner or later say something the first does
    /// not.
    fn angle_already_to_axis(
        &self,
        segment: SegmentId,
        axis: SketchAxis,
    ) -> Option<DimensionTarget> {
        self.dimensions()
            .iter()
            .map(|dimension| dimension.target)
            .find(|target| {
                matches!(*target, DimensionTarget::AxisAngle { segment: one, axis: other, .. }
                    if one == segment && other == axis)
            })
    }

    /// The same angle, turned to face `placed`: the quarter it is put down in,
    /// about the trait's middle.
    ///
    /// A place lies between two arms when it stands on the trait arm's side of
    /// the line along the axis, and on the axis arm's side of the trait. A
    /// trait running the way the axis runs cuts no quarters, and comes back as
    /// it was.
    pub(crate) fn facing_axis(&self, target: DimensionTarget, placed: DVec2) -> DimensionTarget {
        let DimensionTarget::AxisAngle { segment, axis, .. } = target else {
            return target;
        };
        if let Some(laid) = self.angle_already_to_axis(segment, axis) {
            return laid;
        }
        if self.segments().get(segment.0).is_none() {
            return target;
        }
        let (start, end) = self.endpoints(segment);
        let (along, across) = ((end - start).normalize_or_zero(), axis.direction());
        let turn = across.perp_dot(along);
        if turn.abs() < RUN_THE_SAME_WAY {
            return target;
        }
        let side = placed - (start + end) * 0.5;
        DimensionTarget::AxisAngle {
            segment,
            segment_toward: match across.perp_dot(side) * turn >= 0.0 {
                true => Toward::End,
                false => Toward::Start,
            },
            axis,
            axis_toward: AxisToward::of(along.perp_dot(side) * along.perp_dot(across)),
        }
    }

    /// Where an angle against an axis is read, and its two arms out from
    /// there: the trait's middle, the way along the axis, and the way along
    /// the trait, as long as the trait.
    pub(crate) fn axis_arms(&self, target: DimensionTarget) -> Option<(DVec2, DVec2, DVec2)> {
        let DimensionTarget::AxisAngle {
            segment,
            segment_toward,
            axis,
            axis_toward,
        } = target
        else {
            return None;
        };
        self.segments().get(segment.0)?;
        let (start, end) = self.endpoints(segment);
        let along_the_axis = match axis_toward {
            AxisToward::Positive => axis.direction(),
            AxisToward::Negative => -axis.direction(),
        };
        Some((
            (start + end) * 0.5,
            along_the_axis,
            self.arm(segment, segment_toward),
        ))
    }
}
