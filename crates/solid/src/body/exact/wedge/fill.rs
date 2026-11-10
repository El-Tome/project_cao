//! The join a fill makes (#778): the prism a concave edge is filled with,
//! raised along it as a cut's is, trimmed flush at each end on the surface of
//! the face the edge ends on there — a plane, or a round (#788) — and joined
//! to the body, its ends merged into those faces and the edges either side of
//! the edge's ends made one where they run on along one curve.

mod round;

use glam::DVec3;

use super::end::{beyond, end_face};
use super::{Prism, Wedge};
use crate::body::exact::Exact;
use crate::brep::{self, Declined, EdgeId, FaceId, Surface};

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
        self.joined(&Exact { next, ..trimmed })?.merged_at(&corners)
    }
}

/// The solid to cut from the prism so that it ends flush on the surface of
/// the face `id` its edge ends on at `point`, running on `past` there: on a
/// plane facing out of the matter along `past`, the box [`beyond`] raises;
/// on a cylinder or a cone the edge runs out of at `point`, whichever side
/// of it the matter lies — a rib's round end bulging out or drawn in by
/// draft, or a slot's round end turning in toward the slot off its wall,
/// which the fill then runs on into — what [`round::outside_cylinder`]
/// raises or [`round::outside_cone`] turns. Declined `Unsupported` on a
/// plane turned back along the edge, a cylinder or a cone the edge runs
/// into, or another surface; otherwise as those declines.
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
    match surface {
        Surface::Plane(plane) if outward.dot(past) > 0.0 => beyond(prism, plane, outward),
        Surface::Cylinder(cylinder) if normal.dot(past) > 0.0 => {
            round::outside_cylinder(prism, cylinder, past)
        }
        Surface::Cone(cone) => round::outside_cone(prism, cone, point, past, body.scale().reach()),
        _ => Err(Declined::Unsupported),
    }
}
