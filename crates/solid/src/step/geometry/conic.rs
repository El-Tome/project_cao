//! A conic as STEP's own: `p'''= −k·p'` gives `p = C + X·cos ωτ + Y·sin ωτ`
//! for an ellipse, the same with `cosh` and `sinh` for a hyperbola, and
//! `A + T·τ + N·τ²/2` for a parabola, `ω = √|k|`. The anchor need not be a
//! vertex, so `X` and `Y` are conjugate half-diameters, turned onto the
//! axes by a shift of the parameter that keeps its sense.

use glam::DVec3;

use super::super::Unwritten;
use super::super::instances::Instances;
use super::{placed, reach, spline};
use crate::brep::{Conic, Curve};

/// How many reaches the centre may stand from the curve before doubles no
/// longer hold the conic written about it: a reader adds terms that large,
/// each rounded by a part in 10¹⁶, and a million reaches leaves that at a
/// hundredth of the kernel's tolerance, a fifty-millionth of the reach. The
/// kernel leaves a cut along a ruling, a parabola, a `k` of rounding size,
/// its centre a billion billion reaches away.
const FAR: f64 = 1e6;

const SAMPLES: usize = 64;

pub fn written(
    instances: &mut Instances,
    conic: &Conic,
    curve: &Curve,
    from: f64,
    to: f64,
) -> Result<usize, Unwritten> {
    let (low, high) = (from.min(to), from.max(to));
    let (reach, tolerance) = reach(curve, low, high);
    if conic.bend.length() / conic.k.abs() > FAR * reach {
        let strays = (0..=SAMPLES)
            .map(|index| low + (high - low) * index as f64 / SAMPLES as f64)
            .map(|t| {
                let parabola = conic.anchor + conic.tangent * t + conic.bend * (t * t / 2.0);
                parabola.distance(conic.point(t))
            })
            .fold(0.0, f64::max);
        return if strays <= tolerance {
            Ok(parabola(instances, conic))
        } else {
            spline::written(instances, curve, from, to)
        };
    }
    Ok(about_the_centre(instances, conic))
}

fn about_the_centre(instances: &mut Instances, conic: &Conic) -> usize {
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
