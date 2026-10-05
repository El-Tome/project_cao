//! The triangles an exact body is drawn with, each knowing the face it was cut
//! from: what the view draws, lights and picks in, and what the body becomes
//! when it has to be joined to flats.

use glam::DVec3;

use super::FaceHit;
use crate::brep::{self, FaceId};
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

#[derive(Clone, Debug, Default)]
pub(super) struct Drawn {
    pub triangles: Vec<[DVec3; 3]>,
    faces: Vec<FaceId>,
    whole: bool,
}

impl Drawn {
    /// The triangles of a body, each beside the face it was cut from.
    pub fn of(body: &brep::Body) -> Drawn {
        let cut = body.triangles_by_face(DRAWN * body.scale().reach());
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
        Mesh::of_triangles(
            self.triangles
                .iter()
                .zip(&self.faces)
                .filter_map(|(triangle, id)| {
                    Some((*triangle, *body.numbers(*id).first()? as usize))
                }),
        )
    }
}
