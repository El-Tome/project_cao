//! Which points make up the shape a dragged point belongs to, and which of
//! them stays where it is while the hand pulls.

use glam::DVec2;

use crate::constraints::Constraint;
use crate::sketch::{PointId, SegmentId, Sketch};

impl Sketch {
    /// Every pair of points the drawing joins: the two ends of a trait, the
    /// centre of an arc to each of its ends, an ellipse's centre to the ends
    /// of its axes, a point to what a rule holds it on, and a tangency's
    /// touch to both the things that touch.
    pub(crate) fn joined_pairs(&self) -> Vec<(PointId, PointId)> {
        let mut pairs: Vec<(PointId, PointId)> = self
            .live_segments()
            .map(|(_, line)| (line.start, line.end))
            .collect();
        for (_, arc) in self.live_arcs() {
            pairs.extend([(arc.center, arc.start), (arc.center, arc.end)]);
        }
        for (id, ellipse) in self.live_ellipses() {
            let [centre, ends @ ..] = self.ellipse_points(id);
            pairs.extend(ends.map(|end| (centre, end)));
            if let Some((from, to)) = ellipse.drawn {
                pairs.extend([(centre, from), (centre, to)]);
            }
        }
        let end_of = |segment: SegmentId| self.segments().get(segment.0).map(|line| line.start);
        for constraint in self.constraints() {
            let (point, other) = match *constraint {
                Constraint::OnSegment { point, segment }
                | Constraint::Midpoint { point, segment } => (Some(point), end_of(segment)),
                Constraint::OnCircle { point, circle } => (
                    Some(point),
                    self.circles().get(circle.0).map(|round| round.center),
                ),
                Constraint::OnArc { point, arc } => (
                    Some(point),
                    self.arcs().get(arc.0).map(|curve| curve.center),
                ),
                Constraint::OnEllipse { point, ellipse } => (
                    Some(point),
                    self.ellipses().get(ellipse.0).map(|curve| curve.center),
                ),
                Constraint::Tangent {
                    circle,
                    segment,
                    at,
                } => {
                    let centre = self.circles().get(circle.0).map(|round| round.center);
                    pairs.extend(at.zip(end_of(segment)));
                    (centre, end_of(segment))
                }
                Constraint::ArcTangent { arc, segment, at } => {
                    let centre = self.arcs().get(arc.0).map(|curve| curve.center);
                    pairs.extend(at.zip(end_of(segment)));
                    (centre, end_of(segment))
                }
                Constraint::EllipseTangent {
                    ellipse,
                    segment,
                    at,
                } => {
                    let centre = self.ellipses().get(ellipse.0).map(|curve| curve.center);
                    pairs.extend(at.zip(end_of(segment)));
                    (centre, end_of(segment))
                }
                _ => (None, None),
            };
            pairs.extend(point.zip(other));
        }
        let count = self.points().len();
        pairs.retain(|(one, other)| one.0 < count && other.0 < count);
        pairs
    }

    /// The points joined to `point`, however far round, `point` among them,
    /// in the order they were drawn.
    pub(crate) fn shape_of(&self, point: PointId) -> Vec<PointId> {
        self.steps_from(point, &self.joined_pairs())
            .into_iter()
            .enumerate()
            .filter_map(|(index, steps)| steps.map(|_| PointId(index)))
            .collect()
    }

    /// How many joins away from `point` each point stands, by rank; nothing
    /// for a point it does not reach.
    fn steps_from(&self, point: PointId, pairs: &[(PointId, PointId)]) -> Vec<Option<usize>> {
        let count = self.points().len();
        let mut next_to: Vec<Vec<usize>> = vec![Vec::new(); count];
        for (one, other) in pairs {
            next_to[one.0].push(other.0);
            next_to[other.0].push(one.0);
        }
        let mut steps: Vec<Option<usize>> = vec![None; count];
        if point.0 >= count {
            return steps;
        }
        steps[point.0] = Some(0);
        let mut front = vec![point.0];
        let mut taken = 0;
        while !front.is_empty() {
            taken += 1;
            let mut next = Vec::new();
            for from in front {
                for to in &next_to[from] {
                    if steps[*to].is_none() {
                        steps[*to] = Some(taken);
                        next.push(*to);
                    }
                }
            }
            front = next;
        }
        steps
    }

    /// Whether a point is the centre of a circle, an arc or an ellipse: what
    /// a drag of it carries the curve by.
    pub(crate) fn is_a_centre(&self, point: PointId) -> bool {
        self.live_circles().any(|(_, round)| round.center == point)
            || self.live_arcs().any(|(_, arc)| arc.center == point)
            || self
                .live_ellipses()
                .any(|(_, ellipse)| ellipse.center == point)
    }

