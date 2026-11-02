//! What a cursor finds of a fillet or a chamfer: its curve, as it is drawn.

use glam::DVec2;

use super::CornerCutId;
use super::drawn::CutCurve;
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

    /// The point a press at `position` drags: the point under it, or the
    /// corner of the cut under it. Taking a fillet or a chamfer by its curve is
    /// taking its corner, the one thing it moves with.
    pub fn point_to_drag_at(&self, position: DVec2, tolerance: f64) -> Option<PointId> {
        self.nearest_point(position, tolerance).or_else(|| {
            let cut = self.nearest_cut(position, tolerance)?;
            Some(self.corner_cuts()[cut.0].corner)
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
fn distance_to(curve: CutCurve, position: DVec2) -> f64 {
    nearest_on(curve, position).distance(position)
}

#[cfg(test)]
mod tests;
