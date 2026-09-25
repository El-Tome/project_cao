//! What a side or a curve pulled by the hand asks of the drawing: to travel
//! sideways, to be drawn to another size, or to turn.
//!
//! The gesture decides, and it decides at every instant: pulled across, a side
//! travels and a curve grows or shrinks; slid along itself, the shape it
//! belongs to turns. Across and along are read about the place the shape would
//! turn on, the way the place grabbed goes round it — so a turn past a quarter
//! stays a turn, and a pull straight out stays a pull.

use glam::DVec2;

use crate::length::LengthOutcome;
use crate::resizing::Curved;
use crate::sketch::settling::Kept;
use crate::sketch::{PointId, SegmentId, Sketch};
use crate::snap::SnapSettings;
use crate::turning::{Turn, angle_onto_grid};

/// What a side pulled asks for.
#[derive(Clone, Debug, PartialEq)]
pub enum SideDrag {
    /// Travel sideways by this much, square to the side.
    Across {
        by: DVec2,
    },
    Along(Turn),
}

/// What a curve pulled asks for.
#[derive(Clone, Debug, PartialEq)]
pub enum CurveDrag {
    /// Drawn to another size about its centre, as #375 has it.
    Resize {
        reach: f64,
    },
    Along(Turn),
}

/// What a press takes hold of to pull, when it took hold of no point.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pulled {
    Side(SegmentId),
    Curve(Curved),
}

impl Sketch {
    /// The side or the curve a press takes hold of, whichever lies nearer.
    ///
    /// An ellipse's axis is the ellipse's own and is not a side, and a side
    /// whose two ends can no longer move has nowhere to go: a press on it
    /// still pulls a box, as it always did.
    pub fn pulled_at(&self, place: DVec2, reach: f64, millimeters_per_unit: f64) -> Option<Pulled> {
        let settled = self.settled_points(millimeters_per_unit);
        let pinned = self.pinned_points();
        let stuck = |point: PointId| {
            settled.get(point.0).copied().unwrap_or(false)
                || pinned.get(point.0).copied().unwrap_or(false)
        };
        let side = self
            .nearest_segment(place, reach)
            .filter(|side| self.ellipse_of_axis(*side).is_none())
            .filter(|side| {
                let line = self.segments()[side.0];
                !(stuck(line.start) && stuck(line.end))
            })
            .map(|side| (Pulled::Side(side), self.off_the_side(side, place)));
        let curve = self
            .curve_at(place, reach)
            .map(|curve| (Pulled::Curve(curve), self.off_the_curve(curve, place)));
        [side, curve]
            .into_iter()
            .flatten()
            .min_by(|one, other| one.1.total_cmp(&other.1))
            .map(|(pulled, _)| pulled)
    }

    fn off_the_side(&self, side: SegmentId, place: DVec2) -> f64 {
        let (start, end) = self.endpoints(side);
        let span = end - start;
        let along = ((place - start).dot(span) / span.length_squared().max(1e-18)).clamp(0.0, 1.0);
        place.distance(start + span * along)
    }

    fn off_the_curve(&self, curve: Curved, place: DVec2) -> f64 {
        match curve {
            Curved::Circle(circle) => {
                let round = self.circle(circle);
                (self.point(round.center).distance(place) - round.radius).abs()
            }
            Curved::Arc(arc) => self.distance_to_arc(arc, place),
            Curved::Ellipse(ellipse) => self.distance_to_ellipse(ellipse, place),
        }
    }

