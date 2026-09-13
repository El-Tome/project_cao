//! Where an arc's swept-angle dimension is drawn: traced along its own
//! curve, since a fillet can honestly ask for more than a straight line's
//! worth of turn.

use std::f64::consts::{FRAC_1_SQRT_2, FRAC_PI_2};

use glam::DVec2;

use crate::annotation::{AnnotationMetrics, Moved, arrow, text_clearance};
use crate::arc::ArcId;
use crate::sketch::Sketch;

/// Where an arc's radius dimension is drawn: a line from its centre out to
/// its rim, exactly as a circle's already is.
pub(crate) fn radius(
    sketch: &Sketch,
    arc: ArcId,
    by: Moved,
    metrics: AnnotationMetrics,
    shape: &mut Vec<(DVec2, DVec2)>,
) -> Option<(DVec2, DVec2)> {
    let drawn = *sketch.arcs().get(arc.0)?;
    let center = *sketch.points().get(drawn.center.0)?;
    Some(radial(shape, center, sketch.arc_radius(arc), by, metrics))
}

/// A radius: a line from the centre out to the rim, arrow at the far end.
/// Shared by a circle's own radius dimension and an arc's.
pub(crate) fn radial(
    out: &mut Vec<(DVec2, DVec2)>,
    center: DVec2,
    radius: f64,
    by: Moved,
    metrics: AnnotationMetrics,
) -> (DVec2, DVec2) {
    // A radius is always drawn from the centre outwards, so dragging it turns
    // the leader about the circle rather than detaching it.
    let default = DVec2::splat(FRAC_1_SQRT_2);
    let placed = by.placed.unwrap_or(default * radius) + by.nudge;
    let direction = placed.normalize_or(default);
    let rim = center + direction * radius;
    out.push((center, rim));
    arrow(out, rim, -direction, metrics);

    let aside = DVec2::new(-direction.y, direction.x);
    (
        center + direction * radius * 0.55 + aside * text_clearance(aside) * metrics.pixel,
        direction * radius,
    )
}

/// Where an arc's swept-angle dimension is drawn.
pub(crate) fn sweep(
    sketch: &Sketch,
    arc: ArcId,
    by: Moved,
    metrics: AnnotationMetrics,
    shape: &mut Vec<(DVec2, DVec2)>,
) -> Option<(DVec2, DVec2)> {
    let drawn = *sketch.arcs().get(arc.0)?;
    let center = *sketch.points().get(drawn.center.0)?;
    let start = *sketch.points().get(drawn.start.0)?;
    Some(swept(
        shape,
        center,
        start,
        sketch.arc_sweep(arc),
        by,
        metrics,
    ))
}

/// The angle an arc runs through: traced along its own curve, at whatever
/// radius the value has been pushed out to. Unlike the angle between two
/// segments, an arc's sweep can run past a straight line and back, so nothing
/// here folds it to the smaller way round.
pub(crate) fn swept(
    out: &mut Vec<(DVec2, DVec2)>,
    pivot: DVec2,
    start: DVec2,
    sweep: f64,
    by: Moved,
    metrics: AnnotationMetrics,
) -> (DVec2, DVec2) {
    let from = (start - pivot).to_angle();
    let bisector = DVec2::from_angle(from + sweep * 0.5);
    let clearance = 14.0 * metrics.pixel;
    let default = bisector * (metrics.arc_pixels * metrics.pixel + clearance);
    let reach = by.placed.unwrap_or(default) + by.nudge;
    let radius = (reach.length() - clearance).max(6.0 * metrics.pixel);

    const STEPS: usize = 24;
    let mut previous = None;
    for step in 0..=STEPS {
        let angle = from + sweep * step as f64 / STEPS as f64;
        let point = pivot + DVec2::from_angle(angle) * radius;
        if let Some(previous) = previous {
            out.push((previous, point));
        }
        previous = Some(point);
    }

    let tangent = |angle: f64, sign: f64| DVec2::from_angle(angle + FRAC_PI_2) * sign;
    arrow(
        out,
        pivot + DVec2::from_angle(from) * radius,
        tangent(from, 1.0),
        metrics,
    );
    arrow(
        out,
        pivot + DVec2::from_angle(from + sweep) * radius,
        tangent(from + sweep, -1.0),
        metrics,
    );

    (pivot + reach, reach)
}
