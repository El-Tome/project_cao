//! A face of a cone laid out in `(θ, l)`, as a cylinder's is in `(θ, h)`, but
//! for its apex.
//!
//! At the apex every angle is the same point, and the angle read there is
//! rounding. A loop passing through it — a cone turned part way, whose two
//! rulings meet there — is laid along the floor `l = l_a` instead, from the
//! angle it comes in at to the angle it leaves at, by the cone's grid, each
//! point there standing for the apex's own sample; it goes back round by as
//! much as the rest of the loop goes round, so that it turns by nought. A face
//! holding its apex within it — a point turned whole — has a loop of the floor
//! of its own, going round the other way from its rims, so that the region
//! lies between them. A triangle with two corners on the floor stands for the
//! apex twice and is dropped: what is left is a fan from the apex to the rims'
//! samples.

use std::f64::consts::TAU;

use glam::DVec2;

use super::super::sampling::Samples;
use super::Outline;
use super::cylinder::{CLEAR, apart};
use crate::brep::surface::Cone;

/// The floor of a cone's chart, the line its apex is laid along, and the
/// grid it is laid by.
struct Floor<'a> {
    cone: &'a Cone,
    samples: &'a Samples,
    eps: f64,
    steps: usize,
}

/// A loop laid out in `(θ, l)`, each point beside the sample it stands for,
/// and how many times it goes round the axis.
struct Laid {
    raw: Vec<DVec2>,
    ids: Vec<usize>,
    turns: i64,
}

impl Floor<'_> {
    fn at(&self, theta: f64) -> DVec2 {
        DVec2::new(theta, self.cone.apex_at())
    }

    fn step(&self) -> f64 {
        TAU / self.steps as f64
    }

    /// Whether sample `id` is the vertex at the apex.
    fn tip(&self, id: usize) -> bool {
        self.samples.is_vertex(id) && self.samples.point(id).distance(self.cone.apex()) <= self.eps
    }

    /// The floor from angle `from` to angle `to`, both kept, and every angle
    /// of the grid between them, each standing for the apex's sample `id`.
    /// None when the two do not stand within a turn of each other: a loop
    /// passing through its apex goes round nothing.
    fn between(&self, from: f64, to: f64, id: usize, laid: &mut Laid) -> Option<()> {
        let span = (to - from).abs();
        if span.is_nan() || span > TAU + CLEAR {
            return None;
        }
        let (low, high) = (from.min(to), from.max(to));
        let ranks = (low / self.step()).floor() as i64..=(high / self.step()).ceil() as i64;
        let mut inner: Vec<f64> = ranks
            .map(|rank| rank as f64 * self.step())
            .filter(|theta| theta - low > CLEAR && high - theta > CLEAR)
            .collect();
        if to < from {
            inner.reverse();
        }
        let last = ((to - from).abs() > CLEAR).then_some(to);
        for theta in std::iter::once(from).chain(inner).chain(last) {
            laid.raw.push(self.at(theta));
            laid.ids.push(id);
        }
        Some(())
    }

    /// The loop `lap` laid out, each angle within half a turn of the last,
    /// and each pass through the apex laid along the floor. Read from just
    /// after its first pass, the last pass closes the loop with no turn;
    /// any other turns by less than half a turn. None when it passes twice
    /// in a row or stands at the apex alone.
    fn laid(&self, lap: &[usize]) -> Option<Laid> {
        let count = lap.len();
        let start = match lap.iter().position(|id| self.tip(*id)) {
            Some(first) => (first + 1) % count,
            None => 0,
        };
        let mut laid = Laid {
            raw: Vec::with_capacity(count),
            ids: Vec::with_capacity(count),
            turns: 0,
        };
        let mut passing: Option<usize> = None;
        let mut last: Option<f64> = None;
        for id in (0..count).map(|at| lap[(start + at) % count]) {
            if self.tip(id) {
                if passing.replace(id).is_some() {
                    return None;
                }
                continue;
            }
            let read = self.cone.parameters(self.samples.point(id));
            let theta = last.map_or(read.x, |last| last + apart(read.x, last));
            if let (Some(tip), Some(last)) = (passing.take(), last) {
                self.between(last, theta, tip, &mut laid)?;
            }
            laid.raw.push(DVec2::new(theta, read.y));
            laid.ids.push(id);
            last = Some(theta);
        }
        let (first, last) = (laid.raw.first()?.x, last?);
        match passing {
            Some(tip) => self.between(last, first, tip, &mut laid)?,
            None => {
                let round = last + apart(first, last) - first;
                laid.turns = (round / TAU).round() as i64;
            }
        }
        Some(laid)
    }

    /// The floor going once round on its own, the way `way` says, for the
    /// apex's sample `id`.
    fn round(&self, way: i64, id: usize) -> Laid {
        Laid {
            raw: (0..self.steps)
                .map(|rank| self.at(way as f64 * rank as f64 * self.step()))
                .collect(),
            ids: vec![id; self.steps],
            turns: way,
        }
    }
}

impl Outline {
    /// Lays out the loops of a face of `cone`, whose grid has `steps` steps a
    /// turn, `apex` the sample of the apex the face holds within it, if it
    /// does. None when a loop cannot be read, when a face holding its apex
    /// does not go round it once, or when a face going round has no grid
    /// angle it can be cut open at.
    pub(super) fn conical(
        &mut self,
        cone: &Cone,
        samples: &Samples,
        laps: &[Vec<usize>],
        steps: usize,
        apex: Option<usize>,
        eps: f64,
    ) -> Option<()> {
        let floor = Floor {
            cone,
            samples,
            eps,
            steps,
        };
        let mut laid: Vec<Laid> = laps
            .iter()
            .map(|lap| floor.laid(lap))
            .collect::<Option<_>>()?;
        let turns: i64 = laid.iter().map(|lap| lap.turns).sum();
        match (apex, turns) {
            (None, _) => {}
            (Some(apex), 1 | -1) => laid.push(floor.round(-turns, apex)),
            (Some(_), _) => return None,
        }
        let (raws, ids): (Vec<Vec<DVec2>>, Vec<Vec<usize>>) =
            laid.into_iter().map(|lap| (lap.raw, lap.ids)).unzip();
        self.round_laid(&raws, &ids, steps, |id| {
            !samples.is_vertex(id) || floor.tip(id)
        })
    }
}
