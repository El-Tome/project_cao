//! Where a constraint's own mark belongs, so the drawing can say it without a
//! window: which of the things it holds carries the mark, the exception the
//! right angle makes, and which rule sits nearest a point on the drawing.

use glam::DVec2;

use crate::arc::ArcId;
use crate::constraints::Constraint;
use crate::sketch::{CircleId, Element, PointId, SegmentId, Sketch};

impl Sketch {
    /// Where a rule's marks are written: on each of the things it holds, so
    /// that pointing at one of them says what it is caught up in.
    ///
    /// A right angle is the exception: its mark belongs *in* the corner,
    /// which is the only place it reads as an angle rather than as a note
    /// about two traits.
    pub fn rule_marks(&self, constraint: Constraint) -> Vec<DVec2> {
        let middle = |segment: SegmentId| {
            (segment.0 < self.segments().len() && !self.is_erased_segment(segment)).then(|| {
                let (start, end) = self.endpoints(segment);
                (start + end) * 0.5
            })
        };
        let point = |id: PointId| (id.0 < self.points().len()).then(|| self.point(id));
        let circle = |id: CircleId| {
            (id.0 < self.circles().len()).then(|| {
                let round = self.circle(id);
                self.point(round.center) + DVec2::splat(round.radius * 0.7)
            })
        };
        let both = |first: SegmentId, second: SegmentId| {
            [middle(first), middle(second)]
                .into_iter()
                .flatten()
                .collect()
        };
        let arc = |id: ArcId| (id.0 < self.arcs().len()).then(|| self.arc_midpoint(id));

        match constraint {
            Constraint::Perpendicular { first, second } => match self.corner_of(first, second) {
                Some(at) => vec![at],
                None => both(first, second),
            },
            Constraint::Parallel { first, second }
            | Constraint::Equal { first, second }
            | Constraint::Collinear { first, second } => both(first, second),
            Constraint::EqualRadius { first, second } => [circle(first), circle(second)]
                .into_iter()
                .flatten()
                .collect(),
            Constraint::EqualRadiusArc { first, second } => {
                [arc(first), arc(second)].into_iter().flatten().collect()
            }
            Constraint::AxisCollinear { segment, .. } => middle(segment).into_iter().collect(),
            Constraint::OnSegment { point: held, .. }
            | Constraint::OnAxis { point: held, .. }
            | Constraint::Midpoint { point: held, .. } => point(held).into_iter().collect(),
            // Where the circle actually touches, not somewhere beside it: three
            // tangencies of one circle would otherwise all land on the same spot.
            Constraint::Tangent {
                circle: round,
                segment,
                at,
            } => at
                .and_then(point)
                .or_else(|| {
                    (round.0 < self.circles().len())
                        .then(|| self.foot_on_segment(self.circle(round).center, segment))
                        .flatten()
                })
                .into_iter()
                .collect(),
            Constraint::ArcTangent {
                arc: curve,
                segment,
                at,
            } => at
                .and_then(point)
                .or_else(|| {
                    (curve.0 < self.arcs().len())
                        .then(|| self.foot_on_segment(self.arc(curve).center, segment))
                        .flatten()
                })
                .into_iter()
                .collect(),
            Constraint::OnCircle { point: held, .. } | Constraint::OnArc { point: held, .. } => {
                point(held).into_iter().collect()
            }
            Constraint::Fixed { element } => match element {
                Element::Point(held) => point(held).into_iter().collect(),
                Element::Segment(held) => middle(held).into_iter().collect(),
                Element::Circle(held) => circle(held).into_iter().collect(),
                Element::Arc(held) => match held.0 < self.arcs().len() {
                    true => vec![self.arc_midpoint(held)],
                    false => Vec::new(),
                },
            },
        }
    }

    /// Just inside the corner two traits make, along the bisector.
    ///
    /// They have to actually meet: two traits held square without touching
    /// have no corner to write in, and the marks then go on the traits
    /// themselves.
    fn corner_of(&self, first: SegmentId, second: SegmentId) -> Option<DVec2> {
        let (pivot, a, b) = self.corner_points(first, second)?;
        let reach = (a.distance(pivot).min(b.distance(pivot))) * 0.25;
        let inward = ((a - pivot).normalize_or_zero() + (b - pivot).normalize_or_zero())
            .normalize_or(DVec2::X);
        Some(pivot + inward * reach)
    }

    /// The rule whose nearest mark sits under the cursor.
    pub fn nearest_rule(&self, cursor: DVec2, tolerance: f64) -> Option<Constraint> {
        self.constraints()
            .iter()
            .filter_map(|constraint| {
                let nearest = self
                    .rule_marks(*constraint)
                    .into_iter()
                    .map(|at| at.distance(cursor))
                    .min_by(f64::total_cmp)?;
                Some((*constraint, nearest))
            })
            .filter(|(_, distance)| *distance <= tolerance)
            .min_by(|a, b| a.1.total_cmp(&b.1))
            .map(|(constraint, _)| constraint)
    }
}

#[cfg(test)]
mod tests;
