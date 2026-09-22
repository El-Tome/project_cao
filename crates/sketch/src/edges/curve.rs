//! One curve of the drawing as the edge graph reads it: where it runs, how
//! far along it a place stands, and the runs it is left as once every crossing
//! on it has a vertex.
//!
//! A straight trait and an arc answer the same four questions, and the graph
//! never needs to know which it is holding. Keeping that here is what lets
//! `edges.rs` be about the graph alone.

use glam::DVec2;

use crate::arcing::{ArcDraft, sweep_of};
use crate::crossing::ellipse::{
    where_arc_crosses_ellipse, where_ellipses_cross_along, where_segment_crosses_ellipse,
};
use crate::crossing::{
    round_arc, where_arcs_cross, where_segment_crosses_arc, where_segments_cross,
};
use crate::ellipsing::EllipseDraft;

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
    /// A run of an ellipse, counter-clockwise in the ellipse's own turn from
    /// one vertex to the other.
    ///
    /// Where it starts and how far it goes are worked out once, when the
    /// ellipse is broken: reading them back off the two places costs an
    /// arctangent apiece, and the graph asks for them thousands of times.
    Oval {
        drawn: EllipseDraft,
        starts: f64,
        sweep: f64,
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
            Curve::Straight { from, to }
            | Curve::Bent { from, to, .. }
            | Curve::Oval { from, to, .. } => (*from, *to),
        }
    }

    /// The ellipse this run is cut out of, where it runs from and how far
    /// round it goes — all as fractions of the ellipse's own whole turn.
    fn run(&self) -> Option<(EllipseDraft, f64, f64)> {
        match self {
            Curve::Oval {
                drawn,
                starts,
                sweep,
                ..
            } => Some((*drawn, *starts, *sweep)),
            _ => None,
        }
    }

    fn draft(&self, places: &[DVec2]) -> Option<ArcDraft> {
        match self {
            Curve::Straight { .. } | Curve::Oval { .. } => None,
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
        if let Some((drawn, starts, sweep)) = self.run() {
            // Squashed, the curve is the unit circle, and how far off one the
            // place stands puts a floor under its real distance to the curve.
            // Turning a place away costs an arctangent; hunting the nearest
            // place on the curve costs a hundred times that, and the graph
            // asks this of every point against every run.
            let out = (drawn.squashed(place).length() - 1.0).abs();
            if out * drawn.first.length().min(drawn.second) > off {
                return None;
            }
            if drawn.distance(place) > off {
                return None;
            }
            let round = drawn.turn_on(place) / std::f64::consts::TAU;
            let along = (round - starts).rem_euclid(1.0) / sweep;
            return (along <= 1.0).then_some(along);
        }
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
        if let Some((drawn, starts, sweep)) = self.run() {
            return drawn.at((starts + sweep * fraction) * std::f64::consts::TAU);
        }
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
    if first.run().is_some() || second.run().is_some() {
        return where_a_run_of_an_ellipse_crosses(first, second, places);
    }
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

/// The same where one of the two is a run of an ellipse, whose crossings are
/// found against the whole curve and then brought back to the run's own
/// fraction — nothing at all where the crossing falls outside the run.
fn where_a_run_of_an_ellipse_crosses(
    first: &Curve,
    second: &Curve,
    places: &[DVec2],
) -> Vec<(f64, f64)> {
    let along_the_run = |run: Option<(EllipseDraft, f64, f64)>, whole: f64| {
        let Some((_, starts, sweep)) = run else {
            return Some(whole);
        };
        let along = (whole - starts).rem_euclid(1.0) / sweep;
        (along <= 1.0).then_some(along)
    };
    let (near, far) = (first.run(), second.run());
    let found = match (near, far) {
        (Some((drawn, ..)), None) => match second.draft(places) {
            Some(arc) => where_arc_crosses_ellipse(arc, drawn)
                .into_iter()
                .map(|(round, whole)| (whole, round))
                .collect(),
            None => {
                let (from, to) = second.ends();
                where_segment_crosses_ellipse(places[from], places[to], drawn)
                    .into_iter()
                    .map(|(along, whole)| (whole, along))
                    .collect()
            }
        },
        (None, Some(_)) => {
            return where_a_run_of_an_ellipse_crosses(second, first, places)
                .into_iter()
                .map(|(there, here)| (here, there))
                .collect();
        }
        (Some((near, starts, sweep)), Some((far, ..))) => {
            where_ellipses_cross_along(near, starts, sweep, far)
        }
        (None, None) => Vec::new(),
    };
    found
        .into_iter()
        .filter_map(|(here, there)| Some((along_the_run(near, here)?, along_the_run(far, there)?)))
        .collect()
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
