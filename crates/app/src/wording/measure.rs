//! What a measure says, once the drawing has been read.
//!
//! A measure gets more room than a dimension does: a dimension has to be one
//! value, because a value is what the drawing is held to, while a measure
//! holds the drawing to nothing and can say several things at once.
//!
//! A straight run says three, and they are not a list — the distance and the
//! two reaches are the three sides of a right triangle, so each number is put
//! on the side it measures rather than stacked in a block beside the drawing.
//! That is also what lets the two reaches be drawn in the colours of the axes
//! they run along, which says which is which without a word.

use cao_prefs::config::UnitDisplay;
use cao_sketch::{DimensionTarget, Reading};

use crate::lang::Catalogue;

/// What a measure says, and where each number belongs.
#[derive(Clone, Debug, PartialEq)]
pub enum Said {
    /// The three sides of the triangle a straight run is shown as.
    Triangle {
        span: String,
        across: String,
        up: String,
    },
    /// One place to write in, for what is not a run: a circle says its radius
    /// and its diameter, an angle its opening.
    Beside(Vec<String>),
}

/// What a measure says, from what it read.
///
/// Lengths go through the same unit the rest of the interface shows, so a
/// drawing read in centimetres does not suddenly answer in millimetres — but
/// **unrounded**, unlike a dimension's. A dimension is typed and read back, and
/// typing 40 should give 40 rather than 39.999999999999996; a measure reports
/// what is there, and a report that rounds is how a drawing that drifted by a
/// hair goes on looking exact.
pub fn says(
    lang: &Catalogue,
    target: DimensionTarget,
    reading: Reading,
    unit: UnitDisplay,
) -> Said {
    let say = |key: &str, value: String| lang.t_with(key, &[("value", &value)]);
    match reading {
        Reading::Gap { span, offsets } => Said::Triangle {
            span: say(headline(target), unit.in_full(span)),
            across: unit.in_full(offsets.x),
            up: unit.in_full(offsets.y),
        },
        // Both, because a measure need not choose: a hole is drilled to a
        // diameter and a clearance is checked on a radius, and the one gesture
        // this tool would otherwise have to teach is the one that picks
        // between them.
        Reading::Round { radius } => Said::Beside(vec![
            say("measure.radius", unit.in_full(radius)),
            say("measure.diameter", unit.in_full(radius * 2.0)),
        ]),
        Reading::Opening { degrees } => {
            Said::Beside(vec![say("measure.angle", format!("{degrees}"))])
        }
    }
}

/// What the run itself is called: a trait has a length, and everything else is
/// a distance between two things.
///
/// The two reaches carry no word of their own — each is written on a side
/// drawn in its axis's colour, and a `ΔX` beside a red line says nothing the
/// red line has not already said.
fn headline(target: DimensionTarget) -> &'static str {
    match target {
        DimensionTarget::Length(_) => "measure.length",
        _ => "measure.distance",
    }
}

#[cfg(test)]
mod tests;
