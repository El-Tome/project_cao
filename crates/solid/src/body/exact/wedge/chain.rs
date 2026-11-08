//! Whether a straight edge runs on tangentially into the next edge of one
//! of its faces: a tangent chain, which a chamfer or a fillet of the edge
//! alone does not take (#700), read on the body a step is given.

use glam::DVec3;

use super::Wedge;
use crate::brep::{self, Coedge, Declined, EdgeId, FaceId};

impl Wedge {
    /// The wedge of an edge, as [`Wedge::read`] reads it, on no tangent chain.
    pub(super) fn of(body: &brep::Body, edge: EdgeId) -> Result<Wedge, Declined> {
        let wedge = Wedge::read(body, edge)?;
        if wedge
            .faces
            .iter()
            .any(|&face| runs_on(body, face, edge, wedge.along))
        {
            return Err(Declined::Unsupported);
        }
        Ok(wedge)
    }
}

/// The sine of the angle under which an edge running on into the next one
/// is a tangent chain. Well above what a sketch solved within a billionth of
/// its size leaves between a round and the run it was drawn tangent to —
/// some 1.5e-8 for a round of 2 in a drawing of 30, ten times that for a
/// round of 0.2 — and well below any corner a hand draws: a thousandth of
/// a degree is some 1.7e-5. That noise is a billionth of the drawing's size
/// over the round's radius, so this holds it up to a drawing a thousand
/// times the radius; past that, a round a hair off tangent reads as a
/// corner and is cut, soundly.
pub(super) const TANGENT: f64 = 1e-6;

/// Whether, along a loop of `face`, the edge runs on into the edge after it,
/// or the edge before it runs on into it, along the edge's own way within
/// [`TANGENT`]: a tangent chain.
fn runs_on(body: &brep::Body, face: FaceId, edge: EdgeId, along: DVec3) -> bool {
    body.face(face).loops.iter().any(|lap| {
        let Some(at) = lap.iter().position(|coedge| coedge.edge == edge) else {
            return false;
        };
        let running = if lap[at].forward { along } else { -along };
        let [next, before] = [at + 1, at + lap.len() - 1].map(|rank| lap[rank % lap.len()]);
        let on = |way: DVec3| way.dot(running) > 0.0 && way.cross(running).length() <= TANGENT;
        on(ways(body, next)[0]) || on(ways(body, before)[1])
    })
}

/// The way a loop runs along a coedge where it starts and where it ends.
fn ways(body: &brep::Body, coedge: Coedge) -> [DVec3; 2] {
    let edge = body.edge(coedge.edge);
    let curve = body.curve(edge.curve);
    let sign = (edge.to - edge.from).signum();
    let [start, end] = [edge.from, edge.to].map(|at| curve.derivative(at).normalize() * sign);
    if coedge.forward {
        [start, end]
    } else {
        [-end, -start]
    }
}
