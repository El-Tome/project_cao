//! Whether two parallel walls stand face to face at a place: both hold a face
//! there, at some height they share.
//!
//! Only there can their triangles lie on each other where the walls all but
//! meet, and only there are the steps of the grid beside the lines they meet
//! along withheld. Where one of them is gone — a union keeps each wall where
//! the other is not, two blocks stacked keep each wall at its own heights —
//! the place is one wall's alone, and withholding its steps would only
//! lengthen its chords past the tolerance.

use std::cell::OnceCell;

use glam::{DVec2, DVec3};

use super::{APART, Wall, bears, lies_on};
use crate::brep::curve::Curve;
use crate::brep::domain::Location;
use crate::brep::scale::Scale;
use crate::brep::surface::Cylinder;
use crate::brep::topology::{Body, FaceId, SurfaceId};

/// Two parallel walls, and once asked, their faces and the heights they are
/// compared at: one between every two heights where a circle or a vertex of
/// either stands, so that each is a height where no face of either begins or
/// ends.
pub(in crate::brep::tessellation) struct Facing<'a> {
    body: &'a Body,
    walls: [Wall; 2],
    room: f64,
    known: OnceCell<([Vec<FaceId>; 2], Vec<f64>)>,
}

impl<'a> Facing<'a> {
    pub(super) fn of(body: &'a Body, one: &Wall, other: &Wall) -> Facing<'a> {
        let eps = body.scale().eps();
        let between = other.1.origin - one.1.origin;
        let apart = (between - one.1.axis * one.1.axis.dot(between)).length();
        let [near, far] = [
            (one.1.radius - other.1.radius).abs(),
            one.1.radius + other.1.radius,
        ];
        let touching = (apart - near).abs() <= eps || (apart - far).abs() <= eps;
        Facing {
            body,
            walls: [*one, *other],
            room: if touching { eps } else { eps * APART },
            known: OnceCell::new(),
        }
    }

    /// How near each other the two walls stand where they all but meet, and
    /// no sample of either is taken: a fifth of the kernel's tolerance, twice
    /// what the rules tell apart — or the whole of it for two walls decided
    /// to touch, which the kernel may have left overlapping by that much, so
    /// that round the line they touch along one pokes through the other.
    pub(super) fn room(&self) -> f64 {
        self.room
    }

    fn known(&self) -> &([Vec<FaceId>; 2], Vec<f64>) {
        self.known.get_or_init(|| {
            let body = self.body;
            let eps = body.scale().eps();
            let axis = self.walls[0].1.axis;
            let walls = self.walls;
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
            (faces, heights)
        })
    }

    /// Whether both walls hold a face at the angle of `point`, seen from
    /// each one's axis, at a height they share. So it is taken to be when
    /// that cannot be told: a wall standing at one height alone, a wall
    /// holding no face — a circle printed on a cap, its wall gone — or a
    /// face the kernel cannot locate a place against.
    pub(super) fn at(&self, point: DVec3) -> bool {
        let (_, heights) = self.known();
        heights.is_empty() || heights.iter().any(|height| self.both(point, *height))
    }

    fn both(&self, point: DVec3, height: f64) -> bool {
        let (faces, _) = self.known();
        let eps = self.body.scale().eps();
        let holds = |side: usize| {
            let angle = self.walls[side].1.parameters(point).x;
            faces[side].is_empty()
                || faces[side].iter().any(|face| {
                    !matches!(
                        self.body.locate(*face, DVec2::new(angle, height), eps),
                        Ok(Location::Outside)
                    )
                })
        };
        holds(0) && holds(1)
    }
}

/// Every two parallel walls of a body, and where they face each other.
pub(in crate::brep::tessellation) struct Zones<'a> {
    pairs: Vec<Facing<'a>>,
}

impl<'a> Zones<'a> {
    pub(in crate::brep::tessellation) fn of(body: &'a Body, walls: &[Wall]) -> Zones<'a> {
        let mut pairs = Vec::new();
        for (at, one) in walls.iter().enumerate() {
            for other in &walls[at + 1..] {
                if one.1.axis.cross(other.1.axis).length() <= Scale::RELATIVE {
                    pairs.push(Facing::of(body, one, other));
                }
            }
        }
        Zones { pairs }
    }

    /// Where two parallel walls, by id, face each other.
    pub(super) fn between(&self, one: SurfaceId, other: SurfaceId) -> Option<&Facing<'a>> {
        self.pairs.iter().find(|facing| {
            let [first, second] = facing.walls.map(|(id, _)| id);
            (first, second) == (one, other) || (first, second) == (other, one)
        })
    }

    /// Whether `point`, on `cylinder`, stands within the room of a parallel
    /// wall that faces that one there: a sample there would be as good as on
    /// the other wall.
    pub(in crate::brep::tessellation) fn crowded(&self, cylinder: &Cylinder, point: DVec3) -> bool {
        self.pairs.iter().any(|facing| {
            let [one, other] = facing.walls;
            let partner = if one.1 == *cylinder {
                other.1
            } else if other.1 == *cylinder {
                one.1
            } else {
                return false;
            };
            partner.distance(point).abs() < facing.room() && facing.at(point)
        })
    }
}
