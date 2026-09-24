//! Taking a stretch out of a circle, between two of the points sitting on it.
//!
//! Unlike a trait or an arc, a circle has no start and no end, so a cut leaves
//! one piece rather than two — and that piece is no longer a circle. It comes
//! back as an arc of the very circle it was cut from.

use std::f64::consts::TAU;

use glam::DVec2;

use super::carrying::{gone, targets};
use super::{NO_LENGTH, ON_THE_TRAIT};
use crate::arc::ArcId;
use crate::constraints::{Constraint, Dimension, DimensionTarget};
use crate::erased::Erased;
use crate::sketch::{CircleId, Element, PointId, SegmentId, Sketch};

/// What a cut left standing of a circle, and what it cost.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CircleTrimmed {
    /// What is left, an arc of the circle. Nothing when the whole round went.
    pub arc: Option<ArcId>,
    /// Rules that spoke of the circle and do not speak of the arc.
    pub rules_dropped: usize,
    /// Values that measured the circle and do not measure the arc.
    pub values_dropped: usize,
}

impl Sketch {
    /// How far round a circle a place sits, as a fraction of a whole turn from
    /// due east.
    ///
    /// A circle has no start to count from, so the plane's own direction is
    /// what the fractions are read against. Only their order round the round
    /// matters, and that is the same whichever direction is called zero.
    fn round_the_circle(&self, id: CircleId, place: DVec2) -> Option<f64> {
        let centre = self.point(self.circles().get(id.0)?.center);
        Some((place - centre).to_angle().rem_euclid(TAU) / TAU)
    }

    /// The points sitting on a circle, each with how far round it they sit, in
    /// order from due east.
    ///
    /// Never the sketch origin, which every drawing owns and nobody placed, nor
    /// the centre, which is not on the rim at all.
    fn sitting_round_the_circle(&self, id: CircleId) -> Vec<(f64, PointId)> {
        let Some(circle) = self.circles().get(id.0).copied() else {
            return Vec::new();
        };
        if circle.radius < NO_LENGTH {
            return Vec::new();
        }
        let centre = self.point(circle.center);

        let mut sitting: Vec<(f64, PointId)> = self
            .live_points()
            .filter_map(|(point, place)| {
                let sits = !self.is_origin(point)
                    && point != circle.center
                    && (place.distance(centre) - circle.radius).abs() <= ON_THE_TRAIT;
                sits.then(|| Some((self.round_the_circle(id, place)?, point)))
                    .flatten()
            })
            .collect();
        sitting.sort_by(|first, second| first.0.total_cmp(&second.0));
        sitting
    }

    /// The stretch of a circle a click falls in: the two points it runs
    /// between, the first being the one the stretch leaves counter-clockwise.
    ///
    /// Nothing when the circle carries fewer than two points, which is the one
    /// case where there is no stretch to run between — the whole round is then
    /// what a cut takes.
    pub fn circle_stretch_at(&self, id: CircleId, at: DVec2) -> Option<(PointId, PointId)> {
        let sitting = self.sitting_round_the_circle(id);
        if sitting.len() < 2 {
            return None;
        }
        let round = self.round_the_circle(id, at)?;
        let opens = sitting
            .iter()
            .rposition(|(turn, _)| *turn <= round)
            // Past the last point the round wraps, and the stretch the click
            // fell in is the one that closes on the first.
            .unwrap_or(sitting.len() - 1);
        Some((sitting[opens].1, sitting[(opens + 1) % sitting.len()].1))
    }

