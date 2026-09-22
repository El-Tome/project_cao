//! The dimensions a freshly-drawn shape earns on its own, before the user
//! types anything.

use crate::aim::LockedInput;
use crate::constraints::{Constraint, DimensionTarget, SketchAxis};
use crate::ellipse::EllipseId;
use crate::sketch::{SegmentId, Sketch};

/// What a freshly-drawn rectangle earns on its own: three right angles — the
/// fourth follows — held as constraints rather than dimensions, since a right
/// angle is a relationship the rectangle holds by construction, not a
/// measurement someone typed and could retype. Alongside them, the length of
/// whichever of the two neighbouring sides the user actually typed a size
/// for. A side left untyped stays at whatever length the cursor gave it,
/// undimensioned, the same as a line drawn with no value entered. A length
/// already redundant with what the drawing holds is left out.
pub fn rectangle_dimensions(
    sketch: &Sketch,
    sides: [SegmentId; 4],
    locked: LockedInput,
    scale: f64,
) -> (Vec<Constraint>, Vec<(DimensionTarget, f64)>) {
    let corners = (0..3)
        .map(|corner| Constraint::Perpendicular {
            first: sides[corner],
            second: sides[corner + 1],
        })
        .collect();

    let mut wanted: Vec<(DimensionTarget, f64)> = Vec::new();
    for (side, typed) in sides.iter().take(2).zip([locked.first, locked.second]) {
        if typed.is_some() {
            wanted.push((
                DimensionTarget::Length(*side),
                sketch.segment_length(*side) * scale,
            ));
        }
    }
    (corners, settled(sketch, wanted, scale))
}

/// Places on the line just drawn whatever the user typed, and the right angle
/// it was aimed at.
///
/// A value that would say nothing is left out: the drawing already holds it,
/// and a second copy could only be redundant.
pub fn line_dimensions(
    sketch: &Sketch,
    segment: SegmentId,
    locked: LockedInput,
    square_with: Option<SegmentId>,
    scale: f64,
) -> Vec<(DimensionTarget, f64)> {
    let mut wanted: Vec<(DimensionTarget, f64)> = Vec::new();
    if let Some(length) = locked.first {
        wanted.push((DimensionTarget::Length(segment), length));
    }
    if let Some(angle) = locked.second {
        wanted.push((
            DimensionTarget::AxisAngle {
                segment,
                axis: SketchAxis::U,
            },
            angle.abs(),
        ));
    }
    if let Some(first) = square_with {
        wanted.push((
            DimensionTarget::Angle {
                first,
                second: segment,
            },
            90.0,
        ));
    }
    settled(sketch, wanted, scale)
}

/// Places on a freshly-drawn symmetric segment its own length — read from the
/// drawing rather than repeated from what was typed, since a length typed only
/// reaches one of its two edges — and the angle it was drawn at.
pub fn symmetric_segment_dimensions(
    sketch: &Sketch,
    segment: SegmentId,
    locked: LockedInput,
    scale: f64,
) -> Vec<(DimensionTarget, f64)> {
    let mut wanted: Vec<(DimensionTarget, f64)> = Vec::new();
    if locked.first.is_some() {
        wanted.push((
            DimensionTarget::Length(segment),
            sketch.segment_length(segment) * scale,
        ));
    }
    if let Some(angle) = locked.second {
        wanted.push((
            DimensionTarget::AxisAngle {
                segment,
                axis: SketchAxis::U,
            },
            angle.abs(),
        ));
    }
    settled(sketch, wanted, scale)
}

/// Normalises and drops whatever is already redundant with the drawing.
/// Places on the ellipse just drawn the widths typed for its axes, and the
/// angle typed for the first: on the axes themselves, since they are what an
/// ellipse is measured by.
pub fn ellipse_dimensions(
    sketch: &Sketch,
    ellipse: EllipseId,
    first: LockedInput,
    second_width: Option<f64>,
    scale: f64,
) -> Vec<(DimensionTarget, f64)> {
    let Some(oval) = sketch.ellipses().get(ellipse.0) else {
        return Vec::new();
    };
    let mut wanted: Vec<(DimensionTarget, f64)> = Vec::new();
    if let Some(width) = first.first {
        wanted.push((DimensionTarget::Length(oval.first), width));
    }
    if let Some(angle) = first.second {
        wanted.push((
            DimensionTarget::AxisAngle {
                segment: oval.first,
                axis: SketchAxis::U,
            },
            angle.abs(),
        ));
    }
    if let Some(width) = second_width {
        wanted.push((DimensionTarget::Length(oval.second), width));
    }
    settled(sketch, wanted, scale)
}

pub(crate) fn settled(
    sketch: &Sketch,
    wanted: Vec<(DimensionTarget, f64)>,
    scale: f64,
) -> Vec<(DimensionTarget, f64)> {
    wanted
        .into_iter()
        .map(|(target, value)| (target.normalised(), value))
        .filter(|(target, _)| !sketch.would_be_redundant(*target, scale))
        .collect()
}

#[cfg(test)]
mod tests;
