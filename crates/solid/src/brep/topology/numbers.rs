//! The names a body's faces answer to, and the pieces a name is borne by.

use std::collections::BTreeSet;

use super::{Body, EdgeId, FaceId};

impl Body {
    /// The names a face answers to, ascending.
    pub fn numbers(&self, face: FaceId) -> &[u32] {
        &self.face(face).numbers
    }

    /// The same body, every face's numbers moved up by `by`: what a boolean's
    /// second operand is handed as, so that its names stand above the
    /// first's.
    pub fn renumbered(mut self, by: u32) -> Body {
        for face in &mut self.faces {
            for number in &mut face.numbers {
                *number += by;
            }
        }
        self
    }

    /// The faces answering to `number`, in groups that hold together through
    /// the edges they share: a face a cut parted is as many groups as pieces
    /// left apart. Each group in the body's order, the groups in the order of
    /// their first faces.
    pub fn pieces_of(&self, number: u32) -> Vec<Vec<FaceId>> {
        let named: Vec<FaceId> = self
            .face_ids()
            .filter(|&face| self.numbers(face).contains(&number))
            .collect();
        let edges: Vec<BTreeSet<EdgeId>> = named
            .iter()
            .map(|&face| {
                let loops = &self.face(face).loops;
                loops.iter().flatten().map(|coedge| coedge.edge).collect()
            })
            .collect();
        let mut group: Vec<usize> = (0..named.len()).collect();
        for (rank, own) in edges.iter().enumerate() {
            for (other, beside) in edges.iter().enumerate().skip(rank + 1) {
                if !own.is_disjoint(beside) {
                    let [one, other] = [root(&group, rank), root(&group, other)];
                    group[one.max(other)] = one.min(other);
                }
            }
        }
        let mut pieces: Vec<(usize, Vec<FaceId>)> = Vec::new();
        for (rank, &face) in named.iter().enumerate() {
            let first = root(&group, rank);
            match pieces.iter_mut().find(|(root, _)| *root == first) {
                Some((_, piece)) => piece.push(face),
                None => pieces.push((first, vec![face])),
            }
        }
        pieces.into_iter().map(|(_, piece)| piece).collect()
    }

    /// Each of `faces` answering to `to` where it answered to `from`.
    pub fn rename(&mut self, faces: &[FaceId], from: u32, to: u32) {
        for face in faces {
            let numbers = &mut self.faces[face.0 as usize].numbers;
            *numbers = ascending(
                numbers
                    .iter()
                    .map(|&number| if number == from { to } else { number }),
            );
        }
    }
}

/// Names gathered from several faces, ascending and each once.
pub(in crate::brep) fn ascending(numbers: impl IntoIterator<Item = u32>) -> Vec<u32> {
    let mut numbers: Vec<u32> = numbers.into_iter().collect();
    numbers.sort_unstable();
    numbers.dedup();
    numbers
}

fn root(group: &[usize], mut rank: usize) -> usize {
    while group[rank] != rank {
        rank = group[rank];
    }
    rank
}
