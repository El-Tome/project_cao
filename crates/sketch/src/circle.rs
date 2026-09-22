//! A whole circle, as a centre and a size.

use glam::DVec2;
use serde::{Deserialize, Serialize};

use crate::erased::Erased;
use crate::length::LengthOutcome;
use crate::sketch::{PointId, Sketch};

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

impl Sketch {
    pub fn is_erased_circle(&self, circle: CircleId) -> bool {
        Erased::holds(&self.erased.circles, circle.0)
    }

    pub fn live_circles(&self) -> impl Iterator<Item = (CircleId, Circle)> + '_ {
        self.circles
            .iter()
            .enumerate()
            .map(|(rank, circle)| (CircleId(rank), *circle))
            .filter(|(id, _)| !self.is_erased_circle(*id))
    }

    pub fn circles(&self) -> &[Circle] {
        &self.circles
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

    /// Changes a circle's size by a step of the solve. Never below nothing: a
    /// circle turned inside out is not a circle.
    pub(crate) fn grow_circle(&mut self, circle: CircleId, delta: f64) {
        if let Some(round) = self.circles.get_mut(circle.0) {
            round.radius = (round.radius + delta).max(1e-9);
        }
    }

    pub fn set_circle_radius(&mut self, id: CircleId, radius: f64) -> LengthOutcome {
        if radius <= 0.0 {
            return LengthOutcome::Degenerate;
        }
        self.circles[id.0].radius = radius;
        LengthOutcome::Exact
    }
}
