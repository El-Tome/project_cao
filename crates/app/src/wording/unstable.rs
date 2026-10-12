//! What the user reads of a step the kernel built though it reads it
//! unstably (#552): which way the plane leans, and by how little.

use cao_solid::{Lean, Unstable};

use crate::lang::Catalogue;

/// The notice a step built at an unstable lean earns.
pub fn notice(lang: &Catalogue, unstable: Unstable) -> String {
    let key = match unstable.lean {
        Lean::Square => "extrusion.unstable_square",
        Lean::Along => "extrusion.unstable_along",
    };
    lang.t_with(key, &[("angle", &angle(unstable.degrees))])
}

/// An angle to two significant figures, written out without an exponent:
/// the leans said here are all under a tenth of a degree, and a user reads
/// `0.0000002` where `2e-7` reads as a code.
fn angle(degrees: f64) -> String {
    if !degrees.is_finite() || degrees <= 0.0 {
        return "0".to_string();
    }
    let decimals = (1.0 - degrees.log10().floor()).max(0.0) as usize;
    let written = format!("{degrees:.decimals$}");
    if written.contains('.') {
        written
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
    } else {
        written
    }
}

#[cfg(test)]
mod tests;
