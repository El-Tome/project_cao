//! Taking a stretch out of an arc, between two of the points sitting on it.

use std::f64::consts::TAU;

use glam::DVec2;

use super::arc_carrying::{Carried, Piece, still_holds, still_measured};
use super::carrying::{gone, targets};
use super::{NO_LENGTH, ON_THE_TRAIT};
use crate::arc::ArcId;
use crate::constraints::Constraint;
use crate::sketch::{Element, PointId, SegmentId, Sketch};

/// Below this an arc runs nowhere at all: its two ends are the same direction
/// out of its centre, and there is no fraction of a sweep to speak of.
const NO_SWEEP: f64 = 1e-9;

/// What a cut left standing of an arc, and what it cost.
///
/// The same reckoning as a trait's `Trimmed`: what spoke of the curve and of
/// neither piece is counted rather than named, since the drawing loses it
/// either way and what the user needs to know is that something was lost.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ArcTrimmed {
    /// The pieces still drawn, in order from the arc's start. Empty when the
    /// whole curve went.
    pub pieces: Vec<ArcId>,
    /// Rules that spoke of the arc and of neither piece.
    pub rules_dropped: usize,
    /// Values that measured the arc and measure neither piece.
    pub values_dropped: usize,
}

impl Sketch {
    /// How far round an arc a place sits, as a fraction of its sweep.
    ///
    /// Nothing for an arc that runs nowhere, and more than one for a place the
    /// curve never reaches: the rest of the circle it is a piece of counts on
    /// up from its end back round to its start.
    fn round_the_arc(&self, id: ArcId, place: DVec2) -> Option<f64> {
        let arc = self.arcs().get(id.0).copied()?;
        let sweep = self.arc_sweep(id);
        if sweep < NO_SWEEP {
            return None;
        }
        let centre = self.point(arc.center);
        let from = (self.point(arc.start) - centre).to_angle();
        Some(((place - centre).to_angle() - from).rem_euclid(TAU) / sweep)
    }

    /// The same for one of the drawing's points, the arc's own two ends read
    /// off the order they are held in rather than off an angle that rounding
    /// could send the long way about.
    fn point_round_the_arc(&self, id: ArcId, point: PointId) -> Option<f64> {
        let arc = self.arcs().get(id.0).copied()?;
        match point {
            _ if point == arc.start => Some(0.0),
            _ if point == arc.end => Some(1.0),
            _ => self.round_the_arc(id, self.points().get(point.0).copied()?),
        }
    }

    /// The points sitting on an arc, each with how far round it they sit, in
    /// order from its start to its end.
    ///
    /// Its own two ends are the first and the last of them, and between them
    /// falls anything the drawing genuinely put on the curve — but never the
    /// sketch origin, which every drawing owns and nobody placed, nor the
    /// centre, which is not on the curve at all.
    fn sitting_round(&self, id: ArcId) -> Vec<(f64, PointId)> {
        let Some(arc) = self.arcs().get(id.0).copied() else {
            return Vec::new();
        };
        let centre = self.point(arc.center);
        let reach = self.arc_radius(id);
        if reach < NO_LENGTH {
            return Vec::new();
        }

        let mut sitting: Vec<(f64, PointId)> = self
            .live_points()
            .filter_map(|(point, place)| {
                if point == arc.start || point == arc.end {
                    return Some((self.point_round_the_arc(id, point)?, point));
                }
                let round = self.round_the_arc(id, place)?;
                let sits = !self.is_origin(point)
                    && point != arc.center
                    && (place.distance(centre) - reach).abs() <= ON_THE_TRAIT;
                ((0.0..=1.0).contains(&round) && sits).then_some((round, point))
            })
            .collect();
        sitting.sort_by(|first, second| first.0.total_cmp(&second.0));
        sitting
    }

    /// The stretch of an arc a click falls in: the two points it runs between.
    ///
    /// Nothing when the curve runs nowhere, which is the one case where there
    /// is nothing to run between.
    pub fn arc_stretch_at(&self, id: ArcId, at: DVec2) -> Option<(PointId, PointId)> {
        let round = self.brought_onto(id, self.round_the_arc(id, at)?);
        self.sitting_round(id)
            .windows(2)
            .find(|stretch| round <= stretch[1].0)
            .map(|stretch| (stretch[0].1, stretch[1].1))
    }

    /// A place read off the whole circle, brought back onto the arc.
    ///
    /// A click read as just past an end asks for the stretch that ends there
    /// rather than for nothing at all. Which end it went past is the whole
    /// question, and a line never has to ask it: the rest of the circle counts
    /// on from the arc's end round to its start, so the halfway mark of that
    /// gap is what says whether the click fell off the far end or short of the
    /// near one.
    fn brought_onto(&self, id: ArcId, round: f64) -> f64 {
        if round <= 1.0 {
            return round;
        }
        let whole = TAU / self.arc_sweep(id);
        match round < (1.0 + whole) * 0.5 {
            true => 1.0,
            false => 0.0,
        }
    }

