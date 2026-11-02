//! A face of a sphere or a torus, laid out with its boundaries square to the
//! chart, cut in strips between the rows of its grid (#576).
//!
//! Every boundary of such a face runs along a line of the chart: a latitude
//! circle, a floor at a pole or the cut of a ring's chart along the angle
//! about the axis, a meridian or a wall closing the face cut open at an
//! angle along the angle along the meridian. Between two consecutive rows of
//! the grid — the rows, and every angle along the meridian a boundary runs
//! at — the face is a row of columns, each between a boundary running down
//! and the next running up. A column's bottom and top are its boundaries
//! where one runs there, and the row itself where none does: there the face
//! continues across the row, and the lattice points of the grid along it are
//! added inside the face, each once, shared by the strip below and the strip
//! above, and by the two walls of a face cut open, which stand for the same
//! points a turn apart. A wall crossing several rows is split at each. Each
//! strip is then swept alone, so that no triangle reaches from one row past
//! the next.
//!
//! Every coordinate of the chart here is one of a few values made once — a
//! row of the grid, the angle of a latitude circle or a meridian, a pole's
//! line, the cut — and laid on each sample by the edge it stands on, so two
//! of them are compared exactly, bit for bit: a segment runs along a line of
//! the chart or it does not.

use std::collections::BTreeMap;
use std::f64::consts::TAU;

use glam::DVec2;

use super::super::sampling::{Rounded, Samples, rows};
use super::Outline;
use super::cylinder::CLEAR;

/// The lattice of a round's chart: the samples added inside its faces, one
/// for each angle of the grid and each row a face is cut along.
pub(super) struct Lattice {
    pub(super) round: Rounded,
    pub(super) steps: usize,
    pub(super) rows: usize,
    /// The angles along the meridian bounding the faces of the round where
    /// the face lies, which no row stands within the room of.
    pub(super) levels: Vec<f64>,
    /// How close to a level, along the meridian, no row stands: the room
    /// the sampling leaves at the tube's radius.
    pub(super) room: f64,
    added: BTreeMap<(usize, u64), usize>,
}

impl Lattice {
    pub(super) fn new(
        round: Rounded,
        steps: usize,
        rows: usize,
        levels: &[f64],
        room: f64,
    ) -> Lattice {
        Lattice {
            round,
            steps,
            rows,
            levels: levels.to_vec(),
            room,
            added: BTreeMap::new(),
        }
    }

    /// The angle along the meridian a lattice point at `phi` is made at: the
    /// bottom of a torus's window is its top, a turn on.
    fn canonical(&self, phi: f64) -> f64 {
        match self.round.cut() {
            Some(top) if phi == top - TAU => top,
            _ => phi,
        }
    }

    /// The sample at the grid's angle `rank` (any whole number, a turn being
    /// `steps` of them) and at `phi` along the meridian, added once.
    pub(super) fn sample(&mut self, samples: &mut Samples, rank: i64, phi: f64) -> usize {
        let step = rank.rem_euclid(self.steps as i64) as usize;
        let phi = self.canonical(phi);
        *self.added.entry((step, phi.to_bits())).or_insert_with(|| {
            let theta = TAU * step as f64 / self.steps as f64;
            samples.added(self.round.point(DVec2::new(theta, phi)))
        })
    }

    fn angle(&self, rank: i64) -> f64 {
        TAU * rank as f64 / self.steps as f64
    }
}

/// A boundary of a strip running along the angle along the meridian: the
/// angle about the axis it stands at, whether it runs down, the points at
/// its two ends and its segments, more than one where it passes through a
/// point that is no level of the strip.
type Vertical = (f64, bool, [usize; 2], Vec<usize>);

/// A boundary of a strip running along a row: its segment, and the points
/// at its two ends, the one it starts at first.
type Along = ([usize; 2], usize);

