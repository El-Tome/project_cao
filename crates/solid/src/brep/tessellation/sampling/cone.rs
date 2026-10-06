//! The grid of a cone: one for the whole surface, read at its widest, so that
//! every circle lying on it is sampled at the same angles from the `u` it
//! shares with every cylinder of its axis.
//!
//! A circle's own grid is cut for its own radius, and a cone's rims stand at
//! two: sampled each on its own, a narrow rim and a wide one would take
//! angles that never meet, and a triangle between them would span a step of
//! the wide one's from a single sample of the narrow one. On the cone's grid
//! as well, every rim has a sample at every one of its angles, and the strip
//! between two rims at two consecutive angles is a flat trapezoid whose sag is
//! the chord's at the widest radius. Each circle keeps its own grid besides,
//! which the other face along it — a bore's wall, a disc — is cut on. A
//! circle lying on no cone is sampled as before, bit for bit.

use std::collections::BTreeMap;
use std::f64::consts::TAU;

use glam::DVec3;

use super::super::contact::Contact;
use super::divisions;
use crate::brep::curve::{Circle, Curve};
use crate::brep::surface::{Cone, Surface};
use crate::brep::topology::{Body, SurfaceId};

/// The grid of a cone: how many steps a turn, and the angles of the vertices
/// lying on it but at its apex, where the angle is rounding.
pub(super) struct Grid {
    cone: Cone,
    pub(super) steps: usize,
    angles: Vec<f64>,
}

impl Grid {
    /// The directions from the axis a circle of the cone is sampled along:
    /// the grid's, then the vertices'.
    fn rays(&self) -> impl Iterator<Item = DVec3> + '_ {
        (0..self.steps)
            .map(|step| TAU * step as f64 / self.steps as f64)
            .chain(self.angles.iter().copied())
            .map(|angle| self.cone.radial(angle))
    }
}

/// The grid of every cone of the body, read at the furthest any circle lying
/// on it or vertex on it stands from the axis.
pub(super) fn grids(body: &Body, tolerance: f64) -> BTreeMap<SurfaceId, Grid> {
    let eps = body.scale().eps();
    let circles: Vec<&Circle> = body
        .edge_ids()
        .filter_map(|id| match body.curve(body.edge(id).curve) {
            Curve::Circle(circle) => Some(circle),
            Curve::Line(_) | Curve::Meet(_) => None,
        })
        .collect();
    (0..body.surfaces.len() as u32)
        .map(SurfaceId)
        .filter_map(|id| match body.surface(id) {
            Surface::Cone(cone) => Some((id, *cone)),
            Surface::Plane(_) | Surface::Cylinder(_) => None,
        })
        .map(|(id, cone)| {
            let mut widest = circles
                .iter()
                .filter(|circle| cone.holds(circle, eps))
                .map(|circle| circle.radius)
                .fold(0.0, f64::max);
            let mut angles = Vec::new();
            for vertex in body.vertex_ids().map(|vertex| body.vertex(vertex)) {
                if !vertex.on.contains(&id) {
                    continue;
                }
                let from = vertex.point - cone.origin;
                let radial = from - cone.axis * from.dot(cone.axis);
                widest = widest.max(radial.length());
                if radial.length() > eps {
                    angles.push(radial.dot(cone.v).atan2(radial.dot(cone.u)));
                }
            }
            let steps = divisions(widest, tolerance);
            (
                id,
                Grid {
                    cone,
                    steps,
                    angles,
                },
            )
        })
        .collect()
}

/// What a circle takes besides its grid, `contact`, and the rays of every
/// cone it lies on as well: None for a circle lying on no cone, which takes
/// `contact` alone.
pub(super) fn widened(
    contact: &Contact,
    grids: &BTreeMap<SurfaceId, Grid>,
    circle: &Circle,
    eps: f64,
) -> Option<Contact> {
    let mut rays: Vec<DVec3> = grids
        .values()
        .filter(|grid| grid.cone.holds(circle, eps))
        .flat_map(Grid::rays)
        .collect();
    if rays.is_empty() {
        return None;
    }
    rays.splice(0..0, contact.rays.iter().copied());
    Some(Contact {
        rays,
        withheld: contact.withheld.clone(),
        anchors: contact.anchors.clone(),
        beside: contact.beside.clone(),
        beneath: contact.beneath.clone(),
    })
}
