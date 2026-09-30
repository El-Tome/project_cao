//! A face of a cylinder laid out in `(θ, h)`, its angle unwrapped so that no
//! loop jumps by a turn.
//!
//! A face whose loops come back to their start without going round the axis is
//! laid out as it is, its holes placed within the turn its outer loop spans. A
//! face going all the way round — a band between two rings, say — has loops
//! that come back a turn further on, and no seam to lay them out against: it
//! is cut at a grid angle clear of every vertex and every hole, where every
//! loop going round has a sample, and closed there by walls between those
//! samples, standing once at the angle of the cut and once a turn further on.

use std::f64::consts::{PI, TAU};

use glam::DVec2;

use super::super::sampling::Samples;
use super::Outline;
use crate::brep::surface::Cylinder;

/// A loop laid out from one of its points round to the same point again,
/// having gone `turns` times round the axis.
struct Lap {
    placed: Vec<(DVec2, usize)>,
    turns: i64,
}

impl Lap {
    /// The loop `ids` laid out from its point `first`, whose angle is taken
    /// within half a turn of `near`, each next angle within half a turn of the
    /// last. Every angle is its sample's own plus a whole number of turns, so
    /// that a point met twice at one place is met at the same bits.
    fn from(raw: &[DVec2], ids: &[usize], first: usize, near: f64) -> Lap {
        let start = ((near - raw[first].x) / TAU).round() as i64;
        let mut turns = start;
        let mut last = raw[first].x + TAU * turns as f64;
        let mut placed = Vec::with_capacity(ids.len() + 1);
        for step in 0..=ids.len() {
            let at = (first + step) % ids.len();
            let mut angle = raw[at].x + TAU * turns as f64;
            if angle - last > PI {
                turns -= 1;
                angle = raw[at].x + TAU * turns as f64;
            } else if angle - last < -PI {
                turns += 1;
                angle = raw[at].x + TAU * turns as f64;
            }
            placed.push((DVec2::new(angle, raw[at].y), ids[at]));
            last = angle;
        }
        Lap {
            placed,
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

    fn highest(&self) -> f64 {
        self.placed
            .iter()
            .map(|(at, _)| at.x)
            .fold(f64::NEG_INFINITY, f64::max)
    }
}

/// The angle from `to` to `angle`, within half a turn either way.
fn apart(angle: f64, to: f64) -> f64 {
    (angle - to + PI).rem_euclid(TAU) - PI
}

impl Outline {
    /// Lays out the loops of a face of `cylinder`, whose grid has `steps`
    /// steps a turn. None when a face going round has no angle to be cut at,
    /// or loops that do not pair into bands there.
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
                let within = low + (lap.placed[0].0.x - low).rem_euclid(TAU);
                let placed = Lap::from(raw, ids, 0, within).placed;
                self.chain(&placed);
            }
            return Some(());
        }

        let spans: Vec<(f64, f64)> = laid
            .iter()
            .filter(|lap| lap.turns == 0)
            .map(|lap| (lap.lowest(), lap.highest()))
            .collect();
        let corners: Vec<f64> = laps
            .iter()
            .zip(&raws)
            .flat_map(|(ids, raw)| ids.iter().zip(raw))
            .filter(|(id, _)| samples.is_vertex(**id))
            .map(|(_, at)| at.x)
            .collect();
        let cut = (0..steps)
            .map(|step| TAU * step as f64 / steps as f64)
            .find(|angle| {
                corners
                    .iter()
                    .all(|corner| apart(*corner, *angle).abs() > CLEAR)
                    && spans
                        .iter()
                        .all(|(low, high)| (angle - low).rem_euclid(TAU) >= high - low)
            })?;

        let mut ends = Vec::new();
        for ((ids, raw), lap) in laps.iter().zip(&raws).zip(&laid) {
            if lap.turns == 0 {
                let first = lap.placed[0].0.x;
                let within = cut + (first - cut).rem_euclid(TAU);
                self.chain(&Lap::from(raw, ids, 0, within).placed);
                continue;
            }
            let first = raw.iter().position(|at| apart(at.x, cut).abs() <= CLEAR)?;
            let start = if lap.turns > 0 { cut } else { cut + TAU };
            let placed = Lap::from(raw, ids, first, start).placed;
            let (start, end) = (placed[0], placed[placed.len() - 1]);
            self.chain(&placed);
            let (left, right) = if lap.turns > 0 {
                (start, end)
            } else {
                (end, start)
            };
            ends.push((start.0.y, lap.turns > 0, left, right));
        }
        ends.sort_by(|one, other| one.0.total_cmp(&other.0));
        for pair in ends.chunks(2) {
            let [
                (_, true, below_left, below_right),
                (_, false, above_left, above_right),
            ] = pair
            else {
                return None;
            };
            self.wall(*above_left, *below_left);
            self.wall(*below_right, *above_right);
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
