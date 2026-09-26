//! A side pulled across: its line travels and the shape it belongs to
//! stretches after it.

use glam::DVec2;

use crate::length::LengthOutcome;
use crate::sketch::settling::Kept;
use crate::sketch::{PointId, SegmentId, Sketch};

impl Sketch {
    /// Moves a side sideways by `by` and settles the drawing around it.
    ///
    /// The side's line travels, not its two ends: each end stays on the moved
    /// line and is free to slide along it, which is what lets it follow the
    /// trait joining it there — a triangle's hypotenuse pulled out grows the
    /// triangle, and does not refuse because its ends were nailed across. The
    /// side keeps its direction, and so does every trait of its shape a rule of
    /// direction ties; the point of the shape farthest off the side stays
    /// where it is. A trait on its own has no shape to stretch, and is carried
    /// whole. What cannot be had is given back whole: the side stays.
    pub fn move_side(
        &mut self,
        side: SegmentId,
        by: DVec2,
        millimeters_per_unit: f64,
    ) -> LengthOutcome {
        let Some(line) = self.segments().get(side.0).copied() else {
            return LengthOutcome::Degenerate;
        };
        if self.is_erased_segment(side) {
            return LengthOutcome::Degenerate;
        }
        let Some(along) = (self.point(line.end) - self.point(line.start)).try_normalize() else {
            return LengthOutcome::Degenerate;
        };
        let normal = along.perp();
        let travel = by.dot(normal);

        let shape = self.shape_through(&[line.start, line.end]);
        if self.travels_whole(&shape, self.point(line.start), normal) {
            return self.carry(shape, side, by, millimeters_per_unit);
        }
        let mut lines = self.lines_kept_in(&shape, &[line.start, line.end]);
        lines.retain(|kept| kept.segment != Some(side));
        lines.extend(Kept::direction(self, line.start, line.end, Some(side)));
        lines.push(Kept::level(self, line.start, normal, travel));
        lines.push(Kept::level(self, line.end, normal, travel));
        let mut pins: Vec<PointId> = self
            .farthest_off(&shape, &[line.start, line.end], normal)
            .into_iter()
            .collect();

        // An end lying on a curve slides round that curve to where the moved
        // line crosses it: the curve keeps its centre and its size, which only
        // the curve itself says. Not an end the side leaves the curve tangent
        // at — a fillet's, a slot's — which the curve follows instead.
        let mut landed: Vec<(PointId, DVec2)> = Vec::new();
        for end in [line.start, line.end] {
            if let Some(ellipse) = self.ellipse_cut_at(end) {
                pins.extend(self.ellipse_points(ellipse));
                continue;
            }
            let Some((centre, reach)) = self.curve_under(end) else {
                continue;
            };
            let out = self.point(end) - self.point(centre);
            if out.normalize_or_zero().dot(along).abs() < 1e-3 {
                continue;
            }
            let level = self.point(end).dot(normal) + travel;
            let Some(place) = crossing(self.point(centre), reach, normal, level, self.point(end))
            else {
                return LengthOutcome::BestEffort;
            };
            landed.push((end, place));
            pins.extend([centre, end]);
        }
        let kept = self.shapes_now();
        for (end, place) in landed {
            self.move_point(end, place);
        }
        match self.settle_held(pins, lines, millimeters_per_unit) {
            true => LengthOutcome::Exact,
            false => {
                self.give_back(kept);
                LengthOutcome::BestEffort
            }
        }
    }

    /// Whether the shape through a side is that trait alone — nothing joins
    /// it but points held on it, and nothing holds it where it is. With no
    /// shape to stretch or to turn, it goes wherever the hand takes it.
    pub(crate) fn travels_whole(&self, shape: &[PointId], on: DVec2, normal: DVec2) -> bool {
        let pinned = self.pinned_points();
        let level = on.dot(normal);
        shape.iter().all(|point| {
            !pinned[point.0]
                && (self.point(*point).dot(normal) - level).abs() <= self.drawing_size() * 1e-6
        })
    }

