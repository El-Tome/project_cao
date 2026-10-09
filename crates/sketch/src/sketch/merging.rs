//! Two points made one.
//!
//! Everything that named the point that goes names the point that stays: the
//! ends of a trait and of an arc, the centre of every curve, the stretch an
//! ellipse has drawn, the rules held on the point and the values measured from
//! it. An arc is a trait here, and a rule is never lost with its point (#539).
//!
//! The drop, « Coïncidence » and « Concentrique » all come through
//! [`Sketch::join_points`], so that what a gesture leaves never depends on
//! which of the three made it.

use glam::DVec2;

use super::{Element, LengthOutcome, PointId, Sketch};
use crate::constraints::{Constraint, Dimension, DimensionTarget};
use crate::erased::Erased;
use handing_over::handed_over;

mod handing_over;

impl Sketch {
    /// Makes two points one and settles the drawing into it, the point kept
    /// held where it is: the first one clicked stays. A fixed point outranks
    /// the clicks — when the point that goes was fixed and the one kept was
    /// not, the one kept comes to it, with the curves turning about it.
    ///
    /// Whether the drawing took it. A merge that leaves a rule or a value no
    /// way to hold is refused, and the drawing is left as it was.
    pub fn join_points(
        &mut self,
        kept: PointId,
        dropped: PointId,
        millimeters_per_unit: f64,
    ) -> bool {
        let (kept, dropped) = self.which_stays(kept, dropped);
        if kept == dropped {
            return true;
        }
        if !self.can_merge(kept, dropped) {
            return false;
        }
        let before = self.clone();
        let mut carried = Vec::new();
        if self.is_held(Element::Point(dropped)) && !self.is_held(Element::Point(kept)) {
            let step = self.point(dropped) - self.point(kept);
            carried = self.carry_curves_about(kept, step, [kept, dropped]);
            self.move_point(kept, self.point(dropped));
        }
        carried.extend(self.merge_carrying(kept, dropped));
        if self.settle_joined(kept, carried, millimeters_per_unit) == LengthOutcome::BestEffort {
            *self = before;
            return false;
        }
        true
    }

    /// Makes two points one, and settles nothing.
    ///
    /// Everything that referred to `dropped` now refers to `kept`. A curve
    /// turning about `dropped` is carried whole onto `kept`, keeping its size,
    /// its opening and the way it is turned. What the joining leaves with no
    /// length goes: a trait with the same point at both ends, an arc whose ends
    /// meet or that ends on its own centre, an arc of ellipse whose ends meet.
    pub fn merge_points(&mut self, kept: PointId, dropped: PointId) {
        self.merge_carrying(kept, dropped);
    }

    /// The merge, and the points it carried with the curves it moved.
    fn merge_carrying(&mut self, kept: PointId, dropped: PointId) -> Vec<PointId> {
        let (kept, dropped) = self.which_stays(kept, dropped);
        if kept == dropped || !self.has_point(kept) || !self.has_point(dropped) {
            return Vec::new();
        }
        let step = self.point(kept) - self.point(dropped);
        let carried = self.carry_curves_about(dropped, step, [kept, dropped]);
        self.hand_over_the_point(kept, dropped);
        self.take_away_what_collapsed();

        Erased::mark(&mut self.erased.points, dropped.0);
        let dimensions = std::mem::take(&mut self.dimensions);
        self.dimensions = dimensions
            .into_iter()
            .map(|dimension| Dimension {
                target: dimension.target.redirected(kept, dropped),
                ..dimension
            })
            .filter(|dimension| self.measures_live(dimension.target))
            .collect();
        carried
    }

    /// Settles the drawing with the point kept, and the points the merge
    /// carried with its curves, held where they now stand — as a drag of a
    /// centre holds its curve, so that a shape tied to the curve travels with
    /// it rather than bending. Around the point kept alone when that cannot
    /// land.
    fn settle_joined(
        &mut self,
        kept: PointId,
        mut carried: Vec<PointId>,
        millimeters_per_unit: f64,
    ) -> LengthOutcome {
        if !carried.is_empty() {
            let before = self.shapes_now();
            carried.push(kept);
            if self.settle_held(carried, Vec::new(), millimeters_per_unit) {
                return LengthOutcome::Exact;
            }
            self.give_back(before);
        }
        self.land(Some(Element::Point(kept)), None, millimeters_per_unit)
    }

