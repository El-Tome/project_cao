//! Taking a stretch out of a trait, between two of the points sitting on it.

use glam::DVec2;

use crate::constraints::{Constraint, Dimension, DimensionTarget};
use crate::erased::Erased;
use crate::sketch::{Element, PointId, SegmentId, Sketch};
use crate::trimming::carrying::{Carried, Piece, gone, still_holds, still_measured, targets};

/// Below this, the two ends of a piece are the same place and the piece is no
/// trait at all. Far under anything a drawing tells apart: a gap this small
/// only ever comes of a cut landing on a point already sitting at an end.
pub(crate) const NO_LENGTH: f64 = 1e-9;

/// How far off the line a point may be and still be *on* the trait.
///
/// A fact about the drawing, never about the view. A point put on a trait
/// lands on it to within rounding; one a person merely placed nearby is
/// another point, and no amount of zooming out should turn it into an end of
/// this trait — which is what a cut makes of it.
pub(crate) const ON_THE_TRAIT: f64 = 1e-9;

/// What a cut left standing, and what it cost.
///
/// A rule or a value that spoke of the trait and of neither piece goes with the
/// trait. It is counted rather than named: the drawing loses it either way, and
/// what the user needs to know is that something was lost at all.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Trimmed {
    /// The pieces still drawn, in order from the trait's start. Empty when the
    /// whole trait went.
    pub pieces: Vec<SegmentId>,
    /// Rules that spoke of the trait and of neither piece.
    pub rules_dropped: usize,
    /// Values that measured the trait and measure neither piece.
    pub values_dropped: usize,
}

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
    pub fn trim(&mut self, segment: SegmentId, from: PointId, to: PointId) -> Option<Trimmed> {
        let cut = self.segments().get(segment.0).copied()?;
        let rules = self.constraints().to_vec();
        let values = self.dimensions().to_vec();
        let (start, end) = self.endpoints(segment);
        let span = end - start;
        let reach = span.length_squared();
        if reach == 0.0 {
            self.erase(Element::Segment(segment));
            return Some(Trimmed {
                pieces: Vec::new(),
                rules_dropped: gone(&rules, self.constraints()).len(),
                values_dropped: gone(&targets(&values), &targets(self.dimensions())).len(),
            });
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

        let carried = self.carried_by(segment, rules, values);

        let corner = self.corner_of_an_angle_on(segment, &carried.values);

        // A tangency carries its contact point away when it goes, since the
        // point would be left in mid-air. A cut stopping on that contact, or a
        // piece inheriting the tangency, is what puts it back.
        let standing: Vec<PointId> = [cut.start, low, high, cut.end]
            .into_iter()
            .filter(|point| !self.is_erased_point(*point))
            .collect();

        self.erase(Element::Segment(segment));

        for point in standing {
            Erased::unmark(&mut self.erased.points, point.0);
        }
        let dropped = gone(&carried.rules, self.constraints());
        let dropped_values = gone(&targets(&carried.values), &targets(self.dimensions()));

        let below = keeps_below.then(|| self.piece(cut.start, low, cut.construction));
        let above = keeps_above.then(|| self.piece(high, cut.end, cut.construction));

        for (fraction, point) in &carried.held {
            let piece = match fraction {
                _ if *fraction < opens => below,
                _ if *fraction > closes => above,
                _ => None,
            };
            if let Some(segment) = piece {
                self.add_constraint(Constraint::OnSegment {
                    point: *point,
                    segment,
                });
            }
        }

        let pieces: Vec<Piece> = [
            below.map(|id| (id, (0.0, opens))),
            above.map(|id| (id, (closes, 1.0))),
        ]
        .into_iter()
        .flatten()
        .map(|(id, spans)| Piece {
            id,
            spans,
            reaches_the_corner: corner.is_some_and(|corner| {
                let stops_at = self.segments()[id.0];
                stops_at.start == corner || stops_at.end == corner
            }),
        })
        .collect();

        self.hand_over(segment, &pieces, &carried);

        let rules_dropped = dropped
            .iter()
            .filter(|rule| {
                !self.stands_on_a_piece(**rule, segment, &pieces, carried.place_of(**rule))
            })
            .count();
        let values_dropped = dropped_values
            .iter()
            .filter(|value| {
                !self.measured_on_a_piece(
                    **value,
                    segment,
                    &pieces,
                    carried.place_measured(**value),
                )
            })
            .count();
        Some(Trimmed {
            pieces: pieces.iter().map(|piece| piece.id).collect(),
            rules_dropped,
            values_dropped,
        })
    }

    /// Everything standing that spoke of a trait, and where along it the ones
    /// fastened to a place on it sat.
    fn carried_by(
        &self,
        segment: SegmentId,
        rules: Vec<Constraint>,
        values: Vec<Dimension>,
    ) -> Carried {
        let (start, end) = self.endpoints(segment);
        let span = end - start;
        let reach = span.length_squared();
        let along = |place: DVec2| (place - start).dot(span) / reach;
        let place_of_point = |point: PointId| Some(along(self.points().get(point.0).copied()?));

        Carried {
            held: rules
                .iter()
                .filter_map(|rule| match rule {
                    Constraint::OnSegment { point, segment: on } if *on == segment => {
                        Some((place_of_point(*point)?, *point))
                    }
                    _ => None,
                })
                .collect(),
            fastened: rules
                .iter()
                .filter_map(|rule| match rule {
                    Constraint::Tangent {
                        segment: on,
                        at: Some(point),
                        ..
                    } if *on == segment => Some((*rule, place_of_point(*point)?)),
                    _ => None,
                })
                .collect(),
            measured_at: values
                .iter()
                .filter_map(|value| match value.target {
                    DimensionTarget::PointToSegment { point, segment: on } if on == segment => {
                        Some((value.target, place_of_point(point)?))
                    }
                    _ => None,
                })
                .collect(),
            rules,
            values,
        }
    }

    /// Puts back on the pieces everything the trait carried that follows them,
    /// the contact point of an inherited tangency included — the erasing of the
    /// trait took it away.
    fn hand_over(&mut self, cut: SegmentId, pieces: &[Piece], carried: &Carried) {
        for (rule, at) in &carried.fastened {
            if let Constraint::Tangent {
                at: Some(point), ..
            } = rule
                && pieces.iter().any(|piece| piece.holds(*at))
            {
                Erased::unmark(&mut self.erased.points, point.0);
            }
        }

        for piece in pieces {
            for rule in &carried.rules {
                if let Some(moved) = still_holds(*rule, cut, piece, carried.place_of(*rule)) {
                    self.add_constraint(moved);
                }
            }
            for value in &carried.values {
                let place = carried.place_measured(value.target);
                let Some(target) = still_measured(value.target, cut, piece, place) else {
                    continue;
                };
                self.set_dimension(target, value.value, value.driven);
                if let Some(offset) = value.offset {
                    self.offset_dimension(target, offset);
                }
            }
        }
    }

    /// Whether a rule the cut took away came back on one of the pieces, under
    /// the piece's name.
    fn stands_on_a_piece(
        &self,
        rule: Constraint,
        cut: SegmentId,
        pieces: &[Piece],
        place: Option<f64>,
    ) -> bool {
        pieces.iter().any(|piece| {
            let moved = match rule {
                Constraint::OnSegment { point, segment } if segment == cut => {
                    Some(Constraint::OnSegment {
                        point,
                        segment: piece.id,
                    })
                }
                other => still_holds(other, cut, piece, place),
            };
            moved.is_some_and(|moved| self.constraints().contains(&moved.normalised()))
        })
    }

    /// Whether a value the cut took away is read again on one of the pieces.
    fn measured_on_a_piece(
        &self,
        value: DimensionTarget,
        cut: SegmentId,
        pieces: &[Piece],
        place: Option<f64>,
    ) -> bool {
        pieces.iter().any(|piece| {
            still_measured(value, cut, piece, place)
                .is_some_and(|moved| self.dimension_of(moved).is_some())
        })
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

pub(crate) mod arc;
mod arc_carrying;
mod carrying;
#[cfg(test)]
mod tests;
