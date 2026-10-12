//! The flux of the position through a face of a cone (#536).
//!
//! On the cone the position's component along the normal is `c + w_h·o(θ)`,
//! with `c` the meridian line's offset and `o(θ)` the origin's component
//! along the radial direction, and the area element `ρ(l) dθ dl`: the
//! 1-form whose derivative that is, `−Q(l)·(c + w_h·o(θ)) dθ` with `Q` the
//! integral of `ρ` from an anchor, is defined all the way round, so a band
//! with no seam needs none. A ruling adds nothing; a level circle has a
//! closed form; anything else is left to the quadrature.
//!
//! The anchor is the apex on a face that reaches it, by a corner or by
//! holding it within: `Q` vanishes there, so the stretch of the apex no edge
//! runs along adds nothing, as it must. On any other face the loops go round
//! the axis nought times, net, so the anchor cancels, and it is taken at the
//! foot of the meridian line, within the box, where the apex of a cone a
//! hair from a cylinder would stand a hundred million reaches away.

use glam::{DVec2, DVec3};

use crate::brep::surface::Cone;
use crate::brep::topology::{Body, FaceId};

impl Body {
    /// The length along the ruling a face of `cone` anchors its flux at.
    pub(super) fn anchor(&self, face: FaceId, cone: &Cone) -> f64 {
        let apex = cone.apex();
        let eps = self.scale.eps();
        let at_the_apex = self.face(face).loops.iter().flatten().any(|coedge| {
            self.edge(coedge.edge)
                .ends
                .iter()
                .flatten()
                .any(|vertex| self.vertex(*vertex).point.distance(apex) <= eps)
        });
        if at_the_apex || self.apex_held(face).is_some() {
            cone.apex_at()
        } else {
            0.0
        }
    }
}

/// `∫ρ dl` from `anchor` to `l`: exact as a trapezoid, `ρ` being linear.
fn swept(cone: &Cone, anchor: f64, l: f64) -> f64 {
    (l - anchor) * (cone.radius_at(anchor) + cone.radius_at(l)) / 2.0
}

/// What a straight trace in the cone's `(θ, l)` adds: nothing along a
/// ruling, the closed form along a level circle, and none for anything else.
pub(super) fn straight(cone: &Cone, anchor: f64, from: DVec2, to: DVec2) -> Option<f64> {
    if from.x == to.x {
        return Some(0.0);
    }
    if from.y != to.y {
        return None;
    }
    let offset = cone.ruling.perp().dot(cone.foot);
    let (along_u, along_v) = (cone.origin.dot(cone.u), cone.origin.dot(cone.v));
    let (to_sin, to_cos) = to.x.sin_cos();
    let (from_sin, from_cos) = from.x.sin_cos();
    let moment = along_u * (to_sin - from_sin) - along_v * (to_cos - from_cos);
    Some(-swept(cone, anchor, from.y) * (offset * (to.x - from.x) + cone.ruling.x * moment))
}

/// The flux a curve passing through `point` at `speed` adds per unit of its
/// parameter, from the same 1-form: nought at the apex itself, where it
/// vanishes on a face anchored there.
pub(super) fn conical(cone: &Cone, anchor: f64, point: DVec3, speed: DVec3) -> f64 {
    let from = point - cone.origin;
    let height = from.dot(cone.axis);
    let radial = from - cone.axis * height;
    let rho = radial.length();
    if rho == 0.0 {
        return 0.0;
    }
    let outward = radial / rho;
    let turning = speed.dot(cone.axis.cross(outward)) / rho;
    let l = (DVec2::new(height, rho) - cone.foot).dot(cone.ruling);
    let offset = cone.ruling.perp().dot(cone.foot);
    -swept(cone, anchor, l) * (offset + cone.ruling.x * cone.origin.dot(outward)) * turning
}
