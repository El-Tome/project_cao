//! Bodies built by hand from the arenas, for whatever reads a body before the
//! prism and the boolean can make one: the block and the stock of the sixteen
//! cases, and the configurations that break a kernel — a hole bored through, a
//! hole tangent inside the stock's wall, two holes touching, a cylinder lying
//! on a face.
//!
//! The block is 40 by 40 by 10 round the origin, from 0 to 10; the stock a
//! cylinder of radius 20 standing on the XY plane from 0 to 10; every hole has
//! a radius of 5.

use std::collections::BTreeMap;
use std::f64::consts::{PI, TAU};

use glam::DVec3;

use crate::brep::curve::{Circle, Curve, Line, Meet};
use crate::brep::scale::Scale;
use crate::brep::surface::{Cylinder, Plane, Surface};
use crate::brep::topology::{
    Body, Coedge, CurveId, Edge, EdgeId, Face, FaceId, SurfaceId, Vertex, VertexId,
};

pub(crate) const HEIGHT: f64 = 10.0;
pub(crate) const HALF_SIDE: f64 = 20.0;
pub(crate) const STOCK_RADIUS: f64 = 20.0;
pub(crate) const HOLE_RADIUS: f64 = 5.0;

/// A body put together piece by piece, each piece canonical as the kernel
/// would make it.
pub(crate) struct Build {
    body: Body,
    lines: BTreeMap<(VertexId, VertexId), EdgeId>,
}

impl Build {
    pub(crate) fn new() -> Build {
        Build {
            body: Body {
                surfaces: Vec::new(),
                curves: Vec::new(),
                vertices: Vec::new(),
                edges: Vec::new(),
                faces: Vec::new(),
                scale: Scale::of(1.0),
            },
            lines: BTreeMap::new(),
        }
    }

    fn surface(&mut self, surface: Surface) -> SurfaceId {
        self.body.surfaces.push(surface);
        SurfaceId(self.body.surfaces.len() as u32 - 1)
    }

    /// The plane through `point` square to `normal`, and whether a face on it
    /// whose matter lies against `outward` is flipped.
    pub(crate) fn plane(&mut self, point: DVec3, outward: DVec3) -> (SurfaceId, bool) {
        let (plane, turned) = Plane::through(point, outward);
        (self.surface(Surface::Plane(plane)), turned)
    }

    pub(crate) fn cylinder(&mut self, point: DVec3, axis: DVec3, radius: f64) -> SurfaceId {
        self.surface(Surface::Cylinder(Cylinder::about(point, axis, radius)))
    }

    pub(crate) fn vertex(&mut self, point: DVec3) -> VertexId {
        self.body.vertices.push(Vertex {
            point,
            on: Vec::new(),
        });
        VertexId(self.body.vertices.len() as u32 - 1)
    }

    fn edge(&mut self, curve: Curve, ends: Option<[VertexId; 2]>, from: f64, to: f64) -> EdgeId {
        self.body.curves.push(curve);
        let curve = CurveId(self.body.curves.len() as u32 - 1);
        self.body.edges.push(Edge {
            curve,
            ends,
            from,
            to,
        });
        EdgeId(self.body.edges.len() as u32 - 1)
    }

    /// The straight edge between two vertices, made once: it runs the way its
    /// line's parameter grows.
    pub(crate) fn line(&mut self, one: VertexId, other: VertexId) -> EdgeId {
        let key = (one.min(other), one.max(other));
        if let Some(edge) = self.lines.get(&key) {
            return *edge;
        }
        let (start, end) = (self.body.vertex(one).point, self.body.vertex(other).point);
        let line = Line::through(start, end - start);
        let (from, to) = (line.parameter(start), line.parameter(end));
        let edge = if from < to {
            self.edge(Curve::Line(line), Some([one, other]), from, to)
        } else {
            self.edge(Curve::Line(line), Some([other, one]), to, from)
        };
        self.lines.insert(key, edge);
        edge
    }

