//! The join a fill makes (#778): the prism a concave edge is filled with,
//! raised along it as a cut's is, trimmed flush at each end on the surface of
//! the face the edge ends on there — a plane, or a round (#788) — and joined
//! to the body, its ends merged into those faces and the edges either side of
//! the edge's ends made one where they run on along one curve.

use std::f64::consts::PI;

use glam::{DVec2, DVec3};

use super::{Prism, Wedge};
use crate::body::drawn::Drawn;
use crate::body::exact::{Exact, PIECES};
use crate::brep::{self, Cylinder, Declined, EdgeId, FaceId, Plane, Surface, VertexId};
use crate::profile::{Contour, Frame, Run};

/// How far the solid a prism's end is trimmed by stands clear of the prism,
/// past it and round it, as a share of how far the prism's section reaches
/// from the edge: far enough that no face of the solid lies nearly flush
/// with one of the prism's.
const CLEAR: f64 = 0.125;

impl Exact {
    /// The body with the corner along each of `edges` filled in by `tool`,
    /// the prism raised from `prism` — its first walls, one for each edge,
    /// numbered from nought and counted as raised, so that the solids its
    /// ends are trimmed by number their faces past its own: each end of it
    /// trimmed flush on the surface of the face the first edge ends on,
    /// and joined to the body, every face of the prism that merged into one
    /// of the body's answering to the body's numbers alone, so that the face
    /// it merged into stays the face it was: its ends, merged into the faces
    /// at the edges' ends, and a wall of it lying flush on a face of the
    /// body, as the wall squared off a face taken whole lies on the face
    /// beyond. A face of the prism standing alone keeps its number, past the
    /// steps' count, and that alone is what [`Exact::cut_along`] finds
    /// stray, a face the solid an end is trimmed by leaves on the fill among
    /// them: the fill standing past a round that turns away short of its
    /// reach. Where the face at an end meets one of an edge's two along a
    /// curve running on past the edge's end — the plate's top round a rib
    /// ending short of its edge, or round its rounded end — the two edges
    /// either side of that corner are made one, since the fill covered what
    /// of the edge's faces reached it. Declined `Unsupported`: an end where
    /// no face but the edge's two meets it, or several do, or one the fill
    /// cannot be trimmed flush on, as [`past_the_end`] declines it, and
    /// another edge that does not end on the faces the first ends on.
    pub(super) fn filled(
        &self,
        edges: &[(EdgeId, &Wedge)],
        prism: &Prism,
        tool: Exact,
    ) -> Result<Exact, Declined> {
        let &[(edge, wedge), ..] = edges else {
            return Err(Declined::Unsupported);
        };
        let ends = self.brep.edge(edge).ends.ok_or(Declined::Unsupported)?;
        let mut trimmed = tool;
        let mut end_faces = Vec::new();
        for (end, past) in ends.into_iter().zip([-wedge.along, wedge.along]) {
            let face = end_face(&self.brep, end, wedge.faces).ok_or(Declined::Unsupported)?;
            let point = self.brep.vertex(end).point;
            trimmed = trimmed.cut_by(&past_the_end(&self.brep, face, point, past, prism)?)?;
            end_faces.push(face);
        }
        let mut corners = Vec::new();
        for &(edge, wedge) in edges {
            let ends = self.brep.edge(edge).ends.ok_or(Declined::Unsupported)?;
            for end in ends {
                let face = end_face(&self.brep, end, wedge.faces).ok_or(Declined::Unsupported)?;
                if !end_faces.contains(&face) {
                    return Err(Declined::Unsupported);
                }
                corners.push(self.brep.vertex(end).point);
            }
        }
        let next = edges.len() as u32;
        self.filled_by(&Exact { next, ..trimmed }, &corners)
    }

