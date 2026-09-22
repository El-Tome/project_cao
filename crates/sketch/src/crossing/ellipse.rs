//! Where an ellipse crosses the rest of the drawing.
//!
//! Against a straight run the answer is exact: squashing the plane until the
//! ellipse comes out round leaves every fraction along a straight run where it
//! was, so the crossing is the circle's, read back.
//!
//! Against another curve there is no squash that leaves both of them round.
//! What it comes down to is a quartic, and rather than write one nobody can
//! read, the crossings are hunted along the ellipse: the other curve says how
//! far off each place stands, and a place where that changes sign has a
//! crossing between it and the last one.

use glam::DVec2;

use super::{round_arc, turn_at};
use crate::arcing::ArcDraft;
use crate::ellipsing::EllipseDraft;

/// How finely the ellipse is walked when a crossing is hunted rather than
/// solved. Two crossings closer together than one step of this are seen as
/// none, which is the same blind spot a tangency already has.
const STEPS: usize = 256;

/// How many times the step holding a crossing is halved. A double at each
/// turn: thirty of them take a step of a whole ellipse down to a billionth of
/// it, well under what the drawing counts as one place.
const HALVINGS: usize = 40;

/// How far round its turn the ellipse stands at a place on it, as a fraction
/// of a whole turn.
fn fraction_of(oval: EllipseDraft, place: DVec2) -> f64 {
    oval.turn_on(place) / std::f64::consts::TAU
}

/// Where a straight run crosses an ellipse: how far along the run, and how far
/// round the ellipse's own turn.
pub(crate) fn where_segment_crosses_ellipse(
    a1: DVec2,
    a2: DVec2,
    oval: EllipseDraft,
) -> Vec<(f64, f64)> {
    let (from, to) = (oval.squashed(a1), oval.squashed(a2));
    super::along_segment_at_circle(from, to, DVec2::ZERO, 1.0)
        .into_iter()
        .map(|along| (along, turn_at(DVec2::ZERO, from.lerp(to, along))))
        .collect()
}

/// Where a whole circle crosses an ellipse: how far round the circle, and how
/// far round the ellipse.
pub(crate) fn where_circle_crosses_ellipse(
    centre: DVec2,
    radius: f64,
    oval: EllipseDraft,
) -> Vec<(f64, f64)> {
    hunted(oval, |place| place.distance(centre) - radius)
        .into_iter()
        .map(|place| (turn_at(centre, place), fraction_of(oval, place)))
        .collect()
}

/// Where an arc crosses an ellipse: how far round the arc's own sweep, and how
/// far round the ellipse. A crossing that falls on the rest of the arc's
/// circle is not one.
pub(crate) fn where_arc_crosses_ellipse(arc: ArcDraft, oval: EllipseDraft) -> Vec<(f64, f64)> {
    let radius = arc.centre.distance(arc.start);
    hunted(oval, |place| place.distance(arc.centre) - radius)
        .into_iter()
        .filter_map(|place| Some((round_arc(arc, place)?, fraction_of(oval, place))))
        .collect()
}

/// Where two ellipses cross, each as a fraction of its own turn.
///
/// Nowhere at all for one and the same ellipse, which every place of is on
/// both: two pieces cut out of one curve do not cross each other.
pub(crate) fn where_ellipses_cross(near: EllipseDraft, far: EllipseDraft) -> Vec<(f64, f64)> {
    where_ellipses_cross_along(near, 0.0, 1.0, far)
}

/// The same over one run of the near ellipse, given as where it starts and how
/// far it goes, both as fractions of a whole turn.
///
/// Walked as finely as that share of the curve is worth rather than the whole
/// of it: the graph asks this of every run of every ellipse against every run
/// of every other, and hunting the whole curve each time does the same work
/// over and over.
pub(crate) fn where_ellipses_cross_along(
    near: EllipseDraft,
    from: f64,
    sweep: f64,
    far: EllipseDraft,
) -> Vec<(f64, f64)> {
    if near.is_the_curve(&far) {
        return Vec::new();
    }
    let turn = std::f64::consts::TAU;
    hunted_along(near, from * turn, sweep * turn, |place| far.off_by(place))
        .into_iter()
        .map(|place| (fraction_of(near, place), fraction_of(far, place)))
        .collect()
}

/// The places round the whole ellipse where `off_by` changes sign.
fn hunted(oval: EllipseDraft, off_by: impl Fn(DVec2) -> f64) -> Vec<DVec2> {
    hunted_along(oval, 0.0, std::f64::consts::TAU, off_by)
}

/// The same over one stretch of it, walked in steps of the size the whole
/// curve would be walked in — never fewer than a handful, so a short run is
/// still looked at properly.
fn hunted_along(
    oval: EllipseDraft,
    from: f64,
    sweep: f64,
    off_by: impl Fn(DVec2) -> f64,
) -> Vec<DVec2> {
    let share = (sweep / std::f64::consts::TAU).clamp(0.0, 1.0);
    let steps = ((STEPS as f64 * share).ceil() as usize).max(8);
    let turn = |step: usize| from + sweep * step as f64 / steps as f64;
    let mut found = Vec::new();
    let mut behind = off_by(oval.at(from));
    for step in 1..=steps {
        let ahead = off_by(oval.at(turn(step)));
        if (behind < 0.0) != (ahead < 0.0) {
            found.push(halved(oval, &off_by, turn(step - 1), turn(step), behind));
        }
        behind = ahead;
    }
    found
}

/// The place between two turns where the sign changes, closed in on by halving
/// the stretch between them.
fn halved(
    oval: EllipseDraft,
    off_by: &impl Fn(DVec2) -> f64,
    from: f64,
    to: f64,
    inside: f64,
) -> DVec2 {
    let (mut low, mut high) = (from, to);
    for _ in 0..HALVINGS {
        let middle = (low + high) * 0.5;
        if (off_by(oval.at(middle)) < 0.0) == (inside < 0.0) {
            low = middle;
        } else {
            high = middle;
        }
    }
    oval.at((low + high) * 0.5)
}

#[cfg(test)]
mod tests;
