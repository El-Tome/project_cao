//! The names a body's edges and vertices answer to (#582), built from the
//! numbers of the faces they lie between and from what meets them at their
//! ends, read along the loops: what a move, a turn or a change of size that
//! keeps the topology leaves alone. Where nothing topological tells two
//! apart, where they lie in the world does, within the body's tolerance.

mod found;

use std::cmp::Ordering;
use std::collections::BTreeMap;

use glam::DVec3;
use serde::{Deserialize, Serialize};

use super::curve::Curve;
use super::topology::{Body, EdgeId, FaceId, VertexId, ascending};

/// The sides of the faces met at each end of an edge, other than its own two,
/// from where it starts to where it ends along its first side's loop.
type Ends = [Vec<Vec<u32>>; 2];

/// An edge, as the sides of the two faces using it, the smaller first; what
/// meets it at its `ends`, both empty for a whole closed edge; and its rank
/// among the `of` edges alike in both, which only where they lie tells
/// apart. A side is a face's numbers, each taken back to the number of the
/// face it was parted from.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct EdgeName {
    pub between: [Vec<u32>; 2],
    pub ends: [Vec<Vec<u32>>; 2],
    pub rank: u32,
    pub of: u32,
}

/// What a name no longer finds.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Lost {
    /// No edge is the one named: its two sides no longer meet, or meet with
    /// both its ends moved, or fewer edges are alike in what it recorded than
    /// it was ranked among, or more than one keeps an end of it.
    Gone,
    /// It went on in pieces: more edges are alike in what it recorded than
    /// it was ranked among, or one edge starts where it started and another
    /// ends where it ended. `now` counts the edges between its two sides,
    /// not its pieces alone.
    Split { now: u32 },
}

/// A vertex, as the sides of every face at it, ascending; where another
/// vertex is at the same sides, the names of the `edges` ending there,
/// ascending; and its rank among the `of` vertices alike in both, which only
/// where they lie tells apart.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct VertexName {
    pub faces: Vec<Vec<u32>>,
    pub edges: Option<Vec<EdgeName>>,
    pub rank: u32,
    pub of: u32,
}

/// How many points along a whole closed edge where it lies is read from.
const STEPS: u32 = 16;

impl Body {
    /// The name of each edge, by the edge's rank, every number its own root.
    pub fn edge_names(&self) -> Vec<EdgeName> {
        Reading::of(self, |number| number).edge_names()
    }

    /// The name of each vertex, by the vertex's rank, every number its own
    /// root.
    pub fn vertex_names(&self) -> Vec<VertexName> {
        let (_, vertices) = self.names_rooted(|number| number);
        vertices
    }

    /// The name of each edge and of each vertex, by rank, every number taken
    /// to its `root`.
    pub(crate) fn names_rooted(
        &self,
        root: impl Fn(u32) -> u32,
    ) -> (Vec<EdgeName>, Vec<VertexName>) {
        let reading = Reading::of(self, root);
        let edges = reading.edge_names();
        let vertices = reading.vertex_names(&edges);
        (edges, vertices)
    }
}

/// What the names are read from, in one pass over the faces' loops: the side
/// of each face, the faces at each vertex, and the faces using each edge.
struct Reading<'a> {
    body: &'a Body,
    sides: Vec<Vec<u32>>,
    at: Vec<Vec<FaceId>>,
    uses: Vec<Vec<(FaceId, bool)>>,
}

/// An edge as its name is read: the sides it lies between, what meets it at
/// its ends, and where it lies.
struct EdgeRead {
    between: [Vec<u32>; 2],
    ends: Ends,
    point: DVec3,
}

