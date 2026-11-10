//! A cut that stops flush (#801): where matter lies past an end of its edge —
//! the inner corner of an L, a pocket's rim, the foot of a block standing on
//! another — the cut's prism, carried past the end along the edge's line as
//! #699 carries it past a block's corner, would take that matter. It is
//! trimmed flush on the plane of the face the edge runs into there instead,
//! as a fill's is (#778), and that face grows by the cut's end section,
//! keeping its number.

use glam::DVec3;

use super::fill::{beyond, end_face};
use super::overlap::Overlap;
use super::{Prism, Wedge};
use crate::body::exact::Exact;
use crate::brep::{self, Declined, EdgeId, FaceId, Plane, Surface};

/// An end of an edge where matter lies past it: the plane of the face the
/// edge runs into there, the way that face faces out of the matter, and the
/// corner the edge ends at.
pub(super) struct Stop {
    pub plane: Plane,
    pub outward: DVec3,
    pub corner: DVec3,
}

impl Stop {
    /// The box standing on the stop's plane that holds what of the prism
    /// stands past it, in the matter, as [`beyond`] raises it.
    pub fn beyond(&self, prism: &Prism) -> Result<Exact, Declined> {
        beyond(prism, &self.plane, -self.outward)
    }

    /// Where the line through `from` along `along`, the edge's way, crosses
    /// the stop's plane.
    pub fn crossing(&self, from: DVec3, along: DVec3) -> DVec3 {
        from + along * (self.outward.dot(self.corner - from) / self.outward.dot(along))
    }
}

/// Where a cut stops flush: the ends of its edge it stops at, and what it
/// takes past those it shares with an edge laid before it beside a concave
/// edge.
#[derive(Default)]
pub(super) struct Flush {
    pub stops: Vec<Stop>,
    pub overlaps: Vec<Overlap>,
}

impl Flush {
    /// The edge `edge` of `body`, read there as `wedge`, stopping at each end
    /// [`stops`] reads, and taking no overlap.
    pub fn at(body: &brep::Body, edge: EdgeId, wedge: &Wedge) -> Flush {
        Flush {
            stops: stops(body, edge, wedge),
            overlaps: Vec::new(),
        }
    }
}

/// The ends of `edge` on `body`, read there as `wedge`, where the one face met
/// but the edge's two is flat and faces back along the edge, matter lying past
/// it. Any other end, where air lies past or that face is curved or is not
/// one, the cut is carried past, as at a block's corner.
pub(super) fn stops(body: &brep::Body, edge: EdgeId, wedge: &Wedge) -> Vec<Stop> {
    let ends = body.edge(edge).ends.into_iter().flatten();
    ends.zip([-wedge.along, wedge.along])
        .filter_map(|(end, past)| {
            let face = end_face(body, end, wedge.faces)?;
            let (plane, outward) = facing_back(body, face, past)?;
            let corner = body.vertex(end).point;
            Some(Stop {
                plane: *plane,
                outward,
                corner,
            })
        })
        .collect()
}

impl Exact {
    /// The body cut by `tool`, the prism raised from `prism`, its first wall
    /// the cut's, numbered nought and counted as raised, so that the boxes its
    /// ends are trimmed by and the overlaps it takes number their faces past
    /// its own: trimmed flush at each stop of `flush` on its plane, joined to
    /// each overlap, the trimmed ends and the overlaps merged into the faces
    /// they meet, as [`Exact::merged_at`] merges them at the corners the edge
    /// ended at and the points the overlaps clear, and carried past any other
    /// end. Declined as the trim, the join and the cut decline.
    pub(super) fn cut_flush(
        &self,
        flush: &Flush,
        prism: &Prism,
        tool: Exact,
    ) -> Result<Exact, Declined> {
        let mut trimmed = tool;
        for stop in &flush.stops {
            trimmed = trimmed.cut_by(&stop.beyond(prism)?)?;
        }
        for overlap in &flush.overlaps {
            trimmed = trimmed.joined(&overlap.solid)?;
        }
        let cut = self.cut_by(&Exact { next: 1, ..trimmed })?;
        if flush.stops.is_empty() {
            return Ok(cut);
        }
        let corners: Vec<DVec3> = (flush.stops.iter().map(|stop| stop.corner))
            .chain(
                flush
                    .overlaps
                    .iter()
                    .flat_map(|overlap| overlap.cleared.clone()),
            )
            .collect();
        cut.merged_at(&corners)
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
