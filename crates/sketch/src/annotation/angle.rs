//! The arc an angle is drawn as: how wide it stands off the corner it
//! measures, the arrowheads tangent to it, and the leader that reaches the
//! value when it has been dragged outside the two arms.

use std::f64::consts::{FRAC_PI_2, PI, TAU};

use glam::DVec2;

use super::{AnnotationMetrics, Moved, arrow};

/// An angle: an arc between the two arms, with an arrowhead at each end.
pub(super) fn angular(
    out: &mut Vec<(DVec2, DVec2)>,
    pivot: DVec2,
    first: DVec2,
    second: DVec2,
    by: Moved,
    metrics: AnnotationMetrics,
) -> (DVec2, DVec2) {
    let start = (first - pivot).to_angle();
    let mut sweep = (second - pivot).to_angle() - start;
    // Always draw the smaller way round: that is the angle being talked
    // about.
    while sweep > PI {
        sweep -= TAU;
    }
    while sweep < -PI {
        sweep += TAU;
    }

    // An arc stays hinged on the corner it measures: what is recorded is
    // where the value sits relative to that corner, and the arc is drawn
    // just inside it. Splitting the movement into radius and slide instead
    // let a value dragged sideways shrink its own arc to nothing.
    let bisector = DVec2::from_angle(start + sweep * 0.5);
    let clearance = 14.0 * metrics.pixel;
    let default = bisector * (metrics.arc_pixels * metrics.pixel + clearance);
    let reach = by.placed.unwrap_or(default) + by.nudge;
    let radius = (reach.length() - clearance).max(6.0 * metrics.pixel);

    const STEPS: usize = 24;
    let mut previous = None;
    for step in 0..=STEPS {
        let angle = start + sweep * step as f64 / STEPS as f64;
        let point = pivot + DVec2::from_angle(angle) * radius;
        if let Some(previous) = previous {
            out.push((previous, point));
        }
        previous = Some(point);
    }

    // Arrowheads point along the arc, so they lie tangent to it.
    let tangent = |angle: f64, sign: f64| DVec2::from_angle(angle + FRAC_PI_2) * sign;
    let at_start = pivot + DVec2::from_angle(start) * radius;
    let at_end = pivot + DVec2::from_angle(start + sweep) * radius;
    arrow(out, at_start, tangent(start, sweep.signum()), metrics);
    arrow(
        out,
        at_end,
        tangent(start + sweep, -sweep.signum()),
        metrics,
    );

    // Dragged outside the two arms, the value has nothing joining it to the
    // arc it belongs to; a leader says where it comes from.
    let text_at = pivot + reach;
    let towards = reach.to_angle();
    let mut turn = towards - start;
    while turn > PI {
        turn -= TAU;
    }
    while turn < -PI {
        turn += TAU;
    }
    let fraction = turn / sweep;
    if !fraction.is_finite() || !(0.0..=1.0).contains(&fraction) {
        let nearer = if turn.abs() < (turn - sweep).abs() {
            at_start
        } else {
            at_end
        };
        out.push((nearer, text_at - reach.normalize_or_zero() * clearance));
    }

    (text_at, reach)
}
