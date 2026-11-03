//! What a cursor finds of a fillet or a chamfer: its curve, as it is drawn.

use glam::DVec2;

use super::CornerCutId;
use super::drawn::CutCurve;
use crate::holding::Support;
use crate::reading::Reading;
use crate::sketch::{PointId, Sketch};
use crate::snap::Snap;

impl Sketch {
    /// The cut whose curve passes closest to `position`, within `tolerance`.
    pub fn nearest_cut(&self, position: DVec2, tolerance: f64) -> Option<CornerCutId> {
        self.drawn_cuts()
            .into_iter()
            .map(|drawn| (drawn.id, distance_to(drawn.curve, position)))
            .filter(|(_, distance)| *distance <= tolerance)
            .min_by(|(_, one), (_, other)| one.total_cmp(other))
            .map(|(id, _)| id)
    }

    /// The cut under the cursor when its curve passes nearer than what is
    /// drawn of any trait: where a curve meets its side, a press nearer the
    /// side takes the side.
    pub fn cut_before_traits(&self, position: DVec2, tolerance: f64) -> Option<CornerCutId> {
        let drawn = self.drawn_cuts();
        let (cut, from_cut) = drawn
            .iter()
            .map(|cut| (cut.id, distance_to(cut.curve, position)))
            .filter(|(_, distance)| *distance <= tolerance)
            .min_by(|(_, one), (_, other)| one.total_cmp(other))?;
        let pulled = self.pulled_ends(&drawn);
        let from_traits = self
            .live_segments()
            .filter_map(|(id, segment)| {
                let [from, to] = self.visible_ends(id, &pulled)?;
                let from = from.unwrap_or(self.point(segment.start));
                let to = to.unwrap_or(self.point(segment.end));
                Some(nearest_on(CutCurve::Straight { from, to }, position).distance(position))
            })
            .fold(f64::INFINITY, f64::min);
        (from_cut <= from_traits).then_some(cut)
    }

    /// What « Mesurer » reads of a cut: a fillet's radius, a chamfer's cut as
    /// a straight run.
    pub fn read_cut(&self, cut: CornerCutId) -> Option<Reading> {
        let drawn = self.drawn_cut(cut)?;
        Some(match drawn.curve {
            CutCurve::Arc(arc) => Reading::Round {
                radius: arc.centre.distance(arc.start),
            },
            CutCurve::Straight { from, to } => Reading::Gap {
                span: from.distance(to),
                offsets: (to - from).abs(),
            },
        })
    }

    /// The two ends of a chamfer's cut, the run a measure reads off it.
    pub fn cut_run(&self, cut: CornerCutId) -> Option<(DVec2, DVec2)> {
        match self.drawn_cut(cut)?.curve {
            CutCurve::Straight { from, to } => Some((from, to)),
            CutCurve::Arc(_) => None,
        }
    }

    /// The middle of what a cut draws, where its measure is written.
    pub fn cut_middle(&self, cut: CornerCutId) -> Option<DVec2> {
        Some(middle(self.drawn_cut(cut)?.curve))
    }

    /// The point a press at `position` drags: the point under it, or the
    /// corner of the cut under it. Taking a fillet or a chamfer by its curve is
    /// taking its corner, the one thing it moves with — and so is taking it by
    /// one of its points, whether something hangs there or not.
    pub fn point_to_drag_at(&self, position: DVec2, tolerance: f64) -> Option<PointId> {
        if let Some(point) = self.nearest_point(position, tolerance) {
            return Some(self.corner_hung_from(point).unwrap_or(point));
        }
        let cut = self
            .nearest_cut_point(position, tolerance)
            .map(|(cut, _)| cut)
            .or_else(|| self.nearest_cut(position, tolerance))?;
        Some(self.corner_cuts()[cut.0].corner)
    }

    /// The corner of the cut a point hangs at, when it is one of a cut's
    /// points.
    fn corner_hung_from(&self, point: PointId) -> Option<PointId> {
        self.holds_on(point)
            .into_iter()
            .find_map(|support| match support {
                Support::CutPoint(cut, _) if self.cut_holds(cut) => {
                    Some(self.corner_cuts()[cut.0].corner)
                }
                _ => None,
            })
    }
}

impl Sketch {
    /// The place a fillet or a chamfer offers nearest the cursor, within
    /// `reach`, besides its two ends, which [`Sketch::crossings`] already
    /// gives: the centre of a fillet, and the middle of its curve or of a
    /// chamfer's cut.
    pub(crate) fn nearest_cut_place(&self, cursor: DVec2, reach: f64) -> Option<(DVec2, Snap)> {
        self.drawn_cuts()
            .into_iter()
            .flat_map(|drawn| {
                let middle = middle(drawn.curve);
                let centre = match drawn.curve {
                    CutCurve::Arc(arc) => Some((arc.centre, Snap::Centre(arc.centre))),
                    CutCurve::Straight { .. } => None,
                };
                centre.into_iter().chain([(middle, Snap::Midpoint(middle))])
            })
            .filter(|(at, _)| at.distance(cursor) <= reach)
            .min_by(|(one, _), (other, _)| {
                one.distance_squared(cursor)
                    .total_cmp(&other.distance_squared(cursor))
            })
    }

    /// The place on the curve of every cut nearest the cursor, one for each.
    pub(crate) fn places_on_cuts(&self, cursor: DVec2) -> Vec<DVec2> {
        self.drawn_cuts()
            .into_iter()
            .map(|drawn| nearest_on(drawn.curve, cursor))
            .collect()
    }
}

/// The middle of the curve a cut draws.
fn middle(curve: CutCurve) -> DVec2 {
    match curve {
        CutCurve::Straight { from, to } => (from + to) / 2.0,
        CutCurve::Arc(arc) => {
            let from = (arc.start - arc.centre).to_angle();
            let half = crate::arcing::sweep_of(arc) / 2.0;
            arc.centre + DVec2::from_angle(from + half) * arc.centre.distance(arc.start)
        }
    }
}

/// The place on the curve a cut draws nearest `position`: on the curve where
/// it passes by, at the nearer end beyond it.
fn nearest_on(curve: CutCurve, position: DVec2) -> DVec2 {
    match curve {
        CutCurve::Straight { from, to } => {
            let span = to - from;
            let along = ((position - from).dot(span) / span.length_squared()).clamp(0.0, 1.0);
            from + span * along
        }
        CutCurve::Arc(drawn) => {
            let radius = drawn.centre.distance(drawn.start);
            let from = (drawn.start - drawn.centre).to_angle();
            let turned =
                ((position - drawn.centre).to_angle() - from).rem_euclid(std::f64::consts::TAU);
            match turned <= crate::arcing::sweep_of(drawn) {
                true => drawn.centre + (position - drawn.centre).normalize_or_zero() * radius,
                false => match position.distance(drawn.start) <= position.distance(drawn.end) {
                    true => drawn.start,
                    false => drawn.end,
                },
            }
        }
    }
}

/// How far a place sits from the curve a cut draws.
pub(super) fn distance_to(curve: CutCurve, position: DVec2) -> f64 {
    nearest_on(curve, position).distance(position)
}

#[cfg(test)]
mod tests;
