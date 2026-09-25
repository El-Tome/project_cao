//! Which points make up the shape a dragged point belongs to, and which of
//! them stays where it is while the hand pulls.

use glam::DVec2;

use crate::constraints::Constraint;
use crate::sketch::{PointId, SegmentId, Sketch};

impl Sketch {
    /// Every pair of points the drawing joins: the two ends of a trait, the
    /// centre of an arc to each of its ends, an ellipse's centre to the ends
    /// of its axes, a point to what a rule holds it on, and a tangency's
    /// curve and touch to the trait they brush.
    ///
    /// What lies on a trait is joined to both its ends: joined to the start
    /// alone, it would count as nearer the start than the end is, and a drag
    /// of the end would keep it rather than the far end.
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
        let ends_of = |segment: SegmentId| -> Vec<PointId> {
            self.segments()
                .get(segment.0)
                .map(|line| vec![line.start, line.end])
                .unwrap_or_default()
        };
        let circle_centre = |circle: crate::sketch::CircleId| {
            self.circles().get(circle.0).map(|round| round.center)
        };
        let arc_centre = |arc: crate::arc::ArcId| self.arcs().get(arc.0).map(|curve| curve.center);
        let ellipse_centre = |ellipse: crate::ellipse::EllipseId| {
            self.ellipses().get(ellipse.0).map(|curve| curve.center)
        };
        for constraint in self.constraints() {
            let (joined, to): (Vec<PointId>, Vec<PointId>) = match *constraint {
                Constraint::OnSegment { point, segment }
                | Constraint::Midpoint { point, segment } => (vec![point], ends_of(segment)),
                Constraint::OnCircle { point, circle } => {
                    (vec![point], circle_centre(circle).into_iter().collect())
                }
                Constraint::OnArc { point, arc } => {
                    (vec![point], arc_centre(arc).into_iter().collect())
                }
                Constraint::OnEllipse { point, ellipse } => {
                    (vec![point], ellipse_centre(ellipse).into_iter().collect())
                }
                Constraint::Tangent {
                    circle,
                    segment,
                    at,
                } => (
                    circle_centre(circle).into_iter().chain(at).collect(),
                    ends_of(segment),
                ),
                Constraint::ArcTangent { arc, segment, at } => (
                    arc_centre(arc).into_iter().chain(at).collect(),
                    ends_of(segment),
                ),
                Constraint::EllipseTangent {
                    ellipse,
                    segment,
                    at,
                } => (
                    ellipse_centre(ellipse).into_iter().chain(at).collect(),
                    ends_of(segment),
                ),
                _ => (Vec::new(), Vec::new()),
            };
            for one in &joined {
                pairs.extend(to.iter().map(|other| (*one, *other)));
            }
        }
        let count = self.points().len();
        pairs.retain(|(one, other)| one.0 < count && other.0 < count);
        pairs
    }

    /// The points joined to `point`, however far round, `point` among them,
    /// in the order they were drawn.
    ///
    /// A point the drawing holds still — the origin, a fixed point — belongs
    /// to every shape laid on it and joins none of them to the others: two
    /// rectangles drawn from the origin are two shapes.
    pub(crate) fn shape_of(&self, point: PointId) -> Vec<PointId> {
        self.steps_from(point, &self.joined_pairs())
            .into_iter()
            .enumerate()
            .filter_map(|(index, steps)| steps.map(|_| PointId(index)))
            .collect()
    }

    /// The shape that some points of one element belong to — a side's two
    /// ends, a curve's handles — walked from the first of them the drawing does
    /// not hold still: walked from the origin, it would take in every drawing
    /// laid on the origin.
    pub(crate) fn shape_through(&self, points: &[PointId]) -> Vec<PointId> {
        let pinned = self.pinned_points();
        match points
            .iter()
            .find(|point| !pinned.get(point.0).copied().unwrap_or(true))
        {
            Some(free) => self.shape_of(*free),
            None => {
                let mut alone = points.to_vec();
                alone.sort_by_key(|point| point.0);
                alone.dedup();
                alone
            }
        }
    }

    /// How many joins away from `point` each point stands, by rank; nothing
    /// for a point it does not reach. The walk goes to a point the drawing
    /// holds still and no further.
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
        let pinned = self.pinned_points();
        let mut front = vec![point.0];
        let mut taken = 0;
        while !front.is_empty() {
            taken += 1;
            let mut next = Vec::new();
            for from in front {
                if from != point.0 && pinned[from] {
                    continue;
                }
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

    /// Whether a point is a curve's centre and nothing else — no trait ends
    /// on it. A circle snapped onto a rectangle's corner leaves the corner a
    /// corner, which is dragged as one and carries the circle with it.
    pub(crate) fn only_a_centre(&self, point: PointId) -> bool {
        self.is_a_centre(point)
            && !self
                .live_segments()
                .any(|(_, line)| line.start == point || line.end == point)
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
    /// Taken among the points lying on the traits a rule of direction ties —
    /// their ends, and what is held on them — which is what makes it a
    /// rectangle's opposite corner whatever free tail hangs off it; from the
    /// whole shape only when no trait is tied. Farthest counted in joins, then
    /// in distance from where the point stood when the press landed. A point
    /// that is only a curve's centre, an ellipse's axis end and the touch of a
    /// tangency are never it: they are how a curve is held, not where a shape
    /// stands.
    pub(crate) fn stay_point(&self, point: PointId, shape: &[PointId]) -> Option<PointId> {
        let aside = self.held_aside();
        let steps = self.steps_from(point, &self.joined_pairs());
        let eligible = |each: &PointId| {
            *each != point
                && !self.is_erased_point(*each)
                && !self.only_a_centre(*each)
                && !aside.contains(each)
        };
        let ranked = |among: &[PointId]| -> Vec<(PointId, usize)> {
            among
                .iter()
                .filter(|each| eligible(each))
                .filter_map(|each| steps.get(each.0).copied().flatten().map(|far| (*each, far)))
                .collect()
        };
        let mut pool = ranked(&self.points_on(&self.tied_by_direction(shape)));
        if pool.is_empty() {
            pool = ranked(shape);
        }

        let from = self.point(point);
        let tie = self.drawing_size() * 1e-9;
        let mut best: Option<(PointId, usize, f64)> = None;
        for (each, joins) in pool {
            let far = self.point(each).distance(from);
            let wins = match best {
                None => true,
                Some((_, best_joins, best_far)) => {
                    joins > best_joins || (joins == best_joins && far > best_far + tie)
                }
            };
            if wins {
                best = Some((each, joins, far));
            }
        }
        best.map(|(each, _, _)| each)
    }

    /// The ends of an ellipse's axes and the touches of tangencies: held by a
    /// curve, never where a shape stands.
    pub(crate) fn held_aside(&self) -> Vec<PointId> {
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
        aside
    }

    /// The points lying on some traits: their ends, and the points a rule
    /// holds on them.
    pub(crate) fn points_on(&self, traits: &[SegmentId]) -> Vec<PointId> {
        let mut points: Vec<PointId> = traits
            .iter()
            .filter_map(|segment| self.segments().get(segment.0))
            .flat_map(|line| [line.start, line.end])
            .collect();
        points.extend(self.constraints().iter().filter_map(|rule| match *rule {
            Constraint::OnSegment { point, segment } if traits.contains(&segment) => Some(point),
            _ => None,
        }));
        points.sort_by_key(|point| point.0);
        points.dedup();
        points
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
