//! What lies on what: every vertex on the surfaces of the faces around it and
//! of the face holding it within it, every edge's ends on its vertices, every
//! edge on the surfaces of the faces beside it — each surface and curve
//! evaluated by its own formula here.

use std::collections::BTreeSet;

use glam::DVec3;

use super::Mislisted;
use crate::brep::{Curve, Listing, Surface};

/// How many stretches an edge is cut into to be tried along its length: its
/// two ends and fifteen places between.
const PIECES: usize = 16;

pub(super) fn lying(
    listing: &Listing,
    uses: &[Vec<(usize, bool)>],
    room: f64,
) -> Result<(), Mislisted> {
    let mut around = vec![BTreeSet::new(); listing.vertices.len()];
    for (edge, listed) in listing.edges.iter().enumerate() {
        for vertex in listed.ends.iter().flatten() {
            around[*vertex].extend(uses[edge].iter().map(|(face, _)| *face));
        }
    }
    for (face, listed) in listing.faces.iter().enumerate() {
        if let Some(vertex) = listed.apex {
            around[vertex].insert(face);
        }
    }
    for (vertex, faces) in around.iter().enumerate() {
        for &face in faces {
            let distance = off(&listing.faces[face].surface, listing.vertices[vertex]);
            if !within(distance, room) {
                return Err(Mislisted::VertexOffFace {
                    vertex,
                    face,
                    distance,
                });
            }
        }
    }

    for (edge, listed) in listing.edges.iter().enumerate() {
        let Some(ends) = listed.ends else {
            continue;
        };
        for (end, (vertex, at)) in ends.into_iter().zip([listed.from, listed.to]).enumerate() {
            let distance = (point(&listed.curve, at) - listing.vertices[vertex]).length();
            if !within(distance, room) {
                return Err(Mislisted::EndAway {
                    edge,
                    end,
                    vertex,
                    distance,
                });
            }
        }
    }

    for (edge, listed) in listing.edges.iter().enumerate() {
        for piece in 0..=PIECES {
            let at = listed.from + (listed.to - listed.from) * piece as f64 / PIECES as f64;
            let place = point(&listed.curve, at);
            for &(face, _) in &uses[edge] {
                let distance = off(&listing.faces[face].surface, place);
                if !within(distance, room) {
                    return Err(Mislisted::OffFace {
                        edge,
                        face,
                        at,
                        distance,
                    });
                }
            }
        }
    }
    Ok(())
}

/// Whether a distance is short enough, a distance that is no number never
/// being: a kernel that divided by a length of nought must not pass for one
/// that measured nought.
fn within(distance: f64, room: f64) -> bool {
    distance <= room
}

/// How far a place stands from a surface, either side; from a cone, not
/// written yet (#536): beyond any room.
fn off(surface: &Surface, place: DVec3) -> f64 {
    match surface {
        Surface::Plane(plane) => (place - plane.origin).dot(plane.normal.normalize()).abs(),
        Surface::Cylinder(cylinder) => {
            let axis = cylinder.axis.normalize();
            let from = place - cylinder.origin;
            ((from - axis * from.dot(axis)).length() - cylinder.radius).abs()
        }
        Surface::Cone(_) => f64::INFINITY,
    }
}

/// The place a curve reaches at its parameter `at`.
pub(super) fn point(curve: &Curve, at: f64) -> DVec3 {
    match curve {
        Curve::Line(line) => line.origin + line.direction * at,
        Curve::Circle(circle) => {
            let (sin, cos) = at.sin_cos();
            circle.center + (circle.u * cos + circle.v * sin) * circle.radius
        }
        Curve::Meet(meet) => meet.point(at),
    }
}
