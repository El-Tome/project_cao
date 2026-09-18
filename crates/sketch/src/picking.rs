//! What a gesture takes hold of: what sits under the cursor, and which points
//! a held selection would carry. What a box catches is in [`crate::banding`].
//!
//! The tolerance arrives in world units — the front-end knows how many of them
//! a pixel is worth, and this does not.

use glam::DVec2;

use crate::annotation::AnnotationMetrics;
use crate::arc::ArcId;
use crate::constraints::{Constraint, DimensionTarget};
use crate::sketch::{Element, PointId, Sketch};
use crate::snap::onto_rim;

/// What the selection tool is holding, and what pressing the delete key would
/// take away.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Selection {
    Element(Element),
    Dimension(DimensionTarget),
    Rule(Constraint),
}

/// How much easier a wide target is to hit than the trait it is drawn over. An
/// annotation and a rule mark are read rather than aimed at, so they answer to
/// a looser reach than the geometry does.
const WIDE_TARGET_REACH: f64 = 1.5;

impl Sketch {
    /// What the cursor is over, in the order a click should take it.
    ///
    /// A point before a trait before a curve before an annotation: the smaller
    /// the target, the harder it is to hit on purpose, so the smaller one wins.
    /// The origin is never picked — it is there to be measured from, not moved.
    pub fn pick(
        &self,
        cursor: DVec2,
        tolerance: f64,
        metrics: AnnotationMetrics,
    ) -> Option<Selection> {
        if let Some(point) = self
            .nearest_point(cursor, tolerance)
            .filter(|point| !self.is_origin(*point))
        {
            return Some(Selection::Element(Element::Point(point)));
        }
        if let Some(segment) = self.nearest_segment(cursor, tolerance) {
            return Some(Selection::Element(Element::Segment(segment)));
        }
        if let Some(circle) = self.nearest_circle(cursor, tolerance) {
            return Some(Selection::Element(Element::Circle(circle)));
        }
        if let Some(arc) = self.nearest_arc(cursor, tolerance) {
            return Some(Selection::Element(Element::Arc(arc)));
        }
        let wide = tolerance * WIDE_TARGET_REACH;
        if let Some(target) = self.nearest_dimension(cursor, wide, metrics) {
            return Some(Selection::Dimension(target));
        }
        self.nearest_rule(cursor, wide).map(Selection::Rule)
    }

    /// The points a held selection would carry, were it dragged.
    ///
    /// A trait gives both its ends, a circle its centre, and an annotation or a
    /// rule nothing: they are drawn from the geometry rather than placed. The
    /// origin never moves, so it is never carried.
    pub fn points_of(&self, selection: &[Selection]) -> Vec<PointId> {
        let mut points: Vec<PointId> = Vec::new();
        let mut take = |point: PointId| {
            if !self.is_origin(point) && !points.contains(&point) {
                points.push(point);
            }
        };
        for held in selection {
            let Selection::Element(element) = held else {
                continue;
            };
            for point in self.points_it_leans_on(*element) {
                take(point);
            }
        }
        points
    }

    /// Where an arc takes a place near it.
    ///
    /// Beyond either end it is that end, not the far side of the circle the arc
    /// is a piece of: the rest of that circle is not drawn, and a click out
    /// there must find nothing.
    pub fn place_on_arc(&self, id: ArcId, position: DVec2) -> DVec2 {
        let arc = self.arc(id);
        let centre = self.point(arc.center);
        let (start, end) = (self.point(arc.start), self.point(arc.end));
        let Some(on_rim) = onto_rim(position, centre, self.arc_radius(id)) else {
            return start;
        };
        let along = ((position - centre).to_angle() - (start - centre).to_angle())
            .rem_euclid(std::f64::consts::TAU);
        if along <= self.arc_sweep(id) {
            return on_rim;
        }
        match position.distance(start) <= position.distance(end) {
            true => start,
            false => end,
        }
    }

    /// How far a place sits from an arc's curve itself.
    pub fn distance_to_arc(&self, id: ArcId, position: DVec2) -> f64 {
        self.place_on_arc(id, position).distance(position)
    }

    /// The arc whose curve passes closest to `position`.
    pub fn nearest_arc(&self, position: DVec2, tolerance: f64) -> Option<ArcId> {
        self.live_arcs()
            .map(|(id, _)| (id, self.distance_to_arc(id, position)))
            .filter(|(_, distance)| *distance <= tolerance)
            .min_by(|a, b| a.1.total_cmp(&b.1))
            .map(|(id, _)| id)
    }
}

#[cfg(test)]
mod tests;
