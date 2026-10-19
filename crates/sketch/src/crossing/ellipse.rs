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
/// solved. The walk reads the slope between samples as well as their sign, so
/// a crossing pair sharing one step is still seen — as long as the gap keeps
/// to one extremum per two steps, the width of the stretch an extremum is
/// refined over, which conics against conics do.
const STEPS: usize = 256;

/// How many times the stretch holding a meeting is shrunk. The bisection
/// halves it: forty turns take a step of a whole ellipse below a trillionth
/// of it.
const HALVINGS: usize = 40;

/// How far round its turn the ellipse stands at a place on it, as a fraction
/// of a whole turn.
fn fraction_of(oval: EllipseDraft, place: DVec2) -> f64 {
    oval.turn_on(place) / std::f64::consts::TAU
}

/// Where a straight run crosses an ellipse: how far along the run, and how far
/// round the ellipse's own turn.
///
/// Whether a run that comes close grazes it is decided on the real gap: the
/// squash warps distances, so the squashed picture says only where the nearest
/// approach stands, and the drawing's own units say whether it touches.
pub(crate) fn where_segment_crosses_ellipse(
    a1: DVec2,
    a2: DVec2,
    oval: EllipseDraft,
    touch: f64,
) -> Vec<(f64, f64)> {
    let (from, to) = (oval.squashed(a1), oval.squashed(a2));
    let along = to - from;
    let span = along.length_squared();
    if span <= 0.0 {
        return Vec::new();
    }
    let summit = -from.dot(along) / span;
    if oval.distance(a1.lerp(a2, summit)) <= touch {
        return (summit > 0.0 && summit < 1.0)
            .then(|| (summit, turn_at(DVec2::ZERO, from.lerp(to, summit))))
            .into_iter()
            .collect();
    }
    if (from + along * summit).length() >= 1.0 {
        return Vec::new();
    }
    super::along_segment_at_circle(from, to, DVec2::ZERO, 1.0, 0.0)
        .into_iter()
        .map(|along| (along, turn_at(DVec2::ZERO, from.lerp(to, along))))
        .collect()
}

/// Where a whole circle crosses an ellipse: how far round the circle, and how
/// far round the ellipse.
///
/// Nowhere at all for an ellipse whose axes reach alike laid on the circle:
/// that is the circle drawn again, and hunted against itself it reads the
/// rounding as a crossing at nearly every step.
pub(crate) fn where_circle_crosses_ellipse(
    centre: DVec2,
    radius: f64,
    oval: EllipseDraft,
    touch: f64,
) -> Vec<(f64, f64)> {
    if oval.is_the_curve(&EllipseDraft::round(centre, radius)) {
        return Vec::new();
    }
    hunted(oval, |place| place.distance(centre) - radius, touch)
        .into_iter()
        .map(|place| (turn_at(centre, place), fraction_of(oval, place)))
        .collect()
}

/// Where an arc crosses an ellipse: how far round the arc's own sweep, and how
/// far round the ellipse. A crossing that falls on the rest of the arc's
/// circle is not one, and nowhere at all where the ellipse is that circle.
pub(crate) fn where_arc_crosses_ellipse(
    arc: ArcDraft,
    oval: EllipseDraft,
    touch: f64,
) -> Vec<(f64, f64)> {
    let radius = arc.centre.distance(arc.start);
    if oval.is_the_curve(&EllipseDraft::round(arc.centre, radius)) {
        return Vec::new();
    }
    hunted(oval, |place| place.distance(arc.centre) - radius, touch)
        .into_iter()
        .filter_map(|place| Some((round_arc(arc, place)?, fraction_of(oval, place))))
        .collect()
}

/// Where two ellipses cross, each as a fraction of its own turn.
///
/// Nowhere at all for one and the same ellipse, which every place of is on
/// both: two pieces cut out of one curve do not cross each other.
pub(crate) fn where_ellipses_cross(
    near: EllipseDraft,
    far: EllipseDraft,
    touch: f64,
) -> Vec<(f64, f64)> {
    where_ellipses_cross_along(near, 0.0, 1.0, far, touch)
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
    touch: f64,
) -> Vec<(f64, f64)> {
    if near.is_the_curve(&far) {
        return Vec::new();
    }
    let turn = std::f64::consts::TAU;
    hunted_along(
        near,
        from * turn,
        sweep * turn,
        |place| {
            // Brought back to the drawing's own units, roughly: how far out the
            // place stands from the other curve's centre is what its measure is
            // worth there.
            far.off_by(place) * (place - far.centre).length()
        },
        touch,
    )
    .into_iter()
    .map(|place| (fraction_of(near, place), fraction_of(far, place)))
    .collect()
}

