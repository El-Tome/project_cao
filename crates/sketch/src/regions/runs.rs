//! The runs of an outline, each taken as what it is: a straight step between
//! two corners, a piece of a circle about its centre, or a piece of an ellipse.
//!
//! The measure reads them to answer the curve's own surface and length; a
//! kernel that raises the outline reads them to build its walls on the circle
//! itself rather than on the steps it was sampled into.

use glam::DVec2;

use super::Outline;
use crate::edges::half_edge::Bend;

/// What one run of an outline goes along, from the corner it leaves to the
/// corner the next run leaves.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Leg {
    Straight,
    /// Along the circle about `centre`, `turned` radians round it, signed the
    /// way the loop is walked: the short way, the long way, or — a whole
    /// circle, which has a single corner — a whole turn.
    Round {
        centre: DVec2,
        turned: f64,
    },
    /// Along an ellipse, which no exact kernel here can raise yet.
    Oval,
}

/// One run as the walk reads it: its two ends, and the curve it follows with
/// how far round, signed, when it is not straight.
pub(super) struct Span {
    pub(super) from: DVec2,
    pub(super) to: DVec2,
    pub(super) along: Option<(Bend, f64)>,
}

impl Outline {
    /// Every run of the loop in the order it is walked, each from its own
    /// corner, the next run's corner being where it ends.
    pub fn runs(&self) -> Vec<(DVec2, Leg)> {
        self.spans()
            .into_iter()
            .map(|span| {
                let leg = match span.along {
                    None => Leg::Straight,
                    Some((Bend::Round(centre), turned)) => Leg::Round { centre, turned },
                    Some((Bend::Oval(_), _)) => Leg::Oval,
                };
                (span.from, leg)
            })
            .collect()
    }

    /// Walks the loop once, a run at a time.
    ///
    /// How far a curved run turns is read off the sampled places rather than
    /// off its two ends, since the ends alone cannot tell the short way round
    /// from the long one, nor either from the whole curve.
    pub(super) fn spans(&self) -> Vec<Span> {
        let places = self.points.len();
        if places < 2 {
            return Vec::new();
        }
        let mut spans = Vec::new();
        let mut at = 0;
        while at < places {
            let Some(run) = self.curves.get(at).copied().flatten() else {
                spans.push(Span {
                    from: self.points[at],
                    to: self.points[(at + 1) % places],
                    along: None,
                });
                at += 1;
                continue;
            };
            let last = (at..places)
                .take_while(|step| self.curves.get(*step).copied().flatten() == Some(run))
                .last()
                .unwrap_or(at);
            // `arc_regions` numbers a run by where its bend lands, so this
            // never reaches past the end.
            let bend = self.bends[run];
            spans.push(Span {
                from: self.points[at],
                to: self.points[(last + 1) % places],
                along: Some((bend, self.turned(bend, at, last))),
            });
            at = last + 1;
        }
        spans
    }

    /// How far round its curve a run turns, signed the way the loop is walked.
    ///
    /// Added up one sampled step at a time. Taken from the two ends instead it
    /// would be ambiguous exactly where it matters: the same two ends bound the
    /// short way round and the long one, and a whole circle has no two ends at
    /// all.
    fn turned(&self, bend: Bend, first: usize, last: usize) -> f64 {
        let places = self.points.len();
        let turn = |place: DVec2| match bend {
            Bend::Round(centre) => (place - centre).to_angle(),
            Bend::Oval(drawn) => drawn.turn_on(place),
        };
        let mut total = 0.0;
        for step in first..=last {
            let (from, to) = (
                turn(self.points[step]),
                turn(self.points[(step + 1) % places]),
            );
            // Each sampled step is a small fraction of the curve, so the turn
            // across it is the one under half a turn — which is what lets a
            // run of any length, a whole circle included, be added up from
            // steps that are never themselves ambiguous.
            total += shortest_way_round(to - from);
        }
        total
    }
}

/// The same turn brought back between half a turn either way.
fn shortest_way_round(turn: f64) -> f64 {
    let whole = std::f64::consts::TAU;
    let turn = turn % whole;
    if turn > std::f64::consts::PI {
        return turn - whole;
    }
    if turn < -std::f64::consts::PI {
        return turn + whole;
    }
    turn
}

#[cfg(test)]
mod tests;
