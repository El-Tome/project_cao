//! How much matter a solid encloses, and where.

use std::f64::consts::FRAC_1_PI;
use std::ops::Range;

use glam::{BVec3, DVec2, DVec3};

use super::{Flaw, Triangle};

/// The volume a closed surface encloses, from the signed volumes of the
/// tetrahedra its triangles make with the origin. Negative when the surface is
/// inside out.
pub fn enclosed(triangles: &[Triangle]) -> f64 {
    triangles
        .iter()
        .map(|[a, b, c]| a.dot(b.cross(*c)) / 6.0)
        .sum()
}

/// The stretches of one line that lie inside a solid, as distances along it,
/// in order and apart.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Spans {
    stretches: Vec<(f64, f64)>,
    /// What a surface that bounds no solid counts along the line beyond its
    /// stretches: a shell left inside another wraps a place twice, one turned
    /// inside out on its own wraps it the wrong way. Neither is a stretch of
    /// more or less matter, yet both are a wrong answer, and one no other rule
    /// sees: every edge still has its two faces, no face crosses another.
    ///
    /// Always nothing for spans made from others: `union` and `without` build
    /// a promise, and a promise is only ever a place in or out.
    surplus: f64,
}

impl Spans {
    /// Stretches in any order, overlapping or not, made into spans: sorted,
    /// joined where they meet, and the empty ones dropped.
    fn gathered(mut stretches: Vec<(f64, f64)>) -> Spans {
        stretches.retain(|(from, to)| from < to);
        stretches.sort_by(|left, right| left.0.total_cmp(&right.0));
        let mut joined: Vec<(f64, f64)> = Vec::with_capacity(stretches.len());
        for (from, to) in stretches {
            match joined.last_mut() {
                Some(last) if from <= last.1 => last.1 = last.1.max(to),
                _ => joined.push((from, to)),
            }
        }
        Spans {
            stretches: joined,
            surplus: 0.0,
        }
    }

    pub fn stretches(&self) -> &[(f64, f64)] {
        &self.stretches
    }

    /// How much of the line lies inside.
    pub fn length(&self) -> f64 {
        self.stretches.iter().map(|(from, to)| to - from).sum()
    }

    /// The matter along the line by the surface's own account: every place
    /// counted as many times as the surface wraps it, and against it where it
    /// is wrapped the wrong way — what `enclosed` counts, one line at a time.
    fn held(&self) -> f64 {
        self.length() + self.surplus
    }

    /// What is inside either: the promise for matter added.
    pub fn union(&self, other: &Spans) -> Spans {
        let both = self.stretches.iter().chain(&other.stretches);
        Spans::gathered(both.copied().collect())
    }

    /// What is inside this and not the other: the promise for matter taken
    /// away.
    pub fn without(&self, other: &Spans) -> Spans {
        let (mut left, mut first, holes) = (Vec::new(), 0, &other.stretches);
        for &(from, to) in &self.stretches {
            while holes.get(first).is_some_and(|&(_, end)| end <= from) {
                first += 1;
            }
            let mut start = from;
            for &(hole_from, hole_to) in holes[first..]
                .iter()
                .take_while(|(hole_from, _)| *hole_from < to)
            {
                if hole_from > start {
                    left.push((start, hole_from));
                }
                start = start.max(hole_to);
            }
            if start < to {
                left.push((start, to));
            }
        }
        Spans {
            stretches: left,
            surplus: 0.0,
        }
    }
}

/// One line of measure along which a solid and its promise disagree.
#[derive(Clone, Debug, PartialEq)]
pub struct Along {
    pub origin: DVec3,
    pub direction: DVec3,
    pub promised: f64,
    pub enclosed: f64,
}

/// A bundle of parallel lines laid across a region of space, to measure what a
/// solid encloses without trusting the solid's own account of it.
///
/// Every line runs the same way, and a line is found by where it crosses the
/// plane square to that way: `across` and `up` are the two directions of that
/// plane, and the lines stand on a grid of `count` by `count` cells there.
///
/// That way is along no axis, no plane of two axes and no diagonal, so that no
/// face of a drawing on round numbers lies along the lines and no line runs
/// inside a wall, where whether it is in or out is anybody's guess.
pub struct Lines {
    direction: DVec3,
    across: DVec3,
    up: DVec3,
    low: DVec2,
    cell: DVec2,
    count: usize,
    /// How far along the lines their distances are counted from: well before
    /// the region, so that a distance inside it is never negative.
    start: f64,
    reach: f64,
}

