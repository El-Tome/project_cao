//! What keeps a drawing the way up it was drawn while it settles, and the rule
//! a drawing is never asked to state: that it does not turn on the spot.

use glam::DVec2;

use crate::constraints::Constraint;
use crate::equation::Equation;
use crate::independence::turns_nothing;
use crate::sketch::{PointId, Sketch};

/// The way round a group was sitting before a settle, and what it is turned
/// back about.
pub(super) struct WayRound {
    members: Vec<usize>,
    from: PointId,
    to: PointId,
    angle: f64,
    about: DVec2,
}

/// How far off an axis a trait may lean and still count as square to the
/// sketch, which is what lets a shape keep its orientation without being told.
const SQUARE_DEGREES: f64 = 0.5;

impl Sketch {
    /// The direction each free-to-turn group is sitting at, before anything
    /// moves.
    ///
    /// A group nothing holds upright can be spun without breaking a single
    /// dimension, so the solver is free to spin it — and it does: the steps are
    /// finite, and what each of them leaves behind adds up. A rectangle whose
    /// height is changed came out several degrees off, still reporting itself
    /// fully constrained, because it was: it had simply turned.
    ///
    /// Each group is read about the point it turns about, which is not always
    /// the origin: a shape nailed by one corner elsewhere spins about that
    /// corner, and a spin nobody was looking for is a spin nobody undoes.
    pub(super) fn orientations(&self, millimeters_per_unit: f64) -> Vec<WayRound> {
        let equations = self.equations(millimeters_per_unit);
        let pinned = self.pinned_points();
        let joined = self.point_groups();
        let groups = self.turning_groups(&equations, &pinned, &self.turning_points(&joined));
        let turning = self.turning_points(&groups);

        self.rotation_gauges(&groups, &pinned, &turning)
            .into_iter()
            .filter(|(_, gauge)| {
                equations
                    .iter()
                    .all(|equation| turns_nothing(equation, gauge))
            })
            .filter_map(|(owner, _)| {
                let (from, to) = self.orientation_pair(owner, &groups)?;
                let span = self.point(to) - self.point(from);
                (span.length() > 1e-6).then(|| WayRound {
                    members: (0..groups.len())
                        .filter(|index| groups[*index] == owner)
                        .collect(),
                    from,
                    to,
                    angle: span.to_angle(),
                    about: turning[owner],
                })
            })
            .collect()
    }

    /// The rows that grant each group lying square the way up it was drawn,
    /// for the verdict "entirely constrained" — one per group free to turn
    /// about the origin.
    ///
    /// A group standing alone is granted it as it always was, by a turn of
    /// the whole group, when it holds a trait along an axis. Groups a rule
    /// joins into one turn together (#471), and only the shape drawn from the
    /// origin grants them a way up, by holding its own trait along an axis:
    /// a turn of the whole could be offset by a free shape's travel, and a
    /// level trait borrowed from another shape would say which way up a
    /// leaning one is when nothing does.
    pub(super) fn ways_up_granted(
        &self,
        equations: &[Equation],
        anchored: &[bool],
    ) -> Vec<Equation> {
        let about_the_origin = vec![DVec2::ZERO; self.points().len()];
        let joined = self.point_groups();
        let groups = self.turning_groups(equations, anchored, &about_the_origin);
        let square = self.traits_lying_square(&joined);
        let origin = joined[Sketch::ORIGIN.0];
        self.rotation_gauges(&groups, anchored, &about_the_origin)
            .into_iter()
            // A group already measured against an axis says which way up it
            // is; granting it on top would take that freedom twice.
            .filter(|(_, gauge)| {
                equations
                    .iter()
                    .all(|equation| turns_nothing(equation, gauge))
            })
            .filter_map(|(owner, gauge)| {
                let mut members = (0..joined.len())
                    .filter(|index| groups[*index] == owner && !anchored[*index])
                    .map(|index| joined[index]);
                let first = members.next()?;
                if members.all(|member| member == first) {
                    return square
                        .iter()
                        .any(|(group, _)| *group == first)
                        .then_some(gauge);
                }
                if groups[Sketch::ORIGIN.0] != owner {
                    return None;
                }
                let (_, (start, end)) = square.iter().find(|(group, _)| *group == origin)?;
                let span = self.point(*end) - self.point(*start);
                let mut row = Equation::new(self.variables());
                row.add(*end, span.perp());
                row.add(*start, -span.perp());
                for point in [*start, *end].into_iter().filter(|point| anchored[point.0]) {
                    row.gradient[point.0 * 2] = 0.0;
                    row.gradient[point.0 * 2 + 1] = 0.0;
                }
                Some(row)
            })
            .collect()
    }

