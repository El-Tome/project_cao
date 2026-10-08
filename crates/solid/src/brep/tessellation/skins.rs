//! A skin of no thickness: two faces folded onto each other whole, drawn on
//! the same three samples facing opposite ways ([`super::folds`]). Neither is
//! drawn, and every side of the two is still met once each way by what is
//! left, so the surface stays closed.

use std::collections::{BTreeMap, BTreeSet};

use crate::brep::topology::FaceId;

/// Leaves out every two triangles of different faces standing on the same
/// three samples and facing opposite ways, each corner a sample.
pub(super) fn cancelled(oriented: &mut Vec<(FaceId, [usize; 3])>) {
    let mut seen: BTreeMap<[usize; 3], Vec<usize>> = BTreeMap::new();
    for (index, (_, corners)) in oriented.iter().enumerate() {
        let mut key = *corners;
        key.sort_unstable();
        seen.entry(key).or_default().push(index);
    }
    let even = |corners: [usize; 3]| {
        let turns = (0..3)
            .filter(|at| corners[*at] < corners[(at + 1) % 3])
            .count();
        turns == 2
    };
    let mut dropped = BTreeSet::new();
    for indices in seen.values() {
        for (place, &one) in indices.iter().enumerate() {
            if dropped.contains(&one) {
                continue;
            }
            let other = indices[place + 1..].iter().copied().find(|other| {
                !dropped.contains(other)
                    && oriented[*other].0 != oriented[one].0
                    && even(oriented[*other].1) != even(oriented[one].1)
            });
            if let Some(other) = other {
                dropped.insert(one);
                dropped.insert(other);
            }
        }
    }
    let mut index = 0;
    oriented.retain(|_| {
        index += 1;
        !dropped.contains(&(index - 1))
    });
}

#[cfg(test)]
mod tests;