    /// The centres of the curves a point lies on: an arc it is an end of, an
    /// ellipse whose axis ends on it or whose cut ends on it, and any curve a
    /// rule holds it on. They stay where they are while it is dragged.
    pub(crate) fn centres_under(&self, point: PointId) -> Vec<PointId> {
        let mut centres: Vec<PointId> = Vec::new();
        for (_, arc) in self.live_arcs() {
            if arc.start == point || arc.end == point {
                centres.push(arc.center);
            }
        }
        for (id, ellipse) in self.live_ellipses() {
            let on_its_axes = self.ellipse_points(id)[1..].contains(&point);
            let on_its_cut = ellipse
                .drawn
                .is_some_and(|(from, to)| from == point || to == point);
            if on_its_axes || on_its_cut {
                centres.push(ellipse.center);
            }
        }
        for constraint in self.constraints() {
            let centre = match *constraint {
                Constraint::OnCircle {
                    point: held,
                    circle,
                } if held == point => self.circles().get(circle.0).map(|round| round.center),
                Constraint::OnArc { point: held, arc } if held == point => {
                    self.arcs().get(arc.0).map(|curve| curve.center)
                }
                Constraint::OnEllipse {
                    point: held,
                    ellipse,
                } if held == point => self.ellipses().get(ellipse.0).map(|curve| curve.center),
                _ => None,
            };
            centres.extend(centre);
        }
        centres.retain(|centre| *centre != point);
        centres.sort_by_key(|centre| centre.0);
        centres.dedup();
        centres
    }

    /// The point of a shape that stays where it is while `point` is pulled,
    /// when nothing else holds the shape: the one farthest from the hand.
    ///
    /// Farthest counted first in joins, along the traits a rule of direction
    /// ties together — that is what makes it a rectangle's opposite corner,
    /// whatever free tail hangs off it — and only then in distance, from where
    /// the point stood when the press landed. A curve's centre, an ellipse's
    /// axis ends and the touch of a tangency are never it: they are how a
    /// curve is held, not where a shape stands.
    pub(crate) fn stay_point(&self, point: PointId, shape: &[PointId]) -> Option<PointId> {
        let tied: Vec<(PointId, PointId)> = self
            .tied_by_direction(shape)
            .into_iter()
            .map(|segment| {
                let line = self.segments()[segment.0];
                (line.start, line.end)
            })
            .collect();
        let mut aside: Vec<PointId> = self
            .live_ellipses()
            .flat_map(|(id, _)| self.ellipse_points(id)[1..].to_vec())
            .collect();
        aside.extend(self.constraints().iter().filter_map(|rule| match *rule {
            Constraint::Tangent { at, .. }
            | Constraint::ArcTangent { at, .. }
            | Constraint::EllipseTangent { at, .. } => at,
            _ => None,
        }));
        let candidates = |steps: Vec<Option<usize>>| -> Vec<(PointId, usize)> {
            steps
                .into_iter()
                .enumerate()
                .filter_map(|(index, steps)| steps.map(|steps| (PointId(index), steps)))
                .filter(|(each, _)| {
                    *each != point
                        && !self.is_erased_point(*each)
                        && !self.is_a_centre(*each)
                        && !aside.contains(each)
                })
                .collect()
        };
        let mut pool = candidates(self.steps_from(point, &tied));
        if pool.is_empty() {
            pool = candidates(self.steps_from(point, &self.joined_pairs()));
        }

        let from = self.point(point);
        let tie = self.drawing_size() * 1e-9;
        let mut best: Option<(PointId, usize, f64)> = None;
        for (each, steps) in pool {
            let far = self.point(each).distance(from);
            let wins = match best {
                None => true,
                Some((_, best_steps, best_far)) => {
                    steps > best_steps || (steps == best_steps && far > best_far + tie)
                }
            };
            if wins {
                best = Some((each, steps, far));
            }
        }
        best.map(|(each, _, _)| each)
    }

    /// The place a shape turns about when it has to: the one point holding
    /// it, or the centre of the one curve the dragged point lies on, or the
    /// point that stays. Two different places holding it leave it nothing to
    /// turn about.
    pub(crate) fn turned_about(
        &self,
        fixed: &[PointId],
        centres: &[PointId],
        stay: Option<PointId>,
    ) -> Option<PointId> {
        let mut places: Vec<(PointId, DVec2)> = fixed
            .iter()
            .chain(centres)
            .map(|each| (*each, self.point(*each)))
            .collect();
        places.dedup_by(|one, other| one.1.distance(other.1) < 1e-9);
        match places.as_slice() {
            [] => stay,
            [(only, _)] => Some(*only),
            [(first, at), rest @ ..] => rest
                .iter()
                .all(|(_, place)| place.distance(*at) < 1e-9)
                .then_some(*first),
        }
    }
}
