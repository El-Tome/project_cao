//! The matter a body encloses, from its exact faces: by the divergence
//! theorem, a third of the flux of the position out through its boundary.
//!
//! Each face's flux is turned by Green's theorem into an integral over its
//! loops, which keep the face on their left seen from outside: that is
//! anticlockwise in the surface's parameters when the face is not flipped,
//! so a flipped face gives its flux the right sign by itself.

use glam::DVec3;

use super::curve::Curve;
use super::domain::traced;
use super::surface::{Cylinder, Plane, Surface};
use super::topology::{Body, Coedge};
use super::trace::Trace;

impl Body {
    pub fn volume(&self) -> f64 {
        let flux: f64 = self
            .faces
            .iter()
            .flat_map(|face| {
                let surface = self.surface(face.surface);
                face.loops
                    .iter()
                    .flatten()
                    .map(move |coedge| self.flux_along(surface, *coedge))
            })
            .sum();
        flux / 3.0
    }

    /// What one use of an edge adds to the flux through its face: in closed
    /// form for the lines and circles a plane or a cylinder carries, and by
    /// quadrature along the curve for any other.
    fn flux_along(&self, surface: &Surface, coedge: Coedge) -> f64 {
        let edge = self.edge(coedge.edge);
        let curve = self.curve(edge.curve);
        let exact = match (surface, traced(curve, surface, edge.from, edge.to)) {
            (Surface::Plane(plane), Ok(trace)) => swept(&trace).map(|area| plane.offset() * area),
            (Surface::Cylinder(cylinder), Ok(Trace::Segment { from, to })) => {
                wall(cylinder, from.x, to.x, from.y, to.y)
            }
            _ => None,
        };
        let flux = exact.unwrap_or_else(|| along(curve, edge.from, edge.to, surface));
        if coedge.forward { flux } else { -flux }
    }
}

/// `½∫(s dt − t ds)` along a trace in a plane's parameters: the area Green's
/// theorem gives a plane face, whose flux is that area times the plane's
/// offset since the position's component along the normal is the offset
/// everywhere on it.
fn swept(trace: &Trace) -> Option<f64> {
    match *trace {
        Trace::Segment { from, to } => Some(from.perp_dot(to) / 2.0),
        Trace::Round {
            center,
            radius,
            start,
            sweep,
        } => {
            let (end_sin, end_cos) = (start + sweep).sin_cos();
            let (start_sin, start_cos) = start.sin_cos();
            let moment = center.x * (end_sin - start_sin) - center.y * (end_cos - start_cos);
            Some((radius * radius * sweep + radius * moment) / 2.0)
        }
        Trace::Graph { .. } => None,
    }
}

/// `−∫ h (r² + r o·ρ(θ)) dθ` along a straight trace in a cylinder's `(θ, h)`:
/// on the cylinder the position's component along the radial direction
/// `ρ(θ)` is `r + o·ρ(θ)` and the area element `r dθ dh`, and the 1-form
/// whose derivative that is, `−h (r² + r o·ρ) dθ`, is defined all the way
/// round, so a band with no seam needs none. A ruling adds nothing; a level
/// circle has a closed form; anything else is left to the quadrature.
fn wall(cylinder: &Cylinder, from: f64, to: f64, low: f64, high: f64) -> Option<f64> {
    if from == to {
        return Some(0.0);
    }
    if low != high {
        return None;
    }
    let radius = cylinder.radius;
    let (along_u, along_v) = (
        cylinder.origin.dot(cylinder.u),
        cylinder.origin.dot(cylinder.v),
    );
    let (to_sin, to_cos) = to.sin_cos();
    let (from_sin, from_cos) = from.sin_cos();
    let moment = along_u * (to_sin - from_sin) - along_v * (to_cos - from_cos);
    Some(-low * (radius * radius * (to - from) + radius * moment))
}

/// The flux a stretch of any curve adds on either surface, by composite
/// Gauss–Legendre over the curve's own parameter.
fn along(curve: &Curve, from: f64, to: f64, surface: &Surface) -> f64 {
    const PANELS: usize = 64;
    let width = (to - from) / PANELS as f64;
    let mut total = 0.0;
    for panel in 0..PANELS {
        let middle = from + width * (panel as f64 + 0.5);
        for (node, weight) in GAUSS_LEGENDRE {
            for side in [-1.0, 1.0] {
                let t = middle + side * node * width / 2.0;
                let (point, speed) = (curve.point(t), curve.derivative(t));
                let density = match surface {
                    Surface::Plane(plane) => planar(plane, point, speed),
                    Surface::Cylinder(cylinder) => radial(cylinder, point, speed),
                };
                total += weight * density * width / 2.0;
            }
        }
    }
    total
}

fn planar(plane: &Plane, point: DVec3, speed: DVec3) -> f64 {
    let at = plane.parameters(point);
    let pace = glam::DVec2::new(speed.dot(plane.u), speed.dot(plane.v));
    plane.offset() * at.perp_dot(pace) / 2.0
}

fn radial(cylinder: &Cylinder, point: DVec3, speed: DVec3) -> f64 {
    let from = point - cylinder.origin;
    let height = from.dot(cylinder.axis);
    let outward = (from - cylinder.axis * height).normalize();
    let turning = speed.dot(cylinder.axis.cross(outward));
    -height * (cylinder.radius + cylinder.origin.dot(outward)) * turning
}

/// The positive nodes of the eight-point rule on `[-1, 1]`, with their weights.
const GAUSS_LEGENDRE: [(f64, f64); 4] = [
    (0.183_434_642_495_649_8, 0.362_683_783_378_362),
    (0.525_532_409_916_329, 0.313_706_645_877_887_3),
    (0.796_666_477_413_626_7, 0.222_381_034_453_374_5),
    (0.960_289_856_497_536_3, 0.101_228_536_290_376_3),
];

#[cfg(test)]
mod tests;