    /// The origin never moves, so it is always the one kept.
    fn which_stays(&self, kept: PointId, dropped: PointId) -> (PointId, PointId) {
        match self.is_origin(dropped) {
            true => (dropped, kept),
            false => (kept, dropped),
        }
    }

    fn has_point(&self, point: PointId) -> bool {
        point.0 < self.points.len()
    }

    /// Whether every rule and value the two points carry could still hold
    /// once they are one. A rule the merge makes always true is simply let go
    /// of; this is about one it makes impossible, which no settling would
    /// report — the solver would meet it by shrinking a trait or a curve to
    /// nothing.
    fn can_merge(&self, kept: PointId, dropped: PointId) -> bool {
        if !self.has_point(kept) || !self.has_point(dropped) {
            return false;
        }
        let one = |point: PointId| if point == dropped { kept } else { point };
        let same = |a: PointId, b: PointId| one(a) == one(b);
        let radius_typed = |arc| {
            self.dimensions()
                .iter()
                .any(|value| value.target == DimensionTarget::ArcRadius(arc) && !value.driven)
        };
        let onto_its_centre = self.live_arcs().any(|(id, arc)| {
            (same(arc.center, arc.start) || same(arc.center, arc.end)) && radius_typed(id)
        });
        let impossible_rule = self.constraints.iter().any(|rule| match *rule {
            Constraint::Midpoint { point, segment } => {
                let side = self.segments[segment.0];
                !same(side.start, side.end) && (same(point, side.start) || same(point, side.end))
            }
            Constraint::OnCircle { point, circle } => same(point, self.circles[circle.0].center),
            Constraint::OnArc { point, arc } => {
                let curve = self.arcs[arc.0];
                same(point, curve.center)
                    && !same(curve.center, curve.start)
                    && !same(curve.center, curve.end)
            }
            Constraint::OnEllipse { point, ellipse } => {
                same(point, self.ellipses[ellipse.0].center)
            }
            Constraint::Tangent {
                circle,
                at: Some(at),
                ..
            } => same(at, self.circles[circle.0].center),
            Constraint::ArcTangent {
                arc, at: Some(at), ..
            } => same(at, self.arcs[arc.0].center),
            Constraint::EllipseTangent {
                ellipse,
                at: Some(at),
                ..
            } => same(at, self.ellipses[ellipse.0].center),
            _ => false,
        });
        !onto_its_centre && !impossible_rule
    }

    /// Moves every curve turning about `centre` by `step`, with the points it
    /// carries: its ends, its axes, and the points held on it. Curves that
    /// share a centre are locked together — moving the centre alone would
    /// stretch the curve and change its opening.
    ///
    /// Not a curve that stands on a fixed point, nor one that ends on one of
    /// the two points being joined: a fixed point is never the one that moves,
    /// and the settle reshapes that curve about it instead.
    pub(in crate::sketch) fn carry_curves_about(
        &mut self,
        centre: PointId,
        step: DVec2,
        joined: [PointId; 2],
    ) -> Vec<PointId> {
        if step == DVec2::ZERO {
            return Vec::new();
        }
        let arcs = self
            .live_arcs()
            .filter(|(_, arc)| arc.center == centre)
            .filter(|(_, arc)| !joined.contains(&arc.start) && !joined.contains(&arc.end))
            .map(|(id, _)| Element::Arc(id));
        let circles = self
            .live_circles()
            .filter(|(_, circle)| circle.center == centre)
            .map(|(id, _)| Element::Circle(id));
        let ellipses = self
            .live_ellipses()
            .filter(|(_, ellipse)| ellipse.center == centre)
            .map(|(id, _)| Element::Ellipse(id));
        let turning: Vec<Element> = arcs.chain(circles).chain(ellipses).collect();
        let mut carried: Vec<PointId> = Vec::new();
        for curve in turning {
            let points: Vec<PointId> = self
                .points_it_leans_on(curve)
                .into_iter()
                .chain(self.points_held_on(curve))
                .filter(|point| *point != centre && !joined.contains(point))
                .collect();
            if !points
                .iter()
                .any(|point| self.is_held(Element::Point(*point)))
            {
                carried.extend(points);
            }
        }
        carried.sort_by_key(|point| point.0);
        carried.dedup();
        for point in &carried {
            self.move_point(*point, self.point(*point) + step);
        }
        carried
    }

