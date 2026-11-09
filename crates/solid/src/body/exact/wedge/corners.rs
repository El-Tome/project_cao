//! Edges of one step that meet at a corner (#699): laid in turn, each cut
//! carried past the corner along its edge's line, two flats or two rounds
//! trim each other in a mitre there. A corner is taken only where it is
//! convex.

use std::collections::{BTreeMap, BTreeSet};

use glam::DVec3;

use super::Wedge;
use crate::body::exact::Exact;
use crate::brep::{self, Declined, EdgeId, EdgeName, VertexId};

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

/// The corners three of `edges` meet at, each where it stands and with the
/// three edges' ranks in the list, from the lowest, in the order of those
/// ranks: by the lowest, then the next. Declined `Unsupported` where more
/// than three of them meet at one, or three meet where another edge ends
/// too.
pub(super) fn of_three(
    body: &brep::Body,
    edges: &[EdgeId],
) -> Result<Vec<(DVec3, [usize; 3])>, Declined> {
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
                Ok((body.vertex(corner).point, [one, two, three]))
            }
            _ => Err(Declined::Unsupported),
        })
        .collect::<Result<Vec<_>, Declined>>()?;
    corners.sort_by_key(|&(_, ranks)| ranks);
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

/// The names the edges after `rank` are sought by once the edge at `rank` is
/// laid, its face numbered `made`, as a mitre leaves them. At a corner a
/// later edge shares with it on `body`, the body as given, the end of the
/// later edge's name standing there — the one whose sides, as `names` gave
/// them, are those of the faces at that corner but the edge's own two — is
/// met by the face laid: in place of the face the laid edge ran along that
/// the later one does not, cut back from that corner, or beside the faces
/// laid there before. An end is rewritten only where `names`, read on the
/// body as given, tell which it is, so a name still finds an edge only where
/// its ends answer as the step's own cuts left them, and is lost or split as
/// any other.
pub(super) fn trimmed(
    body: &Exact,
    edges: &[EdgeId],
    names: &[EdgeName],
    sought: &mut [EdgeName],
    rank: usize,
    made: u32,
) {
    let ends = |edge: EdgeId| body.brep.edge(edge).ends.into_iter().flatten();
    let laid = &names[rank].between;
    for later in rank + 1..names.len() {
        let name = &names[later];
        let shared = ends(edges[later]).find(|&end| ends(edges[rank]).any(|other| other == end));
        let Some(corner) = shared else {
            continue;
        };
        let mut there = body.vertices[corner.0 as usize].faces.clone();
        there.retain(|side| !name.between.contains(side));
        let at: Vec<usize> = (0..2).filter(|&end| name.ends[end] == there).collect();
        let away: Vec<&Vec<u32>> = (laid.iter())
            .filter(|side| !name.between.contains(side))
            .collect();
        let (&[end], &[away]) = (&at[..], &away[..]) else {
            continue;
        };
        let sides = &mut sought[later].ends[end];
        match sides.iter().position(|side| side == away) {
            Some(place) => sides[place] = vec![made],
            None => sides.push(vec![made]),
        }
    }
}

/// Whether two edges share a corner.
pub(super) fn meet(body: &brep::Body, [one, other]: [EdgeId; 2]) -> bool {
    let ends = |edge: EdgeId| body.edge(edge).ends.into_iter().flatten();
    ends(one).any(|end| ends(other).any(|theirs| theirs == end))
}
