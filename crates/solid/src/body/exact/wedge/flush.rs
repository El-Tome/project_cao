//! A cut that stops flush (#801): where matter lies past an end of its edge —
//! the inner corner of an L, a pocket's rim, the foot of a block standing on
//! another — the cut's prism, carried past the end along the edge's line as
//! #699 carries it past a block's corner, would take that matter. It is
//! trimmed flush on the plane of the face the edge runs into there instead,
//! as a fill's is (#778), and that face grows by the cut's end section,
//! keeping its number. And where an edge's ends are read, a cut's or a
//! fill's: on the body it is laid on, or on the body as given for the later
//! of two edges meeting at a corner beside an edge of the other sense, whose
//! prism takes with it what both prisms hold past that corner.

use glam::DVec3;

use super::corners::Beside;
use super::end::{beyond, end_face};
use super::overlap::{Overlap, overlap};
use super::{Prism, Wedge};
use crate::body::exact::Exact;
use crate::brep::{self, Declined, EdgeId, FaceId, Plane, Scale, Surface, VertexId};

/// An end of an edge where its prism stops flush on a flat face: the plane of
/// the face the edge runs into there, the way from it to the side the prism
/// is trimmed off — into the matter past a cut's end, into the air past a
/// fill's — the corner the edge ends at, and where it stands.
pub(super) struct Stop {
    plane: Plane,
    away: DVec3,
    pub end: VertexId,
    corner: DVec3,
}

impl Stop {
    /// The box standing on the stop's plane that holds what of the prism
    /// stands past it, as [`beyond`] raises it.
    pub fn beyond(&self, prism: &Prism) -> Result<Exact, Declined> {
        beyond(prism, &self.plane, self.away)
    }

    /// Where the line through `from` along `along`, the edge's way, crosses
    /// the stop's plane.
    pub fn crossing(&self, from: DVec3, along: DVec3) -> DVec3 {
        from + along * (self.away.dot(self.corner - from) / self.away.dot(along))
    }

    /// Whether a point lies on the stop's plane, within `eps`.
    pub fn holds(&self, point: DVec3, eps: f64) -> bool {
        self.away.dot(point - self.corner).abs() <= eps
    }
}

/// The stops of the edge of rank `rank` in `edges`, each edge of the step
/// with its wedge on `given`, the body as given, at each of `corners` where it
/// is the earlier of the two edges: there the later edge's overlap completes
/// its face, which stops flush on the plane until then — and, where the edge
/// is shorter than its set-backs reach along it, ends on it in a point off
/// its two faces.
pub(super) fn open(
    given: &brep::Body,
    rank: usize,
    edges: &[(EdgeId, &Wedge)],
    corners: &[Beside],
) -> Vec<Stop> {
    let (edge, wedge) = edges[rank];
    let shared: Vec<VertexId> = (corners.iter())
        .filter(|&&(_, [earlier, _])| earlier == rank)
        .map(|&(corner, _)| corner)
        .collect();
    (stops(given, edge, wedge).into_iter())
        .filter(|stop| shared.contains(&stop.end))
        .collect()
}

/// Where the ends of an edge a step lays are read: `on`, the body it is laid
/// on, or the body as given, for an edge meeting one laid before it at a
/// corner; the edge there; and what its prism takes with it past each corner
/// it shares with an edge laid before it beside an edge of the other sense.
pub(super) struct Flush<'a> {
    on: &'a brep::Body,
    edge: EdgeId,
    overlaps: Vec<Overlap>,
}