    /// The body joined with the prism `tool` fills a corner with, every face
    /// of the prism that merged into one of the body's — whatever face of the
    /// prism it is — answering to the body's numbers alone, a face of the
    /// prism standing alone keeping its own, and the edges either side of
    /// each of `corners` made one where they run on in line, drawn again then.
    fn filled_by(&self, tool: &Exact, corners: &[DVec3]) -> Result<Exact, Declined> {
        let Exact {
            mut brep,
            next,
            pieces,
            roots,
            drawn,
            ..
        } = self.joined(tool)?;
        let brought = |number: &u32| (next..PIECES).contains(number);
        let faces: Vec<_> = brep.face_ids().collect();
        for face in faces {
            let numbers = brep.numbers(face).to_vec();
            let Some(&own) = numbers.iter().find(|number| !brought(number)) else {
                continue;
            };
            for &number in numbers.iter().filter(|number| brought(number)) {
                brep.rename(&[face], number, own);
            }
        }
        let rejoined = brep.rejoined_at(corners)?;
        let drawn = match drawn.into_inner() {
            Some(drawn) if rejoined == brep => drawn,
            _ => Drawn::of(&rejoined),
        };
        if !drawn.is_whole() {
            return Err(Declined::Undrawn);
        }
        let filled = Exact::of(rejoined, next, pieces, roots);
        let _ = filled.drawn.set(drawn);
        Ok(filled)
    }
}

/// The one face, but the edge's two, at a corner of the body; none where no
/// other face or several meet there.
fn end_face(body: &brep::Body, corner: VertexId, faces: [FaceId; 2]) -> Option<FaceId> {
    let mut met = body.face_ids().filter(|face| {
        !faces.contains(face)
            && body.face(*face).loops.iter().flatten().any(|coedge| {
                let ends = body.edge(coedge.edge).ends;
                ends.is_some_and(|ends| ends.contains(&corner))
            })
    });
    match (met.next(), met.next()) {
        (Some(face), None) => Some(face),
        _ => None,
    }
}

/// The solid to cut from the prism so that it ends flush on the surface of
/// the face `id` its edge ends on at `point`, running on `past` there, on
/// the side of it the matter lies: on a plane, the box [`beyond`] raises; on
/// a cylinder the matter lies inside, the piece of a ring [`outside_round`]
/// raises. Declined `Unsupported` where the face does not face out of the
/// matter along `past` at `point` — it turns back along the edge — or is
/// another surface, or a cylinder the matter lies outside; otherwise as the
/// solid's own raise declines.
fn past_the_end(
    body: &brep::Body,
    id: FaceId,
    point: DVec3,
    past: DVec3,
    prism: &Prism,
) -> Result<Exact, Declined> {
    let face = body.face(id);
    let surface = body.surface(face.surface);
    let normal = surface.normal(surface.parameters(point));
    let outward = if face.flipped { -normal } else { normal };
    if outward.dot(past) <= 0.0 {
        return Err(Declined::Unsupported);
    }
    match surface {
        Surface::Plane(plane) => beyond(prism, plane, outward),
        Surface::Cylinder(cylinder) if !face.flipped => outside_round(prism, cylinder, past),
        _ => Err(Declined::Unsupported),
    }
}

/// The points where the prism's long edges cross the plane through `origin`
/// square to `normal`, and those edges' ends past it along `normal`, with
/// how far a solid holding them is to stand clear of them: a share [`CLEAR`]
/// of the section's reach. Declined `Unsupported` where the plane misses one
/// of those edges between the prism's two ends.
fn crossed(
    (outline, frame, travel): &Prism,
    origin: DVec3,
    normal: DVec3,
) -> Result<(Vec<DVec3>, f64), Declined> {
    let offset = normal.dot(origin);
    let rising = normal.dot(*travel);
    let mut reach: f64 = 0.0;
    let mut held = Vec::new();
    for corner in &outline.corners {
        let start = frame.origin + frame.u * corner.x + frame.v * corner.y;
        let crossing = (offset - normal.dot(start)) / rising;
        if !(crossing > 0.0 && crossing < 1.0) {
            return Err(Declined::Unsupported);
        }
        let far = if rising > 0.0 { start + *travel } else { start };
        held.extend([start + *travel * crossing, far]);
        reach = reach.max(corner.length());
    }
    Ok((held, reach * CLEAR))
}

