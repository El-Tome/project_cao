//! Two faces folded onto each other across an edge, drawn apart.
//!
//! Where two surfaces stand within the kernel's tolerance of each other — a
//! wall touching a plane along a line, a turn a hundredth of a degree thick,
//! the ruling a cone's end plane holds — the vertices the kernel put there
//! stand a hair off where the surfaces meet, and the faces either side reach
//! each other over a sliver a few tolerances wide. A triangle cut in that
//! sliver, fanned from a far corner of its face onto the hair, lies flat on
//! the triangle of the other face across their edge, on the same side of it:
//! drawn, the two lie on each other. The sweep cuts each face exactly and
//! knows nothing of the other, so the fold is undone afterwards, in the face's
//! own parameters, by the other diagonal of the triangle it shares an edge
//! with: the face is the same region, cut the other way, and the triangle now
//! reaches out of the sliver.
//!
//! A diagonal is taken only where the two triangles beside it make a convex
//! corner of four, by the exact signs the sweep decides with and by more than
//! the rounding the parameters were read with, so the cut stays a cut of the
//! region in space as in parameters, and only where neither triangle it makes
//! lies on another face or passes through one — a chord of a round a hair off
//! a vertex of the face beside it sinks under it by a fraction of the
//! tolerance. Where the diagonal runs into a point of the boundary, the
//! triangle beyond is cut the other way first; where a triangle it makes is
//! held by one of another face, that one is cut the other way too, and the two
//! flips stand or fall together. Two faces whose triangles end up on the same
//! three samples are a skin of no thickness ([`super::skins`]).

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};

use glam::{DVec2, DVec3};

use super::orientation::turn;
use super::outline::Outline;
use super::sampling::Samples;
use crate::brep::topology::FaceId;

/// A face cut into triangles in its surface's parameters, each corner a point
/// of its outline.
pub(super) struct Drawn {
    pub(super) face: FaceId,
    pub(super) outline: Outline,
    pub(super) triangles: Vec<[usize; 3]>,
}

/// A triangle, by its drawing and its place there.
type At = (usize, usize);

/// Flips made, each two triangles of a face and what they were before.
type Undo = Vec<(At, At, [[usize; 3]; 2])>;

/// How many times over a triangle a flip makes lie on or pass through
/// another is itself cut the other way: once, two folds that hold each other
/// — a needle lying on one face beside a triangle of a third passing through
/// the first — come undone together.
const DEPTH: usize = 1;

/// How far off a triangle's plane, as a share of the tolerance, a corner of
/// another stands across it: far under what the rules see, far over rounding.
const CLEAR: f64 = 1e-2;

/// How far off the line of the other two, in roundings of the parameters,
/// the nearest corner of a triangle a flip makes stands: a thousand, far over
/// the few a reading of a sample carries, and far under the narrowest sliver
/// a flip is wanted for.
const ROUNDINGS: f64 = 1e3;

/// Undoes every fold between the faces of `drawn`, as far as a diagonal can.
pub(super) fn unfolded(drawn: &mut [Drawn], samples: &Samples, eps: f64) {
    let mut folds = Folds {
        drawn,
        samples,
        eps,
        edges: BTreeMap::new(),
    };
    for face in 0..folds.drawn.len() {
        for triangle in 0..folds.drawn[face].triangles.len() {
            folds.index((face, triangle), true);
        }
    }
    for face in 0..folds.drawn.len() {
        for triangle in 0..folds.drawn[face].triangles.len() {
            let Some(other) = folds.lying((face, triangle)) else {
                continue;
            };
            if folds.unfold((face, triangle), DEPTH).is_none() && folds.lying(other).is_some() {
                folds.unfold(other, DEPTH);
            }
        }
    }
}

struct Folds<'a> {
    drawn: &'a mut [Drawn],
    samples: &'a Samples,
    eps: f64,
    /// Each edge between two samples, and the triangles along it.
    edges: BTreeMap<[usize; 2], Vec<At>>,
}

