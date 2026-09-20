use std::cell::RefCell;

use glam::DVec2;
use serde::{Deserialize, Serialize};

use crate::annotation::AnnotationMetrics;
use crate::arc::Arc;
use crate::constraints::{Constraint, Dimension, DimensionTarget, SketchAxis};
use crate::erased::Erased;
use crate::independence::is_dependent;
use crate::length::LengthOutcome;
use crate::plane::WorkPlane;
use crate::solver::SolveOutcome;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PointId(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SegmentId(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct CircleId(pub usize);

/// A circle, kept as a centre point shared with the rest of the drawing plus a
/// radius, so that moving the centre moves the circle with it.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Circle {
    pub center: PointId,
    pub radius: f64,
    #[serde(default)]
    pub construction: bool,
}

/// A straight line between two points. Points are shared: chaining a polyline
/// reuses the previous end, which is what makes a dimension able to drag the
/// rest of the chain along.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Segment {
    pub start: PointId,
    pub end: PointId,
    #[serde(default)]
    pub construction: bool,
}

/// A 2D sketch on a plane. Everything is stored in the plane's own coordinates
/// and in world units; millimetres only appear on dimensions, because the
/// document scale can be redefined by the first one.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sketch {
    pub plane: WorkPlane,
    points: Vec<DVec2>,
    segments: Vec<Segment>,
    #[serde(default)]
    circles: Vec<Circle>,
    #[serde(default)]
    pub(crate) arcs: Vec<Arc>,
    dimensions: Vec<Dimension>,
    /// The rules that carry no value: perpendicular, parallel, equal…
    #[serde(default)]
    constraints: Vec<Constraint>,
    /// What has been deleted, by rank.
    ///
    /// Deleted geometry is marked rather than taken out of the list: a segment
    /// removed from the middle would shift the rank of every later one, and
    /// every dimension already recorded against those ranks would silently
    /// start pointing at a different piece of the drawing.
    #[serde(default)]
    pub(crate) erased: Erased,
    /// The points the user is holding under the cursor, while a drag lasts.
    ///
    /// They do not give: the drawing settles *around* them rather than pulling
    /// them back, which is what makes a shape follow the mouse instead of
    /// squirming away from it. Nothing to save — they live only as long as the gesture.
    #[serde(skip)]
    held: Vec<PointId>,
    /// The last reading of which points can no longer move, against a print of
    /// the drawing it was read from. Worked out from everything else, so it is
    /// never saved and never read back.
    #[serde(skip)]
    settled: RefCell<Option<(u64, Vec<bool>)>>,
}

mod keeping;

pub use crate::element::Element;

impl Sketch {
    /// Every sketch owns a point at its origin from the moment it is created.
    ///
    /// It is not drawn like the others and cannot be moved: it is there to be
    /// snapped onto and measured from, so that pinning a drawing never means
    /// first remembering to place a point by hand.
    pub const ORIGIN: PointId = PointId(0);

    pub fn new(plane: WorkPlane) -> Self {
        Self {
            plane,
            points: vec![DVec2::ZERO],
            segments: Vec::new(),
            circles: Vec::new(),
            arcs: Vec::new(),
            dimensions: Vec::new(),
            constraints: Vec::new(),
            erased: Erased::default(),
            held: Vec::new(),
            settled: RefCell::default(),
        }
    }

    /// The verdict remembered for this print, if it is the one still standing.
    ///
    /// The cell never leaves this method: a borrow held across a reading would
    /// meet the one taken to record the answer, and that is a panic.
    pub(crate) fn settled_read_from(&self, print: u64) -> Option<Vec<bool>> {
        match self.settled.borrow().as_ref() {
            Some((was, verdict)) if *was == print => Some(verdict.clone()),
            _ => None,
        }
    }

    pub(crate) fn remember_settled(&self, print: u64, verdict: Vec<bool>) {
        *self.settled.borrow_mut() = Some((print, verdict));
    }

    #[cfg(test)]
    pub(crate) fn forget_what_is_settled(&self) {
        *self.settled.borrow_mut() = None;
    }

    pub fn is_erased_point(&self, point: PointId) -> bool {
        Erased::holds(&self.erased.points, point.0)
    }

    pub fn is_erased_segment(&self, segment: SegmentId) -> bool {
        Erased::holds(&self.erased.segments, segment.0)
    }