/// The box standing on `plane` that holds what of the prism stands past it,
/// along `outward`, and the clearance [`crossed`] gives round that: raised
/// off the plane over the points where the prism's long edges cross it and
/// its end beyond. Declined as [`crossed`] declines; a box the kernel cannot
/// raise, as the raise says.
fn beyond(prism: &Prism, plane: &Plane, outward: DVec3) -> Result<Exact, Declined> {
    let (held, clear) = crossed(prism, plane.origin, outward)?;
    let depth = (held.iter())
        .map(|&at| outward.dot(at - plane.origin))
        .fold(0.0, f64::max);
    let seen: Vec<DVec2> = held.iter().map(|&at| plane.parameters(at)).collect();
    let low = seen.iter().copied().fold(DVec2::INFINITY, DVec2::min);
    let high = seen.iter().copied().fold(DVec2::NEG_INFINITY, DVec2::max);
    let on_the_plane = Frame {
        origin: plane.origin,
        u: plane.u,
        v: plane.v,
    };
    let rectangle = Contour::rectangle(low - DVec2::splat(clear), high + DVec2::splat(clear));
    Exact::raised(&rectangle, &[], on_the_plane, outward * (depth + clear))
}

/// The piece of a ring about the axis of `cylinder` that holds what of the
/// prism stands outside the cylinder past the plane through the axis square
/// to the way `past` runs off it, and the clearance [`crossed`] gives round
/// that: bounded inside by the cylinder itself, outside by a wider one round
/// the points where the prism's long edges cross that plane and its end
/// beyond, either side by a plane through the axis turned clear of them, or
/// that plane itself, and along the axis clear of them. What of the prism it
/// leaves past the plane stands inside the cylinder. Declined as [`crossed`]
/// declines; a piece the kernel cannot raise, as the raise says.
fn outside_round(prism: &Prism, cylinder: &Cylinder, past: DVec3) -> Result<Exact, Declined> {
    let axis = cylinder.axis;
    let towards = (past - axis * axis.dot(past)).normalize();
    let (held, clear) = crossed(prism, cylinder.origin, towards)?;
    let across = towards.cross(axis);
    let (mut low, mut high, mut outer) = (f64::INFINITY, f64::NEG_INFINITY, cylinder.radius);
    let (mut first, mut last) = (PI, 0.0_f64);
    for at in held {
        let from = at - cylinder.origin;
        let (x, y) = (from.dot(across), from.dot(towards));
        let off = x.hypot(y);
        let turn = (x / off).clamp(-1.0, 1.0).acos();
        let spread = (clear / off).min(1.0).asin();
        first = first.min(turn - spread);
        last = last.max(turn + spread);
        low = low.min(from.dot(axis));
        high = high.max(from.dot(axis));
        outer = outer.max(off);
    }
    let (first, last) = (first.max(0.0), last.min(PI));
    let ray = |turn: f64| DVec2::new(turn.cos(), turn.sin());
    let (inner, outer) = (cylinder.radius, outer + clear);
    let sweep = last - first;
    let piece = Contour {
        corners: vec![
            ray(first) * inner,
            ray(first) * outer,
            ray(last) * outer,
            ray(last) * inner,
        ],
        runs: vec![
            Run::Straight,
            Run::Round {
                center: DVec2::ZERO,
                turn: sweep,
            },
            Run::Straight,
            Run::Round {
                center: DVec2::ZERO,
                turn: -sweep,
            },
        ],
    };
    let frame = Frame {
        origin: cylinder.origin + axis * (low - clear),
        u: across,
        v: towards,
    };
    Exact::raised(&piece, &[], frame, axis * (high - low + 2.0 * clear))
}
