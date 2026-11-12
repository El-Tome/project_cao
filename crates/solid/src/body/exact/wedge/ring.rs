//! Rounds of one step closing on themselves (#790): every edge of the list
//! laid together with two others across the faces it shares with them, the
//! full rounds about one axis, so that they are one cylinder with no seam —
//! the four long edges of a square bar rounded to half its side turn it
//! round, its two ends discs.

use std::collections::BTreeSet;
use std::f64::consts::TAU;

use glam::{DVec2, DVec3};

use super::Wedge;
use super::end::end_face;
use super::pairs::Pair;
use crate::body::exact::Exact;
use crate::brep::{self, Declined, EdgeId, VertexId};
use crate::profile::{Contour, Frame, Run};

/// The cylinder the rounds of a step close into: a point of its axis, the
/// way it runs, its radius, how far along it from that point the edges run,
/// and how far from it they stand at most, past which the cut reaches.
pub(super) struct Ring {
    centre: DVec3,
    along: DVec3,
    radius: f64,
    span: [f64; 2],
    reach: f64,
}

/// The cylinder the rounds of `edges`, their wedges `wedges`, close into,
/// where `pairs` lays every edge together with two others in one closed
/// chain, each a cut, their full rounds about one axis — each face within
/// the tolerance of the pair's value from it — and each end of every edge
/// on a flat face bounded by edges between the edges' ends alone, nothing
/// standing past them for the cut to reach. None anywhere else.
pub(super) fn closed(
    body: &brep::Body,
    edges: &[EdgeId],
    wedges: &[Wedge],
    pairs: &[Pair],
) -> Option<Ring> {
    let count = wedges.len();
    let in_two = (0..count).all(|rank| {
        let met = pairs
            .iter()
            .filter(|&&(one, other, _)| rank == one || rank == other);
        met.count() == 2
    });
    if count < 3 || pairs.len() != count || !in_two || wedges.iter().any(|w| w.inwards) {
        return None;
    }
    if super::pairs::together(0, pairs).len() != count {
        return None;
    }
    let corners: BTreeSet<VertexId> = (edges.iter())
        .flat_map(|&edge| body.edge(edge).ends.into_iter().flatten())
        .collect();
    let between_ends = |coedge: &brep::Coedge| {
        let ends = body.edge(coedge.edge).ends;
        ends.is_some_and(|ends| ends.iter().all(|end| corners.contains(end)))
    };
    let capped = edges.iter().zip(wedges).all(|(&edge, wedge)| {
        (body.edge(edge).ends.into_iter().flatten()).all(|end| {
            end_face(body, end, wedge.faces)
                .is_some_and(|face| body.face(face).loops.iter().flatten().all(between_ends))
        })
    });
    let along = wedges[0].along;
    let base = wedges[0].point;
    let flat = |at: DVec3| at - along * along.dot(at - base);
    let centre = wedges.iter().map(|wedge| flat(wedge.point)).sum::<DVec3>() / count as f64;
    let radius = pairs[0].2;
    let eps = body.scale().eps();
    let about = (wedges.iter()).all(|wedge| {
        (wedge.outward.iter()).all(|side| (side.dot(centre - wedge.point) + radius).abs() <= eps)
    });
    if !capped || !about {
        return None;
    }
    let points: Vec<DVec3> = corners.iter().map(|&end| body.vertex(end).point).collect();
    let (low, high) =
        (points.iter()).fold((f64::INFINITY, f64::NEG_INFINITY), |(low, high), at| {
            let off = along.dot(*at - centre);
            (low.min(off), high.max(off))
        });
    let reach = (points.iter())
        .map(|&at| flat(at).distance(centre))
        .fold(0.0, f64::max);
    Some(Ring {
        centre,
        along,
        radius,
        span: [low, high],
        reach,
    })
}

impl Exact {
    /// The body cut down to the cylinder of `ring`, by a prism raised along
    /// its axis past both ends of the edges — a square standing off them by
    /// the margin the first of `wedges` takes, and the circle of the ring's
    /// radius a hole through it, drawn in as many runs as there are edges —
    /// the cylinder's one face answering to every edge's number, where the
    /// steps' numbers ended, the ends of the body left discs, each bounded by
    /// one circle: a corner the cut left on a rim, where the circle touches
    /// the plane of a face the body still holds elsewhere — a block standing
    /// apart on the bar's floor — is cleared, as [`Exact::merged_at`] clears
    /// it. Declined as the cut declines, and `Unsupported` where it left
    /// anything else than that face, as [`Exact::cut_along`] reads it of
    /// every edge.
    pub(super) fn closed_round(&self, ring: &Ring, wedges: &[Wedge]) -> Result<Exact, Declined> {
        let count = wedges.len();
        let (across, facing) = wedges[0].square();
        let margin = wedges[0].margin(across, facing, self.brep.scale());
        let [low, high] = ring.span;
        let frame = Frame {
            origin: ring.centre + ring.along * (low - margin),
            u: across,
            v: ring.along.cross(across),
        };
        let travel = ring.along * (high - low + 2.0 * margin);
        let side = ring.reach + margin;
        let outline = Contour {
            corners: [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]
                .map(|(x, y)| DVec2::new(x, y) * side)
                .to_vec(),
            runs: vec![Run::Straight; 4],
        };
        let turn = TAU / count as f64;
        let hole = Contour {
            corners: (0..count)
                .map(|at| DVec2::from_angle(turn * at as f64) * ring.radius)
                .collect(),
            runs: vec![
                Run::Round {
                    center: DVec2::ZERO,
                    turn,
                };
                count
            ],
        };
        let raised = Exact::raised(&outline, &[hole], frame, travel)?;
        let first = brep::Body::numbers_raised(&outline, &[]);
        let laid = count as u32;
        let tool = Exact {
            brep: raised.brep.renumbered_by(|number| match number {
                number if number >= first => number - first,
                number => number + laid,
            }),
            next: laid,
            ..raised
        };
        let cut = self.cut_by(&tool)?;
        let wedges: Vec<&Wedge> = wedges.iter().collect();
        if !cut.cut_along(&wedges, self, laid, &[], &[]) {
            return Err(Declined::Unsupported);
        }
        let rims: Vec<DVec3> = (cut.named(self.next as usize))
            .flat_map(|face| cut.brep.face(face).loops.iter().flatten())
            .flat_map(|coedge| cut.brep.edge(coedge.edge).ends.into_iter().flatten())
            .map(|end| cut.brep.vertex(end).point)
            .collect();
        cut.merged_at(&rims)
    }
}
