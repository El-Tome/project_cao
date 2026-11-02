//! A ring torus's regions joined back across the cut of its chart (#623).
//!
//! The chart is a window one turn wide along the meridian, and its bottom
//! and top floors are one circle of the torus. A piece of surface the cut
//! runs through is read as a region bounded by the bottom floor and another
//! bounded by the top one along the same stretch — or as one region bounded
//! by both, where it reaches round the tube — and neither floor is an edge.
//! So the regions using the two floors along one stretch are one region, and
//! its loops are walked again with the floors left out: an arc reaching a
//! tip on one floor is followed by the arc leaving the tip standing for the
//! same place on the other, and the two arcs of an edge the cut parts are
//! one use of it. A floor no meridian reaches is whole, and the regions
//! along the two whole floors are one band the cut runs through — which
//! `combine/seamed.rs` moves the cut out of where it can.

use std::collections::{BTreeMap, BTreeSet};

use crate::brep::Declined;
use crate::brep::overlay::{Arc, Overlay, Region};
use crate::brep::topology::EdgeId;

/// `overlay`'s regions with those the cut parts joined, their cycles the
/// edges' arcs alone, each loop closed. `edges` names the edge of each of
/// the first arcs, the floors' arcs coming after them; `twins` takes each tip
/// of the cut to the one on the other floor at the same angle.
pub(super) fn glued(
    overlay: Overlay,
    arcs: &[Arc],
    edges: &[EdgeId],
    twins: &BTreeMap<usize, usize>,
) -> Result<Overlay, Declined> {
    let floors = edges.len();
    let mut joined: Vec<usize> = (0..overlay.regions.len()).collect();
    let mut users: BTreeMap<usize, usize> = BTreeMap::new();
    for (rank, region) in overlay.regions.iter().enumerate() {
        if region.unbounded {
            continue;
        }
        for &(arc, _) in region.cycles.iter().flatten() {
            if arc >= floors {
                users.insert(arc, rank);
            }
        }
    }
    let by_ends: BTreeMap<[usize; 2], usize> = (floors..arcs.len())
        .filter_map(|arc| Some((arcs[arc].ends?, arc)))
        .collect();
    let whole: Vec<usize> = users
        .iter()
        .filter(|(arc, _)| arcs[**arc].ends.is_none())
        .map(|(_, user)| *user)
        .collect();
    if let [one, other] = whole[..] {
        let (one, other) = (root(&mut joined, one), root(&mut joined, other));
        joined[one.max(other)] = one.min(other);
    }
    for (&arc, &user) in &users {
        let Some([from, to]) = arcs[arc].ends else {
            continue;
        };
        let (Some(&from), Some(&to)) = (twins.get(&from), twins.get(&to)) else {
            return Err(Declined::Unverified);
        };
        let partner = by_ends.get(&[from, to]).ok_or(Declined::Unverified)?;
        let other = *users.get(partner).ok_or(Declined::Unverified)?;
        let (one, other) = (root(&mut joined, user), root(&mut joined, other));
        joined[one.max(other)] = one.min(other);
    }
    let mut groups: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for rank in 0..overlay.regions.len() {
        let root = root(&mut joined, rank);
        groups.entry(root).or_default().push(rank);
    }
    let mut regions = Vec::with_capacity(groups.len());
    for parts in groups.values() {
        let first = &overlay.regions[parts[0]];
        if first.unbounded {
            regions.push(Region {
                cycles: Vec::new(),
                ..first.clone()
            });
            continue;
        }
        let cycles: Vec<&Vec<(usize, bool)>> = parts
            .iter()
            .flat_map(|&part| &overlay.regions[part].cycles)
            .collect();
        regions.push(Region {
            cycles: walked(&cycles, arcs, edges, twins)?,
            ..first.clone()
        });
    }
    Ok(Overlay { regions })
}

/// The loops `cycles` make once their floors are left out: each arc of an
/// edge followed by the next one in its cycle, or, where that is a floor, by
/// the arc leaving the twin of the tip it reaches.
fn walked(
    cycles: &[&Vec<(usize, bool)>],
    arcs: &[Arc],
    edges: &[EdgeId],
    twins: &BTreeMap<usize, usize>,
) -> Result<Vec<Vec<(usize, bool)>>, Declined> {
    let floors = edges.len();
    let ends = |(arc, forward): (usize, bool)| {
        arcs[arc]
            .ends
            .map(|[from, to]| if forward { [from, to] } else { [to, from] })
    };
    let mut order = Vec::new();
    let mut alone = Vec::new();
    let mut next: BTreeMap<(usize, bool), (usize, bool)> = BTreeMap::new();
    let mut reaching: Vec<(usize, bool)> = Vec::new();
    for cycle in cycles {
        for (at, &run) in cycle.iter().enumerate() {
            if run.0 >= floors {
                continue;
            }
            if ends(run).is_none() {
                alone.push(vec![run]);
                continue;
            }
            order.push(run);
            let after = cycle[(at + 1) % cycle.len()];
            if after.0 < floors {
                next.insert(run, after);
            } else {
                reaching.push(run);
            }
        }
    }
    let mut leaving: BTreeMap<usize, Vec<(usize, bool)>> = BTreeMap::new();
    for &run in &order {
        if let Some([from, _]) = ends(run) {
            leaving.entry(from).or_default().push(run);
        }
    }
    let mut continued = BTreeSet::new();
    for run in reaching {
        let [_, tip] = ends(run).ok_or(Declined::Unverified)?;
        let twin = twins.get(&tip).ok_or(Declined::Unverified)?;
        let [after] = leaving.get(twin).map_or(&[][..], Vec::as_slice) else {
            return Err(Declined::Unverified);
        };
        if edges[after.0] == edges[run.0] && after.1 == run.1 {
            continued.insert(*after);
        }
        next.insert(run, *after);
    }
    let mut seen = BTreeSet::new();
    let mut loops = alone;
    for &start in &order {
        if seen.contains(&start) {
            continue;
        }
        let mut lap = Vec::new();
        let mut run = start;
        loop {
            if !seen.insert(run) {
                return Err(Declined::Unverified);
            }
            lap.push(run);
            run = *next.get(&run).ok_or(Declined::Unverified)?;
            if run == start {
                break;
            }
        }
        let kept: Vec<(usize, bool)> = lap
            .iter()
            .copied()
            .filter(|run| !continued.contains(run))
            .collect();
        loops.push(if kept.is_empty() { vec![lap[0]] } else { kept });
    }
    Ok(loops)
}

fn root(joined: &mut [usize], mut rank: usize) -> usize {
    while joined[rank] != rank {
        joined[rank] = joined[joined[rank]];
        rank = joined[rank];
    }
    rank
}
