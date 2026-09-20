//! A circle or an arc drawn to a new size by hand, about the centre it
//! already has.
//!
//! The size rather than a value: dragging a shape bigger says how big it is
//! now, not how big it must stay. What pins a size is a dimension, and a
//! rough drag must not leave one behind.

use glam::DVec2;
use serde::{Deserialize, Serialize};

use crate::arc::ArcId;
use crate::length::LengthOutcome;
use crate::sketch::{Circle, CircleId, Sketch};

/// A curve that stands a reach from a centre, which is what a drag of the
/// curve itself can draw to a new size. A trait has no such reach: its two
/// ends are its size, and they are dragged as the points they are.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Curved {
    Circle(CircleId),
    Arc(ArcId),
}

impl Sketch {
    /// The curve a press takes hold of to draw it to another size: the nearer
    /// of the circle and the arc under it, and nothing where neither is.
    pub fn curve_at(&self, place: DVec2, reach: f64) -> Option<Curved> {
        let off_the_rim = |id: CircleId| {
            let round = self.circle(id);
            (self.point(round.center).distance(place) - round.radius).abs()
        };
        let circle = self
            .nearest_circle(place, reach)
            .map(|id| (Curved::Circle(id), off_the_rim(id)));
        let arc = self
            .nearest_arc(place, reach)
            .map(|id| (Curved::Arc(id), self.distance_to_arc(id, place)));
        [circle, arc]
            .into_iter()
            .flatten()
            .min_by(|left, right| left.1.total_cmp(&right.1))
            .map(|(curve, _)| curve)
    }

    /// Where a curve turns about, which is what a size is measured from.
    pub fn centre_of(&self, curve: Curved) -> DVec2 {
        match curve {
            Curved::Circle(circle) => self.point(self.circle(circle).center),
            Curved::Arc(arc) => self.point(self.arc(arc).center),
        }
    }

    /// Draws a curve to a new size about the centre it already has.
    pub fn resize(
        &mut self,
        curve: Curved,
        reach: f64,
        millimeters_per_unit: f64,
    ) -> LengthOutcome {
        match curve {
            Curved::Circle(circle) => self.resize_circle(circle, reach, millimeters_per_unit),
            Curved::Arc(arc) => self.resize_arc(arc, reach, millimeters_per_unit),
        }
    }
}

/// Below this a circle is not a circle, and an arc turns about nothing.
const NO_REACH: f64 = 1e-9;

impl Sketch {
    /// Draws a circle to a new size about the centre it already has, and
    /// settles the drawing around it.
    ///
    /// The size is not held against the rest: a diameter already typed, or a
    /// trait the circle is tangent to, says what the circle may be, and the
    /// drawing settling is what gives it back the size it is allowed. A drag
    /// that asks for something the drawing forbids therefore leaves the shape
    /// where it was, rather than leaving it half-way.
    pub fn resize_circle(
        &mut self,
        circle: CircleId,
        reach: f64,
        millimeters_per_unit: f64,
    ) -> LengthOutcome {
        if reach < NO_REACH {
            return LengthOutcome::Degenerate;
        }
        let kept = self.shapes_now();
        self.set_circle_radius(circle, reach);
        let outcome = self.resolve(millimeters_per_unit);
        self.keep_or_give_back(kept, outcome)
    }

    /// The same for an arc: its two ends travel out to the new reach, keeping
    /// the sweep the curve was drawn with, and the drawing settles around
    /// them.
    ///
    /// The ends rather than a size of its own: an arc keeps no radius, it is
    /// read off the end it starts at, so drawing one bigger is moving both of
    /// its ends out from the centre.
    pub fn resize_arc(
        &mut self,
        arc: ArcId,
        reach: f64,
        millimeters_per_unit: f64,
    ) -> LengthOutcome {
        if reach < NO_REACH || arc.0 >= self.arcs().len() {
            return LengthOutcome::Degenerate;
        }
        let curve = self.arc(arc);
        let centre = self.point(curve.center);
        let out = |end| {
            (self.point(end) - centre)
                .try_normalize()
                .map(|along| (end, centre + along * reach))
        };
        let Some(dropped) = out(curve.start).zip(out(curve.end)) else {
            return LengthOutcome::Degenerate;
        };
        self.settle_around_all(&[dropped.0, dropped.1], millimeters_per_unit)
    }

    /// Keeps what the settling made of the drawing, or gives it back whole.
    fn keep_or_give_back(
        &mut self,
        kept: (Vec<DVec2>, Vec<Circle>),
        outcome: LengthOutcome,
    ) -> LengthOutcome {
        if !self.has_a_collapsed_trait(self.drawing_size()) && !self.has_a_flipped_tangent() {
            return outcome;
        }
        self.give_back(kept);
        LengthOutcome::BestEffort
    }
}

#[cfg(test)]
mod tests;
