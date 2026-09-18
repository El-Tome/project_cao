use cao_sketch::{DimensionTarget, SketchAxis};

use crate::lang::Catalogue;
use crate::wording::constraints;

/// Shown when a value would add nothing to a shape that is already settled.
pub fn redundant_warning(lang: &Catalogue) -> String {
    lang.t("dimension.redundant")
}

/// Shown when a value cannot be held alongside what is already fixed: it is
/// refused, and the drawing stays exactly as it was.
pub fn conflict_warning(lang: &Catalogue) -> String {
    lang.t("dimension.conflict")
}

/// Shown once, when a dimension is the first a sketch ever gets and fixes its
/// scale instead of moving the drawing.
pub fn scale_defined(lang: &Catalogue, millimeters_per_unit: f64) -> String {
    lang.t_with(
        "sketch.scale_set",
        &[("mm", &format!("{millimeters_per_unit:.4}"))],
    )
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
        DimensionTarget::Angle { .. } | DimensionTarget::ArcSweep(_) => {
            lang.t_with("dimension.label.angle", measured)
        }
        DimensionTarget::AxisAngle { axis, .. } => lang.t_with(
            "dimension.label.axis_angle",
            &[
                ("value", value.as_str()),
                ("axis", &constraints::axis(lang, *axis)),
            ],
        ),
        DimensionTarget::Radius(_) | DimensionTarget::ArcRadius(_) => {
            lang.t_with("dimension.label.radius", measured)
        }
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
        DimensionTarget::ArcRadius(arc) | DimensionTarget::ArcSweep(arc) => {
            lang.t_with("dimension.spans.arc", &[("arc", &arc.0.to_string())])
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
mod tests;
