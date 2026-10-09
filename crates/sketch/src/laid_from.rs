//! The order a pair was clicked in: kept, because the thing clicked first is
//! the one that stays while the rule or the value lands, and forgotten when two
//! rules are compared, because clicking them the other way round is still the
//! same rule.
//!
//! Every match here names each kind of rule and target, so that a kind added
//! to `constraints.rs` is not compared, nor landed, without being placed here.

use serde::{Deserialize, Serialize};

use crate::constraints::{Constraint, DimensionTarget};
use crate::sketch::{Element, Sketch};

/// Which of the two things a tangency, or a point laid on a trait, was laid
/// between was clicked first.
///
/// A trait and a curve have no order of their own the way two traits do, where
/// the pair itself says which came first, and nor do a point and a trait.
/// Without this the rule would have to guess, and one laid one way round
/// would land differently from the same one laid the other.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LaidFrom {
    /// Nothing was clicked: the drawing laid the rule for itself — a fillet,
    /// what a cut leaves of a circle, a point born on a trait — or the point
    /// was a free one, which comes onto the trait whichever was clicked first
    /// (#548). There is no order to read.
    #[default]
    Nowhere,
    /// The curve came first: the trait moves to graze it.
    Curve,
    /// The trait came first: the curve moves, and resizes, to touch it.
    Trait,
    /// The point came first: the trait it is laid on comes to it.
    Point,
}

impl Constraint {
    /// What the rule was laid from: the thing clicked first, which the rule
    /// never moves. `None` for a rule that has no order to it.
    ///
    /// A free point laid on a trait, and a point laid on a curve, have none on
    /// purpose: the point comes onto what holds it whichever was clicked
    /// first. A point that belongs to something keeps the order (#548).
    pub(crate) fn laid_from(self) -> Option<Element> {
        match self {
            Self::Perpendicular { first, .. }
            | Self::Parallel { first, .. }
            | Self::Equal { first, .. }
            | Self::Collinear { first, .. } => Some(Element::Segment(first)),
            Self::EqualRadius { first, .. } => Some(Element::Circle(first)),
            Self::EqualRadiusArc { first, .. } => Some(Element::Arc(first)),
            Self::Tangent {
                circle,
                segment,
                from,
                ..
            } => from.element(Element::Circle(circle), segment),
            Self::ArcTangent {
                arc, segment, from, ..
            } => from.element(Element::Arc(arc), segment),
            Self::EllipseTangent {
                ellipse,
                segment,
                from,
                ..
            } => from.element(Element::Ellipse(ellipse), segment),
            Self::OnSegment {
                point,
                segment,
                from,
            } => from.element(Element::Point(point), segment),
            Self::EqualRadiusArcCircle { .. }
            | Self::OnCircle { .. }
            | Self::OnArc { .. }
            | Self::OnEllipse { .. }
            | Self::OnAxis { .. }
            | Self::Midpoint { .. }
            | Self::AxisCollinear { .. }
            | Self::AxisParallel { .. }
            | Self::AxisPerpendicular { .. }
            | Self::Fixed { .. } => None,
        }
    }