    pub fn is_erased_circle(&self, circle: CircleId) -> bool {
        Erased::holds(&self.erased.circles, circle.0)
    }

    /// The segments still drawn, with their rank.
    pub fn live_points(&self) -> impl Iterator<Item = (PointId, DVec2)> + '_ {
        self.points
            .iter()
            .enumerate()
            .map(|(rank, point)| (PointId(rank), *point))
            .filter(|(id, _)| !self.is_erased_point(*id))
    }

    pub fn live_segments(&self) -> impl Iterator<Item = (SegmentId, Segment)> + '_ {
        self.segments
            .iter()
            .enumerate()
            .map(|(rank, segment)| (SegmentId(rank), *segment))
            .filter(|(id, _)| !self.is_erased_segment(*id))
    }

    pub fn live_circles(&self) -> impl Iterator<Item = (CircleId, Circle)> + '_ {
        self.circles
            .iter()
            .enumerate()
            .map(|(rank, circle)| (CircleId(rank), *circle))
            .filter(|(id, _)| !self.is_erased_circle(*id))
    }

    /// Deletes an element, and everything that leaned on it.
    ///
    /// A segment without its point is not geometry, and a dimension measuring
    /// something that is gone would report on nothing — so they go too, rather
    /// than being left behind as references into a hole.
    pub fn erase(&mut self, element: Element) {
        match element {
            Element::Point(point) => {
                if self.is_origin(point) {
                    return;
                }
                Erased::mark(&mut self.erased.points, point.0);
                let touched: Vec<Element> = self
                    .live_segments()
                    .filter(|(_, segment)| segment.start == point || segment.end == point)
                    .map(|(id, _)| Element::Segment(id))
                    .chain(
                        self.live_circles()
                            .filter(|(_, circle)| circle.center == point)
                            .map(|(id, _)| Element::Circle(id)),
                    )
                    .chain(self.arcs_leaning_on(point))
                    .collect();
                for element in touched {
                    self.erase(element);
                }
            }
            Element::Segment(segment) => Erased::mark(&mut self.erased.segments, segment.0),
            Element::Circle(circle) => Erased::mark(&mut self.erased.circles, circle.0),
            Element::Arc(arc) => Erased::mark(&mut self.erased.arcs, arc.0),
        }
        let dimensions = std::mem::take(&mut self.dimensions);
        self.dimensions = dimensions
            .into_iter()
            .filter(|dimension| self.measures_live(dimension.target))
            .collect();
        let (kept, dropped): (Vec<Constraint>, Vec<Constraint>) =
            std::mem::take(&mut self.constraints)
                .into_iter()
                .partition(|constraint| self.holds_up(*constraint));
        self.constraints = kept;
        // A tangency taking its contact point with it: left behind, the point
        // would sit in mid-air with nothing holding it.
        for constraint in dropped {
            if let Constraint::Tangent {
                at: Some(point), ..
            } = constraint
            {
                Erased::mark(&mut self.erased.points, point.0);
            }
        }
    }

    /// Whether everything a dimension refers to is still drawn.
    fn measures_live(&self, target: DimensionTarget) -> bool {
        match target {
            DimensionTarget::Length(segment) => !self.is_erased_segment(segment),
            DimensionTarget::Distance { from, to } => {
                !self.is_erased_point(from) && !self.is_erased_point(to)
            }
            DimensionTarget::Angle { first, second } => {
                !self.is_erased_segment(first) && !self.is_erased_segment(second)
            }
            DimensionTarget::AxisAngle { segment, .. } => !self.is_erased_segment(segment),
            DimensionTarget::PointToSegment { point, segment } => {
                !self.is_erased_point(point) && !self.is_erased_segment(segment)
            }
            DimensionTarget::Projected { from, to, .. } => {
                !self.is_erased_point(from) && !self.is_erased_point(to)
            }
            DimensionTarget::Radius(circle) | DimensionTarget::Diameter(circle) => {
                !self.is_erased_circle(circle)
            }
            DimensionTarget::ArcRadius(arc) | DimensionTarget::ArcSweep(arc) => {
                !self.is_erased_arc(arc)
            }
        }
    }

    pub fn constraints(&self) -> &[Constraint] {
        &self.constraints
    }

    /// Adds a rule, unless the drawing already carries it.
    pub fn add_constraint(&mut self, constraint: Constraint) {
        // A tangency brings its contact point with it, so it is laid down by
        // the method that knows how to make one.
        if let Constraint::Tangent {
            circle,
            segment,
            at: None,
        } = constraint
        {
            self.add_tangency(circle, segment);
            return;
        }
        let constraint = constraint.normalised();
        if !self.constraints.contains(&constraint) && self.holds_up(constraint) {
            self.constraints.push(constraint);
        }
    }

    /// A circle told to brush a line, and the point where the two touch.
    ///
    /// That point is a point of the drawing like any other — it can be grabbed
    /// to slide the circle along the line, measured from, and snapped to. It is
    /// made here rather than at the click because where it goes is not a
    /// choice: it is the foot of the centre on the line.
    pub fn add_tangency(&mut self, circle: CircleId, segment: SegmentId) {
        let plain = Constraint::Tangent {
            circle,
            segment,
            at: None,
        };
        if !self.holds_up(plain) || self.tangency_index(circle, segment).is_some() {
            return;
        }
        let at = self
            .foot_on_segment(self.circle(circle).center, segment)
            .map(|place| self.add_point(place));
        self.constraints.push(Constraint::Tangent {
            circle,
            segment,
            at,
        });
    }

    fn tangency_index(&self, circle: CircleId, segment: SegmentId) -> Option<usize> {
        self.constraints.iter().position(|held| {
            matches!(
                held,
                Constraint::Tangent { circle: round, segment: line, .. }
                    if *round == circle && *line == segment
            )
        })
    }

    pub fn erase_constraint(&mut self, constraint: Constraint) {
        // A tangency is named by the two things it holds, whatever became of
        // its contact point, and that point goes with it.
        if let Constraint::Tangent {
            circle, segment, ..
        } = constraint
        {
            let Some(rank) = self.tangency_index(circle, segment) else {
                return;
            };
            if let Constraint::Tangent {
                at: Some(point), ..
            } = self.constraints.remove(rank)
            {
                self.erase(Element::Point(point));
            }
            return;
        }
        let constraint = constraint.normalised();
        self.constraints.retain(|held| *held != constraint);
    }

    /// Whether a piece of the drawing is held in place by a rule.
    ///
    /// A trait held still holds its two ends; a circle held still holds its
    /// centre, an arc all three of its own — so each reads as fixed although
    /// only one of them was named.
    pub fn is_held(&self, element: Element) -> bool {
        let holds_point =
            |point: PointId, held: Element| self.points_it_leans_on(held).contains(&point);
        self.constraints.iter().any(|constraint| {
            let Constraint::Fixed { element: held } = constraint else {
                return false;
            };
            match element {
                Element::Point(point) => holds_point(point, *held),
                other => *held == other,
            }
        })
    }

    /// Whether everything a rule speaks of is still drawn.
    fn holds_up(&self, constraint: Constraint) -> bool {
        let segment = |id: SegmentId| id.0 < self.segments.len() && !self.is_erased_segment(id);
        let circle = |id: CircleId| id.0 < self.circles.len() && !self.is_erased_circle(id);
        let point = |id: PointId| id.0 < self.points.len() && !self.is_erased_point(id);
        match constraint {
            Constraint::Perpendicular { first, second }
            | Constraint::Parallel { first, second }
            | Constraint::Equal { first, second }
            | Constraint::Collinear { first, second } => {
                first != second && segment(first) && segment(second)
            }
            Constraint::EqualRadius { first, second } => {
                first != second && circle(first) && circle(second)
            }
            Constraint::EqualRadiusArc { .. } | Constraint::ArcTangent { .. } => {
                self.arc_rule_holds_up(constraint)
            }
            Constraint::OnSegment { .. }
            | Constraint::OnCircle { .. }
            | Constraint::OnArc { .. }
            | Constraint::OnAxis { .. } => self.hold_holds_up(constraint),
            Constraint::Midpoint {
                point: held,
                segment: on,
            } => point(held) && segment(on),
            Constraint::Tangent {
                circle: round,
                segment: line,
                ..
            } => circle(round) && segment(line),
            Constraint::AxisCollinear { segment: on, .. } => segment(on),
            Constraint::Fixed { element } => match element {
                Element::Point(held) => point(held),
                Element::Segment(held) => segment(held),
                Element::Circle(held) => circle(held),
                Element::Arc(held) => held.0 < self.arcs.len() && !self.is_erased_arc(held),
            },
        }
    }

    pub fn erase_dimension(&mut self, target: DimensionTarget) {
        self.dimensions
            .retain(|dimension| dimension.target != target);
    }

    pub fn is_origin(&self, point: PointId) -> bool {
        point == Self::ORIGIN
    }

    /// A point neither coordinate of which can move, whatever the drawing says:
    /// the origin, and anything erased.
    pub(crate) fn out_of_play(&self, point: PointId) -> bool {
        self.is_origin(point) || self.is_erased_point(point)
    }

    /// Whether the user is holding this point under the cursor right now.
    pub(crate) fn is_held_still(&self, point: PointId) -> bool {
        self.held.contains(&point)
    }

    /// Points the user drew, as opposed to the origin the sketch was born with.
    pub fn drawn_points(&self) -> impl Iterator<Item = (PointId, DVec2)> + '_ {
        self.points
            .iter()
            .enumerate()
            .skip(1)
            .map(|(index, point)| (PointId(index), *point))
            .filter(|(id, _)| !self.is_erased_point(*id))
    }

    pub fn points(&self) -> &[DVec2] {
        &self.points
    }

    pub fn segments(&self) -> &[Segment] {
        &self.segments
    }

    pub fn circles(&self) -> &[Circle] {
        &self.circles
    }

    pub fn dimensions(&self) -> &[Dimension] {
        &self.dimensions
    }

    pub fn circle(&self, id: CircleId) -> Circle {
        self.circles[id.0]
    }

    pub fn add_circle(&mut self, center: PointId, radius: f64) -> CircleId {
        self.push_circle(center, radius, false)
    }

    /// Excluded from the area of any region it happens to sit inside or across.
    pub fn add_construction_circle(&mut self, center: PointId, radius: f64) -> CircleId {
        self.push_circle(center, radius, true)
    }

    fn push_circle(&mut self, center: PointId, radius: f64, construction: bool) -> CircleId {
        self.circles.push(Circle {
            center,
            radius,
            construction,
        });
        CircleId(self.circles.len() - 1)
    }

    /// The circle whose outline passes closest to `position`.
    pub fn nearest_circle(&self, position: DVec2, tolerance: f64) -> Option<CircleId> {
        self.circles
            .iter()
            .enumerate()
            .map(|(index, circle)| {
                let distance = (self.point(circle.center).distance(position) - circle.radius).abs();
                (CircleId(index), distance)
            })
            .filter(|(id, _)| !self.is_erased_circle(*id))
            .filter(|(_, distance)| *distance <= tolerance)
            .min_by(|a, b| a.1.total_cmp(&b.1))
            .map(|(id, _)| id)
    }

    pub fn is_empty(&self) -> bool {
        self.live_segments().next().is_none()
    }

    pub fn point(&self, id: PointId) -> DVec2 {
        self.points[id.0]
    }

    pub fn endpoints(&self, id: SegmentId) -> (DVec2, DVec2) {
        let segment = self.segments[id.0];
        (self.point(segment.start), self.point(segment.end))
    }

    pub fn add_point(&mut self, position: DVec2) -> PointId {
        self.points.push(position);
        PointId(self.points.len() - 1)
    }

    /// Moves an annotation away from where it would sit on its own.
    pub fn offset_dimension(&mut self, target: DimensionTarget, offset: DVec2) {
        if let Some(dimension) = self
            .dimensions
            .iter_mut()
            .find(|dimension| dimension.target == target)
        {
            dimension.offset = Some(offset);
        }
    }

    /// The dimension whose annotation sits nearest `position`, within `tolerance`.
    pub fn nearest_dimension(
        &self,
        position: DVec2,
        tolerance: f64,
        metrics: AnnotationMetrics,
    ) -> Option<DimensionTarget> {
        self.anchors(metrics)
            .into_iter()
            .map(|(target, at)| (target, at.distance(position)))
            .filter(|(_, distance)| *distance <= tolerance)
            .min_by(|a, b| a.1.total_cmp(&b.1))
            .map(|(target, _)| target)
    }

    /// Moves a point where the user dragged it. The origin stays put.
    pub fn move_point(&mut self, point: PointId, position: DVec2) {
        if self.is_origin(point) {
            return;
        }
        if let Some(existing) = self.points.get_mut(point.0) {
            *existing = position;
        }
    }

    /// Reuses an existing point when one is within `tolerance`, so that clicking back onto a corner
    /// joins the geometry there instead of laying a second point on top of it.
    pub fn point_at(&mut self, position: DVec2, tolerance: f64) -> PointId {
        match self.nearest_point(position, tolerance) {
            Some(id) => id,
            None => self.add_point(position),
        }
    }

    pub fn nearest_point(&self, position: DVec2, tolerance: f64) -> Option<PointId> {
        self.points
            .iter()
            .enumerate()
            .map(|(index, point)| (PointId(index), point.distance(position)))
            .filter(|(id, _)| !self.is_erased_point(*id))
            .filter(|(_, distance)| *distance <= tolerance)
            .min_by(|a, b| a.1.total_cmp(&b.1))
            .map(|(id, _)| id)
    }

    /// The segment whose body passes closest to `position`, within `tolerance`.
    pub fn nearest_segment(&self, position: DVec2, tolerance: f64) -> Option<SegmentId> {
        (0..self.segments.len())
            .map(|index| {
                let id = SegmentId(index);
                (id, self.distance_to_segment(id, position))
            })
            .filter(|(id, _)| !self.is_erased_segment(*id))
            .filter(|(_, distance)| *distance <= tolerance)
            .min_by(|a, b| a.1.total_cmp(&b.1))
            .map(|(id, _)| id)
    }

    /// The point on a segment's body nearest `position`, within `tolerance`.
    ///
    /// Drawing onto a line already there is far more common than drawing near
    /// it, so a line pulls harder than the grid does.
    pub fn nearest_on_segment(
        &self,
        position: DVec2,
        tolerance: f64,
    ) -> Option<(SegmentId, DVec2)> {
        (0..self.segments.len())
            .map(SegmentId)
            .filter(|id| !self.is_erased_segment(*id))
            .map(|id| (id, self.project_onto(id, position)))
            .filter(|(_, at)| at.distance(position) <= tolerance)
            .min_by(|a, b| a.1.distance(position).total_cmp(&b.1.distance(position)))
    }

    /// The middle of the nearest segment, within `tolerance`.
    pub fn nearest_midpoint(&self, position: DVec2, tolerance: f64) -> Option<(SegmentId, DVec2)> {
        self.live_segments()
            .map(|(id, _)| {
                let (start, end) = self.endpoints(id);
                (id, (start + end) * 0.5)
            })
            .filter(|(_, middle)| middle.distance(position) <= tolerance)
            .min_by(|a, b| a.1.distance(position).total_cmp(&b.1.distance(position)))
    }

    fn project_onto(&self, id: SegmentId, position: DVec2) -> DVec2 {
        let (start, end) = self.endpoints(id);
        let span = end - start;
        let length_squared = span.length_squared();
        if length_squared < 1e-12 {
            return start;
        }
        let t = ((position - start).dot(span) / length_squared).clamp(0.0, 1.0);
        start + span * t
    }

    /// Makes two points one.
    ///
    /// Everything that referred to `dropped` now refers to `kept`, and any
    /// segment left with the same point at both ends goes: it has no length and
    /// no direction, so it is not a line any more.
    pub fn merge_points(&mut self, kept: PointId, dropped: PointId) {
        if kept == dropped || self.is_origin(dropped) && !self.is_origin(kept) {
            // The origin never moves, so it is always the one kept.
            return self.merge_points(dropped, kept);
        }
        if kept.0 >= self.points.len() || dropped.0 >= self.points.len() {
            return;
        }

        for segment in &mut self.segments {
            if segment.start == dropped {
                segment.start = kept;
            }
            if segment.end == dropped {
                segment.end = kept;
            }
        }
        for circle in &mut self.circles {
            if circle.center == dropped {
                circle.center = kept;
            }
        }

        let collapsed: Vec<Element> = self
            .live_segments()
            .filter(|(_, segment)| segment.start == segment.end)
            .map(|(id, _)| Element::Segment(id))
            .collect();
        for segment in collapsed {
            self.erase(segment);
        }

        Erased::mark(&mut self.erased.points, dropped.0);
        let dimensions = std::mem::take(&mut self.dimensions);
        self.dimensions = dimensions
            .into_iter()
            .map(|dimension| Dimension {
                target: redirect(dimension.target, kept, dropped),
                ..dimension
            })
            .filter(|dimension| self.measures_live(dimension.target))
            .collect();
    }

    pub(crate) fn distance_to_segment(&self, id: SegmentId, position: DVec2) -> f64 {
        let (start, end) = self.endpoints(id);
        let span = end - start;
        let length_squared = span.length_squared();
        if length_squared < 1e-12 {
            return start.distance(position);
        }
        let t = ((position - start).dot(span) / length_squared).clamp(0.0, 1.0);
        (start + span * t).distance(position)
    }

    pub fn add_segment(&mut self, start: PointId, end: PointId) -> SegmentId {
        self.push_segment(start, end, false)
    }

    /// Excluded from the area of any region it happens to sit inside or across.
    pub fn add_construction_segment(&mut self, start: PointId, end: PointId) -> SegmentId {
        self.push_segment(start, end, true)
    }

    fn push_segment(&mut self, start: PointId, end: PointId, construction: bool) -> SegmentId {
        self.segments.push(Segment {
            start,
            end,
            construction,
        });
        SegmentId(self.segments.len() - 1)
    }

    pub fn segment_length(&self, id: SegmentId) -> f64 {
        let (start, end) = self.endpoints(id);
        start.distance(end)
    }

    pub fn dimension_of(&self, target: DimensionTarget) -> Option<&Dimension> {
        self.dimensions
            .iter()
            .find(|dimension| dimension.target == target)
    }

    /// Records a value the user typed, replacing any previous one on the same
    /// target. Moving the geometry is a separate step: the very first dimension
    /// of a document sets its scale instead of resizing anything, and a driven
    /// one never moves anything at all.
    pub fn set_dimension(&mut self, target: DimensionTarget, value: f64, driven: bool) {
        match self
            .dimensions
            .iter_mut()
            .find(|dimension| dimension.target == target)
        {
            Some(existing) => {
                existing.value = value;
                existing.driven = driven;
            }
            None => self.dimensions.push(Dimension {
                target,
                value,
                driven,
                offset: None,
            }),
        }
    }

    /// Where the perpendicular from a point meets the line a segment lies on.
    ///
    /// The line, not the segment: a distance to a line is still a distance when
    /// the foot falls past the end of the drawn part, and a drawing says so
    /// with a thin extension line.
    pub fn foot_on_segment(&self, point: PointId, segment: SegmentId) -> Option<DVec2> {
        if point.0 >= self.points.len() || segment.0 >= self.segments.len() {
            return None;
        }
        let (start, end) = self.endpoints(segment);
        let span = end - start;
        let length = span.length();
        if length < 1e-9 {
            return None;
        }
        let direction = span / length;
        Some(start + direction * (self.point(point) - start).dot(direction))
    }

    /// The gap between two points along one axis of the sketch, in units.
    pub fn projected_gap(
        &self,
        from: PointId,
        to: PointId,
        axis: crate::constraints::SketchAxis,
    ) -> Option<f64> {
        if from.0 >= self.points.len() || to.0 >= self.points.len() {
            return None;
        }
        Some(
            (self.point(to) - self.point(from))
                .dot(axis.direction())
                .abs(),
        )
    }

    /// The distance from a point to the line a segment lies on, in units.
    pub fn point_to_segment(&self, point: PointId, segment: SegmentId) -> Option<f64> {
        let foot = self.foot_on_segment(point, segment)?;
        Some(self.point(point).distance(foot))
    }

    /// The angle at the point two segments share, in degrees, or `None` when
    /// they do not meet.
    pub fn angle_between(&self, first: SegmentId, second: SegmentId) -> Option<f64> {
        let (pivot, a, b) = self.corner(first, second)?;
        let first = (self.point(a) - self.point(pivot)).normalize_or_zero();
        let second = (self.point(b) - self.point(pivot)).normalize_or_zero();
        if first == DVec2::ZERO || second == DVec2::ZERO {
            return None;
        }
        Some(first.dot(second).clamp(-1.0, 1.0).acos().to_degrees())
    }

    /// The shared point of two segments, with their far ends.
    fn corner(&self, first: SegmentId, second: SegmentId) -> Option<(PointId, PointId, PointId)> {
        let (a, b) = (self.segments[first.0], self.segments[second.0]);
        for (pivot, far_a) in [(a.start, a.end), (a.end, a.start)] {
            if b.start == pivot {
                return Some((pivot, far_a, b.end));
            }
            if b.end == pivot {
                return Some((pivot, far_a, b.start));
            }
        }
        None
    }

    /// The corner two segments share, as positions: the pivot and the two far
    /// ends. What an annotation needs to draw the angle.
    pub fn corner_points(
        &self,
        first: SegmentId,
        second: SegmentId,
    ) -> Option<(DVec2, DVec2, DVec2)> {
        let (pivot, a, b) = self.corner(first, second)?;
        Some((self.point(pivot), self.point(a), self.point(b)))
    }

    /// The shared point of two segments, with their far ends.
    pub(crate) fn shared_corner(
        &self,
        first: SegmentId,
        second: SegmentId,
    ) -> Option<(PointId, PointId, PointId)> {
        self.corner(first, second)
    }

    pub(crate) fn translate_point(&mut self, point: PointId, delta: DVec2) {
        self.points[point.0] += delta;
    }

    /// Changes a circle's size by a step of the solve. Never below nothing: a
    /// circle turned inside out is not a circle.
    pub(crate) fn grow_circle(&mut self, circle: CircleId, delta: f64) {
        if let Some(round) = self.circles.get_mut(circle.0) {
            round.radius = (round.radius + delta).max(1e-9);
        }
    }

    pub(crate) fn place_point(&mut self, point: PointId, position: DVec2) {
        self.points[point.0] = position;
    }

    /// The angle a segment makes with one of the sketch axes, in degrees.
    pub fn angle_with_axis(&self, segment: SegmentId, axis: SketchAxis) -> Option<f64> {
        let (start, end) = self.endpoints(segment);
        let direction = (end - start).normalize_or_zero();
        if direction == DVec2::ZERO {
            return None;
        }
        Some(
            direction
                .dot(axis.direction())
                .clamp(-1.0, 1.0)
                .acos()
                .to_degrees(),
        )
    }

    pub fn set_circle_radius(&mut self, id: CircleId, radius: f64) -> LengthOutcome {
        if radius <= 0.0 {
            return LengthOutcome::Degenerate;
        }
        self.circles[id.0].radius = radius;
        LengthOutcome::Exact
    }

    /// Smallest axis-aligned box containing every point, in sketch coordinates.
    pub fn bounds(&self) -> Option<(DVec2, DVec2)> {
        let first = *self.points.first()?;
        Some(
            self.points
                .iter()
                .enumerate()
                .filter(|(rank, _)| !self.is_erased_point(PointId(*rank)))
                .fold((first, first), |(min, max), (_, point)| {
                    (min.min(*point), max.max(*point))
                }),
        )
    }
}

