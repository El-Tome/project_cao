//! The names a body's edges and vertices answer to (#582), built from the
//! numbers of the faces they lie between and read off the topology and the
//! way the loops run, which a move, a turn or a change of size that keeps the
//! topology leaves alone. The parameters of a surface, read off the world's
//! axes, only part what nothing else tells apart.

use std::cmp::Ordering;
use std::collections::BTreeSet;

use glam::DVec2;
use serde::{Deserialize, Serialize};

use super::topology::{Body, EdgeId, FaceId, VertexId, ascending};

/// An edge, as the sides of the two faces using it, the smaller first, and
/// its rank among the `of` edges of the body between those same two sides.
/// A side is a face's numbers, each taken back to the number of the face it
/// was parted from.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct EdgeName {
    pub between: [Vec<u32>; 2],
    pub rank: u32,
    pub of: u32,
}

/// A vertex, as the sides of every face at it, ascending, and its rank among
/// the `of` vertices of the body at the same sides.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct VertexName {
    pub faces: Vec<Vec<u32>>,
    pub rank: u32,
    pub of: u32,
}

impl Body {
    /// The name of each edge, by the edge's rank, every number its own root.
    pub fn edge_names(&self) -> Vec<EdgeName> {
        self.edge_names_rooted(|number| number)
    }

    /// The name of each vertex, by the vertex's rank, every number its own
    /// root.
    pub fn vertex_names(&self) -> Vec<VertexName> {
        self.vertex_names_rooted(|number| number)
    }

    /// The name of each edge, by the edge's rank, every number taken to its
    /// `root`.
    pub(crate) fn edge_names_rooted(&self, root: impl Fn(u32) -> u32) -> Vec<EdgeName> {
        Reading::of(self, root).edge_names()
    }

    /// The name of each vertex, by the vertex's rank, every number taken to
    /// its `root`.
    pub(crate) fn vertex_names_rooted(&self, root: impl Fn(u32) -> u32) -> Vec<VertexName> {
        let reading = Reading::of(self, root);
        reading.vertex_names(&reading.edge_names())
    }
}

/// What the names are read from: the side of each face, and the faces at
/// each vertex.
struct Reading<'a> {
    body: &'a Body,
    sides: Vec<Vec<u32>>,
    at: Vec<BTreeSet<FaceId>>,
}

/// What ranks an edge among those between the same two sides: the sides met
/// at its ends seen from the first side, then the parameters of its middle
/// on the first side's surface.
type EdgeKey = ([Vec<Vec<u32>>; 2], DVec2);

/// What ranks a vertex among those at the same sides: the names of the edges
/// ending there, ascending, then its parameters on the surface of the first
/// of those sides.
type VertexKey<'n> = (Vec<&'n EdgeName>, DVec2);

