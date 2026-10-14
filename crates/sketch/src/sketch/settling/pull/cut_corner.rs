//! The free end of a side of a cut corner, pulled: the side turns about the
//! corner, as it does at a corner nothing was cut from (#565).
//!
//! Left alone, the drag would rather slide the whole corner than turn the side:
//! a fillet's curve is a shape of its own, whose ends keep their bearing from
//! its centre while it is dragged, and a side tangent to it keeps its direction
//! with them.

use glam::DVec2;

use super::super::give::Give;
use super::super::kept::Kept;
use crate::sketch::{PointId, Sketch};

impl Sketch {
    /// What a drag of `point` holds once a cut corner it is the free end of a
    /// side of is taken into account: the corner's own point stays, and the
    /// cut's curve is let turn with the side. `held` is what it holds
    /// otherwise — the points that stay, the lines kept and the way the point
    /// follows.
    ///
    /// Taken only when the point follows the hand as freely that way: a corner
    /// held still in a rectangle leaves the side it is on nowhere to go but
    /// along itself, and the corner moves with the side as it would uncut.
    pub(super) fn held_at_cut_corners(
        &self,
        point: PointId,
        shape: &[PointId],
        about: Option<PointId>,
        held: (Vec<PointId>, Vec<Kept>, Give),
        millimeters_per_unit: f64,
    ) -> (Vec<PointId>, Vec<Kept>, Give) {
        let cut = self.cut_corners();
        if cut
            .iter()
            .any(|corner| corner.sides.iter().any(|side| side.near == point))
        {
            return held;
        }
        let corners: Vec<_> = cut
            .into_iter()
            .filter(|corner| corner.sides.iter().any(|side| side.far == point))
            .collect();
        if corners.is_empty() {
            return held;
        }
        let curves: Vec<_> = corners
            .iter()
            .filter_map(|corner| self.cut_curve(corner))
            .map(|arc| self.arc(arc))
            .collect();
        let (pins, lines, give) = held;
        let mut turning_pins = pins.clone();
        turning_pins.extend(corners.iter().map(|corner| corner.point));
        turning_pins.sort_by_key(|pin| pin.0);
        turning_pins.dedup();
        let turning_lines: Vec<Kept> = lines
            .iter()
            .copied()
            .filter(|line| {
                !curves.iter().any(|arc| {
                    line.from == Some(arc.center) && [arc.start, arc.end].contains(&line.to)
                })
            })
            .collect();
        let turning = self.give_of(
            point,
            shape,
            &turning_pins,
            &turning_lines,
            about,
            millimeters_per_unit,
        );
        match follows_as_freely(give, turning) {
            true => (turning_pins, turning_lines, turning),
            false => (pins, lines, give),
        }
    }

    /// Takes `point` to `landing`, and lays again the cut of each corner it is
    /// the free end of a side of, where that corner stays: the drawing is left
    /// only the solver's precision to mend. The corners whole before the move
    /// are what the settling that follows may not turn inside out.
    ///
    /// Left to the solver, a side swung far round in one go can bring its end
    /// at the cut out past the corner, and the cut inside out with it — and a
    /// drag is laid in one go from the press at every frame.
    pub(super) fn move_laying_cuts(&mut self, point: PointId, pins: &[PointId], landing: DVec2) {
        let whole = self.cut_corners();
        self.move_point(point, landing);
        for corner in whole
            .iter()
            .filter(|corner| pins.contains(&corner.point))
            .filter(|corner| corner.sides.iter().any(|side| side.far == point))
        {
            self.lay_cut_again(corner);
        }
        self.held.cut = Some(whole);
    }
}

/// Whether a point held one way follows the hand as freely as held the
/// other: anywhere both ways, or along the same line.
fn follows_as_freely(before: Give, after: Give) -> bool {
    match (before, after) {
        (Give::Free, Give::Free) => true,
        (Give::Along(one), Give::Along(other)) => one.perp_dot(other).abs() < 1e-6,
        _ => false,
    }
}
