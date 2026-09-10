use cao_sketch::{DimensionTarget, SketchAxis};

use crate::wording::constraints;

/// Shown when a value would add nothing to a shape that is already settled.
pub const REDUNDANT_WARNING: &str = "Cette cote n'apporte rien : ce qu'elle mesure est déjà tenu. Elle sera posée en simple lecture.";

/// The only place a dimension is turned into a name.
///
/// The value is millimetres for a length or a radius, degrees for an angle.
pub fn label(target: &DimensionTarget, value: f64) -> String {
    let value = short(value);
    match target {
        DimensionTarget::Angle { .. } => format!("Angle {value}°"),
        DimensionTarget::AxisAngle { axis, .. } => {
            format!("Angle {value}° / {}", constraints::axis(*axis))
        }
        DimensionTarget::Radius(_) => format!("Rayon {value} mm"),
        DimensionTarget::Diameter(_) => format!("Diamètre {value} mm"),
        DimensionTarget::Projected { axis, .. } => match axis {
            SketchAxis::U => format!("Largeur {value} mm"),
            SketchAxis::V => format!("Hauteur {value} mm"),
        },
        DimensionTarget::PointToSegment { .. }
        | DimensionTarget::Length(_)
        | DimensionTarget::Distance { .. } => format!("Cote {value} mm"),
    }
}

/// What the dimension is taken across, as the history says it once it knows
/// which sketch the drawing belongs to.
pub fn spans(target: &DimensionTarget) -> String {
    match target {
        DimensionTarget::Distance { from, to } => {
            format!("points {} et {}", from.0, to.0)
        }
        DimensionTarget::Length(segment) => format!("trait {}", segment.0),
        DimensionTarget::Angle { first, second } => {
            format!("traits {} et {}", first.0, second.0)
        }
        DimensionTarget::AxisAngle { segment, axis } => {
            format!("trait {} / {}", segment.0, constraints::axis(*axis))
        }
        DimensionTarget::PointToSegment { point, segment } => {
            format!("point {} au trait {}", point.0, segment.0)
        }
        DimensionTarget::Projected { from, to, axis } => {
            format!(
                "points {} et {} sur l'{}",
                from.0,
                to.0,
                constraints::axis(*axis)
            )
        }
        DimensionTarget::Radius(circle) | DimensionTarget::Diameter(circle) => {
            format!("cercle {}", circle.0)
        }
    }
}

/// A value as it reads in the history: a dimension taken from the drawing
/// itself is a full float, and "Cote 60.878967 mm" is unreadable.
fn short(value: f64) -> String {
    let text = format!("{value:.2}");
    match text.contains('.') {
        true => text.trim_end_matches('0').trim_end_matches('.').to_string(),
        false => text,
    }
}

#[cfg(test)]
mod tests {
    use cao_sketch::{CircleId, PointId, SegmentId};

    use super::*;

    const SEGMENT: SegmentId = SegmentId(0);
    const CIRCLE: CircleId = CircleId(3);

    fn along(axis: SketchAxis) -> DimensionTarget {
        DimensionTarget::Projected {
            from: PointId(1),
            to: PointId(2),
            axis,
        }
    }

    #[test]
    fn a_dimension_is_named_after_what_it_measures() {
        let named = [
            (
                DimensionTarget::Angle {
                    first: SegmentId(0),
                    second: SegmentId(1),
                },
                "Angle 60°",
            ),
            (
                DimensionTarget::AxisAngle {
                    segment: SEGMENT,
                    axis: SketchAxis::U,
                },
                "Angle 60° / axe horizontal",
            ),
            (along(SketchAxis::U), "Largeur 60 mm"),
            (along(SketchAxis::V), "Hauteur 60 mm"),
            (DimensionTarget::Radius(CIRCLE), "Rayon 60 mm"),
            (DimensionTarget::Diameter(CIRCLE), "Diamètre 60 mm"),
            (DimensionTarget::Length(SEGMENT), "Cote 60 mm"),
            (
                DimensionTarget::Distance {
                    from: PointId(1),
                    to: PointId(2),
                },
                "Cote 60 mm",
            ),
            (
                DimensionTarget::PointToSegment {
                    point: PointId(1),
                    segment: SEGMENT,
                },
                "Cote 60 mm",
            ),
        ];

        for (target, reads) in named {
            assert_eq!(label(&target, 60.0), reads, "{target:?} reads {reads:?}");
        }
    }

    #[test]
    fn a_measured_value_is_cut_to_two_decimals_and_keeps_no_trailing_zero() {
        let length = DimensionTarget::Length(SEGMENT);

        assert_eq!(label(&length, 60.878_967), "Cote 60.88 mm");
        assert_eq!(label(&length, 60.0), "Cote 60 mm");
        assert_eq!(label(&length, 60.1), "Cote 60.1 mm");
        assert_eq!(label(&length, 0.001), "Cote 0 mm");
    }

    #[test]
    fn a_dimension_says_what_it_was_taken_across() {
        let spanned = [
            (
                DimensionTarget::Distance {
                    from: PointId(1),
                    to: PointId(2),
                },
                "points 1 et 2",
            ),
            (DimensionTarget::Length(SEGMENT), "trait 0"),
            (
                DimensionTarget::Angle {
                    first: SegmentId(0),
                    second: SegmentId(1),
                },
                "traits 0 et 1",
            ),
            (
                DimensionTarget::AxisAngle {
                    segment: SEGMENT,
                    axis: SketchAxis::U,
                },
                "trait 0 / axe horizontal",
            ),
            (
                DimensionTarget::PointToSegment {
                    point: PointId(1),
                    segment: SEGMENT,
                },
                "point 1 au trait 0",
            ),
            (along(SketchAxis::V), "points 1 et 2 sur l'axe vertical"),
            (DimensionTarget::Radius(CIRCLE), "cercle 3"),
            (DimensionTarget::Diameter(CIRCLE), "cercle 3"),
        ];

        for (target, reads) in spanned {
            assert_eq!(spans(&target), reads, "{target:?} reads {reads:?}");
        }
    }
}
