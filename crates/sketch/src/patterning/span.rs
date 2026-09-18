//! How wide what is held stands, whichever way it is measured.
//!
//! What a rectangular pattern's step opens on, so that the fields appear on a
//! number worth correcting rather than on nothing at all.

use glam::DVec2;

use crate::element::Element;
use crate::sketch::Sketch;

use super::NO_STEP;

impl Sketch {
    /// How wide what is held stands, measured whichever way gives the widest
    /// answer.
    ///
    /// A pattern's step opens on this: the click naming the direction has not
    /// happened yet, so the extent along it cannot be read, and the widest way
    /// round is the one number no direction can make too small.
    ///
    /// Nothing when what is held stands all in one place — a lone point has no
    /// width to go on.
    pub fn widest_span(&self, of: &[Element]) -> Option<f64> {
        let reach: Vec<(DVec2, f64)> = of.iter().flat_map(|held| self.reach_of(*held)).collect();
        let mut widest = 0.0_f64;
        for (here, out) in &reach {
            for (there, back) in &reach {
                widest = widest.max(here.distance(*there) + out + back);
            }
        }
        (widest > NO_STEP).then_some(widest)
    }

    /// Every place one piece of the drawing stands on, and how far it reaches
    /// out from there.
    fn reach_of(&self, held: Element) -> Vec<(DVec2, f64)> {
        match held {
            Element::Circle(id) => match self.circles().get(id.0) {
                Some(round) => vec![(self.point(round.center), round.radius)],
                None => Vec::new(),
            },
            Element::Arc(id) => match self.arcs().get(id.0) {
                Some(curve) => vec![(self.point(curve.center), self.arc_radius(id))],
                None => Vec::new(),
            },
            other => self
                .points_it_leans_on(other)
                .into_iter()
                .map(|point| (self.point(point), 0.0))
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests;
