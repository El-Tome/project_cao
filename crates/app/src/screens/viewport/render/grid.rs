//! The grid and the drawing's own two axes, on the plane a sketch is open on.

use cao_prefs::theme::Theme;
use cao_render::{GridPlane, GridStyle, push_grid, push_plane_axes};
use cao_sketch::WorkPlane;
use glam::Vec3;

use super::{axis_style, tint};
use crate::screens::viewport::ViewScale;

/// What the grid reaches past the corners of the screen by. A grid of finite
/// size ending inside the view shows its own outer fade as a disc floating in
/// the middle of it.
const BEYOND_THE_CORNERS: f64 = 1.5;

/// How far the drawing's axes run, in multiples of the camera's distance.
const AXIS_REACH: f32 = 50.0;

/// The grid on `plane`, counted from that plane's own origin, and the two axes
/// of the drawing along with it.
pub(crate) fn push(
    out: &mut Vec<cao_render::Vertex>,
    plane: WorkPlane,
    target: Vec3,
    distance: f32,
    scale: ViewScale,
    theme: &Theme,
) {
    let half_extent =
        (scale.units_per_pixel * scale.diagonal_px as f64 * BEYOND_THE_CORNERS) as f32;
    push_grid(
        out,
        GridPlane {
            origin: plane.origin.as_vec3(),
            u: plane.u.as_vec3(),
            v: plane.v.as_vec3(),
        },
        target,
        scale.step as f32,
        half_extent,
        &GridStyle {
            minor: tint(theme.grid_minor),
            major: tint(theme.grid_major),
            minor_width: theme.grid_minor_width,
            major_width: theme.grid_major_width,
            major_every: theme.grid_major_every.max(1),
            ..GridStyle::default()
        },
    );
    push_plane_axes(
        out,
        plane.origin.as_vec3(),
        plane.u.as_vec3(),
        plane.v.as_vec3(),
        distance * AXIS_REACH,
        &axis_style(theme),
    );
}

#[cfg(test)]
mod tests;