impl Lines {
    /// `count` by `count` lines covering the box between `low` and `high`.
    pub fn across(low: DVec3, high: DVec3, count: usize) -> Self {
        assert!(count > 0, "a bundle of no lines measures nothing");
        let direction = DVec3::new(FRAC_1_PI, 5f64.sqrt() - 2.0, 1.0).normalize();
        let across = direction.cross(DVec3::Z).normalize();
        let up = direction.cross(across);
        let corners = (0..8).map(|index| {
            let high_on = BVec3::new(index & 1 != 0, index & 2 != 0, index & 4 != 0);
            DVec3::select(high_on, high, low)
        });
        let flat = |corner: DVec3| DVec2::new(across.dot(corner), up.dot(corner));
        let (mut least, mut most) = (DVec2::INFINITY, DVec2::NEG_INFINITY);
        let mut nearest = f64::INFINITY;
        for corner in corners {
            least = least.min(flat(corner));
            most = most.max(flat(corner));
            nearest = nearest.min(direction.dot(corner));
        }
        Self {
            direction,
            across,
            up,
            low: least,
            cell: ((most - least) / count as f64).max(DVec2::splat(f64::MIN_POSITIVE)),
            count,
            start: nearest - (high - low).length() - 1.0,
            reach: low.abs().max(high.abs()).max_element().max(1.0),
        }
    }

    /// For every line, the stretches of it inside the solid these triangles
    /// bound.
    ///
    /// A line is inside wherever more faces were entered along it than left,
    /// so a surface turned inside out encloses nothing.
    pub fn inside(&self, triangles: &[Triangle]) -> Vec<Spans> {
        let mut crossings = vec![Vec::new(); self.count * self.count];
        for triangle in triangles {
            self.meet(triangle, &mut crossings);
        }
        crossings.into_iter().map(sweep).collect()
    }

    /// The volume a set of spans adds up to, one per line, counted the way
    /// `enclosed` counts it.
    pub fn volume(&self, spans: &[Spans]) -> f64 {
        assert_eq!(spans.len(), self.count * self.count);
        self.cell.x * self.cell.y * spans.iter().map(Spans::held).sum::<f64>()
    }

    /// Whether a solid encloses, along every line, what it was promised.
    pub fn compare(&self, promised: &[Spans], enclosed: &[Spans]) -> Result<(), Flaw> {
        let lines = self.count * self.count;
        assert!(promised.len() == lines && enclosed.len() == lines);
        let mut worst = None;
        let mut widest = ALONG_A_LINE * self.reach;
        for (index, (promise, measure)) in promised.iter().zip(enclosed).enumerate() {
            let gap = (promise.held() - measure.held()).abs();
            if gap > widest {
                (worst, widest) = (Some(index), gap);
            }
        }
        let Some(index) = worst else {
            return Ok(());
        };
        let place = self.place(index);
        Err(Flaw::Volume {
            promised: self.volume(promised),
            enclosed: self.volume(enclosed),
            worst: Some(Along {
                origin: self.across * place.x + self.up * place.y + self.direction * self.start,
                direction: self.direction,
                promised: promised[index].held(),
                enclosed: enclosed[index].held(),
            }),
        })
    }

    fn flat(&self, corner: DVec3) -> DVec2 {
        DVec2::new(self.across.dot(corner), self.up.dot(corner))
    }

    /// Where line `index` crosses the plane square to the lines.
    fn place(&self, index: usize) -> DVec2 {
        let cell = DVec2::new((index % self.count) as f64, (index / self.count) as f64);
        self.low + (cell + OFF_CENTRE) * self.cell
    }

    /// Adds, to every line this triangle meets, where it does and which way:
    /// one step in when the face looks against the lines, one out when it
    /// looks along them.
    ///
    /// Only the lines under the triangle's shadow on the grid are tried, row
    /// by row: every line against every face would not finish on a solid of
    /// many facets, and neither would every line under the face's box, for a
    /// long face slanting across the grid has a box over most of it.
    fn meet(&self, triangle: &Triangle, crossings: &mut [Vec<(f64, i32)>]) {
        let mut flat = triangle.map(|corner| self.flat(corner));
        let mut along = triangle.map(|corner| self.direction.dot(corner) - self.start);
        let area = (flat[1] - flat[0]).perp_dot(flat[2] - flat[0]);
        if area == 0.0 {
            return;
        }
        let step = if area > 0.0 {
            -1
        } else {
            flat.swap(1, 2);
            along.swap(1, 2);
            1
        };
        let lowest = flat[0].y.min(flat[1].y).min(flat[2].y);
        let highest = flat[0].y.max(flat[1].y).max(flat[2].y);
        let half = self.cell.y / 2.0;
        for row in self.near(1, lowest, highest) {
            let height = self.low.y + (row as f64 + OFF_CENTRE.y) * self.cell.y;
            let (least, most) = breadth(flat, height - half, height + half);
            for column in self.near(0, least, most) {
                let index = row * self.count + column;
                if let Some(weights) = covered(flat, self.place(index)) {
                    let at = weights[0] * along[0] + weights[1] * along[1] + weights[2] * along[2];
                    crossings[index].push((at, step));
                }
            }
        }
    }

