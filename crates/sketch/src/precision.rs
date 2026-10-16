//! One rule for how near is the same, measured on the drawing's size.
//!
//! Three levels, each read against [`drawing_size`] so that a drawing means
//! the same thing in any unit and anywhere on the plane. The rest of the
//! crate still carries older constants of its own; they move onto these
//! levels part by part (#514).

use glam::DVec2;

use crate::sketch::Sketch;

/// How near is treated as one at the use: a tenth of a micron on a part up to
/// a metre.
pub const USE: f64 = 1e-7;

/// How near two places must stand to be the same place: a hundredth of a
/// micron on a part up to a metre.
pub const SAME: f64 = 1e-8;

/// How near a solved value must come to its dimension: a thousandth of a
/// micron on a part up to a metre.
pub const SOLVED: f64 = 1e-9;

/// How big a drawing is: the diagonal of the smallest box holding every live
/// curve by its extent, construction curves included and the origin left out.
/// One unit while nothing is drawn, or while what is drawn spans under a
/// millionth of one — the solver's own guard against a collapsed drawing — so
/// a tolerance read against it still means something.
pub fn drawing_size(sketch: &Sketch) -> f64 {
    let mut corners: Option<(DVec2, DVec2)> = None;
    let mut take = |low: DVec2, high: DVec2| {
        corners = Some(match corners {
            None => (low, high),
            Some((min, max)) => (min.min(low), max.max(high)),
        });
    };
    for (_, segment) in sketch.live_segments() {
        let (from, to) = (sketch.point(segment.start), sketch.point(segment.end));
        take(from.min(to), from.max(to));
    }
    for (id, _) in sketch.live_arcs() {
        let (low, high) = crate::arcing::bounds_of(sketch.arc_draft(id));
        take(low, high);
    }
    for (_, circle) in sketch.live_circles() {
        let centre = sketch.point(circle.center);
        let reach = DVec2::splat(circle.radius);
        take(centre - reach, centre + reach);
    }
    for (id, _) in sketch.live_ellipses() {
        let (low, high) = sketch.ellipse_bounds(id);
        take(low, high);
    }
    corners
        .map(|(min, max)| (max - min).length())
        .filter(|size| *size > 1e-6)
        .unwrap_or(1.0)
}

#[cfg(test)]
mod tests;