    /// Takes the stretch running counter-clockwise from the first named point
    /// round to the second out of a circle, and leaves the rest of the round as
    /// an arc.
    ///
    /// The order of the two points is what says which of the two stretches
    /// between them goes: a circle has no start to read it off, where an arc
    /// does.
    ///
    /// Named nothing to cut between — a circle carrying fewer than two points —
    /// the stretch "between two points" is the whole round, and the circle is
    /// taken away entire.
    pub fn trim_circle(
        &mut self,
        id: CircleId,
        between: Option<(PointId, PointId)>,
    ) -> Option<CircleTrimmed> {
        let circle = self.circles().get(id.0).copied()?;
        if self.is_erased_circle(id) {
            return None;
        }
        if let Some((from, to)) = between
            && !self.is_a_stretch_of(id, from, to)
        {
            return None;
        }
        let rules = self.constraints().to_vec();
        let values = self.dimensions().to_vec();
        let kept = between.map(|(from, to)| self.stretch_kept(id, from, to));

        self.erase(Element::Circle(id));
        let dropped = gone(&rules, self.constraints());
        let dropped_values = gone(&targets(&values), &targets(self.dimensions()));

        let arc = between.map(|(from, to)| match circle.construction {
            true => self.add_construction_arc(circle.center, to, from),
            false => self.add_arc(circle.center, to, from),
        });
        if let (Some(arc), Some(kept)) = (arc, &kept) {
            self.hand_the_round_over(id, arc, kept, &rules, &values);
        }

        let onto = arc.zip(kept);
        let rules_dropped = dropped
            .iter()
            .filter(|rule| {
                !onto.as_ref().is_some_and(|(arc, kept)| {
                    self.moved_onto(**rule, id, *arc, kept)
                        .is_some_and(|moved| self.constraints().contains(&moved.normalised()))
                })
            })
            .count();
        let values_dropped = dropped_values
            .iter()
            .filter(|value| {
                !arc.is_some_and(|arc| {
                    read_again(**value, id, arc)
                        .is_some_and(|target| self.dimension_of(target).is_some())
                })
            })
            .count();

        Some(CircleTrimmed {
            arc,
            rules_dropped,
            values_dropped,
        })
    }

    /// Whether a stretch named between two points is one this circle can give
    /// up: two points that are not the same one, both still drawn, and both
    /// sitting on its rim.
    ///
    /// Asked again here rather than trusted from the click, because the two
    /// points are recorded once and replayed afterwards: an earlier step edited
    /// since can have carried them off the rim, and an arc built from them
    /// would be an arc of nothing.
    fn is_a_stretch_of(&self, id: CircleId, from: PointId, to: PointId) -> bool {
        let sitting = self.sitting_round_the_circle(id);
        let on_the_rim = |point: PointId| sitting.iter().any(|(_, sits)| *sits == point);
        from != to && on_the_rim(from) && on_the_rim(to)
    }

    /// What a rule the circle carried becomes on the arc a cut would leave, for
    /// a reckoning made before the cut: `Kept` stays this module's business.
    pub(super) fn circle_hands_over(
        &self,
        rule: Constraint,
        cut: CircleId,
        between: Option<(PointId, PointId)>,
        arc: ArcId,
    ) -> Option<Constraint> {
        let (from, to) = between?;
        self.moved_onto(rule, cut, arc, &self.stretch_kept(cut, from, to))
    }

    /// What a rule laid on a circle becomes on the arc a cut left of it, or
    /// nothing when the arc no longer carries what the rule spoke of.
    ///
    /// An arc is a circle a sweep was taken from, so a reach held against
    /// another round is held still — under the name of a rule that pairs the
    /// two kinds. A brush follows only where it touched.
    fn moved_onto(
        &self,
        rule: Constraint,
        cut: CircleId,
        arc: ArcId,
        kept: &Kept,
    ) -> Option<Constraint> {
        match rule {
            Constraint::EqualRadius { first, second } if first == cut || second == cut => {
                let other = match first == cut {
                    true => second,
                    false => first,
                };
                Some(Constraint::EqualRadiusArcCircle { arc, circle: other })
            }
            Constraint::Tangent {
                circle,
                segment,
                at,
            } if circle == cut => {
                let touches = self.contact_round(cut, segment, at)?;
                kept.holds(touches)
                    .then_some(Constraint::ArcTangent { arc, segment, at })
            }
            Constraint::OnCircle { point, circle } if circle == cut => {
                let place = self.points().get(point.0).copied()?;
                let stands = self.round_the_circle(cut, place)?;
                kept.holds(stands)
                    .then_some(Constraint::OnArc { point, arc })
            }
            _ => None,
        }
    }

