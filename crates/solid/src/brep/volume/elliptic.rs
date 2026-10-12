//! The flux of the position out through a face of an elliptic wall (#571).
//!
//! With `A` and `B` its half-axes and `o` its origin, the flux through
//! `∂θ × ∂h` is `f(θ) = AB + B (o·u) cos θ + A (o·v) sin θ`, whatever the
//! height, and the 1-form whose derivative that is, `−h f(θ) dθ`, is defined
//! all the way round, so a band with no seam needs none. A ruling adds
//! nothing; a level stretch and the conic a plane cuts the wall along, whose
//! height is `h₀ + h_c cos θ + h_s sin θ`, have closed forms; any other curve
//! is left to the quadrature, at the density `density` reads.

use glam::{DVec2, DVec3};

use crate::brep::conic::Conic;
use crate::brep::surface::EllipticCylinder;

/// What a straight trace in the wall's `(θ, h)` adds: nothing along a
/// ruling, a closed form at one height; none for any other.
pub(super) fn straight(wall: &EllipticCylinder, from: DVec2, to: DVec2) -> Option<f64> {
    if from.x == to.x {
        return Some(0.0);
    }
    (from.y == to.y).then(|| -swept(wall, [from.y, 0.0, 0.0], [from.x, to.x]))
}

/// What the stretch from `from` to `to` of a conic on the wall adds: its
/// height a sinusoid of the wall's angle, which runs as `√k` times its
/// parameter.
pub(super) fn graph(wall: &EllipticCylinder, conic: &Conic, from: f64, to: f64) -> f64 {
    let normal = conic.plane.normal;
    let rise = normal.dot(wall.axis);
    let level = conic.plane.offset() - normal.dot(wall.origin);
    let height = [
        level / rise,
        -wall.major * normal.dot(wall.u) / rise,
        -wall.minor * normal.dot(wall.v) / rise,
    ];
    let start = wall.parameters(conic.anchor).x;
    let root = conic.k.sqrt();
    -swept(wall, height, [start + root * from, start + root * to])
}

/// `∫ h(θ) f(θ) dθ` from `θ₁` to `θ₂`, `h = h₀ + h_c cos θ + h_s sin θ`.
fn swept(wall: &EllipticCylinder, [h0, hc, hs]: [f64; 3], [one, two]: [f64; 2]) -> f64 {
    let (major, minor) = (wall.major, wall.minor);
    let flat = major * minor;
    let p = minor * wall.origin.dot(wall.u);
    let q = major * wall.origin.dot(wall.v);
    let (sin_two, cos_two) = two.sin_cos();
    let (sin_one, cos_one) = one.sin_cos();
    let turned = two - one;
    let sines = sin_two - sin_one;
    let cosines = cos_two - cos_one;
    let doubled = ((2.0 * two).sin() - (2.0 * one).sin()) / 4.0;
    let squares = (sin_two * sin_two - sin_one * sin_one) / 2.0;
    h0 * flat * turned + (h0 * p + hc * flat) * sines - (h0 * q + hs * flat) * cosines
        + hc * p * (turned / 2.0 + doubled)
        + hs * q * (turned / 2.0 - doubled)
        + (hc * q + hs * p) * squares
}

/// The 1-form `−h f(θ) dθ` read along a curve at `point`, running at
/// `speed`: the turn of its angle read in the section squashed onto the unit
/// circle.
pub(super) fn density(wall: &EllipticCylinder, point: DVec3, speed: DVec3) -> f64 {
    let from = point - wall.origin;
    let height = from.dot(wall.axis);
    let (major, minor) = (wall.major, wall.minor);
    let [x, y] = [from.dot(wall.u) / major, from.dot(wall.v) / minor];
    let [run_x, run_y] = [speed.dot(wall.u) / major, speed.dot(wall.v) / minor];
    let turning = (x * run_y - y * run_x) / (x * x + y * y);
    let (sin, cos) = (y / x.hypot(y), x / x.hypot(y));
    let flux = major * minor
        + minor * wall.origin.dot(wall.u) * cos
        + major * wall.origin.dot(wall.v) * sin;
    -height * flux * turning
}
