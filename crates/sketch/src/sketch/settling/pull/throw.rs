//! How far a drag may throw a point: read on the drawing as the press found
//! it, and judged at every try of the drag. A landing may hold every rule; a
//! point sent far beyond the drawing is still not what the hand asked for, and
//! the try is refused as a cut corner turned inside out is (#597).

use glam::DVec2;

use super::PointPull;
use crate::length::LengthOutcome;
use crate::sketch::Sketch;

/// Where the press found every point, and how far from there any point may
/// end, the dragged one held to it like any other: twice the sum of the hand's
/// way and the drawing's size. A shape stretched after a corner taken far
/// carries its points as far as the hand went, never farther; a shape turned
/// about a point near the hand swings its far side by up to twice the
/// drawing's size.
#[derive(Clone, Debug)]
pub(crate) struct Throw {
    press: Vec<DVec2>,
    bound: f64,
}

impl Sketch {
    /// Lays one place of the drag down: the point taken towards `position`
    /// and the drawing settled around it, as `pull` allows.
    ///
    /// `Exact` when the drawing settled, the point wherever it could go;
    /// `BestEffort` when nothing could be had and the drawing is given back
    /// as it was. Every frame is laid on the drawing as the press found it,
    /// which is where the bound of the drag's throw is read.
    pub fn settle_pulled(
        &mut self,
        pull: &PointPull,
        position: DVec2,
        millimeters_per_unit: f64,
    ) -> LengthOutcome {
        let outer = self.held.throw.replace(Throw {
            press: self.points().to_vec(),
            bound: 2.0 * (position.distance(pull.from) + self.drawing_size()),
        });
        let outcome = self.lay_pulled(pull, position, millimeters_per_unit);
        self.held.throw = outer;
        outcome
    }

    /// Whether the drag under way has thrown a point, the dragged one
    /// included, past its bound. Never outside a drag.
    pub(crate) fn throws_a_point(&self) -> bool {
        let Some(throw) = &self.held.throw else {
            return false;
        };
        self.live_points().any(|(point, place)| {
            throw
                .press
                .get(point.0)
                .is_some_and(|was| place.distance(*was) > throw.bound)
        })
    }
}
