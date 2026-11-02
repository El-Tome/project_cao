//! The floors of a sphere's or a torus's chart (#576): its poles, each a
//! line the chart ends along, laid as a cone's apex is — a loop passing
//! through a pole runs along its floor from the angle it comes in at to the
//! angle it leaves at, the low pole's floor running the angle up and the high
//! pole's down, the way that keeps the face on the loop's left; a face
//! holding a pole within it takes the whole floor as a loop of its own.

use std::collections::BTreeMap;
use std::f64::consts::TAU;

use glam::DVec2;

use super::super::super::sampling::Samples;
use super::super::cone::{Laid, near_turn};
use super::super::cylinder::{CLEAR, apart};
use crate::brep::domain::floor::Pole;

/// The floors of a round's chart: its poles, each a line the chart ends
/// along, laid by the round's grid of `steps` angles.
pub(super) struct Floors<'a> {
    pub(super) samples: &'a Samples,
    pub(super) poles: Vec<Pole>,
    pub(super) read: BTreeMap<usize, DVec2>,
    pub(super) eps: f64,
    pub(super) steps: usize,
}

impl Floors<'_> {
    fn at(&self, pole: usize, theta: f64) -> DVec2 {
        DVec2::new(theta, self.poles[pole].at)
    }

    fn angle(&self, rank: i64) -> f64 {
        TAU * rank as f64 / self.steps as f64
    }

    /// The pole sample `id` stands at, if it is a vertex there.
    pub(super) fn tip(&self, id: usize) -> Option<usize> {
        if !self.samples.is_vertex(id) {
            return None;
        }
        let point = self.samples.point(id);
        self.poles
            .iter()
            .position(|pole| point.distance(pole.point) <= self.eps)
    }

    /// The floor of `pole` from angle `from` to angle `to`, both kept, and
    /// every angle of the grid between them, each standing for the pole's
    /// sample `id`. None when the two do not stand within a turn of each
    /// other.
    fn between(&self, from: f64, to: f64, pole: usize, id: usize, laid: &mut Laid) -> Option<()> {
        let span = (to - from).abs();
        if span.is_nan() || span > TAU + CLEAR {
            return None;
        }
        let (low, high) = (from.min(to), from.max(to));
        let step = TAU / self.steps as f64;
        let ranks = (low / step).floor() as i64..=(high / step).ceil() as i64;
        let mut inner: Vec<f64> = ranks
            .map(|rank| self.angle(rank))
            .filter(|theta| theta - low > CLEAR && high - theta > CLEAR)
            .collect();
        if to < from {
            inner.reverse();
        }
        let last = (span > CLEAR).then_some(to);
        for theta in std::iter::once(from).chain(inner).chain(last) {
            laid.raw.push(self.at(pole, theta));
            laid.ids.push(id);
        }
        Some(())
    }

    /// The angle a floor of `pole` leaving `from` reaches the angle `raw` at,
    /// a whole number of turns on: the low pole's floor runs the angle up,
    /// the high pole's down — the way that keeps the face on the loop's
    /// left, as the domain's joins run — to the nearest such angle, which
    /// is `from` itself within [`CLEAR`] of it. The same bits as the next
    /// sample's own angle unwrapped, so that the meridian leaving the floor
    /// stands exactly square to it.
    fn onward(&self, from: f64, raw: f64, pole: usize) -> f64 {
        let near = ((from - raw) / TAU).round();
        let high = self.poles[pole].high;
        (-1..=1)
            .map(|turns| raw + TAU * (near + turns as f64))
            .filter(|to| {
                if high {
                    *to <= from + CLEAR
                } else {
                    *to >= from - CLEAR
                }
            })
            .min_by(|one, other| (one - from).abs().total_cmp(&(other - from).abs()))
            .unwrap_or(raw)
    }

    /// The loop `lap` laid out, each angle within half a turn of the last,
    /// and each pass through a pole laid along its floor, the way that pole's
    /// floor runs; the last pass closes the loop. None when it passes a pole
    /// twice in a row or stands at a pole alone.
    pub(super) fn laid(&self, lap: &[usize]) -> Option<Laid> {
        let count = lap.len();
        let start = match lap.iter().position(|id| self.tip(*id).is_some()) {
            Some(first) => (first + 1) % count,
            None => 0,
        };
        let mut laid = Laid {
            raw: Vec::with_capacity(count),
            ids: Vec::with_capacity(count),
            turns: 0,
        };
        let mut passing: Option<(usize, usize)> = None;
        let mut last: Option<f64> = None;
        for id in (0..count).map(|at| lap[(start + at) % count]) {
            if let Some(pole) = self.tip(id) {
                if passing.replace((pole, id)).is_some() {
                    return None;
                }
                continue;
            }
            let read = *self.read.get(&id)?;
            let theta = match (passing.take(), last) {
                (Some((pole, tip)), Some(last)) => {
                    let to = self.onward(last, read.x, pole);
                    self.between(last, to, pole, tip, &mut laid)?;
                    to
                }
                (None, Some(last)) => near_turn(read.x, last),
                (_, None) => read.x,
            };
            laid.raw.push(DVec2::new(theta, read.y));
            laid.ids.push(id);
            last = Some(theta);
        }
        let (first, last) = (laid.raw.first()?.x, last?);
        match passing {
            Some((pole, tip)) => {
                let to = self.onward(last, first, pole);
                self.between(last, to, pole, tip, &mut laid)?;
                laid.turns = ((to - first) / TAU).round() as i64;
            }
            None => {
                let round = last + apart(first, last) - first;
                laid.turns = (round / TAU).round() as i64;
            }
        }
        Some(laid)
    }

    /// The floor of `pole` going once round on its own, the way `way` says,
    /// for the pole's sample `id`.
    pub(super) fn round(&self, way: i64, pole: usize, id: usize) -> Laid {
        Laid {
            raw: (0..self.steps as i64)
                .map(|rank| self.at(pole, self.angle(way * rank)))
                .collect(),
            ids: vec![id; self.steps],
            turns: way,
        }
    }
}
