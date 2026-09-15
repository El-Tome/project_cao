//! The curve an arc is, once its three places are known: how far round it
//! runs, the box it fits in, and the run of straight steps it is drawn and
//! measured as.

use glam::DVec2;

/// An arc as the three places it stands on: the one a tool is part-way through
/// drawing, and the one already in the sketch once its points are looked up.
/// The curve is made of nothing else.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ArcDraft {
    pub centre: DVec2,
    pub start: DVec2,
    pub end: DVec2,
}

/// How far round a curve runs, in radians, always between zero and a full
/// turn: the way round is carried by the order of the two ends, so the answer
/// never needs a sign to say it.
pub fn sweep_of(drawn: ArcDraft) -> f64 {
    let from = (drawn.start - drawn.centre).to_angle();
    let to = (drawn.end - drawn.centre).to_angle();
    (to - from).rem_euclid(std::f64::consts::TAU)
}

/// The smallest box the curve fits in, as its two opposite corners.
///
/// The two ends are not enough: a curve that reaches its furthest north, south,
/// east or west between them leaves the box they alone would draw.
pub(crate) fn bounds_of(drawn: ArcDraft) -> (DVec2, DVec2) {
    let radius = drawn.centre.distance(drawn.start);
    let from = (drawn.start - drawn.centre).to_angle();
    let sweep = sweep_of(drawn);
    let mut lowest = drawn.start.min(drawn.end);
    let mut highest = drawn.start.max(drawn.end);
    for quarter in 0..4 {
        let angle = std::f64::consts::FRAC_PI_2 * f64::from(quarter);
        if (angle - from).rem_euclid(std::f64::consts::TAU) > sweep {
            continue;
        }
        let furthest = drawn.centre + DVec2::from_angle(angle) * radius;
        lowest = lowest.min(furthest);
        highest = highest.max(furthest);
    }
    (lowest, highest)
}

/// How finely a whole turn would be cut up. An arc takes its share of it.
pub(crate) const FULL_CIRCLE_STEPS: usize = 48;

/// Into how many straight steps the curve is cut.
///
/// Read from the sweep rather than fixed, so that a small fillet does not
/// become a visible polygon and a long arc does not cost what a whole circle
/// costs.
pub fn steps_along(drawn: ArcDraft) -> usize {
    let turns = sweep_of(drawn) / std::f64::consts::TAU;
    ((turns * FULL_CIRCLE_STEPS as f64).ceil() as usize).max(2)
}

/// The curve as a run of places, ends included.
pub fn places_along(drawn: ArcDraft) -> Vec<DVec2> {
    let radius = drawn.centre.distance(drawn.start);
    let sweep = sweep_of(drawn);
    let from = (drawn.start - drawn.centre).to_angle();
    let steps = steps_along(drawn);
    (0..=steps)
        .map(|step| {
            let angle = from + sweep * step as f64 / steps as f64;
            drawn.centre + DVec2::from_angle(angle) * radius
        })
        .collect()
}