    /// What pulling `side` from `pressed` to `cursor` asks for. Both are the
    /// hand's own places, before any magnet: pulled back onto the side it was
    /// pressed on, a pull would read as a slide.
    ///
    /// A shape with no place to turn about, or whose place lies on the side's
    /// own line — a trait on its own — has no along: it only travels.
    pub fn side_drag(
        &self,
        side: SegmentId,
        pressed: DVec2,
        cursor: DVec2,
        grid: &SnapSettings,
        millimeters_per_unit: f64,
    ) -> SideDrag {
        let Some(line) = self.segments().get(side.0).copied() else {
            return SideDrag::Across { by: DVec2::ZERO };
        };
        let (start, end) = (self.point(line.start), self.point(line.end));
        let Some(along) = (end - start).try_normalize() else {
            return SideDrag::Across { by: DVec2::ZERO };
        };
        let normal = along.perp();
        let across = SideDrag::Across {
            by: normal * (cursor - pressed).dot(normal),
        };

        let shape = self.shape_through(&[line.start, line.end]);
        let Some(about) = self.turning_centre(&shape, millimeters_per_unit) else {
            return across;
        };
        if ((about - start).dot(normal)).abs() <= self.drawing_size() * 1e-6 {
            return across;
        }
        // Whichever reading leaves the place grabbed nearer the hand: a pull
        // across leaves it short by its slip along the side, a turn by how far
        // the hand went out from or in towards the place it turns on. A pull
        // straight across slips nothing, wherever the side was pressed; a hand
        // going round slips nothing either, however far round it goes.
        let across_slip = (cursor - pressed).dot(along).abs();
        let along_slip = (cursor.distance(about) - pressed.distance(about)).abs();
        if across_slip <= along_slip {
            return across;
        }
        let angle = turned(about, pressed, cursor);
        let end = [line.start, line.end]
            .into_iter()
            .min_by(|one, other| {
                let place = |point: PointId| {
                    about + DVec2::from_angle(angle).rotate(self.point(point) - about)
                };
                place(*one)
                    .distance(cursor)
                    .total_cmp(&place(*other).distance(cursor))
            })
            .map(|point| self.point(point))
            .unwrap_or(start);
        SideDrag::Along(Turn {
            points: shape,
            about,
            angle: angle_onto_grid(about, end, angle, grid),
        })
    }

    /// What pulling `curve` from `pressed` to `cursor` asks for.
    ///
    /// Towards or away from the centre it is drawn to another size, exactly as
    /// before — and the size is read at `snapped`, the cursor as the magnets
    /// left it, as it always was. Round the centre it turns, with the shape it
    /// belongs to. A circle looks the same whichever way up it is, so a circle
    /// is always drawn to another size.
    pub fn curve_drag(
        &self,
        curve: Curved,
        pressed: DVec2,
        cursor: DVec2,
        snapped: DVec2,
        grid: &SnapSettings,
        millimeters_per_unit: f64,
    ) -> CurveDrag {
        let resize = CurveDrag::Resize {
            reach: self.reach_through(curve, snapped),
        };
        let (centre, handles) = match curve {
            Curved::Circle(_) => return resize,
            Curved::Arc(arc) => {
                let drawn = self.arc(arc);
                (drawn.center, vec![drawn.start, drawn.end])
            }
            Curved::Ellipse(ellipse) => {
                let [centre, ends @ ..] = self.ellipse_points(ellipse);
                (centre, ends.to_vec())
            }
        };
        let about = self.point(centre);
        let shape = self.shape_through(&handles);
        if !self.turns_freely_about(&shape, about, millimeters_per_unit) {
            return resize;
        }
        let angle = turned(about, pressed, cursor);
        let round = pressed.distance(about) * angle.abs();
        let out = (cursor.distance(about) - pressed.distance(about)).abs();
        if round <= out {
            return resize;
        }
        let end = handles
            .iter()
            .map(|point| self.point(*point))
            .min_by(|one, other| {
                let place = |at: DVec2| about + DVec2::from_angle(angle).rotate(at - about);
                place(*one)
                    .distance(cursor)
                    .total_cmp(&place(*other).distance(cursor))
            })
            .unwrap_or(pressed);
        CurveDrag::Along(Turn {
            points: shape,
            about,
            angle: angle_onto_grid(about, end, angle, grid),
        })
    }

    /// Moves a side sideways by `by` and settles the drawing around it.
    ///
    /// The side's line travels, not its two ends: each end stays on the moved
    /// line and is free to slide along it, which is what lets it follow the
    /// trait joining it there — a triangle's hypotenuse pulled out grows the
    /// triangle, and does not refuse because its ends were nailed across. The
    /// side keeps its direction, and so does every trait of its shape a rule of
    /// direction ties; the point of the shape farthest off the side stays
    /// where it is. What cannot be had is given back whole: the side stays.
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

/// How far round `about` the hand has gone from `pressed` to `cursor`, in
/// radians between a half turn back and a half turn on.
fn turned(about: DVec2, pressed: DVec2, cursor: DVec2) -> f64 {
    match (
        (pressed - about).try_normalize(),
        (cursor - about).try_normalize(),
    ) {
        (Some(from), Some(to)) => from.angle_to(to),
        _ => 0.0,
    }
}

#[cfg(test)]
mod tests;
