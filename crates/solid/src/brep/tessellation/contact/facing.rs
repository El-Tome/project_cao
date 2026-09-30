//! Whether two parallel walls stand face to face at a place: both hold a face
//! there, at some height they share.
//!
//! Only there can their triangles lie on each other where the walls all but
//! meet, and only there are the steps of the grid beside the lines they meet
//! along withheld. Where one of them is gone — a union keeps each wall where
//! the other is not, two blocks stacked keep each wall at its own heights —
//! the place is one wall's alone, and withholding its steps would only
//! lengthen its chords past the tolerance.

use glam::{DVec2, DVec3};

use super::{Wall, bears, lies_on};
use crate::brep::curve::Curve;
use crate::brep::domain::Location;
use crate::brep::topology::{Body, FaceId};

/// The faces of two parallel walls and the heights they are compared at:
/// one between every two heights where a circle or a vertex of either
/// stands, so that each is a height where no face of either begins or ends.
pub(super) struct Facing<'a> {
    body: &'a Body,
    walls: [Wall; 2],
    faces: [Vec<FaceId>; 2],
    heights: Vec<f64>,
}

impl<'a> Facing<'a> {
    pub(super) fn of(body: &'a Body, one: &Wall, other: &Wall) -> Facing<'a> {
        let eps = body.scale().eps();
        let axis = one.1.axis;
        let walls = [*one, *other];
        let faces = walls.map(|(id, _)| {
            body.face_ids()
                .filter(|face| body.face(*face).surface == id)
                .collect()
        });
        let circles = body
            .edge_ids()
            .filter_map(|id| match body.curve(body.edge(id).curve) {
                Curve::Circle(circle)
                    if walls.iter().any(|(_, wall)| lies_on(circle, wall, eps)) =>
                {
                    Some(circle.center.dot(axis))
                }
                _ => None,
            });
        let vertices = body
            .vertex_ids()
            .map(|id| body.vertex(id))
            .filter(|vertex| walls.iter().any(|wall| bears(body, vertex, wall)))
            .map(|vertex| vertex.point.dot(axis));
        let mut levels: Vec<f64> = circles.chain(vertices).collect();
        levels.sort_by(f64::total_cmp);
        levels.dedup_by(|higher, lower| *higher - *lower <= eps);
        let heights = levels
            .windows(2)
            .map(|pair| (pair[0] + pair[1]) / 2.0)
            .collect();
        Facing {
            body,
            walls,
            faces,
            heights,
        }
    }

    /// Whether both walls hold a face at the angle of `point`, seen from
    /// each one's axis, at a height they share. So it is taken to be when
    /// that cannot be told: a wall standing at one height alone, or a face
    /// the kernel cannot locate a place against.
    pub(super) fn at(&self, point: DVec3) -> bool {
        if self.heights.is_empty() {
            return true;
        }
        let eps = self.body.scale().eps();
        let holds = |side: usize, height: f64| {
            let angle = self.walls[side].1.parameters(point).x;
            self.faces[side].iter().any(|face| {
                !matches!(
                    self.body.locate(*face, DVec2::new(angle, height), eps),
                    Ok(Location::Outside)
                )
            })
        };
        self.heights
            .iter()
            .any(|height| holds(0, *height) && holds(1, *height))
    }
}