    /// Which group each point turns with: joined geometry, and further, the
    /// groups a value or a rule ties so that one cannot turn without the
    /// other. A parallel between two shapes turns both or neither, so the two
    /// keep one way up between them, not none (#471).
    ///
    /// `about` is indexed as `point_groups` names the groups.
    fn turning_groups(
        &self,
        equations: &[Equation],
        pinned: &[bool],
        about: &[DVec2],
    ) -> Vec<usize> {
        let joined = self.point_groups();
        let gauges = self.rotation_gauges(&joined, pinned, about);
        let mut parent: Vec<usize> = (0..joined.len()).collect();
        fn root(parent: &mut [usize], mut group: usize) -> usize {
            while parent[group] != group {
                parent[group] = parent[parent[group]];
                group = parent[group];
            }
            group
        }
        for equation in equations {
            if gauges
                .iter()
                .all(|(_, gauge)| turns_nothing(equation, gauge))
            {
                continue;
            }
            // Every group the equation speaks of, not only those it turns: an
            // arc's centre standing on the point its group turns about turns
            // nothing, and still has to go round with the arc's ends.
            let mut spoken = (0..joined.len())
                .filter(|index| !pinned[*index])
                .filter(|index| {
                    equation.gradient[index * 2] != 0.0 || equation.gradient[index * 2 + 1] != 0.0
                })
                .map(|index| joined[index]);
            let Some(first) = spoken.next() else {
                continue;
            };
            for other in spoken {
                let (a, b) = (root(&mut parent, first), root(&mut parent, other));
                parent[a] = b;
            }
        }
        joined
            .iter()
            .map(|owner| root(&mut parent, *owner))
            .collect()
    }

    /// The one point nothing ever moves, the origin: what a drawing counts as
    /// decided is measured from it alone. A fixed point keeps its place while
    /// something else changes; it decides nothing.
    pub(crate) fn only_the_origin(&self) -> Vec<bool> {
        (0..self.points().len())
            .map(|index| self.is_origin(PointId(index)))
            .collect()
    }

    /// The points that stay whatever happens: the sketch's own origin, which
    /// is what everything else is measured from, and whatever a `Fixed` rule
    /// nails down — but for a fixed point let go of while a value lands that
    /// only it stood in the way of.
    ///
    /// Not what the hand is holding: that lasts one gesture, and a drawing
    /// turned about the cursor is the drag turning it, which is the hand's to
    /// do.
    pub(crate) fn points_that_stay(&self) -> Vec<bool> {
        let mut stays = self.only_the_origin();
        for constraint in self.constraints() {
            let Constraint::Fixed { element } = constraint else {
                continue;
            };
            for point in self.points_it_leans_on(*element) {
                if point.0 < stays.len() && !self.is_let_go(point) {
                    stays[point.0] = true;
                }
            }
        }
        stays
    }

    /// The point each group turns about: the one point of it that stays for
    /// good, or the origin when it has none.
    ///
    /// The origin rather than a point of the group itself, because a group
    /// nothing holds may still be measured from the origin — a distance typed
    /// from it — and that is a turn about the origin and no other. The price
    /// is that turning such a group back also carries it, the further the
    /// further out it was drawn. Where a drawing free to travel lands when a
    /// value is typed is #455, not this rule.
    ///
    /// A single point, never two: a group nailed at two places cannot turn,
    /// and it is the equations that say so.
    ///
    /// Indexed by the point that stands for a group, as `point_groups` names
    /// them, not by point.
    fn turning_points(&self, groups: &[usize]) -> Vec<DVec2> {
        let stays = self.points_that_stay();
        let mut about = vec![DVec2::ZERO; self.points().len()];
        let mut found = vec![false; self.points().len()];

        for index in 0..self.points().len() {
            let owner = groups[index];
            if found[owner] || !stays[index] {
                continue;
            }
            about[owner] = self.point(PointId(index));
            found[owner] = true;
        }
        about
    }

    /// The pair of points whose direction stands for a group's own. The first
    /// trait drawn in it, or — for a lone point — the line from the origin,
    /// which is the only other thing there is to lean on.
    fn orientation_pair(&self, owner: usize, groups: &[usize]) -> Option<(PointId, PointId)> {
        if let Some((_, segment)) = self
            .live_segments()
            .find(|(_, segment)| groups[segment.start.0] == owner)
        {
            return Some((segment.start, segment.end));
        }
        let lone = groups.iter().position(|group| *group == owner)?;
        (lone != Sketch::ORIGIN.0).then_some((Sketch::ORIGIN, PointId(lone)))
    }

