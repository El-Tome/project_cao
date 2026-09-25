//! A shape turned by the hand: about what, by how much, and how the grid pulls
//! it back square.
//!
//! Turning is the one gesture that changes a shape's way up on purpose. It is
//! reached by sliding a side or a curve along itself; what it turns is the
//! whole shape the grab belongs to, rigidly, and the rest of the drawing
//! settles around it.

use glam::DVec2;

use crate::arcing::bounds_of;
use crate::independence::turns_nothing;
use crate::length::LengthOutcome;
use crate::sketch::{PointId, Sketch};
use crate::snap::SnapSettings;

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

    /// Where a shape turns about when the hand turns it: the one place that
    /// holds it, or the middle of what it draws. Nothing when two different
    /// places hold it, or when a rigid turn about that place would break one
    /// of the drawing's rules — a trait held level, an angle against an axis,
    /// a distance to something else.
    pub(crate) fn turning_centre(
        &self,
        shape: &[PointId],
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
            None => self.middle_of(shape)?,
        };
        self.turns_freely_about(shape, about, millimeters_per_unit)
            .then_some(about)
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
pub fn angle_onto_grid(about: DVec2, end: DVec2, angle: f64, grid: &SnapSettings) -> f64 {
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
