//! What a ray meets of the part as drawn, read on the part itself. What is
//! drawn may be the part shown with its volumes' states, cut by a section or
//! a sketch, and the ghosts of the volumes shown see-through, and the numbers
//! its faces carry are not relied on. A flat face — the same plane however it
//! is cut into triangles — is found on the part by where the ray meets it, and
//! one the part has nothing of lets the ray go on. A curved face stops the
//! ray where it is met.

use cao_solid::{Body, FaceHit};
use glam::DVec3;

use super::{Drawn, drawn_under};

/// How many faces a ray goes through before it gives up looking.
const FACES_GONE_THROUGH: usize = 64;

/// What the ray meets: the part's own face — met where a drawn flat face lies
/// on it, its distance from the ray's origin — or a drawn flat face the part
/// has nothing of, where the caller asked the ray to stop, or a curved face.
pub(crate) enum Met<'a> {
    /// The part's own face, and the piece and face drawn on it.
    Part {
        own: FaceHit,
        piece: &'a Body,
        drawn: FaceHit,
    },
    Drawn(&'a Body, FaceHit),
    /// A curved face drawn, where the ray stops whatever the part has there:
    /// its triangles stand wherever its piece's cutting put them, so where it
    /// is met says nothing of the part's own face.
    Curved(&'a Body, FaceHit),
}

/// How far apart two places may stand and still be one place on the part,
/// for a part of its size: well past the hundred-thousandth a ray travels
/// before it may meet a triangle, and well short of any face's width.
pub(crate) fn nearness(part: &Body) -> f64 {
    let reach = part
        .bounds()
        .map_or(1.0, |(low, high)| (high - low).length());
    reach.max(10.0) * 1e-5
}

/// The first place along the ray where what is drawn lies on a face of the
/// part, or where `stops`, told where and facing which way, says the ray
/// stops on a drawn face the part has nothing of.
pub(crate) fn met_on_the_part<'a>(
    drawn: &'a Drawn<'_>,
    part: &Body,
    (origin, direction): (DVec3, DVec3),
    stops: impl Fn(DVec3, DVec3) -> bool,
) -> Option<Met<'a>> {
    let near = nearness(part);
    let mut travelled = 0.0;
    for _ in 0..FACES_GONE_THROUGH {
        let from = origin + direction * travelled;
        let (piece, mut hit) = drawn_under(drawn, from, direction)?;
        hit.distance += travelled;
        if !piece.is_flat(hit.face) {
            return Some(Met::Curved(piece, hit));
        }
        let at = origin + direction * hit.distance;
        if let Some(mut own) = the_part_s_face_at(part, at, hit.normal, near) {
            own.distance = hit.distance;
            return Some(Met::Part {
                own,
                piece,
                drawn: hit,
            });
        }
        if stops(at, hit.normal) {
            return Some(Met::Drawn(piece, hit));
        }
        travelled = hit.distance + 10.0 * near;
    }
    None
}

/// The part's own face lying at `at`, square to `normal` one way or the
/// other: the face a ray from a hair off it, back along the normal, meets
/// right there.
fn the_part_s_face_at(part: &Body, at: DVec3, normal: DVec3, near: f64) -> Option<FaceHit> {
    let hit = part.ray_hit(at + normal * near, -normal)?;
    let there = (hit.distance - near).abs() <= near / 2.0;
    let square = hit.normal.dot(normal).abs() > 1.0 - 1e-6;
    (there && square).then_some(hit)
}
