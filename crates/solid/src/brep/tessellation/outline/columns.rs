//! A face laid out in a chart whose first coordinate is an angle — a cone's
//! `(θ, l)` — cut in columns between the rulings of its grid, each swept
//! alone, so that no triangle reaches across a ruling: its chord then sags
//! no deeper than a step of the grid, as the triangles of a band between
//! two rims do, whatever the curves the face is bounded by (#577).
//!
//! Every edge of a cone's face is sampled where it crosses a ruling of the
//! grid, so no segment of the outline crosses one between its ends. A
//! sample standing on a ruling, to `CLEAR`, is laid on it to the bit; a
//! ruling is then cut into intervals by the points standing on it, each
//! wholly inside the face or wholly out, and decided at its middle by the
//! winding of the outline round it. An interval inside closes the column on
//! its left going up and the column on its right going down; a segment
//! along a ruling closes the column its region lies in. No point is added.
//!
//! `(θ, l)` is not the cone rolled out flat: away from a ruling, a straight
//! segment there is no straight line on the surface. A side running almost
//! along a ruling, fanned from a corner far along it, is cut into a sliver
//! the right way round in the chart and upside down in space (#820). A
//! column one of whose triangles stands flat or the other way round on the
//! cone rolled out is swept there instead, on copies of its points standing
//! for the same samples; where it cannot be, it keeps the chart's cut. Every
//! column rolled out was tried and set aside: it snips ears off a conic past
//! the tolerance (`conics::a_point_cut_at_a_slant_below_a_narrow_rim_is_drawn_closed_and_uncrossed`).
//!
//! Where a segment does cross a ruling, or a column does not sweep, the face
//! is left whole, as it was laid out. The apex of a face holding it is a
//! line of the chart standing for one point, which these columns do not yet
//! read.

use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::f64::consts::TAU;

use glam::DVec2;

use super::super::sweep;
use super::Outline;
use super::cylinder::CLEAR;
use crate::brep::surface::Cone;
use crate::orientation::turn;

/// The points, the sample each stands for, the segments and the strips of
/// a face cut in columns.
type Columns = (Vec<DVec2>, Vec<usize>, Vec<[usize; 2]>, Vec<Vec<usize>>);

impl Outline {
    /// The face laid out so far on `cone` cut in columns between the rulings
    /// of a grid of `steps` angles a turn, where it can be; left whole
    /// otherwise.
    pub(super) fn columned(&mut self, steps: usize, cone: &Cone) {
        if let Some((points, samples, segments, strips)) = self.columns(steps, cone) {
            self.points = points;
            self.samples = samples;
            self.segments = segments;
            self.strips = strips;
        }
    }

    fn columns(&self, steps: usize, cone: &Cone) -> Option<Columns> {
        let step = TAU / steps as f64;
        let ruling = |rank: i64| TAU * rank as f64 / steps as f64;
        let mut points = self.points.clone();
        let ranks: Vec<Option<i64>> = points
            .iter_mut()
            .map(|point| {
                let rank = (point.x / step).round() as i64;
                ((point.x - ruling(rank)).abs() <= CLEAR).then(|| {
                    point.x = ruling(rank);
                    rank
                })
            })
            .collect();

        let mut segments = self.segments.clone();
        let mut columns: BTreeMap<i64, Vec<usize>> = BTreeMap::new();
        for (index, &[from, to]) in segments.iter().enumerate() {
            let (a, b) = (points[from], points[to]);
            let column = match (ranks[from], ranks[to]) {
                (Some(one), Some(other)) if one == other => {
                    if b.y > a.y {
                        one - 1
                    } else {
                        one
                    }
                }
                _ => {
                    let column = ((a.x + b.x) / 2.0 / step).floor() as i64;
                    let (low, high) = (a.x.min(b.x), a.x.max(b.x));
                    if low < ruling(column) || high > ruling(column + 1) {
                        return None;
                    }
                    column
                }
            };
            columns.entry(column).or_default().push(index);
        }

        let mut on: BTreeMap<i64, Vec<usize>> = BTreeMap::new();
        for (point, rank) in ranks.iter().enumerate() {
            if let Some(rank) = rank {
                on.entry(*rank).or_default().push(point);
            }
        }
        for (rank, mut along) in on {
            along.sort_by(|one, other| points[*one].y.total_cmp(&points[*other].y));
            for pair in along.windows(2) {
                let [low, high] = [pair[0], pair[1]];
                let bounded = self
                    .segments
                    .iter()
                    .any(|&[from, to]| (from == low && to == high) || (from == high && to == low));
                let middle = DVec2::new(ruling(rank), (points[low].y + points[high].y) / 2.0);
                if bounded || winding(&points, &segments, middle) == 0 {
                    continue;
                }
                segments.push([low, high]);
                columns
                    .entry(rank - 1)
                    .or_default()
                    .push(segments.len() - 1);
                segments.push([high, low]);
                columns.entry(rank).or_default().push(segments.len() - 1);
            }
        }

        let mut samples = self.samples.clone();
        let mut strips = Vec::new();
        for (column, strip) in columns {
            let local: Vec<[usize; 2]> = strip.iter().map(|&index| segments[index]).collect();
            let mut used: Vec<usize> = local.iter().flatten().copied().collect();
            used.sort_unstable();
            used.dedup();
            let placed: Vec<DVec2> = used.iter().map(|&point| points[point]).collect();
            let renumbered: Vec<[usize; 2]> = local
                .iter()
                .map(|pair| pair.map(|point| used.binary_search(&point).unwrap_or(0)))
                .collect();
            let cut = sweep::triangles(&placed, &renumbered)?;
            let middle = ruling(column) + step / 2.0;
            let unrolled: Vec<DVec2> = placed
                .iter()
                .map(|at| unrolled(cone, *at, middle))
                .collect();
            let folded = cut
                .iter()
                .any(|&[a, b, c]| turn(unrolled[a], unrolled[b], unrolled[c]) != Ordering::Greater);
            if folded && sweep::triangles(&unrolled, &renumbered).is_some() {
                let first = points.len();
                points.extend(unrolled);
                samples.extend(used.iter().map(|&point| self.samples[point]));
                for (&index, pair) in strip.iter().zip(&renumbered) {
                    segments[index] = pair.map(|point| first + point);
                }
            }
            strips.push(strip);
        }
        Some((points, samples, segments, strips))
    }
}

/// `at`, of `(θ, l)`, on the cone rolled out flat, its ruling at `middle`
/// along the second axis: there a segment is a straight line on the
/// surface, which in `(θ, l)` it is only along a ruling.
fn unrolled(cone: &Cone, at: DVec2, middle: f64) -> DVec2 {
    let opening = cone.ruling.y;
    let angle = opening * (at.x - middle);
    let across = cone.radius_at(at.y) * angle.sin() / opening;
    DVec2::new(across, at.y - across * (angle / 2.0).tan())
}

/// How many times the outline winds round `at`, counterclockwise counted
/// up: a ray from it along the first coordinate, each segment crossing it
/// counted by the way it runs, half-open at its ends.
pub(super) fn winding(points: &[DVec2], segments: &[[usize; 2]], at: DVec2) -> i64 {
    segments
        .iter()
        .map(|&[from, to]| (points[from], points[to]))
        .filter(|(a, b)| (a.y <= at.y) != (b.y <= at.y))
        .filter(|(a, b)| a.x + (at.y - a.y) * (b.x - a.x) / (b.y - a.y) > at.x)
        .map(|(a, b)| if b.y > a.y { 1 } else { -1 })
        .sum()
}

#[cfg(test)]
mod tests;
