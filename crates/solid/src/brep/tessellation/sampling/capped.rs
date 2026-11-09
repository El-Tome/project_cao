//! Whether the most steps a turn is cut into leave some round of a body
//! further from its triangles than the tolerance asked.

use super::Samples;
use super::steps::{most, needed};
use crate::brep::curve::Curve;
use crate::brep::surface::Surface;
use crate::brep::topology::Body;

impl Samples {
    /// Whether some round the faces of `body` are cut along is too wide for
    /// the most steps a turn is cut into to hold it within `tolerance`: a
    /// wall, a circle edge, or the grid of a cone or a round.
    pub(in crate::brep::tessellation) fn capped(&self, body: &Body, tolerance: f64) -> bool {
        let span = self.span;
        let walls = body
            .face_ids()
            .any(|id| match body.surface(body.face(id).surface) {
                Surface::Cylinder(wall) => beyond_most(wall.radius, tolerance, span),
                Surface::Elliptic(wall) => beyond_most(wall.major, tolerance, span),
                _ => false,
            });
        let circles = body.edge_ids().any(|id| {
            matches!(body.curve(body.edge(id).curve),
                Curve::Circle(circle) if beyond_most(circle.radius, tolerance, span))
        });
        walls || circles || self.grids.values().any(|grid| grid.capped)
    }
}

/// Whether a turn of `radius`, on a part spanning `span`, needs more steps
/// than the most it is cut into for its chords to stand within `tolerance`.
pub(super) fn beyond_most(radius: f64, tolerance: f64, span: f64) -> bool {
    needed(radius, tolerance).is_none_or(|needed| needed > most(radius, span) as f64)
}
