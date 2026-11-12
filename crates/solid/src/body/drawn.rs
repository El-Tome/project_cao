//! The triangles an exact body is drawn with, each knowing the face it was cut
//! from: what the view draws, lights and picks in, and what the body becomes
//! when it has to be joined to flats.

use glam::DVec3;

use super::FaceHit;
use crate::brep::{self, FaceId, Scale};
use crate::mesh::{Mesh, ray_triangle};

/// How far the triangles may stand from the exact surfaces, as a share of the
/// body's reach.
///
/// The flats cut every circle into 48 steps whatever its size, which stands
/// a chord 0.21 % of the radius inside it. A fifth of a thousandth of the
/// reach cuts a circle a tenth as wide as the part into 52 steps and one as
/// wide as the part into 160: no circle of a tenth of the part or more is
/// drawn coarser than the flats drew it; a smaller one, which the view shows
/// small, may be. The eighteen cases of `a_bored_cylinder_on_two_kernels`
/// are drawn this fine closed and uncrossed, each in a millisecond or two.
pub(super) const DRAWN: f64 = 2e-4;

/// How far the triangles of `body` may stand from its exact surfaces.
pub(super) fn tolerance(body: &brep::Body) -> f64 {
    DRAWN * reach(body)
}

/// How far `body` itself reaches from the origin, as [`Scale`] reads a reach,
/// and never further than its scale's: what its triangles' tolerance is a
/// share of. The scale of a body cut by a tool wider than itself carries the
/// tool's reach, so that a later operation never decides finer than that
/// cut did (#824); the tool is gone, and the part is drawn at its own size.
fn reach(body: &brep::Body) -> f64 {
    let scale = body.scale().reach();
    body.bounds().map_or(scale, |(low, high)| {
        let own = Scale::of(low.abs().max(high.abs()).max_element());
        own.reach().min(scale)
    })
}

#[derive(Clone, Debug, Default)]
pub(super) struct Drawn {
    pub triangles: Vec<[DVec3; 3]>,
    faces: Vec<FaceId>,
    whole: bool,
}

impl Drawn {
    /// The triangles of a body, each beside the face it was cut from.
    pub fn of(body: &brep::Body) -> Drawn {
        let cut = body.triangles_by_face(tolerance(body));
        Drawn {
            triangles: cut.triangles,
            faces: cut.faces,
            whole: cut.uncut.is_empty(),
        }
    }

    /// Whether every face of the body could be cut into triangles, which
    /// leaves the drawing closed.
    pub fn is_whole(&self) -> bool {
        self.whole
    }

    /// The nearest triangle a ray crosses names the face; the face's own
    /// surface, not the triangle, says which way it faces there.
    pub fn ray_hit(&self, body: &brep::Body, origin: DVec3, direction: DVec3) -> Option<FaceHit> {
        let (distance, id) = self
            .triangles
            .iter()
            .zip(&self.faces)
            .filter_map(|(triangle, id)| Some((ray_triangle(origin, direction, *triangle)?, *id)))
            .min_by(|(near, _), (far, _)| near.total_cmp(far))?;
        let face = body.face(id);
        let surface = body.surface(face.surface);
        let normal = surface.normal(surface.parameters(origin + direction * distance));
        Some(FaceHit {
            distance,
            face: *face.numbers.first()? as usize,
            normal: if face.flipped { -normal } else { normal },
        })
    }

    pub fn of_face<'a>(
        &'a self,
        body: &'a brep::Body,
        number: usize,
    ) -> impl Iterator<Item = [DVec3; 3]> + 'a {
        self.triangles
            .iter()
            .zip(&self.faces)
            .filter(move |(_, id)| body.numbers(**id).iter().any(|&n| n as usize == number))
            .map(|(triangle, _)| *triangle)
    }

    /// The triangles as flat pieces, each on the first number of its face.
    pub fn flats(&self, body: &brep::Body) -> Mesh {
        Mesh::of_triangles(self.numbered(body))
    }

    /// The triangles clipped to `normal · p ≤ offset` as flat pieces, each on
    /// the first number of its face, left open where the plane went through.
    pub fn behind(&self, body: &brep::Body, normal: DVec3, offset: f64) -> Mesh {
        Mesh::of_triangles(self.numbered(body).flat_map(|(triangle, face)| {
            clipped(triangle, normal, offset)
                .into_iter()
                .map(move |piece| (piece, face))
        }))
    }

    fn numbered<'a>(
        &'a self,
        body: &'a brep::Body,
    ) -> impl Iterator<Item = ([DVec3; 3], usize)> + 'a {
        self.triangles
            .iter()
            .zip(&self.faces)
            .filter_map(|(triangle, id)| Some((*triangle, *body.numbers(*id).first()? as usize)))
    }
}

/// What of a triangle lies on `normal · p ≤ offset`, cut into triangles
/// turned as it is.
fn clipped(corners: [DVec3; 3], normal: DVec3, offset: f64) -> Vec<[DVec3; 3]> {
    let mut kept = Vec::with_capacity(4);
    for index in 0..3 {
        let (from, to) = (corners[index], corners[(index + 1) % 3]);
        let (here, there) = (from.dot(normal) - offset, to.dot(normal) - offset);
        if here <= 0.0 {
            kept.push(from);
        }
        if (here < 0.0 && there > 0.0) || (here > 0.0 && there < 0.0) {
            kept.push(from + (to - from) * (here / (here - there)));
        }
    }
    (1..kept.len().saturating_sub(1))
        .map(|index| [kept[0], kept[index], kept[index + 1]])
        .collect()
}
