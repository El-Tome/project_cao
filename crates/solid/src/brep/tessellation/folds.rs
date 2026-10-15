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

mod flip;
mod through;

use std::cmp::Ordering;
use std::collections::BTreeMap;

use glam::DVec2;

use super::orientation::turn;
use super::outline::Outline;
use super::sampling::Samples;
use crate::brep::topology::FaceId;
use through::through;

/// A face cut into triangles in its surface's parameters, each corner a point
/// of its outline.
pub(super) struct Drawn {
    pub(super) face: FaceId,
    pub(super) outline: Outline,
    pub(super) triangles: Vec<[usize; 3]>,
}

/// A triangle, by its drawing and its place there.
type At = (usize, usize);

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
        around: BTreeMap::new(),
    };
    for face in 0..folds.drawn.len() {
        for triangle in 0..folds.drawn[face].triangles.len() {
            folds.index((face, triangle), true);
        }
    }
    for face in 0..folds.drawn.len() {
        for triangle in 0..folds.drawn[face].triangles.len() {
            let Some(other) = folds.folded((face, triangle)) else {
                continue;
            };
            if folds.unfold((face, triangle), DEPTH).is_none() && folds.folded(other).is_some() {
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
    /// Each sample, and the triangles with a corner there.
    around: BTreeMap<usize, Vec<At>>,
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
        let keep = |list: &mut Vec<At>| {
            if adding {
                list.push(at);
            } else {
                list.retain(|each| *each != at);
            }
        };
        for side in 0..3 {
            let (one, other) = (corners[side], corners[(side + 1) % 3]);
            keep(
                self.edges
                    .entry([one.min(other), one.max(other)])
                    .or_default(),
            );
            keep(self.around.entry(one).or_default());
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
                            && self.pass_through(corners, theirs)
                    })
                })
        })
    }

    /// A triangle of another face the triangle at `at` lies on, or passes
    /// through beside a corner the two share: where a face turns into the
    /// matter at a corner — a post's wall where a hole cut into it leaves
    /// its ceiling — a chord of the wall cut across the corner, true to the
    /// region in the wall's parameters, sags through the face beyond it
    /// (5550629622).
    fn folded(&self, at: At) -> Option<At> {
        self.lying(at).or_else(|| {
            let corners = self.corners(at)?;
            corners.iter().find_map(|corner| {
                self.around.get(corner)?.iter().copied().find(|beside| {
                    beside.0 != at.0
                        && self
                            .corners(*beside)
                            .is_some_and(|theirs| self.pass_through(corners, theirs))
                })
            })
        })
    }

    /// Whether the triangles on two sets of samples pass through each other.
    fn pass_through(&self, one: [usize; 3], other: [usize; 3]) -> bool {
        let points = |corners: [usize; 3]| corners.map(|id| self.samples.point(id));
        through(points(one), points(other), self.eps * CLEAR)
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

#[cfg(test)]
mod tests;
