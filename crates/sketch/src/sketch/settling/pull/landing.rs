//! Where a pulled point is taken at each frame, and what it lands on: what
//! the frame asks of a pull, the drawing's own answers.

use glam::DVec2;

use super::{Holding, PointPull, Way};
use crate::sketch::settling::give::Give;
use crate::sketch::{PointId, Sketch};
use crate::snap::SnapSettings;
use crate::turning::angle_onto_grid;

impl PointPull {
    /// Where the shape turns about when the drag can only turn it.
    fn pivot(&self, sketch: &Sketch) -> Option<DVec2> {
        match &self.way {
            Way::Held(Holding {
                give: Give::Nowhere,
                about: Some(about),
                ..
            }) => Some(sketch.point(*about)),
            _ => None,
        }
    }

    /// Whether the drag can only turn the shape — the one drag that does not
    /// follow the magnets, and does not go where the hand is.
    pub fn turns(&self) -> bool {
        matches!(
            &self.way,
            Way::Held(Holding {
                give: Give::Nowhere,
                about: Some(_),
                ..
            })
        )
    }

    /// Where the point is taken at a frame of the drag: along whatever holds
    /// it, towards the cursor as the magnets left it; with the key that pulls
    /// it off what holds it, to that cursor itself; and when its shape can
    /// only turn, towards the hand before any magnet, its end pulled onto the
    /// grid.
    pub fn landing(
        &self,
        sketch: &Sketch,
        hand: DVec2,
        snapped: DVec2,
        letting_go: bool,
        grid: &SnapSettings,
    ) -> DVec2 {
        match (letting_go, self.turns()) {
            (true, _) => snapped,
            (false, true) => self.onto_grid(sketch, hand, grid),
            (false, false) => sketch.slide(self.point, snapped),
        }
    }

    /// Where to take the point once the grid has had its say. When the drag
    /// can only turn the shape, the point is turned towards `cursor` and its
    /// end pulled onto a grid point within reach — what brings a shape drawn
    /// on the grid back square in one gesture. Otherwise `cursor` as it is.
    pub fn onto_grid(&self, sketch: &Sketch, cursor: DVec2, grid: &SnapSettings) -> DVec2 {
        let Some(about) = self.pivot(sketch) else {
            return cursor;
        };
        let (Some(was), Some(wanted)) = (
            (self.from - about).try_normalize(),
            (cursor - about).try_normalize(),
        ) else {
            return cursor;
        };
        let angle = angle_onto_grid(about, self.from, was.angle_to(wanted), grid);
        about + DVec2::from_angle(angle).rotate(self.from - about)
    }

    /// Whether the drag took the point all the way to `landing` in the
    /// drawing it settled — or stopped it short, the shape unable to follow.
    pub fn arrived(&self, settled: &Sketch, landing: DVec2) -> bool {
        settled
            .points()
            .get(self.point.0)
            .is_some_and(|place| place.distance(landing) <= settled.drawing_size() * 1e-9)
    }

    /// The point a drop joins the dragged one to, when there is one within
    /// `reach` of where it lands. Nothing for a point that stopped short of
    /// the hand, since what the hand is over is not where the point is; and
    /// when the shape turned, only a point the end landed on exactly — the
    /// magnets never pulled that end, so being near is not being on.
    pub fn joined_to(
        &self,
        sketch: &Sketch,
        settled: &Sketch,
        landing: DVec2,
        reach: f64,
    ) -> Option<PointId> {
        if !self.arrived(settled, landing) {
            return None;
        }
        let other = sketch
            .nearest_point(landing, reach)
            .filter(|other| *other != self.point)?;
        match self.turns() {
            true => (sketch.point(other).distance(landing) <= reach * 1e-6).then_some(other),
            false => Some(other),
        }
    }
}