    /// The circle a cylinder has at `height`, whole: from a vertex round to
    /// the same vertex, or a ring with none.
    pub(crate) fn circle(
        &mut self,
        on: SurfaceId,
        height: f64,
        through: Option<VertexId>,
    ) -> EdgeId {
        let Surface::Cylinder(cylinder) = *self.body.surface(on) else {
            panic!("a circle is made on a cylinder");
        };
        let circle = Circle::on(&cylinder, height);
        match through {
            Some(vertex) => {
                let from = circle.parameter(self.body.vertex(vertex).point);
                self.edge(
                    Curve::Circle(circle),
                    Some([vertex, vertex]),
                    from,
                    from + TAU,
                )
            }
            None => self.edge(Curve::Circle(circle), None, 0.0, TAU),
        }
    }

    /// One whole loop of the curve two perpendicular cylinders meet along,
    /// with no vertex on it.
    pub(crate) fn meet(&mut self, first: SurfaceId, second: SurfaceId) -> EdgeId {
        let cylinder = |id| match *self.body.surface(id) {
            Surface::Cylinder(cylinder) => cylinder,
            Surface::Plane(_) => panic!("the curve is where two cylinders meet"),
        };
        let meet = Meet {
            first: cylinder(first),
            second: cylinder(second),
            component: 0,
        };
        self.edge(Curve::Meet(meet), None, 0.0, TAU)
    }

    /// The use of an edge leaving `from`: along its way when it starts there.
    pub(crate) fn leaving(&self, edge: EdgeId, from: VertexId) -> Coedge {
        let ends = self.body.edge(edge).ends.expect("an edge with ends");
        Coedge {
            edge,
            forward: ends[0] == from,
        }
    }

    pub(crate) fn face(
        &mut self,
        surface: SurfaceId,
        flipped: bool,
        loops: Vec<Vec<Coedge>>,
    ) -> FaceId {
        self.body.faces.push(Face {
            surface,
            flipped,
            loops,
        });
        FaceId(self.body.faces.len() as u32 - 1)
    }

    /// A flat face round `corners`, which turn counterclockwise seen from
    /// the side `outward` points to.
    pub(crate) fn polygon(&mut self, corners: &[VertexId], outward: DVec3) -> FaceId {
        let (surface, flipped) = self.plane(self.body.vertex(corners[0]).point, outward);
        let coedges = (0..corners.len())
            .map(|at| {
                let (here, next) = (corners[at], corners[(at + 1) % corners.len()]);
                let edge = self.line(here, next);
                self.leaving(edge, here)
            })
            .collect();
        self.face(surface, flipped, vec![coedges])
    }

    pub(crate) fn add_loop(&mut self, face: FaceId, coedges: Vec<Coedge>) {
        self.body.faces[face.0 as usize].loops.push(coedges);
    }

    /// The body, each vertex told the surfaces of the faces round it.
    pub(crate) fn finish(mut self, reach: f64) -> Body {
        let mut on: Vec<Vec<SurfaceId>> = vec![Vec::new(); self.body.vertices.len()];
        for face in &self.body.faces {
            for coedge in face.loops.iter().flatten() {
                for end in self.body.edges[coedge.edge.0 as usize]
                    .ends
                    .into_iter()
                    .flatten()
                {
                    on[end.0 as usize].push(face.surface);
                }
            }
        }
        for (vertex, mut surfaces) in self.body.vertices.iter_mut().zip(on) {
            surfaces.sort();
            surfaces.dedup();
            vertex.on = surfaces;
        }
        self.body.scale = Scale::of(reach);
        self.body
    }
}

