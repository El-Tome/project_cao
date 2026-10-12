//! Decision 6: arcs between the same two corners, on one surface, within the
//! tolerance of each other along their whole length, are one arc. A plane
//! touching a cylinder a hair from a corner leaves the circle of a cap
//! running within a hair of the plane's edge from the touch to the corner:
//! kept apart, the two would bound a sliver thinner than rounding.
//!
//! One curve is kept for the arc — a line before a circle before a conic
//! before the curve two cylinders meet along, then the one registered first
//! — and the arc lies on every surface either lay on, over its own stretch:
//! the rest of the kept curve does not lie on the other's surfaces. Two arcs
//! whose surfaces include two decided apart stay two.
//!
//! A conic goes before a line it is one arc with where its wall would not
//! see the line (#552): a wall sees a line only as the segment between its
//! ends in the wall's own parameters, which bulges off the line as the wall
//! curves — a frustum's rim, cut a hair inside, two tolerances off the
//! chord the plane leaves there — while a plane sees the conic as that very
//! chord. Where the wall sees the line, the line is kept, and the curve the
//! next operation crosses is still the one its planes cross. That conic
//! goes before a circle the arc is one with too: the circle's arc leans on
//! the plane the line and the conic lie on, which sees it only as its
//! chord, as far off it as the line is off the wall (5542125645).

use std::collections::BTreeMap;
use std::f64::consts::TAU;

use glam::DVec2;

use crate::brep::canonical::{Apart, parting};
use crate::brep::conic::{Conic, Round};
use crate::brep::curve::{Curve, Line};
use crate::brep::scale::Scale;
use crate::brep::surface::Surface;
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
        .map(|ranks| laid_once(&edges, &lying, &ranks, curves, eps))
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
/// together holding none decided apart. Two arcs both lying on two surfaces
/// decided to touch are where a third surface crosses their band, the
/// curve it cuts on each: each stands within the tolerance of both, and
/// they part by as much as the tolerance over the sine of the angle the
/// third crosses at, up to the hair — a cone's wall crossing a wedge's
/// wall a hair inside a block's side, at 45° (5550223366).
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
        ) <= if banded(on_one, on_other, apart) {
            Scale::HAIR * eps
        } else {
            eps
        }
}

/// Whether two surfaces decided to touch are among those both arcs lie on.
fn banded(on_one: &[SurfaceId], on_other: &[SurfaceId], apart: &Apart) -> bool {
    let shared: Vec<SurfaceId> = on_one
        .iter()
        .copied()
        .filter(|surface| on_other.contains(surface))
        .collect();
    shared.iter().enumerate().any(|(index, &one)| {
        shared[index + 1..]
            .iter()
            .any(|&other| apart.touch(one, other))
    })
}

/// A group of arcs as one: the kept arc, lying on the surfaces of all.
fn laid_once(
    edges: &[Edge],
    lying: &[Vec<SurfaceId>],
    ranks: &[usize],
    curves: &[Curve],
    eps: f64,
) -> (Edge, Vec<SurfaceId>) {
    let conics: Vec<&Conic> = ranks
        .iter()
        .filter_map(|rank| match &curves[edges[*rank].curve.0 as usize] {
            Curve::Conic(conic) => Some(conic),
            _ => None,
        })
        .collect();
    let unseen = |edge: &Edge| match &curves[edge.curve.0 as usize] {
        Curve::Line(line) => conics
            .iter()
            .any(|conic| !seen_straight(conic, line, edge, eps)),
        _ => false,
    };
    let bent = ranks.iter().any(|&rank| unseen(&edges[rank]));
    let kind = |edge: &Edge| match &curves[edge.curve.0 as usize] {
        Curve::Line(_) if unseen(edge) => 4,
        Curve::Line(_) => 0,
        Curve::Conic(_) if bent => 1,
        Curve::Circle(_) => 2,
        Curve::Conic(_) => 3,
        Curve::Meet(_) => 5,
    };
    let kept = ranks
        .iter()
        .map(|&rank| &edges[rank])
        .min_by_key(|edge| (kind(edge), edge.curve))
        .expect("a group holds an arc");
    let mut support: Vec<SurfaceId> = ranks
        .iter()
        .flat_map(|&rank| lying[rank].iter().copied())
        .collect();
    support.sort();
    support.dedup();
    (kept.clone(), support)
}

/// Whether a conic's wall sees a line's arc as that line: the place the
/// wall's own parameters put half way between the arc's ends stands within
/// the tolerance of the arc's middle.
fn seen_straight(conic: &Conic, line: &Line, edge: &Edge, eps: f64) -> bool {
    let wall = match conic.round {
        Round::Cylinder(cylinder) => Surface::Cylinder(cylinder),
        Round::Cone(cone) => Surface::Cone(cone),
        Round::Elliptic(wall) => Surface::Elliptic(wall),
    };
    let [from, to] = [edge.from, edge.to].map(|at| wall.parameters(line.point(at)));
    let turn = to.x - from.x;
    let to = DVec2::new(from.x + turn - TAU * (turn / TAU).round(), to.y);
    let middle = wall.point((from + to) / 2.0);
    middle.distance(line.point((edge.from + edge.to) / 2.0)) <= eps
}
