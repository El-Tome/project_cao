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
/// drawing read in centimetres does not suddenly answer in millimetres — and
/// held to `figures` significant figures, where a dimension is rounded far
/// harder. A dimension is typed and read back, so it rounds to what a person
/// would type; a measure reports what is there, so how much of the number is
/// worth reading is the reader's call, and `figures` is where they say it.
pub fn says(
    lang: &Catalogue,
    target: DimensionTarget,
    reading: Reading,
    unit: UnitDisplay,
    figures: u32,
) -> Said {
    let say = |key: &str, value: String| lang.t_with(key, &[("value", &value)]);
    match reading {
        Reading::Gap { span, offsets } => Said::Triangle {
            span: say(headline(target), unit.in_figures(span, figures)),
            across: unit.in_figures(offsets.x, figures),
            up: unit.in_figures(offsets.y, figures),
        },
        // Both, because a measure need not choose: a hole is drilled to a
        // diameter and a clearance is checked on a radius, and the one gesture
        // this tool would otherwise have to teach is the one that picks
        // between them.
        Reading::Round { radius } => Said::Beside(vec![
            say("measure.radius", unit.in_figures(radius, figures)),
            say("measure.diameter", unit.in_figures(radius * 2.0, figures)),
        ]),
        Reading::Opening { degrees } => Said::Beside(vec![say(
            "measure.angle",
            cao_prefs::config::to_figures(degrees, figures),
        )]),
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
