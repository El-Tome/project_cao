//! A cut corner pulled: the free end of one of its sides turns that side about
//! the corner (#565), and the corner itself goes where the hand is, as a sharp
//! corner would, its sides turning about what they run out to (#591).
//!
//! Left alone, the drag would rather slide the whole corner than turn a side:
//! a fillet's curve is a shape of its own, whose ends keep their bearing from
//! its centre while it is dragged, and a side tangent to it keeps its direction
//! with them.

use glam::DVec2;

use super::super::give::Give;
use super::super::kept::Kept;
use crate::corner::CutCorner;
use crate::sketch::{PointId, Sketch};

impl Sketch {
    /// What a drag of `point` holds once the cut corners it belongs to are
    /// taken into account. `held` is what it holds otherwise — the points that
    /// stay, the lines kept and the way the point follows.
    ///
    /// The free end of a side: the corner's own point stays, and the cut's
    /// curve is let turn with the side — taken only when the point follows the
    /// hand as freely that way: a corner held still on a side held level
    /// leaves the side nowhere to go but along itself, and the corner moves
    /// with the side as it would uncut.
    pub(super) fn held_at_cut_corners(
        &self,
        point: PointId,
        shape: &[PointId],
        about: Option<PointId>,
        held: (Vec<PointId>, Vec<Kept>, Give),
        millimeters_per_unit: f64,
    ) -> (Vec<PointId>, Vec<Kept>, Give) {
        let cut = self.cut_corners();
        if let Some(corner) = cut.iter().find(|corner| corner.point == point) {
            return self.held_at_its_corner(corner, &cut, shape, held.1, millimeters_per_unit);
        }
        if cut
            .iter()
            .any(|corner| corner.sides.iter().any(|side| side.near == point))
        {
            return held;
        }
        let corners: Vec<CutCorner> = cut
            .into_iter()
            .filter(|corner| corner.sides.iter().any(|side| side.far == point))
            .collect();
        if corners.is_empty() {
            return held;
        }
        let (pins, lines, give) = held;
        let mut turning_pins = pins.clone();
        turning_pins.extend(corners.iter().map(|corner| corner.point));
        turning_pins.sort_by_key(|pin| pin.0);
        turning_pins.dedup();
        let turning_lines = self.without_cut_curves(&lines, &corners);
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

    /// The corner a cut left, pulled itself: what it runs out to stays — each
    /// side's free end, or the corner at the side's other end where that one
    /// is cut too — and its cut and the cuts at those corners are let turn.
    fn held_at_its_corner(
        &self,
        corner: &CutCorner,
        cut: &[CutCorner],
        shape: &[PointId],
        lines: Vec<Kept>,
        millimeters_per_unit: f64,
    ) -> (Vec<PointId>, Vec<Kept>, Give) {
        let turned = self.turned_with(corner, cut);
        let pins: Vec<PointId> = corner
            .sides
            .iter()
            .map(|side| self.anchor_of(cut, side))
            .collect();
        let lines = self.without_cut_curves(&lines, &turned);
        let give = self.give_of(
            corner.point,
            shape,
            &pins,
            &lines,
            None,
            millimeters_per_unit,
        );
        (pins, lines, give)
    }

    /// A cut corner and the cut corners its sides run out to: the cuts a drag
    /// of the corner turns.
    fn turned_with(&self, corner: &CutCorner, cut: &[CutCorner]) -> Vec<CutCorner> {
        let anchors = corner.sides.map(|side| self.anchor_of(cut, &side));
        std::iter::once(*corner)
            .chain(
                cut.iter()
                    .filter(|other| anchors.contains(&other.point))
                    .copied(),
            )
            .collect()
    }

    /// The lines kept but those that would hold the cuts of `corners` the way
    /// they lie: a curve's ends to the bearing they have from its centre, and
    /// the direction of a stretch or of a chamfer's cut, which an angle typed
    /// between them ties — the angle keeps its value as the cut turns.
    fn without_cut_curves(&self, lines: &[Kept], corners: &[CutCorner]) -> Vec<Kept> {
        let curves: Vec<_> = corners
            .iter()
            .filter_map(|corner| self.cut_curve(corner))
            .map(|arc| self.arc(arc))
            .collect();
        let traits: Vec<_> = corners
            .iter()
            .flat_map(|corner| self.cut_traits(corner))
            .collect();
        lines
            .iter()
            .copied()
            .filter(|line| {
                line.segment
                    .is_none_or(|segment| !traits.contains(&segment))
            })
            .filter(|line| {
                !curves.iter().any(|arc| {
                    line.from == Some(arc.center) && [arc.start, arc.end].contains(&line.to)
                })
            })
            .collect()
    }

    /// Takes `point` to `landing`, and lays again the cuts the drag turns: of
    /// each corner `point` is the free end of a side of, where that corner
    /// stays, or, when `point` is a cut corner, its own and those of the
    /// corners its sides run out to. The drawing is left only the solver's
    /// precision to mend. The corners whole before the move are what the
    /// settling that follows may not turn inside out.
    ///
    /// Left to the solver, a side swung far round in one go can bring its end
    /// at the cut out past the corner, and the cut inside out with it — and a
    /// drag is laid in one go from the press at every frame.
    pub(super) fn move_laying_cuts(&mut self, point: PointId, pins: &[PointId], landing: DVec2) {
        let whole = self.cut_corners();
        self.move_point(point, landing);
        let turned: Vec<CutCorner> = match whole.iter().find(|corner| corner.point == point) {
            Some(corner) => self.turned_with(corner, &whole),
            None => whole
                .iter()
                .filter(|corner| pins.contains(&corner.point))
                .filter(|corner| corner.sides.iter().any(|side| side.far == point))
                .copied()
                .collect(),
        };
        for corner in &turned {
            self.lay_cut_again(corner, &whole);
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
