//! One band of a region between two of its boundaries, cut into triangles as
//! the sweep reaches its points: the monotone polygon of de Berg et al.,
//! chapter 3, cut as its points arrive rather than once it is whole, so that a
//! band splitting round a hole or two bands merging past one need no polygon
//! drawn out first.

use std::cmp::Ordering;

use glam::DVec2;

use super::super::orientation::turn;

/// Which boundary of a band a point arrives on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Chain {
    Lower,
    Upper,
}

/// Where the triangles go, and the points they are made of.
pub(super) struct Cutter<'a> {
    pub(super) points: &'a [DVec2],
    pub(super) triangles: Vec<[usize; 3]>,
}

impl Cutter<'_> {
    /// Keeps a triangle counterclockwise, and drops one with no area: three
    /// points in a row cover nothing.
    fn triangle(&mut self, a: usize, b: usize, c: usize) {
        let [pa, pb, pc] = [a, b, c].map(|index| self.points[index]);
        match turn(pa, pb, pc) {
            Ordering::Greater => self.triangles.push([a, b, c]),
            Ordering::Less => self.triangles.push([a, c, b]),
            Ordering::Equal => {}
        }
    }

    /// Whether `point`, arriving on `chain`, sees `previous` past `last`: the
    /// corner at `last` bulges into the band, so that the diagonal from `point`
    /// to `previous` stays inside it.
    fn sees(&self, previous: usize, last: usize, point: usize, chain: Chain) -> bool {
        let side = turn(self.points[previous], self.points[last], self.points[point]);
        side == match chain {
            Chain::Lower => Ordering::Greater,
            Chain::Upper => Ordering::Less,
        }
    }
}

/// What of a band left of the sweep is not cut yet: points each seeing the
/// next one to come, all on one boundary but for the first. The last is the
/// point the sweep reached most recently, which every point to come sees.
pub(super) struct Piece {
    stack: Vec<(usize, Chain)>,
}

impl Piece {
    fn add(&mut self, point: usize, chain: Chain, cutter: &mut Cutter) {
        let Some(&top) = self.stack.last().filter(|_| self.stack.len() > 1) else {
            self.stack.push((point, chain));
            return;
        };
        if top.1 != chain {
            for pair in self.stack.windows(2) {
                cutter.triangle(pair[0].0, pair[1].0, point);
            }
            self.stack = vec![top, (point, chain)];
            return;
        }
        let mut last = top;
        self.stack.pop();
        while let Some(&previous) = self.stack.last() {
            if !cutter.sees(previous.0, last.0, point, chain) {
                break;
            }
            cutter.triangle(previous.0, last.0, point);
            last = previous;
            self.stack.pop();
        }
        self.stack.push(last);
        self.stack.push((point, chain));
    }

    /// Cuts what is left, where both boundaries meet at `point`.
    fn finish(self, point: usize, cutter: &mut Cutter) {
        for pair in self.stack.windows(2) {
            cutter.triangle(pair[0].0, pair[1].0, point);
        }
    }

    fn top(&self) -> (usize, Chain) {
        self.stack[self.stack.len() - 1]
    }
}

/// A band: one piece, or two joined at the point where two bands merged,
/// until the next point parts them by a diagonal to that point.
pub(super) enum Band {
    One(Piece),
    Two(Piece, Piece),
}

impl Band {
    pub(super) fn starting(point: usize) -> Band {
        Band::One(Piece {
            stack: vec![(point, Chain::Lower)],
        })
    }

    pub(super) fn add(self, point: usize, chain: Chain, cutter: &mut Cutter) -> Band {
        Band::One(self.joined(point, chain, cutter))
    }

    /// The band taking `point` on one of its boundaries, as one piece: of two,
    /// the one on the other side is closed by the diagonal to their meeting.
    fn joined(self, point: usize, chain: Chain, cutter: &mut Cutter) -> Piece {
        match (self, chain) {
            (Band::One(mut piece), _) => {
                piece.add(point, chain, cutter);
                piece
            }
            (Band::Two(lower, mut upper), Chain::Lower) => {
                lower.finish(point, cutter);
                upper.add(point, Chain::Lower, cutter);
                upper
            }
            (Band::Two(mut lower, upper), Chain::Upper) => {
                upper.finish(point, cutter);
                lower.add(point, Chain::Upper, cutter);
                lower
            }
        }
    }

    /// The band closed at `point`, where its two boundaries meet.
    pub(super) fn finish(self, point: usize, cutter: &mut Cutter) {
        match self {
            Band::One(piece) => piece.finish(point, cutter),
            Band::Two(lower, upper) => {
                lower.finish(point, cutter);
                upper.finish(point, cutter);
            }
        }
    }

    /// The band parted by a hole whose first point is `point`, into the band
    /// below the hole and the band above it. The diagonal that parts them runs
    /// from `point` to the last point the band reached, which it sees.
    pub(super) fn split(self, point: usize, cutter: &mut Cutter) -> (Band, Band) {
        match self {
            Band::One(mut piece) => {
                let (reached, chain) = piece.top();
                let fresh = |chain: Chain, other: Chain| Piece {
                    stack: vec![(reached, chain), (point, other)],
                };
                match chain {
                    Chain::Lower => {
                        piece.add(point, Chain::Lower, cutter);
                        let below = fresh(Chain::Lower, Chain::Upper);
                        (Band::One(below), Band::One(piece))
                    }
                    Chain::Upper => {
                        piece.add(point, Chain::Upper, cutter);
                        let above = fresh(Chain::Upper, Chain::Lower);
                        (Band::One(piece), Band::One(above))
                    }
                }
            }
            Band::Two(mut lower, mut upper) => {
                lower.add(point, Chain::Upper, cutter);
                upper.add(point, Chain::Lower, cutter);
                (Band::One(lower), Band::One(upper))
            }
        }
    }

    /// Two bands becoming one past the last point of what parted them.
    pub(super) fn merge(below: Band, above: Band, point: usize, cutter: &mut Cutter) -> Band {
        let lower = below.joined(point, Chain::Upper, cutter);
        let upper = above.joined(point, Chain::Lower, cutter);
        Band::Two(lower, upper)
    }
}
