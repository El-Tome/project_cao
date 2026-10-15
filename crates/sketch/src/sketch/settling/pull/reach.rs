//! How far along the hand's way a shape can stretch, when it cannot stretch
//! all the way: the way halved until the last place it still follows to.

use glam::DVec2;

use super::{Holding, PROGRESS, PointPull, REACHED_WITHIN};
use crate::sketch::Sketch;

/// How many halvings the way to the last place a shape follows to may take
/// at the most.
const HALVINGS: usize = 24;

impl Sketch {
    /// The point held as far towards `landing` as the shape can stretch: the
    /// way there halved until the last place it still follows to.
    pub(super) fn stretch_towards(
        &mut self,
        pull: &PointPull,
        holding: &Holding,
        landing: DVec2,
        millimeters_per_unit: f64,
    ) -> bool {
        let (mut reached, mut missed) = (0.0, 1.0);
        let within = self.drawing_size() * REACHED_WITHIN / pull.from.distance(landing).max(1e-12);
        for _ in 0..HALVINGS {
            if missed - reached <= within {
                break;
            }
            let halfway = (reached + missed) / 2.0;
            let place = pull.from.lerp(landing, halfway);
            let kept = self.shapes_now();
            match self.stretch(pull.point, holding, place, millimeters_per_unit) {
                true => reached = halfway,
                false => missed = halfway,
            }
            self.give_back(kept);
        }
        // A hair's breadth past the press is no way at all: the hand's way
        // runs off a curve the point is held to, and letting it go is what
        // brings it round.
        reached * pull.from.distance(landing) > self.drawing_size() * PROGRESS
            && self.stretch(
                pull.point,
                holding,
                pull.from.lerp(landing, reached),
                millimeters_per_unit,
            )
    }
}
