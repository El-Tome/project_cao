//! A face of a cylinder laid out in `(θ, h)`, its angle unwrapped so that no
//! loop jumps by a turn.
//!
//! A face whose loops come back to their start without going round the axis is
//! laid out as it is, its holes placed within the turn its outer loop spans. A
//! face going all the way round — a band between two rings, say — has loops
//! that come back a turn further on, and no seam to lay them out against: it
//! is cut open at a grid angle that every loop passes at one of its own
//! samples, never between two and never at a vertex. Each loop falls into
//! pieces between the places it passes that angle, each piece laid within the
//! turn from it; walls between those places, standing once at the angle of
//! the cut and once a turn further on, close the pieces into one region. A
//! hole the cut passes through is two pieces like any other, so that a window
//! leaving a strut too narrow for a grid angle is cut through the window.

use std::f64::consts::{PI, TAU};

use glam::DVec2;

use super::super::sampling::Samples;
use super::Outline;
use crate::brep::surface::Cylinder;

/// A loop laid out from one of its points round to the same point again,
/// having gone `turns` times round the axis.
struct Lap {
    placed: Vec<(DVec2, usize)>,
    /// For each point placed, the rank of its sample in the loop and the
    /// whole turns added to the sample's own angle.
    turned: Vec<(usize, i64)>,
    turns: i64,
}

/// The angle of a sample whose own is `raw`, a whole number of turns on: the
/// same bits wherever the same sample is met at the same place.
fn turned(raw: DVec2, turns: i64) -> DVec2 {
    DVec2::new(raw.x + TAU * turns as f64, raw.y)
}

impl Lap {
    /// The loop `ids` laid out from its point `first`, whose angle is taken
    /// within half a turn of `near`, each next angle within half a turn of the
    /// last.
    fn from(raw: &[DVec2], ids: &[usize], first: usize, near: f64) -> Lap {
        let start = ((near - raw[first].x) / TAU).round() as i64;
        let mut turns = start;
        let mut last = turned(raw[first], turns).x;
        let mut placed = Vec::with_capacity(ids.len() + 1);
        let mut ranks = Vec::with_capacity(ids.len() + 1);
        for step in 0..=ids.len() {
            let at = (first + step) % ids.len();
            let mut angle = turned(raw[at], turns).x;
            if angle - last > PI {
                turns -= 1;
                angle = turned(raw[at], turns).x;
            } else if angle - last < -PI {
                turns += 1;
                angle = turned(raw[at], turns).x;
            }
            placed.push((DVec2::new(angle, raw[at].y), ids[at]));
            ranks.push((at, turns));
            last = angle;
        }
        Lap {
            placed,
            turned: ranks,
            turns: turns - start,
        }
    }

    fn area(&self) -> f64 {
        self.placed
            .windows(2)
            .map(|pair| pair[0].0.perp_dot(pair[1].0) / 2.0)
            .sum()
    }

    fn lowest(&self) -> f64 {
        self.placed
            .iter()
            .map(|(at, _)| at.x)
            .fold(f64::INFINITY, f64::min)
    }
}

/// The angle from `to` to `angle`, within half a turn either way.
fn apart(angle: f64, to: f64) -> f64 {
    (angle - to + PI).rem_euclid(TAU) - PI
}

impl Outline {
    /// Lays out the loops of a face of `cylinder`, whose grid has `steps`
    /// steps a turn. None when a face going round has no grid angle it can be
    /// cut open at.
    pub(super) fn round(
        &mut self,
        cylinder: &Cylinder,
        samples: &Samples,
        laps: &[Vec<usize>],
        steps: usize,
    ) -> Option<()> {
        let raws: Vec<Vec<DVec2>> = laps
            .iter()
            .map(|ids| {
                ids.iter()
                    .map(|id| cylinder.parameters(samples.point(*id)))
                    .collect()
            })
            .collect();
        let laid: Vec<Lap> = laps
            .iter()
            .zip(&raws)
            .map(|(ids, raw)| Lap::from(raw, ids, 0, raw[0].x))
            .collect();
        if laid.iter().all(|lap| lap.turns == 0) {
            let outer = laid
                .iter()
                .max_by(|one, other| one.area().total_cmp(&other.area()))?;
            let low = outer.lowest();
            for ((ids, raw), lap) in laps.iter().zip(&raws).zip(&laid) {
                let turns = ((low - lap.lowest()) / TAU).ceil();
                let placed = Lap::from(raw, ids, 0, lap.placed[0].0.x + TAU * turns).placed;
                self.chain(&placed);
            }
            return Some(());
        }

        let opened = (0..steps)
            .map(|step| TAU * step as f64 / steps as f64)
            .find_map(|cut| Opened::at(cut, laps, &raws, samples))?;
        for piece in &opened.pieces {
            self.chain(piece);
        }
        for [from, to] in opened.walls {
            self.wall(from, to);
        }
        Some(())
    }

