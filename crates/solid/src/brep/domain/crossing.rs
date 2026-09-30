//! Where the loops of a face cross the vertical line through an abscissa: the
//! heights a ray up the second parameter meets. Counted half-open, so that a
//! vertex and a vertical stretch count once; on a cylinder the abscissa is an
//! angle, taken modulo a turn.

use std::f64::consts::{FRAC_PI_2, PI, TAU};

use crate::brep::trace::Trace;

/// A stretch of a trace, between two of its parameters, over which its first
/// parameter only grows or only shrinks — and on a cylinder by no more than a
/// quarter turn, so that passing the line and passing the far side of the
/// cylinder cannot be taken for one another.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct Stretch {
    pub(super) trace: Trace,
    pub(super) from: f64,
    pub(super) to: f64,
}

impl Stretch {
    pub(super) fn start(&self) -> f64 {
        self.trace.at(self.from)[0].x
    }

    pub(super) fn end(&self) -> f64 {
        self.trace.at(self.to)[0].x
    }
}

/// The heights at which the loops cross the line through `abscissa`, sorted.
///
/// Each stretch ends exactly where the next one starts, the same number read
/// once, so a ray through a vertex is counted by one stretch and not by the
/// other, whatever the rounding of either trace.
pub(super) fn heights(loops: &[Vec<Trace>], abscissa: f64, periodic: bool) -> Vec<f64> {
    let mut found = Vec::new();
    for lap in loops {
        let stretches: Vec<Stretch> = lap
            .iter()
            .flat_map(|trace| stretches(trace, periodic))
            .collect();
        let starts: Vec<f64> = stretches.iter().map(Stretch::start).collect();
        for (index, stretch) in stretches.iter().enumerate() {
            let start = starts[index];
            let end = starts[(index + 1) % starts.len()];
            let [before, after] = [start, end].map(|x| offset(x - abscissa, periodic));
            if periodic && (after - before).abs() > PI {
                continue;
            }
            if (before > 0.0) != (after > 0.0) {
                found.push(height(stretch, start - before));
            }
        }
    }
    found.sort_by(f64::total_cmp);
    found
}

/// How far an abscissa stands past another: on a cylinder the nearest way
/// round, in `[-π, π]`.
fn offset(gap: f64, periodic: bool) -> f64 {
    if periodic {
        gap - TAU * (gap / TAU).round()
    } else {
        gap
    }
}

/// A trace cut into stretches: a segment in quarter turns on a cylinder, a
/// round wherever it turns back, which is at every half turn of its angle.
pub(super) fn stretches(trace: &Trace, periodic: bool) -> Vec<Stretch> {
    let cuts = match *trace {
        Trace::Segment { from, to } => {
            let count = if periodic {
                ((to.x - from.x).abs() / FRAC_PI_2).ceil().max(1.0) as usize
            } else {
                1
            };
            (1..count).map(|cut| cut as f64 / count as f64).collect()
        }
        Trace::Round { start, sweep, .. } => {
            let (low, high) = (start.min(start + sweep), start.max(start + sweep));
            let first = (low / PI).floor() as i64 + 1;
            let last = (high / PI).ceil() as i64 - 1;
            let mut cuts: Vec<f64> = (first..=last)
                .map(|turn| (turn as f64 * PI - start) / sweep)
                .filter(|cut| *cut > 0.0 && *cut < 1.0)
                .collect();
            cuts.sort_by(f64::total_cmp);
            cuts
        }
        Trace::Graph { .. } => sampled(trace, periodic),
    };
    let bounds: Vec<f64> = std::iter::once(0.0)
        .chain(cuts)
        .chain(std::iter::once(1.0))
        .collect();
    bounds
        .windows(2)
        .map(|pair| Stretch {
            trace: *trace,
            from: pair[0],
            to: pair[1],
        })
        .collect()
}

/// Where any trace turns back in its first parameter, found between samples,
/// and on a cylinder where it has gone a quarter turn since the last cut.
pub(super) fn sampled(trace: &Trace, periodic: bool) -> Vec<f64> {
    const SAMPLES: usize = 64;
    let pace = |u: f64| trace.at(u)[1].x;
    let place = |u: f64| trace.at(u)[0].x;
    let mut turns = Vec::new();
    for sample in 0..SAMPLES {
        let (low, high) = (
            sample as f64 / SAMPLES as f64,
            (sample + 1) as f64 / SAMPLES as f64,
        );
        if pace(low) * pace(high) < 0.0 {
            turns.push(bisect(pace, low, high));
        }
    }
    if !periodic {
        return turns;
    }
    let bounds: Vec<f64> = std::iter::once(0.0)
        .chain(turns)
        .chain(std::iter::once(1.0))
        .collect();
    let mut cuts = Vec::new();
    for pair in bounds.windows(2) {
        let (start, end) = (place(pair[0]), place(pair[1]));
        let count = ((end - start).abs() / FRAC_PI_2).ceil().max(1.0) as usize;
        for cut in 1..count {
            let target = start + (end - start) * cut as f64 / count as f64;
            cuts.push(bisect(|u| place(u) - target, pair[0], pair[1]));
        }
        if pair[1] < 1.0 {
            cuts.push(pair[1]);
        }
    }
    cuts
}

/// The height of a stretch where its first parameter is `target`.
fn height(stretch: &Stretch, target: f64) -> f64 {
    match stretch.trace {
        Trace::Segment { from, to } => {
            let along = if to.x == from.x {
                (stretch.from + stretch.to) / 2.0
            } else {
                ((target - from.x) / (to.x - from.x)).clamp(stretch.from, stretch.to)
            };
            from.y + (to.y - from.y) * along
        }
        Trace::Round {
            center,
            radius,
            start,
            sweep,
        } => {
            let middle = start + sweep * (stretch.from + stretch.to) / 2.0;
            let cosine = ((target - center.x) / radius).clamp(-1.0, 1.0);
            let sine = ((1.0 - cosine) * (1.0 + cosine)).sqrt();
            center.y + radius * sine.copysign(middle.sin())
        }
        Trace::Graph { .. } => bisected(stretch, target),
    }
}

/// The height of any stretch where its first parameter is `target`, found by
/// halving the stretch, along which that parameter is monotone.
pub(super) fn bisected(stretch: &Stretch, target: f64) -> f64 {
    let place = |u: f64| stretch.trace.at(u)[0].x - target;
    stretch.trace.at(bisect(place, stretch.from, stretch.to))[0].y
}

/// Where `f` changes sign between `low` and `high`, to the last bit.
fn bisect(f: impl Fn(f64) -> f64, mut low: f64, mut high: f64) -> f64 {
    let rising = f(low) <= 0.0;
    loop {
        let middle = (low + high) / 2.0;
        if middle <= low || middle >= high {
            return middle;
        }
        if (f(middle) <= 0.0) == rising {
            low = middle;
        } else {
            high = middle;
        }
    }
}
