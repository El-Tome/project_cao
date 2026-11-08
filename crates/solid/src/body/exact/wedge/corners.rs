//! Edges of one step that meet at a corner (#699): laid in turn, each cut
//! carried past the corner along its edge's line, two flats or two rounds
//! trim each other in a mitre there. A corner is taken only where it is
//! convex.

use std::collections::BTreeSet;

use super::Wedge;
use crate::brep::{self, EdgeId, EdgeName, VertexId};

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

/// The names the edges after `rank` are sought by once the edge at `rank` is
/// laid, its face numbered `made`, as a mitre leaves them: at a corner a later
/// edge shares with it, the face the laid edge ran along that the later one
/// does not, cut back from that corner, is met there by the face laid
/// instead. An end is rewritten only where that face meets one end of the
/// later edge, so a name still finds an edge only where its ends answer as
/// the step's own cuts left them, and is lost or split as any other.
pub(super) fn trimmed(
    body: &brep::Body,
    edges: &[EdgeId],
    names: &mut [EdgeName],
    rank: usize,
    made: u32,
) {
    let laid = names[rank].between.clone();
    for later in rank + 1..names.len() {
        if !meet(body, [edges[rank], edges[later]]) {
            continue;
        }
        let name = &mut names[later];
        let away: Vec<&Vec<u32>> = laid
            .iter()
            .filter(|side| !name.between.contains(side))
            .collect();
        let at: Vec<usize> = (0..2)
            .filter(|&end| away.iter().any(|side| name.ends[end].contains(side)))
            .collect();
        if let (&[away], &[end]) = (&away[..], &at[..]) {
            for side in name.ends[end].iter_mut().filter(|side| *side == away) {
                *side = vec![made];
            }
        }
    }
}

/// Whether two edges share a corner.
pub(super) fn meet(body: &brep::Body, [one, other]: [EdgeId; 2]) -> bool {
    let ends = |edge: EdgeId| body.edge(edge).ends.into_iter().flatten();
    ends(one).any(|end| ends(other).any(|theirs| theirs == end))
}
