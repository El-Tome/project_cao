//! A shape turned by the hand: about what, by how much, and how the grid pulls
//! it back square.
//!
//! Turning is the one gesture that changes a shape's way up on purpose. It is
//! reached by sliding a side or a curve along itself, or by taking an
//! ellipse's axis end well off its axis; what it turns is the whole shape the
//! grab belongs to, rigidly, and the rest of the drawing settles around it.

use glam::DVec2;

use crate::arcing::bounds_of;
use crate::independence::turns_nothing;
use crate::length::LengthOutcome;
use crate::sketch::{PointId, SegmentId, Sketch};
use crate::snap::SnapSettings;

/// How near the farthest an end has to stand, as a share of how far that
/// is, to belong to the same far side: a hand-drawn top a hair aslant is one
/// top, and a U's arms a hair apart are still both its top.
const NEARLY: f64 = 0.05;

/// A shape turned about a place, as the hand asked: the points turned, where
/// they turn about, and how far round, in radians — counter-clockwise when
/// positive.
#[derive(Clone, Debug, PartialEq)]
pub struct Turn {
    pub points: Vec<PointId>,
    pub about: DVec2,
    pub angle: f64,
}

impl Sketch {
    /// Turns `points` about `about` by `angle` and holds them there while the
    /// rest of the drawing settles. What cannot be had is given back whole:
    /// the shape stays where it was.
    pub fn turn_shape(
        &mut self,
        points: &[PointId],
        about: DVec2,
        angle: f64,
        millimeters_per_unit: f64,
    ) -> LengthOutcome {
        let points: Vec<PointId> = points
            .iter()
            .copied()
            .filter(|point| point.0 < self.points().len())
            .collect();
        let pinned = self.pinned_points();
        let turn = DVec2::from_angle(angle);
        let moved =
            |sketch: &Self, point: PointId| about + turn.rotate(sketch.point(point) - about);
        if points
            .iter()
            .any(|point| pinned[point.0] && moved(self, *point).distance(self.point(*point)) > 1e-9)
        {
            return LengthOutcome::BestEffort;
        }
        let kept = self.shapes_now();
        for point in &points {
            let place = moved(self, *point);
            self.move_point(*point, place);
        }
        match self.settle_held(points, Vec::new(), millimeters_per_unit) {
            true => LengthOutcome::Exact,
            false => {
                self.give_back(kept);
                LengthOutcome::BestEffort
            }
        }
    }

    /// Where a shape turns about when one of its sides is slid along itself:
    /// the one place that holds it; else the middle of the side standing
    /// opposite the one pulled, or the far corner when none stands opposite —
    /// a triangle's; else the middle of what it draws. Nothing when two
    /// different places hold it, or when a rigid turn about that place would
    /// break one of the drawing's rules — a trait held level, an angle against
    /// an axis, a distance to something else.
    pub(crate) fn turning_centre(
        &self,
        shape: &[PointId],
        side: SegmentId,
        millimeters_per_unit: f64,
    ) -> Option<DVec2> {
        let pinned = self.pinned_points();
        let mut held = shape
            .iter()
            .filter(|point| pinned[point.0])
            .map(|point| self.point(*point));
        let about = match held.next() {
            Some(first) => held
                .all(|place| place.distance(first) < 1e-9)
                .then_some(first)?,
            None => self
                .opposite(shape, side)
                .or_else(|| self.middle_of(shape))?,
        };
        self.turns_freely_about(shape, about, millimeters_per_unit)
            .then_some(about)
    }