    /// The lines of the grid, counted along `axis`, that pass between `least`
    /// and `most`, and one more on each side: a line through the very end is
    /// not lost to the rounding of which cell that end falls in.
    fn near(&self, axis: usize, least: f64, most: f64) -> Range<usize> {
        if least > most {
            return 0..0;
        }
        let edge = (self.count - 1) as f64;
        let cells = |at: f64| (at - self.low[axis]) / self.cell[axis] - OFF_CENTRE[axis];
        let first = (cells(least).ceil() - 1.0).clamp(0.0, edge);
        let last = (cells(most).floor() + 1.0).clamp(0.0, edge);
        first as usize..last as usize + 1
    }
}

/// How far across a triangle lies within the band between two heights: the
/// least and the most of where its edges enter and leave the band, and of its
/// corners inside it.
///
/// An edge rising less than the band is taken whole. It lies across two rows
/// at most, and where it crosses a height, found by dividing by that small
/// rise, could land anywhere along it.
fn breadth(corners: [DVec2; 3], low: f64, high: f64) -> (f64, f64) {
    let (mut least, mut most) = (f64::INFINITY, f64::NEG_INFINITY);
    for (index, &from) in corners.iter().enumerate() {
        let to = corners[(index + 1) % 3];
        let (bottom, top) = if from.y <= to.y {
            (from, to)
        } else {
            (to, from)
        };
        if top.y < low || bottom.y > high {
            continue;
        }
        let rise = top.y - bottom.y;
        let shares = if rise < high - low {
            [0.0, 1.0]
        } else {
            [low, high].map(|height| ((height - bottom.y) / rise).clamp(0.0, 1.0))
        };
        for share in shares {
            let across = bottom.x + share * (top.x - bottom.x);
            (least, most) = (least.min(across), most.max(across));
        }
    }
    (least, most)
}

/// Where in its cell a line passes, as a fraction of the cell from its lower
/// corner: the fractional parts of the golden ratio and of the square root of
/// two, rather than the middle, so that no line runs through a corner or an
/// edge a drawing put on round numbers.
const OFF_CENTRE: DVec2 = DVec2::new(0.618_033_988_749_894_8, 0.414_213_562_373_095_1);

/// How far the matter along one line may be from what was promised, as a
/// fraction of how far the region reaches.
///
/// A crossing is placed from the corners of the face it passes through, and a
/// kernel that cut that face may have moved them by its own tolerance, `NEAR`
/// of the reach. A face met at a slant stretches that shift along the line. A
/// millionth leaves a thousand times the kernel's noise for the slant, and is
/// still far below what a face missing, doubled or out of place leaves along
/// every line through it.
const ALONG_A_LINE: f64 = 1e-6;

/// The stretches of one line inside a surface, from where it crosses the
/// surface and which way. Crossings at the same distance are taken together,
/// so that a line through an edge leaves no stretch of no length.
fn sweep(mut crossings: Vec<(f64, i32)>) -> Spans {
    crossings.sort_by(|left, right| left.0.total_cmp(&right.0));
    let mut stretches = Vec::new();
    let (mut depth, mut from, mut last, mut surplus) = (0, 0.0, 0.0, 0.0);
    for group in crossings.chunk_by(|left, right| left.0 == right.0) {
        let at = group[0].0;
        surplus += f64::from(depth - i32::from(depth > 0)) * (at - last);
        let after = depth + group.iter().map(|(_, step)| step).sum::<i32>();
        if depth <= 0 && after > 0 {
            from = at;
        } else if depth > 0 && after <= 0 {
            stretches.push((from, at));
        }
        (depth, last) = (after, at);
    }
    Spans { stretches, surplus }
}

/// Whether a point lies on a triangle turning anticlockwise, and if so how
/// much of each corner is in it.
///
/// A point on an edge belongs to the triangle on one side of it only, which
/// side depending on which way the edge runs: two faces sharing an edge run
/// along it opposite ways, so a line through it is counted once. The same
/// choice, made the same way at every edge, gives a corner several faces
/// share to exactly one of them.
fn covered(corners: [DVec2; 3], point: DVec2) -> Option<[f64; 3]> {
    let [a, b, c] = corners;
    let weights = [side(b, c, point), side(c, a, point), side(a, b, point)];
    let owned = [(b, c), (c, a), (a, b)]
        .iter()
        .zip(weights)
        .all(|(&(from, to), weight)| weight > 0.0 || (weight == 0.0 && claims(from, to)));
    let total: f64 = weights.iter().sum();
    (owned && total > 0.0).then(|| weights.map(|weight| weight / total))
}

/// How far to the left of the edge from `from` to `to` a point lies, times the
/// edge's length. Worked out from the lesser end whichever way the edge runs,
/// so that two faces sharing it get answers exactly opposite, bit for bit.
fn side(from: DVec2, to: DVec2, point: DVec2) -> f64 {
    if (from.x, from.y) < (to.x, to.y) {
        (to - from).perp_dot(point - from)
    } else {
        -(from - to).perp_dot(point - to)
    }
}

/// Whether a point lying exactly on an edge belongs to the face on its left:
/// for exactly one of the two ways the edge can run.
fn claims(from: DVec2, to: DVec2) -> bool {
    to.y < from.y || (to.y == from.y && to.x < from.x)
}

#[cfg(test)]
mod tests;
