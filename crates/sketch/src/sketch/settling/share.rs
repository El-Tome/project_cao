//! A point held on a trait, a circle or an arc keeps its place along what
//! holds it while a gesture moves that: its share of the trait, its angle about
//! the circle's centre, its share of the arc's sweep.
//!
//! Read when the gesture starts and put back once the drawing has settled, so
//! nothing new is kept in the file: a replay reads the same drawing and puts
//! the point back at the same place. A point a value places is the value's.

use glam::DVec2;

use super::kept::Kept;
use crate::arc::ArcId;
use crate::constraints::Constraint;
use crate::equation::Equation;
use crate::independence::turns_nothing;
use crate::length::LengthOutcome;
use crate::sketch::{CircleId, PointId, SegmentId, Sketch};

/// Where along what holds it a point stands.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Share {
    point: PointId,
    along: Along,
}

#[derive(Clone, Copy, Debug)]
enum Along {
    /// Its share of the trait, counted from the trait's start.
    Trait(SegmentId, f64),
    /// Its angle about the circle's centre.
    Circle(CircleId, f64),
    /// Its share of the arc's sweep, counted from the arc's start.
    Arc(ArcId, f64),
}

impl Sketch {
    /// Where each point held on a trait, a circle or an arc stands along it,
    /// read before a gesture moves anything. Not the points in `except`, which
    /// the gesture holds itself, nor one held on two things at once, nor one a
    /// value or another rule places along what holds it: sliding it there
    /// would break that rule, and the rule wins.
    ///
    /// A rule tying the point to something free to follow does not place it:
    /// a trait standing square on it slides along with it. One tying it only
    /// to what does not move — what holds it, what the gesture holds, what
    /// the drawing holds, the size of its circle — does.
    pub(crate) fn shares(&self, except: &[PointId], millimeters_per_unit: f64) -> Vec<Share> {
        let pinned = self.pinned_points();
        let mut rows: Option<Vec<Equation>> = None;
        let mut shares = Vec::new();
        for rule in self.constraints() {
            let Some((point, along, way)) = self.standing_along(*rule) else {
                continue;
            };
            if except.contains(&point) || pinned[point.0] || self.holds_on(point).len() != 1 {
                continue;
            }
            let rows =
                rows.get_or_insert_with(|| self.equations_pinned_by(millimeters_per_unit, &pinned));
            let mut slide = Equation::new(self.variables());
            slide.add(point, way);
            let mut still = except.to_vec();
            still.extend(self.defining(along));
            let followed = |row: &Equation| {
                (0..self.points().len()).any(|other| {
                    other != point.0
                        && !pinned[other]
                        && !still.contains(&PointId(other))
                        && (row.gradient[other * 2] != 0.0 || row.gradient[other * 2 + 1] != 0.0)
                })
            };
            let speaks_of = |row: &&Equation| {
                row.gradient[point.0 * 2] != 0.0 || row.gradient[point.0 * 2 + 1] != 0.0
            };
            if rows
                .iter()
                .filter(speaks_of)
                .all(|row| turns_nothing(row, &slide) || followed(row))
            {
                shares.push(Share { point, along });
            }
        }
        shares
    }

    /// The points `shares` read, put back at their place along what holds
    /// them, and the drawing settled with them and `held` held still and
    /// `lines` kept. Left as the gesture left it when that cannot be had.
    pub(crate) fn keep_shares(
        &mut self,
        shares: &[Share],
        held: &[PointId],
        lines: &[Kept],
        millimeters_per_unit: f64,
    ) {
        let size = self.drawing_size();
        let moved: Vec<(PointId, DVec2)> = shares
            .iter()
            .filter_map(|share| Some((share.point, self.place_of(share.along)?)))
            .filter(|(point, place)| place.distance(self.point(*point)) > size * 1e-7)
            .collect();
        if moved.is_empty() {
            return;
        }
        if self.put_back(&moved, held, lines, millimeters_per_unit) {
            return;
        }
        // Two places the drawing cannot have at once — two points a rule ties
        // — cost none of the others theirs: they are put back one at a time,
        // each kept when the drawing takes it.
        let mut kept: Vec<(PointId, DVec2)> = Vec::new();
        for each in moved {
            kept.push(each);
            if !self.put_back(&kept, held, lines, millimeters_per_unit) {
                kept.pop();
            }
        }
        self.put_back(&kept, held, lines, millimeters_per_unit);
    }