/// The block's six faces round its eight corners, its top and its bottom
/// handed back so that holes can be bored through them.
fn block_in(build: &mut Build) -> (FaceId, FaceId) {
    let corner = |index: usize| {
        let pick = |bit: usize, low: f64, high: f64| if index & bit == 0 { low } else { high };
        DVec3::new(
            pick(1, -HALF_SIDE, HALF_SIDE),
            pick(2, -HALF_SIDE, HALF_SIDE),
            pick(4, 0.0, HEIGHT),
        )
    };
    let corners: Vec<VertexId> = (0..8).map(|index| build.vertex(corner(index))).collect();
    let quad = |build: &mut Build, picked: [usize; 4], outward: DVec3| {
        build.polygon(&picked.map(|index| corners[index]), outward)
    };
    let bottom = quad(build, [0, 2, 3, 1], DVec3::NEG_Z);
    let top = quad(build, [4, 5, 7, 6], DVec3::Z);
    quad(build, [0, 4, 6, 2], DVec3::NEG_X);
    quad(build, [1, 3, 7, 5], DVec3::X);
    quad(build, [0, 1, 5, 4], DVec3::NEG_Y);
    quad(build, [2, 6, 7, 3], DVec3::Y);
    (top, bottom)
}

/// A hole of radius five bored from the bottom to the top at `center`.
fn bore(build: &mut Build, top: FaceId, bottom: FaceId, center: DVec3) {
    let wall = build.cylinder(center, DVec3::Z, HOLE_RADIUS);
    let (low, high) = (
        build.circle(wall, 0.0, None),
        build.circle(wall, HEIGHT, None),
    );
    let use_of = |edge, forward| Coedge { edge, forward };
    build.face(
        wall,
        true,
        vec![vec![use_of(low, false)], vec![use_of(high, true)]],
    );
    build.add_loop(top, vec![use_of(high, false)]);
    build.add_loop(bottom, vec![use_of(low, true)]);
}

/// The stock's top and bottom, flat, handed back to be given their loops.
fn caps(build: &mut Build) -> ((SurfaceId, bool), (SurfaceId, bool)) {
    let top = build.plane(DVec3::new(0.0, 0.0, HEIGHT), DVec3::Z);
    let bottom = build.plane(DVec3::ZERO, DVec3::NEG_Z);
    (top, bottom)
}

pub(crate) fn block() -> Body {
    let mut build = Build::new();
    block_in(&mut build);
    build.finish(HALF_SIDE)
}

/// The stock's three faces, its top and its bottom handed back so that holes
/// can be bored through them.
fn stock_in(build: &mut Build) -> (FaceId, FaceId) {
    let wall = build.cylinder(DVec3::ZERO, DVec3::Z, STOCK_RADIUS);
    let (low, high) = (
        build.circle(wall, 0.0, None),
        build.circle(wall, HEIGHT, None),
    );
    let ((top, top_flipped), (bottom, bottom_flipped)) = caps(build);
    let use_of = |edge, forward| Coedge { edge, forward };
    build.face(
        wall,
        false,
        vec![vec![use_of(low, true)], vec![use_of(high, false)]],
    );
    let top = build.face(top, top_flipped, vec![vec![use_of(high, true)]]);
    let bottom = build.face(bottom, bottom_flipped, vec![vec![use_of(low, false)]]);
    (top, bottom)
}

/// The stock: a wall going all the way round between two rings, no vertex.
pub(crate) fn stock() -> Body {
    let mut build = Build::new();
    stock_in(&mut build);
    build.finish(STOCK_RADIUS)
}

/// The stock with a hole bored through at `center`, clear of its wall: its
/// top and its bottom each have two rings.
pub(crate) fn stock_with_a_hole_at(center: DVec3) -> Body {
    let mut build = Build::new();
    let (top, bottom) = stock_in(&mut build);
    bore(&mut build, top, bottom, center);
    build.finish(STOCK_RADIUS)
}

pub(crate) fn block_with_a_hole() -> Body {
    let mut build = Build::new();
    let (top, bottom) = block_in(&mut build);
    bore(&mut build, top, bottom, DVec3::new(8.0, 3.0, 0.0));
    build.finish(HALF_SIDE)
}