impl Sketch {
    /// Whether a value on this target would say anything new.
    ///
    /// A constraint is redundant when its equation is a combination of those
    /// already there — exactly the case of a triangle's third side once its
    /// other sides and angles are fixed. Counting constraints could never see
    /// that; comparing their directions can.
    pub fn would_be_redundant(&self, target: DimensionTarget, millimeters_per_unit: f64) -> bool {
        if self.dimension_of(target).is_some() {
            return false;
        }

        let existing = self.equations(millimeters_per_unit);
        let Some(candidate) = self.candidate_equation(target, millimeters_per_unit) else {
            return false;
        };
        is_dependent(&existing, &candidate)
    }

    /// The equation a not-yet-placed dimension would contribute, taken at the
    /// value the geometry already has so only its direction matters.
    fn candidate_equation(
        &self,
        target: DimensionTarget,
        millimeters_per_unit: f64,
    ) -> Option<crate::equation::Equation> {
        let value = match target {
            DimensionTarget::Length(segment) => {
                (segment.0 < self.segments.len()).then(|| self.segment_length(segment))?
                    * millimeters_per_unit.max(1e-9)
            }
            DimensionTarget::Distance { from, to } => {
                let (a, b) = (self.points.get(from.0)?, self.points.get(to.0)?);
                a.distance(*b) * millimeters_per_unit.max(1e-9)
            }
            DimensionTarget::Angle { first, second } => self.angle_between(first, second)?,
            DimensionTarget::AxisAngle { segment, axis } => self.angle_with_axis(segment, axis)?,
            DimensionTarget::PointToSegment { point, segment } => {
                self.point_to_segment(point, segment)? * millimeters_per_unit.max(1e-9)
            }
            DimensionTarget::Projected { from, to, axis } => {
                self.projected_gap(from, to, axis)? * millimeters_per_unit.max(1e-9)
            }
            DimensionTarget::Radius(circle) => {
                self.circles.get(circle.0)?.radius * millimeters_per_unit.max(1e-9)
            }
            DimensionTarget::Diameter(circle) => {
                self.circles.get(circle.0)?.radius * 2.0 * millimeters_per_unit.max(1e-9)
            }
            DimensionTarget::ArcRadius(arc) => self.arc_radius_value(arc, millimeters_per_unit)?,
            DimensionTarget::ArcSweep(arc) => self.arc_sweep_value(arc)?,
        };

        let mut probe = self.clone();
        probe.dimensions.clear();
        probe.set_dimension(target, value, false);
        probe.equations(millimeters_per_unit).into_iter().next()
    }

