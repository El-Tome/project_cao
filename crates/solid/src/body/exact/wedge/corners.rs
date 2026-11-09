//! Edges of one step that meet at a corner (#699): laid in turn, each cut
//! carried past the corner along its edge's line, two flats or two rounds
//! trim each other in a mitre there. A corner is taken only where it is
//! convex.

use std::collections::{BTreeMap, BTreeSet};

use glam::DVec3;

use super::Wedge;
use crate::body::exact::{Exact, root};
use crate::brep::{self, Declined, EdgeId, EdgeName, Scale, VertexId};

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
/// laid on `laid`: each later edge that meets it at a corner of `given`, the
/// body as given, found again on `laid` — the one edge there whose two ends
/// lie on its line, read off its wedge, and whose middle stands within its
/// old stretch: what the cuts left of it, between its own two faces, since
/// no other face holds that line — and named as `laid` names that edge. A
/// later edge no edge of `laid` stands for, the cuts having taken the whole
/// of it, joins `gone`, to be laid from its wedge on the body as given.
/// Declined `Unsupported` where several edges of `laid` stand for one, or
/// the name `laid` gives the one does not find it.
pub(super) fn tracked(
    given: &Exact,
    laid: &Exact,
    (edges, wedges): (&[EdgeId], &[Wedge]),
    (sought, gone): (&mut [EdgeName], &mut BTreeSet<usize>),
    rank: usize,
) -> Result<(), Declined> {
    let near = Scale::HAIR * laid.brep.scale().eps();
    for later in rank + 1..edges.len() {
        if gone.contains(&later) || !meet(&given.brep, [edges[rank], edges[later]]) {
            continue;
        }
        let wedge = &wedges[later];
        let at = |point: DVec3| wedge.along.dot(point - wedge.point);
        let on_line = |point: DVec3| (point - wedge.point).cross(wedge.along).length() <= near;
        let ends = (given.brep.edge(edges[later]).ends).ok_or(Declined::Unsupported)?;
        let [from, to] = ends.map(|end| at(given.brep.vertex(end).point));
        let standing: Vec<EdgeId> = (laid.brep.edge_ids())
            .filter(|&edge| {
                let Some(ends) = laid.brep.edge(edge).ends else {
                    return false;
                };
                let [one, other] = ends.map(|end| laid.brep.vertex(end).point);
                let middle = at((one + other) / 2.0);
                on_line(one) && on_line(other) && from.min(to) <= middle && middle <= from.max(to)
            })
            .collect();
        match standing[..] {
            [] => {
                gone.insert(later);
            }
            [edge] => {
                let name = &laid.edges[edge.0 as usize];
                let found = laid
                    .brep
                    .find_rooted(name, |number| root(&laid.roots, number));
                if found != Ok(edge) {
                    return Err(Declined::Unsupported);
                }
                sought[later] = name.clone();
            }
            _ => return Err(Declined::Unsupported),
        }
    }
    Ok(())
}

/// Whether two edges share a corner.
pub(super) fn meet(body: &brep::Body, [one, other]: [EdgeId; 2]) -> bool {
    let ends = |edge: EdgeId| body.edge(edge).ends.into_iter().flatten();
    ends(one).any(|end| ends(other).any(|theirs| theirs == end))
}
