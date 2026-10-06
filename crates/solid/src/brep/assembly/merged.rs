//! Step 8 of `docs/exact-kernel.md`: faces on one surface with one side are
//! merged across an arc nothing else uses, and two edges on one curve
//! meeting at a vertex nothing else uses become one — a closed curve whose
//! last vertex goes is whole again.
//!
//! A seam is what joining two blocks with a side in one plane leaves: each
//! block's edge inside the other's face. Kept, it is an edge of one surface
//! alone that the next operation reads again, a hair off whatever a later,
//! larger tolerance takes the surface for, and its ends corners a hair off
//! the corners beside them.
//!
//! A face is taken out of a loop the way a half-edge structure removes an
//! edge: a use followed by a seam's use is followed instead by what follows
//! the seam's other use, so the order round every vertex is kept without
//! measuring an angle.

use std::collections::BTreeMap;

use crate::brep::curve::Curve;
use crate::brep::topology::{Coedge, Edge, EdgeId, Face, SurfaceId, Vertex, VertexId, ascending};

/// A use of an edge: the face, the loop, the rank in the loop.
type Use = (usize, usize, usize);

/// The faces with every seam taken out, and the edges each pair meeting at a
/// vertex nothing else uses is joined into, appended to `edges`.
pub(super) fn merged(
    edges: &mut Vec<Edge>,
    curves: &[Curve],
    vertices: &[Vertex],
    faces: Vec<Face>,
) -> Vec<Face> {
    let mut faces = unseamed(faces);
    while let Some((at, [one, other])) = joint(edges, curves, vertices, &faces) {
        let joined = joined(edges, curves, at, one, other);
        let rank = EdgeId(edges.len() as u32);
        edges.push(joined);
        for face in &mut faces {
            for lap in &mut face.loops {
                rejoined(lap, one, other, rank);
            }
        }
    }
    faces
}

/// Every use of every edge, in the faces' order.
fn uses(faces: &[Face]) -> BTreeMap<EdgeId, Vec<Use>> {
    let mut uses: BTreeMap<EdgeId, Vec<Use>> = BTreeMap::new();
    for (face, laid) in faces.iter().enumerate() {
        for (lap, coedges) in laid.loops.iter().enumerate() {
            for (rank, coedge) in coedges.iter().enumerate() {
                uses.entry(coedge.edge).or_default().push((face, lap, rank));
            }
        }
    }
    uses
}

fn coedge(faces: &[Face], (face, lap, rank): Use) -> Coedge {
    faces[face].loops[lap][rank]
}

/// The faces, each group joined across seams one face, in the place of its
/// first.
fn unseamed(faces: Vec<Face>) -> Vec<Face> {
    let mut twin: BTreeMap<Use, Use> = BTreeMap::new();
    let mut group: Vec<usize> = (0..faces.len()).collect();
    for list in uses(&faces).values() {
        let &[one, other] = list.as_slice() else {
            continue;
        };
        let [first, second] = [one, other].map(|(face, ..)| &faces[face]);
        if first.surface != second.surface
            || first.flipped != second.flipped
            || coedge(&faces, one).forward == coedge(&faces, other).forward
        {
            continue;
        }
        twin.insert(one, other);
        twin.insert(other, one);
        let [one, other] = [root(&group, one.0), root(&group, other.0)];
        group[one.max(other)] = one.min(other);
    }
    if twin.is_empty() {
        return faces;
    }
    let mut members: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for face in 0..faces.len() {
        members.entry(root(&group, face)).or_default().push(face);
    }
    members
        .values()
        .flat_map(|members| {
            let seamed = members.len() > 1 || twin.keys().any(|seam| seam.0 == members[0]);
            match seamed.then(|| traced(&faces, members, &twin)).flatten() {
                Some(merged) => vec![merged],
                None => members.iter().map(|&face| faces[face].clone()).collect(),
            }
        })
        .collect()
}

fn root(group: &[usize], mut rank: usize) -> usize {
    while group[rank] != rank {
        rank = group[rank];
    }
    rank
}

/// The face a group makes once its seams are out, its loops traced again;
/// none where they do not close, and the group is left as it was.
fn traced(faces: &[Face], members: &[usize], twin: &BTreeMap<Use, Use>) -> Option<Face> {
    let first = &faces[members[0]];
    let all: Vec<Use> = members
        .iter()
        .flat_map(|&face| {
            faces[face]
                .loops
                .iter()
                .enumerate()
                .flat_map(move |(lap, coedges)| {
                    (0..coedges.len()).map(move |rank| (face, lap, rank))
                })
        })
        .collect();
    let next = |(face, lap, rank): Use| (face, lap, (rank + 1) % faces[face].loops[lap].len());
    let mut seen: BTreeMap<Use, bool> = BTreeMap::new();
    let mut loops = Vec::new();
    for &start in &all {
        if twin.contains_key(&start) || seen.contains_key(&start) {
            continue;
        }
        let mut lap = Vec::new();
        let mut at = start;
        loop {
            if seen.insert(at, true).is_some() {
                return None;
            }
            lap.push(coedge(faces, at));
            let mut after = next(at);
            let mut jumps = 0;
            while let Some(&other) = twin.get(&after) {
                after = next(other);
                jumps += 1;
                if jumps > all.len() {
                    return None;
                }
            }
            at = after;
            if at == start {
                break;
            }
        }
        loops.push(lap);
    }
    let numbers = ascending(
        members
            .iter()
            .flat_map(|&face| faces[face].numbers.iter().copied()),
    );
    Some(Face {
        surface: first.surface,
        flipped: first.flipped,
        loops,
        numbers,
        apex: None,
    })
}

