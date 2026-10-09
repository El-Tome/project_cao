//! A value typed between free points: the one nearest the origin stays where
//! it is, and the rest of the drawing comes to it.

use glam::DVec2;

use super::{LengthOutcome, PointId, Sketch};
use crate::constraints::DimensionTarget;
use crate::sketch::Element;
use crate::solver::TOLERANCE;

impl Sketch {
    /// The free point a value typed leaves where it is, for as long as it
    /// lands.
    pub(crate) fn staying_while_it_lands(&self) -> Vec<PointId> {
        self.held.stays.clone()
    }

    /// Settles a value that names no trait to hold, the point of it nearest
    /// the origin left where it is. Without it, the correction spreads over
    /// every free point: a trait alone moves both its ends by half, and a
    /// shape whose width is retyped changes a height nobody typed.
    ///
    /// The value is a movement before it is an equation: the far point is put
    /// where the value says, what hangs on it carried along, and the far point
    /// held there, so the solver has little or nothing left to mend — nothing
    /// to spread, and no height let drift with a width.
    /// When that leaves the drawing no way to hold, the far point is let go,
    /// then the near one, and whatever outranks them — the origin, a fixed
    /// point — decides, as it did before.
    pub(super) fn land_near(
        &mut self,
        target: Option<DimensionTarget>,
        millimeters_per_unit: f64,
    ) -> LengthOutcome {
        let Some((near, far)) =
            target.and_then(|target| self.nearest_of(target, millimeters_per_unit))
        else {
            return self.resolve_keeping_places(millimeters_per_unit);
        };
        let kept = self.shapes_now();
        let tries = match far {
            Some(_) => vec![far, None],
            None => vec![None],
        };
        for far in tries {
            let held: Vec<PointId> = far.iter().map(|(point, _)| *point).collect();
            let still: Vec<PointId> = held.iter().copied().chain([near]).collect();
            // Read on the drawing as it was: a point held on a trait keeps its
            // place along the trait before the value, not after it (#446).
            let shares = self.shares(&still, millimeters_per_unit);
            if let Some((point, position)) = far {
                self.stretch_to_the_value(near, point, position, millimeters_per_unit);
            }
            self.held.stays = vec![near];
            let landed = self.settle_held(held, Vec::new(), millimeters_per_unit);
            self.held.stays.clear();
            if landed {
                self.keep_shares(&shares, &still, &[], millimeters_per_unit);
                return LengthOutcome::Exact;
            }
            self.give_back(kept.clone());
        }
        self.resolve_keeping_places(millimeters_per_unit)
    }

    /// Takes `far` to `to`, the shape it belongs to stretched along the way it
    /// goes: what lies level with `near` or behind it stays, what lies level
    /// with `far` or beyond it goes the whole way, and what lies between goes
    /// part of it. A far side goes with its corner, and a height stays a
    /// height. Another shape the value reaches travels whole.
    ///
    /// Where that stretch breaks something already decided — a side typed
    /// beside the one the value stretches — the far point goes alone instead,
    /// if that leaves the solver nothing to mend. Mending the stretch would
    /// move a corner nobody asked to move (#530).
    ///
    /// What stays for good does not move, and the solver takes it from there.
    fn stretch_to_the_value(
        &mut self,
        near: PointId,
        far: PointId,
        to: DVec2,
        millimeters_per_unit: f64,
    ) {
        let (from, shift) = (self.point(near), to - self.point(far));
        let reach = self.point(far) - from;
        let way = shift.try_normalize().unwrap_or(DVec2::ZERO);
        let way = way * way.dot(reach).signum();
        let span = reach.dot(way);
        if span < 1e-9 {
            self.move_point(far, to);
            return;
        }
        let groups = self.shapes_joined();
        let apart = groups[near.0] != groups[far.0];
        let stays = self.points_that_stay();
        let drawn = self.shapes_now();
        for index in 0..self.points().len() {
            let point = PointId(index);
            if stays[index] || point == near || self.out_of_play(point) {
                continue;
            }
            let share = match groups[index] {
                group if apart && group == groups[far.0] => 1.0,
                group if !apart && group == groups[near.0] => {
                    ((self.point(point) - from).dot(way) / span).clamp(0.0, 1.0)
                }
                _ => continue,
            };
            self.place_point(point, self.point(point) + shift * share);
        }
        if apart || self.leaves_nothing_to_mend(millimeters_per_unit) {
            return;
        }

        let stretched = self.shapes_now();
        self.give_back(drawn);
        self.move_point(far, to);
        if !self.leaves_nothing_to_mend(millimeters_per_unit) {
            self.give_back(stretched);
        }
    }

