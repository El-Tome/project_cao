//! An angle of exactly 0°, 90° or 180° says how two things stand to each
//! other rather than how far apart they turn: it is the parallel or the
//! perpendicular rule, written as a number.

use crate::constraints::{Constraint, DimensionTarget, SketchAxis};
use crate::sketch::{Element, PointId, SegmentId, Sketch};

/// The two angles a rule says without a number.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExactAngle {
    /// 0° or 180°: the parallel rule has no way round, its error is a sine.
    Straight,
    /// 90°.
    Square,
}

impl ExactAngle {
    /// The rule a value typed into a dimension means: only 0, 90 and 180
    /// exactly. A hair off them is an angle someone meant.
    pub fn of(value: f64) -> Option<Self> {
        if value == 0.0 || value == 180.0 {
            Some(Self::Straight)
        } else if value == 90.0 {
            Some(Self::Square)
        } else {
            None
        }
    }

    /// The rule an angle typed while drawing means. The trait goes the way it
    /// was typed, so -90° and 270° hold it along the same line as 90°.
    pub fn drawn(value: f64) -> Option<Self> {
        Self::of(value.rem_euclid(180.0))
    }

    /// The rule between two traits, in the order the angle held them.
    pub fn between(self, first: SegmentId, second: SegmentId) -> Constraint {
        match self {
            Self::Straight => Constraint::Parallel { first, second },
            Self::Square => Constraint::Perpendicular { first, second },
        }
    }

    /// The rule between a trait and one of the sketch's axes.
    pub fn to_axis(self, segment: SegmentId, axis: SketchAxis) -> Constraint {
        match self {
            Self::Straight => Constraint::AxisParallel { segment, axis },
            Self::Square => Constraint::AxisPerpendicular { segment, axis },
        }
    }
}

impl DimensionTarget {
    /// The rule this angle stands for at `value`, or nothing: an angle to an
    /// axis gives the rule to that axis, an angle between two traits the rule
    /// between them, and every other dimension none.
    pub fn rule_at(self, value: f64) -> Option<Constraint> {
        let exact = ExactAngle::of(value)?;
        match self {
            Self::AxisAngle { segment, axis, .. } => Some(exact.to_axis(segment, axis)),
            Self::Angle { first, second, .. } | Self::AngleBetween { first, second, .. } => {
                Some(exact.between(first, second))
            }
            _ => None,
        }
    }
}

/// What takes the place of an angle typed at exactly 0°, 90° or 180°.
#[derive(Clone, Debug, PartialEq)]
pub struct InPlace {
    pub rule: Constraint,
    /// What goes with the dimension: the arm the angle was read against, when
    /// it was one, since the rule is then to the axis the arm ran along.
    pub erased: Vec<Element>,
}

impl Sketch {
    /// The rule a value typed into the dimension on `target` lays instead of
    /// the dimension, or nothing when the value is an angle like any other.
    ///
    /// The angle of a trait drawn at a typed angle is read against an arm
    /// along the horizontal. A rule between the arm and the trait would lay
    /// the trait along a line left on screen for nothing: the rule is to the
    /// horizontal axis instead, and the arm goes.
    pub fn rule_in_place_of(&self, target: DimensionTarget, value: f64) -> Option<InPlace> {
        let rule = target.rule_at(value)?;
        let (first, second) = match rule {
            Constraint::Parallel { first, second }
            | Constraint::Perpendicular { first, second } => (first, second),
            _ => {
                return Some(InPlace {
                    rule,
                    erased: Vec::new(),
                });
            }
        };
        let exact = ExactAngle::of(value)?;
        for (arm, drawn) in [(first, second), (second, first)] {
            if let Some(far) = self.arm_end(arm, target) {
                return Some(InPlace {
                    rule: exact.to_axis(drawn, SketchAxis::U),
                    erased: vec![Element::Segment(arm), Element::Point(far)],
                });
            }
        }
        Some(InPlace {
            rule,
            erased: Vec::new(),
        })
    }

    /// The far end of `segment` when it is the arm a typed angle was laid
    /// against: construction, held along the horizontal, with nothing on it
    /// but that rule and the angle `target`, and its far end shared with
    /// nothing. A construction line someone leans more on is theirs.
    fn arm_end(&self, segment: SegmentId, target: DimensionTarget) -> Option<PointId> {
        let laid = self.segments().get(segment.0)?;
        let held = Constraint::AxisParallel {
            segment,
            axis: SketchAxis::U,
        };
        if !laid.construction
            || self.is_erased_segment(segment)
            || !self.constraints().contains(&held)
            || self.dimension_of(target).is_none()
        {
            return None;
        }
        let far = laid.end;
        let shared = self
            .live_segments()
            .any(|(other, it)| other != segment && (it.start == far || it.end == far));
        let mut without = self.clone();
        without.erase(Element::Segment(segment));
        without.erase(Element::Point(far));
        let rules_lost = self.constraints().len() - without.constraints().len();
        let values_lost = self.dimensions().len() - without.dimensions().len();
        let curves = |sketch: &Self| {
            sketch.live_circles().count()
                + sketch.live_arcs().count()
                + sketch.live_ellipses().count()
        };
        (!shared && rules_lost == 1 && values_lost == 1 && curves(self) == curves(&without))
            .then_some(far)
    }
}

#[cfg(test)]
mod tests;
