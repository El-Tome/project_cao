//! Steps 5 to 7 of `docs/exact-kernel.md`: each surface an operand has a face
//! on is parted by the arcs lying on it into regions, and a region is kept
//! where the operation says something different just above it and just below
//! it — the side its surface's own normal points to, and the other.
//!
//! On each side of a region, each operand wraps it some number of times: a
//! face of the operand covering the region says one on its matter's side and
//! nought on the other; no face covering it says the same on both sides, as
//! a ray cast through the operand counts. One rule gives coincident faces
//! once, drops a shared wall, cuts a flush hole, and leaves a non-manifold
//! edge its four uses. Regions of two surfaces bounded by the same arcs are
//! one piece of surface, decided once ([`twins`]).

mod twins;
mod wrapped;

use std::collections::BTreeMap;

use glam::{DVec2, DVec3};

use super::Declined;
use super::combine::{Arena, Operands, Operation};
use super::overlay::{Arc, Overlay, Region};
use super::surface::Surface;
use super::topology::{Coedge, EdgeId, Face, SurfaceId, VertexId};
use twins::{Twin, twins};
use wrapped::{Wrapped, collapsed, covering, wound};

pub(super) fn selected(
    operands: &Operands,
    arena: &Arena,
    operation: Operation,
) -> Result<Vec<Face>, Declined> {
    let carried: Vec<SurfaceId> = (0..operands.surfaces.list.len() as u32)
        .map(SurfaceId)
        .filter(|surface| operands.carries(0, *surface) || operands.carries(1, *surface))
        .collect();
    let parted = carried
        .iter()
        .map(|surface| parted(arena, *surface))
        .collect::<Result<Vec<_>, _>>()?;
    let seen: Vec<(&Surface, &[EdgeId], &Overlay)> = carried
        .iter()
        .zip(&parted)
        .map(|(surface, (edges, overlay))| {
            (arena.body.surface(*surface), edges.as_slice(), overlay)
        })
        .collect();
    let twins = twins(&seen, operands.eps());
    let mut faces = Vec::new();
    for (rank, &surface) in carried.iter().enumerate() {
        let (edges, overlay) = &parted[rank];
        for (index, region) in overlay.regions.iter().enumerate() {
            let others: &[(usize, usize, bool)] = match &twins[rank][index] {
                _ if region.unbounded => continue,
                Twin::Other => continue,
                Twin::Alone => &[],
                Twin::First(others) => others,
            };
            let members: Vec<Member> = std::iter::once((rank, index, false))
                .chain(others.iter().copied())
                .map(|(rank, index, turned)| Member {
                    surface: carried[rank],
                    geometry: seen[rank].0,
                    region: &parted[rank].1.regions[index],
                    turned,
                })
                .collect();
            let Some([first, second]) = wraps(operands, &members)? else {
                continue;
            };
            let above = operation.holds(first.above >= 1, second.above >= 1);
            let below = operation.holds(first.below >= 1, second.below >= 1);
            if above != below {
                faces.push(face(surface, above, region, edges));
            }
        }
    }
    Ok(faces)
}

/// A region to be decided, and whether its surface's normal points the other
/// way from the first's.
struct Member<'a> {
    surface: SurfaceId,
    geometry: &'a Surface,
    region: &'a Region,
    turned: bool,
}

/// How each operand wraps a region and its twins, as the first's surface
/// sees it, or nothing where neither covers any of them.
///
/// Read at each region's point; where an operand touches the surface at that
/// very point — a bar's cap resting on a post's wall — the answer is a tie,
/// and it is read again further along each region's chord, where the answer
/// is the same.
fn wraps(operands: &Operands, members: &[Member]) -> Result<Option<[Wrapped; 2]>, Declined> {
    for share in [None, Some(0.25), Some(0.75)] {
        let places: Vec<(SurfaceId, DVec3, bool)> = members
            .iter()
            .map(|member| {
                let region = member.region;
                let [low, high] = region.chord;
                let at = share.map_or(region.inside, |share| {
                    DVec2::new(region.inside.x, low + (high - low) * share)
                });
                (member.surface, member.geometry.point(at), member.turned)
            })
            .collect();
        match wrapped_at(operands, &places) {
            Err(Declined::Tie) => continue,
            answer => return answer,
        }
    }
    Err(Declined::Tie)
}