    /// Puts a point where it was dropped and settles the rest of the drawing
    /// around it, that point staying exactly where it was put.
    pub fn settle_around(
        &mut self,
        point: PointId,
        position: DVec2,
        millimeters_per_unit: f64,
    ) -> LengthOutcome {
        self.settle_around_all(&[(point, position)], millimeters_per_unit)
    }

    /// The same for a whole handful of points dropped at once, which is how a
    /// selection is moved in one block.
    ///
    /// Held, they do not give: the drawing settles around them rather than
    /// pulling them back, so a shape follows the mouse instead of squirming
    /// away from it.
    ///
    /// When holding them is more than the drawing can bear — a corner dragged
    /// somewhere no tangency can reach it — the values already given win over
    /// the cursor: everything goes back and settles the ordinary way. Leaving
    /// the half-solved state was what let a circle be dragged out of shape and
    /// stay that way until the next change put it right.
    pub fn settle_around_all(
        &mut self,
        dropped: &[(PointId, DVec2)],
        millimeters_per_unit: f64,
    ) -> LengthOutcome {
        let kept = self.shapes_now();
        let place = |sketch: &mut Self| {
            for (point, position) in dropped {
                sketch.move_point(*point, *position);
            }
        };

        place(self);
        self.held = dropped.iter().map(|(point, _)| *point).collect();
        let outcome = self.resolve(millimeters_per_unit);
        self.held.clear();
        if outcome == LengthOutcome::Exact {
            return outcome;
        }

        self.points.clone_from(&kept.0);
        self.circles.clone_from(&kept.1);
        place(self);
        let outcome = self.resolve(millimeters_per_unit);
        if !self.has_a_collapsed_trait(self.drawing_size()) && !self.has_a_flipped_tangent() {
            return outcome;
        }

        // Neither way leaves a drawing worth keeping: a trait may have collapsed,
        // or a tangency's contact slid off its segment. The gesture is refused
        // rather than the shape broken — the point simply does not go there.
        self.give_back(kept);
        LengthOutcome::BestEffort
    }

    /// Re-satisfies every dimension at once, reporting whether it managed.
    pub fn resolve(&mut self, millimeters_per_unit: f64) -> LengthOutcome {
        match self.solve(millimeters_per_unit) {
            SolveOutcome::Solved | SolveOutcome::Nothing => LengthOutcome::Exact,
            SolveOutcome::Residual => LengthOutcome::BestEffort,
        }
    }
}

/// Points a dimension at the point that was kept.
fn redirect(target: DimensionTarget, kept: PointId, dropped: PointId) -> DimensionTarget {
    let swap = |point: PointId| if point == dropped { kept } else { point };
    match target {
        DimensionTarget::Distance { from, to } => DimensionTarget::Distance {
            from: swap(from),
            to: swap(to),
        },
        DimensionTarget::PointToSegment { point, segment } => DimensionTarget::PointToSegment {
            point: swap(point),
            segment,
        },
        DimensionTarget::Projected { from, to, axis } => DimensionTarget::Projected {
            from: swap(from),
            to: swap(to),
            axis,
        },
        other => other,
    }
}

#[cfg(test)]
mod tests;
