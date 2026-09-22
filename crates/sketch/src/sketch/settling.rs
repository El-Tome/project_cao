//! A point, or a handful of them, put where they were dropped, and the rest of
//! the drawing settled around them.

use glam::DVec2;

use super::{LengthOutcome, PointId, Sketch};

impl Sketch {
    /// Puts a point where it was dropped and settles the rest of the drawing
    /// around it, that point staying exactly where it was put.
    ///
    /// An end of an ellipse's axis turns and stretches the curve about its
    /// centre, which is held where it stands for that: left free, it would
    /// share the pull with the end and slide off towards the cursor.
    pub fn settle_around(
        &mut self,
        point: PointId,
        position: DVec2,
        millimeters_per_unit: f64,
    ) -> LengthOutcome {
        let mut dropped = vec![(point, position)];
        dropped.extend(
            self.centre_turned_about(point)
                .map(|centre| (centre, self.point(centre))),
        );
        self.settle_around_all(&dropped, millimeters_per_unit)
    }

    /// The same for a whole handful of points dropped at once, which is how a
    /// selection is moved in one block.
    ///
    /// Held, they do not give: the drawing settles around them rather than
    /// pulling them back, so a shape follows the mouse instead of squirming
    /// away from it.
    ///
    /// When holding them is more than the drawing can bear — a corner dragged
    /// somewhere no tangency can reach it — the values already given win over
    /// the cursor: everything goes back and settles the ordinary way. Leaving
    /// the half-solved state was what let a circle be dragged out of shape and
    /// stay that way until the next change put it right.
    pub fn settle_around_all(
        &mut self,
        dropped: &[(PointId, DVec2)],
        millimeters_per_unit: f64,
    ) -> LengthOutcome {
        let kept = self.shapes_now();
        let place = |sketch: &mut Self| {
            for (point, position) in dropped {
                sketch.move_point(*point, *position);
            }
        };

        place(self);
        self.held = dropped.iter().map(|(point, _)| *point).collect();
        let outcome = self.resolve(millimeters_per_unit);
        self.held.clear();
        if outcome == LengthOutcome::Exact {
            return outcome;
        }

        self.points.clone_from(&kept.0);
        self.circles.clone_from(&kept.1);
        place(self);
        let outcome = self.resolve(millimeters_per_unit);
        if !self.has_a_collapsed_trait(self.drawing_size()) && !self.has_a_flipped_tangent() {
            return outcome;
        }

        // Neither way leaves a drawing worth keeping: a trait may have collapsed,
        // or a tangency's contact slid off its segment. The gesture is refused
        // rather than the shape broken — the point simply does not go there.
        self.give_back(kept);
        LengthOutcome::BestEffort
    }
}
