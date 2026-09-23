//! How much surface a closed area holds, and how far it is round it.
//!
//! Neither is read off the steps the outline was sampled into. A circle of
//! radius ten read off its own steps comes out under 314 mm² and under 62.8 mm
//! round, and a measuring tool that answers a value it knows to be wrong is
//! one nobody trusts twice. What the sampling is for is tinting and
//! triangulating; what a measure wants is the curve, and `Outline::bends` is
//! where the face walk left it.
//!
//! Both are read by walking the loop once, taking each run as what it is: a
//! straight step between two places, a piece of a circle about its centre, or
//! a piece of an ellipse about its two axes.

use glam::DVec2;

use super::{Outline, Region};
use crate::edges::half_edge::Bend;

impl Region {
    /// The surface the area holds, what it is hollow of taken out.
    ///
    /// The holes come out because that is the matter an extrusion would make
    /// of it — the same answer the tinting and the triangulation give, and the
    /// one a machinist means by the word.
    pub fn area(&self) -> f64 {
        self.outline.area() - self.holes.iter().map(Outline::area).sum::<f64>()
    }

    /// How far it is round the area, the outside only.
    ///
    /// A hole has a perimeter of its own, read by clicking inside it. Adding
    /// the two would answer a question — how much there is to cut — that
    /// nobody asked here, and would make the number disagree with the outline
    /// that is lit while it is shown.
    pub fn perimeter(&self) -> f64 {
        self.outline.perimeter()
    }
}

impl Outline {
    /// The surface this loop encloses.
    ///
    /// Green's theorem, run by run: the surface inside a closed loop is
    /// `½∮(x dy − y dx)`, which a straight step and a piece of a curve each
    /// answer in closed form. Summing the steps a curve was sampled into would
    /// answer the polygon drawn through them instead.
    pub fn area(&self) -> f64 {
        self.walked(
            |from, to| from.perp_dot(to),
            |bend, from, to, turned| match bend {
                Bend::Round(centre) => {
                    let radius = from.distance(centre);
                    centre.perp_dot(to - from) + radius * radius * turned
                }
                Bend::Oval(drawn) => {
                    drawn.centre.perp_dot(to - from)
                        + drawn.first.perp_dot(drawn.second_axis()) * turned
                }
            },
        ) / 2.0
    }

    /// How far it is round this loop.
    pub fn perimeter(&self) -> f64 {
        self.walked(
            |from, to| from.distance(to),
            |bend, from, _, turned| match bend {
                Bend::Round(centre) => from.distance(centre) * turned.abs(),
                Bend::Oval(drawn) => drawn.run_of(drawn.turn_on(from), turned),
            },
        )
    }

    /// Walks the loop once, handing each straight step to `straight` and each
    /// run of a curve to `curved`, and adds up what they say.
    ///
    /// `curved` is given the run's two ends and how far round its curve it
    /// turned, signed — which is read off the sampled places rather than off
    /// the two ends, since the ends alone cannot tell the short way round from
    /// the long one, nor either from the whole curve.
    fn walked(
        &self,
        straight: impl Fn(DVec2, DVec2) -> f64,
        curved: impl Fn(Bend, DVec2, DVec2, f64) -> f64,
    ) -> f64 {
        let places = self.points.len();
        if places < 2 {
            return 0.0;
        }
        let mut total = 0.0;
        let mut at = 0;
        while at < places {
            let to = self.points[(at + 1) % places];
            let Some(run) = self.curves.get(at).copied().flatten() else {
                total += straight(self.points[at], to);
                at += 1;
                continue;
            };
            let last = (at..places)
                .take_while(|step| self.curves.get(*step).copied().flatten() == Some(run))
                .last()
                .unwrap_or(at);
            let ends = (self.points[at], self.points[(last + 1) % places]);
            let Some(bend) = self.bends.get(run).copied() else {
                // A run nobody said what curves along: read as the steps it was
                // sampled into, which is what this file exists to improve on,
                // but is never worse than nothing.
                total += (at..=last)
                    .map(|step| straight(self.points[step], self.points[(step + 1) % places]))
                    .sum::<f64>();
                at = last + 1;
                continue;
            };
            total += curved(bend, ends.0, ends.1, self.turned(bend, at, last));
            at = last + 1;
        }
        total
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
