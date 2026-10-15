//! Where a creeping sweep is heading, read off its last passes.
//!
//! A drawing holding a shape that almost gives — a chamfer on a corner a few
//! degrees from straight (#592) — is corrected one row at a time along a way
//! no row holds firmly, and the sweep creeps: a fifth of a percent of its
//! error a pass, thousands of passes a solve. Each pass then moves the drawing
//! a little less than the one before, and the same way. Where they are going
//! is read off the last few and gone to: Anderson's mixing, the place the
//! passes come to once the drawing is near enough to settled that a pass
//! moves it as a linear map would.
//!
//! Only while the error is above the tolerance: the passes that only polish a
//! drawing already holding its rules are left to themselves, since mixing
//! them would carry a point a rule barely holds as far as it likes.

use std::collections::VecDeque;

use glam::DVec2;

use crate::sketch::{CircleId, PointId, Sketch};

use super::TOLERANCE;

/// How many passes back a mix reads.
const PASSES: usize = 5;
/// How far the mixing may take any point from where it began, as a share of
/// the drawing's size. Where a sweep creeps because a shape almost gives, the
/// passes go a few hundredths of the drawing; mixed along a way the rows
/// barely hold, a point can be carried as far as it likes, which no number of
/// passes would ever do. Past this, the drawing is put back as the mixing
/// found it and the sweep goes on by its passes alone.
const FARTHEST: f64 = 0.1;
/// How much the error may grow past the least it has had before the passes
/// read are dropped and the reading begun again.
const GROWN: f64 = 10.0;
/// Under this, a column of differences adds nothing the earlier ones did not
/// say, as a share of its own length.
const NOTHING_NEW: f64 = 1e-10;

/// The unknowns a pass started from and the ones it left.
struct Pass {
    from: Vec<f64>,
    to: Vec<f64>,
}

/// How a sweep's passes are being taken.
enum Taken {
    /// One after the other, as they come.
    AsTheyCome,
    /// Mixed into where they are heading, from the drawing as it `began`.
    Mixed {
        began: Vec<f64>,
        passes: VecDeque<Pass>,
        least: f64,
    },
    /// As they come again, for good: the mixing went further than any pass
    /// would.
    GivenUp,
}

/// The last passes of a sweep, and whether they are being mixed.
pub(super) struct Heading {
    taken: Taken,
    from: Vec<f64>,
}

impl Heading {
    pub(super) fn new() -> Self {
        Self {
            taken: Taken::AsTheyCome,
            from: Vec::new(),
        }
    }

    /// Before a pass: where it starts from, when it is to be read.
    pub(super) fn starting(&mut self, sketch: &Sketch) {
        if let Taken::Mixed { .. } = self.taken {
            self.from = unknowns(sketch);
        }
    }

    /// After a pass that left `worst`: the drawing taken where the passes are
    /// heading, once they are found `creeping` above the tolerance. Says
    /// whether the drawing was put back as the mixing found it instead.
    pub(super) fn passed(
        &mut self,
        sketch: &mut Sketch,
        worst: f64,
        creeping: bool,
        pinned: &[bool],
        scale: f64,
    ) -> bool {
        let Taken::Mixed {
            began,
            passes,
            least,
        } = &mut self.taken
        else {
            if matches!(self.taken, Taken::AsTheyCome) && creeping && worst >= TOLERANCE {
                self.taken = Taken::Mixed {
                    began: unknowns(sketch),
                    passes: VecDeque::new(),
                    least: worst,
                };
            }
            return false;
        };
        if worst < TOLERANCE {
            self.taken = Taken::AsTheyCome;
            return false;
        }
        if worst > *least * GROWN {
            passes.clear();
        }
        *least = least.min(worst);
        passes.push_back(Pass {
            from: std::mem::take(&mut self.from),
            to: unknowns(sketch),
        });
        if passes.len() > PASSES + 1 {
            passes.pop_front();
        }
        let Some(heading) = mixed(passes) else {
            return false;
        };
        if farthest(&heading, began, sketch.points().len()) > FARTHEST * scale {
            place(sketch, began, pinned);
            self.taken = Taken::GivenUp;
            return true;
        }
        place(sketch, &heading, pinned);
        false
    }
}