/// The first vertex nothing reaches but two edges on one curve, one ending
/// there and the other starting there, every use of the one followed by a
/// use of the other — or one edge of a closed curve at both its ends — and
/// lying on no surface but those of the faces beside them: a corner where
/// the curve touches another wall is where that wall's triangles take a
/// sample, a pocket's rim touching a lying wall at one place.
fn joint(
    edges: &[Edge],
    curves: &[Curve],
    vertices: &[Vertex],
    faces: &[Face],
) -> Option<(VertexId, [EdgeId; 2])> {
    let used = uses(faces);
    let mut ends: BTreeMap<VertexId, Vec<(EdgeId, usize)>> = BTreeMap::new();
    for &edge in used.keys() {
        if let Some(pair) = edges[edge.0 as usize].ends {
            for (side, vertex) in pair.into_iter().enumerate() {
                ends.entry(vertex).or_default().push((edge, side));
            }
        }
    }
    ends.into_iter().find_map(|(vertex, list)| {
        let &[(one, one_side), (other, other_side)] = list.as_slice() else {
            return None;
        };
        let [first, second] = [one, other].map(|edge| &edges[edge.0 as usize]);
        let beside: Vec<SurfaceId> = [one, other]
            .iter()
            .flat_map(|edge| used[edge].iter().map(|&(face, ..)| faces[face].surface))
            .collect();
        if first.curve != second.curve
            || vertices[vertex.0 as usize].on.iter().any(|surface| {
                !beside.contains(surface) && faces.iter().any(|face| face.surface == *surface)
            })
        {
            return None;
        }
        let period = curves[first.curve.0 as usize].period();
        if one == other {
            let whole = period.is_some_and(|period| close(first.to - first.from, period, period));
            return whole.then_some((vertex, [one, one]));
        }
        let [ending, starting] = match (one_side, other_side) {
            (1, 0) => [one, other],
            (0, 1) => [other, one],
            _ => return None,
        };
        let [before, after] = [ending, starting].map(|edge| &edges[edge.0 as usize]);
        let shift = period.map_or(0.0, |period| {
            ((before.to - after.from) / period).round() * period
        });
        (close(before.to, after.from + shift, period.unwrap_or(1.0))
            && follow(faces, &used, ending, starting))
        .then_some((vertex, [ending, starting]))
    })
}

/// Whether two parameters are one, but for rounding at the scale of `span`.
fn close(one: f64, other: f64, span: f64) -> bool {
    (one - other).abs() <= 1e-12 * span.abs().max(one.abs()).max(1.0)
}

/// Whether every use of `ending` along it is followed by a use of
/// `starting` along it, and every use against `starting` by one against
/// `ending`, as many each way.
fn follow(
    faces: &[Face],
    used: &BTreeMap<EdgeId, Vec<Use>>,
    ending: EdgeId,
    starting: EdgeId,
) -> bool {
    let next = |(face, lap, rank): Use| {
        let coedges = &faces[face].loops[lap];
        coedges[(rank + 1) % coedges.len()]
    };
    let way = |edge: EdgeId, forward: bool| -> Vec<Use> {
        used[&edge]
            .iter()
            .copied()
            .filter(|&at| coedge(faces, at).forward == forward)
            .collect()
    };
    [(ending, starting, true), (starting, ending, false)]
        .into_iter()
        .all(|(first, second, forward)| {
            let firsts = way(first, forward);
            firsts.len() == way(second, forward).len()
                && firsts.iter().all(|&at| {
                    let after = next(at);
                    after.edge == second && after.forward == forward
                })
        })
}

/// The edge two edges of one curve meeting at `at` make, or the one edge
/// both of whose ends are `at`, whole once that vertex goes.
fn joined(edges: &[Edge], curves: &[Curve], at: VertexId, one: EdgeId, other: EdgeId) -> Edge {
    let [before, after] = [one, other].map(|edge| &edges[edge.0 as usize]);
    let period = curves[before.curve.0 as usize].period();
    if one == other {
        return Edge {
            curve: before.curve,
            ends: None,
            from: before.from,
            to: before.to,
        };
    }
    let shift = period.map_or(0.0, |period| {
        ((before.to - after.from) / period).round() * period
    });
    let [start, _] = before.ends.expect("a joined edge has ends");
    let [_, end] = after.ends.expect("a joined edge has ends");
    debug_assert!(start != at && end != at);
    Edge {
        curve: before.curve,
        ends: Some([start, end]),
        from: before.from,
        to: after.to + shift,
    }
}

/// A loop with each use of `one` then `other` along them, and of `other`
/// then `one` against them, made one use of `joined`; a loop using a closed
/// edge `one` alone uses the whole curve instead.
fn rejoined(lap: &mut Vec<Coedge>, one: EdgeId, other: EdgeId, joined: EdgeId) {
    if one == other {
        for coedge in lap.iter_mut().filter(|coedge| coedge.edge == one) {
            coedge.edge = joined;
        }
        return;
    }
    loop {
        let count = lap.len();
        let found = (0..count).find(|&rank| {
            let [first, second] = [lap[rank], lap[(rank + 1) % count]];
            first.forward && second.forward && first.edge == one && second.edge == other
                || !first.forward && !second.forward && first.edge == other && second.edge == one
        });
        let Some(rank) = found else {
            return;
        };
        let forward = lap[rank].forward;
        let following = (rank + 1) % count;
        lap[rank] = Coedge {
            edge: joined,
            forward,
        };
        lap.remove(following);
    }
}