    /// The middle of the side of `shape` standing opposite `side`, read off
    /// the ends of its drawn traits — those a rule of direction ties, when any
    /// drawn one is, so that a tail hanging off it does not count — on the
    /// side of `side` the shape stands on.
    ///
    /// The ends standing nearly as far off as the farthest are the far side,
    /// whole or in pieces, and the middle of the first and the last of them
    /// along `side` is the answer: a rectangle's, a U's arms, a top rounded or
    /// cut short at its corners. One end alone that far is a corner: the far
    /// side then runs down from it, along the one trait whose other end stands
    /// more than half as far off — a trapezoid's other leg, a slanted top —
    /// and is the corner itself when no trait does, or two alike do: a
    /// triangle's, a roof's.
    fn opposite(&self, shape: &[PointId], side: SegmentId) -> Option<DVec2> {
        let line = self.segments().get(side.0).copied()?;
        let start = self.point(line.start);
        let along = (self.point(line.end) - start).try_normalize()?;
        let tied = self.tied_by_direction(shape);
        let drawn = |tied_only: bool| -> Vec<(PointId, PointId)> {
            self.live_segments()
                .filter(|(id, other)| {
                    *id != side
                        && !other.construction
                        && shape.contains(&other.start)
                        && shape.contains(&other.end)
                        && (!tied_only || tied.contains(id))
                })
                .map(|(_, other)| (other.start, other.end))
                .collect()
        };
        let mut traits = drawn(true);
        if traits.is_empty() {
            traits = drawn(false);
        }
        let mut ends: Vec<PointId> = traits.iter().flat_map(|(from, to)| [*from, *to]).collect();
        ends.sort_by_key(|point| point.0);
        ends.dedup();
        let signed = |point: PointId| (self.point(point) - start).dot(along.perp());
        let reach = ends
            .iter()
            .map(|point| signed(*point).abs())
            .fold(0.0, f64::max);
        if reach <= self.drawing_size() * 1e-6 {
            return None;
        }
        let nearly = reach * NEARLY;
        let way = {
            let highest = ends
                .iter()
                .map(|point| signed(*point))
                .fold(f64::MIN, f64::max);
            let lowest = ends
                .iter()
                .map(|point| signed(*point))
                .fold(f64::MAX, f64::min);
            match (highest >= reach - nearly, -lowest >= reach - nearly) {
                (true, true) => match ends.iter().map(|point| signed(*point)).sum::<f64>() {
                    bulk if bulk < 0.0 => -1.0,
                    _ => 1.0,
                },
                (true, false) => 1.0,
                (false, _) => -1.0,
            }
        };
        let off = |point: PointId| signed(point) * way;
        let level: Vec<PointId> = ends
            .iter()
            .copied()
            .filter(|point| off(*point) >= reach - nearly)
            .collect();
        let along_it = |one: &PointId, other: &PointId| {
            (self.point(*one).dot(along)).total_cmp(&self.point(*other).dot(along))
        };
        if let [corner] = level[..] {
            let down: Vec<PointId> = traits
                .iter()
                .filter_map(|(from, to)| match (*from == corner, *to == corner) {
                    (true, _) => Some(*to),
                    (_, true) => Some(*from),
                    _ => None,
                })
                .filter(|other| off(*other) > reach / 2.0)
                .collect();
            let across = down
                .iter()
                .copied()
                .max_by(|one, other| off(*one).total_cmp(&off(*other)));
            let alike = |across: PointId| {
                down.iter()
                    .filter(|other| off(across) - off(**other) <= nearly)
                    .count()
            };
            return Some(match across {
                Some(across) if alike(across) == 1 => {
                    (self.point(corner) + self.point(across)) / 2.0
                }
                _ => self.point(corner),
            });
        }
        let first = level.iter().copied().min_by(along_it)?;
        let last = level.iter().copied().max_by(along_it)?;
        Some((self.point(first) + self.point(last)) / 2.0)
    }

    /// Whether the whole shape can be turned about `about` without a single
    /// rule of the drawing giving, and with nothing holding it anywhere else.
    pub(crate) fn turns_freely_about(
        &self,
        shape: &[PointId],
        about: DVec2,
        millimeters_per_unit: f64,
    ) -> bool {
        let pinned = self.pinned_points();
        if shape
            .iter()
            .any(|point| pinned[point.0] && self.point(*point).distance(about) > 1e-9)
        {
            return false;
        }
        let mut turn = crate::equation::Equation::new(self.variables());
        for point in shape.iter().filter(|point| !pinned[point.0]) {
            turn.add(*point, (self.point(*point) - about).perp());
        }
        self.equations_pinned_by(millimeters_per_unit, &pinned)
            .iter()
            .all(|row| turns_nothing(row, &turn))
    }

    /// The middle of the box the shape's drawn curves fit in — construction
    /// left out, an arc by the stretch it runs rather than by its centre.
    /// Nothing drawn, and the box of its points stands in.
    fn middle_of(&self, shape: &[PointId]) -> Option<DVec2> {
        let mut places: Vec<DVec2> = Vec::new();
        for (_, line) in self.live_segments() {
            if !line.construction && shape.contains(&line.start) {
                places.extend([self.point(line.start), self.point(line.end)]);
            }
        }
        for (_, round) in self.live_circles() {
            if !round.construction && shape.contains(&round.center) {
                let centre = self.point(round.center);
                places.extend([
                    centre - DVec2::splat(round.radius),
                    centre + DVec2::splat(round.radius),
                ]);
            }
        }
        for (id, arc) in self.live_arcs() {
            if !arc.construction && shape.contains(&arc.center) {
                let (low, high) = bounds_of(self.arc_draft(id));
                places.extend([low, high]);
            }
        }
        for (id, ellipse) in self.live_ellipses() {
            if !ellipse.construction && shape.contains(&ellipse.center) {
                let drawn = self.ellipse_draft(id);
                let reach = DVec2::new(
                    drawn.first.x.hypot(drawn.second_axis().x),
                    drawn.first.y.hypot(drawn.second_axis().y),
                );
                places.extend([drawn.centre - reach, drawn.centre + reach]);
            }
        }
        if places.is_empty() {
            places = shape.iter().map(|point| self.point(*point)).collect();
        }
        let first = *places.first()?;
        let (low, high) = places.iter().fold((first, first), |(low, high), place| {
            (low.min(*place), high.max(*place))
        });
        Some((low + high) / 2.0)
    }
}

/// The angle that turns `end` about `about` onto the grid: when the place the
/// end would stand at, turned by `angle`, has a grid point within the grid's
/// reach, the end is pointed at it — and lands on it when the grid point is as
/// far from `about` as the end is. Otherwise `angle` as it was.
pub(crate) fn angle_onto_grid(about: DVec2, end: DVec2, angle: f64, grid: &SnapSettings) -> f64 {
    let turned = about + DVec2::from_angle(angle).rotate(end - about);
    let Some(node) = grid.node_near(turned) else {
        return angle;
    };
    match (
        (end - about).try_normalize(),
        (node - about).try_normalize(),
    ) {
        (Some(was), Some(wanted)) => was.angle_to(wanted),
        _ => angle,
    }
}

#[cfg(test)]
mod tests;