    /// The points of `moved` put at their places and the drawing settled with
    /// them and `held` held still: whether it came out whole. Given back when
    /// it does not.
    fn put_back(
        &mut self,
        moved: &[(PointId, DVec2)],
        held: &[PointId],
        lines: &[Kept],
        millimeters_per_unit: f64,
    ) -> bool {
        let kept = self.shapes_now();
        for (point, place) in moved {
            self.move_point(*point, *place);
        }
        let mut points = held.to_vec();
        points.extend(moved.iter().map(|(point, _)| *point));
        if self.settle_held(points, lines.to_vec(), millimeters_per_unit) {
            return true;
        }
        self.give_back(kept);
        false
    }

    /// Settles the drawing after a value or a rule changed, each point held on
    /// a trait, a circle or an arc kept at its place along it, as a gesture
    /// does. Read once the change is made, so that a value placing the point
    /// along what holds it is the value's.
    pub fn resolve_keeping_places(&mut self, millimeters_per_unit: f64) -> LengthOutcome {
        let shares = self.shares(&[], millimeters_per_unit);
        let outcome = self.resolve(millimeters_per_unit);
        if outcome == LengthOutcome::Exact {
            self.keep_shares(&shares, &[], &[], millimeters_per_unit);
        }
        outcome
    }

    /// The points that say where a trait, a circle or an arc stands.
    fn defining(&self, along: Along) -> Vec<PointId> {
        match along {
            Along::Trait(segment, _) => {
                let line = self.segments()[segment.0];
                vec![line.start, line.end]
            }
            Along::Circle(circle, _) => vec![self.circle(circle).center],
            Along::Arc(arc, _) => {
                let curve = self.arc(arc);
                vec![curve.center, curve.start, curve.end]
            }
        }
    }

    /// The point a rule holds, where it stands along what holds it, and the
    /// way it would slide along it there.
    fn standing_along(&self, rule: Constraint) -> Option<(PointId, Along, DVec2)> {
        match rule {
            Constraint::OnSegment { point, segment, .. } => {
                let (start, end) = self.endpoints(segment);
                let span = end - start;
                let share = (self.point(point) - start).dot(span) / span.length_squared();
                share
                    .is_finite()
                    .then(|| (point, Along::Trait(segment, share), span.normalize()))
            }
            Constraint::OnCircle { point, circle } => {
                let out = self.point(point) - self.point(self.circle(circle).center);
                let way = out.try_normalize()?.perp();
                Some((point, Along::Circle(circle, out.to_angle()), way))
            }
            Constraint::OnArc { point, arc } => {
                let drawn = self.arc_draft(arc);
                let out = self.point(point) - drawn.centre;
                let way = out.try_normalize()?.perp();
                let sweep = self.arc_sweep(arc);
                let tau = std::f64::consts::TAU;
                let round =
                    (out.to_angle() - (drawn.start - drawn.centre).to_angle()).rem_euclid(tau);
                // Held on the arc's whole circle, a point may stand past either
                // end: past the start it counts back from it, not nearly a whole
                // turn on.
                let round = match round > sweep && tau - round < round - sweep {
                    true => round - tau,
                    false => round,
                };
                (sweep > 1e-9).then(|| (point, Along::Arc(arc, round / sweep), way))
            }
            _ => None,
        }
    }

    /// Where a place along a trait, a circle or an arc stands in the drawing
    /// as it is now.
    fn place_of(&self, along: Along) -> Option<DVec2> {
        match along {
            Along::Trait(segment, share) => {
                let (start, end) = self.endpoints(segment);
                Some(start.lerp(end, share))
            }
            Along::Circle(circle, angle) => {
                let round = self.circle(circle);
                Some(self.point(round.center) + DVec2::from_angle(angle) * round.radius)
            }
            Along::Arc(arc, share) => {
                let drawn = self.arc_draft(arc);
                let out = drawn.start - drawn.centre;
                let turn = share * self.arc_sweep(arc);
                Some(drawn.centre + DVec2::from_angle(turn).rotate(out))
            }
        }
    }
}

#[cfg(test)]
mod tests;
