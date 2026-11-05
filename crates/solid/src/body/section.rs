//! What is left of a body on one side of a plane: an exact body cut by the
//! exact kernel and closed by a true face on the plane, flats clipped.

use glam::{DVec2, DVec3};

use super::drawn::DRAWN;
use super::exact::Exact;
use super::{Body, Matter};
use crate::profile::{Contour, Frame};

impl Body {
    /// What is left of the body behind a plane, `normal · p ≤ offset`, a face
    /// lying on the plane with it: a way of looking inside rather than of
    /// taking matter away. An exact body is closed by a face on the plane;
    /// flats, and an exact body whose cut the kernel declines, whose own drawn
    /// triangles are clipped, are left open where the plane went through.
    pub fn behind(&self, normal: DVec3, offset: f64) -> Body {
        match &self.matter {
            Matter::Exact(exact) => exact.behind(normal, offset).map_or_else(
                || {
                    let clipped = exact.drawn().behind(exact.brep(), normal, offset);
                    Body::of_flats(clipped, self.faces_end())
                },
                Body::of_exact,
            ),
            Matter::Flats(flats) => Body::of_flats(flats.mesh.behind(normal, offset), flats.next),
        }
    }

    /// What stands strictly in front of a plane, closed as [`Body::behind`]
    /// closes it. An exact body is cut a drawing tolerance off the plane, so
    /// that none of it lies on a sketch drawn there.
    pub fn in_front(&self, normal: DVec3, offset: f64) -> Body {
        match &self.matter {
            Matter::Exact(exact) => self.behind(-normal, -offset - DRAWN * exact.reach()),
            Matter::Flats(flats) => Body::of_flats(flats.mesh.parted(normal, offset).1, flats.next),
        }
    }

    /// The body parted by a plane: what [`Body::behind`] leaves, and what
    /// [`Body::in_front`] does.
    pub fn parted(&self, normal: DVec3, offset: f64) -> (Body, Body) {
        (self.behind(normal, offset), self.in_front(normal, offset))
    }
}

impl Exact {
    /// The body cut by a slab standing on the plane in front of it, sized to
    /// the body: a box much wider would coarsen the kernel's tolerance, which
    /// follows the larger reach of the two. `None` where the kernel declines
    /// the cut, or cannot read the body's box.
    fn behind(&self, normal: DVec3, offset: f64) -> Option<Exact> {
        let (low, high) = self.bounds()?;
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
            return Some(self.clone());
        }
        let margin = (high - low).length() / 10.0;
        let outline = Contour::rectangle(
            DVec2::new(near.x, near.y) - margin,
            DVec2::new(far.x, far.y) + margin,
        );
        let frame = Frame { origin, u, v };
        let slab = Exact::raised(&outline, &[], frame, normal * (far.z + margin)).ok()?;
        self.cut_by(&slab).ok()
    }
}