/// A line up from `bottom` to the top of the block, which two walls touch
/// along: each goes round from it back to it, and the line has four uses.
struct Touching {
    line: EdgeId,
    low: VertexId,
    high: VertexId,
    bottom: f64,
}

impl Touching {
    fn at(build: &mut Build, point: DVec3) -> Touching {
        Touching::down_to(build, point, 0.0)
    }

    fn down_to(build: &mut Build, point: DVec3, bottom: f64) -> Touching {
        let low = build.vertex(point + DVec3::Z * bottom);
        let high = build.vertex(point + DVec3::Z * HEIGHT);
        let line = build.line(low, high);
        Touching {
            line,
            low,
            high,
            bottom,
        }
    }

    /// A wall from the line round to it again, and the circles it ends on.
    /// Flipped for a hole, whose matter lies outside it.
    fn wall(&self, build: &mut Build, wall: SurfaceId, flipped: bool) -> (EdgeId, EdgeId) {
        let low = build.circle(wall, self.bottom, Some(self.low));
        let high = build.circle(wall, HEIGHT, Some(self.high));
        let use_of = |edge, forward| Coedge { edge, forward };
        let (up, down) = (
            build.leaving(self.line, self.low),
            build.leaving(self.line, self.high),
        );
        let lap = if flipped {
            vec![up, use_of(high, true), down, use_of(low, false)]
        } else {
            vec![use_of(low, true), up, use_of(high, false), down]
        };
        build.face(wall, flipped, vec![lap]);
        (low, high)
    }
}

/// The stock with a hole bored through at (15, 0), whose wall touches the
/// stock's inside along the line x = 20: both walls go round from it, and the
/// top and the bottom each visit its ends twice.
pub(crate) fn stock_with_a_tangent_hole() -> Body {
    let mut build = Build::new();
    let touching = Touching::at(&mut build, DVec3::new(STOCK_RADIUS, 0.0, 0.0));
    let outer = build.cylinder(DVec3::ZERO, DVec3::Z, STOCK_RADIUS);
    let center = DVec3::new(STOCK_RADIUS - HOLE_RADIUS, 0.0, 0.0);
    let inner = build.cylinder(center, DVec3::Z, HOLE_RADIUS);
    let (outer_low, outer_high) = touching.wall(&mut build, outer, false);
    let (inner_low, inner_high) = touching.wall(&mut build, inner, true);
    let ((top, top_flipped), (bottom, bottom_flipped)) = caps(&mut build);
    let use_of = |edge, forward| Coedge { edge, forward };
    let top_lap = vec![use_of(outer_high, true), use_of(inner_high, false)];
    build.face(top, top_flipped, vec![top_lap]);
    let bottom_lap = vec![use_of(outer_low, false), use_of(inner_low, true)];
    build.face(bottom, bottom_flipped, vec![bottom_lap]);
    build.finish(STOCK_RADIUS)
}

/// How deep the pocket tangent to the stock's wall goes from the top.
pub(crate) const POCKET_DEPTH: f64 = 5.0;

