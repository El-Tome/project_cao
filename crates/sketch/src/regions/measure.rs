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
    /// What a measure reads of this area.
    ///
    /// Here rather than beside `read_inside`, so the canvas — which has the
    /// area in hand already, having just found it to light it — reads it the
    /// very same way rather than by a second copy of these two lines.
    pub fn read(&self) -> crate::reading::Reading {
        crate::reading::Reading::Surface {
            area: self.area(),
            perimeter: self.perimeter(),
        }
    }

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
    pub(crate) fn area(&self) -> f64 {
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
    pub(crate) fn perimeter(&self) -> f64 {
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
    /// turned, signed — read off the sampled places, as `spans` says why.
    fn walked(
        &self,
        straight: impl Fn(DVec2, DVec2) -> f64,
        curved: impl Fn(Bend, DVec2, DVec2, f64) -> f64,
    ) -> f64 {
        self.spans()
            .into_iter()
            .map(|span| match span.along {
                None => straight(span.from, span.to),
                Some((bend, turned)) => curved(bend, span.from, span.to, turned),
            })
            .sum()
    }
}

#[cfg(test)]
mod tests;
