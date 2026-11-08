//! The join a fill makes (#778): the prism a concave edge is filled with,
//! raised along it as a cut's is, trimmed flush at each end by the plane of
//! the face the edge ends on there, and joined to the body, its ends merged
//! into those faces and the edges either side of the edge's ends made one
//! where they run on in line.

use glam::{DVec2, DVec3};

use super::{Prism, Wedge};
use crate::body::drawn::Drawn;
use crate::body::exact::{Exact, PIECES};
use crate::brep::{self, Declined, EdgeId, FaceId, Plane, Surface, VertexId};
use crate::profile::{Contour, Frame};

/// How far the box a prism's end is trimmed by stands clear of the prism,
/// past it and round it, as a share of how far the prism's section reaches
/// from the edge: far enough that no face of the box lies nearly flush with
/// one of the prism's.
const CLEAR: f64 = 0.125;

impl Exact {
    /// The body with the corner along each of `edges` filled in by `tool`,
    /// the prism raised from `prism` — its first walls, one for each edge,
    /// numbered from nought and counted as raised, so that the boxes its
    /// ends are trimmed by number their faces past its own: each end of it
    /// trimmed flush by the plane of the face the first edge ends on there,
    /// and joined to the body, every face of the prism that merged into one
    /// of the body's answering to the body's numbers alone, so that the face
    /// it merged into stays the face it was: its ends, merged into the faces
    /// at the edges' ends, and a wall of it lying flush on a face of the
    /// body, as the wall squared off a face taken whole lies on the face
    /// beyond. A face of the prism standing alone keeps its number, past the
    /// steps' count, and that alone is what [`Exact::cut_along`] finds
    /// stray. Where the face at an end meets one of an edge's two along a
    /// line running on past the edge's end — the plate's top round a rib
    /// ending short of its edge — the two edges either side of that corner
    /// are made one, since the fill covered what of the edge's faces reached
    /// it. Declined `Unsupported`: an end where no face but the edge's two
    /// meets it, or several do, or one that is not flat, or faces back along
    /// the edge, or leans so far that its plane passes the prism's own ends,
    /// and another edge that does not end on the faces the first ends on.
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
            let (face, plane, outward) =
                end_plane(&self.brep, end, wedge.faces).ok_or(Declined::Unsupported)?;
            if outward.dot(past) <= 0.0 {
                return Err(Declined::Unsupported);
            }
            trimmed = trimmed.cut_by(&beyond(prism, &plane, outward)?)?;
            end_faces.push(face);
        }
        let mut corners = Vec::new();
        for &(edge, wedge) in edges {
            let ends = self.brep.edge(edge).ends.ok_or(Declined::Unsupported)?;
            for end in ends {
                let (face, ..) =
                    end_plane(&self.brep, end, wedge.faces).ok_or(Declined::Unsupported)?;
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

/// The one face, but the edge's two, at a corner of the body, its plane, and
/// the way it faces out of the matter; none where no other face or several
/// meet there, or that face is not flat.
fn end_plane(
    body: &brep::Body,
    corner: VertexId,
    faces: [FaceId; 2],
) -> Option<(FaceId, Plane, DVec3)> {
    let mut met = body.face_ids().filter(|face| {
        !faces.contains(face)
            && body.face(*face).loops.iter().flatten().any(|coedge| {
                let ends = body.edge(coedge.edge).ends;
                ends.is_some_and(|ends| ends.contains(&corner))
            })
    });
    let (Some(id), None) = (met.next(), met.next()) else {
        return None;
    };
    let face = body.face(id);
    let Surface::Plane(plane) = *body.surface(face.surface) else {
        return None;
    };
    let outward = if face.flipped {
        -plane.normal
    } else {
        plane.normal
    };
    Some((id, plane, outward))
}

/// The box standing on `plane` that holds what of the prism stands past it,
/// along `outward`, and a share [`CLEAR`] of the section's reach round that:
/// raised off the plane over the points where the prism's long edges cross
/// it and its end beyond. Declined `Unsupported` where the plane misses one
/// of those edges between the prism's two ends; a box the kernel cannot
/// raise, as the raise says.
fn beyond(
    (outline, frame, travel): &Prism,
    plane: &Plane,
    outward: DVec3,
) -> Result<Exact, Declined> {
    let offset = outward.dot(plane.origin);
    let rising = outward.dot(*travel);
    let mut reach: f64 = 0.0;
    let mut held = Vec::new();
    let mut depth: f64 = 0.0;
    for corner in &outline.corners {
        let start = frame.origin + frame.u * corner.x + frame.v * corner.y;
        let crossing = (offset - outward.dot(start)) / rising;
        if !(crossing > 0.0 && crossing < 1.0) {
            return Err(Declined::Unsupported);
        }
        let far = if rising > 0.0 { start + *travel } else { start };
        held.extend([start + *travel * crossing, far]);
        depth = depth.max(outward.dot(far) - offset);
        reach = reach.max(corner.length());
    }
    let clear = reach * CLEAR;
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