    /// Takes the stretch between `from` and `to` out of an arc, and leaves what
    /// is left of it as arcs of their own, around the same centre.
    ///
    /// The two named points survive, as the ends the pieces now stop at. Named
    /// the same point twice, nothing is taken away and the curve is merely cut
    /// in two there.
    ///
    /// Nothing when the cut cannot be made: an arc or a point the drawing does
    /// not have, a point that does not sit on the curve, or a cut that would
    /// take nothing away and hand the whole arc back.
    pub fn trim_arc(&mut self, id: ArcId, from: PointId, to: PointId) -> Option<ArcTrimmed> {
        let arc = self.arcs().get(id.0).copied()?;
        if self.is_erased_arc(id) || self.is_erased_point(from) || self.is_erased_point(to) {
            return None;
        }
        let from_at = self.point_round_the_arc(id, from)?;
        let to_at = self.point_round_the_arc(id, to)?;
        if !(0.0..=1.0).contains(&from_at) || !(0.0..=1.0).contains(&to_at) {
            return None;
        }
        let (low, high) = match from_at <= to_at {
            true => (from, to),
            false => (to, from),
        };
        let (opens, closes) = (from_at.min(to_at), from_at.max(to_at));

        let keeps_below = self.is_a_piece(arc.start, low);
        let keeps_above = self.is_a_piece(high, arc.end);
        let (start, end) = (self.point(arc.start), self.point(arc.end));
        let hands_the_arc_back =
            (keeps_below && !keeps_above && self.point(low).distance(end) <= NO_LENGTH)
                || (keeps_above && !keeps_below && self.point(high).distance(start) <= NO_LENGTH);
        if hands_the_arc_back {
            return None;
        }

        let carried = self.carried_round(id);
        self.erase(Element::Arc(id));
        let dropped = gone(&carried.rules, self.constraints());
        let dropped_values = gone(&targets(&carried.values), &targets(self.dimensions()));

        let below =
            keeps_below.then(|| self.arc_piece(arc.center, arc.start, low, arc.construction));
        let above =
            keeps_above.then(|| self.arc_piece(arc.center, high, arc.end, arc.construction));

        let pieces: Vec<Piece> = [
            below.map(|id| (id, (0.0, opens))),
            above.map(|id| (id, (closes, 1.0))),
        ]
        .into_iter()
        .flatten()
        .enumerate()
        .map(|(rank, (id, spans))| Piece {
            id,
            spans,
            carries_the_reach: rank == 0,
        })
        .collect();

        if let (Some(below), Some(above)) = (below, above) {
            self.add_constraint(Constraint::EqualRadiusArc {
                first: below,
                second: above,
            });
        }
        self.hand_round(id, &pieces, &carried);

        let rules_dropped = dropped
            .iter()
            .filter(|rule| {
                !pieces.iter().any(|piece| {
                    still_holds(**rule, id, piece, carried.place_of(**rule))
                        .is_some_and(|moved| self.constraints().contains(&moved.normalised()))
                })
            })
            .count();
        let values_dropped = dropped_values
            .iter()
            .filter(|value| {
                !pieces.iter().any(|piece| {
                    still_measured(**value, id, piece)
                        .is_some_and(|moved| self.dimension_of(moved).is_some())
                })
            })
            .count();

        Some(ArcTrimmed {
            pieces: pieces.iter().map(|piece| piece.id).collect(),
            rules_dropped,
            values_dropped,
        })
    }

    fn carried_round(&self, id: ArcId) -> Carried {
        Carried {
            fastened: self
                .constraints()
                .iter()
                .filter_map(|rule| match rule {
                    Constraint::ArcTangent { arc, segment, at } if *arc == id => {
                        let touches = self.contact(self.arc(id).center, *segment, *at)?;
                        Some((*rule, self.round_the_arc(id, touches)?))
                    }
                    _ => None,
                })
                .collect(),
            rules: self.constraints().to_vec(),
            values: self.dimensions().to_vec(),
        }
    }

    /// Where a tangency touches: the point it was given, or else the foot of
    /// the arc's centre on the line, which is the only place a line can graze a
    /// circle it is tangent to.
    fn contact(&self, centre: PointId, segment: SegmentId, at: Option<PointId>) -> Option<DVec2> {
        match at {
            Some(point) => self.points().get(point.0).copied(),
            None => self.foot_on_segment(centre, segment),
        }
    }

    /// Puts back on the pieces everything the arc carried that follows them.
    fn hand_round(&mut self, cut: ArcId, pieces: &[Piece], carried: &Carried) {
        for piece in pieces {
            for rule in &carried.rules {
                if let Some(moved) = still_holds(*rule, cut, piece, carried.place_of(*rule)) {
                    self.add_constraint(moved);
                }
            }
            for value in &carried.values {
                let Some(target) = still_measured(value.target, cut, piece) else {
                    continue;
                };
                self.set_dimension(target, value.value, value.driven);
                if let Some(offset) = value.offset {
                    self.offset_dimension(target, offset);
                }
            }
        }
    }

    fn arc_piece(
        &mut self,
        centre: PointId,
        from: PointId,
        to: PointId,
        construction: bool,
    ) -> ArcId {
        match construction {
            true => self.add_construction_arc(centre, from, to),
            false => self.add_arc(centre, from, to),
        }
    }
}
