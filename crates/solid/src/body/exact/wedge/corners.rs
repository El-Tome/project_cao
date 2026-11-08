//! Edges of one step that meet at a corner (#699): laid in turn, each cut
//! carried past the corner along its edge's line, two flats or two rounds
//! trim each other in a mitre there. A corner is taken only where it is
//! convex.

use std::collections::BTreeSet;

use super::Wedge;
use crate::brep::{self, EdgeId, VertexId};

/// Whether every corner two of `edges` meet at is convex: each edge ending
/// there straight between two flat faces that meet outwards. The inner
/// corner of an L and the corners of a pocket are not.
pub(super) fn convex(body: &brep::Body, edges: &[EdgeId]) -> bool {
    let mut ends = BTreeSet::new();
    let shared: Vec<VertexId> = edges
        .iter()
        .flat_map(|&edge| body.edge(edge).ends.into_iter().flatten())
        .filter(|&end| !ends.insert(end))
        .collect();
    shared.into_iter().all(|corner| {
        body.edge_ids()
            .filter(|&edge| {
                body.edge(edge)
                    .ends
                    .is_some_and(|ends| ends.contains(&corner))
            })
            .all(|edge| Wedge::of(body, edge).is_ok_and(|wedge| !wedge.inwards))
    })
}