impl<'a> Reading<'a> {
    fn of(body: &'a Body, root: impl Fn(u32) -> u32) -> Reading<'a> {
        let sides = body
            .face_ids()
            .map(|face| ascending(body.numbers(face).iter().map(|&number| root(number))))
            .collect();
        let mut at = vec![Vec::new(); body.vertex_ids().count()];
        let mut uses = vec![Vec::new(); body.edge_ids().count()];
        for face in body.face_ids() {
            for coedge in body.face(face).loops.iter().flatten() {
                uses[coedge.edge.0 as usize].push((face, coedge.forward));
                for vertex in body.edge(coedge.edge).ends.into_iter().flatten() {
                    at[vertex.0 as usize].push(face);
                }
            }
            for pole in &body.face(face).poles {
                at[pole.0 as usize].push(face);
            }
        }
        for faces in &mut at {
            faces.dedup();
        }
        Reading {
            body,
            sides,
            at,
            uses,
        }
    }

    fn side(&self, face: FaceId) -> &[u32] {
        &self.sides[face.0 as usize]
    }

    /// The sides of the faces at a vertex, ascending and each once.
    fn sides_at(&self, vertex: VertexId) -> Vec<Vec<u32>> {
        let at = self.at[vertex.0 as usize].iter();
        let mut sides: Vec<Vec<u32>> = at.map(|&face| self.side(face).to_vec()).collect();
        sides.sort();
        sides.dedup();
        sides
    }

    /// An edge read along its use on the smaller side: the forward one when
    /// both are the same.
    fn edge(&self, edge: EdgeId) -> EdgeRead {
        let mut uses: Vec<(&[u32], bool)> = self.uses[edge.0 as usize]
            .iter()
            .map(|&(face, forward)| (self.side(face), !forward))
            .collect();
        uses.sort();
        let (first, backward) = uses.first().copied().unwrap_or_default();
        let second = uses.get(1).map_or(first, |&(side, _)| side);
        let between = [first.to_vec(), second.to_vec()];
        let around = |vertex: VertexId| {
            let mut sides = self.sides_at(vertex);
            sides.retain(|side| !between.contains(side));
            sides
        };
        let ends = match self.body.edge(edge).ends {
            Some([from, to]) if backward => [around(to), around(from)],
            Some([from, to]) => [around(from), around(to)],
            None => Default::default(),
        };
        EdgeRead {
            between,
            ends,
            point: self.place(edge),
        }
    }

    /// Where an edge lies, read off its curve and its vertices alone, not
    /// off the stretch it was built with: halfway between its vertices, or
    /// for a whole closed edge the mean of points at even steps over one
    /// period from the curve's own origin.
    fn place(&self, edge: EdgeId) -> DVec3 {
        let stretch = self.body.edge(edge);
        let curve = self.body.curve(stretch.curve);
        if let Some(ends) = stretch.ends {
            let [from, to] =
                ends.map(|end| self.body.parameter_on(edge, self.body.vertex(end).point));
            return curve.point((from + to) / 2.0);
        }
        match curve.period() {
            Some(period) => mean_along(curve, period),
            None => curve.point((stretch.from + stretch.to) / 2.0),
        }
    }

    fn edge_names(&self) -> Vec<EdgeName> {
        let read: Vec<EdgeRead> = self.body.edge_ids().map(|edge| self.edge(edge)).collect();
        let alike: Vec<_> = read
            .iter()
            .map(|each| (&each.between, &each.ends))
            .collect();
        let ranks = ranked(&alike, |one, other| {
            self.placed(read[one].point, read[other].point)
        });
        alike
            .into_iter()
            .zip(ranks)
            .map(|((between, ends), (rank, of))| EdgeName {
                between: between.clone(),
                ends: ends.clone(),
                rank,
                of,
            })
            .collect()
    }

    fn vertex_names(&self, edge_names: &[EdgeName]) -> Vec<VertexName> {
        let mut ending: Vec<Vec<&EdgeName>> = vec![Vec::new(); self.at.len()];
        for edge in self.body.edge_ids() {
            if let Some([from, to]) = self.body.edge(edge).ends {
                let name = &edge_names[edge.0 as usize];
                ending[from.0 as usize].push(name);
                if to != from {
                    ending[to.0 as usize].push(name);
                }
            }
        }
        let faces: Vec<Vec<Vec<u32>>> = self
            .body
            .vertex_ids()
            .map(|vertex| self.sides_at(vertex))
            .collect();
        let shared = counted(faces.iter());
        let alike: Vec<_> = faces
            .iter()
            .zip(ending)
            .map(|(sides, mut names)| {
                names.sort();
                let edges = (shared[sides] > 1).then(|| names.into_iter().cloned().collect());
                (sides, edges)
            })
            .collect();
        let point = |vertex: usize| self.body.vertex(VertexId(vertex as u32)).point;
        let ranks = ranked(&alike, |one, other| self.placed(point(one), point(other)));
        alike
            .into_iter()
            .zip(ranks)
            .map(|((faces, edges), (rank, of))| VertexName {
                faces: faces.clone(),
                edges,
                rank,
                of,
            })
            .collect()
    }

    /// Two points compared coordinate by coordinate in the world, one
    /// within the body's tolerance of the other counting as equal before
    /// the next is compared.
    fn placed(&self, one: DVec3, other: DVec3) -> Ordering {
        let eps = self.body.scale().eps();
        (0..3)
            .map(|axis| (one[axis], other[axis]))
            .find(|(one, other)| (one - other).abs() > eps)
            .map_or(Ordering::Equal, |(one, other)| one.total_cmp(&other))
    }
}

/// The mean of points along a closed curve at even steps over one period,
/// from its parameter nought.
fn mean_along(curve: &Curve, period: f64) -> DVec3 {
    let step = period / f64::from(STEPS);
    let points = (0..STEPS).map(|index| curve.point(step * f64::from(index)));
    points.sum::<DVec3>() / f64::from(STEPS)
}

/// How many times each key comes.
fn counted<K: Ord>(keys: impl Iterator<Item = K>) -> BTreeMap<K, u32> {
    let mut counts = BTreeMap::new();
    for key in keys {
        *counts.entry(key).or_insert(0) += 1;
    }
    counts
}

/// For each item, its rank among the items alike, by `order` and then by its
/// own place, and how many are alike.
fn ranked<G: Ord>(alike: &[G], order: impl Fn(usize, usize) -> Ordering) -> Vec<(u32, u32)> {
    let mut sorted: Vec<usize> = (0..alike.len()).collect();
    sorted.sort_by(|&one, &other| alike[one].cmp(&alike[other]));
    let mut ranks = vec![(0, 0); alike.len()];
    for run in sorted.chunk_by_mut(|&one, &other| alike[one] == alike[other]) {
        ordered(run, &order);
        for (rank, &item) in run.iter().enumerate() {
            ranks[item] = (rank as u32, run.len() as u32);
        }
    }
    ranks
}

/// The items put in `order`, the first of two it holds equal kept first.
fn ordered(items: &mut [usize], order: impl Fn(usize, usize) -> Ordering) {
    // By insertion rather than `sort_by`: an order read to within a
    // tolerance is not transitive, which `sort_by` may panic on.
    for next in 1..items.len() {
        let mut at = next;
        while at > 0 && order(items[at], items[at - 1]).is_lt() {
            items.swap(at, at - 1);
            at -= 1;
        }
    }
}
