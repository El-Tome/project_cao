//! A piece of a circle, as a centre and the two ends it runs between.

use glam::DVec2;
use serde::{Deserialize, Serialize};

use crate::arcing::{ArcDraft, places_along, steps_along, sweep_of};
use crate::equation::Equation;
use crate::erased::Erased;
use crate::sketch::{Element, PointId, Sketch};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ArcId(pub usize);

/// A piece of a circle, running counter-clockwise from `start` to `end` around
/// `center`.
///
/// The three points are ordinary points of the drawing, shared like a segment's
/// ends: an arc a polyline can run into is one a trim can cut to and a fillet
/// can leave behind, where an arc holding its own private angles could only
/// ever sit on its own.
///
/// The radius is not kept, it is read off `start`. One number fewer to hold in
/// step, and it is what leaves the direction of travel a consequence of the
/// order of the two ends rather than a fourth field free to disagree with them:
/// the other arc between the same ends is this one with `start` and `end`
/// swapped.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Arc {
    pub center: PointId,
    pub start: PointId,
    pub end: PointId,
    #[serde(default)]
    pub construction: bool,
}

impl Sketch {
    pub fn add_arc(&mut self, center: PointId, start: PointId, end: PointId) -> ArcId {
        self.push_arc(center, start, end, false)
    }

    /// Helps place the rest of the drawing without becoming part of it:
    /// excluded from the area of any region it borders.
    pub fn add_construction_arc(&mut self, center: PointId, start: PointId, end: PointId) -> ArcId {
        self.push_arc(center, start, end, true)
    }

    fn push_arc(
        &mut self,
        center: PointId,
        start: PointId,
        end: PointId,
        construction: bool,
    ) -> ArcId {
        self.arcs.push(Arc {
            center,
            start,
            end,
            construction,
        });
        ArcId(self.arcs.len() - 1)
    }

    /// How far round the arc goes, in radians.
    pub fn arc_sweep(&self, id: ArcId) -> f64 {
        sweep_of(self.arc_draft(id))
    }

    /// The three places the arc stands on, which is all the curve is made of.
    pub fn arc_draft(&self, id: ArcId) -> ArcDraft {
        let arc = self.arcs[id.0];
        ArcDraft {
            centre: self.point(arc.center),
            start: self.point(arc.start),
            end: self.point(arc.end),
        }
    }

    pub fn arcs(&self) -> &[Arc] {
        &self.arcs
    }

    pub fn arc(&self, id: ArcId) -> Arc {
        self.arcs[id.0]
    }

    pub fn is_erased_arc(&self, id: ArcId) -> bool {
        Erased::holds(&self.erased.arcs, id.0)
    }

    pub fn live_arcs(&self) -> impl Iterator<Item = (ArcId, Arc)> + '_ {
        self.arcs
            .iter()
            .enumerate()
            .map(|(rank, arc)| (ArcId(rank), *arc))
            .filter(|(id, _)| !self.is_erased_arc(*id))
    }

    /// The arcs a point holds up: an arc missing its centre or either of its
    /// ends is not geometry, so erasing one erases them the way erasing a point
    /// already takes its segments.
    pub(crate) fn arcs_leaning_on(&self, point: PointId) -> Vec<Element> {
        self.live_arcs()
            .filter(|(_, arc)| arc.center == point || arc.start == point || arc.end == point)
            .map(|(id, _)| Element::Arc(id))
            .collect()
    }

    /// The two ends of every arc centred on `point`, which are meant to move
    /// with it the way a circle's rim follows its centre: dragging the centre
    /// alone would otherwise leave the ends behind and stretch the curve.
    pub fn arc_ends_around(&self, point: PointId) -> Vec<PointId> {
        self.live_arcs()
            .filter(|(_, arc)| arc.center == point)
            .flat_map(|(_, arc)| [arc.start, arc.end])
            .collect()
    }

    /// How far the curve sits from its centre, read off the end it starts at.
    pub fn arc_radius(&self, id: ArcId) -> f64 {
        let arc = self.arcs[id.0];
        self.point(arc.center).distance(self.point(arc.start))
    }

    /// The curve as a run of places, ends included.
    pub fn arc_polyline(&self, id: ArcId) -> Vec<DVec2> {
        places_along(self.arc_draft(id))
    }

    /// Halfway along the curve, which is where a mark about the whole arc
    /// belongs: its centre can sit far outside the drawing, and its ends are
    /// shared with whatever else runs into them.
    pub fn arc_midpoint(&self, id: ArcId) -> DVec2 {
        let arc = self.arcs[id.0];
        let centre = self.point(arc.center);
        let from = (self.point(arc.start) - centre).to_angle();
        centre + DVec2::from_angle(from + self.arc_sweep(id) * 0.5) * self.arc_radius(id)
    }

    pub fn arc_steps(&self, id: ArcId) -> usize {
        steps_along(self.arc_draft(id))
    }
}

impl Sketch {
    /// What being an arc asks of the drawing, one equation per arc.
    ///
    /// Written for every arc rather than only when a rule asks for it. Three
    /// points are six numbers where an arc has five, and without this one the
    /// spare number is free to wander: dragging an end would leave a shape that
    /// is no longer a piece of any circle.
    pub(crate) fn arc_equations(&self, pinned: &[bool], into: &mut Vec<Equation>) {
        for index in 0..self.arcs.len() {
            self.arc_equation(ArcId(index), pinned, into);
        }
    }

    pub(crate) fn arc_equation(&self, id: ArcId, pinned: &[bool], into: &mut Vec<Equation>) {
        let Some(mut equation) = self.equal_reach_equation(id) else {
            return;
        };
        for (point, pinned) in pinned.iter().enumerate() {
            if *pinned {
                equation.gradient[point * 2] = 0.0;
                equation.gradient[point * 2 + 1] = 0.0;
            }
        }
        into.push(equation);
    }

    /// The two ends of an arc, held equally far from its centre.
    ///
    /// The same shape as the solver's `rim_equation` without the radius column:
    /// an arc keeps no radius of its own to give on, so the whole correction
    /// falls on the three points.
    fn equal_reach_equation(&self, id: ArcId) -> Option<Equation> {
        let arc = *self.arcs.get(id.0)?;
        if self.is_erased_arc(id) {
            return None;
        }
        let centre = self.point(arc.center);
        let (to_start, to_end) = (self.point(arc.start) - centre, self.point(arc.end) - centre);
        let (near, far) = (to_start.length(), to_end.length());
        if near < 1e-9 || far < 1e-9 {
            return None;
        }
        let (from_start, from_end) = (to_start / near, to_end / far);

        let mut equation = Equation::new(self.variables());
        equation.error = far - near;
        equation.add(arc.end, from_end);
        equation.add(arc.start, -from_start);
        equation.add(arc.center, from_start - from_end);
        Some(equation)
    }
}

#[cfg(test)]
mod tests;
