//! What the rows of the section view's panel say.

use cao_render::MatterInFront;

use crate::lang::Catalogue;

/// Which side of the cutting plane a row is about.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    Behind,
    InFront,
}

/// A side's row: which side, and how its matter is drawn.
pub fn side(lang: &Catalogue, side: Side, state: MatterInFront) -> String {
    let state = super::matter::state(lang, state);
    let key = match side {
        Side::Behind => "section.behind",
        Side::InFront => "section.in_front",
    };
    lang.t_with(key, &[("state", &state)])
}