impl<'a> Flush<'a> {
    /// The edge `edge` of `on`, its ends read there, taking no overlap.
    pub fn at(on: &'a brep::Body, edge: EdgeId) -> Flush<'a> {
        Flush {
            on,
            edge,
            overlaps: Vec::new(),
        }
    }

    /// The edge of rank `rank` in `edges`, each edge of the step with its
    /// wedge on `given`, the body as given, its ends read there, taking with
    /// it the overlap at each of `corners` where it is the later of the two
    /// edges, as [`overlap`] reads it for `taken`, set back `set_back` of it,
    /// with the prisms `tool` raises. Declined as [`overlap`] declines.
    pub fn beside(
        given: &'a brep::Body,
        rank: usize,
        edges: &[(EdgeId, &Wedge)],
        corners: &[Beside],
        taken: f64,
        set_back: &impl Fn(&Wedge) -> f64,
        tool: &impl Fn(&Wedge, f64, Scale, (DVec3, DVec3)) -> Result<Prism, Declined>,
    ) -> Result<Flush<'a>, Declined> {
        let (edge, _) = edges[rank];
        let overlaps = (corners.iter())
            .filter(|&&(_, [_, later])| later == rank)
            .map(|&(corner, pair)| {
                let pair = pair.map(|rank| edges[rank]);
                overlap(given, corner, pair, taken, set_back, tool)
            })
            .collect::<Result<_, _>>()?;
        Ok(Flush {
            on: given,
            edge,
            overlaps,
        })
    }
}

/// The ends of `edge` on `body`, read there as `wedge`, where the one face met
/// but the edge's two is flat and the edge's prism stops flush on it: a cut's
/// where that face faces back along the edge, matter lying past it; a fill's
/// where it faces on along it, air lying past. Any other end a cut is carried
/// past, as at a block's corner, and a fill trimmed as [`Exact::filled`]
/// trims it.
pub(super) fn stops(body: &brep::Body, edge: EdgeId, wedge: &Wedge) -> Vec<Stop> {
    let ends = body.edge(edge).ends.into_iter().flatten();
    ends.zip([-wedge.along, wedge.along])
        .filter_map(|(end, past)| {
            let face = end_face(body, end, wedge.faces)?;
            let (plane, outward) = flat(body, face)?;
            let on = outward.dot(past);
            (on != 0.0 && (on > 0.0) == wedge.inwards).then(|| Stop {
                plane: *plane,
                away: outward * on.signum(),
                end,
                corner: body.vertex(end).point,
            })
        })
        .collect()
}

impl Exact {
    /// The body with the edge `flush` reads, as `wedge`, cut off or filled
    /// in by `tool`, the prism raised from `prism`, its first wall the edge's,
    /// numbered nought and counted as raised, so that the solids its ends are
    /// trimmed by and the overlaps it takes number their faces past its own:
    /// a fill trimmed flush at both ends and joined, as [`Exact::filled`]
    /// lays it; a cut trimmed flush at each of its [`stops`], joined to each
    /// overlap, and cut, the trimmed ends and the overlaps merged into the
    /// faces they meet, as [`Exact::merged_at`] merges them at the corners the
    /// edge ended at and the points the overlaps clear, and carried past any
    /// other end. Declined as the trims, the joins and the booleans decline.
    pub(super) fn laid_flush(
        &self,
        flush: &Flush,
        wedge: &Wedge,
        prism: &Prism,
        tool: Exact,
    ) -> Result<Exact, Declined> {
        let edges = [(flush.edge, wedge)];
        if wedge.inwards {
            return self.filled(flush.on, &edges, &flush.overlaps, prism, tool);
        }
        let stops = stops(flush.on, flush.edge, wedge);
        let mut trimmed = tool;
        for stop in &stops {
            trimmed = trimmed.cut_by(&stop.beyond(prism)?)?;
        }
        for overlap in &flush.overlaps {
            trimmed = trimmed.joined(&overlap.solid)?;
        }
        let cut = self.cut_by(&Exact { next: 1, ..trimmed })?;
        if stops.is_empty() {
            return Ok(cut);
        }
        let corners: Vec<DVec3> = (stops.iter().map(|stop| stop.corner))
            .chain(
                flush
                    .overlaps
                    .iter()
                    .flat_map(|overlap| overlap.cleared.iter().copied()),
            )
            .collect();
        cut.merged_at(&corners)
    }
}

/// The plane of the face `id` and the way it faces out of the matter, where
/// it is flat; none on any other face.
fn flat(body: &brep::Body, id: FaceId) -> Option<(&Plane, DVec3)> {
    let face = body.face(id);
    let Surface::Plane(plane) = body.surface(face.surface) else {
        return None;
    };
    let outward = if face.flipped {
        -plane.normal
    } else {
        plane.normal
    };
    Some((plane, outward))
}
