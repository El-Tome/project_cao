//! The faces, edges and vertices of a raised profile: a cap at each end, and a
//! wall on each piece, with a vertical edge wherever two walls meet.

use std::f64::consts::TAU;

use glam::DVec3;

use super::piece::{Named, Piece};
use crate::brep::curve::{Circle, Curve, Line};
use crate::brep::scale::Scale;
use crate::brep::surface::{Cylinder, Plane, Surface};
use crate::brep::topology::{
    Body, Coedge, CurveId, Edge, EdgeId, Face, SurfaceId, Vertex, VertexId,
};
use crate::profile::Frame;

/// The body the pieces make when their frame is lifted by `lift`, which
/// stands along the frame's normal: every outline anticlockwise about it,
/// every hole clockwise, so that the matter is on the left of each piece.
pub(super) fn raise(frame: Frame, lift: DVec3, contours: &[Vec<Named>], eps: f64) -> Body {
    let mut walls = Walls {
        body: Body {
            surfaces: Vec::new(),
            curves: Vec::new(),
            vertices: Vec::new(),
            edges: Vec::new(),
            faces: Vec::new(),
            scale: Scale::of(1.0),
            arrivals: Vec::new(),
        },
        frame,
        lift,
        up: lift.normalize(),
        eps,
    };
    let (bottom, bottom_flipped) = walls.plane(frame.origin, -walls.up);
    let (top, top_flipped) = walls.plane(frame.origin + lift, walls.up);
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
        },
        Face {
            surface: top,
            flipped: top_flipped,
            loops: top_loops,
            numbers: vec![1],
        },
    ];
    walls.body.faces = caps.into_iter().chain(sides).collect();
    walls.body.scale = Scale::of(walls.body.reach());
    walls.body.arrivals = vec![walls.body.scale; walls.body.surfaces.len()];
    walls.body
}

struct Walls {
    body: Body,
    frame: Frame,
    lift: DVec3,
    up: DVec3,
    eps: f64,
}

/// The wall a piece stands on, and whether its matter lies on the side its
/// surface's own normal points to.
struct Side {
    surface: SurfaceId,
    flipped: bool,
    cylinder: Option<Cylinder>,
}

fn reversed(coedge: Coedge) -> Coedge {
    Coedge {
        edge: coedge.edge,
        forward: !coedge.forward,
    }
}

impl Walls {
    /// The walls of one contour, pushed onto `faces`, and the runs of its
    /// bottom and its top, in the contour's own order.
    fn contour(
        &mut self,
        named: &[Named],
        caps: [SurfaceId; 2],
        faces: &mut Vec<Face>,
    ) -> (Vec<Coedge>, Vec<Coedge>) {
        let contour: Vec<Piece> = named.iter().map(|named| named.piece).collect();
        let contour = contour.as_slice();
        let sides: Vec<Side> = contour.iter().map(|piece| self.side(piece)).collect();
        if let ([piece @ Piece::Arc { center, sweep, .. }], [side]) = (contour, sides.as_slice())
            && piece.is_ring()
            && let Some(cylinder) = side.cylinder
        {
            let center = self.frame.at(*center);
            let below = self.ring(&cylinder, center, *sweep);
            let above = self.ring(&cylinder, center + self.lift, *sweep);
            faces.push(Face {
                surface: side.surface,
                flipped: side.flipped,
                loops: vec![vec![below], vec![reversed(above)]],
                numbers: named[0].numbers.clone(),
            });
            return (vec![below], vec![above]);
        }
        let count = contour.len();
        let mut corners = Vec::with_capacity(count);
        for (index, piece) in contour.iter().enumerate() {
            let before = sides[(index + count - 1) % count].surface;
            let point = self.frame.at(piece.from());
            let bottom = self.vertex(point, [caps[0], before, sides[index].surface]);
            let top = self.vertex(point + self.lift, [caps[1], before, sides[index].surface]);
            corners.push((bottom, top, self.line(bottom, top)));
        }
        let (mut below, mut above) = (Vec::new(), Vec::new());
        for (index, piece) in contour.iter().enumerate() {
            let (start, end) = (corners[index], corners[(index + 1) % count]);
            let (bottom, top) = match (*piece, sides[index].cylinder) {
                (Piece::Arc { center, sweep, .. }, Some(cylinder)) => {
                    let center = self.frame.at(center);
                    (
                        self.arc(&cylinder, center, [start.0, end.0], sweep),
                        self.arc(&cylinder, center + self.lift, [start.1, end.1], sweep),
                    )
                }
                _ => (self.line(start.0, end.0), self.line(start.1, end.1)),
            };
            faces.push(Face {
                surface: sides[index].surface,
                flipped: sides[index].flipped,
                loops: vec![vec![bottom, end.2, reversed(top), reversed(start.2)]],
                numbers: named[index].numbers.clone(),
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
                let surface = self.surface(Surface::Plane(plane), &corners);
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
                    surface: self.surface(Surface::Cylinder(cylinder), &corners),
                    flipped: sweep < 0.0,
                    cylinder: Some(cylinder),
                }
            }
        }
    }

