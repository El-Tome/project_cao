//! The dimensions a freshly-drawn arc earns on its own, before the user types
//! anything — the sibling of [`crate::shape_dimensions`] for the one shape
//! whose two typed legs arrive in different frames.

use crate::arc::ArcId;
use crate::arc_placing::ArcMode;
use crate::constraints::DimensionTarget;
use crate::shape_dimensions::settled;
use crate::sketch::Sketch;

/// Places on a freshly-drawn arc whatever the user typed: for `ByCenter`, its
/// radius and the angle it sweeps; for `ByEnds`, the distance it was given
/// between its two ends and the radius it was bent to.
///
/// The two legs are read apart, as plain flags rather than a shared
/// `LockedInput`, because they arrive in different frames: the field that
/// held the first is cleared and reused for the second before the arc is
/// settled enough to be dimensioned.
pub fn arc_dimensions(
    sketch: &Sketch,
    arc: ArcId,
    mode: ArcMode,
    first_typed: bool,
    second_typed: bool,
    scale: f64,
) -> Vec<(DimensionTarget, f64)> {
    let drawn = sketch.arc(arc);
    let mut wanted: Vec<(DimensionTarget, f64)> = Vec::new();
    match mode {
        ArcMode::ByCenter => {
            if first_typed && let Some(value) = sketch.arc_radius_value(arc, scale) {
                wanted.push((DimensionTarget::ArcRadius(arc), value));
            }
            if second_typed && let Some(value) = sketch.arc_sweep_value(arc) {
                wanted.push((DimensionTarget::ArcSweep(arc), value));
            }
        }
        ArcMode::ByEnds => {
            if first_typed {
                wanted.push((
                    DimensionTarget::Distance {
                        from: drawn.start,
                        to: drawn.end,
                    },
                    sketch.point(drawn.start).distance(sketch.point(drawn.end)) * scale,
                ));
            }
            if second_typed && let Some(value) = sketch.arc_radius_value(arc, scale) {
                wanted.push((DimensionTarget::ArcRadius(arc), value));
            }
        }
    }
    settled(sketch, wanted, scale)
}

#[cfg(test)]
mod tests;
