//! A region of the plane cut into triangles by a sweep from left to right,
//! with no point added: every corner of every triangle is a point of the
//! boundary, so that a face shares its edges' points with its neighbours and
//! nothing else.
//!
//! The points are taken in the order of their first coordinate, then their
//! second: a boundary square to the sweep is then a boundary like any other.
//! Between two points the boundaries crossing the sweep keep their order, and
//! each band between two of them that holds the region is cut as it goes, by
//! [`band`]. A band starts where two boundaries leave one point, ends where two
//! meet, is parted by a hole's first point and joined past its last. A point
//! where several boundaries meet — a loop visiting a corner twice, two holes
//! touching, a slit into the region or a hair running out of it — is each of
//! those at once, band by band.
//!
//! Every choice is a sign of [`turn`], exact, so the region is cut the same way
//! whatever the rounding of its points, as long as its boundaries do not cross.

mod band;

use std::cmp::Ordering;

use glam::DVec2;

use super::orientation::turn;
use band::{Band, Chain, Cutter};

/// The triangles of the region `segments` bound, each segment running between
/// two of `points` with the region on its left. Counterclockwise, by the index
/// of their corners. None when the segments bound no region the sweep can
/// follow: crossing, left open, or two points at one place.
pub(super) fn triangles(points: &[DVec2], segments: &[[usize; 2]]) -> Option<Vec<[usize; 3]>> {
    let mut order: Vec<usize> = (0..points.len()).collect();
    order.sort_by(|&one, &other| before(points[one], points[other]));
    if order
        .windows(2)
        .any(|pair| points[pair[0]] == points[pair[1]])
    {
        return None;
    }
    let mut rank = vec![0; points.len()];
    for (position, &point) in order.iter().enumerate() {
        rank[point] = position;
    }
    let mut sweep = Sweep {
        segments,
        rank,
        starting: vec![Vec::new(); points.len()],
        status: Vec::new(),
        bands: Vec::new(),
        cutter: Cutter {
            points,
            triangles: Vec::new(),
        },
    };
    for (index, &[from, to]) in segments.iter().enumerate() {
        if from == to {
            return None;
        }
        let low = sweep.low(index);
        sweep.starting[low].push(index);
    }
    for point in order {
        sweep.reach(point)?;
    }
    sweep.status.is_empty().then_some(sweep.cutter.triangles)
}

fn before(one: DVec2, other: DVec2) -> Ordering {
    one.x.total_cmp(&other.x).then(one.y.total_cmp(&other.y))
}

struct Sweep<'a> {
    segments: &'a [[usize; 2]],
    rank: Vec<usize>,
    starting: Vec<Vec<usize>>,
    /// The segments crossing the sweep, from the bottom up: one rising with
    /// the region above it, then one falling with the region below it, and so
    /// on, each pair holding a band.
    status: Vec<usize>,
    bands: Vec<Band>,
    cutter: Cutter<'a>,
}

