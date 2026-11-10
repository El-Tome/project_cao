//! Edges of one step that meet at a corner: which corners a step takes, and
//! the corners three of its edges meet at. At a convex corner (#699) each cut
//! is carried past the corner along its edge's line, and two flats or two
//! rounds trim each other in a mitre there. Beside an edge of the other sense
//! left out of the list (#801) — two cuts beside a concave edge, two fills
//! beside a convex one — each stops flush and the two run on past the corner
//! into a mitre. Two concave edges beside a third concave one, at a pocket's
//! bottom corner, are each trimmed flush and meet in a valley. Any other
//! corner is declined.

use std::collections::BTreeMap;

use glam::DVec3;

use super::Wedge;
use crate::brep::{self, Declined, EdgeId, VertexId};

/// A corner two edges of a step meet at beside a third that is not convex
/// with them, and the two edges' ranks in the list, the lower first.
pub(super) type Beside = (VertexId, [usize; 2]);

/// Every corner two of `edges` meet at taken or declined, and those not
/// convex handed back: three edges ending there, the two of the list straight
/// between flat faces that meet the same way, and the third, left out of it,
/// between faces that meet the other way — two meeting outwards beside one
/// meeting inwards at the inner corner of an L or a pocket's rim above an
/// upright, two meeting inwards beside one meeting outwards at the feet of a
/// block standing on another — or, the two meeting inwards, inwards too, at
/// a pocket's bottom corner. Every other corner two of them meet at is to be
/// convex: each edge ending there straight between two flat faces that meet
/// outwards, whatever those faces hold past the corner. Declined
/// `Unsupported` where one is none of these.
pub(super) fn admit(body: &brep::Body, edges: &[EdgeId]) -> Result<Vec<Beside>, Declined> {
    let meeting = |edge, inwards| Wedge::faced(body, edge).is_ok_and(|w| w.inwards == inwards);
    let mut beside = Vec::new();
    for (corner, ranks) in ranked(body, edges)
        .into_iter()
        .filter(|(_, ranks)| ranks.len() > 1)
    {
        let ending: Vec<EdgeId> = ending(body, corner).collect();
        if ending.iter().all(|&edge| meeting(edge, false)) {
            continue;
        }
        let left_out = |edge: &&EdgeId| !ranks.iter().any(|&rank| edges[rank] == **edge);
        match (ranks.as_slice(), ending.len()) {
            (&[one, other], 3)
                if [false, true].into_iter().any(|inwards| {
                    meeting(edges[one], inwards)
                        && meeting(edges[other], inwards)
                        && (ending.iter().filter(left_out))
                            .all(|&edge| meeting(edge, !inwards) || inwards && meeting(edge, true))
                }) =>
            {
                beside.push((corner, [one, other]));
            }
            _ => return Err(Declined::Unsupported),
        }
    }
    Ok(beside)
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
    let mut corners = ranked(body, edges)
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

/// The ranks in `edges` of the edges ending at each corner one of them ends
/// at, from the lowest.
fn ranked(body: &brep::Body, edges: &[EdgeId]) -> BTreeMap<VertexId, Vec<usize>> {
    let mut at: BTreeMap<VertexId, Vec<usize>> = BTreeMap::new();
    for (rank, &edge) in edges.iter().enumerate() {
        for end in body.edge(edge).ends.into_iter().flatten() {
            at.entry(end).or_default().push(rank);
        }
    }
    at
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

#[cfg(test)]
mod tests;
