//! A cut that stops flush (#801): where matter lies past an end of its edge —
//! the inner corner of an L, a pocket's rim, the foot of a block standing on
//! another — the cut's prism, carried past the end along the edge's line as
//! #699 carries it past a block's corner, would take that matter. It is
//! trimmed flush on the plane of the face the edge runs into there instead,
//! as a fill's is (#778), and that face grows by the cut's end section,
//! keeping its number.

use glam::DVec3;

use super::fill::{beyond, end_face};
use super::{Prism, Wedge};
use crate::body::exact::Exact;
use crate::brep::{self, Declined, EdgeId, FaceId, Plane, Surface};

impl Exact {
    /// The body with the edge of `wedge`, `edge` on this body, cut off by
    /// `tool`, the prism raised from `prism`, its first wall the cut's,
    /// numbered nought and counted as raised, so that the boxes its ends are
    /// trimmed by number their faces past its own. At each end where the one
    /// face met there but the edge's two is flat and faces back along the
    /// edge, matter lying past it, the prism is trimmed flush on that face's
    /// plane, the trimmed end merged into that face, as [`Exact::merged_at`]
    /// merges it at the corner the edge ended at; at any other end it is
    /// carried past, as at a block's corner. Declined as the trim and the cut
    /// decline.
    pub(super) fn cut_flush(
        &self,
        edge: EdgeId,
        wedge: &Wedge,
        prism: &Prism,
        tool: Exact,
    ) -> Result<Exact, Declined> {
        let ends = self.brep.edge(edge).ends.into_iter().flatten();
        let mut trimmed = tool;
        let mut corners = Vec::new();
        for (end, past) in ends.zip([-wedge.along, wedge.along]) {
            let behind = end_face(&self.brep, end, wedge.faces)
                .and_then(|face| facing_back(&self.brep, face, past));
            let Some((plane, outward)) = behind else {
                continue;
            };
            trimmed = trimmed.cut_by(&beyond(prism, plane, -outward)?)?;
            corners.push(self.brep.vertex(end).point);
        }
        let cut = self.cut_by(&Exact { next: 1, ..trimmed })?;
        if corners.is_empty() {
            Ok(cut)
        } else {
            cut.merged_at(&corners)
        }
    }
}

/// The plane of the face `id` and the way it faces out of the matter, where
/// it is flat and faces back against `past`, the way the edge runs on past
/// its end: matter lies past that end. None on any other face.
fn facing_back(body: &brep::Body, id: FaceId, past: DVec3) -> Option<(&Plane, DVec3)> {
    let face = body.face(id);
    let Surface::Plane(plane) = body.surface(face.surface) else {
        return None;
    };
    let outward = if face.flipped {
        -plane.normal
    } else {
        plane.normal
    };
    (outward.dot(past) < 0.0).then_some((plane, outward))
}
