//! How deep each area lies, and what it is hollow of.

use glam::DVec2;

use super::{Outline, Region, lies_within};

/// Gives each area its depth — how many areas enclose it — and its holes: the
/// areas directly inside it, one level deeper. What sits inside a hole is
/// matter again, and belongs to its own area.
///
/// Holes that touch each other are one opening: where several of them tile
/// the outside of a piece of the drawing, that outside takes their place, so
/// that no wall stands between two windows sharing a side or a corner.
pub(super) fn nest(regions: &mut [Region], outsides: Vec<Outline>) {
    // `within[inner][outer]`, read once: depth and holes both ask it.
    let within: Vec<Vec<bool>> = regions
        .iter()
        .enumerate()
        .map(|(inner, region)| {
            regions
                .iter()
                .enumerate()
                .map(|(outer, other)| {
                    inner != outer && lies_within(&region.outline.points, &other.outline.points)
                })
                .collect()
        })
        .collect();
    let depths: Vec<usize> = within
        .iter()
        .map(|outers| outers.iter().filter(|inside| **inside).count())
        .collect();

    let holes: Vec<Vec<Outline>> = (0..regions.len())
        .map(|outer| {
            let mut inner: Vec<usize> = (0..regions.len())
                .filter(|inner| within[*inner][outer] && depths[*inner] == depths[outer] + 1)
                .collect();
            let mut holes = Vec::new();
            let around = &regions[outer].outline.points;
            for outside in outsides
                .iter()
                .filter(|outside| lies_within(&outside.points, around))
            {
                let tiling: Vec<usize> = inner
                    .iter()
                    .copied()
                    .filter(|hole| shares_a_place(&regions[*hole].outline.points, &outside.points))
                    .collect();
                if tiling.len() > 1 && tiles(outside, tiling.iter().map(|hole| &regions[*hole])) {
                    inner.retain(|hole| !tiling.contains(hole));
                    holes.push(outside.clone());
                }
            }
            holes.extend(inner.iter().map(|hole| regions[*hole].outline.clone()));
            holes
        })
        .collect();
    for ((region, holes), depth) in regions.iter_mut().zip(holes).zip(depths) {
        region.holes = holes;
        region.depth = depth;
    }
}

/// Whether two loops pass through a place in common: drawn from one graph,
/// a place two loops share is the very same place.
fn shares_a_place(one: &[DVec2], other: &[DVec2]) -> bool {
    one.iter().any(|place| other.contains(place))
}

/// Whether the areas cover exactly what the outside encloses.
fn tiles<'a>(outside: &Outline, areas: impl Iterator<Item = &'a Region>) -> bool {
    let enclosed = outside.area();
    let covered: f64 = areas.map(|area| area.outline.area()).sum();
    (enclosed - covered).abs() <= 1e-9 * enclosed.abs().max(1.0)
}
