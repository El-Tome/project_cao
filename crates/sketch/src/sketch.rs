use glam::Vec2;
use serde::{Deserialize, Serialize};

use crate::constraints::{Components, Dimension, DimensionTarget, Freedom, is_anchor};
use crate::plane::WorkPlane;

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
    pub radius: f32,
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
    points: Vec<Vec2>,
    segments: Vec<Segment>,
    #[serde(default)]
    circles: Vec<Circle>,
    dimensions: Vec<Dimension>,
}

impl Sketch {
    pub fn new(plane: WorkPlane) -> Self {
        Self {
            plane,
            points: Vec::new(),
            segments: Vec::new(),
            circles: Vec::new(),
            dimensions: Vec::new(),
        }
    }

    pub fn points(&self) -> &[Vec2] {
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

    pub fn add_circle(&mut self, center: PointId, radius: f32) -> CircleId {
        self.circles.push(Circle { center, radius });
        CircleId(self.circles.len() - 1)
    }

    /// The circle whose outline passes closest to `position`.
    pub fn nearest_circle(&self, position: Vec2, tolerance: f32) -> Option<CircleId> {
        self.circles
            .iter()
            .enumerate()
            .map(|(index, circle)| {
                let distance = (self.point(circle.center).distance(position) - circle.radius).abs();
                (CircleId(index), distance)
            })
            .filter(|(_, distance)| *distance <= tolerance)
            .min_by(|a, b| a.1.total_cmp(&b.1))
            .map(|(id, _)| id)
    }

    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }

    pub fn point(&self, id: PointId) -> Vec2 {
        self.points[id.0]
    }

    pub fn endpoints(&self, id: SegmentId) -> (Vec2, Vec2) {
        let segment = self.segments[id.0];
        (self.point(segment.start), self.point(segment.end))
    }

    pub fn add_point(&mut self, position: Vec2) -> PointId {
        self.points.push(position);
        PointId(self.points.len() - 1)
    }

    /// Reuses an existing point when one is within `tolerance`, so that clicking
    /// back onto a corner joins the geometry there instead of laying a second
    /// point on top of it.
    pub fn point_at(&mut self, position: Vec2, tolerance: f32) -> PointId {
        match self.nearest_point(position, tolerance) {
            Some(id) => id,
            None => self.add_point(position),
        }
    }

    pub fn nearest_point(&self, position: Vec2, tolerance: f32) -> Option<PointId> {
        self.points
            .iter()
            .enumerate()
            .map(|(index, point)| (PointId(index), point.distance(position)))
            .filter(|(_, distance)| *distance <= tolerance)
            .min_by(|a, b| a.1.total_cmp(&b.1))
            .map(|(id, _)| id)
    }

    /// The segment whose body passes closest to `position`, within `tolerance`.
    pub fn nearest_segment(&self, position: Vec2, tolerance: f32) -> Option<SegmentId> {
        (0..self.segments.len())
            .map(|index| {
                let id = SegmentId(index);
                (id, self.distance_to_segment(id, position))
            })
            .filter(|(_, distance)| *distance <= tolerance)
            .min_by(|a, b| a.1.total_cmp(&b.1))
            .map(|(id, _)| id)
    }

