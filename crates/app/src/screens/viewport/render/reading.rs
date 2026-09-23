//! What a measure looks like: a dashed line between what was read, and the
//! numbers beside it.
//!
//! Dashed, and in the quieter of the two dimension colours, because the one
//! thing a measure must never be mistaken for is a dimension. A dimension is a
//! promise the drawing is held to; a measure is a glance, and it is gone on
//! the next click. The shape itself is `Sketch::place`'s — the same geometry a
//! dimension is drawn with, so an angle between two traits lying apart is
//! solved once rather than twice — and only the dashes and the label are this
//! module's own.

use cao_prefs::theme::Theme;
use cao_sketch::Sketch;
use glam::DVec2;

use super::curves::push_line;
use super::overlays::{tint_to_color, to_screen};
use super::tint;
use crate::screens::annotations::metrics;
use crate::screens::viewport::input::measure_showing;
use crate::screens::viewport::{SketchContext, ViewScale, ViewportState};

/// How big the numbers are drawn, matching the dimensions' own labels.
const TEXT_POINTS: f32 = 13.0;

/// The dashed run a measure is drawn as.
pub(crate) fn push_measure(
    out: &mut Vec<cao_render::Vertex>,
    sketch: &Sketch,
    context: &SketchContext<'_>,
    theme: &Theme,
    scale: ViewScale,
) {
    let Some(target) = measure_showing(context) else {
        return;
    };
    let Some(placed) = sketch.place(target, metrics(scale.units_per_pixel, DVec2::ZERO)) else {
        return;
    };
    let color = tint(theme.dimension_driven);
    for (from, to) in placed.shape {
        push_line(
            out,
            sketch.plane.to_world(from),
            sketch.plane.to_world(to),
            color,
            theme.sketch_width,
            true,
            scale,
        );
    }
}

/// The numbers a measure says, written where the dimension's value would go.
///
/// Several lines rather than one: a measure is not held to being a single
/// value the way a dimension is, so it says the distance and the reach along
/// each axis, or the radius and the diameter, at once.
pub(crate) fn paint_measure(
    ui: &egui::Ui,
    state: &ViewportState,
    rect: egui::Rect,
    context: &SketchContext<'_>,
) {
    let Some(target) = measure_showing(context) else {
        return;
    };
    let Some(index) = context.editor.active_sketch() else {
        return;
    };
    let Some(sketch) = context.document.sketches().get(index) else {
        return;
    };
    let Some(reading) = sketch.read(target) else {
        return;
    };
    // The very pixel size the dashes were drawn with. Read any other way, the
    // label works out `text_at` for an annotation standing further off the
    // geometry than the one on screen, and floats clear of the run it belongs
    // to — by the ratio of the two, which on a retina screen is more than
    // double.
    let pixel = state
        .camera
        .world_units_per_pixel(rect.height() * ui.ctx().pixels_per_point()) as f64;
    let Some(placed) = sketch.place(target, metrics(pixel, DVec2::ZERO)) else {
        return;
    };
    let view_projection = state
        .camera
        .view_projection(rect.width() / rect.height().max(1.0));
    let Some(at) = to_screen(sketch.plane.to_world(placed.text_at), view_projection, rect) else {
        return;
    };

    let said = crate::wording::measure::lines(
        context.lang,
        target,
        reading.scaled(context.document.scale()),
        state.config.unit,
    )
    .join("\n");
    // Clipped to the canvas, like every other painter here: `to_screen` hands
    // back a position for a place behind the viewport's edge just as readily as
    // for one inside it, and a number drawn over the toolbar belongs to nothing.
    ui.painter_at(rect).text(
        at,
        egui::Align2::CENTER_CENTER,
        said,
        egui::FontId::proportional(TEXT_POINTS),
        tint_to_color(state.theme.dimension_driven),
    );
}

#[cfg(test)]
mod tests;
