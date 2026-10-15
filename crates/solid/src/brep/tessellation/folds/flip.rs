//! Two triangles of a face sharing a side cut the other way, so that none
//! of the triangles made lies on another face or passes through one; and
//! put back where that fails.

use std::collections::BTreeSet;

use super::{At, Folds, opens};

/// Flips made, each two triangles of a face and what they were before.
type Undo = Vec<(At, At, [[usize; 3]; 2])>;

impl Folds<'_> {
    /// Cuts the triangle at `at` the other way, with one of the triangles of
    /// its face across an inner side, so that none of the triangles it makes
    /// lies on another face or passes through one. One that does is cut the
    /// other way in turn, `depth` times over. The flips made, to undo them.
    pub(super) fn unfold(&mut self, at: At, depth: usize) -> Option<Undo> {
        for side in self.inner_sides(at) {
            let Some(partner) = self.beyond(at, side) else {
                continue;
            };
            let mut ways: Vec<Option<At>> = vec![None];
            ways.extend(
                self.inner_sides(partner)
                    .into_iter()
                    .filter_map(|other| self.beyond(partner, other))
                    .filter(|other| *other != at)
                    .map(Some),
            );
            for first in ways {
                let mut undo = Undo::new();
                if let Some(other) = first {
                    let Some(was) = self.flip(partner, other) else {
                        continue;
                    };
                    undo.push((partner, other, was));
                }
                let now = self.beyond(at, side);
                if let Some(was) = now.and_then(|now| self.flip(at, now).map(|was| (now, was))) {
                    undo.push((at, was.0, was.1));
                    if let Some(more) = self.settled(&undo, depth) {
                        undo.extend(more);
                        return Some(undo);
                    }
                }
                self.undo(undo);
            }
        }
        None
    }

    /// Whether the triangles the flips of `undo` made lie on or pass through
    /// no triangle of another face, once those they do are cut the other way
    /// within `depth`: the flips that took, if any were needed.
    fn settled(&mut self, undo: &Undo, depth: usize) -> Option<Undo> {
        let made: Vec<At> = undo
            .iter()
            .flat_map(|(one, other, _)| [*one, *other])
            .collect();
        let blocking = |folds: &Self| -> Vec<At> {
            made.iter()
                .flat_map(|each| [folds.lying(*each), folds.crossing(*each)])
                .flatten()
                .collect()
        };
        let blockers = blocking(self);
        if blockers.is_empty() {
            return Some(Undo::new());
        }
        if depth == 0 {
            return None;
        }
        for blocker in blockers {
            let Some(more) = self.unfold(blocker, depth - 1) else {
                continue;
            };
            if blocking(self).is_empty() {
                return Some(more);
            }
            self.undo(more);
        }
        None
    }

    fn undo(&mut self, undo: Undo) {
        for (one, other, [was, theirs]) in undo.into_iter().rev() {
            self.replace(one, other, was, theirs);
        }
    }

    /// The sides of a triangle, by their two corners in the outline, that are
    /// no segment of it.
    fn inner_sides(&self, (face, triangle): At) -> Vec<[usize; 2]> {
        let drawn = &self.drawn[face];
        let bounds: BTreeSet<[usize; 2]> = drawn
            .outline
            .segments
            .iter()
            .map(|&[a, b]| [a.min(b), a.max(b)])
            .collect();
        let corners = drawn.triangles[triangle];
        (0..3)
            .map(|side| {
                let (one, other) = (corners[side], corners[(side + 1) % 3]);
                [one.min(other), one.max(other)]
            })
            .filter(|side| !bounds.contains(side))
            .collect()
    }

    /// The other triangle of the face along a side.
    fn beyond(&self, (face, triangle): At, [one, other]: [usize; 2]) -> Option<At> {
        let triangles = &self.drawn[face].triangles;
        (0..triangles.len())
            .find(|each| {
                *each != triangle
                    && triangles[*each].contains(&one)
                    && triangles[*each].contains(&other)
            })
            .map(|each| (face, each))
    }

    /// Replaces two triangles of a face sharing a side by the two the other
    /// diagonal makes, when they make a convex corner of four; the two
    /// replaced, to put back.
    fn flip(&mut self, at: At, partner: At) -> Option<[[usize; 3]; 2]> {
        let face = at.0;
        let (old, theirs) = (
            self.drawn[face].triangles[at.1],
            self.drawn[face].triangles[partner.1],
        );
        let side = (0..3)
            .find(|side| theirs.contains(&old[*side]) && theirs.contains(&old[(*side + 1) % 3]))?;
        let (u, v, c) = (old[side], old[(side + 1) % 3], old[(side + 2) % 3]);
        let &d = theirs
            .iter()
            .find(|corner| **corner != u && **corner != v)?;
        let points = &self.drawn[face].outline.points;
        let convex = |a: usize, b: usize, e: usize| opens(points[a], points[b], points[e]);
        if !(convex(c, u, d) && convex(c, d, v)) {
            return None;
        }
        self.replace(at, partner, [c, u, d], [c, d, v]);
        Some([old, theirs])
    }

    fn replace(&mut self, at: At, partner: At, one: [usize; 3], other: [usize; 3]) {
        self.index(at, false);
        self.index(partner, false);
        self.drawn[at.0].triangles[at.1] = one;
        self.drawn[partner.0].triangles[partner.1] = other;
        self.index(at, true);
        self.index(partner, true);
    }
}