impl<'a> Reading<'a> {
    fn of(body: &'a Body, root: impl Fn(u32) -> u32) -> Reading<'a> {
        let sides = body
            .face_ids()
            .map(|face| ascending(body.numbers(face).iter().map(|&number| root(number))))
            .collect();
        let mut at = vec![BTreeSet::new(); body.vertex_ids().count()];
        for face in body.face_ids() {
            let bounds = body.face(face).loops.iter().flatten();
            let ends = bounds.flat_map(|coedge| body.edge(coedge.edge).ends.into_iter().flatten());
            for vertex in ends.chain(body.face(face).poles.iter().copied()) {
                at[vertex.0 as usize].insert(face);
            }
        }
        Reading { body, sides, at }
    }

    fn side(&self, face: FaceId) -> &[u32] {
        &self.sides[face.0 as usize]
    }

    /// The sides of the faces at a vertex, ascending and each once.
    fn sides_at(&self, vertex: VertexId) -> Vec<Vec<u32>> {
        let sides: BTreeSet<&[u32]> = self.at[vertex.0 as usize]
            .iter()
            .map(|&face| self.side(face))
            .collect();
        sides.into_iter().map(<[u32]>::to_vec).collect()
    }

    fn edge_names(&self) -> Vec<EdgeName> {
        let read: Vec<([Vec<u32>; 2], EdgeKey)> =
            self.body.edge_ids().map(|edge| self.edge(edge)).collect();
        let ranks = ranked(&read, |one, other| {
            one.0.cmp(&other.0).then(lexical(one.1, other.1))
        });
        read.into_iter()
            .zip(ranks)
            .map(|((between, _), (rank, of))| EdgeName { between, rank, of })
            .collect()
    }

    /// The sides an edge lies between, and what ranks it among the edges
    /// between the same two, read along its use on the smaller side: the
    /// forward one when both are the same.
    fn edge(&self, edge: EdgeId) -> ([Vec<u32>; 2], EdgeKey) {
        let mut uses: Vec<(&[u32], bool, FaceId)> = self
            .body
            .uses(edge)
            .into_iter()
            .map(|(face, forward)| (self.side(face), !forward, face))
            .collect();
        uses.sort();
        let Some(&(first, backward, face)) = uses.first() else {
            return Default::default();
        };
        let second = uses.get(1).map_or(first, |&(side, _, _)| side);
        let between = [first.to_vec(), second.to_vec()];
        let around = |vertex: VertexId| -> Vec<Vec<u32>> {
            let mut sides = self.sides_at(vertex);
            sides.retain(|side| !between.contains(side));
            sides
        };
        let stretch = self.body.edge(edge);
        let ends = match stretch.ends {
            Some([from, to]) if backward => [around(to), around(from)],
            Some([from, to]) => [around(from), around(to)],
            None => Default::default(),
        };
        let halfway = self.body.point_on(edge, (stretch.from + stretch.to) / 2.0);
        let surface = self.body.surface(self.body.face(face).surface);
        (between, (ends, surface.parameters(halfway)))
    }

    fn vertex_names(&self, edge_names: &[EdgeName]) -> Vec<VertexName> {
        let mut ending: Vec<Vec<&EdgeName>> = vec![Vec::new(); self.at.len()];
        for edge in self.body.edge_ids() {
            let ends: BTreeSet<VertexId> =
                self.body.edge(edge).ends.into_iter().flatten().collect();
            for vertex in ends {
                ending[vertex.0 as usize].push(&edge_names[edge.0 as usize]);
            }
        }
        let read: Vec<(Vec<Vec<u32>>, VertexKey)> = self
            .body
            .vertex_ids()
            .zip(ending)
            .map(|(vertex, mut names)| {
                names.sort();
                (self.sides_at(vertex), (names, self.parameters(vertex)))
            })
            .collect();
        let ranks = ranked(&read, |one, other| {
            one.0.cmp(&other.0).then(lexical(one.1, other.1))
        });
        read.into_iter()
            .zip(ranks)
            .map(|((faces, _), (rank, of))| VertexName { faces, rank, of })
            .collect()
    }

    /// The parameters of a vertex on the surface of the first face at it of
    /// the smallest side.
    fn parameters(&self, vertex: VertexId) -> DVec2 {
        let first = self.at[vertex.0 as usize]
            .iter()
            .min_by(|&&one, &&other| self.side(one).cmp(self.side(other)));
        first.map_or(DVec2::ZERO, |&face| {
            let surface = self.body.surface(self.body.face(face).surface);
            surface.parameters(self.body.vertex(vertex).point)
        })
    }
}

/// For each item, its rank among the items of its group, by `order` and then
/// by its own place, and how many the group holds.
fn ranked<G: Ord, K>(items: &[(G, K)], order: impl Fn(&K, &K) -> Ordering) -> Vec<(u32, u32)> {
    let mut sorted: Vec<usize> = (0..items.len()).collect();
    sorted.sort_by(|&one, &other| {
        let ((group, key), (other_group, other_key)) = (&items[one], &items[other]);
        group.cmp(other_group).then_with(|| order(key, other_key))
    });
    let mut ranks = vec![(0, 0); items.len()];
    for run in sorted.chunk_by(|&one, &other| items[one].0 == items[other].0) {
        for (rank, &item) in run.iter().enumerate() {
            ranks[item] = (rank as u32, run.len() as u32);
        }
    }
    ranks
}

fn lexical(one: DVec2, other: DVec2) -> Ordering {
    one.x.total_cmp(&other.x).then(one.y.total_cmp(&other.y))
}
