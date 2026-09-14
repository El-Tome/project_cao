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
mod tests {
    use super::*;
    use crate::plane::WorkPlane;
    use glam::DVec2;

    fn a_quarter_arc() -> (Sketch, ArcId) {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let center = sketch.add_point(DVec2::ZERO);
        let start = sketch.add_point(DVec2::new(40.0, 0.0));
        let end = sketch.add_point(DVec2::new(0.0, 40.0));
        let arc = sketch.add_arc(center, start, end);
        (sketch, arc)
    }

    #[test]
    fn an_arc_bent_by_its_centre_and_typed_on_both_legs_earns_a_radius_and_a_sweep() {
        let (sketch, arc) = a_quarter_arc();

        let wanted = arc_dimensions(&sketch, arc, ArcMode::ByCenter, true, true, 1.0);

        assert!(
            wanted
                .iter()
                .any(|(target, value)| *target == DimensionTarget::ArcRadius(arc)
                    && (*value - 40.0).abs() < 1e-9)
        );
        assert!(
            wanted
                .iter()
                .any(|(target, value)| *target == DimensionTarget::ArcSweep(arc)
                    && (*value - 90.0).abs() < 1e-9)
        );
    }

    #[test]
    fn an_arc_bent_by_its_centre_with_nothing_typed_earns_no_dimension() {
        let (sketch, arc) = a_quarter_arc();

        let wanted = arc_dimensions(&sketch, arc, ArcMode::ByCenter, false, false, 1.0);

        assert!(
            wanted.is_empty(),
            "an arc dragged with no value typed stays undimensioned, same as a dragged line"
        );
    }

    #[test]
    fn an_arc_bent_by_its_ends_typed_on_both_legs_earns_a_distance_and_a_radius() {
        let (sketch, arc) = a_quarter_arc();
        let drawn = sketch.arc(arc);

        let wanted = arc_dimensions(&sketch, arc, ArcMode::ByEnds, true, true, 1.0);

        assert!(wanted.iter().any(|(target, value)| {
            *target
                == DimensionTarget::Distance {
                    from: drawn.start,
                    to: drawn.end,
                }
                .normalised()
                && (*value - 40.0 * std::f64::consts::SQRT_2).abs() < 1e-9
        }));
        assert!(
            wanted
                .iter()
                .any(|(target, value)| *target == DimensionTarget::ArcRadius(arc)
                    && (*value - 40.0).abs() < 1e-9)
        );
    }

    #[test]
    fn an_arc_bent_by_its_ends_typed_only_on_the_first_leg_earns_only_the_distance() {
        let (sketch, arc) = a_quarter_arc();

        let wanted = arc_dimensions(&sketch, arc, ArcMode::ByEnds, true, false, 1.0);

        assert_eq!(wanted.len(), 1);
        assert!(!matches!(wanted[0].0, DimensionTarget::ArcRadius(_)));
    }
}