    /// The plane through `point` whose matter lies against `outward`.
    fn plane(&mut self, point: DVec3, outward: DVec3) -> (SurfaceId, bool) {
        let (plane, turned) = Plane::through(point, outward);
        (self.surface(Surface::Plane(plane), &[point]), turned)
    }

    /// The surface's id, shared with a surface already there when the piece
    /// standing on it would stand on that one within `eps` too: each of its
    /// `corners`, and on a cylinder each point of its circles. Two runs of a
    /// contour on one line stand on one plane; two a hair apart do not.
    fn surface(&mut self, surface: Surface, corners: &[DVec3]) -> SurfaceId {
        let eps = self.eps;
        let same = |known: &Surface| {
            let alike = match (known, &surface) {
                (Surface::Plane(known), Surface::Plane(plane)) => {
                    known.normal.dot(plane.normal) > 0.0
                }
                (Surface::Cylinder(known), Surface::Cylinder(cylinder)) => {
                    known.axis.dot(cylinder.axis) > 0.0
                        && known.axis.cross(cylinder.axis).length() <= Scale::RELATIVE
                        && known.origin.distance(cylinder.origin)
                            + (known.radius - cylinder.radius).abs()
                            <= eps
                }
                _ => false,
            };
            alike
                && corners
                    .iter()
                    .all(|corner| known.distance(*corner).abs() <= eps)
        };
        let found = self.body.surfaces.iter().position(same);
        let index = found.unwrap_or_else(|| {
            self.body.surfaces.push(surface);
            self.body.surfaces.len() - 1
        });
        SurfaceId(index as u32)
    }

    fn vertex(&mut self, point: DVec3, on: [SurfaceId; 3]) -> VertexId {
        let mut on = on.to_vec();
        on.sort();
        on.dedup();
        self.body.vertices.push(Vertex { point, on });
        VertexId(self.body.vertices.len() as u32 - 1)
    }

    /// A straight edge from `start` to `end`, used that way.
    fn line(&mut self, start: VertexId, end: VertexId) -> Coedge {
        let [from, to] = [start, end].map(|id| self.body.vertex(id).point);
        let line = Line::through(from, to - from);
        let [from, to] = [from, to].map(|point| line.parameter(point));
        if from < to {
            self.edge(Curve::Line(line), Some([start, end]), from, to, true)
        } else {
            self.edge(Curve::Line(line), Some([end, start]), to, from, false)
        }
    }

    /// An arc of the cylinder's circle about `center`, from `ends[0]` turning
    /// by `sweep` about `up` to `ends[1]`, used that way.
    fn arc(
        &mut self,
        cylinder: &Cylinder,
        center: DVec3,
        ends: [VertexId; 2],
        sweep: f64,
    ) -> Coedge {
        let circle = Circle::on(cylinder, (center - cylinder.origin).dot(cylinder.axis));
        let turning = sweep * cylinder.axis.dot(self.up).signum();
        let [start, end] = if turning > 0.0 {
            ends
        } else {
            [ends[1], ends[0]]
        };
        let from = circle.parameter(self.body.vertex(start).point);
        let span = turning.abs();
        self.edge(
            Curve::Circle(circle),
            Some([start, end]),
            from,
            from + span,
            turning > 0.0,
        )
    }

    /// A whole circle of the cylinder about `center`, used the way `sweep`
    /// turns about `up`.
    fn ring(&mut self, cylinder: &Cylinder, center: DVec3, sweep: f64) -> Coedge {
        let circle = Circle::on(cylinder, (center - cylinder.origin).dot(cylinder.axis));
        let forward = sweep * cylinder.axis.dot(self.up) > 0.0;
        self.edge(Curve::Circle(circle), None, 0.0, TAU, forward)
    }

    fn edge(
        &mut self,
        curve: Curve,
        ends: Option<[VertexId; 2]>,
        from: f64,
        to: f64,
        forward: bool,
    ) -> Coedge {
        self.body.curves.push(curve);
        let curve = CurveId(self.body.curves.len() as u32 - 1);
        self.body.edges.push(Edge {
            curve,
            ends,
            from,
            to,
        });
        Coedge {
            edge: EdgeId(self.body.edges.len() as u32 - 1),
            forward,
        }
    }
}
