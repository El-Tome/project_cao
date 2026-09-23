//! One end of one curved piece, as the face walk reads it: the direction it
//! leaves in, how hard it bends there, and the run of places it draws.
//!
//! A straight piece needs none of this — its direction is the line to its far
//! end and it bends not at all — which is why the graph keeps the curved ones
//! apart.

use glam::DVec2;

use crate::arcing::{ArcDraft, places_along};
use crate::ellipsing::{EllipseDraft, FULL_ELLIPSE_STEPS};

/// What a curved half-edge bends along: the centre a piece of a circle turns
/// about, or the ellipse a run of one follows.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Bend {
    Round(DVec2),
    Oval(EllipseDraft),
}

/// One end of one curved piece, as a graph half-edge: where it leaves from,
/// the tangent it leaves along — not the straight line to its far end, which
/// is what tells the region walk apart from a plain segment's — and which way
/// round the curve it walks.
pub(crate) struct CurvedHalfEdge {
    pub(crate) bend: Bend,
    /// Whether this half leaves the piece's own start, curving the way it was
    /// drawn, or leaves its end and so walks the same curve backwards.
    pub(crate) forward: bool,
}

impl CurvedHalfEdge {
    /// The direction it leaves `from` in: along the curve at that place,
    /// turned the way this half actually walks it.
    pub(crate) fn departure(&self, from: DVec2) -> DVec2 {
        let along = match &self.bend {
            Bend::Round(centre) => (from - *centre).perp(),
            Bend::Oval(drawn) => {
                let turn = drawn.turn_on(from);
                -drawn.first * turn.sin() + drawn.second_axis() * turn.cos()
            }
        };
        match self.forward {
            true => along,
            false => -along,
        }
    }

    /// How hard it bends as it leaves, and which way: positive turning left,
    /// nought for a straight run.
    ///
    /// Two curves that touch leave that place in the very same direction, and
    /// the walk turns at a vertex by the order of the directions round it. It
    /// is how hard each one bends that puts them in order there: of two edges
    /// leaving the same way, the one turning left the harder lies further
    /// round.
    pub(crate) fn bending(&self, from: DVec2) -> f64 {
        let turning = match &self.bend {
            Bend::Round(centre) => 1.0 / from.distance(*centre).max(1e-12),
            Bend::Oval(drawn) => {
                let turn = drawn.turn_on(from);
                let across = drawn.second_axis();
                let along = -drawn.first * turn.sin() + across * turn.cos();
                let bends = -drawn.first * turn.cos() - across * turn.sin();
                let speed = along.length().max(1e-12);
                along.perp_dot(bends) / (speed * speed * speed)
            }
        };
        match self.forward {
            true => turning,
            false => -turning,
        }
    }

    /// The curve this half contributes to an outline: sampled from its own
    /// `from` up to, but not including, `to` — the same convention a segment's
    /// single point already follows, so the next half-edge, or the walk
    /// closing, supplies the rest.
    pub(crate) fn points_along(&self, from: DVec2, to: DVec2) -> Vec<DVec2> {
        let (start, end) = match self.forward {
            true => (from, to),
            false => (to, from),
        };
        let mut sampled = match &self.bend {
            Bend::Round(centre) => places_along(ArcDraft {
                centre: *centre,
                start,
                end,
            }),
            Bend::Oval(drawn) => places_round(drawn, start, end),
        };
        if !self.forward {
            sampled.reverse();
        }
        sampled.pop();
        sampled
    }
}

/// A run of an ellipse as a run of places, ends included, counter-clockwise
/// from one to the other. As many steps as that share of the whole curve is
/// worth, so a short run is not drawn as one straight step.
fn places_round(drawn: &EllipseDraft, start: DVec2, end: DVec2) -> Vec<DVec2> {
    let from = drawn.turn_on(start);
    let sweep = match (drawn.turn_on(end) - from).rem_euclid(std::f64::consts::TAU) {
        sweep if sweep <= 0.0 => std::f64::consts::TAU,
        sweep => sweep,
    };
    let steps =
        ((sweep / std::f64::consts::TAU * FULL_ELLIPSE_STEPS as f64).ceil() as usize).max(2);
    drawn.places_along(from, sweep, steps)
}
