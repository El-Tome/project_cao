//! One curve of the drawing as the edge graph reads it: where it runs, how
//! far along it a place stands, and the runs it is left as once every crossing
//! on it has a vertex.
//!
//! A straight trait and an arc answer the same four questions, and the graph
//! never needs to know which it is holding. Keeping that here is what lets
//! `edges.rs` be about the graph alone.

use glam::DVec2;

use crate::arcing::{ArcDraft, sweep_of};
use crate::crossing::{
    round_arc, where_arcs_cross, where_segment_crosses_arc, where_segments_cross,
};

use super::off_by;

pub(super) enum Curve {
    Straight {
        from: usize,
        to: usize,
    },
    Bent {
        centre: DVec2,
        from: usize,
        to: usize,
    },
}

impl Curve {
    pub(super) fn is_straight(&self) -> bool {
        matches!(self, Curve::Straight { .. })
    }

    pub(super) fn ends(&self) -> (usize, usize) {
        match self {
            Curve::Straight { from, to } | Curve::Bent { from, to, .. } => (*from, *to),
        }
    }

    fn draft(&self, places: &[DVec2]) -> Option<ArcDraft> {
        match self {
            Curve::Straight { .. } => None,
            Curve::Bent { centre, from, to } => Some(ArcDraft {
                centre: *centre,
                start: places[*from],
                end: places[*to],
            }),
        }
    }

    /// How far along the curve a place stands, when it stands on it at all.
    pub(super) fn fraction_at(&self, places: &[DVec2], place: DVec2) -> Option<f64> {
        let off = off_by(place);
        match self.draft(places) {
            None => {
                let (from, to) = self.ends();
                let (start, along) = (places[from], places[to] - places[from]);
                let span = along.length_squared();
                if span <= 0.0 {
                    return None;
                }
                let fraction = (place - start).dot(along) / span;
                let aside = place.distance(start + along * fraction);
                (aside <= off).then_some(fraction)
            }
            Some(drawn) => {
                let radius = drawn.centre.distance(drawn.start);
                ((place.distance(drawn.centre) - radius).abs() <= off)
                    .then(|| round_arc(drawn, place))
                    .flatten()
            }
        }
    }

    pub(super) fn place_at(&self, places: &[DVec2], fraction: f64) -> DVec2 {
        let (from, to) = self.ends();
        match self.draft(places) {
            None => places[from].lerp(places[to], fraction),
            Some(drawn) => {
                let radius = drawn.centre.distance(drawn.start);
                let angle = (drawn.start - drawn.centre).to_angle() + sweep_of(drawn) * fraction;
                drawn.centre + DVec2::from_angle(angle) * radius
            }
        }
    }
}

/// How far along each of the two a crossing stands, for every crossing they
/// have.
pub(super) fn between(first: &Curve, second: &Curve, places: &[DVec2]) -> Vec<(f64, f64)> {
    let (this, that) = (first.ends(), second.ends());
    match (first.draft(places), second.draft(places)) {
        (None, None) => where_segments_cross(
            places[this.0],
            places[this.1],
            places[that.0],
            places[that.1],
        )
        .into_iter()
        .collect(),
        (None, Some(curve)) => where_segment_crosses_arc(places[this.0], places[this.1], curve),
        (Some(curve), None) => where_segment_crosses_arc(places[that.0], places[that.1], curve)
            .into_iter()
            .map(|(along, round)| (round, along))
            .collect(),
        (Some(near), Some(far)) => where_arcs_cross(near, far),
    }
}

/// The curve cut into the runs between its crossings, each as the two vertices
/// it joins.
pub(super) fn pieces(curve: &Curve, cuts: &[(f64, usize)]) -> Vec<(usize, usize)> {
    let (from, to) = curve.ends();
    let mut sorted = cuts.to_vec();
    sorted.sort_by(|left, right| left.0.total_cmp(&right.0));
    let mut chain = vec![from];
    chain.extend(sorted.iter().map(|(_, vertex)| *vertex));
    chain.push(to);
    chain.dedup();
    chain.windows(2).map(|pair| (pair[0], pair[1])).collect()
}
