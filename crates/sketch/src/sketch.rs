use glam::Vec2;
use serde::{Deserialize, Serialize};

use crate::plane::WorkPlane;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PointId(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SegmentId(pub usize);

/// A straight line between two points. Points are shared: chaining a polyline
/// reuses the previous end, which is what makes a dimension able to drag the
/// rest of the chain along.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Segment {
    pub start: PointId,
    pub end: PointId,
}

/// A length the user has fixed on a segment, kept in millimetres — the unit
/// they typed, independent of the document scale.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Dimension {
    pub segment: SegmentId,
    pub millimeters: f32,
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
    dimensions: Vec<Dimension>,
}

impl Sketch {
    pub fn new(plane: WorkPlane) -> Self {
        Self {
            plane,
            points: Vec::new(),
            segments: Vec::new(),
            dimensions: Vec::new(),
        }
    }

    pub fn points(&self) -> &[Vec2] {
        &self.points
    }

    pub fn segments(&self) -> &[Segment] {
        &self.segments
    }

    pub fn dimensions(&self) -> &[Dimension] {
        &self.dimensions
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

    pub fn dimension_of(&self, id: SegmentId) -> Option<&Dimension> {
        self.dimensions
            .iter()
            .find(|dimension| dimension.segment == id)
    }

    /// Records the length the user typed on a segment, replacing any previous
    /// one. Moving the geometry is a separate step: the very first dimension of
    /// a document sets its scale instead of resizing anything.
    pub fn set_dimension(&mut self, segment: SegmentId, millimeters: f32) {
        match self
            .dimensions
            .iter_mut()
            .find(|dimension| dimension.segment == segment)
        {
            Some(existing) => existing.millimeters = millimeters,
            None => self.dimensions.push(Dimension {
                segment,
                millimeters,
            }),
        }
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
