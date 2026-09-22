//! An ellipse, as a centre and the two axes it is laid with.

use glam::DVec2;
use serde::{Deserialize, Serialize};

use crate::ellipsing::EllipseDraft;
use crate::erased::Erased;
use crate::sketch::{Element, PointId, SegmentId, Sketch};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct EllipseId(pub usize);

/// An ellipse, held by its centre and its two axes.
///
/// The axes are ordinary traits of the drawing, laid as construction, each
/// running across the whole curve through the centre — as on a drawing. They
/// are what the ellipse is dimensioned by: a length on an axis is its width
/// that way, as a diameter is a circle's. Their four ends are the handles it is
/// turned and stretched by.
///
/// Nothing else is kept. The size, the direction and the way round are all
/// read off the axes, so none of them can disagree with what is drawn.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Ellipse {
    pub center: PointId,
    pub first: SegmentId,
    pub second: SegmentId,
    #[serde(default)]
    pub construction: bool,
}

impl Sketch {
    /// Lays an ellipse from its centre and the four ends of its axes, the two
    /// axes drawn as construction traits.
    pub fn add_ellipse(
        &mut self,
        center: PointId,
        first: [PointId; 2],
        second: [PointId; 2],
    ) -> EllipseId {
        self.push_ellipse(center, first, second, false)
    }

    /// Helps place the rest of the drawing without becoming part of it.
    pub fn add_construction_ellipse(
        &mut self,
        center: PointId,
        first: [PointId; 2],
        second: [PointId; 2],
    ) -> EllipseId {
        self.push_ellipse(center, first, second, true)
    }

    fn push_ellipse(
        &mut self,
        center: PointId,
        [first_start, first_end]: [PointId; 2],
        [second_start, second_end]: [PointId; 2],
        construction: bool,
    ) -> EllipseId {
        let first = self.add_construction_segment(first_start, first_end);
        let second = self.add_construction_segment(second_start, second_end);
        self.ellipses.push(Ellipse {
            center,
            first,
            second,
            construction,
        });
        EllipseId(self.ellipses.len() - 1)
    }

    pub fn ellipses(&self) -> &[Ellipse] {
        &self.ellipses
    }

    pub fn is_erased_ellipse(&self, ellipse: EllipseId) -> bool {
        Erased::holds(&self.erased.ellipses, ellipse.0)
    }

