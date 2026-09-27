//! A point, or a handful of them, put where they were dropped, and the rest of
//! the drawing settled around them.

use glam::DVec2;

use super::{LengthOutcome, PointId, Sketch};

mod give;
mod kept;
mod pull;
mod shape;

pub(crate) use kept::Kept;
pub use pull::PointPull;

/// What the user is holding while a drag lasts: points the solver reads as
/// immovable, and the lines the drag keeps where they lie. Nothing to save:
/// it lives only as long as the gesture.
#[derive(Clone, Debug, Default)]
pub(crate) struct Held {
    pub(crate) points: Vec<PointId>,
    pub(crate) lines: Vec<Kept>,
}

impl Sketch {
    /// Takes a point where it was dropped and settles the rest of the drawing
    /// around it, as [`Sketch::pull`] reads the drag from the drawing as it
    /// stands — the pull read and laid down at once, for a drag of one frame.
    /// A replay reads the pull before the drop lays its holds, and lays it
    /// with [`Sketch::settle_pulled`].
    pub fn settle_around(
        &mut self,
        point: PointId,
        position: DVec2,
        millimeters_per_unit: f64,
    ) -> LengthOutcome {
        let pull = self.pull(point, millimeters_per_unit);
        self.settle_pulled(&pull, position, millimeters_per_unit)
    }

    /// The drag as it always was, for a point nothing else stays for: the
    /// point exactly where it was put and the drawing settled around it.
    ///
    /// An end of an ellipse's axis stretches the curve about its centre,
    /// which is held where it stands for that: left free, it would share the
    /// pull with the end and slide off towards the cursor.
    fn settle_plainly(
        &mut self,
        point: PointId,
        position: DVec2,
        millimeters_per_unit: f64,
    ) -> LengthOutcome {
        let mut dropped = vec![(point, position)];
        let anchored: Vec<PointId> = self.centre_turned_about(point).into_iter().collect();
        dropped.extend(anchored.iter().map(|centre| (*centre, self.point(*centre))));
        self.settle_around_dropped(&dropped, &anchored, millimeters_per_unit)
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
        self.settle_around_dropped(dropped, &[], millimeters_per_unit)
    }

    /// The same, told which of the points stay put whatever happens.
    ///
    /// An ellipse's centre is one: the values already given may refuse the
    /// drag outright — a length and an angle leave an axis free to travel and
    /// nothing else — and the drawing settling the ordinary way then slides
    /// the whole curve after the cursor. What the gesture cannot do, it does
    /// not do: the centre is pinned for that pass too.
    fn settle_around_dropped(
        &mut self,
        dropped: &[(PointId, DVec2)],
        anchored: &[PointId],
        millimeters_per_unit: f64,
    ) -> LengthOutcome {
        let kept = self.shapes_now();
        let place = |sketch: &mut Self| {
            for (point, position) in dropped {
                sketch.move_point(*point, *position);
            }
        };

        place(self);
        self.held.points = dropped.iter().map(|(point, _)| *point).collect();
        let outcome = self.resolve(millimeters_per_unit);
        self.held.points.clear();
        if outcome == LengthOutcome::Exact {
            return outcome;
        }

        self.points.clone_from(&kept.0);
        self.circles.clone_from(&kept.1);
        place(self);
        self.held.points = anchored.to_vec();
        let outcome = self.resolve(millimeters_per_unit);
        self.held.points.clear();
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

#[cfg(test)]
mod tests;