    /// The same rule with its pair in a fixed order: the form two rules are
    /// compared in, so that the two ways of clicking one are the same rule.
    /// The drawing keeps the rule as it was clicked, since that order says
    /// which of the two stays.
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
            Self::EqualRadiusArc { first, second } if second.0 < first.0 => Self::EqualRadiusArc {
                first: second,
                second: first,
            },
            Self::Collinear { first, second } if second.0 < first.0 => Self::Collinear {
                first: second,
                second: first,
            },
            Self::Tangent {
                circle,
                segment,
                at,
                ..
            } => Self::Tangent {
                circle,
                segment,
                at,
                from: LaidFrom::Nowhere,
            },
            Self::ArcTangent {
                arc, segment, at, ..
            } => Self::ArcTangent {
                arc,
                segment,
                at,
                from: LaidFrom::Nowhere,
            },
            Self::EllipseTangent {
                ellipse,
                segment,
                at,
                ..
            } => Self::EllipseTangent {
                ellipse,
                segment,
                at,
                from: LaidFrom::Nowhere,
            },
            Self::OnSegment { point, segment, .. } => Self::OnSegment {
                point,
                segment,
                from: LaidFrom::Nowhere,
            },
            other @ (Self::Perpendicular { .. }
            | Self::Parallel { .. }
            | Self::Equal { .. }
            | Self::EqualRadius { .. }
            | Self::EqualRadiusArc { .. }
            | Self::Collinear { .. }
            | Self::EqualRadiusArcCircle { .. }
            | Self::OnCircle { .. }
            | Self::OnArc { .. }
            | Self::OnEllipse { .. }
            | Self::OnAxis { .. }
            | Self::Midpoint { .. }
            | Self::AxisCollinear { .. }
            | Self::AxisParallel { .. }
            | Self::AxisPerpendicular { .. }
            | Self::Fixed { .. }) => other,
        }
    }

    /// Whether two rules say the same thing, whichever way round each was
    /// clicked.
    pub(crate) fn is_the_same_as(self, other: Self) -> bool {
        self.normalised() == other.normalised()
    }
}

impl LaidFrom {
    /// The thing clicked first, of the trait and the other one — a curve for
    /// a tangency, a point for a point laid on the trait.
    fn element(self, other: Element, segment: crate::sketch::SegmentId) -> Option<Element> {
        match self {
            Self::Nowhere => None,
            Self::Curve | Self::Point => Some(other),
            Self::Trait => Some(Element::Segment(segment)),
        }
    }
}

impl DimensionTarget {
    /// What an angle was typed from: the trait clicked first, which the value
    /// never turns. Retyped later, the same one stays.
    pub(crate) fn laid_from(self) -> Option<Element> {
        match self {
            Self::Angle { first, .. } | Self::AngleBetween { first, .. } => {
                Some(Element::Segment(first))
            }
            Self::Length(_)
            | Self::Distance { .. }
            | Self::AxisAngle { .. }
            | Self::PointToSegment { .. }
            | Self::Projected { .. }
            | Self::Radius(_)
            | Self::Diameter(_)
            | Self::ArcRadius(_)
            | Self::ArcSweep(_) => None,
        }
    }

    /// The same target with its pair put in a fixed order: the form two
    /// targets are compared in.
    ///
    /// Clicking two segments one way round and the other way round means the
    /// same angle; compared as they were clicked they are two different
    /// targets, and the drawing ends up carrying the same dimension twice.
    pub fn normalised(self) -> Self {
        match self {
            Self::Distance { from, to } if to.0 < from.0 => Self::Distance { from: to, to: from },
            Self::Angle { first, second } if second.0 < first.0 => Self::Angle {
                first: second,
                second: first,
            },
            Self::AngleBetween {
                first,
                first_toward,
                second,
                second_toward,
            } if second.0 < first.0 => Self::AngleBetween {
                first: second,
                first_toward: second_toward,
                second: first,
                second_toward: first_toward,
            },
            Self::Projected { from, to, axis } if to.0 < from.0 => Self::Projected {
                from: to,
                to: from,
                axis,
            },
            other @ (Self::Length(_)
            | Self::Distance { .. }
            | Self::Angle { .. }
            | Self::AngleBetween { .. }
            | Self::AxisAngle { .. }
            | Self::PointToSegment { .. }
            | Self::Projected { .. }
            | Self::Radius(_)
            | Self::Diameter(_)
            | Self::ArcRadius(_)
            | Self::ArcSweep(_)) => other,
        }
    }

    /// Whether two targets measure the same thing, whichever way round each
    /// was clicked.
    pub(crate) fn is_the_same_as(self, other: Self) -> bool {
        self.normalised() == other.normalised()
    }
}

impl Sketch {
    /// Whether the drawing carries this rule, whichever way round it was
    /// clicked.
    pub fn carries(&self, constraint: Constraint) -> bool {
        self.constraints()
            .iter()
            .any(|held| held.is_the_same_as(constraint))
    }
}