    /// How far round the circle a trait brushing it touches: the point the
    /// tangency was given, or else the foot of the centre on the line, which is
    /// the only place a line can graze a circle it is tangent to.
    fn contact_round(&self, id: CircleId, segment: SegmentId, at: Option<PointId>) -> Option<f64> {
        let centre = self.circles().get(id.0)?.center;
        let touches = match at {
            Some(point) => self.points().get(point.0).copied()?,
            None => self.foot_on_segment(centre, segment)?,
        };
        self.round_the_circle(id, touches)
    }

    /// What the cut leaves standing of the round, as the stretch of turn it
    /// covers counted from the point the cut opens on.
    ///
    /// A place is on it when it sits at or past where the cut closes, reading
    /// round the same way the cut does.
    fn stretch_kept(&self, id: CircleId, from: PointId, to: PointId) -> Kept {
        let turn_of = |point: PointId| {
            self.points()
                .get(point.0)
                .and_then(|place| self.round_the_circle(id, *place))
        };
        Kept {
            opens: turn_of(from),
            closes: turn_of(to),
        }
    }

    /// Puts on the arc everything the circle carried that follows it.
    fn hand_the_round_over(
        &mut self,
        cut: CircleId,
        arc: ArcId,
        kept: &Kept,
        rules: &[Constraint],
        values: &[Dimension],
    ) {
        for rule in rules {
            let Some(moved) = self.moved_onto(*rule, cut, arc, kept) else {
                continue;
            };
            // A tangency carries its contact point away when it goes, since the
            // point would be left in mid-air. One the arc inherits puts it back.
            if let Constraint::Tangent {
                at: Some(point), ..
            } = *rule
            {
                Erased::unmark(&mut self.erased.points, point.0);
            }
            self.add_constraint(moved);
        }
        for value in values {
            let Some((target, measured)) = still_measured(value, cut, arc) else {
                continue;
            };
            // A diameter read again as the arc's radius is half the number it
            // was, and the drawing cannot say what half of what it was written
            // as would be: the number goes over, the writing does not.
            let written = value.written.clone().filter(|_| measured == value.value);
            let carried = Dimension {
                value: measured,
                written,
                ..value.clone()
            };
            self.carry_dimension(target, &carried);
        }
    }
}

/// The stretch of a round a cut leaves standing, named by its two ends.
struct Kept {
    opens: Option<f64>,
    closes: Option<f64>,
}

impl Kept {
    /// Whether a place sits on the stretch that stayed.
    ///
    /// The cut runs counter-clockwise from where it opens to where it closes,
    /// so what stayed is everything from `closes` round to `opens` — the two
    /// ends included, since they are the arc's own.
    fn holds(&self, turn: f64) -> bool {
        let (Some(opens), Some(closes)) = (self.opens, self.closes) else {
            return false;
        };
        let round = |turn: f64| (turn - opens).rem_euclid(1.0);
        let gone = round(closes);
        let place = round(turn);
        place >= gone || place <= ON_THE_ROUND || gone <= ON_THE_ROUND
    }
}

/// How far round a circle two places may sit and still be the same place, as a
/// fraction of a whole turn. Far under anything a drawing tells apart: it only
/// ever catches a cut whose end and a contact are one and the same point.
const ON_THE_ROUND: f64 = 1e-12;

/// Where a value read on a circle is read again once the cut is made.
///
/// An arc is dimensioned by its reach everywhere, so what was read right across
/// the round comes back as half of it: `Ø 40` reads `R 20`.
pub(super) fn read_again(
    value: DimensionTarget,
    cut: CircleId,
    arc: ArcId,
) -> Option<DimensionTarget> {
    match value {
        DimensionTarget::Radius(circle) | DimensionTarget::Diameter(circle) if circle == cut => {
            Some(DimensionTarget::ArcRadius(arc))
        }
        _ => None,
    }
}

/// The same, with what the value now reads.
fn still_measured(value: &Dimension, cut: CircleId, arc: ArcId) -> Option<(DimensionTarget, f64)> {
    let target = read_again(value.target, cut, arc)?;
    let measured = match value.target {
        DimensionTarget::Diameter(_) => value.value / 2.0,
        _ => value.value,
    };
    Some((target, measured))
}
