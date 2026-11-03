//! The equations a rule hanging a point on a fillet or a chamfer asks of the
//! solver.
//!
//! The cut is not solved: what it draws is worked out from its corner and the
//! far end of each side. So is each place it offers, and the rule holding a
//! point there reads it the same way, with how it moves with those three
//! points — that is what lets a point pulled by something else move the
//! corner, and the corner moving carry the point.

use glam::DVec2;

use crate::constraints::Constraint;
use crate::corner_cut::CornerCutId;
use crate::corner_cut::points::{COORDINATES, CutPoint, OWN};
use crate::equation::Equation;
use crate::jet::{Jet, JetPlace};
use crate::sketch::{PointId, Sketch};

impl Sketch {
    /// What a rule hanging a point on a cut asks of the drawing: nothing
    /// while the cut's corner gives it no place.
    pub(super) fn cut_hold_equations(&self, constraint: Constraint, into: &mut Vec<Equation>) {
        match constraint {
            Constraint::AtCutPoint { point, cut, at } => self.at_cut_point(point, cut, at, into),
            Constraint::OnCut { point, cut } => into.extend(self.on_cut(point, cut)),
            _ => {}
        }
    }

    /// A point standing at one of a cut's places: one row along the first
    /// side and one across it. Read in the cut's own frame rather than the
    /// sketch's, the two rows stand still when the whole shape turns, which
    /// is what lets a settle read the shape as free to turn and keep it the
    /// way up it was drawn.
    fn at_cut_point(
        &self,
        point: PointId,
        cut: CornerCutId,
        at: CutPoint,
        into: &mut Vec<Equation>,
    ) {
        if point.0 >= self.points().len() {
            return;
        }
        let Some(places) = self.cut_places(cut) else {
            return;
        };
        let Some(place) = places.at(at) else {
            return;
        };
        let off = JetPlace::free(self.point(point), OWN) - place;
        for row in [off.dot(places.along), places.along.perp_dot(off)] {
            let mut equation = Equation::new(self.variables());
            equation.error = row.value;
            self.add_slopes(&mut equation, cut, &[point], row);
            into.push(equation);
        }
    }

    /// A point on the whole curve a cut draws: as far from a fillet's centre
    /// as its radius, or on the line of a chamfer's cut.
    fn on_cut(&self, point: PointId, cut: CornerCutId) -> Option<Equation> {
        if point.0 >= self.points().len() {
            return None;
        }
        let places = self.cut_places(cut)?;
        let held = JetPlace::free(self.point(point), OWN);
        let off = match places.round {
            Some((centre, radius)) => {
                let reach = (held - centre).length();
                (reach.value > 1e-9).then(|| reach + -radius)?
            }
            None => {
                let [from, to] = places.touches;
                let run = to - from;
                (run.length().value > 1e-12).then(|| run.normalized().perp_dot(held - from))?
            }
        };
        let mut equation = Equation::new(self.variables());
        equation.error = off.value;
        self.add_slopes(&mut equation, cut, &[point], off);
        Some(equation)
    }

    /// Adds to a row how `value` moves with the corner, the far end of each
    /// side, then each of `own`, in the order their coordinates were given.
    fn add_slopes(
        &self,
        equation: &mut Equation,
        cut: CornerCutId,
        own: &[PointId],
        value: Jet<COORDINATES>,
    ) {
        let points = self
            .cut_stands_on(cut)
            .into_iter()
            .chain(own.iter().copied());
        for (rank, point) in points.enumerate() {
            let slope = DVec2::new(value.slope[rank * 2], value.slope[rank * 2 + 1]);
            equation.add(point, slope);
        }
    }
}