/// The places round the whole ellipse where the other curve is met.
fn hunted(oval: EllipseDraft, off_by: impl Fn(DVec2) -> f64, touch: f64) -> Vec<DVec2> {
    hunted_along(oval, 0.0, std::f64::consts::TAU, off_by, touch)
}

/// The same over one stretch of it, walked in steps of the size the whole
/// curve would be walked in — never fewer than a handful, so a short run is
/// still looked at properly.
///
/// `off_by` says how far off the other curve a place stands, in the drawing's
/// own units and signed: which side of it the place is on.
///
/// Everything near a tangency lives where the reading stops changing, so the
/// walk looks for the places where its slope turns round, and closes in on
/// each as a root of the slope — which crosses nought straight where the
/// reading itself only flattens. The reading there settles the meeting: within
/// `touch` of the other curve it is one touch, one vertex — a circle sitting
/// inside an ellipse against the ends of its short axis touches it twice, and
/// without those vertices the ring between the two curves is one pinched face
/// rather than the two the drawing shows. Beyond `touch` and on the far side
/// of the samples round it, it is a crossing pair no sample fell between, and
/// each of the two is closed in on by halving. Everything else is the plain
/// sign change a crossing leaves, halved as ever.
fn hunted_along(
    oval: EllipseDraft,
    from: f64,
    sweep: f64,
    off_by: impl Fn(DVec2) -> f64,
    touch: f64,
) -> Vec<DVec2> {
    let share = (sweep / std::f64::consts::TAU).clamp(0.0, 1.0);
    let steps = ((STEPS as f64 * share).ceil() as usize).max(8);
    let stride = sweep / steps as f64;
    let turn = |step: usize| from + stride * step as f64;
    let off: Vec<f64> = (0..=steps)
        .map(|step| off_by(oval.at(turn(step))))
        .collect();
    let slope = |step: usize| off[step + 1] - off[step];
    // A closed loop has no seam: the stretch past its last sample is the
    // stretch past its first, and a tangency standing there is still one.
    let closed = sweep >= std::f64::consts::TAU;
    let mut taken = vec![false; steps];

    let mut found = Vec::new();
    let turning = (1..steps)
        .map(|step| (step, slope(step - 1), slope(step)))
        .chain(closed.then(|| (steps, slope(steps - 1), slope(0))));
    for (step, before, after) in turning {
        if (before < 0.0) == (after < 0.0) {
            continue;
        }
        let (low, high) = (turn(step - 1), turn(step + 1));
        let still = stillest_between(oval, &off_by, low, high, before);
        let gap = off_by(oval.at(still));
        if gap.abs() <= touch {
            found.push(oval.at(still));
            taken[step - 1] = true;
            taken[step % steps] = true;
            continue;
        }
        let outside = off[step - 1];
        let middle = off[step];
        let ahead = off[if step == steps { 1 } else { step + 1 }];
        let one_side = (middle < 0.0) == (outside < 0.0) && (ahead < 0.0) == (outside < 0.0);
        if one_side && (gap < 0.0) != (outside < 0.0) {
            found.push(halved(oval, &off_by, low, still, outside));
            found.push(halved(oval, &off_by, still, high, gap));
            taken[step - 1] = true;
            taken[step % steps] = true;
        }
    }
    for step in 1..=steps {
        let (behind, ahead) = (off[step - 1], off[step]);
        if (behind < 0.0) != (ahead < 0.0) && !taken[step - 1] {
            found.push(halved(oval, &off_by, turn(step - 1), turn(step), behind));
        }
    }
    found
}

/// The turn over a stretch of the curve where the reading stops changing,
/// closed in on by halving on the sign of its slope, read over a whisker of
/// what is left. `entering` is how the reading runs into the stretch.
fn stillest_between(
    oval: EllipseDraft,
    off_by: &impl Fn(DVec2) -> f64,
    from: f64,
    to: f64,
    entering: f64,
) -> f64 {
    let (mut low, mut high) = (from, to);
    for _ in 0..HALVINGS {
        let middle = (low + high) * 0.5;
        let whisker = (high - low) * 0.125;
        let drift = off_by(oval.at(middle + whisker)) - off_by(oval.at(middle - whisker));
        if (drift < 0.0) == (entering < 0.0) {
            low = middle;
        } else {
            high = middle;
        }
    }
    (low + high) * 0.5
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
