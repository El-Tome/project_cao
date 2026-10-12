//! The angles an elliptic wall's rims take besides its grid (#571), and
//! where a whole one starts.

use std::f64::consts::TAU;

use crate::brep::conic::{Conic, On};
use crate::brep::surface::{EllipticCylinder, Surface};
use crate::brep::topology::{Body, Edge, SurfaceId};

/// The angles about an elliptic wall of every vertex lying on it, where a
/// ruling leaves that every rim of the wall takes a sample on.
pub(super) fn on_the_wall(body: &Body, wall: &EllipticCylinder) -> Vec<f64> {
    let Some(rank) = body
        .surfaces
        .iter()
        .position(|surface| *surface == Surface::Elliptic(*wall))
    else {
        return Vec::new();
    };
    let id = SurfaceId(rank as u32);
    body.vertices
        .iter()
        .filter(|vertex| vertex.on.contains(&id))
        .map(|vertex| wall.parameters(vertex.point).x)
        .collect()
}

/// A whole ring of an elliptic wall read from half way between two steps of
/// the wall's grid of `steps` steps a turn, as long as it ran; any other
/// stretch as it is. The ring starts at the end of its major axis, a step of
/// the grid, and that step's angle, read back off the ring's anchor, may
/// stand a rounding past both ends of the turn and be sampled at neither.
pub(super) fn between_steps(conic: &Conic, edge: &Edge, steps: usize) -> Edge {
    if edge.ends.is_some() {
        return edge.clone();
    }
    let step = TAU / steps as f64;
    let start = conic.seen_on(On::Wall, edge.from)[0].x;
    let middle = (start / step).floor() * step + step / 2.0;
    let shift = (middle - start) / conic.k.sqrt();
    Edge {
        from: edge.from + shift,
        to: edge.to + shift,
        ..edge.clone()
    }
}
