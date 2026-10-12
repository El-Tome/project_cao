//! A conic as a listing gives it, read by its own formula:
//!
//! ```text
//! p(τ) = A + T·τ·s(kτ²) + N·τ²·c(kτ²)
//! ```
//!
//! summed term by term near the anchor and read off the sine of half the
//! angle beyond it — not the kernel's route, so that a slip in one is not
//! checked by the same slip. Neither form subtracts two values close to
//! each other, so a place keeps its digits however far the conic's centre
//! stands.

use glam::DVec3;

use crate::brep::Conic;

/// Up to this `|kτ²|` both sums are taken term by term: every term is then
/// smaller than the one before it, and the last below rounding.
const SUMMED: f64 = 1.0;

/// Terms after the first: the tenth is `1/21!` of the first at `|kτ²| = 1`.
const TERMS: u32 = 10;

/// The place a conic reaches at its parameter `at`.
pub(super) fn point(conic: &Conic, at: f64) -> DVec3 {
    let (along, across) = reach(conic.k, at);
    conic.anchor + conic.tangent * along + conic.bend * across
}

/// How far the anchored form goes along its tangent and along its bend at
/// `at`: `τ·s(kτ²)` and `τ²·c(kτ²)`, the first `sin x / √k` and the second
/// `(1 − cos x) / k` with `x = √k·τ`, through `sinh` on a hyperbola.
fn reach(k: f64, at: f64) -> (f64, f64) {
    let z = k * at * at;
    if z.abs() <= SUMMED {
        return summed(z, at);
    }
    let x = z.abs().sqrt();
    let half = x / 2.0;
    let (sine, half_sine) = if k > 0.0 {
        (x.sin(), half.sin())
    } else {
        (x.sinh(), half.sinh())
    };
    (
        at * sine / x,
        at * at * (half_sine / half) * (half_sine / half) / 2.0,
    )
}

/// `Σ (−z)ʲ τ^(2j+1) / (2j+1)!` and `Σ (−z)ʲ τ^(2j+2) / (2j+2)!`, from
/// their first terms on.
fn summed(z: f64, at: f64) -> (f64, f64) {
    let (mut odd, mut even) = (at, at * at / 2.0);
    let (mut along, mut across) = (odd, even);
    for term in 1..=TERMS {
        let n = f64::from(2 * term);
        odd *= -z / (n * (n + 1.0));
        even *= -z / ((n + 1.0) * (n + 2.0));
        along += odd;
        across += even;
    }
    (along, across)
}