impl Outline {
    /// Cuts the region laid out so far, every segment of which runs along a
    /// line of the chart, in strips between the rows of `lattice`'s grid
    /// and the angles along the meridian its boundaries run at. None when a
    /// segment runs along no line of the chart, when a boundary other than a
    /// wall crosses a row, or when the columns of a strip do not close.
    pub(super) fn stripped(&mut self, samples: &mut Samples, lattice: &mut Lattice) -> Option<()> {
        let (low, high) = self
            .points
            .iter()
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(low, high), at| {
                (low.min(at.y), high.max(at.y))
            });
        let walls = std::mem::take(&mut self.walls);
        let mut passing = self.passing(&walls);
        let mut levels: Vec<f64> = self
            .points
            .iter()
            .enumerate()
            .filter(|(point, _)| !passing.contains_key(point))
            .map(|(_, at)| at.y)
            .collect();
        levels.extend(rows(lattice.rows, low, high, &lattice.levels, lattice.room));
        levels.sort_by(f64::total_cmp);
        levels.dedup();
        let rank: BTreeMap<u64, usize> = levels
            .iter()
            .enumerate()
            .map(|(rank, level)| (level.to_bits(), rank))
            .collect();
        let rank = |phi: f64| rank.get(&phi.to_bits()).copied();
        passing.retain(|point, _| rank(self.points[*point].y).is_none());

        let mut verticals: Vec<Vec<Vertical>> = vec![Vec::new(); levels.len()];
        let mut bottoms: Vec<Vec<Along>> = vec![Vec::new(); levels.len()];
        let mut tops: Vec<Vec<Along>> = vec![Vec::new(); levels.len()];
        for index in 0..self.segments.len() {
            let [from, to] = self.segments[index];
            let (a, b) = (self.points[from], self.points[to]);
            if from == to {
                return None;
            }
            if a.x == b.x {
                if passing.contains_key(&from) {
                    continue;
                }
                let mut chain = vec![index];
                let mut end = to;
                while let Some(&next) = passing.get(&end) {
                    chain.push(next);
                    end = self.segments[next][1];
                }
                let (lower, upper) = if a.y < b.y { (from, end) } else { (end, from) };
                let (lo, hi) = (rank(self.points[lower].y)?, rank(self.points[upper].y)?);
                let pieces = if hi - lo > 1 {
                    if !walls.contains(&index) {
                        return None;
                    }
                    let step = (a.x / (TAU / lattice.steps as f64)).round() as i64;
                    let mut chain = vec![lower];
                    for level in &levels[lo + 1..hi] {
                        let id = lattice.sample(samples, step, *level);
                        chain.push(self.point(DVec2::new(a.x, *level), id));
                    }
                    chain.push(upper);
                    let down = a.y > b.y;
                    let mut pieces = Vec::new();
                    for (rank, pair) in chain.windows(2).enumerate() {
                        let segment = if down {
                            [pair[1], pair[0]]
                        } else {
                            [pair[0], pair[1]]
                        };
                        let at = if rank == 0 {
                            self.segments[index] = segment;
                            index
                        } else {
                            self.segments.push(segment);
                            self.segments.len() - 1
                        };
                        pieces.push((lo + rank, segment, vec![at]));
                    }
                    pieces
                } else {
                    vec![(lo, [from, end], chain)]
                };
                for (strip, [from, to], chain) in pieces {
                    let down = self.points[from].y > self.points[to].y;
                    verticals[strip].push((a.x, down, [from, to], chain));
                }
            } else if a.y == b.y {
                let level = rank(a.y)?;
                if b.x > a.x {
                    bottoms[level].push(([from, to], index));
                } else {
                    tops[level.checked_sub(1)?].push(([from, to], index));
                }
            } else {
                return None;
            }
        }