impl Folds<'_> {
    /// The samples at the corners of a triangle, None when two are one: such
    /// a triangle is not drawn.
    fn corners(&self, (face, triangle): At) -> Option<[usize; 3]> {
        let drawn = &self.drawn[face];
        let [a, b, c] = drawn.triangles[triangle].map(|corner| drawn.outline.samples[corner]);
        (a != b && b != c && c != a).then_some([a, b, c])
    }

    fn index(&mut self, at: At, adding: bool) {
        let Some(corners) = self.corners(at) else {
            return;
        };
        for side in 0..3 {
            let (one, other) = (corners[side], corners[(side + 1) % 3]);
            let along = self
                .edges
                .entry([one.min(other), one.max(other)])
                .or_default();
            if adding {
                along.push(at);
            } else {
                along.retain(|each| *each != at);
            }
        }
    }

    /// A triangle of another face the triangle at `at` passes through.
    fn crossing(&self, at: At) -> Option<At> {
        let corners = self.corners(at)?;
        let reach = |corners: [usize; 3]| {
            let [a, b, c] = corners.map(|id| self.samples.point(id));
            (a.min(b).min(c) - self.eps, a.max(b).max(c) + self.eps)
        };
        let (low, high) = reach(corners);
        self.drawn.iter().enumerate().find_map(|(face, drawn)| {
            if face == at.0 {
                return None;
            }
            (0..drawn.triangles.len())
                .map(|triangle| (face, triangle))
                .find(|other| {
                    self.corners(*other).is_some_and(|theirs| {
                        let (from, to) = reach(theirs);
                        low.cmple(to).all()
                            && from.cmple(high).all()
                            && through(
                                corners.map(|id| self.samples.point(id)),
                                theirs.map(|id| self.samples.point(id)),
                                self.eps * CLEAR,
                            )
                    })
                })
        })
    }

    /// A triangle of another face the triangle at `at` lies on, if any. Its
    /// twin on the same three samples is not one: the two are a skin of no
    /// thickness, left out together ([`super::skins`]).
    fn lying(&self, at: At) -> Option<At> {
        let corners = self.corners(at)?;
        (0..3).find_map(|side| {
            let (one, other) = (corners[side], corners[(side + 1) % 3]);
            let edge = [one.min(other), one.max(other)];
            self.edges.get(&edge)?.iter().copied().find(|beside| {
                beside.0 != at.0
                    && self.corners(*beside).is_some_and(|under| {
                        !under.iter().all(|id| corners.contains(id))
                            && self.lies_on(corners, under, edge)
                    })
            })
        })
    }

    /// Whether triangle `one` lies on `other` across the edge both have: its
    /// third corner within the tolerance of the other's plane, on the side of
    /// the edge the other covers.
    fn lies_on(&self, one: [usize; 3], other: [usize; 3], [a, b]: [usize; 2]) -> bool {
        let third = |corners: [usize; 3]| corners.into_iter().find(|id| *id != a && *id != b);
        let (Some(mine), Some(theirs)) = (third(one), third(other)) else {
            return false;
        };
        let [from, to, mine, theirs] = [a, b, mine, theirs].map(|id| self.samples.point(id));
        let Some(normal) = (to - from).cross(theirs - from).try_normalize() else {
            return false;
        };
        let side = (to - from).cross(mine - from).dot(normal);
        normal.dot(mine - from).abs() <= self.eps && side > 0.0
    }

    /// Cuts the triangle at `at` the other way, with one of the triangles of
    /// its face across an inner side, so that none of the triangles it makes
    /// lies on another face or passes through one. One that does is cut the
    /// other way in turn, `depth` times over. The flips made, to undo them.
    fn unfold(&mut self, at: At, depth: usize) -> Option<Undo> {
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

/// Whether the triangle `a`, `b`, `e` of a face's parameters turns left, and
/// opens by more than `ROUNDINGS` roundings of its corners: three samples of
/// a rim, collinear in the parameters but for the rounding they were read
/// with, make no triangle there, and one in space whose neighbour's chord
/// skips the middle sample.
fn opens(a: DVec2, b: DVec2, e: DVec2) -> bool {
    let longest = (b - a).length().max((e - b).length()).max((a - e).length());
    let size = a.abs().max(b.abs()).max(e.abs()).max_element();
    turn(a, b, e) == Ordering::Greater
        && (b - a).perp_dot(e - a) > ROUNDINGS * f64::EPSILON * size * longest
}

/// Whether two triangles pass through each other: each stands across the
/// other's plane, and the stretches they cut each other's plane along
/// overlap by more than `clear`, as two sharing a corner do not where
/// they leave it on either side of the line their planes meet along.
fn through(one: [DVec3; 3], other: [DVec3; 3], clear: f64) -> bool {
    let normal = |[a, b, c]: [DVec3; 3]| (b - a).cross(c - a).try_normalize();
    let (Some(mine), Some(theirs)) = (normal(one), normal(other)) else {
        return false;
    };
    let Some(along) = mine.cross(theirs).try_normalize() else {
        return false;
    };
    let stretch = |corners: [DVec3; 3], normal: DVec3, base: DVec3| {
        let heights = corners.map(|corner| normal.dot(corner - base));
        let above = heights.iter().any(|height| *height > clear);
        let below = heights.iter().any(|height| *height < -clear);
        if !(above && below) {
            return None;
        }
        let mut reach: Vec<f64> = Vec::with_capacity(3);
        for here in 0..3 {
            let next = (here + 1) % 3;
            let (height, onward) = (heights[here], heights[next]);
            if height.abs() <= clear {
                reach.push(along.dot(corners[here]));
            } else if onward.abs() > clear && (height > 0.0) != (onward > 0.0) {
                let point = corners[here].lerp(corners[next], height / (height - onward));
                reach.push(along.dot(point));
            }
        }
        let low = reach.iter().copied().fold(f64::INFINITY, f64::min);
        let high = reach.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        Some((low, high))
    };
    let (Some(first), Some(second)) =
        (stretch(other, mine, one[0]), stretch(one, theirs, other[0]))
    else {
        return false;
    };
    first.1.min(second.1) - first.0.max(second.0) > clear
}

#[cfg(test)]
mod tests;
