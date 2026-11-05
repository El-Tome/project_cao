//! What is left of a body on one side of a plane: an exact body cut by the
//! exact kernel and closed by a true face on the plane, flats clipped.

use glam::{DVec2, DVec3};

use super::drawn::DRAWN;
use super::exact::Exact;
use super::{Body, Matter};
use crate::brep::Declined;
use crate::profile::{Contour, Frame};

impl Body {
    /// What is left of the body behind a plane, `normal · p < offset`: a way
    /// of looking inside rather than of taking matter away. An exact body is
    /// closed by a face on the plane; flats, and an exact body whose cut the
    /// kernel declines, whose own drawn triangles are clipped, are left open
    /// where the plane went through.
    pub fn behind(&self, normal: DVec3, offset: f64) -> Body {
        match &self.matter {
            Matter::Exact(exact) => exact.behind(normal, offset).map_or_else(
                |_| {
                    let clipped = exact.drawn().behind(exact.brep(), normal, offset);
                    Body::of_flats(clipped, self.faces_end())
                },
                Body::of_exact,
            ),
            Matter::Flats(flats) => Body::of_flats(flats.mesh.behind(normal, offset), flats.next),
        }
    }

    /// The body parted by a plane: what [`Body::behind`] leaves, and what
    /// stands strictly in front, each closed as `behind` closes it. A face
    /// lying on the plane goes behind; an exact body's front is cut a drawing
    /// tolerance off the plane, so that none of it lies on a sketch drawn there.
    pub fn parted(&self, normal: DVec3, offset: f64) -> (Body, Body) {
        match &self.matter {
            Matter::Exact(exact) => {
                let gap = DRAWN * exact.reach();
                (
                    self.behind(normal, offset),
                    self.behind(-normal, -offset - gap),
                )
            }
            Matter::Flats(flats) => {
                let (behind, in_front) = flats.mesh.parted(normal, offset);
                (
                    Body::of_flats(behind, flats.next),
                    Body::of_flats(in_front, flats.next),
                )
            }
        }
    }
}

impl Exact {
    /// The body cut by a slab standing on the plane in front of it, sized to
    /// the body: a box much wider would coarsen the kernel's tolerance, which
    /// follows the larger reach of the two.
    fn behind(&self, normal: DVec3, offset: f64) -> Result<Exact, Declined> {
        let Some((low, high)) = self.bounds() else {
            return Ok(self.clone());
        };
        let centre = (low + high) / 2.0;
        let origin = centre - normal * (centre.dot(normal) - offset);
        let u = normal.any_orthonormal_vector();
        let v = normal.cross(u);
        let corners = (0..8).map(|corner| {
            let at = DVec3::select(
                glam::BVec3::new(corner & 1 != 0, corner & 2 != 0, corner & 4 != 0),
                high,
                low,
            ) - origin;
            DVec3::new(at.dot(u), at.dot(v), at.dot(normal))
        });
        let (near, far) = corners.fold(
            (DVec3::splat(f64::INFINITY), DVec3::splat(f64::NEG_INFINITY)),
            |(near, far), at| (near.min(at), far.max(at)),
        );
        if far.z <= 0.0 {
            return Ok(self.clone());
        }
        let margin = (high - low).length() / 10.0;
        let outline = Contour::rectangle(
            DVec2::new(near.x, near.y) - margin,
            DVec2::new(far.x, far.y) + margin,
        );
        let frame = Frame { origin, u, v };
        let slab = Exact::raised(&outline, &[], frame, normal * (far.z + margin))?;
        self.cut_by(&slab)
    }
}
