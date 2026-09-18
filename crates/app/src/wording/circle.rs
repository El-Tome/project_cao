use cao_sketch::CircleMode;

use crate::lang::Catalogue;

/// The only place a way of drawing a circle is turned into an instruction.
///
/// `cao_sketch` knows what each mode needs and how many clicks that is; this
/// decides how the waiting is said while the tool asks for them.
pub fn asks_for(lang: &Catalogue, mode: CircleMode) -> String {
    lang.t(match mode {
        CircleMode::Center => "circle.asks_for.center",
        CircleMode::TwoPoints => "circle.asks_for.two_points",
        CircleMode::ThreePoints => "circle.asks_for.three_points",
        CircleMode::TwoTangents => "circle.asks_for.two_tangents",
        CircleMode::ThreeTangents => "circle.asks_for.three_tangents",
    })
}

#[cfg(test)]
mod tests;
