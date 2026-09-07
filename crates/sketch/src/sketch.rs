use glam::DVec2;
use serde::{Deserialize, Serialize};

use crate::constraints::{Constraint, Dimension, DimensionTarget, Freedom, SketchAxis};
use crate::plane::WorkPlane;
use crate::solver::{self, SolveOutcome};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PointId(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SegmentId(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CircleId(pub usize);

/// A circle, kept as a centre point shared with the rest of the drawing plus a
/// radius, so that moving the centre moves the circle with it.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Circle {
    pub center: PointId,
    pub radius: f64,
}

/// A straight line between two points. Points are shared: chaining a polyline
/// reuses the previous end, which is what makes a dimension able to drag the
/// rest of the chain along.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Segment {
    pub start: PointId,
    pub end: PointId,
}

/// What happened when a length was applied.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LengthOutcome {
    /// The segment now has exactly the requested length, and whatever hung off
    /// its far end moved rigidly with it.
    Exact,
    /// The far end could not move freely because the geometry loops back to the
    /// fixed end. Only the far end moved, so the shapes around it are distorted.
    BestEffort,
    /// The segment has no direction to stretch along.
    Degenerate,
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
    erased: Erased,
    /// The point the user is holding under the cursor, while a drag lasts.
    ///
    /// It does not give: the drawing settles *around* it rather than pulling it
    /// back, which is what makes a shape follow the mouse instead of squirming
    /// away from it. Nothing to save — it lives only as long as the gesture.
    #[serde(skip)]
    held: Option<PointId>,
}

/// The ranks that no longer count.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct Erased {
    #[serde(default)]
    points: Vec<bool>,
    #[serde(default)]
    segments: Vec<bool>,
    #[serde(default)]
    circles: Vec<bool>,
}

impl Erased {
    fn holds(list: &[bool], rank: usize) -> bool {
        list.get(rank).copied().unwrap_or(false)
    }

    fn mark(list: &mut Vec<bool>, rank: usize) {
        if list.len() <= rank {
            list.resize(rank + 1, false);
        }
        list[rank] = true;
    }
}

