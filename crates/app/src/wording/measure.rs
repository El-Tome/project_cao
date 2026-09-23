//! What a measure says, once the drawing has been read.
//!
//! A measure gets more room than a dimension does: a dimension has to be one
//! value, because a value is what the drawing is held to, while a measure
//! holds the drawing to nothing and can say several things at once. So the
//! lines here are a list rather than a sentence — the distance and the reach
//! along each axis, the radius and the diameter together.

use cao_prefs::config::UnitDisplay;
use cao_sketch::{DimensionTarget, Reading};

use crate::lang::Catalogue;

/// The lines a measure's label is written on, in order.
///
/// Lengths go through the same unit the rest of the interface shows, so a
/// drawing read in centimetres does not suddenly answer in millimetres.
pub fn lines(
    lang: &Catalogue,
    target: DimensionTarget,
    reading: Reading,
    unit: UnitDisplay,
) -> Vec<String> {
    let say = |key: &str, value: String| lang.t_with(key, &[("value", &value)]);
    match reading {
        Reading::Gap { span, offsets } => vec![
            say(headline(target), unit.format(span)),
            say("measure.offset_across", unit.format(offsets.x)),
            say("measure.offset_up", unit.format(offsets.y)),
        ],
        // Both, because a measure need not choose: a hole is drilled to a
        // diameter and a clearance is checked on a radius, and the one gesture
        // this tool would otherwise have to teach is the one that picks
        // between them.
        Reading::Round { radius } => vec![
            say("measure.radius", unit.format(radius)),
            say("measure.diameter", unit.format(radius * 2.0)),
        ],
        Reading::Opening { degrees } => vec![say("measure.angle", format!("{degrees:.1}"))],
    }
}

/// What the first line of a straight run is called: a trait has a length, and
/// everything else is a distance between two things.
fn headline(target: DimensionTarget) -> &'static str {
    match target {
        DimensionTarget::Length(_) => "measure.length",
        _ => "measure.distance",
    }
}

#[cfg(test)]
mod tests;