    /// A trait on its own carried by `by`, with what is held on it, and held
    /// there while the rest of the drawing settles: what a rule ties it to
    /// follows. When that place cannot be had — a rule holds it to an axis, or
    /// at a distance from a point — it is let go of there, whole, and the
    /// drawing takes it back as near as its rules allow, as long and as
    /// upright as it was. The drawing given back when neither can be had.
    fn carry(
        &mut self,
        shape: Vec<PointId>,
        side: SegmentId,
        by: DVec2,
        millimeters_per_unit: f64,
    ) -> LengthOutcome {
        let line = self.segments()[side.0];
        let kept = self.shapes_now();
        let carried = |sketch: &mut Self, held: Vec<PointId>, lines: Vec<Kept>| {
            for point in &shape {
                let place = sketch.point(*point) + by;
                sketch.move_point(*point, place);
            }
            let whole = sketch.settle_held(held, lines, millimeters_per_unit);
            if !whole {
                sketch.give_back(kept.clone());
            }
            whole
        };
        let lines = Kept::whole(self, line.start, line.end, Some(side));
        match carried(self, shape.clone(), Vec::new()) || carried(self, Vec::new(), lines) {
            true => LengthOutcome::Exact,
            false => LengthOutcome::BestEffort,
        }
    }

    /// The point of a shape standing farthest off a line, which stays where it
    /// is while that line travels — nothing when something already holds the
    /// shape, or when every point of it lies on the line.
    ///
    /// Taken among the points lying on the traits a rule of direction ties
    /// first, as the point a drag keeps is: a tail hanging off a rectangle,
    /// however far it reaches, does not take the place of its opposite side.
    fn farthest_off(&self, shape: &[PointId], side: &[PointId], normal: DVec2) -> Option<PointId> {
        let pinned = self.pinned_points();
        if shape.iter().any(|point| pinned[point.0]) {
            return None;
        }
        let level = self.point(side[0]).dot(normal);
        let aside = self.held_aside();
        let off = |point: &PointId| (self.point(*point).dot(normal) - level).abs();
        let candidates = |among: Vec<PointId>| -> Vec<PointId> {
            among
                .into_iter()
                .filter(|point| {
                    !side.contains(point)
                        && !self.only_a_centre(*point)
                        && !aside.contains(point)
                        && off(point) > self.drawing_size() * 1e-6
                })
                .collect()
        };
        let mut pool = candidates(self.points_on(&self.tied_by_direction(shape)));
        if pool.is_empty() {
            pool = candidates(shape.to_vec());
        }
        let tie = self.drawing_size() * 1e-9;
        let mut best: Option<(PointId, f64)> = None;
        for point in pool {
            let far = off(&point);
            if best.is_none_or(|(_, farthest)| far > farthest + tie) {
                best = Some((point, far));
            }
        }
        best.map(|(point, _)| point)
    }

    /// The ellipse a point ends the drawn stretch of, when it ends one.
    fn ellipse_cut_at(&self, point: PointId) -> Option<crate::ellipse::EllipseId> {
        self.live_ellipses()
            .find(|(_, ellipse)| {
                ellipse
                    .drawn
                    .is_some_and(|(from, to)| from == point || to == point)
            })
            .map(|(id, _)| id)
    }

    /// The centre and the reach of the circle or the arc a point lies on, when
    /// it lies on exactly one: what it slides round.
    fn curve_under(&self, point: PointId) -> Option<(PointId, f64)> {
        let [centre] = self.centres_under(point)[..] else {
            return None;
        };
        let round = self.live_circles().any(|(_, round)| round.center == centre)
            || self.live_arcs().any(|(_, arc)| arc.center == centre);
        round.then(|| (centre, self.point(point).distance(self.point(centre))))
    }
}

/// Where the line of the plane `place · normal = level` crosses the circle
/// about `centre`, on the side nearer `near`; nothing when it misses it.
fn crossing(centre: DVec2, reach: f64, normal: DVec2, level: f64, near: DVec2) -> Option<DVec2> {
    let foot = centre + normal * (level - centre.dot(normal));
    let off = foot.distance(centre);
    if off > reach {
        return None;
    }
    let half = (reach * reach - off * off).sqrt();
    let along = normal.perp();
    [foot + along * half, foot - along * half]
        .into_iter()
        .min_by(|one, other| one.distance(near).total_cmp(&other.distance(near)))
}
