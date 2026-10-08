//! A drawing's areas read into plain data: what the rules are held against.

use std::f64::consts::TAU;

use cao_sketch::{Outline, Region, Sketch};
use glam::DVec2;

use super::rules::{Area, reach};

/// How finely a whole curve is sampled for tinting: `FULL_CIRCLE_STEPS` and
/// `FULL_ELLIPSE_STEPS` (`crates/sketch/src/arcing.rs`, `ellipsing.rs`). A run
/// of a curve takes its share of them, rounded up, so no step turns further.
const STEPS_PER_TURN: f64 = 48.0;

/// What a run may be off by, as a share of how far the drawing reaches, once
/// its ends are welded where the walk calls two places one (`THE_SAME_PLACE`,
/// `crates/sketch/src/edges.rs`): a curve sampled from an end standing that
/// far off it has its radius off by as much all the way round.
const WELDED: f64 = 1e-6;

/// Every area of the drawing, as plain data.
pub fn areas_of(sketch: &Sketch) -> Vec<Area> {
    let regions = sketch.regions();
    regions
        .iter()
        .map(|region| Area {
            outline: region.outline.points.clone(),
            holes: region
                .holes
                .iter()
                .map(|hole| hole.points.clone())
                .collect(),
            triangles: region.triangles.clone(),
            measure: measure(region, &regions, regions.len()),
            sampling: sampling(sketch, &region.outline),
        })
        .collect()
}

/// What an outline encloses, read on its curves: the area of the region it
/// bounds, with what that region is hollow of added back — each hole being
/// the outline of an area one level deeper, or the outside of several such
/// areas touching each other (#540), measured the same way.
fn measure(region: &Region, regions: &[Region], depth: usize) -> f64 {
    let hollow: f64 = region
        .holes
        .iter()
        .map(|hole| {
            let tiling: Vec<&Region> = regions
                .iter()
                .filter(|inner| {
                    inner.depth == region.depth + 1
                        && inner
                            .outline
                            .points
                            .iter()
                            .any(|place| hole.points.contains(place))
                })
                .collect();
            match regions
                .iter()
                .find(|inner| inner.outline.points == hole.points)
            {
                Some(inner) if depth > 0 => measure(inner, regions, depth - 1),
                None if depth > 0 && tiling.len() > 1 => tiling
                    .iter()
                    .map(|inner| measure(inner, regions, depth - 1))
                    .sum(),
                _ => shoelace(&hole.points),
            }
        })
        .sum();
    region.area() + hollow
}

fn shoelace(places: &[DVec2]) -> f64 {
    (0..places.len())
        .map(|index| places[index].perp_dot(places[(index + 1) % places.len()]))
        .sum::<f64>()
        / 2.0
}

/// How far the tint of an outline may stand from its measure: what its
/// curved steps leave out or take in, plus a little for the ends a weld moved.
///
/// Read off the steps the outline actually walks, not off the curves it names:
/// a small piece cut from a big circle is allowed what its own few steps can
/// stray, and a walk that runs further round a curve than is drawn is allowed
/// what it walked. A straight step across a stretch of circle turning by `θ`,
/// a chord `c` long, leaves out `c² (θ − sin θ) / (8 sin²(θ/2))`, which grows
/// with `θ`; across a stretch of ellipse, where the steps are spread by its
/// own turn, at most that times how much longer it is than it is wide.
fn sampling(sketch: &Sketch, outline: &Outline) -> f64 {
    let step = TAU / STEPS_PER_TURN;
    let stray = (step - step.sin()) / (8.0 * (step / 2.0).sin().powi(2));
    let longest = sketch
        .live_ellipses()
        .filter(|(_, ellipse)| !ellipse.construction)
        .map(|(id, _)| {
            let drawn = sketch.ellipse_draft(id);
            let (along, across) = (drawn.first.length(), drawn.second);
            along.max(across) / along.min(across)
        })
        .fold(1.0, f64::max);
    let places = &outline.points;
    let curved: f64 = outline
        .curves
        .iter()
        .enumerate()
        .filter(|(_, run)| run.is_some())
        .map(|(index, _)| {
            let chord = places[index].distance(places[(index + 1) % places.len()]);
            chord * chord * stray * longest
        })
        .sum();
    let mut runs: Vec<usize> = outline.curves.iter().flatten().copied().collect();
    runs.dedup();
    let reach = reach(places);
    curved + runs.len() as f64 * WELDED * (1.0 + reach) * reach
}
