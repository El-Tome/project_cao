//! The loop the faces a piece of torus replaces make towards the rest of the
//! body, read where it runs: parted where it lies on the plane a corner left
//! sharp ends the piece on (#846), and each of its coedges by its two ends.

use glam::DVec3;

use crate::brep::{self, Coedge, Curve, VertexId};

/// The loop `lap` parted where it runs on a plane: the stretch of it whose
/// edges lie on the plane, as `on_the_end` reads their ends, and the rest,
/// each in the loop's order, the rest first, so that each runs on into the
/// other. None where no edge, or every edge, lies on it, or those that do are
/// not one stretch.
pub(super) fn parted(
    body: &brep::Body,
    lap: &[Coedge],
    on_the_end: impl Fn(DVec3) -> bool,
) -> Option<(Vec<Coedge>, Vec<Coedge>)> {
    let lying = |coedge: &Coedge| {
        let stretch = body.edge(coedge.edge);
        matches!(body.curve(stretch.curve), Curve::Line(_))
            && (stretch.ends.into_iter().flatten()).all(|end| on_the_end(body.vertex(end).point))
    };
    let on: Vec<bool> = lap.iter().map(lying).collect();
    let starts = (0..lap.len()).filter(|&at| on[at] && !on[(at + lap.len() - 1) % lap.len()]);
    let [start] = starts.collect::<Vec<_>>()[..] else {
        return None;
    };
    let turned: Vec<Coedge> = (0..lap.len())
        .map(|at| lap[(start + at) % lap.len()])
        .collect();
    let length = on.iter().filter(|&&lies| lies).count();
    let (flat, round) = turned.split_at(length);
    Some((round.to_vec(), flat.to_vec()))
}

/// Where a coedge starts and ends, the way its face runs it.
pub(super) fn ends(body: &brep::Body, coedge: Coedge) -> Option<[VertexId; 2]> {
    let [from, to] = body.edge(coedge.edge).ends?;
    Some(if coedge.forward {
        [from, to]
    } else {
        [to, from]
    })
}
