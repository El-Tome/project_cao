//! Taking a stretch out of a trait, between two of the points sitting on it.

use glam::DVec2;

use crate::constraints::{Constraint, Dimension, DimensionTarget};
use crate::erased::Erased;
use crate::sketch::{Element, PointId, SegmentId, Sketch};

/// Below this, the two ends of a piece are the same place and the piece is no
/// trait at all. Far under anything a drawing tells apart: a gap this small
/// only ever comes of a cut landing on a point already sitting at an end.
const NO_LENGTH: f64 = 1e-9;

/// How far off the line a point may be and still be *on* the trait.
///
/// A fact about the drawing, never about the view. A point put on a trait
/// lands on it to within rounding; one a person merely placed nearby is
/// another point, and no amount of zooming out should turn it into an end of
/// this trait — which is what a cut makes of it.
const ON_THE_TRAIT: f64 = 1e-9;

impl Sketch {
    /// The points sitting on a trait, each with how far along it they sit, in
    /// order from its start to its end.
    ///
    /// Its own two ends are the first and the last of them. Between the two
    /// falls anything a rule holds on its body, wherever the solver left it,
    /// and anything that genuinely lies on it — but never the sketch origin on
    /// that second count alone: every drawing owns one, it is not drawn like
    /// the others, and nobody put it there.
    fn sitting_along(&self, segment: SegmentId) -> Vec<(f64, PointId)> {
        let Some(trait_) = self.segments().get(segment.0).copied() else {
            return Vec::new();
        };
        let (start, end) = self.endpoints(segment);
        let span = end - start;
        let reach = span.length_squared();
        if reach == 0.0 {
            return Vec::new();
        }
        let ends = [trait_.start, trait_.end];

        let mut sitting: Vec<(f64, PointId)> = self
            .live_points()
            .filter_map(|(id, place)| {
                let fraction = (place - start).dot(span) / reach;
                let foot = start + span * fraction;
                let sits = ends.contains(&id)
                    || self.is_held_on(id, segment)
                    || (!self.is_origin(id) && place.distance(foot) <= ON_THE_TRAIT);
                ((0.0..=1.0).contains(&fraction) && sits).then_some((fraction, id))
            })
            .collect();
        sitting.sort_by(|first, second| first.0.total_cmp(&second.0));
        sitting
    }

    fn is_held_on(&self, point: PointId, segment: SegmentId) -> bool {
        self.constraints()
            .contains(&Constraint::OnSegment { point, segment })
    }

    /// Takes the stretch between `from` and `to` out of a trait, and leaves
    /// what is left of it as traits of their own.
    ///
    /// The two named points survive, as the ends the pieces now stop at. Named
    /// the same point twice, nothing is taken away and the trait is merely cut
    /// in two there — which is what a fillet, a chamfer and a split each need
    /// of a trait before they can do their own work.
    ///
    /// Nothing when the cut cannot be made: a trait or a point the drawing does
    /// not have, a point that does not fall between the trait's own ends, or a
    /// cut that would take nothing away and hand back the whole trait.
    pub fn trim(
        &mut self,
        segment: SegmentId,
        from: PointId,
        to: PointId,
    ) -> Option<Vec<SegmentId>> {
        let cut = self.segments().get(segment.0).copied()?;
        let (start, end) = self.endpoints(segment);
        let span = end - start;
        let reach = span.length_squared();
        if reach == 0.0 {
            self.erase(Element::Segment(segment));
            return Some(Vec::new());
        }
        let along = |place: DVec2| (place - start).dot(span) / reach;

        let from_at = along(self.points().get(from.0).copied()?);
        let to_at = along(self.points().get(to.0).copied()?);
        if !(0.0..=1.0).contains(&from_at) || !(0.0..=1.0).contains(&to_at) {
            return None;
        }
        let (low, high) = match from_at <= to_at {
            true => (from, to),
            false => (to, from),
        };
        let (opens, closes) = (from_at.min(to_at), from_at.max(to_at));

        let keeps_below = self.is_a_piece(cut.start, low);
        let keeps_above = self.is_a_piece(high, cut.end);
        let hands_the_trait_back =
            (keeps_below && !keeps_above && self.point(low).distance(end) <= NO_LENGTH)
                || (keeps_above && !keeps_below && self.point(high).distance(start) <= NO_LENGTH);
        if hands_the_trait_back {
            return None;
        }

        let held: Vec<(f64, PointId)> = self
            .constraints()
            .iter()
            .filter_map(|rule| match rule {
                Constraint::OnSegment { point, segment: on } if *on == segment => {
                    Some((along(self.points().get(point.0).copied()?), *point))
                }
                _ => None,
            })
            .collect();

        let rules = self.constraints().to_vec();
        let values = self.dimensions().to_vec();
        let corner = self.corner_of_an_angle_on(segment, &values);

        // A tangency carries its contact point away when it goes, since the
        // point would be left in mid-air. Stopping a cut on that contact is
        // the case where it would not: it is an end of a trait now, and that
        // is what holds it.
        let standing: Vec<PointId> = [cut.start, low, high, cut.end]
            .into_iter()
            .filter(|point| !self.is_erased_point(*point))
            .collect();

        self.erase(Element::Segment(segment));

        for point in standing {
            Erased::unmark(&mut self.erased.points, point.0);
        }

        let below = keeps_below.then(|| self.piece(cut.start, low, cut.construction));
        let above = keeps_above.then(|| self.piece(high, cut.end, cut.construction));

        for (fraction, point) in held {
            let piece = match fraction {
                _ if fraction < opens => below,
                _ if fraction > closes => above,
                _ => None,
            };
            if let Some(segment) = piece {
                self.add_constraint(Constraint::OnSegment { point, segment });
            }
        }

        for piece in [below, above].into_iter().flatten() {
            let reaches_the_corner = corner.is_some_and(|corner| {
                let stops_at = self.segments()[piece.0];
                stops_at.start == corner || stops_at.end == corner
            });
            for rule in &rules {
                if let Some(moved) = about_direction(*rule, segment, piece) {
                    self.add_constraint(moved);
                }
            }
            for value in &values {
                let Some(target) = still_measured(value.target, segment, piece, reaches_the_corner)
                else {
                    continue;
                };
                self.set_dimension(target, value.value, value.driven);
                if let Some(offset) = value.offset {
                    self.offset_dimension(target, offset);
                }
            }
        }

        Some(below.into_iter().chain(above).collect())
    }