    fn leaves_nothing_to_mend(&self, millimeters_per_unit: f64) -> bool {
        self.worst_error(millimeters_per_unit, self.characteristic_size()) < TOLERANCE
    }

    /// Which shape each point belongs to, as `point_groups` reads them, with
    /// what an arc or an ellipse stands on joined to where it hangs from: an
    /// arc off the end of a trait goes with that end.
    fn shapes_joined(&self) -> Vec<usize> {
        let mut group = self.point_groups();
        fn root(group: &mut [usize], mut point: usize) -> usize {
            while group[point] != point {
                group[point] = group[group[point]];
                point = group[point];
            }
            point
        }
        let curves: Vec<Element> = self
            .live_arcs()
            .map(|(id, _)| Element::Arc(id))
            .chain(self.live_ellipses().map(|(id, _)| Element::Ellipse(id)))
            .collect();
        for curve in curves {
            let points = self.points_it_leans_on(curve);
            for pair in points.windows(2) {
                let (a, b) = (root(&mut group, pair[0].0), root(&mut group, pair[1].0));
                group[a] = b;
            }
        }
        (0..group.len())
            .map(|point| root(&mut group, point))
            .collect()
    }

    /// The point of a value that stays while it lands — the one nearest the
    /// origin, or of two as near, the one drawn first — and where the value
    /// puts the other, when it names a way to go. None when a point of it
    /// already stays for good, which ranks above.
    fn nearest_of(
        &self,
        target: DimensionTarget,
        millimeters_per_unit: f64,
    ) -> Option<(PointId, Option<(PointId, DVec2)>)> {
        let (points, axis) = match target {
            DimensionTarget::Length(segment) => {
                let line = self.segments()[segment.0];
                (vec![line.start, line.end], None)
            }
            DimensionTarget::Distance { from, to } => (vec![from, to], None),
            DimensionTarget::Projected { from, to, axis } => (vec![from, to], Some(axis)),
            DimensionTarget::PointToSegment { point, segment } => {
                let line = self.segments()[segment.0];
                (vec![point, line.start, line.end], None)
            }
            _ => return None,
        };
        let stays = self.points_that_stay();
        if points.iter().any(|point| stays[point.0]) {
            return None;
        }
        let near = self.nearest_the_origin(&points)?;
        if points.len() != 2 {
            return Some((near, None));
        }
        let other = points.into_iter().find(|point| *point != near)?;
        let value = self.dimension_of(target)?.value / millimeters_per_unit.max(1e-9);
        let span = self.point(other) - self.point(near);
        let moved = match axis {
            None => self.point(near) + span.try_normalize()? * value,
            Some(axis) => {
                let along = span.dot(axis.direction());
                let way = if along < 0.0 { -1.0 } else { 1.0 };
                self.point(other) + axis.direction() * (way * value - along)
            }
        };
        Some((near, Some((other, moved))))
    }

    /// The point of `points` nearest the origin, or of two as near, the one
    /// drawn first: what stays while a value, or a rule turning a shape, lands.
    pub(super) fn nearest_the_origin(&self, points: &[PointId]) -> Option<PointId> {
        // Read to a millionth of a unit, so that two points drawn as far out
        // tie and the one drawn first stays.
        let origin = self.point(Sketch::ORIGIN);
        let far = |point: &PointId| (self.point(*point).distance(origin) * 1e6).round() as i64;
        points
            .iter()
            .copied()
            .filter(|point| !self.out_of_play(*point))
            .min_by_key(|point| (far(point), point.0))
    }
}

#[cfg(test)]
mod tests;
