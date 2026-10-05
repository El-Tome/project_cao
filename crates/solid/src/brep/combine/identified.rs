//! Decision 6: arcs between the same two corners, on one surface, within the
//! tolerance of each other along their whole length, are one arc. A plane
//! touching a cylinder a hair from a corner leaves the circle of a cap
//! running within a hair of the plane's edge from the touch to the corner:
//! kept apart, the two would bound a sliver thinner than rounding.
//!
//! One curve is kept for the arc — a line before a circle before the curve
//! two cylinders meet along, then the one registered first — and the arc
//! lies on every surface either lay on, over its own stretch: the rest of
//! the kept curve does not lie on the other's surfaces. Two arcs whose
//! surfaces include two decided apart stay two.

use std::collections::BTreeMap;

use crate::brep::canonical::{Apart, parting};
use crate::brep::curve::Curve;
use crate::brep::topology::{Edge, SurfaceId, VertexId};

/// The arcs left once each group of arcs that are one is one, in the order
/// the first of each came, each with the surfaces it lies on.
pub(super) fn identified(
    edges: Vec<Edge>,
    lying: Vec<Vec<SurfaceId>>,
    curves: &[Curve],
    apart: &Apart,
    eps: f64,
) -> (Vec<Edge>, Vec<Vec<SurfaceId>>) {
    let mut between: BTreeMap<[VertexId; 2], Vec<usize>> = BTreeMap::new();
    for (rank, edge) in edges.iter().enumerate() {
        if let Some([from, to]) = edge.ends
            && from != to
        {
            between
                .entry([from.min(to), from.max(to)])
                .or_default()
                .push(rank);
        }
    }
    let mut group: Vec<usize> = (0..edges.len()).collect();
    for ranks in between.values() {
        for (index, &one) in ranks.iter().enumerate() {
            for &other in &ranks[index + 1..] {
                let arcs = [one, other].map(|rank| (&edges[rank], lying[rank].as_slice()));
                if one_arc(arcs, curves, apart, eps) {
                    let [one, other] = [root(&group, one), root(&group, other)];
                    group[one.max(other)] = one.min(other);
                }
            }
        }
    }
    let mut members: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for rank in 0..edges.len() {
        members.entry(root(&group, rank)).or_default().push(rank);
    }
    members
        .into_values()
        .map(|ranks| laid_once(&edges, &lying, &ranks, curves))
        .unzip()
}

fn root(group: &[usize], mut rank: usize) -> usize {
    while group[rank] != rank {
        rank = group[rank];
    }
    rank
}

/// Whether two arcs between the same corners, on different curves sharing a
/// surface, part by no more than the tolerance anywhere, their surfaces
/// together holding none decided apart.
fn one_arc(
    [(one, on_one), (other, on_other)]: [(&Edge, &[SurfaceId]); 2],
    curves: &[Curve],
    apart: &Apart,
    eps: f64,
) -> bool {
    let [first, second] = [one, other].map(|edge| edge.curve.0 as usize);
    first != second
        && on_one.iter().any(|surface| on_other.contains(surface))
        && !apart.across(on_one, on_other)
        && parting(
            &curves[first],
            [one.from, one.to],
            &curves[second],
            [other.from, other.to],
        ) <= eps
}

/// A group of arcs as one: the kept arc, lying on the surfaces of all.
fn laid_once(
    edges: &[Edge],
    lying: &[Vec<SurfaceId>],
    ranks: &[usize],
    curves: &[Curve],
) -> (Edge, Vec<SurfaceId>) {
    let kind = |curve: &Curve| match curve {
        Curve::Line(_) => 0,
        Curve::Circle(_) => 1,
        Curve::Meet(_) => 2,
    };
    let kept = ranks
        .iter()
        .map(|&rank| &edges[rank])
        .min_by_key(|edge| (kind(&curves[edge.curve.0 as usize]), edge.curve))
        .expect("a group holds an arc");
    let mut support: Vec<SurfaceId> = ranks
        .iter()
        .flat_map(|&rank| lying[rank].iter().copied())
        .collect();
    support.sort();
    support.dedup();
    (kept.clone(), support)
}
