//! A point well inside a face, for whatever must ask on which side of another
//! body the face lies: the middle of a vertical chord of its domain, at the
//! middle of a strip between the abscissae where its boundary has a vertex or
//! turns back, chosen as the one standing farthest from every edge.

use std::f64::consts::TAU;

use glam::DVec2;

use super::crossing::{heights, stretches};
use super::distance::{Unrolling, distance};
use crate::brep::trace::Trace;

pub(super) fn point_inside(loops: &[Vec<Trace>], unrolling: Unrolling) -> Option<DVec2> {
    let periodic = unrolling.periodic();
    let mut abscissae: Vec<f64> = loops
        .iter()
        .flatten()
        .flat_map(|trace| stretches(trace, false))
        .flat_map(|stretch| [stretch.start(), stretch.end()])
        .map(|x| if periodic { x.rem_euclid(TAU) } else { x })
        .collect();
    abscissae.sort_by(f64::total_cmp);
    abscissae.dedup();
    let mut middles: Vec<f64> = abscissae
        .windows(2)
        .map(|pair| (pair[0] + pair[1]) / 2.0)
        .collect();
    if periodic && let (Some(first), Some(last)) = (abscissae.first(), abscissae.last()) {
        middles.push((last + first + TAU) / 2.0);
    }
    let mut best: Option<(f64, DVec2)> = None;
    for x in middles {
        for [low, high] in heights(loops, x, periodic).as_chunks::<2>().0 {
            let candidate = DVec2::new(x, (low + high) / 2.0);
            let clearance = loops
                .iter()
                .flatten()
                .map(|trace| distance(trace, candidate, unrolling))
                .fold(f64::INFINITY, f64::min);
            if best.is_none_or(|(most, _)| clearance > most) {
                best = Some((clearance, candidate));
            }
        }
    }
    best.map(|(_, point)| point)
}
