//! The edge a name finds (#582), as a face is found: by a number its sides
//! share with the faces now, its ends answering as its sides do, and on one
//! end kept when a cut has moved the other. Both ends moved, it is gone; in
//! more pieces than there were edges between its faces, it is split; with
//! fewer, its pieces joined back, it is gone, rather than found on an edge
//! another piece would find as well.

use std::cmp::Ordering;

use super::{EdgeName, EdgeRead, Lost, Reading, ordered};
use crate::brep::topology::{Body, EdgeId};

impl Body {
    /// The edge a name finds in the body, every number its own root, or
    /// what became of it.
    pub fn find(&self, name: &EdgeName) -> Result<EdgeId, Lost> {
        self.find_rooted(name, |number| number)
    }

    /// The edge a name finds in the body, every number taken to its `root`.
    pub(crate) fn find_rooted(
        &self,
        name: &EdgeName,
        root: impl Fn(u32) -> u32,
    ) -> Result<EdgeId, Lost> {
        let reading = Reading::of(self, root);
        let siblings: Vec<(EdgeId, EdgeRead)> = self
            .edge_ids()
            .filter_map(|edge| seen_from(name, reading.edge(edge)).map(|read| (edge, read)))
            .collect();
        let split = Err(Lost::Split {
            now: siblings.len() as u32,
        });
        let [start, end] = &name.ends;
        let starts = |sibling: &usize| end_answers(start, &siblings[*sibling].1.ends[0]);
        let finishes = |sibling: &usize| end_answers(end, &siblings[*sibling].1.ends[1]);
        let mut ties: Vec<usize> = (0..siblings.len())
            .filter(|sibling| starts(sibling) && finishes(sibling))
            .collect();
        if ties.len() as u32 == name.of {
            ordered(&mut ties, |one, other| {
                reading.placed(siblings[one].1.place, siblings[other].1.place)
            });
            let found = ties.get(name.rank as usize);
            return found.map(|&sibling| siblings[sibling].0).ok_or(Lost::Gone);
        }
        if ties.len() as u32 > name.of {
            return split;
        }
        if !ties.is_empty() {
            return Err(Lost::Gone);
        }
        match (siblings.len() as u32).cmp(&name.among) {
            Ordering::Greater => return split,
            Ordering::Less => return Err(Lost::Gone),
            Ordering::Equal => {}
        }
        let starting: Vec<usize> = (0..siblings.len()).filter(starts).collect();
        let ending: Vec<usize> = (0..siblings.len()).filter(finishes).collect();
        if starting
            .iter()
            .any(|one| ending.iter().any(|other| other != one))
        {
            return split;
        }
        match [starting, ending].concat()[..] {
            [only] => Ok(siblings[only].0),
            _ => Err(Lost::Gone),
        }
    }
}

/// The edge read from the side the name was read from, when its two sides
/// answer to the name's: its ends swapped when they answer the other way
/// round, which reads it along its other face's loop.
fn seen_from(name: &EdgeName, mut read: EdgeRead) -> Option<EdgeRead> {
    let [first, second] = &name.between;
    let [one, other] = &read.between;
    if answers(first, one) && answers(second, other) {
        return Some(read);
    }
    if answers(first, other) && answers(second, one) {
        read.ends.reverse();
        return Some(read);
    }
    None
}

/// Whether a side recorded answers in a side now, as a face answers to any
/// of its numbers: the two share one.
fn answers(recorded: &[u32], now: &[u32]) -> bool {
    recorded.iter().any(|number| now.contains(number))
}

/// Whether an end recorded answers in an end now: every side of it answers
/// in one of the end's sides now, and every side now to one recorded. Two
/// empty ends answer each other.
fn end_answers(recorded: &[Vec<u32>], now: &[Vec<u32>]) -> bool {
    recorded
        .iter()
        .all(|side| now.iter().any(|other| answers(side, other)))
        && now
            .iter()
            .all(|other| recorded.iter().any(|side| answers(side, other)))
}
