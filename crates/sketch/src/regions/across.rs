//! An area cut along a line, one piece per side: what a turn about an axis
//! running through the middle of an area makes of each side.

use glam::DVec2;

use super::Region;
use crate::sketch::Sketch;

impl Sketch {
    /// The pieces the area falls into once a trait is drawn along the line
    /// through `origin` running along `direction`, the pieces left of the line
    /// first.
    ///
    /// The line is drawn on a copy, right across the area, and the copy's own
    /// walk does the cutting: a circle crossed comes back as two arcs that
    /// keep their bend, which a cut written again here would have to learn.
    /// Whatever the drawn line closes outside the area is left out.
    pub fn pieces_across(&self, region: &Region, origin: DVec2, direction: DVec2) -> Vec<Region> {
        let along = direction.normalize_or_zero();
        let Some((low, high)) = reach_along(region, origin, along) else {
            return vec![region.clone()];
        };
        let past = high - low;
        let mut cut = self.clone();
        let start = cut.add_point(origin + along * (low - past));
        let end = cut.add_point(origin + along * (high + past));
        cut.add_segment(start, end);

        let mut pieces: Vec<(Region, DVec2)> = cut
            .regions()
            .into_iter()
            .filter_map(|piece| {
                let inside = inner_point(&piece)?;
                region.contains(inside).then_some((piece, inside))
            })
            .collect();
        pieces.sort_by_key(|(_, inside)| along.perp_dot(*inside - origin) <= 0.0);
        pieces.into_iter().map(|(piece, _)| piece).collect()
    }
}

/// How far along the line the area reaches either way from `origin`, or
/// nothing when the line runs nowhere or the area has no extent along it.
fn reach_along(region: &Region, origin: DVec2, along: DVec2) -> Option<(f64, f64)> {
    if along == DVec2::ZERO {
        return None;
    }
    let (low, high) = region
        .outline
        .points
        .iter()
        .map(|point| (*point - origin).dot(along))
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(low, high), at| {
            (low.min(at), high.max(at))
        });
    (high > low).then_some((low, high))
}

/// A point strictly inside the piece's own face, its holes left out.
fn inner_point(piece: &Region) -> Option<DVec2> {
    let [a, b, c] = *piece.face_triangles().first()?;
    Some((a + b + c) / 3.0)
}

#[cfg(test)]
mod tests;