    fn wall(&mut self, from: (DVec2, usize), to: (DVec2, usize)) {
        let (from, to) = (self.point(from.0, from.1), self.point(to.0, to.1));
        self.segments.push([from, to]);
    }
}

/// How close to a grid angle a sample must be to stand on it, and how far a
/// vertex must stand from the angle a face is cut at.
const CLEAR: f64 = 1e-9;

/// A face going round, cut open at an angle: its loops in pieces, each within
/// the turn from that angle, and the walls along both edges of the turn that
/// close them into a region.
struct Opened {
    pieces: Vec<Vec<(DVec2, usize)>>,
    walls: Vec<[(DVec2, usize); 2]>,
}

/// An end of a piece on an edge of the turn: its height, whether the region
/// lies above it along that edge, and its point.
type End = (f64, bool, (DVec2, usize));

impl Opened {
    /// The face cut open at `cut`. None when a loop passes that angle at a
    /// vertex or between two of its samples, or the ends of the pieces along
    /// an edge of the turn do not alternate into stretches of the region.
    fn at(cut: f64, laps: &[Vec<usize>], raws: &[Vec<DVec2>], samples: &Samples) -> Option<Opened> {
        let on_cut = |angle: f64| apart(angle, cut).abs() <= CLEAR;
        let mut pieces = Vec::new();
        for (ids, raw) in laps.iter().zip(raws) {
            let passing: Vec<usize> = (0..ids.len()).filter(|at| on_cut(raw[*at].x)).collect();
            if passing.iter().any(|at| samples.is_vertex(ids[*at])) {
                return None;
            }
            let (first, near) = match passing.first() {
                Some(&first) => (first, cut),
                None => (0, cut + (raw[0].x - cut).rem_euclid(TAU)),
            };
            let lap = Lap::from(raw, ids, first, near);
            let mut piece = vec![lap.turned[0]];
            for &(at, turns) in &lap.turned[1..] {
                piece.push((at, turns));
                if on_cut(raw[at].x) {
                    pieces.push(within(cut, &piece, ids, raw)?);
                    piece = vec![(at, turns)];
                }
            }
            if passing.is_empty() {
                pieces.push(within(cut, &piece, ids, raw)?);
            }
        }

        let (mut left, mut right): (Vec<End>, Vec<End>) = (Vec::new(), Vec::new());
        for piece in &pieces {
            let (start, end) = (piece[0], piece[piece.len() - 1]);
            if !on_cut(start.0.x) {
                continue;
            }
            for (place, leaving) in [(start, true), (end, false)] {
                let at_cut = place.0.x < cut + PI;
                let side = if at_cut { &mut left } else { &mut right };
                side.push((place.0.y, leaving == at_cut, place));
            }
        }
        let mut walls = Vec::new();
        for (side, rising) in [(left, false), (right, true)] {
            for [below, above] in stretches(side)? {
                let wall = if rising {
                    [below, above]
                } else {
                    [above, below]
                };
                if wall[0].0 != wall[1].0 {
                    walls.push(wall);
                }
            }
        }
        Some(Opened { pieces, walls })
    }
}

/// A piece of a loop between two places it passes the cut, or a whole loop
/// that never does, placed within the turn from `cut`. None when its points
/// between those places do not all stand in one turn.
fn within(
    cut: f64,
    piece: &[(usize, i64)],
    ids: &[usize],
    raw: &[DVec2],
) -> Option<Vec<(DVec2, usize)>> {
    let strip = |&(at, turns): &(usize, i64)| {
        let angle = turned(raw[at], turns).x;
        (apart(angle, cut).abs() > CLEAR).then(|| ((angle - cut) / TAU).floor() as i64)
    };
    let mut strips = piece.iter().filter_map(strip);
    let first = strips.next()?;
    if strips.any(|other| other != first) {
        return None;
    }
    Some(
        piece
            .iter()
            .map(|&(at, turns)| (turned(raw[at], turns - first), ids[at]))
            .collect(),
    )
}

/// The ends along one edge of the turn, paired from the bottom up into the
/// stretches of the edge the region lies against: each from an end the region
/// lies above to the next, which it lies below. None when they do not
/// alternate so.
fn stretches(mut side: Vec<End>) -> Option<Vec<[(DVec2, usize); 2]>> {
    side.sort_by(|one, other| one.0.total_cmp(&other.0).then(one.1.cmp(&other.1)));
    side.chunks(2)
        .map(|pair| match pair {
            [(_, true, below), (_, false, above)] => Some([*below, *above]),
            _ => None,
        })
        .collect()
}
