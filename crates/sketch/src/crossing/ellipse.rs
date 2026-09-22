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

impl EllipseDraft {
    /// A place in the ellipse's own measure, where the curve itself is the
    /// circle of radius one about the origin.
    fn squashed(&self, place: DVec2) -> DVec2 {
        let out = place - self.centre;
        let along = self.first.length();
        DVec2::new(
            out.dot(self.first) / (along * along),
            out.dot(self.second_axis()) / (self.second * self.second),
        )
    }

    /// How far off the curve a place stands, in that same measure: nought on
    /// it, negative inside, positive outside.
    fn off_by(&self, place: DVec2) -> f64 {
        self.squashed(place).length() - 1.0
    }

    /// How far round its turn the ellipse stands at a place on it, as a
    /// fraction of a whole turn.
    fn fraction_at(&self, place: DVec2) -> f64 {
        let squashed = self.squashed(place);
        squashed.to_angle().rem_euclid(std::f64::consts::TAU) / std::f64::consts::TAU
    }
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
        .map(|place| (turn_at(centre, place), oval.fraction_at(place)))
        .collect()
}

/// Where an arc crosses an ellipse: how far round the arc's own sweep, and how
/// far round the ellipse. A crossing that falls on the rest of the arc's
/// circle is not one.
pub(crate) fn where_arc_crosses_ellipse(arc: ArcDraft, oval: EllipseDraft) -> Vec<(f64, f64)> {
    let radius = arc.centre.distance(arc.start);
    hunted(oval, |place| place.distance(arc.centre) - radius)
        .into_iter()
        .filter_map(|place| Some((round_arc(arc, place)?, oval.fraction_at(place))))
        .collect()
}

/// Where two ellipses cross, each as a fraction of its own turn.
///
/// Nowhere at all for one and the same ellipse, which every place of is on
/// both: two pieces cut out of one curve do not cross each other.
pub(crate) fn where_ellipses_cross(near: EllipseDraft, far: EllipseDraft) -> Vec<(f64, f64)> {
    if near == far {
        return Vec::new();
    }
    hunted(near, |place| far.off_by(place))
        .into_iter()
        .map(|place| (near.fraction_at(place), far.fraction_at(place)))
        .collect()
}

/// The places round the ellipse where `off_by` changes sign, hunted step by
/// step and then halved down.
fn hunted(oval: EllipseDraft, off_by: impl Fn(DVec2) -> f64) -> Vec<DVec2> {
    let turn = |step: usize| std::f64::consts::TAU * step as f64 / STEPS as f64;
    let mut found = Vec::new();
    let mut behind = off_by(oval.at(0.0));
    for step in 1..=STEPS {
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
