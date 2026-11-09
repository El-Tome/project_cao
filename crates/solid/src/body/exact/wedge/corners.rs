//! Edges of one step that meet at a corner (#699): laid in turn, each cut
//! carried past the corner along its edge's line, two flats or two rounds
//! trim each other in a mitre there. A corner is taken only where it is
//! convex.

use std::collections::{BTreeMap, BTreeSet};

use glam::DVec3;

use super::Wedge;
use crate::brep::{self, Declined, EdgeId, VertexId};

/// Whether every corner two of `edges` meet at is convex: each edge ending
/// there straight between two flat faces that meet outwards, whatever those
/// faces hold past the corner. The inner corner of an L and the corners of a
/// pocket are not.
pub(super) fn convex(body: &brep::Body, edges: &[EdgeId]) -> bool {
    let mut ends = BTreeSet::new();
    let shared: Vec<VertexId> = edges
        .iter()
        .flat_map(|&edge| body.edge(edge).ends.into_iter().flatten())
        .filter(|&end| !ends.insert(end))
        .collect();
    shared.into_iter().all(|corner| {
        let outwards = |edge| Wedge::faced(body, edge).is_ok_and(|wedge| !wedge.inwards);
        ending(body, corner).all(outwards)
    })
}

/// A corner three edges of a step meet at: where it stands, the three edges'
/// ranks in the list, from the lowest, and the way each runs from the corner
/// to its other end.
pub(super) type Three = (DVec3, [usize; 3], [DVec3; 3]);

/// The corners three of `edges` meet at, each where it stands, with the
/// three edges' ranks in the list, from the lowest, and the way each edge
/// runs from the corner to its other end, in the order of those ranks: by
/// the lowest, then the next. Declined `Unsupported` where more than three
/// of them meet at one, or three meet where another edge ends too.
pub(super) fn of_three(body: &brep::Body, edges: &[EdgeId]) -> Result<Vec<Three>, Declined> {
    let mut at: BTreeMap<VertexId, Vec<usize>> = BTreeMap::new();
    for (rank, &edge) in edges.iter().enumerate() {
        for end in body.edge(edge).ends.into_iter().flatten() {
            at.entry(end).or_default().push(rank);
        }
    }
    let mut corners = at
        .into_iter()
        .filter(|(_, ranks)| ranks.len() > 2)
        .map(|(corner, ranks)| match ranks[..] {
            [one, two, three] if ending(body, corner).count() == 3 => {
                let at = body.vertex(corner).point;
                let way = |rank: usize| {
                    let ends = body.edge(edges[rank]).ends.into_iter().flatten();
                    let far = ends
                        .filter(|&end| end != corner)
                        .map(|end| body.vertex(end).point);
                    far.map(|point| (point - at).normalize())
                        .next()
                        .unwrap_or(DVec3::ZERO)
                };
                Ok((at, [one, two, three], [one, two, three].map(way)))
            }
            _ => Err(Declined::Unsupported),
        })
        .collect::<Result<Vec<_>, Declined>>()?;
    corners.sort_by_key(|&(_, ranks, _)| ranks);
    Ok(corners)
}

/// The edges that end at a corner.
fn ending(body: &brep::Body, corner: VertexId) -> impl Iterator<Item = EdgeId> + '_ {
    body.edge_ids().filter(move |&edge| {
        body.edge(edge)
            .ends
            .is_some_and(|ends| ends.contains(&corner))
    })
}

/// Whether two edges share a corner.
pub(super) fn meet(body: &brep::Body, [one, other]: [EdgeId; 2]) -> bool {
    let ends = |edge: EdgeId| body.edge(edge).ends.into_iter().flatten();
    ends(one).any(|end| ends(other).any(|theirs| theirs == end))
}
