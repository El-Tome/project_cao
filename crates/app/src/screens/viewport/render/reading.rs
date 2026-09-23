//! What a measure looks like: a right triangle over what was read, each side
//! carrying its own number.
//!
//! A straight run has three things to say — how far, how far across, how far
//! up — and stacking them in a block beside the drawing made a paragraph to
//! read. As a triangle they are one shape: the hypotenuse is the distance, and
//! the two legs *are* the reaches, drawn in the colours the sketch's own axes
//! are drawn in. Which reach is which needs no word once the side is red or
//! green.
//!
//! Dashed, and never in the dimensions' colour, because the one thing a
//! measure must not be mistaken for is a dimension: a dimension is a promise
//! the drawing is held to, a measure is a glance. A circle and an angle are
//! not runs and keep `Sketch::place`'s own shape, which already solves the two
//! awkward angles — traits lying apart, a trait against an axis.

use cao_prefs::theme::{Rgba, Theme};
use cao_sketch::Sketch;
use glam::DVec2;

use super::curves::push_line;
use super::overlays::{tint_to_color, to_screen};
use super::tint;
use crate::screens::annotations::metrics;
use crate::screens::viewport::input::measure_showing;
use crate::screens::viewport::{SketchContext, ViewScale, ViewportState};
use crate::wording::measure::Said;

/// How big the numbers are drawn, matching the dimensions' own labels.
const TEXT_POINTS: f32 = 13.0;

/// How far off its side a number sits, in points, so the line does not run
/// through the digits.
const CLEAR_OF_THE_SIDE: f32 = 10.0;

/// A reach thinner than this on screen, in pixels, is no triangle at all.
///
/// Read on screen rather than as an angle, because that is where the trouble
/// is: the same half-degree trait is a sliver zoomed out and a proper triangle
/// zoomed right in, and it is only worth coming apart when there is room to
/// see it come apart.
const A_TRIANGLE_NEEDS: f64 = 18.0;

/// Whether a run has two reaches worth reading, or is square to an axis.
///
/// Square to an axis, its length *is* its reach along that axis and the other
/// is nothing: the long leg lies along the run and the short one is a dot, so
/// the triangle is one line drawn three times over with two numbers on the
/// same spot. The drawing tool already refuses to offer a trait square to an
/// axis a width and a height for the same reason — two names for one
/// measurement is one name too many.
fn comes_apart(from: DVec2, to: DVec2, units_per_pixel: f64) -> bool {
    let reach = (to - from).abs() / units_per_pixel.max(f64::MIN_POSITIVE);
    reach.x.min(reach.y) >= A_TRIANGLE_NEEDS
}

/// The three sides of the triangle a straight run is shown as, in the drawing's
/// own coordinates: the run itself, then the reach across and the reach up.
///
/// The corner is taken level with the first place and under the second, so the
/// triangle always falls the same side of the run rather than flipping about as
/// the two ends are picked in one order or the other.
fn sides(from: DVec2, to: DVec2) -> [(DVec2, DVec2); 3] {
    let corner = DVec2::new(to.x, from.y);
    [(from, to), (from, corner), (corner, to)]
}

/// What colour each side is drawn in: the run in the measure's own colour, and
/// each reach in the colour its axis already wears on the canvas.
fn colours(theme: &Theme) -> [Rgba; 3] {
    [theme.measure, theme.axis_x, theme.axis_y]
}

/// The dashed triangle, or — for what is not a run — the annotation's own
/// shape.
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
    let width = theme.sketch_width;
    if let Some((from, to)) = sketch.run_of(target) {
        let apart = comes_apart(from, to, scale.units_per_pixel);
        for ((start, end), colour) in sides(from, to)
            .into_iter()
            .zip(colours(theme))
            .take(if apart { 3 } else { 1 })
        {
            push_line(
                out,
                sketch.plane.to_world(start),
                sketch.plane.to_world(end),
                tint(colour),
                width,
                true,
                scale,
            );
        }
        return;
    }
    let Some(placed) = sketch.place(target, metrics(scale.units_per_pixel, DVec2::ZERO)) else {
        return;
    };
    for (start, end) in placed.shape {
        push_line(
            out,
            sketch.plane.to_world(start),
            sketch.plane.to_world(end),
            tint(theme.measure),
            width,
            true,
            scale,
        );
    }
}

/// The numbers, each on the side it measures, each on a pill of its own so the
/// drawing under it cannot swallow it.
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
    let view_projection = state
        .camera
        .view_projection(rect.width() / rect.height().max(1.0));
    let onto_the_screen =
        |place: DVec2| to_screen(sketch.plane.to_world(place), view_projection, rect);

    let said = crate::wording::measure::says(
        context.lang,
        target,
        reading.scaled(context.document.scale()),
        state.config.unit,
    );
    let theme = &state.theme;

    // The very pixel size the shape was drawn with. Read any other way, a run
    // comes apart here and not there, or a label works out its place for an
    // annotation standing further off the geometry than the one on screen and
    // floats clear of the run it belongs to — by the ratio of the two, which
    // on a retina screen is more than double.
    let pixel = state
        .camera
        .world_units_per_pixel(rect.height() * ui.ctx().pixels_per_point()) as f64;

    match (said, sketch.run_of(target)) {
        (Said::Triangle { span, across, up }, Some((from, to))) => {
            let apart = comes_apart(from, to, pixel);
            for (((start, end), colour), number) in sides(from, to)
                .into_iter()
                .zip(colours(theme))
                .zip([span, across, up])
                .take(if apart { 3 } else { 1 })
            {
                let (Some(start), Some(end)) = (onto_the_screen(start), onto_the_screen(end))
                else {
                    continue;
                };
                write(ui, rect, beside(start, end), &number, colour, theme);
            }
        }
        (Said::Beside(lines), _) => {
            let Some(placed) = sketch.place(target, metrics(pixel, DVec2::ZERO)) else {
                return;
            };
            let Some(at) = onto_the_screen(placed.text_at) else {
                return;
            };
            write(ui, rect, at, &lines.join("\n"), theme.measure, theme);
        }
        _ => {}
    }
}

/// Where a side's number goes: level with its middle, pushed square off the
/// line so the digits are not crossed by it.
fn beside(start: egui::Pos2, end: egui::Pos2) -> egui::Pos2 {
    let middle = start + (end - start) / 2.0;
    let along = (end - start).normalized();
    middle + egui::vec2(-along.y, along.x) * CLEAR_OF_THE_SIDE
}

/// One number, on a pill of its own.
///
/// The pill is what puts a number *in front* rather than merely last: painted
/// over an ellipse or a filled area, bare text keeps the lines running through
/// its digits whatever order it was drawn in.
fn write(
    ui: &egui::Ui,
    rect: egui::Rect,
    at: egui::Pos2,
    number: &str,
    colour: Rgba,
    theme: &Theme,
) {
    let painter = ui.painter_at(rect);
    let text = painter.layout_no_wrap(
        number.to_owned(),
        egui::FontId::proportional(TEXT_POINTS),
        tint_to_color(colour),
    );
    let pill = egui::Rect::from_center_size(at, text.size() + egui::vec2(8.0, 4.0));
    painter.rect_filled(pill, 4.0, backdrop(theme));
    painter.galley(
        pill.center() - text.size() / 2.0,
        text,
        egui::Color32::WHITE,
    );
}

/// The pill's own colour: the canvas's background, held back from opaque so
/// the drawing stays readable through it.
fn backdrop(theme: &Theme) -> egui::Color32 {
    tint_to_color(theme.background.sample(0.0).with_alpha(0.82))
}

#[cfg(test)]
mod tests;