/// The stock with a pocket sunk from its top at (15, 0), its wall touching the
/// stock's inside along a line that stops at the pocket's floor: the stock's
/// wall, which goes all the way round, has that line hanging from its top as
/// a slit.
pub(crate) fn stock_with_a_tangent_pocket() -> Body {
    let mut build = Build::new();
    let floor = HEIGHT - POCKET_DEPTH;
    let touching = Touching::down_to(&mut build, DVec3::new(STOCK_RADIUS, 0.0, 0.0), floor);
    let outer = build.cylinder(DVec3::ZERO, DVec3::Z, STOCK_RADIUS);
    let ring = build.circle(outer, 0.0, None);
    let outer_high = build.circle(outer, HEIGHT, Some(touching.high));
    let use_of = |edge, forward| Coedge { edge, forward };
    let (up, down) = (
        build.leaving(touching.line, touching.low),
        build.leaving(touching.line, touching.high),
    );
    let hanging = vec![use_of(outer_high, false), down, up];
    build.face(outer, false, vec![vec![use_of(ring, true)], hanging]);
    let center = DVec3::new(STOCK_RADIUS - HOLE_RADIUS, 0.0, 0.0);
    let inner = build.cylinder(center, DVec3::Z, HOLE_RADIUS);
    let (inner_low, inner_high) = touching.wall(&mut build, inner, true);
    let ((top, top_flipped), (bottom, bottom_flipped)) = caps(&mut build);
    let top_lap = vec![use_of(outer_high, true), use_of(inner_high, false)];
    build.face(top, top_flipped, vec![top_lap]);
    build.face(bottom, bottom_flipped, vec![vec![use_of(ring, false)]]);
    let (sunk, sunk_flipped) = build.plane(DVec3::Z * floor, DVec3::Z);
    build.face(sunk, sunk_flipped, vec![vec![use_of(inner_low, true)]]);
    build.finish(STOCK_RADIUS)
}

/// The block with two holes whose circles touch at the origin: the line
/// through it has four uses, one wall on each side of it.
pub(crate) fn block_with_two_touching_holes() -> Body {
    let mut build = Build::new();
    let (top, bottom) = block_in(&mut build);
    let touching = Touching::at(&mut build, DVec3::ZERO);
    let use_of = |edge, forward| Coedge { edge, forward };
    for side in [-1.0, 1.0] {
        let center = DVec3::new(side * HOLE_RADIUS, 0.0, 0.0);
        let wall = build.cylinder(center, DVec3::Z, HOLE_RADIUS);
        let (low, high) = touching.wall(&mut build, wall, true);
        build.add_loop(top, vec![use_of(high, false)]);
        build.add_loop(bottom, vec![use_of(low, true)]);
    }
    build.finish(HALF_SIDE)
}

/// The radius and the length of the cylinder lying on the block.
pub(crate) const LYING_RADIUS: f64 = 4.0;
pub(crate) const LYING_LENGTH: f64 = 20.0;

/// The block with a cylinder lying along X on its top, touching it along a
/// line: the top face has a slit, the line used by it both ways.
pub(crate) fn block_with_a_lying_cylinder() -> Body {
    let mut build = Build::new();
    let (top, _) = block_in(&mut build);
    let half = LYING_LENGTH / 2.0;
    let near = build.vertex(DVec3::new(-half, 0.0, HEIGHT));
    let far = build.vertex(DVec3::new(half, 0.0, HEIGHT));
    let line = build.line(near, far);
    let axis = DVec3::new(0.0, 0.0, HEIGHT + LYING_RADIUS);
    let wall = build.cylinder(axis, DVec3::X, LYING_RADIUS);
    let low = build.circle(wall, -half, Some(near));
    let high = build.circle(wall, half, Some(far));
    let use_of = |edge, forward| Coedge { edge, forward };
    let (along, back) = (build.leaving(line, near), build.leaving(line, far));
    build.face(
        wall,
        false,
        vec![vec![use_of(low, true), along, use_of(high, false), back]],
    );
    let (near_cap, near_flipped) = build.plane(DVec3::new(-half, 0.0, 0.0), DVec3::NEG_X);
    build.face(near_cap, near_flipped, vec![vec![use_of(low, false)]]);
    let (far_cap, far_flipped) = build.plane(DVec3::new(half, 0.0, 0.0), DVec3::X);
    build.face(far_cap, far_flipped, vec![vec![use_of(high, true)]]);
    build.add_loop(top, vec![along, back]);
    build.finish(HALF_SIDE)
}

/// The volumes the fixtures enclose, by arithmetic.
pub(crate) fn block_volume() -> f64 {
    (2.0 * HALF_SIDE).powi(2) * HEIGHT
}

pub(crate) fn disc_volume(radius: f64, length: f64) -> f64 {
    PI * radius * radius * length
}