/// One thing a sketch is made of, for deleting it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Element {
    Point(PointId),
    Segment(SegmentId),
    Circle(CircleId),
}

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
            dimensions: Vec::new(),
            constraints: Vec::new(),
            erased: Erased::default(),
            held: None,
        }
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
                    .collect();
                for element in touched {
                    self.erase(element);
                }
            }
            Element::Segment(segment) => Erased::mark(&mut self.erased.segments, segment.0),
            Element::Circle(circle) => Erased::mark(&mut self.erased.circles, circle.0),
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
            if let Constraint::Tangent { at: Some(point), .. } = constraint {
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
            if let Constraint::Tangent { at: Some(point), .. } = self.constraints.remove(rank) {
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
    /// centre, so both read as fixed although only one of them was named.
    pub fn is_held(&self, element: Element) -> bool {
        let holds_point = |point: PointId, held: Element| match held {
            Element::Point(id) => id == point,
            Element::Segment(id) => self
                .segments
                .get(id.0)
                .is_some_and(|segment| segment.start == point || segment.end == point),
            Element::Circle(id) => self.circles.get(id.0).is_some_and(|c| c.center == point),
        };
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
            Constraint::OnSegment {
                point: held,
                segment: on,
            }
            | Constraint::Midpoint {
                point: held,
                segment: on,
            } => point(held) && segment(on),
            Constraint::Tangent {
                circle: round,
                segment: line,
                ..
            } => circle(round) && segment(line),
            Constraint::OnCircle {
                point: held,
                circle: round,
            } => point(held) && circle(round),
            Constraint::AxisCollinear { segment: on, .. } => segment(on),
            Constraint::Fixed { element } => match element {
                Element::Point(held) => point(held),
                Element::Segment(held) => segment(held),
                Element::Circle(held) => circle(held),
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

    /// Whether the user is holding this point under the cursor right now.
    pub(crate) fn is_held_still(&self, point: PointId) -> bool {
        self.held == Some(point)
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
        self.circles.push(Circle { center, radius });
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

    /// The dimension whose annotation sits nearest `position`, within
    /// `tolerance`. Needs where each one is drawn, which only the front-end
    /// knows.
    pub fn nearest_dimension(
        &self,
        anchors: &[(DimensionTarget, DVec2)],
        position: DVec2,
        tolerance: f64,
    ) -> Option<DimensionTarget> {
        anchors
            .iter()
            .map(|(target, at)| (*target, at.distance(position)))
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

    /// Reuses an existing point when one is within `tolerance`, so that clicking
    /// back onto a corner joins the geometry there instead of laying a second
    /// point on top of it.
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

    fn distance_to_segment(&self, id: SegmentId, position: DVec2) -> f64 {
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
        self.segments.push(Segment { start, end });
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
    /// Coordinates that no constraint holds, worked out from the rank of the
    /// system: how many independent things the dimensions actually say.
    ///
    /// Points pinned to the origin are taken out of the count outright, since
    /// neither of their coordinates can move.
    pub fn freedom(&self, millimeters_per_unit: f64) -> Freedom {
        // The origin never moves, so its two coordinates are not in play.
        let free_coordinates = self.points.len().saturating_sub(1) * 2;
        let held = solver::rank(&self.analysed_system(millimeters_per_unit)).min(free_coordinates);

        // A circle brings its own radius, which only its own dimension can
        // settle; that pair never touches the point coordinates.
        let radii_without_a_value = self.circles.len().saturating_sub(
            self.dimensions
                .iter()
                .filter(|dimension| {
                    !dimension.driven && matches!(dimension.target, DimensionTarget::Radius(_))
                })
                .count(),
        );

        Freedom {
            degrees_of_freedom: (free_coordinates - held) + radii_without_a_value,
        }
    }

    pub fn is_fully_constrained(&self, millimeters_per_unit: f64) -> bool {
        self.points.len() > 1 && self.freedom(millimeters_per_unit).fully_constrained()
    }

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
        solver::is_dependent(&existing, &candidate)
    }

    /// The equation a not-yet-placed dimension would contribute, taken at the
    /// value the geometry already has so only its direction matters.
    fn candidate_equation(
        &self,
        target: DimensionTarget,
        millimeters_per_unit: f64,
    ) -> Option<solver::Equation> {
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
        };

        let mut probe = self.clone();
        probe.dimensions.clear();
        probe.set_dimension(target, value, false);
        probe.equations(millimeters_per_unit).into_iter().next()
    }

    /// Whether the drawing has any freedom left, as a whole.
    pub fn is_settled(&self, millimeters_per_unit: f64) -> bool {
        self.is_fully_constrained(millimeters_per_unit)
    }

    /// Which points can no longer move at all.
    ///
    /// A drawing is rarely all-or-nothing: one contour can be nailed down while
    /// another is still floating beside it. Showing that per point, rather than
    /// one verdict for the whole sketch, says what is left to do.
    pub fn settled_points(&self, millimeters_per_unit: f64) -> Vec<bool> {
        let variables = self.variables();
        let pinned: Vec<bool> = (0..self.points.len())
            .map(|index| self.is_origin(PointId(index)))
            .collect();
        let free = solver::null_space(
            &self.analysed_system(millimeters_per_unit),
            &pinned,
            variables,
        );

        (0..self.points.len())
            .map(|index| {
                if pinned[index] {
                    return true;
                }
                // Settled means no way to move survives at this point.
                free.iter().all(|direction| {
                    direction[index * 2].abs() < 1e-3 && direction[index * 2 + 1].abs() < 1e-3
                })
            })
            .collect()
    }

    /// Re-satisfies every dimension at once, reporting whether it managed.
    /// Puts a point where it was dropped and settles the rest of the drawing
    /// around it, that point staying exactly where it was put.
    ///
    /// Solving without holding it lets the constraints pull it back part of the
    /// way, so the shape slides out from under the cursor — which is what made
    /// dragging a tangent circle feel like wrestling it.
    pub fn settle_around(
        &mut self,
        point: PointId,
        position: DVec2,
        millimeters_per_unit: f64,
    ) -> LengthOutcome {
        self.move_point(point, position);
        self.held = Some(point);
        let outcome = self.resolve(millimeters_per_unit);
        self.held = None;
        outcome
    }

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
mod tests {
    use super::*;
    use crate::constraints::SketchAxis;

    #[test]
    fn a_point_is_pushed_square_to_a_line() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let start = sketch.add_point(DVec2::new(0.0, 0.0));
        let end = sketch.add_point(DVec2::new(40.0, 0.0));
        let line = sketch.add_segment(start, end);
        let floating = sketch.add_point(DVec2::new(10.0, 5.0));

        sketch.set_dimension(
            DimensionTarget::PointToSegment {
                point: floating,
                segment: line,
            },
            12.0,
            false,
        );
        assert_eq!(sketch.resolve(1.0), LengthOutcome::Exact);

        let distance = sketch.point_to_segment(floating, line).unwrap();
        assert!((distance - 12.0).abs() < 1e-2, "distance = {distance}");
    }

    #[test]
    fn the_distance_holds_when_the_foot_falls_off_the_segment() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let start = sketch.add_point(DVec2::new(0.0, 0.0));
        let end = sketch.add_point(DVec2::new(10.0, 0.0));
        let line = sketch.add_segment(start, end);
        let far = sketch.add_point(DVec2::new(80.0, 3.0));

        sketch.set_dimension(
            DimensionTarget::PointToSegment {
                point: far,
                segment: line,
            },
            20.0,
            false,
        );
        sketch.resolve(1.0);

        let distance = sketch.point_to_segment(far, line).unwrap();
        assert!((distance - 20.0).abs() < 1e-2, "distance = {distance}");
    }

    #[test]
    fn a_width_moves_the_trait_sideways_and_leaves_its_height_alone() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let start = sketch.add_point(DVec2::new(0.0, 0.0));
        let end = sketch.add_point(DVec2::new(40.0, 30.0));
        sketch.add_segment(start, end);

        sketch.set_dimension(
            DimensionTarget::Projected {
                from: start,
                to: end,
                axis: SketchAxis::U,
            },
            60.0,
            false,
        );
        assert_eq!(sketch.resolve(1.0), LengthOutcome::Exact);

        let width = sketch.projected_gap(start, end, SketchAxis::U).unwrap();
        let height = sketch.projected_gap(start, end, SketchAxis::V).unwrap();
        assert!((width - 60.0).abs() < 1e-2, "largeur = {width}");
        assert!((height - 30.0).abs() < 1e-2, "hauteur = {height}");
    }

    #[test]
    fn a_width_and_a_height_together_pin_a_trait_down() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let start = Sketch::ORIGIN;
        let end = sketch.add_point(DVec2::new(40.0, 30.0));
        sketch.add_segment(start, end);

        for (axis, value) in [(SketchAxis::U, 80.0), (SketchAxis::V, 15.0)] {
            sketch.set_dimension(
                DimensionTarget::Projected {
                    from: start,
                    to: end,
                    axis,
                },
                value,
                false,
            );
        }
        assert_eq!(sketch.resolve(1.0), LengthOutcome::Exact);

        let landed = sketch.point(end);
        assert!(
            (landed - DVec2::new(80.0, 15.0)).length() < 1e-2,
            "arrivée = {landed:?}"
        );
    }

    /// Un dessin minimal : deux traits qui partent du même coin.
    fn corner() -> (Sketch, SegmentId, SegmentId) {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let pivot = sketch.add_point(DVec2::new(10.0, 10.0));
        let a = sketch.add_point(DVec2::new(60.0, 14.0));
        let b = sketch.add_point(DVec2::new(16.0, 55.0));
        let first = sketch.add_segment(pivot, a);
        let second = sketch.add_segment(pivot, b);
        (sketch, first, second)
    }

    fn direction(sketch: &Sketch, segment: SegmentId) -> DVec2 {
        let (start, end) = sketch.endpoints(segment);
        (end - start).normalize()
    }

    #[test]
    fn a_right_angle_can_be_asked_for_without_a_value() {
        let (mut sketch, first, second) = corner();
        sketch.add_constraint(Constraint::Perpendicular { first, second });
        assert_eq!(sketch.resolve(1.0), LengthOutcome::Exact);

        let square = direction(&sketch, first).dot(direction(&sketch, second));
        assert!(square.abs() < 1e-6, "produit scalaire = {square}");
    }

    #[test]
    fn two_traits_can_be_told_to_keep_the_same_direction() {
        let (mut sketch, first, second) = corner();
        sketch.add_constraint(Constraint::Parallel { first, second });
        assert_eq!(sketch.resolve(1.0), LengthOutcome::Exact);

        let across = direction(&sketch, first).perp_dot(direction(&sketch, second));
        assert!(across.abs() < 1e-6, "produit vectoriel = {across}");
    }

    #[test]
    fn two_traits_can_be_told_to_have_the_same_length() {
        let (mut sketch, first, second) = corner();
        sketch.add_constraint(Constraint::Equal { first, second });
        assert_eq!(sketch.resolve(1.0), LengthOutcome::Exact);

        let gap = sketch.segment_length(first) - sketch.segment_length(second);
        assert!(gap.abs() < 1e-4, "écart de longueur = {gap}");
    }

    #[test]
    fn a_point_can_be_held_on_a_line() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let start = sketch.add_point(DVec2::new(0.0, 0.0));
        let end = sketch.add_point(DVec2::new(100.0, 20.0));
        let line = sketch.add_segment(start, end);
        let floating = sketch.add_point(DVec2::new(40.0, 50.0));

        sketch.add_constraint(Constraint::OnSegment {
            point: floating,
            segment: line,
        });
        assert_eq!(sketch.resolve(1.0), LengthOutcome::Exact);

        let gap = sketch.point_to_segment(floating, line).unwrap();
        assert!(gap < 1e-6, "le point est à {gap} de la droite");
    }

    #[test]
    fn a_point_can_be_held_halfway_along_a_trait() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let start = sketch.add_point(DVec2::new(0.0, 0.0));
        let end = sketch.add_point(DVec2::new(80.0, 40.0));
        let line = sketch.add_segment(start, end);
        let floating = sketch.add_point(DVec2::new(10.0, 60.0));

        sketch.add_constraint(Constraint::Midpoint {
            point: floating,
            segment: line,
        });
        assert_eq!(sketch.resolve(1.0), LengthOutcome::Exact);

        let middle = (sketch.point(start) + sketch.point(end)) * 0.5;
        assert!(sketch.point(floating).distance(middle) < 1e-6);
    }

    #[test]
    fn a_circle_brushes_a_line_drawn_the_other_way_round_too() {
        // The trait runs right to left, which puts the circle on the negative
        // side of it. Growing the circle then closes the gap the other way; a
        // size correction taken with the wrong sign sends the radius running.
        let mut sketch = Sketch::new(WorkPlane::XY);
        let start = sketch.add_point(DVec2::new(40.0, 0.0));
        let end = sketch.add_point(DVec2::new(-40.0, 0.0));
        let line = sketch.add_segment(start, end);
        let center = sketch.add_point(DVec2::new(0.0, 30.0));
        let circle = sketch.add_circle(center, 12.0);
        sketch.add_constraint(Constraint::Fixed {
            element: Element::Segment(line),
        });
        sketch.add_constraint(Constraint::Fixed {
            element: Element::Circle(circle),
        });
        sketch.add_constraint(Constraint::Tangent {
            circle,
            segment: line,
            at: None,
        });

        assert_eq!(sketch.resolve(1.0), LengthOutcome::Exact);
        let round = sketch.circle(circle);
        assert!(
            (round.radius - 30.0).abs() < 1e-3,
            "le rayon a rejoint la droite : {}",
            round.radius
        );
    }

    #[test]
    fn a_tangency_keeps_a_point_where_the_two_touch() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let start = sketch.add_point(DVec2::new(-40.0, 0.0));
        let end = sketch.add_point(DVec2::new(40.0, 0.0));
        let line = sketch.add_segment(start, end);
        let center = sketch.add_point(DVec2::new(0.0, 30.0));
        let circle = sketch.add_circle(center, 25.0);

        sketch.add_constraint(Constraint::Tangent {
            circle,
            segment: line,
            at: None,
        });
        assert_eq!(sketch.resolve(1.0), LengthOutcome::Exact);

        let Some(Constraint::Tangent { at: Some(touch), .. }) = sketch
            .constraints()
            .iter()
            .find(|rule| matches!(rule, Constraint::Tangent { .. }))
            .copied()
        else {
            panic!("la tangence n'a pas de point de contact");
        };
        let contact = sketch.point(touch);
        let foot = sketch.foot_on_segment(center, line).unwrap();
        assert!(contact.distance(foot) < 1e-3, "le contact a glissé : {contact}");

        // The line moved out from under it: the contact follows, it does not
        // stay behind on the old spot.
        sketch.settle_around(end, DVec2::new(40.0, 40.0), 1.0);
        let foot = sketch.foot_on_segment(sketch.circle(circle).center, line).unwrap();
        assert!(sketch.point(touch).distance(foot) < 1e-2);
    }

    #[test]
    fn a_point_on_the_rim_resizes_the_circle_when_it_is_pulled() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let center = sketch.add_point(DVec2::new(0.0, 0.0));
        let circle = sketch.add_circle(center, 10.0);
        let rim = sketch.add_point(DVec2::new(10.0, 0.0));
        sketch.add_constraint(Constraint::OnCircle { point: rim, circle });

        sketch.settle_around(rim, DVec2::new(30.0, 0.0), 1.0);

        assert!(
            sketch.point(rim).distance(DVec2::new(30.0, 0.0)) < 1e-9,
            "le point lâché ne bouge plus"
        );
        let round = sketch.circle(circle);
        let reach = sketch.point(rim).distance(sketch.point(round.center));
        assert!(
            (reach - round.radius).abs() < 1e-3,
            "le point est resté sur le bord"
        );
        assert!(round.radius > 10.0, "le cercle a grandi : {}", round.radius);
    }

    #[test]
    fn a_circle_dragged_by_its_centre_stays_under_the_cursor() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let start = sketch.add_point(DVec2::new(-60.0, 0.0));
        let end = sketch.add_point(DVec2::new(60.0, 0.0));
        let line = sketch.add_segment(start, end);
        let center = sketch.add_point(DVec2::new(0.0, 20.0));
        let circle = sketch.add_circle(center, 20.0);
        sketch.add_constraint(Constraint::Tangent {
            circle,
            segment: line,
            at: None,
        });

        let dropped = DVec2::new(35.0, 45.0);
        sketch.settle_around(center, dropped, 1.0);

        assert!(
            sketch.point(center).distance(dropped) < 1e-9,
            "le centre a glissé sous le curseur : {}",
            sketch.point(center)
        );
        let gap = sketch.point_to_segment(center, line).unwrap();
        assert!(
            (gap - sketch.circle(circle).radius).abs() < 1e-2,
            "et la tangence tient toujours"
        );
    }

    #[test]
    fn a_circle_told_to_brush_a_line_takes_the_size_that_touches() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let start = sketch.add_point(DVec2::new(0.0, 0.0));
        let end = sketch.add_point(DVec2::new(100.0, 0.0));
        let line = sketch.add_segment(start, end);
        let center = sketch.add_point(DVec2::new(50.0, 30.0));
        let circle = sketch.add_circle(center, 20.0);

        sketch.add_constraint(Constraint::Tangent {
            circle,
            segment: line,
            at: None,
        });
        assert_eq!(sketch.resolve(1.0), LengthOutcome::Exact);

        // Nothing says how big it is, so the circle meets the line by moving
        // and by growing at once — whichever costs least.
        let gap = sketch.point_to_segment(center, line).unwrap();
        assert!(
            (gap - sketch.circle(circle).radius).abs() < 1e-6,
            "le cercle ne touche pas : {gap} contre {}",
            sketch.circle(circle).radius
        );
        assert!(sketch.circle(circle).radius > 20.0, "il a bien grandi");
    }

    #[test]
    fn a_circle_of_a_said_size_moves_to_keep_touching() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let start = sketch.add_point(DVec2::new(0.0, 0.0));
        let end = sketch.add_point(DVec2::new(100.0, 0.0));
        let line = sketch.add_segment(start, end);
        let center = sketch.add_point(DVec2::new(50.0, 30.0));
        let circle = sketch.add_circle(center, 20.0);

        sketch.add_constraint(Constraint::Tangent {
            circle,
            segment: line,
            at: None,
        });
        sketch.set_dimension(DimensionTarget::Radius(circle), 20.0, false);
        assert_eq!(sketch.resolve(1.0), LengthOutcome::Exact);

        // The size is the decision now, so the circle comes down to the line.
        let held = sketch.circle(circle).radius;
        assert!((held - 20.0).abs() < 1e-3, "rayon = {held}");
        let gap = sketch.point_to_segment(center, line).unwrap();
        assert!((gap - 20.0).abs() < 1e-3, "distance au centre = {gap}");
    }

    #[test]
    fn an_inscribed_circle_follows_the_triangle_it_sits_in() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let a = sketch.add_point(DVec2::new(0.0, 0.0));
        let b = sketch.add_point(DVec2::new(200.0, 0.0));
        let c = sketch.add_point(DVec2::new(100.0, 150.0));
        let sides = [
            sketch.add_segment(a, b),
            sketch.add_segment(b, c),
            sketch.add_segment(c, a),
        ];
        let (place, radius) = crate::construct::circle_touching_three(
            (sketch.point(a), sketch.point(b)),
            (sketch.point(b), sketch.point(c)),
            (sketch.point(c), sketch.point(a)),
        )
        .unwrap();
        let center = sketch.add_point(place);
        let circle = sketch.add_circle(center, radius);
        for segment in sides {
            sketch.add_constraint(Constraint::Tangent {
                circle,
                segment,
                at: None,
            });
        }

        sketch.set_dimension(DimensionTarget::Length(sides[0]), 300.0, false);
        sketch.resolve(1.0);

        let held = sketch.circle(circle).radius;
        for segment in sides {
            let gap = sketch.point_to_segment(center, segment).unwrap();
            assert!(
                (gap - held).abs() < 0.05,
                "le cercle est à {gap} d'un côté pour un rayon de {held}"
            );
        }
        assert!((sketch.segment_length(sides[0]) - 300.0).abs() < 0.05);
    }

    #[test]
    fn two_traits_can_be_laid_on_the_same_line() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let a = sketch.add_point(DVec2::new(0.0, 0.0));
        let b = sketch.add_point(DVec2::new(50.0, 0.0));
        let c = sketch.add_point(DVec2::new(70.0, 18.0));
        let d = sketch.add_point(DVec2::new(120.0, 30.0));
        let first = sketch.add_segment(a, b);
        let second = sketch.add_segment(c, d);

        sketch.add_constraint(Constraint::Collinear { first, second });
        assert_eq!(sketch.resolve(1.0), LengthOutcome::Exact);

        for point in [c, d] {
            let gap = sketch.point_to_segment(point, first).unwrap();
            assert!(gap < 1e-3, "un bout est à {gap} de l'autre droite");
        }
    }

    #[test]
    fn the_second_trait_takes_the_length_of_the_first() {
        let (mut sketch, first, second) = corner();
        let wanted = sketch.segment_length(first);
        sketch.add_constraint(Constraint::Equal { first, second });
        assert_eq!(sketch.resolve(1.0), LengthOutcome::Exact);

        assert!(
            (sketch.segment_length(first) - wanted).abs() < 1e-9,
            "le premier trait a bougé"
        );
        assert!((sketch.segment_length(second) - wanted).abs() < 1e-4);
    }

    #[test]
    fn a_trait_can_be_laid_on_an_axis_of_the_sketch() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let start = sketch.add_point(DVec2::new(10.0, 12.0));
        let end = sketch.add_point(DVec2::new(70.0, 40.0));
        let line = sketch.add_segment(start, end);

        sketch.add_constraint(Constraint::AxisCollinear {
            segment: line,
            axis: SketchAxis::U,
        });
        assert_eq!(sketch.resolve(1.0), LengthOutcome::Exact);

        assert!(sketch.point(start).y.abs() < 1e-6);
        assert!(sketch.point(end).y.abs() < 1e-6);
    }

    #[test]
    fn fixing_a_trait_holds_both_of_its_ends() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let start = sketch.add_point(DVec2::new(10.0, 10.0));
        let end = sketch.add_point(DVec2::new(60.0, 10.0));
        let line = sketch.add_segment(start, end);
        let far = sketch.add_point(DVec2::new(60.0, 60.0));
        let other = sketch.add_segment(end, far);

        sketch.add_constraint(Constraint::Fixed {
            element: Element::Segment(line),
        });
        sketch.set_dimension(DimensionTarget::Length(other), 90.0, false);
        sketch.resolve(1.0);

        assert!(sketch.point(start).distance(DVec2::new(10.0, 10.0)) < 1e-9);
        assert!(sketch.point(end).distance(DVec2::new(60.0, 10.0)) < 1e-9);
        assert!((sketch.segment_length(other) - 90.0).abs() < 1e-4);
    }

    #[test]
    fn a_fixed_point_stays_where_it_was_put() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let start = sketch.add_point(DVec2::new(10.0, 10.0));
        let end = sketch.add_point(DVec2::new(60.0, 10.0));
        let line = sketch.add_segment(start, end);

        sketch.add_constraint(Constraint::Fixed {
            element: Element::Point(start),
        });
        sketch.set_dimension(DimensionTarget::Length(line), 90.0, false);
        assert_eq!(sketch.resolve(1.0), LengthOutcome::Exact);

        assert!(sketch.point(start).distance(DVec2::new(10.0, 10.0)) < 1e-9);
        assert!((sketch.segment_length(line) - 90.0).abs() < 1e-4);
    }

    #[test]
    fn a_rule_goes_when_what_it_spoke_of_goes() {
        let (mut sketch, first, second) = corner();
        sketch.add_constraint(Constraint::Perpendicular { first, second });
        assert_eq!(sketch.constraints().len(), 1);

        sketch.erase(Element::Segment(second));
        assert!(sketch.constraints().is_empty());
    }

    #[test]
    fn changing_an_angle_turns_the_far_shape_instead_of_bending_it() {
        // A chain hung off the origin, with a corner dimensioned halfway along
        // and nothing else holding it: exactly the drawing in progress where
        // the far end used to wander.
        let mut sketch = Sketch::new(WorkPlane::XY);
        let o = Sketch::ORIGIN;
        let a = sketch.add_point(DVec2::new(40.0, 0.0));
        let corner = sketch.add_point(DVec2::new(80.0, 0.0));
        let c = sketch.add_point(DVec2::new(80.0, 40.0));
        let d = sketch.add_point(DVec2::new(120.0, 60.0));
        sketch.add_segment(o, a);
        let first = sketch.add_segment(a, corner);
        let second = sketch.add_segment(corner, c);
        sketch.add_segment(c, d);

        sketch.set_dimension(DimensionTarget::Angle { first, second }, 90.0, false);
        sketch.resolve(1.0);

        let shape_of = |sketch: &Sketch| {
            [
                sketch.point(o).distance(sketch.point(a)),
                sketch.point(c).distance(sketch.point(d)),
                (sketch.point(d) - sketch.point(c))
                    .perp_dot(sketch.point(corner) - sketch.point(c)),
            ]
        };
        let (was_anchored, was_far) = (sketch.point(a), shape_of(&sketch));

        sketch.set_dimension(DimensionTarget::Angle { first, second }, 60.0, false);
        assert_eq!(sketch.resolve(1.0), LengthOutcome::Exact);

        // The side hanging off the origin has not budged...
        assert!(
            sketch.point(a).distance(was_anchored) < 1e-6,
            "le côté ancré a bougé : {:?}",
            sketch.point(a)
        );
        // ...and the far side kept its shape, corner included, rather than
        // being bent to absorb the change.
        let now = shape_of(&sketch);
        for (before, after) in was_far.iter().zip(now.iter()) {
            assert!(
                (before - after).abs() < 1e-3 * before.abs().max(1.0),
                "la figure éloignée s'est déformée : {was_far:?} puis {now:?}"
            );
        }
        let angle = sketch.angle_between(first, second).unwrap();
        assert!((angle - 60.0).abs() < 1e-3, "angle obtenu : {angle}");
    }

    #[test]
    fn a_rectangle_does_not_turn_when_one_of_its_sides_changes() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let corner = Sketch::ORIGIN;
        let right = sketch.add_point(DVec2::new(100.0, 0.0));
        let far = sketch.add_point(DVec2::new(100.0, 50.0));
        let top = sketch.add_point(DVec2::new(0.0, 50.0));
        let sides = [
            sketch.add_segment(corner, right),
            sketch.add_segment(right, far),
            sketch.add_segment(far, top),
            sketch.add_segment(top, corner),
        ];
        for pair in sides.windows(2) {
            sketch.set_dimension(
                DimensionTarget::Angle {
                    first: pair[0],
                    second: pair[1],
                },
                90.0,
                false,
            );
        }
        sketch.set_dimension(DimensionTarget::Length(sides[0]), 100.0, false);
        sketch.set_dimension(DimensionTarget::Length(sides[1]), 50.0, false);
        sketch.resolve(1.0);
        assert!(sketch.is_fully_constrained(1.0));

        sketch.set_dimension(DimensionTarget::Length(sides[1]), 80.0, false);
        sketch.resolve(1.0);

        // Nothing holds the rectangle upright but the implicit rule that it
        // does not turn on its own, which is exactly what is being checked.
        let base = sketch.point(right) - sketch.point(corner);
        assert!(
            base.y.atan2(base.x).to_degrees().abs() < 0.05,
            "le rectangle s'est incliné : {base:?}"
        );
        assert!((sketch.point(far) - sketch.point(right)).length() - 80.0 < 0.05);
    }

    #[test]
    fn the_two_ways_of_naming_a_pair_are_one_target() {
        let first = DimensionTarget::Angle {
            first: SegmentId(3),
            second: SegmentId(1),
        };
        let second = DimensionTarget::Angle {
            first: SegmentId(1),
            second: SegmentId(3),
        };
        assert_eq!(first.normalised(), second.normalised());

        let there = DimensionTarget::Distance {
            from: PointId(5),
            to: PointId(2),
        };
        let back = DimensionTarget::Distance {
            from: PointId(2),
            to: PointId(5),
        };
        assert_eq!(there.normalised(), back.normalised());
    }

    /// A right-angled triangle hung off the sketch origin.
    fn triangle() -> (Sketch, [SegmentId; 3]) {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let corner = Sketch::ORIGIN;
        let right = sketch.add_point(DVec2::new(40.0, 0.0));
        let top = sketch.add_point(DVec2::new(0.0, 30.0));
        let base = sketch.add_segment(corner, right);
        let side = sketch.add_segment(corner, top);
        let hypotenuse = sketch.add_segment(right, top);
        (sketch, [base, side, hypotenuse])
    }

    /// The bug this solver exists for: setting a second value must not undo the
    /// first. Applying each dimension once and forgetting it could never do
    /// this.
    #[test]
    fn an_angle_still_holds_after_a_length_is_changed() {
        let (mut sketch, [base, side, _]) = triangle();

        sketch.set_dimension(
            DimensionTarget::Angle {
                first: base,
                second: side,
            },
            60.0,
            false,
        );
        sketch.resolve(1.0);
        assert!((sketch.angle_between(base, side).unwrap() - 60.0).abs() < 0.1);

        sketch.set_dimension(DimensionTarget::Length(base), 100.0, false);
        sketch.resolve(1.0);

        let angle = sketch.angle_between(base, side).unwrap();
        let length = sketch.segment_length(base);
        assert!((angle - 60.0).abs() < 0.1, "the angle drifted to {angle}°");
        assert!((length - 100.0).abs() < 0.1, "the length is {length}");
    }

    #[test]
    fn every_value_holds_at_once() {
        let (mut sketch, [base, side, _]) = triangle();
        sketch.set_dimension(DimensionTarget::Length(base), 50.0, false);
        sketch.set_dimension(DimensionTarget::Length(side), 20.0, false);
        sketch.set_dimension(
            DimensionTarget::Angle {
                first: base,
                second: side,
            },
            45.0,
            false,
        );

        assert_eq!(sketch.resolve(1.0), LengthOutcome::Exact);

        assert!((sketch.segment_length(base) - 50.0).abs() < 0.1);
        assert!((sketch.segment_length(side) - 20.0).abs() < 0.1);
        assert!((sketch.angle_between(base, side).unwrap() - 45.0).abs() < 0.1);
    }

    /// The case reported from the drawing: with two angles and two sides given,
    /// the third side follows and cannot be set independently.
    #[test]
    fn the_third_side_of_a_settled_triangle_is_redundant() {
        let (mut sketch, [base, side, hypotenuse]) = triangle();
        sketch.set_dimension(DimensionTarget::Length(base), 40.0, false);
        sketch.set_dimension(DimensionTarget::Length(side), 30.0, false);
        sketch.set_dimension(
            DimensionTarget::Angle {
                first: base,
                second: side,
            },
            90.0,
            false,
        );
        sketch.resolve(1.0);

        assert!(
            sketch.would_be_redundant(DimensionTarget::Length(hypotenuse), 1.0),
            "the hypotenuse follows from the two sides and their angle"
        );
    }

    /// Nothing is redundant while the shape can still change.
    #[test]
    fn a_side_of_an_open_shape_is_not_redundant() {
        let (sketch, [_, _, hypotenuse]) = triangle();
        assert!(!sketch.would_be_redundant(DimensionTarget::Length(hypotenuse), 1.0));
    }

    /// Pinning a point takes away the two ways a drawing can slide, never the
    /// way it can turn: that last freedom needs an angle to a fixed direction.
    #[test]
    fn a_drawing_needs_no_angle_to_the_axes_to_be_complete() {
        let (mut sketch, [base, side, _]) = triangle();
        sketch.set_dimension(DimensionTarget::Length(base), 40.0, false);
        sketch.set_dimension(DimensionTarget::Length(side), 30.0, false);
        sketch.set_dimension(
            DimensionTarget::Angle {
                first: base,
                second: side,
            },
            90.0,
            false,
        );
        sketch.resolve(1.0);

        // Which way up the drawing sits is implicit, like its origin point.
        assert_eq!(sketch.freedom(1.0).degrees_of_freedom, 0);
        assert!(sketch.is_fully_constrained(1.0));
    }

    /// Stating the orientation by hand must not take the same freedom twice,
    /// or a drawing free to slide would be reported as pinned.
    #[test]
    fn an_angle_to_an_axis_replaces_the_implicit_one() {
        let (mut sketch, [base, side, _]) = triangle();
        sketch.set_dimension(DimensionTarget::Length(base), 40.0, false);
        sketch.set_dimension(DimensionTarget::Length(side), 30.0, false);
        sketch.set_dimension(
            DimensionTarget::Angle {
                first: base,
                second: side,
            },
            90.0,
            false,
        );
        sketch.set_dimension(
            DimensionTarget::AxisAngle {
                segment: base,
                axis: SketchAxis::U,
            },
            0.0,
            false,
        );
        sketch.resolve(1.0);

        assert_eq!(sketch.freedom(1.0).degrees_of_freedom, 0);
    }

    /// One contour nailed down beside another still floating: the drawing has
    /// to say so per element, not give one verdict for the whole.
    #[test]
    fn part_of_a_drawing_can_be_settled_while_the_rest_floats() {
        let mut sketch = Sketch::new(WorkPlane::XY);

        let held = sketch.add_point(DVec2::new(20.0, 0.0));
        let fixed = sketch.add_segment(Sketch::ORIGIN, held);
        sketch.set_dimension(DimensionTarget::Length(fixed), 20.0, false);
        sketch.set_dimension(
            DimensionTarget::AxisAngle {
                segment: fixed,
                axis: SketchAxis::U,
            },
            0.0,
            false,
        );

        let loose_a = sketch.add_point(DVec2::new(50.0, 50.0));
        let loose_b = sketch.add_point(DVec2::new(70.0, 50.0));
        sketch.add_segment(loose_a, loose_b);

        let settled = sketch.settled_points(1.0);

        assert!(settled[Sketch::ORIGIN.0], "the origin never moves");
        assert!(settled[held.0], "held by a length and a direction");
        assert!(!settled[loose_a.0], "nothing holds this one");
        assert!(!settled[loose_b.0]);
        assert!(!sketch.is_fully_constrained(1.0));
    }

    /// A length from the origin is enough on its own: the drawing keeps the
    /// direction it was drawn in, so the far end has nowhere left to go.
    #[test]
    fn a_length_from_the_origin_settles_its_end() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let end = sketch.add_point(DVec2::new(20.0, 0.0));
        let segment = sketch.add_segment(Sketch::ORIGIN, end);
        sketch.set_dimension(DimensionTarget::Length(segment), 20.0, false);

        assert!(sketch.settled_points(1.0)[end.0]);
    }

    /// A shape drawn beside another must not stop it from being settled: each
    /// group of joined geometry keeps the direction it was drawn in on its own.
    #[test]
    fn a_loose_shape_beside_a_measured_one_leaves_it_settled() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let corner = sketch.add_point(DVec2::new(80.0, 0.0));
        let side = sketch.add_segment(Sketch::ORIGIN, corner);
        sketch.set_dimension(DimensionTarget::Length(side), 80.0, false);

        let loose_a = sketch.add_point(DVec2::new(200.0, 200.0));
        let loose_b = sketch.add_point(DVec2::new(260.0, 200.0));
        sketch.add_segment(loose_a, loose_b);

        let settled = sketch.settled_points(1.0);
        assert!(
            settled[corner.0],
            "held by a length and the way it was drawn"
        );
        assert!(!settled[loose_a.0]);
    }

    /// A length that hangs off nothing fixed still swings freely.
    #[test]
    fn a_length_away_from_the_origin_leaves_its_end_free() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let anchor = sketch.add_point(DVec2::new(30.0, 30.0));
        let end = sketch.add_point(DVec2::new(50.0, 30.0));
        let segment = sketch.add_segment(anchor, end);
        sketch.set_dimension(DimensionTarget::Length(segment), 20.0, false);

        assert!(!sketch.settled_points(1.0)[end.0]);
    }

    /// A drawing that touches nothing fixed can still slide about, however many
    /// values it carries.
    #[test]
    fn a_drawing_that_is_not_pinned_is_never_complete() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let a = sketch.add_point(DVec2::new(5.0, 5.0));
        let b = sketch.add_point(DVec2::new(15.0, 5.0));
        let segment = sketch.add_segment(a, b);
        sketch.set_dimension(DimensionTarget::Length(segment), 10.0, false);
        sketch.set_dimension(
            DimensionTarget::AxisAngle {
                segment,
                axis: SketchAxis::U,
            },
            0.0,
            false,
        );

        assert!(!sketch.is_fully_constrained(1.0));
        assert_eq!(sketch.freedom(1.0).degrees_of_freedom, 2, "free to slide");
    }

    /// A readout says nothing about the shape, so it must not remove freedom.
    #[test]
    fn a_driven_dimension_constrains_nothing() {
        let (mut sketch, [base, _, _]) = triangle();
        let before = sketch.freedom(1.0).degrees_of_freedom;
        sketch.set_dimension(DimensionTarget::Length(base), 100.0, true);
        assert_eq!(sketch.freedom(1.0).degrees_of_freedom, before);
    }

    #[test]
    fn an_angle_needs_two_segments_that_meet() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let a = sketch.add_point(DVec2::ZERO);
        let b = sketch.add_point(DVec2::new(10.0, 0.0));
        let c = sketch.add_point(DVec2::new(0.0, 5.0));
        let d = sketch.add_point(DVec2::new(5.0, 5.0));
        let first = sketch.add_segment(a, b);
        let apart = sketch.add_segment(c, d);

        assert!(sketch.angle_between(first, apart).is_none());
    }

    #[test]
    fn an_axis_angle_measures_from_the_sketch_direction() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let a = sketch.add_point(DVec2::ZERO);
        let b = sketch.add_point(DVec2::new(10.0, 10.0));
        let segment = sketch.add_segment(a, b);

        let angle = sketch.angle_with_axis(segment, SketchAxis::U).unwrap();
        assert!((angle - 45.0).abs() < 1e-3, "got {angle}°");
    }

    /// Contradictory values cannot both be met, and the solver has to say so
    /// rather than quietly settling on one of them.
    #[test]
    fn impossible_values_are_reported() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let a = sketch.add_point(DVec2::ZERO);
        let b = sketch.add_point(DVec2::new(10.0, 0.0));
        let first = sketch.add_segment(a, b);
        let second = sketch.add_segment(a, b);

        sketch.set_dimension(DimensionTarget::Length(first), 50.0, false);
        sketch.set_dimension(DimensionTarget::Length(second), 90.0, false);

        assert_eq!(sketch.resolve(1.0), LengthOutcome::BestEffort);
    }

    #[test]
    fn a_line_pulls_along_its_whole_body() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let end = sketch.add_point(DVec2::new(100.0, 0.0));
        let side = sketch.add_segment(Sketch::ORIGIN, end);

        let (found, at) = sketch
            .nearest_on_segment(DVec2::new(30.0, 2.0), 5.0)
            .expect("le trait attire");
        assert_eq!(found, side);
        assert!(at.distance(DVec2::new(30.0, 0.0)) < 1e-4, "{at:?}");

        assert_eq!(sketch.nearest_on_segment(DVec2::new(30.0, 40.0), 5.0), None);
        // Past the end, the pull stops at the end rather than off in space.
        let (_, beyond) = sketch
            .nearest_on_segment(DVec2::new(104.0, 0.0), 5.0)
            .expect("le bout attire encore");
        assert!(beyond.distance(DVec2::new(100.0, 0.0)) < 1e-4);
    }

    #[test]
    fn the_middle_of_a_line_is_its_own_catch() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let end = sketch.add_point(DVec2::new(100.0, 0.0));
        let side = sketch.add_segment(Sketch::ORIGIN, end);

        let (found, at) = sketch
            .nearest_midpoint(DVec2::new(48.0, 3.0), 5.0)
            .expect("le milieu attire");
        assert_eq!(found, side);
        assert_eq!(at, DVec2::new(50.0, 0.0));
        assert_eq!(sketch.nearest_midpoint(DVec2::new(20.0, 0.0), 5.0), None);
    }

    /// Two ends laid on top of each other are one corner, not two.
    #[test]
    fn merging_two_points_joins_what_they_held() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let left = sketch.add_point(DVec2::new(-10.0, 0.0));
        let meeting = sketch.add_point(DVec2::new(0.0, 10.0));
        let twin = sketch.add_point(DVec2::new(0.0, 10.0));
        let right = sketch.add_point(DVec2::new(10.0, 0.0));
        let first = sketch.add_segment(left, meeting);
        let second = sketch.add_segment(twin, right);

        sketch.merge_points(meeting, twin);

        assert!(sketch.is_erased_point(twin));
        assert_eq!(sketch.segments()[second.0].start, meeting);
        assert_eq!(sketch.segments()[first.0].end, meeting);
        assert_eq!(sketch.live_segments().count(), 2, "les deux traits restent");
    }

    /// A segment whose two ends became one has no length and no direction.
    #[test]
    fn merging_the_ends_of_a_line_takes_the_line() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let a = sketch.add_point(DVec2::new(0.0, 0.0));
        let b = sketch.add_point(DVec2::new(1.0, 0.0));
        let short = sketch.add_segment(a, b);

        sketch.merge_points(a, b);
        assert!(sketch.is_erased_segment(short));
    }

    /// The origin is never the one that gives way.
    #[test]
    fn merging_onto_the_origin_keeps_the_origin() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let stray = sketch.add_point(DVec2::ZERO);
        let far = sketch.add_point(DVec2::new(10.0, 0.0));
        sketch.add_segment(stray, far);

        sketch.merge_points(stray, Sketch::ORIGIN);

        assert!(!sketch.is_erased_point(Sketch::ORIGIN));
        assert!(sketch.is_erased_point(stray));
        assert_eq!(sketch.segments()[0].start, Sketch::ORIGIN);
    }

    #[test]
    fn merging_moves_a_dimension_onto_the_point_that_stays() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let kept = sketch.add_point(DVec2::new(10.0, 0.0));
        let twin = sketch.add_point(DVec2::new(10.0, 0.0));
        let far = sketch.add_point(DVec2::new(10.0, 20.0));
        sketch.add_segment(twin, far);
        sketch.set_dimension(
            DimensionTarget::Distance {
                from: Sketch::ORIGIN,
                to: twin,
            },
            10.0,
            false,
        );

        sketch.merge_points(kept, twin);

        assert!(
            sketch
                .dimension_of(DimensionTarget::Distance {
                    from: Sketch::ORIGIN,
                    to: kept,
                })
                .is_some(),
            "la cote suit le point conservé"
        );
    }

    /// Deleting must not shift the rank of what stays: a dimension already
    /// recorded against a segment would then measure another one.
    #[test]
    fn erasing_a_segment_leaves_the_others_where_they_were() {
        let (mut sketch, [base, side, third]) = triangle();
        sketch.set_dimension(DimensionTarget::Length(third), 40.0, false);

        sketch.erase(Element::Segment(base));

        assert!(sketch.is_erased_segment(base));
        assert!(!sketch.is_erased_segment(side));
        assert_eq!(sketch.live_segments().count(), 2);
        assert!(
            sketch
                .dimension_of(DimensionTarget::Length(third))
                .is_some(),
            "la cote du troisième côté est intacte"
        );
        assert_eq!(sketch.nearest_segment(DVec2::new(50.0, 0.0), 1.0), None);
    }

    /// A dimension measuring something deleted would report on nothing.
    #[test]
    fn erasing_takes_the_dimensions_that_measured_it() {
        let (mut sketch, [base, _, _]) = triangle();
        sketch.set_dimension(DimensionTarget::Length(base), 40.0, false);
        assert_eq!(sketch.dimensions().len(), 1);

        sketch.erase(Element::Segment(base));
        assert!(sketch.dimensions().is_empty());
    }

    /// A segment without its point is not geometry.
    #[test]
    fn erasing_a_point_takes_what_leaned_on_it() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let corner = sketch.add_point(DVec2::new(10.0, 0.0));
        let far = sketch.add_point(DVec2::new(10.0, 10.0));
        let touching = sketch.add_segment(Sketch::ORIGIN, corner);
        let apart = sketch.add_segment(corner, far);
        sketch.add_circle(corner, 3.0);

        sketch.erase(Element::Point(corner));

        assert!(sketch.is_erased_segment(touching));
        assert!(sketch.is_erased_segment(apart));
        assert_eq!(sketch.live_circles().count(), 0);
        assert!(!sketch.is_erased_point(far), "le point d'en face reste");
    }

    /// The origin is what everything else is measured from.
    #[test]
    fn the_origin_cannot_be_erased() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        sketch.erase(Element::Point(Sketch::ORIGIN));
        assert!(!sketch.is_erased_point(Sketch::ORIGIN));
    }

    #[test]
    fn an_erased_shape_no_longer_encloses_an_area() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let corners = [
            sketch.add_point(DVec2::ZERO),
            sketch.add_point(DVec2::new(10.0, 0.0)),
            sketch.add_point(DVec2::new(10.0, 10.0)),
            sketch.add_point(DVec2::new(0.0, 10.0)),
        ];
        let mut sides = Vec::new();
        for index in 0..4 {
            sides.push(sketch.add_segment(corners[index], corners[(index + 1) % 4]));
        }
        assert_eq!(sketch.regions().len(), 1);

        sketch.erase(Element::Segment(sides[0]));
        assert!(sketch.regions().is_empty());
    }

    #[test]
    fn clicking_back_onto_a_corner_reuses_it() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let first = sketch.point_at(DVec2::new(5.0, 5.0), 0.5);
        let again = sketch.point_at(DVec2::new(5.2, 5.1), 0.5);
        let elsewhere = sketch.point_at(DVec2::new(40.0, 5.0), 0.5);

        assert_eq!(first, again);
        assert_ne!(first, elsewhere);
        assert_eq!(sketch.points().len(), 3, "the origin plus the two placed");
    }

    /// Clicking where the origin sits must join it rather than lay a second
    /// point on top: that is how a drawing gets pinned without thinking about
    /// it.
    #[test]
    fn clicking_the_origin_joins_it() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        assert_eq!(
            sketch.point_at(DVec2::new(0.05, -0.05), 0.5),
            Sketch::ORIGIN
        );
        assert_eq!(sketch.points().len(), 1);
    }

    #[test]
    fn the_origin_never_moves() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        sketch.move_point(Sketch::ORIGIN, DVec2::new(10.0, 10.0));
        assert_eq!(sketch.point(Sketch::ORIGIN), DVec2::ZERO);
    }

    /// A distance can be measured between any two points, joined or not, which
    /// is what lets a shape be positioned from the origin.
    #[test]
    fn a_distance_pins_a_point_against_the_origin() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let free = sketch.add_point(DVec2::new(3.0, 4.0));

        sketch.set_dimension(
            DimensionTarget::Distance {
                from: Sketch::ORIGIN,
                to: free,
            },
            10.0,
            false,
        );
        assert_eq!(sketch.resolve(1.0), LengthOutcome::Exact);

        let distance = sketch.point(free).length();
        assert!((distance - 10.0).abs() < 0.01, "got {distance}");
        assert_eq!(sketch.point(Sketch::ORIGIN), DVec2::ZERO);
    }

    #[test]
    fn the_nearest_segment_is_found_along_its_body() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let a = sketch.add_point(DVec2::ZERO);
        let b = sketch.add_point(DVec2::new(100.0, 0.0));
        let segment = sketch.add_segment(a, b);

        assert_eq!(
            sketch.nearest_segment(DVec2::new(50.0, 2.0), 5.0),
            Some(segment)
        );
        assert_eq!(sketch.nearest_segment(DVec2::new(50.0, 40.0), 5.0), None);
        assert_eq!(sketch.nearest_segment(DVec2::new(150.0, 0.0), 5.0), None);
    }

    #[test]
    fn a_circle_is_found_by_its_outline() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let center = sketch.add_point(DVec2::ZERO);
        let circle = sketch.add_circle(center, 10.0);

        assert_eq!(
            sketch.nearest_circle(DVec2::new(10.2, 0.0), 1.0),
            Some(circle)
        );
        assert_eq!(
            sketch.nearest_circle(DVec2::ZERO, 1.0),
            None,
            "not the middle"
        );
        assert_eq!(sketch.nearest_circle(DVec2::new(30.0, 0.0), 1.0), None);
    }

    #[test]
    fn bounds_cover_every_point() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        sketch.add_point(DVec2::new(-3.0, 7.0));
        sketch.add_point(DVec2::new(12.0, -1.0));
        let (min, max) = sketch.bounds().expect("some points");
        // The origin is a point like any other as far as framing goes.
        assert_eq!(min, DVec2::new(-3.0, -1.0));
        assert_eq!(max, DVec2::new(12.0, 7.0));
    }
}