impl Sweep<'_> {
    fn low(&self, segment: usize) -> usize {
        let [from, to] = self.segments[segment];
        if self.rank[from] < self.rank[to] {
            from
        } else {
            to
        }
    }

    fn high(&self, segment: usize) -> usize {
        let [from, to] = self.segments[segment];
        if self.rank[from] < self.rank[to] {
            to
        } else {
            from
        }
    }

    /// Whether a segment runs the way of the sweep, with the region above it.
    fn rises(&self, segment: usize) -> bool {
        self.segments[segment][0] == self.low(segment)
    }

    fn at(&self, point: usize) -> DVec2 {
        self.cutter.points[point]
    }

    /// Whether `point` stands above a segment crossing the sweep. None when it
    /// lies on it: the boundaries cross there.
    fn above(&self, segment: usize, point: usize) -> Option<bool> {
        let (low, high) = (self.low(segment), self.high(segment));
        match turn(self.at(low), self.at(high), self.at(point)) {
            Ordering::Greater => Some(true),
            Ordering::Less => Some(false),
            Ordering::Equal => None,
        }
    }

    /// The segments leaving `point`, from the bottom up, the first of them
    /// at `low` in the status. Two along one line are a slit: inside a band,
    /// the falling one bounds the band below, so it goes first; outside any,
    /// the slit is a hair running out of the region, and the rising one goes
    /// first, the two bounding a band of nothing between them.
    fn leaving(&self, point: usize, low: usize) -> Vec<usize> {
        let mut leaving = self.starting[point].clone();
        let from = self.at(point);
        leaving.sort_by(|&one, &other| {
            let side = turn(from, self.at(self.high(one)), self.at(self.high(other)));
            side.reverse()
                .then(self.rises(one).cmp(&self.rises(other)))
                .then(one.cmp(&other))
        });
        for at in 1..leaving.len() {
            let (one, other) = (leaving[at - 1], leaving[at]);
            let along = turn(from, self.at(self.high(one)), self.at(self.high(other)));
            if along == Ordering::Equal
                && self.rises(one) != self.rises(other)
                && self.rises(one) != (low + at - 1).is_multiple_of(2)
            {
                leaving.swap(at - 1, at);
            }
        }
        leaving
    }

    fn reach(&mut self, point: usize) -> Option<()> {
        let ending: Vec<usize> = (0..self.status.len())
            .filter(|&at| self.high(self.status[at]) == point)
            .collect();
        let (low, count) = match ending.first() {
            Some(&first) if ending.last() == Some(&(first + ending.len() - 1)) => {
                (first, ending.len())
            }
            Some(_) => return None,
            None => {
                let mut below = 0;
                for &segment in &self.status {
                    if !self.above(segment, point)? {
                        break;
                    }
                    below += 1;
                }
                (below, 0)
            }
        };
        let leaving = self.leaving(point, low);
        let alternating = leaving
            .iter()
            .enumerate()
            .all(|(offset, &segment)| self.rises(segment) == (low + offset).is_multiple_of(2));
        let crossing = count + leaving.len();
        if !alternating || !crossing.is_multiple_of(2) || crossing == 0 {
            return None;
        }

        let first_band = low / 2;
        let old_end = (low + count).div_ceil(2);
        let old: Vec<Band> = self.bands.drain(first_band..old_end).collect();
        let fresh = self.carry(old, point, low, count, leaving.len())?;
        if fresh.len() != (low + leaving.len()).div_ceil(2) - first_band {
            return None;
        }
        self.bands.splice(first_band..first_band, fresh);
        let arrived = leaving.len();
        self.status.splice(low..low + count, leaving);
        let neighbours = [low, low + arrived].map(|above| {
            let below = above.checked_sub(1)?;
            Some((*self.status.get(below)?, *self.status.get(above)?))
        });
        if neighbours
            .into_iter()
            .flatten()
            .any(|(below, above)| self.meet(below, above))
        {
            return None;
        }
        Some(())
    }

    /// Whether two segments meet anywhere but at a point both end at: they
    /// cross, or one ends on the other. Asked of every two segments as they
    /// come to lie side by side in the status, which is enough to find the
    /// first crossing, if there is one, before the sweep reaches it.
    fn meet(&self, one: usize, other: usize) -> bool {
        let ends = |segment| [self.low(segment), self.high(segment)];
        let [[a, b], [c, d]] = [ends(one), ends(other)];
        let parts = |a: usize, b: usize, c: usize, d: usize| {
            let (first, second) = (
                turn(self.at(a), self.at(b), self.at(c)),
                turn(self.at(a), self.at(b), self.at(d)),
            );
            first != Ordering::Equal && second != Ordering::Equal && first != second
        };
        let inside = |a: usize, b: usize, c: usize| {
            c != a
                && c != b
                && turn(self.at(a), self.at(b), self.at(c)) == Ordering::Equal
                && before(self.at(a), self.at(c)) == Ordering::Less
                && before(self.at(c), self.at(b)) == Ordering::Less
        };
        (parts(a, b, c, d) && parts(c, d, a, b))
            || inside(a, b, c)
            || inside(a, b, d)
            || inside(c, d, a)
            || inside(c, d, b)
    }

    /// The bands after `point`, from those before it: `count` boundaries end
    /// there and `leaving` start, the first of them at `low` in the status.
    fn carry(
        &mut self,
        old: Vec<Band>,
        point: usize,
        low: usize,
        count: usize,
        leaving: usize,
    ) -> Option<Vec<Band>> {
        let cutter = &mut self.cutter;
        let inner = |length: usize| (low + length).saturating_sub(low + low % 2) / 2;
        let mut old = old.into_iter();
        let mut fresh = Vec::new();
        let below = if low % 2 == 1 { old.next() } else { None };
        if count == 0 {
            let (under, over) = match below {
                Some(band) => {
                    let (under, over) = band.split(point, cutter);
                    (Some(under), Some(over))
                }
                None => (None, None),
            };
            fresh.extend(under);
            fresh.extend((0..inner(leaving)).map(|_| Band::starting(point)));
            fresh.extend(over);
            return Some(fresh);
        }
        for _ in 0..inner(count) {
            old.next()?.finish(point, cutter);
        }
        let above = old.next();
        match (below, above, leaving) {
            (Some(below), Some(above), 0) => fresh.push(Band::merge(below, above, point, cutter)),
            (None, None, _) => {
                fresh.extend((0..inner(leaving)).map(|_| Band::starting(point)));
            }
            (below, above, 1..) => {
                fresh.extend(below.map(|band| band.add(point, Chain::Upper, cutter)));
                fresh.extend((0..inner(leaving)).map(|_| Band::starting(point)));
                fresh.extend(above.map(|band| band.add(point, Chain::Lower, cutter)));
            }
            _ => return None,
        }
        Some(fresh)
    }
}

#[cfg(test)]
mod tests;
