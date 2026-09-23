//! A curve told to brush a trait, and the point where the two touch.
//!
//! That point is a point of the drawing like any other — it can be grabbed to
//! slide the curve along the trait, measured from, and snapped to. It is made
//! when the rule is laid rather than at the click, because where it goes is no
//! choice: it is the one place the two have in common.

use glam::DVec2;

use super::{CircleId, Element, PointId, SegmentId, Sketch};
use crate::constraints::Constraint;
use crate::ellipse::EllipseId;

impl Sketch {
    /// A circle told to brush a line, and the point where the two touch.
    pub fn add_tangency(&mut self, circle: CircleId, segment: SegmentId) {
        self.lay_tangency(
            Constraint::Tangent {
                circle,
                segment,
                at: None,
            },
            |sketch| sketch.foot_on_segment(sketch.circle(circle).center, segment),
        );
    }

    /// An ellipse told to brush a line, and the point where the two touch.
    pub fn add_ellipse_tangency(&mut self, ellipse: EllipseId, segment: SegmentId) {
        self.lay_tangency(
            Constraint::EllipseTangent {
                ellipse,
                segment,
                at: None,
            },
            |sketch| sketch.ellipse_touching(ellipse, segment),
        );
    }

    /// Whether the rule is a tangency, and laid with its contact point if so.
    pub(crate) fn laid_as_a_tangency(&mut self, constraint: Constraint) -> bool {
        match constraint {
            Constraint::Tangent {
                circle,
                segment,
                at: None,
            } => {
                self.add_tangency(circle, segment);
                true
            }
            Constraint::EllipseTangent {
                ellipse,
                segment,
                at: None,
            } => {
                self.add_ellipse_tangency(ellipse, segment);
                true
            }
            _ => false,
        }
    }

    /// Whether the rule is a tangency, and taken away with its contact point if
    /// so.
    ///
    /// A tangency is named by the two things it holds, whatever became of that
    /// point.
    pub(crate) fn erased_as_a_tangency(&mut self, constraint: Constraint) -> bool {
        if !matches!(
            constraint,
            Constraint::Tangent { .. } | Constraint::EllipseTangent { .. }
        ) {
            return false;
        }
        if let Some(rank) = self.tangency_index(constraint)
            && let Some(point) = contact_of(self.constraints.remove(rank))
        {
            self.erase(Element::Point(point));
        }
        true
    }

    fn lay_tangency(
        &mut self,
        plain: Constraint,
        touch: impl Fn(&Self) -> Option<DVec2>,
    ) -> Option<()> {
        if !self.holds_up(plain) || self.tangency_index(plain).is_some() {
            return None;
        }
        let at = touch(self).map(|place| self.add_point(place));
        self.constraints.push(with_contact(plain, at)?);
        Some(())
    }

    /// Where a tangency of the same two things stands among the rules, whatever
    /// contact point it was given.
    fn tangency_index(&self, constraint: Constraint) -> Option<usize> {
        self.constraints
            .iter()
            .position(|held| with_contact(*held, None) == with_contact(constraint, None))
    }

    /// Whether a tangency's contact point has slid past one end of the
    /// segment it is supposed to touch.
    ///
    /// The equations that hold a contact point only put it on the segment's
    /// infinite line, where the curve brushes that line — nothing stops that
    /// place from landing beyond either end once the curve is dragged far
    /// enough. Past that point the tangency is no longer physically real, so
    /// a solve that reaches it is treated as having failed rather than as
    /// having quietly moved the touch somewhere the segment does not go.
    pub(crate) fn has_a_flipped_tangent(&self) -> bool {
        self.constraints().iter().any(|constraint| {
            let (Some(segment), Some(at)) = (segment_of(*constraint), contact_of(*constraint))
            else {
                return false;
            };
            let Some(at) = self.live_point(Some(at)) else {
                return false;
            };
            let Some(line) = self.segments().get(segment.0).copied() else {
                return false;
            };
            let (a, b) = (self.point(line.start), self.point(line.end));
            let span = b - a;
            let length_squared = span.length_squared();
            if length_squared < 1e-12 {
                return false;
            }
            let t = (self.point(at) - a).dot(span) / length_squared;
            const SPAN_MARGIN: f64 = 1e-6;
            !(-SPAN_MARGIN..=1.0 + SPAN_MARGIN).contains(&t)
        })
    }
}

/// The same tangency, told to hold a contact point or none — which is what
/// makes two of them the same rule whatever point either was given.
fn with_contact(constraint: Constraint, at: Option<PointId>) -> Option<Constraint> {
    match constraint {
        Constraint::Tangent {
            circle, segment, ..
        } => Some(Constraint::Tangent {
            circle,
            segment,
            at,
        }),
        Constraint::EllipseTangent {
            ellipse, segment, ..
        } => Some(Constraint::EllipseTangent {
            ellipse,
            segment,
            at,
        }),
        _ => None,
    }
}

/// The point a tangency holds where the two touch, when it has one.
fn contact_of(constraint: Constraint) -> Option<PointId> {
    match constraint {
        Constraint::Tangent { at, .. } | Constraint::EllipseTangent { at, .. } => at,
        _ => None,
    }
}

/// The trait a tangency brushes.
fn segment_of(constraint: Constraint) -> Option<SegmentId> {
    match constraint {
        Constraint::Tangent { segment, .. } | Constraint::EllipseTangent { segment, .. } => {
            Some(segment)
        }
        _ => None,
    }
}
