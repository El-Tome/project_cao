//! A piece of a circle, as a centre and the two ends it runs between.

use glam::DVec2;
use serde::{Deserialize, Serialize};

use crate::equation::Equation;
use crate::erased::Erased;
use crate::sketch::{Element, PointId, Sketch};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

    /// How far round the arc goes, in radians, always between zero and a full
    /// turn: the way round is carried by the order of the ends, so the answer
    /// never needs a sign to say it.
    pub fn arc_sweep(&self, id: ArcId) -> f64 {
        let arc = self.arcs[id.0];
        let centre = self.point(arc.center);
        let from = (self.point(arc.start) - centre).to_angle();
        let to = (self.point(arc.end) - centre).to_angle();
        (to - from).rem_euclid(std::f64::consts::TAU)
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

    /// How far the curve sits from its centre, read off the end it starts at.
    pub fn arc_radius(&self, id: ArcId) -> f64 {
        let arc = self.arcs[id.0];
        self.point(arc.center).distance(self.point(arc.start))
    }

    /// The curve as a run of places, ends included.
    ///
    /// How many places is read from the sweep rather than fixed, so that a
    /// small fillet does not become a visible polygon and a long arc does not
    /// cost what a whole circle costs.
    pub fn arc_polyline(&self, id: ArcId) -> Vec<DVec2> {
        let arc = self.arcs[id.0];
        let centre = self.point(arc.center);
        let radius = self.arc_radius(id);
        let sweep = self.arc_sweep(id);
        let from = (self.point(arc.start) - centre).to_angle();
        let steps = self.arc_steps(id);
        (0..=steps)
            .map(|step| {
                let angle = from + sweep * step as f64 / steps as f64;
                centre + DVec2::from_angle(angle) * radius
            })
            .collect()
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
        let turns = self.arc_sweep(id) / std::f64::consts::TAU;
        ((turns * FULL_CIRCLE_STEPS as f64).ceil() as usize).max(2)
    }
}

/// How finely a whole turn would be cut up. An arc takes its share of it.
const FULL_CIRCLE_STEPS: usize = 48;

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
mod tests {
    use super::*;
    use crate::constraints::DimensionTarget;
    use crate::plane::WorkPlane;

    const TOLERANCE: f64 = 1e-9;

    /// What being an arc means, whatever the solver did to get there: the two
    /// ends the same reach from the centre, wherever the centre ended up.
    fn assert_round(sketch: &Sketch, arc: ArcId) {
        let centre = sketch.point(sketch.arc(arc).center);
        let near = centre.distance(sketch.point(sketch.arc(arc).start));
        let far = centre.distance(sketch.point(sketch.arc(arc).end));
        assert!(
            (far - near).abs() < 1e-6 * near.max(1.0),
            "one end {near} from the centre, the other {far}",
        );
    }

    fn quarter() -> (Sketch, ArcId) {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let centre = sketch.add_point(DVec2::ZERO);
        let east = sketch.add_point(DVec2::new(10.0, 0.0));
        let north = sketch.add_point(DVec2::new(0.0, 10.0));
        let arc = sketch.add_arc(centre, east, north);
        (sketch, arc)
    }

    #[test]
    fn an_arc_runs_counter_clockwise_from_its_start_to_its_end() {
        let (sketch, arc) = quarter();
        let sweep = sketch.arc_sweep(arc);

        assert!(
            (sweep - std::f64::consts::FRAC_PI_2).abs() < TOLERANCE,
            "a quarter turn expected, got {sweep}",
        );
    }

    #[test]
    fn a_guide_arc_says_it_is_one_and_an_ordinary_arc_does_not() {
        let (mut sketch, drawn) = quarter();
        let arc = sketch.arc(drawn);
        let guide = sketch.add_construction_arc(arc.center, arc.start, arc.end);

        assert!(!sketch.arc(drawn).construction);
        assert!(sketch.arc(guide).construction);
    }

