//! What the interface says about what an operation just did.
//!
//! Most operations say nothing: the drawing shows what happened. The ones that
//! changed something invisible — a scale the part now holds, a value it could
//! not honour — are applied from several call sites, and this is the one place
//! that turns what they answered into a sentence.

use cao_part::{DimensionOutcome, Outcome};
use cao_sketch::LengthOutcome;

use crate::lang::Catalogue;
use crate::wording::dimension;

pub fn message(lang: &Catalogue, outcome: Option<Outcome>) -> Option<String> {
    match outcome? {
        Outcome::Dimension(DimensionOutcome::ScaleDefined {
            millimeters_per_unit,
        }) => Some(dimension::scale_defined(lang, millimeters_per_unit)),
        Outcome::Dimension(DimensionOutcome::Reference) => Some(dimension::redundant_warning(lang)),
        Outcome::Dimension(DimensionOutcome::Geometry(LengthOutcome::BestEffort)) => {
            Some(dimension::conflict_warning(lang))
        }
        Outcome::Dimension(DimensionOutcome::Geometry(
            LengthOutcome::Exact | LengthOutcome::Degenerate,
        )) => None,
        Outcome::Cut { rules, values } => cut_cost(lang, rules, values),
    }
}

/// What a cut could not carry over to either piece: counted, never named. A
/// line has no room to list them, and a drawing that quietly loses a rule is
/// one whose shape moves for no reason the user can see.
fn cut_cost(lang: &Catalogue, rules: usize, values: usize) -> Option<String> {
    let (rules, values) = (rules.to_string(), values.to_string());
    match (rules.as_str(), values.as_str()) {
        ("0", "0") => None,
        (_, "0") => Some(lang.t_with("sketch.cut_lost_rules", &[("count", &rules)])),
        ("0", _) => Some(lang.t_with("sketch.cut_lost_values", &[("count", &values)])),
        _ => Some(lang.t_with(
            "sketch.cut_lost_both",
            &[("rules", &rules), ("values", &values)],
        )),
    }
}

#[cfg(test)]
mod tests;