        let mut strips = Vec::new();
        for strip in 0..levels.len().saturating_sub(1) {
            let mut segments = Vec::new();
            let columns = &mut verticals[strip];
            columns.sort_by(|one, other| one.0.total_cmp(&other.0));
            if !columns.len().is_multiple_of(2) {
                return None;
            }
            let mut used = 0;
            for pair in columns.chunks(2) {
                let [(left, down, left_ends, _), (right, up, right_ends, _)] = pair else {
                    return None;
                };
                if !down || *up || right - left <= CLEAR {
                    return None;
                }
                let (bottom_left, top_left) = (left_ends[1], left_ends[0]);
                let (bottom_right, top_right) = (right_ends[0], right_ends[1]);
                let bottom = self.chained(
                    samples,
                    lattice,
                    (bottom_left, bottom_right),
                    &bottoms[strip],
                    &mut used,
                )?;
                let top = self.chained(
                    samples,
                    lattice,
                    (top_right, top_left),
                    &tops[strip],
                    &mut used,
                )?;
                segments.extend(bottom);
                segments.extend(top);
            }
            if used != bottoms[strip].len() + tops[strip].len() {
                return None;
            }
            segments.extend(
                columns
                    .iter()
                    .flat_map(|(_, _, _, chain)| chain.iter().copied()),
            );
            strips.push(segments);
        }
        self.strips = strips;
        Some(())
    }

    /// The points a boundary only passes through along a meridian — a vertex
    /// parting a meridian arc where an edge of another face meets it — each
    /// with the segment leaving it: two segments touch it, one running into
    /// it and one out of it the same way along the same angle about the
    /// axis, neither a wall. Where such a point stands at no row and no
    /// level of the face, the column runs on through it, and the boundary
    /// across the face from it need not stop there.
    fn passing(&self, walls: &[usize]) -> BTreeMap<usize, usize> {
        let mut touching = vec![0usize; self.points.len()];
        let mut arriving = BTreeMap::new();
        let mut leaving = BTreeMap::new();
        for (index, &[from, to]) in self.segments.iter().enumerate() {
            touching[from] += 1;
            touching[to] += 1;
            let (a, b) = (self.points[from], self.points[to]);
            if a.x == b.x && a.y != b.y && !walls.contains(&index) {
                arriving.insert(to, b.y > a.y);
                leaving.insert(from, (index, b.y > a.y));
            }
        }
        leaving
            .into_iter()
            .filter(|(point, (_, up))| {
                touching[*point] == 2 && arriving.get(point).is_some_and(|way| way == up)
            })
            .map(|(point, (index, _))| (point, index))
            .collect()
    }

    /// The boundary of a column along one row, from the point `from` to the
    /// point `to`, running the way they stand: the segments of `along`
    /// running between them, and lattice points on every stretch between
    /// where none does. None when a segment of `along` runs only partly
    /// between them, or the wrong way.
    fn chained(
        &mut self,
        samples: &mut Samples,
        lattice: &mut Lattice,
        (from, to): (usize, usize),
        along: &[Along],
        used: &mut usize,
    ) -> Option<Vec<usize>> {
        let (start, end) = (self.points[from], self.points[to]);
        let forwards = end.x > start.x;
        let (low, high) = (start.x.min(end.x), start.x.max(end.x));
        let mut covered: Vec<([usize; 2], usize)> = along
            .iter()
            .filter(|([a, b], _)| {
                let (a, b) = (self.points[*a].x, self.points[*b].x);
                a.min(b) >= low - CLEAR && a.max(b) <= high + CLEAR
            })
            .copied()
            .collect();
        covered.sort_by(|one, other| {
            let (one, other) = (self.points[one.0[0]].x, self.points[other.0[0]].x);
            if forwards {
                one.total_cmp(&other)
            } else {
                other.total_cmp(&one)
            }
        });
        *used += covered.len();
        let phi = start.y;
        let mut segments = Vec::new();
        let mut cursor = from;
        let mut fill =
            |outline: &mut Outline, cursor: usize, next: usize, segments: &mut Vec<usize>| {
                let (a, b) = (outline.points[cursor].x, outline.points[next].x);
                let step = TAU / lattice.steps as f64;
                let (low, high) = (a.min(b), a.max(b));
                let mut ranks: Vec<i64> = (((low + CLEAR) / step).ceil() as i64
                    ..=((high - CLEAR) / step).floor() as i64)
                    .collect();
                if !forwards {
                    ranks.reverse();
                }
                let mut last = cursor;
                for rank in ranks {
                    let id = lattice.sample(samples, rank, phi);
                    let point = outline.point(DVec2::new(lattice.angle(rank), phi), id);
                    outline.segments.push([last, point]);
                    segments.push(outline.segments.len() - 1);
                    last = point;
                }
                if last != next {
                    outline.segments.push([last, next]);
                    segments.push(outline.segments.len() - 1);
                }
            };
        for ([a, b], index) in covered {
            if a != cursor {
                let ahead = if forwards {
                    self.points[a].x > self.points[cursor].x
                } else {
                    self.points[a].x < self.points[cursor].x
                };
                if !ahead {
                    return None;
                }
                fill(self, cursor, a, &mut segments);
            }
            segments.push(index);
            cursor = b;
        }
        if cursor != to {
            fill(self, cursor, to, &mut segments);
        }
        Some(segments)
    }
}
