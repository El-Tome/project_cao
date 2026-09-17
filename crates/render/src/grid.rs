//! The adaptive grid drawn on the work plane a sketch is open on.
//!
//! It is given a plane rather than one of three named ones, so any work plane
//! gets a grid — including one lying at an angle on a face of the part.

use glam::Vec3;

use crate::geometry::{Vertex, srgb};

/// Picks the grid step from the 1–2–5–10 sequence, the smallest one whose
/// on-screen spacing stays above `target_pixel_spacing`. Zooming in therefore
/// subdivides the grid, zooming out merges it.
pub fn adaptive_step(world_units_per_pixel: f32, target_pixel_spacing: f32) -> f32 {
    let minimum = (world_units_per_pixel * target_pixel_spacing).max(1e-6);
    let decade = 10f32.powf(minimum.log10().floor());
    for multiple in [1.0, 2.0, 5.0] {
        if decade * multiple >= minimum {
            return decade * multiple;
        }
    }
    decade * 10.0
}

pub struct GridStyle {
    pub minor: [f32; 4],
    pub major: [f32; 4],
    pub minor_width: f32,
    pub major_width: f32,
    /// Every n-th line uses `major`.
    pub major_every: i32,
    /// Segments per line; more segments make the radial fade smoother.
    pub segments: i32,
}

impl Default for GridStyle {
    fn default() -> Self {
        Self {
            minor: srgb(0.55, 0.58, 0.62, 0.35),
            major: srgb(0.65, 0.68, 0.73, 0.60),
            minor_width: 1.0,
            major_width: 1.5,
            major_every: 10,
            segments: 24,
        }
    }
}

/// The plane a grid is drawn on: where its lines are counted from, and the two
/// directions they run along.
#[derive(Clone, Copy, Debug)]
pub struct GridPlane {
    pub origin: Vec3,
    pub u: Vec3,
    pub v: Vec3,
}

impl GridPlane {
    /// `center` brought onto the plane and rounded to the nearest crossing of
    /// the grid, so the lines stay put while the view is panned.
    ///
    /// Measured from the world origin rather than from the plane's own, the
    /// component along the normal is dropped — and that component is the whole
    /// of what tells a face of the part from the plane through the world
    /// origin parallel to it.
    fn snapped(&self, center: Vec3, step: f32) -> Vec3 {
        let offset = center - self.origin;
        self.origin
            + self.u * (offset.dot(self.u) / step).round() * step
            + self.v * (offset.dot(self.v) / step).round() * step
    }
}

/// A grid on `plane`, centred on `center` brought onto it and snapped to the
/// step so lines stay put while panning, fading out radially instead of ending
/// on a hard edge.
///
/// Taking a plane rather than one of three named ones means any work plane
/// gets a grid, including one lying at an angle on a face.
pub fn push_grid(
    out: &mut Vec<Vertex>,
    plane: GridPlane,
    center: Vec3,
    step: f32,
    half_extent: f32,
    style: &GridStyle,
) {
    let lines = (half_extent / step).ceil() as i32;
    let center = plane.snapped(center, step);

    for (along, across) in [(plane.u, plane.v), (plane.v, plane.u)] {
        for index in -lines..=lines {
            let offset = index as f32 * step;
            let base = center + across * offset;
            let from_origin = (base - plane.origin).dot(across);

            if from_origin.abs() < step * 0.001 {
                continue;
            }

            // "Major" must follow the distance to the origin, not the index:
            // the index is counted from the panned centre, so using it would
            // make the heavy lines drift away from the axes as you pan.
            let steps_from_origin = from_origin / (step * style.major_every as f32);
            let major = (steps_from_origin - steps_from_origin.round()).abs() < 1e-3;
            let color = if major { style.major } else { style.minor };
            let width = if major {
                style.major_width
            } else {
                style.minor_width
            };

            push_faded_line(
                out,
                &GridLine {
                    base,
                    direction: along,
                    color,
                    width,
                },
                half_extent,
                center,
                style.segments,
            );
        }
    }
}

/// One grid line, ready to be emitted as segments.
struct GridLine {
    base: Vec3,
    direction: Vec3,
    color: [f32; 4],
    width: f32,
}

/// Emits one grid line as a strip of segments whose alpha falls off with the
/// distance to the grid centre.
fn push_faded_line(
    out: &mut Vec<Vertex>,
    line: &GridLine,
    half_extent: f32,
    center: Vec3,
    segments: i32,
) {
    let GridLine {
        base,
        direction,
        color,
        width,
    } = *line;
    let segments = segments.max(1);
    let mut previous = None;

    for index in 0..=segments {
        let t = index as f32 / segments as f32 * 2.0 - 1.0;
        let point = base + direction * (t * half_extent);
        let fade = radial_fade(point.distance(center), half_extent);
        let vertex = Vertex::line(
            point,
            [color[0], color[1], color[2], color[3] * fade],
            width,
        );

        if let Some(previous) = previous {
            out.push(previous);
            out.push(vertex);
        }
        previous = Some(vertex);
    }
}

/// Fully opaque out to `FADE_START` of the extent, then falling off to
/// nothing at the edge. Fading from the very centre would draw the grid as a
/// small bright disc floating in the middle of the view.
const FADE_START: f32 = 0.55;

fn radial_fade(distance: f32, half_extent: f32) -> f32 {
    let normalized = (distance / half_extent).clamp(0.0, 1.0);
    if normalized <= FADE_START {
        return 1.0;
    }
    let outer = (normalized - FADE_START) / (1.0 - FADE_START);
    (1.0 - outer * outer).clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests;