    #[test]
    fn erasing_a_point_an_arc_leans_on_takes_the_arc_with_it() {
        for which in 0..3 {
            let mut sketch = Sketch::new(WorkPlane::XY);
            let centre = sketch.add_point(DVec2::ZERO);
            let east = sketch.add_point(DVec2::new(10.0, 0.0));
            let north = sketch.add_point(DVec2::new(0.0, 10.0));
            let arc = sketch.add_arc(centre, east, north);
            assert_eq!(sketch.live_arcs().count(), 1);

            sketch.erase(Element::Point([centre, east, north][which]));

            assert!(
                sketch.is_erased_arc(arc),
                "point {which} left the arc behind"
            );
            assert_eq!(sketch.live_arcs().count(), 0);
        }
    }

    #[test]
    fn an_arc_stays_round_when_the_drawing_pulls_one_of_its_ends_out() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let east = sketch.add_point(DVec2::new(10.0, 0.0));
        let north = sketch.add_point(DVec2::new(0.0, 10.0));
        let arc = sketch.add_arc(Sketch::ORIGIN, east, north);

        sketch.set_dimension(
            DimensionTarget::Distance {
                from: Sketch::ORIGIN,
                to: east,
            },
            20.0,
            false,
        );
        sketch.resolve(1.0);

        let start = sketch.point(east).length();
        let end = sketch.point(north).length();
        assert!(
            (start - 20.0).abs() < 1e-2,
            "the end asked for is at {start}"
        );
        assert!(
            (end - start).abs() < 1e-2,
            "the far end stayed at {end} while the near one went to {start}",
        );
        assert!((sketch.arc_radius(arc) - 20.0).abs() < 1e-2);
    }

    #[test]
    fn an_arc_stays_round_when_one_of_its_ends_is_dragged() {
        let (mut sketch, arc) = quarter();
        let dragged = sketch.arc(arc).end;
        let dropped_at = DVec2::new(-3.0, 17.0);
        sketch.settle_around(dragged, dropped_at, 1.0);

        assert!(
            sketch.point(dragged).distance(dropped_at) < 1e-6,
            "the end let go of the cursor, at {:?}",
            sketch.point(dragged),
        );
        assert_round(&sketch, arc);
    }

    #[test]
    fn an_erased_arc_leaves_the_points_it_leaned_on_alone() {
        let (mut sketch, arc) = quarter();
        sketch.erase(Element::Arc(arc));

        assert!(sketch.is_erased_arc(arc));
        assert_eq!(sketch.live_points().count(), 4);
    }

    #[test]
    fn every_place_along_an_arc_is_the_same_reach_from_its_centre() {
        let (sketch, arc) = quarter();
        let places = sketch.arc_polyline(arc);

        assert!(places.len() >= 3, "a quarter turn is not two straight bits");
        for place in &places {
            let reach = place.length();
            assert!(
                (reach - 10.0).abs() < TOLERANCE,
                "reach {reach} at {place:?}"
            );
        }
        assert!(places.first().unwrap().distance(DVec2::new(10.0, 0.0)) < TOLERANCE);
        assert!(places.last().unwrap().distance(DVec2::new(0.0, 10.0)) < TOLERANCE);
    }

    #[test]
    fn a_longer_arc_is_cut_into_more_pieces_than_a_shorter_one() {
        let (sketch, quarter) = quarter();
        let mut wider = Sketch::new(WorkPlane::XY);
        let centre = wider.add_point(DVec2::ZERO);
        let east = wider.add_point(DVec2::new(10.0, 0.0));
        let west = wider.add_point(DVec2::new(-10.0, 0.0));
        let half = wider.add_arc(centre, east, west);

        assert!(wider.arc_steps(half) > sketch.arc_steps(quarter));
    }

    #[test]
    fn the_same_two_ends_the_other_way_round_are_the_rest_of_the_circle() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let centre = sketch.add_point(DVec2::ZERO);
        let east = sketch.add_point(DVec2::new(10.0, 0.0));
        let north = sketch.add_point(DVec2::new(0.0, 10.0));
        let long_way = sketch.add_arc(centre, north, east);
        let sweep = sketch.arc_sweep(long_way);

        assert!(
            (sweep - 3.0 * std::f64::consts::FRAC_PI_2).abs() < TOLERANCE,
            "three quarters expected, got {sweep}",
        );
    }
}
