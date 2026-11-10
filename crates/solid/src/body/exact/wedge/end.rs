//! What a prism raised along an edge meets at the edge's ends, and how it is
//! trimmed and merged there — what a fill (#778) and a cut that stops flush
//! (#801) share: the one face the edge runs into at an end, the box standing
//! on a plane that holds what of the prism stands past it, and the body the
//! prism was combined with, every face of the prism that merged into one of
//! the body's answering to the body's number.

use glam::{DVec2, DVec3};

use super::Prism;
use crate::body::drawn::Drawn;
use crate::body::exact::{Exact, PIECES};
use crate::brep::{self, Declined, FaceId, Plane, VertexId};
use crate::profile::{Contour, Frame};

/// How far the solid a prism's end is trimmed by stands clear of the prism,
/// past it and round it, as a share of how far the prism's section reaches
/// from the edge: far enough that no face of the solid lies nearly flush
/// with one of the prism's.
pub(super) const CLEAR: f64 = 0.125;

impl Exact {
    /// The body a step combined with its prism, every face of the prism that
    /// merged into one of the body's — whatever face of the prism it is —
    /// answering to the body's numbers alone, a face of the prism standing
    /// alone keeping its own, past the step's count, and the edges either side
    /// of each of `corners` made one where they run on in line, drawn again
    /// then.
    pub(super) fn merged_at(self, corners: &[DVec3]) -> Result<Exact, Declined> {
        let Exact {
            mut brep,
            next,
            pieces,
            roots,
            drawn,
            ..
        } = self;
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
pub(super) fn end_face(body: &brep::Body, corner: VertexId, faces: [FaceId; 2]) -> Option<FaceId> {
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
pub(super) fn beyond(prism: &Prism, plane: &Plane, outward: DVec3) -> Result<Exact, Declined> {
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