    /// Turns each group back the way it was pointing. A rigid turn about the
    /// point the group turns about leaves every dimension of it free to turn
    /// exactly as it found it — that is what "free to turn" means — so this
    /// straightens the drawing without touching what it measures.
    ///
    /// That point stays for good, so where it was read before the settle is
    /// where it still is.
    pub(super) fn hold_orientations(&mut self, held: &[WayRound]) {
        if held.is_empty() {
            return;
        }
        let pinned = self.pinned_points();

        for way in held {
            let span = self.point(way.to) - self.point(way.from);
            if span.length() < 1e-6 {
                continue;
            }
            let drift = wrap(span.to_angle() - way.angle);
            if drift.abs() < 1e-6 {
                continue;
            }
            let turn = DVec2::from_angle(-drift);
            for index in way.members.iter().copied() {
                if pinned[index] {
                    continue;
                }
                let moved = way.about + turn.rotate(self.point(PointId(index)) - way.about);
                self.place_point(PointId(index), moved);
            }
        }
    }

    /// Which group of joined geometry each point belongs to, as the index of a
    /// representative point. Two shapes drawn apart are two groups.
    pub(crate) fn point_groups(&self) -> Vec<usize> {
        let mut group: Vec<usize> = (0..self.points().len()).collect();

        fn root(group: &mut [usize], mut point: usize) -> usize {
            while group[point] != point {
                group[point] = group[group[point]];
                point = group[point];
            }
            point
        }
        for (_, segment) in self.live_segments() {
            let (a, b) = (
                root(&mut group, segment.start.0),
                root(&mut group, segment.end.0),
            );
            group[a] = b;
        }
        (0..group.len())
            .map(|point| root(&mut group, point))
            .collect()
    }

    /// The rule a drawing is never asked to state: that it does not turn on the
    /// spot.
    ///
    /// Spinning a shape about a point of its own leaves every length and every
    /// angle exactly as it was, so no dimension can ever see it. `about` says
    /// which point that is, indexed by the point that stands for a group — not
    /// by point, as `pinned` is.
    ///
    /// One rule per group of joined geometry: two shapes drawn apart turn
    /// independently, so a single shared rule would leave both able to turn
    /// against each other and neither would ever count as settled.
    ///
    /// It carries no error: it never moves anything, it only accounts for a
    /// freedom that is not really there.
    fn rotation_gauges(
        &self,
        groups: &[usize],
        pinned: &[bool],
        about: &[DVec2],
    ) -> Vec<(usize, Equation)> {
        let mut gauges: Vec<(usize, Equation)> = Vec::new();
        for (index, point) in self.points().iter().enumerate() {
            if pinned[index] {
                continue;
            }
            let owner = groups[index];
            let equation = match gauges.iter_mut().find(|(each, _)| *each == owner) {
                Some((_, equation)) => equation,
                None => {
                    gauges.push((owner, Equation::new(self.variables())));
                    &mut gauges.last_mut().expect("just pushed").1
                }
            };
            let arm = *point - about[owner];
            equation.add(PointId(index), DVec2::new(-arm.y, arm.x));
        }

        gauges.retain(|(_, equation)| equation.norm_squared() > 1e-12);
        gauges
    }

    /// A trait along an axis in each group that holds one, as the group's own
    /// index and the trait's two ends.
    ///
    /// Such a trait is what lets a shape keep the way up it was drawn without
    /// being told: square to the sketch is a way up like any other, and the
    /// commonest one. Anything leaning has to carry an angle.
    pub(crate) fn traits_lying_square(&self, groups: &[usize]) -> Vec<(usize, (PointId, PointId))> {
        let mut square: Vec<(usize, (PointId, PointId))> = Vec::new();
        for (_, segment) in self.live_segments() {
            let span = self.point(segment.end) - self.point(segment.start);
            if span.length() < 1e-9 {
                continue;
            }
            let leaning = span.y.atan2(span.x).to_degrees().rem_euclid(90.0);
            if leaning.min(90.0 - leaning) > SQUARE_DEGREES {
                continue;
            }
            let owner = groups[segment.start.0];
            if square.iter().all(|(each, _)| *each != owner) {
                square.push((owner, (segment.start, segment.end)));
            }
        }
        square
    }
}

/// An angle brought back into [-pi, pi], so a drift either side of a turn reads
/// as the small angle it is.
fn wrap(mut angle: f64) -> f64 {
    while angle > std::f64::consts::PI {
        angle -= std::f64::consts::TAU;
    }
    while angle < -std::f64::consts::PI {
        angle += std::f64::consts::TAU;
    }
    angle
}

#[cfg(test)]
mod tests;