    /// The points a rule holds on a curve.
    fn points_held_on(&self, curve: Element) -> Vec<PointId> {
        self.constraints
            .iter()
            .filter_map(|rule| match (*rule, curve) {
                (Constraint::OnCircle { point, circle }, Element::Circle(on)) if circle == on => {
                    Some(point)
                }
                (Constraint::OnArc { point, arc }, Element::Arc(on)) if arc == on => Some(point),
                (Constraint::OnEllipse { point, ellipse }, Element::Ellipse(on))
                    if ellipse == on =>
                {
                    Some(point)
                }
                _ => None,
            })
            .collect()
    }

    /// Points everything that named `dropped` at `kept`, and lets go of a rule
    /// the joining has made always true, or has made twice.
    fn hand_over_the_point(&mut self, kept: PointId, dropped: PointId) {
        let one = |point: PointId| if point == dropped { kept } else { point };
        for segment in &mut self.segments {
            (segment.start, segment.end) = (one(segment.start), one(segment.end));
        }
        for circle in &mut self.circles {
            circle.center = one(circle.center);
        }
        for arc in &mut self.arcs {
            (arc.center, arc.start, arc.end) = (one(arc.center), one(arc.start), one(arc.end));
        }
        for ellipse in &mut self.ellipses {
            ellipse.center = one(ellipse.center);
            ellipse.drawn = ellipse.drawn.map(|(from, to)| (one(from), one(to)));
        }
        for rule in std::mem::take(&mut self.constraints) {
            let rule = handed_over(rule, one);
            if !self.holds_by_itself(rule) && !self.carries(rule) {
                self.constraints.push(rule);
            }
        }
    }

    /// A point held on a trait or an arc it now ends, or on an ellipse whose
    /// axis it now ends: nothing left to hold. The two ends of an arc of
    /// ellipse are not among them — the curve is drawn from its axes, and
    /// those ends are on it only because a rule holds them there.
    fn holds_by_itself(&self, rule: Constraint) -> bool {
        match rule {
            Constraint::OnSegment { point, segment, .. } => {
                let side = self.segments[segment.0];
                point == side.start || point == side.end
            }
            Constraint::OnArc { point, arc } => {
                let curve = self.arcs[arc.0];
                point == curve.start || point == curve.end
            }
            Constraint::OnEllipse { point, ellipse } => {
                self.ellipse_points(ellipse)[1..].contains(&point)
                    && point != self.ellipses[ellipse.0].center
            }
            _ => false,
        }
    }

    /// Takes away what the joining left with no length. An arc whose two ends
    /// met leaves its radius behind as the distance from its centre to where
    /// they met, so that the number typed is kept; a value on its opening has
    /// nothing left to measure, and goes with it.
    fn take_away_what_collapsed(&mut self) {
        let traits: Vec<Element> = self
            .live_segments()
            .filter(|(_, segment)| segment.start == segment.end)
            .map(|(id, _)| Element::Segment(id))
            .collect();
        for collapsed in traits {
            self.erase(collapsed);
        }
        let arcs: Vec<_> = self
            .live_arcs()
            .filter(|(_, arc)| {
                arc.start == arc.end || arc.center == arc.start || arc.center == arc.end
            })
            .collect();
        for (id, arc) in arcs {
            let radius = self
                .dimensions()
                .iter()
                .find(|value| value.target == DimensionTarget::ArcRadius(id))
                .cloned();
            if let Some(radius) = radius.filter(|_| arc.start == arc.end) {
                let onto = DimensionTarget::Distance {
                    from: arc.center,
                    to: arc.start,
                };
                self.carry_dimension(onto, &radius);
            }
            self.erase(Element::Arc(id));
        }
        let ellipses: Vec<Element> = self
            .live_ellipses()
            .filter(|(_, ellipse)| ellipse.drawn.is_some_and(|(from, to)| from == to))
            .map(|(id, _)| Element::Ellipse(id))
            .collect();
        for collapsed in ellipses {
            self.erase(collapsed);
        }
    }
}

#[cfg(test)]
mod tests;