    pub fn live_ellipses(&self) -> impl Iterator<Item = (EllipseId, Ellipse)> + '_ {
        self.ellipses
            .iter()
            .enumerate()
            .map(|(rank, ellipse)| (EllipseId(rank), *ellipse))
            .filter(|(id, _)| !self.is_erased_ellipse(*id))
    }

    /// The curve as its axes draw it now.
    pub fn ellipse_draft(&self, id: EllipseId) -> EllipseDraft {
        let ellipse = self.ellipses[id.0];
        let (start, end) = self.endpoints(ellipse.first);
        let (across_start, across_end) = self.endpoints(ellipse.second);
        EllipseDraft {
            centre: self.point(ellipse.center),
            first: (end - start) * 0.5,
            second: across_start.distance(across_end) * 0.5,
        }
    }

    /// The whole curve as a closed run of places.
    pub fn ellipse_polyline(&self, id: EllipseId) -> Vec<DVec2> {
        self.ellipse_draft(id).places()
    }

    /// The ellipse whose curve passes closest to `position`.
    pub fn nearest_ellipse(&self, position: DVec2, tolerance: f64) -> Option<EllipseId> {
        self.live_ellipses()
            .map(|(id, _)| (id, self.ellipse_draft(id).distance(position)))
            .filter(|(_, distance)| *distance <= tolerance)
            .min_by(|a, b| a.1.total_cmp(&b.1))
            .map(|(id, _)| id)
    }

    /// The ellipse a trait is an axis of, if it is one.
    pub fn ellipse_of_axis(&self, segment: SegmentId) -> Option<EllipseId> {
        self.live_ellipses()
            .find(|(_, ellipse)| ellipse.first == segment || ellipse.second == segment)
            .map(|(id, _)| id)
    }

    /// The five points an ellipse stands on: its centre, then the ends of its
    /// first axis and of its second.
    pub fn ellipse_points(&self, id: EllipseId) -> [PointId; 5] {
        let ellipse = self.ellipses[id.0];
        let (first, second) = (
            self.segments()[ellipse.first.0],
            self.segments()[ellipse.second.0],
        );
        [
            ellipse.center,
            first.start,
            first.end,
            second.start,
            second.end,
        ]
    }

    /// The ends of the axes of every ellipse centred on `point`, which move
    /// with it: dragging the centre alone would leave them behind and pull the
    /// curve out of shape.
    pub fn ellipse_ends_around(&self, point: PointId) -> Vec<PointId> {
        self.live_ellipses()
            .filter(|(_, ellipse)| ellipse.center == point)
            .flat_map(|(id, _)| self.ellipse_points(id)[1..].to_vec())
            .collect()
    }

    /// The centre an end of an ellipse's axis turns about, when it is one.
    pub(crate) fn centre_turned_about(&self, point: PointId) -> Option<PointId> {
        self.live_ellipses()
            .find(|(id, _)| self.ellipse_points(*id)[1..].contains(&point))
            .map(|(_, ellipse)| ellipse.center)
    }

    /// Every ellipse standing on `point`, to be taken with it.
    pub(crate) fn ellipses_leaning_on(&self, point: PointId) -> Vec<Element> {
        self.live_ellipses()
            .filter(|(id, _)| self.ellipse_points(*id).contains(&point))
            .map(|(id, _)| Element::Ellipse(id))
            .collect()
    }

    /// Takes an ellipse away with its two axes, and the points it stood on that
    /// nothing else stands on: an axis cannot outlive the curve it measures,
    /// and the curve cannot stand without its axes.
    pub(crate) fn erase_ellipse(&mut self, id: EllipseId) {
        if self.is_erased_ellipse(id) {
            return;
        }
        Erased::mark(&mut self.erased.ellipses, id.0);
        let ellipse = self.ellipses[id.0];
        let points = self.ellipse_points(id);
        self.erase(Element::Segment(ellipse.first));
        self.erase(Element::Segment(ellipse.second));
        for point in points {
            if !self.is_erased_point(point) && !self.anything_stands_on(point) {
                self.erase(Element::Point(point));
            }
        }
    }

    /// Takes away the ellipse a trait was an axis of, if it was one.
    pub(crate) fn erase_ellipse_of(&mut self, segment: SegmentId) {
        if let Some(ellipse) = self.ellipse_of_axis(segment) {
            self.erase_ellipse(ellipse);
        }
    }

    /// Whether a point is one of an ellipse's own handles and nothing else
    /// leans on it — nothing, that is, but the two axes the ellipse is laid
    /// with.
    pub(crate) fn is_a_bare_handle_of(&self, ellipse: EllipseId, point: PointId) -> bool {
        if !self.ellipse_points(ellipse).contains(&point) {
            return false;
        }
        let axes = self.ellipses()[ellipse.0];
        !self.live_segments().any(|(id, segment)| {
            id != axes.first
                && id != axes.second
                && (segment.start == point || segment.end == point)
        }) && !self
            .live_circles()
            .any(|(_, circle)| circle.center == point)
            && self.arcs_leaning_on(point).is_empty()
            && self
                .ellipses_leaning_on(point)
                .iter()
                .all(|held| *held == Element::Ellipse(ellipse))
    }

    /// Whether any curve still drawn stands on a point.
    fn anything_stands_on(&self, point: PointId) -> bool {
        self.live_segments()
            .any(|(_, segment)| segment.start == point || segment.end == point)
            || self
                .live_circles()
                .any(|(_, circle)| circle.center == point)
            || !self.arcs_leaning_on(point).is_empty()
            || !self.ellipses_leaning_on(point).is_empty()
    }
}

#[cfg(test)]
mod tests;
