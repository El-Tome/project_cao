//! The faces, edges and vertices of a raised profile: a cap at each end, and a
//! wall on each piece, with a vertical edge wherever two walls meet — and
//! where two loops touch, one both loops share.

use glam::DVec3;

use super::touch::Parted;
use crate::brep::laying::{Laying, reversed};
use crate::brep::piece::{Named, Piece};
use crate::brep::scale::Scale;
use crate::brep::surface::{Cylinder, Plane, Surface};
use crate::brep::topology::{Body, Coedge, Face, SurfaceId, VertexId};
use crate::profile::Frame;

/// The body the pieces make when their frame is lifted by `lift`, which
/// stands along the frame's normal: every outline anticlockwise about it,
/// every hole clockwise, so that the matter is on the left of each piece.
/// Each of the `touches` is one corner at each end, and one line between.
pub(super) fn raise(
    frame: Frame,
    lift: DVec3,
    contours: &[Parted],
    touches: usize,
    eps: f64,
) -> Body {
    let mut walls = Walls {
        laying: Laying::new(eps),
        frame,
        lift,
        up: lift.normalize(),
        touches: vec![None; touches],
    };
    let (bottom, bottom_flipped) = walls.laying.plane(frame.origin, -walls.up);
    let (top, top_flipped) = walls.laying.plane(frame.origin + lift, walls.up);
    let mut sides = Vec::new();
    let (mut bottom_loops, mut top_loops) = (Vec::new(), Vec::new());
    for contour in contours {
        let (below, above) = walls.contour(contour, [bottom, top], &mut sides);
        bottom_loops.push(below.iter().rev().map(|coedge| reversed(*coedge)).collect());
        top_loops.push(above);
    }
    let caps = [
        Face {
            surface: bottom,
            flipped: bottom_flipped,
            loops: bottom_loops,
            numbers: vec![0],
            apex: None,
        },
        Face {
            surface: top,
            flipped: top_flipped,
            loops: top_loops,
            numbers: vec![1],
            apex: None,
        },
    ];
    walls.laying.body.faces = caps.into_iter().chain(sides).collect();
    walls.laying.body.scale = Scale::of(walls.laying.body.reach());
    walls.laying.body.arrivals = vec![walls.laying.body.scale; walls.laying.body.surfaces.len()];
    walls.laying.body
}

struct Walls {
    laying: Laying,
    frame: Frame,
    lift: DVec3,
    up: DVec3,
    /// The corners at each end of a touch and the line between, once laid.
    touches: Vec<Option<(VertexId, VertexId, Coedge)>>,
}

/// The wall a piece stands on, and whether its matter lies on the side its
/// surface's own normal points to.
struct Side {
    surface: SurfaceId,
    flipped: bool,
    cylinder: Option<Cylinder>,
}

impl Walls {
    /// The walls of one contour, pushed onto `faces`, and the runs of its
    /// bottom and its top, in the contour's own order.
    fn contour(
        &mut self,
        parted: &Parted,
        caps: [SurfaceId; 2],
        faces: &mut Vec<Face>,
    ) -> (Vec<Coedge>, Vec<Coedge>) {
        let named: &[Named] = &parted.pieces;
        let contour: Vec<Piece> = named.iter().map(|named| named.piece).collect();
        let contour = contour.as_slice();
        let sides: Vec<Side> = contour.iter().map(|piece| self.side(piece)).collect();
        if let ([piece @ Piece::Arc { center, sweep, .. }], [side]) = (contour, sides.as_slice())
            && piece.is_ring()
            && parted.touches[0].is_none()
            && let Some(cylinder) = side.cylinder
        {
            let center = self.frame.at(*center);
            let below = self.laying.ring(&cylinder, center, *sweep, self.up);
            let above = self
                .laying
                .ring(&cylinder, center + self.lift, *sweep, self.up);
            faces.push(Face {
                surface: side.surface,
                flipped: side.flipped,
                loops: vec![vec![below], vec![reversed(above)]],
                numbers: named[0].numbers.clone(),
                apex: None,
            });
            return (vec![below], vec![above]);
        }
        let count = contour.len();
        let mut corners = Vec::with_capacity(count);
        for (index, piece) in contour.iter().enumerate() {
            let before = sides[(index + count - 1) % count].surface;
            let point = self.frame.at(piece.from());
            let [below, above] = caps.map(|cap| [cap, before, sides[index].surface]);
            corners.push(match parted.touches[index] {
                Some(touch) => self.touched(touch, point, [below, above]),
                None => {
                    let bottom = self.laying.vertex(point, &below);
                    let top = self.laying.vertex(point + self.lift, &above);
                    (bottom, top, self.laying.line(bottom, top))
                }
            });
        }
        let (mut below, mut above) = (Vec::new(), Vec::new());
        for (index, piece) in contour.iter().enumerate() {
            let (start, end) = (corners[index], corners[(index + 1) % count]);
            let (bottom, top) = match (*piece, sides[index].cylinder) {
                (Piece::Arc { center, sweep, .. }, Some(cylinder)) => {
                    let center = self.frame.at(center);
                    (
                        self.laying
                            .arc(&cylinder, center, [start.0, end.0], sweep, self.up),
                        self.laying.arc(
                            &cylinder,
                            center + self.lift,
                            [start.1, end.1],
                            sweep,
                            self.up,
                        ),
                    )
                }
                _ => (
                    self.laying.line(start.0, end.0),
                    self.laying.line(start.1, end.1),
                ),
            };
            faces.push(Face {
                surface: sides[index].surface,
                flipped: sides[index].flipped,
                loops: vec![vec![bottom, end.2, reversed(top), reversed(start.2)]],
                numbers: named[index].numbers.clone(),
                apex: None,
            });
            below.push(bottom);
            above.push(top);
        }
        (below, above)
    }

    fn side(&mut self, piece: &Piece) -> Side {
        let [from, to] = [piece.from(), piece.to()].map(|corner| self.frame.at(corner));
        let corners = [from, to, from + self.lift, to + self.lift];
        match *piece {
            Piece::Straight { .. } => {
                let (plane, flipped) = Plane::through(from, (to - from).cross(self.up));
                let surface = self.laying.surface(Surface::Plane(plane), &corners);
                Side {
                    surface,
                    flipped,
                    cylinder: None,
                }
            }
            Piece::Arc {
                center,
                radius,
                sweep,
                ..
            } => {
                let cylinder = Cylinder::about(self.frame.at(center), self.up, radius);
                Side {
                    surface: self.laying.surface(Surface::Cylinder(cylinder), &corners),
                    flipped: sweep < 0.0,
                    cylinder: Some(cylinder),
                }
            }
        }
    }

    /// The corners at either end of a touch, on the surfaces each loop
    /// through it adds, and the line between them, laid by the first loop.
    fn touched(
        &mut self,
        touch: usize,
        point: DVec3,
        on: [[SurfaceId; 3]; 2],
    ) -> (VertexId, VertexId, Coedge) {
        let Some((bottom, top, line)) = self.touches[touch] else {
            let bottom = self.laying.vertex(point, &on[0]);
            let top = self.laying.vertex(point + self.lift, &on[1]);
            let laid = (bottom, top, self.laying.line(bottom, top));
            self.touches[touch] = Some(laid);
            return laid;
        };
        for (corner, surfaces) in [bottom, top].into_iter().zip(on) {
            let held = &mut self.laying.body.vertices[corner.0 as usize].on;
            held.extend(surfaces);
            held.sort();
            held.dedup();
        }
        (bottom, top, line)
    }
}
