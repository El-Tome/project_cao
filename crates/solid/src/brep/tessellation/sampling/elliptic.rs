//! The angles an elliptic wall's rims take besides its grid (#571).

use crate::brep::surface::{EllipticCylinder, Surface};
use crate::brep::topology::{Body, SurfaceId};

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