/// Anderson's mix of the passes read: the last place a pass left, corrected
/// by the differences between passes that best cancel what the last pass
/// still moved.
fn mixed(passes: &VecDeque<Pass>) -> Option<Vec<f64>> {
    if passes.len() < 2 {
        return None;
    }
    let moved: Vec<Vec<f64>> = passes
        .iter()
        .map(|pass| difference(&pass.to, &pass.from))
        .collect();
    let pairs = passes.len() - 1;
    let changes: Vec<Vec<f64>> = (0..pairs)
        .map(|at| difference(&moved[at + 1], &moved[at]))
        .collect();
    let weights = least_squares(&changes, &moved[pairs]);
    if weights.iter().any(|weight| !weight.is_finite()) {
        return None;
    }
    let mut heading = passes[pairs].to.clone();
    for (at, weight) in weights.iter().enumerate() {
        let step = difference(&passes[at + 1].to, &passes[at].to);
        for (value, change) in heading.iter_mut().zip(step) {
            *value -= weight * change;
        }
    }
    Some(heading)
}

/// The weights `w` that make `columns · w` nearest `target`: the columns
/// orthonormalised one after another, a column saying nothing new given no
/// weight.
fn least_squares(columns: &[Vec<f64>], target: &[f64]) -> Vec<f64> {
    let count = columns.len();
    let mut basis: Vec<Vec<f64>> = Vec::new();
    let mut kept: Vec<usize> = Vec::new();
    let mut triangle = vec![vec![0.0; count]; count];
    for (at, column) in columns.iter().enumerate() {
        let length = norm(column);
        let mut left = column.clone();
        for (row, unit) in basis.iter().enumerate() {
            let along = dot(unit, &left);
            triangle[row][at] = along;
            for (value, part) in left.iter_mut().zip(unit) {
                *value -= along * part;
            }
        }
        let rest = norm(&left);
        if rest <= NOTHING_NEW * length {
            continue;
        }
        triangle[basis.len()][at] = rest;
        left.iter_mut().for_each(|value| *value /= rest);
        basis.push(left);
        kept.push(at);
    }
    let projected: Vec<f64> = basis.iter().map(|unit| dot(unit, target)).collect();
    let mut weights = vec![0.0; count];
    for row in (0..kept.len()).rev() {
        let mut sum = projected[row];
        for later in row + 1..kept.len() {
            sum -= triangle[row][kept[later]] * weights[kept[later]];
        }
        weights[kept[row]] = sum / triangle[row][kept[row]];
    }
    weights
}

fn dot(one: &[f64], other: &[f64]) -> f64 {
    one.iter().zip(other).map(|(a, b)| a * b).sum()
}

fn norm(values: &[f64]) -> f64 {
    dot(values, values).sqrt()
}

fn difference(one: &[f64], other: &[f64]) -> Vec<f64> {
    one.iter().zip(other).map(|(a, b)| a - b).collect()
}

/// The most any point stands apart, or any circle's size, between two
/// readings of the unknowns of a drawing of `points` points.
fn farthest(one: &[f64], other: &[f64], points: usize) -> f64 {
    let (one_points, one_sizes) = one.split_at(points * 2);
    let (other_points, other_sizes) = other.split_at(points * 2);
    let apart = one_points
        .as_chunks::<2>()
        .0
        .iter()
        .zip(other_points.as_chunks::<2>().0)
        .map(|([ax, ay], [bx, by])| DVec2::new(ax - bx, ay - by).length());
    let resized = one_sizes
        .iter()
        .zip(other_sizes)
        .map(|(a, b)| (a - b).abs());
    apart.chain(resized).fold(0.0, f64::max)
}

/// Every unknown of the drawing in one row: each point's two coordinates,
/// then each circle's size.
fn unknowns(sketch: &Sketch) -> Vec<f64> {
    let mut all: Vec<f64> = sketch
        .points()
        .iter()
        .flat_map(|point| [point.x, point.y])
        .collect();
    all.extend(sketch.circles().iter().map(|circle| circle.radius));
    all
}

/// The drawing set to `values`, its pinned points where they are.
fn place(sketch: &mut Sketch, values: &[f64], pinned: &[bool]) {
    let points = sketch.points().len();
    for (point, held) in pinned.iter().enumerate() {
        if !held {
            sketch.place_point(
                PointId(point),
                DVec2::new(values[point * 2], values[point * 2 + 1]),
            );
        }
    }
    for circle in 0..sketch.circles().len() {
        let change = values[points * 2 + circle] - sketch.circles()[circle].radius;
        if change != 0.0 {
            sketch.grow_circle(CircleId(circle), change);
        }
    }
}

#[cfg(test)]
mod tests;
