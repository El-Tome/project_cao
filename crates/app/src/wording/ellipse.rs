use cao_sketch::EllipseMode;

use crate::lang::Catalogue;

/// The only place a way of drawing an ellipse is turned into an instruction.
///
/// `cao_sketch` knows what each mode needs and how many clicks that is; this
/// decides how the waiting is said while the tool asks for them.
pub fn asks_for(lang: &Catalogue, mode: EllipseMode) -> String {
    lang.t(match mode {
        EllipseMode::ByCentre => "ellipse.asks_for.by_centre",
        EllipseMode::ByEnds => "ellipse.asks_for.by_ends",
    })
}

#[cfg(test)]
mod tests;