    fn distance_to_segment(&self, id: SegmentId, position: Vec2) -> f32 {
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

    pub fn segment_length(&self, id: SegmentId) -> f32 {
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
    pub fn set_dimension(&mut self, target: DimensionTarget, value: f32, driven: bool) {
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
            }),
        }
    }

    /// The angle at the point two segments share, in degrees, or `None` when
    /// they do not meet.
    pub fn angle_between(&self, first: SegmentId, second: SegmentId) -> Option<f32> {
        let (pivot, a, b) = self.corner(first, second)?;
        let first = (self.point(a) - self.point(pivot)).normalize_or_zero();
        let second = (self.point(b) - self.point(pivot)).normalize_or_zero();
        if first == Vec2::ZERO || second == Vec2::ZERO {
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

    /// Turns `second` about the point it shares with `first` until the angle
    /// between them is `degrees`, taking whatever hangs off its far end with
    /// it. Same spirit as a length: one side is the anchor, the other gives.
    pub fn set_angle(
        &mut self,
        first: SegmentId,
        second: SegmentId,
        degrees: f32,
    ) -> LengthOutcome {
        let Some((pivot, far_first, far_second)) = self.corner(first, second) else {
            return LengthOutcome::Degenerate;
        };

        let center = self.point(pivot);
        let toward_first = (self.point(far_first) - center).normalize_or_zero();
        let toward_second = (self.point(far_second) - center).normalize_or_zero();
        if toward_first == Vec2::ZERO || toward_second == Vec2::ZERO {
            return LengthOutcome::Degenerate;
        }

        // Signed so the angle keeps opening the same way: asking for 30° on a
        // corner that turns clockwise must not flip it over to the other side.
        let current = toward_first
            .perp_dot(toward_second)
            .atan2(toward_first.dot(toward_second));
        let target = degrees.to_radians() * if current < 0.0 { -1.0 } else { 1.0 };
        let rotation = target - current;

        let group = self.connected_from(far_second, second);
        if group.contains(&pivot) {
            self.points[far_second.0] = rotate_about(self.points[far_second.0], center, rotation);
            return LengthOutcome::BestEffort;
        }
        for point in group {
            self.points[point.0] = rotate_about(self.points[point.0], center, rotation);
        }
        LengthOutcome::Exact
    }

    pub fn set_circle_radius(&mut self, id: CircleId, radius: f32) -> LengthOutcome {
        if radius <= 0.0 {
            return LengthOutcome::Degenerate;
        }
        self.circles[id.0].radius = radius;
        LengthOutcome::Exact
    }

    /// Stretches a segment to `target` world units.
    ///
    /// The start of the segment is the anchor and never moves; the far end
    /// slides along the segment's own direction, dragging along everything that
    /// hangs off it. If the geometry loops back round to the anchor, there is
    /// no rigid move that satisfies the length, so only the far end is moved.
    pub fn set_segment_length(&mut self, id: SegmentId, target: f32) -> LengthOutcome {
        let segment = self.segments[id.0];
        let (start, end) = self.endpoints(id);
        let span = end - start;
        let current = span.length();

        if current < 1e-6 || target <= 0.0 {
            return LengthOutcome::Degenerate;
        }

        let shift = span / current * (target - current);
        let group = self.connected_from(segment.end, id);

        if group.contains(&segment.start) {
            self.points[segment.end.0] += shift;
            return LengthOutcome::BestEffort;
        }

        for point in group {
            self.points[point.0] += shift;
        }
        LengthOutcome::Exact
    }

    /// Every point reachable from `from` by walking segments, ignoring the one
    /// being resized: that is exactly the part of the drawing that should
    /// follow the moving end.
    fn connected_from(&self, from: PointId, ignored: SegmentId) -> Vec<PointId> {
        let mut reached = vec![from];
        let mut frontier = vec![from];

        while let Some(point) = frontier.pop() {
            for (index, segment) in self.segments.iter().enumerate() {
                if SegmentId(index) == ignored {
                    continue;
                }
                let next = if segment.start == point {
                    segment.end
                } else if segment.end == point {
                    segment.start
                } else {
                    continue;
                };
                if !reached.contains(&next) {
                    reached.push(next);
                    frontier.push(next);
                }
            }
        }

        reached
    }

    /// Smallest axis-aligned box containing every point, in sketch coordinates.
    pub fn bounds(&self) -> Option<(Vec2, Vec2)> {
        let first = *self.points.first()?;
        Some(
            self.points
                .iter()
                .fold((first, first), |(min, max), point| {
                    (min.min(*point), max.max(*point))
                }),
        )
    }
}

fn rotate_about(point: Vec2, center: Vec2, radians: f32) -> Vec2 {
    let offset = point - center;
    let (sin, cos) = radians.sin_cos();
    center
        + Vec2::new(
            offset.x * cos - offset.y * sin,
            offset.x * sin + offset.y * cos,
        )
}

impl Sketch {
    /// Which independent piece of the drawing each point belongs to. Points
    /// joined by a segment move together, so they share a piece.
    fn components(&self) -> Vec<usize> {
        let mut sets = Components::new(self.points.len().max(1));
        for segment in &self.segments {
            sets.union(segment.start.0, segment.end.0);
        }
        (0..self.points.len())
            .map(|index| sets.find(index))
            .collect()
    }

    /// How much freedom each piece of the drawing still has.
    ///
    /// Every point is two unknowns, every circle adds its radius, every
    /// dimension that drives the geometry removes one, and a point pinned to
    /// the sketch origin removes two. A piece with nothing left is what the
    /// user sees as fully constrained.
    pub fn freedom_by_component(&self) -> Vec<Freedom> {
        let components = self.components();
        let mut counts = vec![0i32; self.points.len().max(1)];

        for (index, root) in components.iter().enumerate() {
            counts[*root] += 2;
            if is_anchor(self.points[index]) {
                counts[*root] -= 2;
            }
        }
        for circle in &self.circles {
            counts[components[circle.center.0]] += 1;
        }
        for dimension in &self.dimensions {
            if dimension.driven {
                continue;
            }
            if let Some(root) = self.component_of_target(dimension.target, &components) {
                counts[root] -= 1;
            }
        }

        counts
            .into_iter()
            .map(|degrees_of_freedom| Freedom { degrees_of_freedom })
            .collect()
    }

    fn component_of_target(&self, target: DimensionTarget, components: &[usize]) -> Option<usize> {
        let point = match target {
            DimensionTarget::Length(segment) => self.segments.get(segment.0)?.start,
            DimensionTarget::Angle { first, .. } => self.segments.get(first.0)?.start,
            DimensionTarget::Radius(circle) => self.circles.get(circle.0)?.center,
        };
        components.get(point.0).copied()
    }

    /// Whether the piece of drawing a point belongs to has any freedom left.
    pub fn point_is_constrained(&self, point: PointId) -> bool {
        let components = self.components();
        let freedom = self.freedom_by_component();
        components
            .get(point.0)
            .and_then(|root| freedom.get(*root))
            .is_some_and(|freedom| freedom.fully_constrained())
    }

    pub fn segment_is_constrained(&self, segment: SegmentId) -> bool {
        self.segments
            .get(segment.0)
            .is_some_and(|segment| self.point_is_constrained(segment.start))
    }

    /// True when the drawing has nothing left to determine.
    pub fn is_fully_constrained(&self) -> bool {
        !self.points.is_empty()
            && self
                .freedom_by_component()
                .iter()
                .enumerate()
                .filter(|(root, _)| self.components().contains(root))
                .all(|(_, freedom)| freedom.fully_constrained())
    }

    /// Whether a new dimension on this target would add nothing: its piece of
    /// the drawing is already fully determined. Such a dimension is still worth
    /// placing to read the value, but it must not drive anything.
    pub fn would_be_redundant(&self, target: DimensionTarget) -> bool {
        if self.dimension_of(target).is_some() {
            return false;
        }
        let components = self.components();
        let freedom = self.freedom_by_component();
        self.component_of_target(target, &components)
            .and_then(|root| freedom.get(root))
            .is_some_and(|freedom| freedom.fully_constrained())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Two segments in a chain: fixing the length of the first must slide the
    /// second along, keeping its own length and direction.
    #[test]
    fn a_dimension_drags_the_rest_of_the_chain() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let a = sketch.add_point(Vec2::ZERO);
        let b = sketch.add_point(Vec2::new(10.0, 0.0));
        let c = sketch.add_point(Vec2::new(10.0, 4.0));
        let first = sketch.add_segment(a, b);
        let second = sketch.add_segment(b, c);

        assert_eq!(sketch.set_segment_length(first, 25.0), LengthOutcome::Exact);

        assert!((sketch.segment_length(first) - 25.0).abs() < 1e-4);
        assert_eq!(sketch.point(a), Vec2::ZERO, "the anchor must not move");
        assert!((sketch.point(b) - Vec2::new(25.0, 0.0)).length() < 1e-4);
        assert!(
            (sketch.point(c) - Vec2::new(25.0, 4.0)).length() < 1e-4,
            "the far segment should travel rigidly"
        );
        assert!((sketch.segment_length(second) - 4.0).abs() < 1e-4);
    }

    /// A closed shape cannot absorb a rigid move: the far end alone gives way,
    /// and we say so rather than pretending the length was satisfied cleanly.
    #[test]
    fn a_closed_shape_is_stretched_best_effort() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let a = sketch.add_point(Vec2::ZERO);
        let b = sketch.add_point(Vec2::new(10.0, 0.0));
        let c = sketch.add_point(Vec2::new(10.0, 10.0));
        let d = sketch.add_point(Vec2::new(0.0, 10.0));
        let bottom = sketch.add_segment(a, b);
        sketch.add_segment(b, c);
        sketch.add_segment(c, d);
        sketch.add_segment(d, a);

        assert_eq!(
            sketch.set_segment_length(bottom, 30.0),
            LengthOutcome::BestEffort
        );
        assert!((sketch.segment_length(bottom) - 30.0).abs() < 1e-4);
        assert_eq!(sketch.point(a), Vec2::ZERO);
        assert!((sketch.point(b) - Vec2::new(30.0, 0.0)).length() < 1e-4);
        assert_eq!(sketch.point(c), Vec2::new(10.0, 10.0), "the far side stays");
    }

    #[test]
    fn a_lone_segment_just_stretches() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let a = sketch.add_point(Vec2::ZERO);
        let b = sketch.add_point(Vec2::new(3.0, 4.0));
        let segment = sketch.add_segment(a, b);

        assert_eq!(
            sketch.set_segment_length(segment, 10.0),
            LengthOutcome::Exact
        );
        assert!((sketch.segment_length(segment) - 10.0).abs() < 1e-4);
        // Direction preserved: the 3-4-5 triangle scales to 6-8-10.
        assert!((sketch.point(b) - Vec2::new(6.0, 8.0)).length() < 1e-4);
    }

    #[test]
    fn a_zero_length_segment_cannot_be_stretched() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let a = sketch.add_point(Vec2::ZERO);
        let b = sketch.add_point(Vec2::ZERO);
        let segment = sketch.add_segment(a, b);
        assert_eq!(
            sketch.set_segment_length(segment, 10.0),
            LengthOutcome::Degenerate
        );
    }

    #[test]
    fn clicking_back_onto_a_corner_reuses_it() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let first = sketch.point_at(Vec2::new(5.0, 5.0), 0.5);
        let again = sketch.point_at(Vec2::new(5.2, 5.1), 0.5);
        let elsewhere = sketch.point_at(Vec2::new(40.0, 5.0), 0.5);

        assert_eq!(first, again);
        assert_ne!(first, elsewhere);
        assert_eq!(sketch.points().len(), 2);
    }

    /// A right angle asked to become 30° must actually measure 30°, and take
    /// the rest of the chain with it.
    #[test]
    fn an_angle_turns_the_second_segment() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let corner = sketch.add_point(Vec2::ZERO);
        let right = sketch.add_point(Vec2::new(10.0, 0.0));
        let up = sketch.add_point(Vec2::new(0.0, 10.0));
        let tip = sketch.add_point(Vec2::new(0.0, 14.0));
        let first = sketch.add_segment(corner, right);
        let second = sketch.add_segment(corner, up);
        sketch.add_segment(up, tip);

        assert!((sketch.angle_between(first, second).unwrap() - 90.0).abs() < 1e-3);
        assert_eq!(sketch.set_angle(first, second, 30.0), LengthOutcome::Exact);

        let measured = sketch.angle_between(first, second).unwrap();
        assert!((measured - 30.0).abs() < 1e-2, "got {measured}°");
        // The far segment travelled with it and kept its own length.
        assert!((sketch.point(up).distance(sketch.point(tip)) - 4.0).abs() < 1e-3);
    }

    #[test]
    fn an_angle_needs_two_segments_that_meet() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let a = sketch.add_point(Vec2::ZERO);
        let b = sketch.add_point(Vec2::new(10.0, 0.0));
        let c = sketch.add_point(Vec2::new(0.0, 5.0));
        let d = sketch.add_point(Vec2::new(5.0, 5.0));
        let first = sketch.add_segment(a, b);
        let apart = sketch.add_segment(c, d);

        assert!(sketch.angle_between(first, apart).is_none());
        assert_eq!(
            sketch.set_angle(first, apart, 45.0),
            LengthOutcome::Degenerate
        );
    }

    /// A lone segment has 4 unknowns; its length removes one, pinning a point
    /// to the origin removes two, and the last one is its direction.
    #[test]
    fn freedom_is_counted_down_as_constraints_are_added() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let anchor = sketch.add_point(Vec2::ZERO);
        let far = sketch.add_point(Vec2::new(10.0, 0.0));
        let segment = sketch.add_segment(anchor, far);

        let freedom = |sketch: &Sketch| sketch.freedom_by_component()[0].degrees_of_freedom;
        // 2 points = 4, minus 2 for the anchored one.
        assert_eq!(freedom(&sketch), 2);

        sketch.set_dimension(DimensionTarget::Length(segment), 100.0, false);
        assert_eq!(freedom(&sketch), 1);
        assert!(!sketch.is_fully_constrained());

        let other = sketch.add_point(Vec2::new(10.0, 10.0));
        let second = sketch.add_segment(far, other);
        sketch.set_dimension(DimensionTarget::Length(second), 50.0, false);
        sketch.set_dimension(
            DimensionTarget::Angle {
                first: segment,
                second,
            },
            90.0,
            false,
        );
        // 3 points = 6, minus 2 anchored, minus 3 dimensions = 1 left: the
        // direction of the first segment.
        assert_eq!(freedom(&sketch), 1);
    }

    /// A driven dimension is only a readout, so it must not count as removing
    /// any freedom.
    #[test]
    fn a_driven_dimension_constrains_nothing() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let anchor = sketch.add_point(Vec2::ZERO);
        let far = sketch.add_point(Vec2::new(10.0, 0.0));
        let segment = sketch.add_segment(anchor, far);

        sketch.set_dimension(DimensionTarget::Length(segment), 100.0, true);
        assert_eq!(sketch.freedom_by_component()[0].degrees_of_freedom, 2);
    }

    /// Once a piece of the drawing has nothing left to determine, a further
    /// dimension on it adds nothing and must be flagged.
    #[test]
    fn a_dimension_on_a_settled_shape_is_redundant() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let anchor = sketch.add_point(Vec2::ZERO);
        let far = sketch.add_point(Vec2::new(10.0, 0.0));
        let segment = sketch.add_segment(anchor, far);
        let circle = sketch.add_circle(anchor, 5.0);

        assert!(!sketch.would_be_redundant(DimensionTarget::Length(segment)));

        // Take away the last two freedoms: the segment length and the radius.
        sketch.set_dimension(DimensionTarget::Length(segment), 100.0, false);
        sketch.set_dimension(DimensionTarget::Radius(circle), 20.0, false);
        // 2 points(4) + radius(1) - anchor(2) - 2 dimensions = 1 left.
        let second = sketch.add_segment(anchor, far);
        sketch.set_dimension(DimensionTarget::Length(second), 100.0, false);

        assert!(sketch.is_fully_constrained());
        let extra = sketch.add_segment(far, anchor);
        assert!(sketch.would_be_redundant(DimensionTarget::Length(extra)));
    }

    #[test]
    fn a_circle_is_found_by_its_outline() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let center = sketch.add_point(Vec2::ZERO);
        let circle = sketch.add_circle(center, 10.0);

        assert_eq!(
            sketch.nearest_circle(Vec2::new(10.2, 0.0), 1.0),
            Some(circle)
        );
        assert_eq!(
            sketch.nearest_circle(Vec2::ZERO, 1.0),
            None,
            "not the middle"
        );
        assert_eq!(sketch.nearest_circle(Vec2::new(30.0, 0.0), 1.0), None);
    }

    #[test]
    fn the_nearest_segment_is_found_along_its_body() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let a = sketch.add_point(Vec2::ZERO);
        let b = sketch.add_point(Vec2::new(100.0, 0.0));
        let segment = sketch.add_segment(a, b);

        assert_eq!(
            sketch.nearest_segment(Vec2::new(50.0, 2.0), 5.0),
            Some(segment)
        );
        assert_eq!(sketch.nearest_segment(Vec2::new(50.0, 40.0), 5.0), None);
        assert_eq!(sketch.nearest_segment(Vec2::new(150.0, 0.0), 5.0), None);
    }

    #[test]
    fn bounds_cover_every_point() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        assert!(sketch.bounds().is_none());
        sketch.add_point(Vec2::new(-3.0, 7.0));
        sketch.add_point(Vec2::new(12.0, -1.0));
        let (min, max) = sketch.bounds().expect("two points");
        assert_eq!(min, Vec2::new(-3.0, -1.0));
        assert_eq!(max, Vec2::new(12.0, 7.0));
    }
}
