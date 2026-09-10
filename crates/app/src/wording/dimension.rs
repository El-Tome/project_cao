use cao_sketch::{DimensionTarget, SketchAxis};

use crate::lang::Catalogue;
use crate::wording::constraints;

/// Shown when a value would add nothing to a shape that is already settled.
pub fn redundant_warning(lang: &Catalogue) -> String {
    lang.t("dimension.redundant")
}

/// The only place a dimension is turned into a name.
///
/// The value is millimetres for a length or a radius, degrees for an angle.
/// It is rounded here rather than in the language file, so that a translation
/// never has to carry a format specifier.
pub fn label(lang: &Catalogue, target: &DimensionTarget, value: f64) -> String {
    let value = short(value);
    let measured = &[("value", value.as_str())];
    match target {
        DimensionTarget::Angle { .. } => lang.t_with("dimension.label.angle", measured),
        DimensionTarget::AxisAngle { axis, .. } => lang.t_with(
            "dimension.label.axis_angle",
            &[
                ("value", value.as_str()),
                ("axis", &constraints::axis(lang, *axis)),
            ],
        ),
        DimensionTarget::Radius(_) => lang.t_with("dimension.label.radius", measured),
        DimensionTarget::Diameter(_) => lang.t_with("dimension.label.diameter", measured),
        DimensionTarget::Projected { axis, .. } => match axis {
            SketchAxis::U => lang.t_with("dimension.label.width", measured),
            SketchAxis::V => lang.t_with("dimension.label.height", measured),
        },
        DimensionTarget::PointToSegment { .. }
        | DimensionTarget::Length(_)
        | DimensionTarget::Distance { .. } => lang.t_with("dimension.label.length", measured),
    }
}

/// What the dimension is taken across, as the history says it once it knows
/// which sketch the drawing belongs to.
pub fn spans(lang: &Catalogue, target: &DimensionTarget) -> String {
    match target {
        DimensionTarget::Distance { from, to } => lang.t_with(
            "dimension.spans.points",
            &[
                ("first", &from.0.to_string()),
                ("second", &to.0.to_string()),
            ],
        ),
        DimensionTarget::Length(segment) => lang.t_with(
            "dimension.spans.segment",
            &[("segment", &segment.0.to_string())],
        ),
        DimensionTarget::Angle { first, second } => lang.t_with(
            "dimension.spans.segments",
            &[
                ("first", &first.0.to_string()),
                ("second", &second.0.to_string()),
            ],
        ),
        DimensionTarget::AxisAngle { segment, axis } => lang.t_with(
            "dimension.spans.segment_to_axis",
            &[
                ("segment", &segment.0.to_string()),
                ("axis", &constraints::axis(lang, *axis)),
            ],
        ),
        DimensionTarget::PointToSegment { point, segment } => lang.t_with(
            "dimension.spans.point_to_segment",
            &[
                ("point", &point.0.to_string()),
                ("segment", &segment.0.to_string()),
            ],
        ),
        DimensionTarget::Projected { from, to, axis } => lang.t_with(
            "dimension.spans.points_along_axis",
            &[
                ("first", &from.0.to_string()),
                ("second", &to.0.to_string()),
                ("axis", &constraints::axis(lang, *axis)),
            ],
        ),
        DimensionTarget::Radius(circle) | DimensionTarget::Diameter(circle) => lang.t_with(
            "dimension.spans.circle",
            &[("circle", &circle.0.to_string())],
        ),
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

    fn named(target: &DimensionTarget, value: f64) -> String {
        label(&Catalogue::french(), target, value)
    }

    fn across(target: &DimensionTarget) -> String {
        spans(&Catalogue::french(), target)
    }

    #[test]
    fn a_dimension_is_named_after_what_it_measures() {
        let measured = [
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

        for (target, reads) in measured {
            assert_eq!(named(&target, 60.0), reads, "{target:?} reads {reads:?}");
        }
    }

    #[test]
    fn a_measured_value_is_cut_to_two_decimals_and_keeps_no_trailing_zero() {
        let length = DimensionTarget::Length(SEGMENT);

        assert_eq!(named(&length, 60.878_967), "Cote 60.88 mm");
        assert_eq!(named(&length, 60.0), "Cote 60 mm");
        assert_eq!(named(&length, 60.1), "Cote 60.1 mm");
        assert_eq!(named(&length, 0.001), "Cote 0 mm");
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
            assert_eq!(across(&target), reads, "{target:?} reads {reads:?}");
        }
    }

    #[test]
    fn a_value_that_measures_nothing_new_says_so_in_the_language_file() {
        assert!(
            redundant_warning(&Catalogue::french()).starts_with("Cette cote n'apporte rien"),
            "the warning comes from the catalogue, not from a constant",
        );
    }
}