    /// The place an angle measured against this trait is taken at: the point
    /// the two traits share. Only the piece that still reaches it keeps the
    /// angle, since that is where it was read.
    fn corner_of_an_angle_on(&self, segment: SegmentId, values: &[Dimension]) -> Option<PointId> {
        values.iter().find_map(|value| match value.target {
            DimensionTarget::Angle { first, second } if first == segment || second == segment => {
                let other = match first == segment {
                    true => second,
                    false => first,
                };
                self.shared_corner(segment, other)
                    .map(|(corner, _, _)| corner)
            }
            _ => None,
        })
    }

    /// Whether what runs between these two points is a trait at all.
    fn is_a_piece(&self, from: PointId, to: PointId) -> bool {
        from != to && self.point(from).distance(self.point(to)) > NO_LENGTH
    }

    fn piece(&mut self, from: PointId, to: PointId, construction: bool) -> SegmentId {
        match construction {
            true => self.add_construction_segment(from, to),
            false => self.add_segment(from, to),
        }
    }

    /// The stretch of a trait a click falls in: the two points it runs
    /// between.
    ///
    /// Nothing when the trait carries no stretch at all — a trait with no
    /// length, which is the one case where there is nothing to run between.
    pub fn stretch_at(&self, segment: SegmentId, at: DVec2) -> Option<(PointId, PointId)> {
        let (start, end) = self.endpoints(segment);
        let span = end - start;
        let reach = span.length_squared();
        if reach == 0.0 {
            return None;
        }

        // Clamped, so that a click read as just past an end still asks for the
        // stretch that ends there rather than for nothing at all.
        let fraction = ((at - start).dot(span) / reach).clamp(0.0, 1.0);
        self.sitting_along(segment)
            .windows(2)
            .find(|stretch| fraction <= stretch[1].0)
            .map(|stretch| (stretch[0].1, stretch[1].1))
    }
}

#[cfg(test)]
mod tests;

/// The same rule, said of a piece of the trait it named.
///
/// Only what a rule says about *direction* survives a cut: the pieces lie on
/// the line the trait lay on, so they stand to everything else exactly as it
/// did. A rule about its length speaks of a trait that is no longer there.
fn about_direction(rule: Constraint, cut: SegmentId, piece: SegmentId) -> Option<Constraint> {
    let moved = |id: SegmentId| match id == cut {
        true => piece,
        false => id,
    };
    match rule {
        Constraint::Perpendicular { first, second } if first == cut || second == cut => {
            Some(Constraint::Perpendicular {
                first: moved(first),
                second: moved(second),
            })
        }
        Constraint::Parallel { first, second } if first == cut || second == cut => {
            Some(Constraint::Parallel {
                first: moved(first),
                second: moved(second),
            })
        }
        Constraint::Collinear { first, second } if first == cut || second == cut => {
            Some(Constraint::Collinear {
                first: moved(first),
                second: moved(second),
            })
        }
        Constraint::AxisCollinear { segment, axis } if segment == cut => {
            Some(Constraint::AxisCollinear {
                segment: piece,
                axis,
            })
        }
        _ => None,
    }
}

/// What a value measured against the trait still measures, once the trait is a
/// piece of itself.
///
/// An angle against an axis is read off the direction, which both pieces
/// inherit. An angle at a corner belongs to whichever piece still reaches that
/// corner. A length measures a trait that is shorter than what was typed, and
/// says nothing about either piece.
fn still_measured(
    value: DimensionTarget,
    cut: SegmentId,
    piece: SegmentId,
    reaches_the_corner: bool,
) -> Option<DimensionTarget> {
    let moved = |id: SegmentId| match id == cut {
        true => piece,
        false => id,
    };
    match value {
        DimensionTarget::AxisAngle { segment, axis } if segment == cut => {
            Some(DimensionTarget::AxisAngle {
                segment: piece,
                axis,
            })
        }
        DimensionTarget::Angle { first, second }
            if (first == cut || second == cut) && reaches_the_corner =>
        {
            Some(DimensionTarget::Angle {
                first: moved(first),
                second: moved(second),
            })
        }
        _ => None,
    }
}
