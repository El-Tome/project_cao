//! What of a drawing stands nearest a place.
//!
//! The cursor is a place, and every tool starts by asking what is under it: a
//! point to join, a line to lean on, the middle of a trait, a dimension to
//! drag. One answer per kind, each within a tolerance the view decides — a
//! line pulls from further away when the drawing is zoomed out.
//!
//! What a click then *takes hold of* is `picking.rs`; this is what it had to
//! find first.

use glam::DVec2;

use crate::annotation::AnnotationMetrics;
use crate::constraints::DimensionTarget;
use crate::sketch::{CircleId, PointId, SegmentId, Sketch};

impl Sketch {
    /// The circle whose outline passes closest to `position`.
    pub fn nearest_circle(&self, position: DVec2, tolerance: f64) -> Option<CircleId> {
        self.circles()
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

    /// Reuses an existing point when one is within `tolerance`, so that clicking back onto a corner
    /// joins the geometry there instead of laying a second point on top of it.
    pub fn point_at(&mut self, position: DVec2, tolerance: f64) -> PointId {
        match self.nearest_point(position, tolerance) {
            Some(id) => id,
            None => self.add_point(position),
        }
    }

    pub fn nearest_point(&self, position: DVec2, tolerance: f64) -> Option<PointId> {
        self.points()
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
        (0..self.segments().len())
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
        (0..self.segments().len())
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
}

#[cfg(test)]
mod tests;
