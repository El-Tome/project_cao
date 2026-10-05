//! Regions of two surfaces bounded by the very same arcs, which are one piece
//! of surface: a plane touching a cylinder a hair from a side leaves a strip
//! of the plane and a strip of the wall between the line they touch along and
//! the side's edge, whose ends decision 6 made the same arcs. Every arc
//! bounding them lies on both surfaces within the tolerance, and a point
//! inside either stands within it of the other surface: a point of either is
//! as good as on the other, and neither can be wound by a ray. The point is
//! what tells them from the two caps two cylinders crossing bound with the
//! one loop they meet along.
//!
//! Such regions are decided once, from how each operand covers either, and
//! kept on the first surface — a plane before a cylinder, then the one
//! ranked first — so that a skin thinner than the tolerance is not left
//! between two faces back to back.

use std::collections::BTreeMap;

use crate::brep::overlay::{Overlay, Region};
use crate::brep::surface::Surface;
use crate::brep::topology::EdgeId;

/// What a region of a surface is to the regions of other surfaces bounded by
/// the same arcs.
#[derive(Clone, Debug, PartialEq)]
pub(super) enum Twin {
    /// Bounded like no region of another surface.
    Alone,
    /// Decided for the others, by surface and region, each with whether its
    /// surface's normal points the other way.
    First(Vec<(usize, usize, bool)>),
    /// Decided by another.
    Other,
}

/// For every surface, by its rank in `parted`, and every region of it, what
/// it is to the others.
pub(super) fn twins(parted: &[(&Surface, &[EdgeId], &Overlay)], eps: f64) -> Vec<Vec<Twin>> {
    let mut alike: BTreeMap<Vec<EdgeId>, Vec<(usize, usize)>> = BTreeMap::new();
    let mut found: Vec<Vec<Twin>> = parted
        .iter()
        .map(|(_, _, overlay)| vec![Twin::Alone; overlay.regions.len()])
        .collect();
    for (surface, (_, edges, overlay)) in parted.iter().enumerate() {
        for (rank, region) in overlay.regions.iter().enumerate() {
            if !region.unbounded && !region.cycles.is_empty() {
                alike
                    .entry(bounding(region, edges))
                    .or_default()
                    .push((surface, rank));
            }
        }
    }
    for group in alike.into_values().filter(|group| group.len() > 1) {
        let first = *group
            .iter()
            .min_by_key(|(surface, _)| (!matches!(parted[*surface].0, Surface::Plane(_)), *surface))
            .expect("a group holds a region");
        let region = |(surface, rank): (usize, usize)| &parted[surface].2.regions[rank];
        let close = |one: (usize, usize), other: (usize, usize)| {
            let inside = parted[one.0].0.point(region(one).inside);
            parted[other.0].0.distance(inside).abs() <= eps
        };
        let others: Vec<(usize, usize, bool)> = group
            .iter()
            .filter(|member| **member != first)
            .filter(|&&member| close(member, first) && close(first, member))
            .filter_map(|&member| {
                let facing = facing(
                    region(first),
                    parted[first.0].1,
                    region(member),
                    parted[member.0].1,
                )?;
                Some((member.0, member.1, !facing))
            })
            .collect();
        if others.len() + 1 != group.len() {
            continue;
        }
        for &(surface, rank, _) in &others {
            found[surface][rank] = Twin::Other;
        }
        found[first.0][first.1] = Twin::First(others);
    }
    found
}

/// The arcs bounding a region, as edges, sorted.
fn bounding(region: &Region, edges: &[EdgeId]) -> Vec<EdgeId> {
    let mut bounding: Vec<EdgeId> = region
        .cycles
        .iter()
        .flatten()
        .map(|&(arc, _)| edges[arc])
        .collect();
    bounding.sort();
    bounding
}

/// Whether two regions bounded by the same arcs run round them the same way,
/// which is whether their surfaces' normals point the same way: each keeps
/// its region on the left seen from its normal's side. Read on an arc each
/// runs along once; none where every arc is run along twice.
fn facing(one: &Region, on_one: &[EdgeId], other: &Region, on_other: &[EdgeId]) -> Option<bool> {
    let runs = |region: &Region, edges: &[EdgeId]| -> BTreeMap<EdgeId, Vec<bool>> {
        let mut runs: BTreeMap<EdgeId, Vec<bool>> = BTreeMap::new();
        for &(arc, forward) in region.cycles.iter().flatten() {
            runs.entry(edges[arc]).or_default().push(forward);
        }
        runs
    };
    let [one, other] = [runs(one, on_one), runs(other, on_other)];
    one.iter().find_map(
        |(edge, ways)| match (ways.as_slice(), other.get(edge)?.as_slice()) {
            ([way], [other]) => Some(way == other),
            _ => None,
        },
    )
}
