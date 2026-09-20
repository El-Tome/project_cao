//! What holds a point where it was laid down.
//!
//! A point that lands on a curve of the drawing is not merely put there: it is
//! held, and the rule holding it is what makes it follow when the curve moves.

use glam::DVec2;
use serde::{Deserialize, Serialize};

use crate::arc::ArcId;
use crate::constraints::{Constraint, SketchAxis};
use crate::edges::off_by;
use crate::sketch::{CircleId, Element, PointId, SegmentId, Sketch};
use crate::snap::onto_rim;

/// What a point can be held on: a curve of the drawing, or one of the two
/// axes its plane is counted from.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Support {
    Segment(SegmentId),
    Circle(CircleId),
    Arc(ArcId),
    Axis(SketchAxis),
}

impl Support {
    /// The rule that holds a point on it.
    pub fn holding(self, point: PointId) -> Constraint {
        match self {
            Self::Segment(segment) => Constraint::OnSegment { point, segment },
            Self::Circle(circle) => Constraint::OnCircle { point, circle },
            Self::Arc(arc) => Constraint::OnArc { point, arc },
            Self::Axis(axis) => Constraint::OnAxis { point, axis },
        }
    }

    /// The point a rule holds, when it is one of these.
    pub fn held_by(constraint: Constraint) -> Option<(PointId, Self)> {
        match constraint {
            Constraint::OnSegment { point, segment } => Some((point, Self::Segment(segment))),
            Constraint::OnCircle { point, circle } => Some((point, Self::Circle(circle))),
            Constraint::OnArc { point, arc } => Some((point, Self::Arc(arc))),
            Constraint::OnAxis { point, axis } => Some((point, Self::Axis(axis))),
            _ => None,
        }
    }
}

impl Sketch {
    /// Everything a point laid down at this place would land on, in the order
    /// the drawing holds them.
    ///
    /// Judged where the place actually falls rather than within a reach the
    /// cursor had: what lands on a curve is a click the magnets already pulled
    /// onto it, and a point a hair away from a trait was not put on it.
    pub fn supports_at(&self, place: DVec2) -> Vec<Support> {
        let near_enough = off_by(place);
        let on_traits = self
            .live_segments()
            .filter(|(id, _)| self.distance_to_segment(*id, place) <= near_enough)
            .map(|(id, _)| Support::Segment(id));
        let on_circles = self
            .live_circles()
            .filter(|(_, circle)| {
                (self.point(circle.center).distance(place) - circle.radius).abs() <= near_enough
            })
            .map(|(id, _)| Support::Circle(id));
        let on_arcs = self
            .live_arcs()
            .filter(|(id, _)| self.distance_to_arc(*id, place) <= near_enough)
            .map(|(id, _)| Support::Arc(id));
        let on_axes = [SketchAxis::U, SketchAxis::V]
            .into_iter()
            .filter(move |axis| across(*axis, place).abs() <= near_enough)
            .map(Support::Axis);

        on_traits
            .chain(on_circles)
            .chain(on_arcs)
            .chain(on_axes)
            .collect()
    }

    /// The same for a point the drawing already has: what that point is a
    /// part of does not hold it.
    ///
    /// An end of a trait pulled back along its own trait still lands on it,
    /// and holding it there would forbid that end from ever turning the trait
    /// again — while holding nothing, since a trait passes through its own
    /// ends whatever they do.
    pub fn supports_for(&self, point: PointId, place: DVec2) -> Vec<Support> {
        self.supports_at(place)
            .into_iter()
            .filter(|support| !self.drawn_from(*support, point))
            .collect()
    }

    /// Whether a curve is drawn from this very point.
    fn drawn_from(&self, support: Support, point: PointId) -> bool {
        let element = match support {
            Support::Segment(segment) => Element::Segment(segment),
            Support::Circle(circle) => Element::Circle(circle),
            Support::Arc(arc) => Element::Arc(arc),
            Support::Axis(_) => return false,
        };
        self.points_it_leans_on(element).contains(&point)
    }

    /// What holds a point where it is.
    pub fn holds_on(&self, point: PointId) -> Vec<Support> {
        self.constraints()
            .iter()
            .filter_map(|rule| Support::held_by(*rule))
            .filter(|(held, _)| *held == point)
            .map(|(_, support)| support)
            .collect()
    }

    /// Pulls a point off whatever holds it, and hands back the rules it
    /// dropped.
    pub fn let_go(&mut self, point: PointId) -> Vec<Constraint> {
        let dropped: Vec<Constraint> = self
            .constraints()
            .iter()
            .copied()
            .filter(|rule| Support::held_by(*rule).is_some_and(|(held, _)| held == point))
            .collect();
        for rule in &dropped {
            self.erase_constraint(*rule);
        }
        dropped
    }

    /// Where a point goes when it is pulled towards a place: along what holds
    /// it, and nowhere else.
    ///
    /// Held on two things at once it stands at their crossing, and a crossing
    /// is not somewhere a drag can take it: it moves when they do, and not
    /// otherwise.
    pub fn slide(&self, point: PointId, towards: DVec2) -> DVec2 {
        match self.holds_on(point).as_slice() {
            [] => towards,
            [only] => self.along(*only, towards).unwrap_or(towards),
            _ => self.point(point),
        }
    }

    /// Where a place lands when it is pulled straight onto what holds it.
    ///
    /// A trait holds the line it lies on rather than the stretch drawn, and an
    /// arc the whole circle it is a piece of: both are what the rule itself
    /// says, and a point slid past an end has not left what holds it.
    fn along(&self, support: Support, place: DVec2) -> Option<DVec2> {
        match support {
            Support::Segment(segment) => {
                let (start, end) = self.endpoints(segment);
                let span = end - start;
                let length = span.length();
                (length > 1e-9).then(|| {
                    let along = span / length;
                    start + along * (place - start).dot(along)
                })
            }
            Support::Circle(circle) => {
                let round = self.circle(circle);
                onto_rim(place, self.point(round.center), round.radius)
            }
            Support::Arc(arc) => {
                let curve = self.arc(arc);
                onto_rim(place, self.point(curve.center), self.arc_radius(arc))
            }
            Support::Axis(axis) => {
                let along = axis.direction();
                Some(along * place.dot(along))
            }
        }
    }

    /// Whether a rule holding a point still speaks of what the drawing has.
    pub(crate) fn hold_holds_up(&self, constraint: Constraint) -> bool {
        let drawn = |id: PointId| id.0 < self.points().len() && !self.is_erased_point(id);
        match constraint {
            Constraint::OnSegment { point, segment } => {
                drawn(point)
                    && segment.0 < self.segments().len()
                    && !self.is_erased_segment(segment)
            }
            Constraint::OnCircle { point, circle } => {
                drawn(point) && circle.0 < self.circles().len() && !self.is_erased_circle(circle)
            }
            Constraint::OnArc { point, arc } => {
                drawn(point) && arc.0 < self.arcs().len() && !self.is_erased_arc(arc)
            }
            Constraint::OnAxis { point, .. } => drawn(point),
            _ => false,
        }
    }
}

/// How far across an axis a place stands, signed.
fn across(axis: SketchAxis, place: DVec2) -> f64 {
    let along = axis.direction();
    place.dot(DVec2::new(-along.y, along.x))
}

#[cfg(test)]
mod tests;
