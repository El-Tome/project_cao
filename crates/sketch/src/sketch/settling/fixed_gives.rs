//! A value or a rule that only fixed points stand in the way of: they give,
//! as few of them as can, and stay fixed wherever they land.

use super::super::{LengthOutcome, PointId, Sketch};
use crate::constraints::Constraint;
use crate::solver::TOLERANCE;

/// How many sets of fixed points are tried before the value is refused. Each
/// try is a settle; a drawing with many fixed points would otherwise pay for
/// every combination of them.
const TRIES: usize = 64;

impl Sketch {
    /// Whether a fixed point is let go of while a value lands that only it
    /// stands in the way of.
    pub(crate) fn is_let_go(&self, point: PointId) -> bool {
        self.held.released.contains(&point)
    }

    /// Settles with `land` as it is, then — when that leaves something untrue —
    /// with fixed points let go of, the fewest first, until one set lands. The
    /// Fixed rules stay: a point that gave is fixed where it now stands.
    pub(super) fn landing_or_giving(
        &mut self,
        millimeters_per_unit: f64,
        mut land: impl FnMut(&mut Self) -> LengthOutcome,
    ) -> LengthOutcome {
        let kept = self.shapes_now();
        let outcome = land(self);
        if outcome == LengthOutcome::Exact {
            return outcome;
        }
        let refused = self.shapes_now();
        self.give_back(kept.clone());
        let reached = self.reached_from_what_is_untrue(millimeters_per_unit);
        for released in self.what_fixed_can_give(&reached) {
            self.give_back(kept.clone());
            self.held.released = released;
            let landed = land(self);
            self.held.released.clear();
            if landed == LengthOutcome::Exact {
                return landed;
            }
        }
        self.give_back(refused);
        outcome
    }

    /// The sets of fixed points to let go of, in the order they are tried: one
    /// point before two, and among as many, those that should stay last — the
    /// nearest the origin, then the one fixed first. The origin is never in
    /// them, nor a fixed point nothing untrue reaches.
    fn what_fixed_can_give(&self, reached: &[bool]) -> Vec<Vec<PointId>> {
        let giving_first: Vec<PointId> = self
            .fixed_giving_first()
            .into_iter()
            .filter(|point| reached[point.0])
            .collect();
        let mut sets = Vec::new();
        for size in 1..=giving_first.len() {
            let mut chosen: Vec<usize> = (0..size).collect();
            loop {
                if sets.len() == TRIES {
                    return sets;
                }
                sets.push(chosen.iter().map(|index| giving_first[*index]).collect());
                if !next_combination(&mut chosen, giving_first.len()) {
                    break;
                }
            }
        }
        sets
    }

    /// Every fixed point but the origin, the one that should give first ahead.
    fn fixed_giving_first(&self) -> Vec<PointId> {
        let mut fixed: Vec<(PointId, usize)> = Vec::new();
        for (order, rule) in self.constraints().iter().enumerate() {
            let Constraint::Fixed { element } = rule else {
                continue;
            };
            for point in self.points_it_leans_on(*element) {
                if !self.out_of_play(point) && fixed.iter().all(|(each, _)| *each != point) {
                    fixed.push((point, order));
                }
            }
        }
        // Read to a millionth of a unit, so that two points drawn as far out
        // tie and the order they were fixed in decides.
        let origin = self.point(Sketch::ORIGIN);
        let far = |point: PointId| (self.point(point).distance(origin) * 1e6).round() as i64;
        fixed
            .sort_by(|(a, a_order), (b, b_order)| far(*b).cmp(&far(*a)).then(b_order.cmp(a_order)));
        fixed.into_iter().map(|(point, _)| point).collect()
    }

    /// The points tied, through traits and through every value and rule, to
    /// what the drawing does not hold yet — read before anything moved, when
    /// that is the value or the rule just laid. A fixed point elsewhere cannot
    /// help it, and trying it would only spend the tries.
    fn reached_from_what_is_untrue(&self, millimeters_per_unit: f64) -> Vec<bool> {
        let count = self.points().len();
        let rows = self.equations_pinned_by(millimeters_per_unit, &vec![false; count]);
        let scale = self.characteristic_size();
        let mut reached = vec![false; count];
        let mut spoken: Vec<Vec<usize>> = Vec::new();
        for row in &rows {
            let points: Vec<usize> = (0..count)
                .filter(|index| {
                    row.gradient[index * 2] != 0.0 || row.gradient[index * 2 + 1] != 0.0
                })
                .collect();
            if row.off_by(scale) >= TOLERANCE {
                for index in &points {
                    reached[*index] = true;
                }
            }
            spoken.push(points);
        }
        spoken.extend(
            self.live_segments()
                .map(|(_, segment)| vec![segment.start.0, segment.end.0]),
        );
        loop {
            let mut grew = false;
            for points in &spoken {
                if points.iter().any(|index| reached[*index]) {
                    for index in points {
                        grew |= !reached[*index];
                        reached[*index] = true;
                    }
                }
            }
            if !grew {
                return reached;
            }
        }
    }
}

/// Steps `chosen`, a strictly rising set of indices below `count`, to the next
/// one in order. False once it was the last.
fn next_combination(chosen: &mut [usize], count: usize) -> bool {
    let size = chosen.len();
    for slot in (0..size).rev() {
        if chosen[slot] < count - size + slot {
            chosen[slot] += 1;
            for after in slot + 1..size {
                chosen[after] = chosen[after - 1] + 1;
            }
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests;
