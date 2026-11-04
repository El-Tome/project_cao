//! A conic as STEP's own: `p'''= −k·p'` gives `p = C + X·cos ωτ + Y·sin ωτ`
//! for an ellipse, the same with `cosh` and `sinh` for a hyperbola, and
//! `A + T·τ + N·τ²/2` for a parabola, `ω = √|k|`. The anchor need not be a
//! vertex, so `X` and `Y` are conjugate half-diameters, turned onto the
//! axes by a shift of the parameter that keeps its sense.

use glam::DVec3;

use super::super::instances::Instances;
use super::placed;
use crate::brep::Conic;

pub fn written(instances: &mut Instances, conic: &Conic) -> usize {
    if conic.k == 0.0 {
        return parabola(instances, conic);
    }
    let omega = conic.k.abs().sqrt();
    let center = conic.anchor + conic.bend / conic.k;
    let (x, y) = (-conic.bend / conic.k, conic.tangent / omega);
    let dot = x.dot(y);
    let (first, second) = if conic.k > 0.0 {
        let shift = (2.0 * dot).atan2(x.length_squared() - y.length_squared()) / 2.0;
        let (sin, cos) = shift.sin_cos();
        (x * cos + y * sin, y * cos - x * sin)
    } else {
        let shift = (-2.0 * dot / (x.length_squared() + y.length_squared())).atanh() / 2.0;
        let (sinh, cosh) = (shift.sinh(), shift.cosh());
        (x * cosh + y * sinh, x * sinh + y * cosh)
    };
    let placement = placed(instances, center, first.normalize(), second);
    let (a, b) = (
        instances.length(first.length()),
        instances.length(second.length()),
    );
    let kind = if conic.k > 0.0 {
        "ELLIPSE"
    } else {
        "HYPERBOLA"
    };
    instances.add(format!("{kind}('',#{placement},{a},{b})"))
}

/// The vertex where the speed along the axis `N` vanishes; there the
/// parabola is `V + w·c·σ + n·|N|·σ²/2`, which STEP's `V + f·(n·u² + w·2u)`
/// is for `f = c²/(2|N|)` and `u = σ·|N|/c`.
fn parabola(instances: &mut Instances, conic: &Conic) -> usize {
    let bend = conic.bend.length();
    let axis = conic.bend / bend;
    let across: DVec3 = conic.tangent - axis * conic.tangent.dot(axis);
    let vertex = conic.point(-conic.tangent.dot(axis) / bend);
    let placement = placed(instances, vertex, axis, across);
    let focal = instances.length(across.length_squared() / (2.0 * bend));
    instances.add(format!("PARABOLA('',#{placement},{focal})"))
}