/// An operand covering two twins at once lies back to back with itself
/// there: a skin or a crack an earlier operation left thinner than this
/// one's tolerance, which wraps both sides alike once the sheet is gone
/// ([`collapsed`]), and covers neither.
fn wrapped_at(
    operands: &Operands,
    places: &[(SurfaceId, DVec3, bool)],
) -> Result<Option<[Wrapped; 2]>, Declined> {
    let mut covered = [None, None];
    let mut sheet = [None, None];
    for operand in 0..2 {
        let mut found = Vec::new();
        for &(surface, point, turned) in places {
            if let Some(wrapped) = covering(operands, operand, surface, point)? {
                found.push((
                    surface,
                    point,
                    if turned { wrapped.turned() } else { wrapped },
                ));
            }
        }
        match found.as_slice() {
            [] => {}
            [(_, _, wrapped)] => covered[operand] = Some(*wrapped),
            [one, other] => sheet[operand] = Some(collapsed(operands, operand, [*one, *other])?),
            _ => return Err(Declined::Tie),
        }
    }
    if covered.iter().all(Option::is_none) {
        return Ok(None);
    }
    let point = places[0].1;
    let [first, second] = [0, 1].map(|operand| match (covered[operand], sheet[operand]) {
        (Some(wrapped), _) => Ok(wrapped),
        (None, Some(winding)) => Ok(Wrapped {
            covered: false,
            above: winding,
            below: winding,
        }),
        (None, None) => wound(operands, operand, point),
    });
    Ok(Some([first?, second?]))
}

/// The arcs lying on a surface, by the edge each is, and the regions they
/// part the surface into.
pub(super) fn parted(
    arena: &Arena,
    surface: SurfaceId,
) -> Result<(Vec<EdgeId>, Overlay), Declined> {
    let body = &arena.body;
    let geometry = body.surface(surface);
    let edges: Vec<EdgeId> = body
        .edge_ids()
        .filter(|edge| {
            arena.supports[edge.0 as usize]
                .binary_search(&surface)
                .is_ok()
        })
        .collect();
    let mut local: BTreeMap<VertexId, usize> = BTreeMap::new();
    let mut vertices = Vec::new();
    let mut arcs = Vec::with_capacity(edges.len());
    for &edge in &edges {
        let trace = body.trace(edge, surface)?;
        let ends = match body.edge(edge).ends {
            Some(ends) => {
                let mut ranks = [0; 2];
                for (rank, vertex) in ranks.iter_mut().zip(ends) {
                    *rank = *local.entry(vertex).or_insert_with(|| {
                        vertices.push(geometry.parameters(body.vertex(vertex).point));
                        vertices.len() - 1
                    });
                }
                Some(ranks)
            }
            None => None,
        };
        arcs.push(Arc { trace, ends });
    }
    let overlay = Overlay::of(&vertices, &arcs, geometry.period())?;
    Ok((edges, overlay))
}

/// A region as a face, its matter on the side its surface's own normal
/// points to when `flipped`: its cycles keep it on their left in the
/// surface's parameters, which is seen from outside the matter only when it
/// is not flipped.
fn face(surface: SurfaceId, flipped: bool, region: &Region, edges: &[EdgeId]) -> Face {
    let loops = region
        .cycles
        .iter()
        .map(|cycle| {
            let coedges = cycle.iter().map(|&(arc, forward)| Coedge {
                edge: edges[arc],
                forward,
            });
            if flipped {
                coedges
                    .rev()
                    .map(|coedge| Coedge {
                        forward: !coedge.forward,
                        ..coedge
                    })
                    .collect()
            } else {
                coedges.collect()
            }
        })
        .collect();
    Face {
        surface,
        flipped,
        loops,
    }
}

#[cfg(test)]
mod tests;
