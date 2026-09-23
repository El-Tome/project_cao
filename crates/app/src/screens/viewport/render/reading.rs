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
use cao_sketch::{Measured, Sketch};
use glam::DVec2;

use super::curves::push_line;
use super::overlays::{tint_to_color, to_screen};
use super::{tint, tint_at};
use crate::screens::annotations::metrics;
use crate::screens::viewport::input::measure_showing;
use crate::screens::viewport::{SketchContext, ViewScale, ViewportState};
use crate::wording::measure::Said;

/// How big the numbers are drawn, matching the dimensions' own labels.
const TEXT_POINTS: f32 = 13.0;

/// How far off its side a number sits, in points, so the line does not run
/// through the digits.
const CLEAR_OF_THE_SIDE: f32 = 10.0;

/// The gap left between two numbers that would otherwise land on each other.
const BETWEEN_TWO_LINES: f32 = 3.0;

/// How much two pills may graze each other before one steps aside.
///
/// Without it, two corners touching by a tenth of a point send a number a
/// whole line down — a jump forty times the overlap that caused it, and the
/// number ends up further from the side it measures than the graze ever was.
const A_GRAZE: f32 = 2.5;

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
    surfaces: &mut Vec<cao_render::Vertex>,
    sketch: &Sketch,
    context: &SketchContext<'_>,
    theme: &Theme,
    scale: ViewScale,
) {
    let target = match measure_showing(context) {
        Some(Measured::Of(target)) => target,
        // An area is shown by tinting it, the way the extrusion tints the ones
        // it is offered: two nested shapes make it genuinely ambiguous which
        // one was read, and a number with no shape under it answers for
        // nothing.
        Some(Measured::Inside(place)) => return tint_the_area(surfaces, sketch, place, theme),
        None => return,
    };
    let width = theme.sketch_width;
    if let Some((from, to)) = sketch.run_of(target) {
        for ((start, end), colour) in sides(from, to).into_iter().zip(colours(theme)) {
            // A reach of nothing at all is a trait square to its axis. There is
            // no side to draw and no number to write: its length already is its
            // reach that way.
            if start.distance(end) <= f64::EPSILON {
                continue;
            }
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

/// Tints the closed area a place falls in, so the number beside it has a shape
/// to belong to.
fn tint_the_area(
    surfaces: &mut Vec<cao_render::Vertex>,
    sketch: &Sketch,
    place: DVec2,
    theme: &Theme,
) {
    let regions = sketch.regions();
    let Some(area) = cao_sketch::area_under(&regions, place).map(|rank| &regions[rank]) else {
        return;
    };
    let colour = tint_at(theme.measure, 0.18);
    // Holes stay empty: what is lit is exactly the surface the number reports.
    for corner in area.face_triangles().into_iter().flatten() {
        surfaces.push(cao_render::Vertex::solid(
            sketch.plane.to_world(corner).as_vec3(),
            colour,
        ));
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
    let Some(measured) = measure_showing(context) else {
        return;
    };
    let Some(index) = context.editor.active_sketch() else {
        return;
    };
    let Some(sketch) = context.document.sketches().get(index) else {
        return;
    };
    let reading = match measured {
        Measured::Of(target) => sketch.read(target),
        Measured::Inside(place) => sketch.read_inside(place),
    };
    let Some(reading) = reading else {
        return;
    };
    let view_projection = state
        .camera
        .view_projection(rect.width() / rect.height().max(1.0));
    let onto_the_screen =
        |place: DVec2| to_screen(sketch.plane.to_world(place), view_projection, rect);

    let said = crate::wording::measure::says(
        context.lang,
        measured,
        reading.scaled(context.document.scale()),
        state.config.unit,
        state.config.measure_figures,
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

    // Where the numbers already are. A near-axis run puts its length and its
    // long reach at almost the same spot, and two numbers written over each
    // other are worse than either alone — so the second steps down onto a line
    // of its own instead of being dropped. Both are true and both are wanted.
    let mut taken: Vec<egui::Rect> = Vec::new();

    let run = match measured {
        Measured::Of(target) => sketch.run_of(target),
        Measured::Inside(_) => None,
    };

    match (said, run) {
        (Said::Triangle { span, across, up }, Some((from, to))) => {
            for (((start, end), colour), number) in sides(from, to)
                .into_iter()
                .zip(colours(theme))
                .zip([span, across, up])
            {
                // A reach of nothing at all: the run is square to its axis, its
                // length already says that reach, and "0 mm" says nothing.
                if start.distance(end) <= f64::EPSILON {
                    continue;
                }
                let (Some(start), Some(end)) = (onto_the_screen(start), onto_the_screen(end))
                else {
                    continue;
                };
                write(
                    ui,
                    rect,
                    beside(start, end),
                    &number,
                    colour,
                    theme,
                    &mut taken,
                );
            }
        }
        (Said::Beside(lines), _) => {
            // An area is written where it was clicked; everything else where
            // the annotation for it would have put its own value.
            let belongs = match measured {
                Measured::Inside(place) => Some(place),
                Measured::Of(target) => sketch
                    .place(target, metrics(pixel, DVec2::ZERO))
                    .map(|placed| placed.text_at),
            };
            let Some(at) = belongs.and_then(onto_the_screen) else {
                return;
            };
            write(
                ui,
                rect,
                at,
                &lines.join("\n"),
                theme.measure,
                theme,
                &mut taken,
            );
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

/// The nearest place under `wanted` where a pill of this size touches none of
/// the pills already written.
///
/// Straight down, one line at a time, which is what turns two numbers landing
/// on each other into two numbers on two lines.
fn onto_a_free_line(wanted: egui::Pos2, size: egui::Vec2, taken: &[egui::Rect]) -> egui::Pos2 {
    let mut at = wanted;
    let step = size.y + BETWEEN_TWO_LINES;
    // One try per pill already written, and one more: each step clears at most
    // one of them, so there is always a line free by the end.
    for _ in 0..=taken.len() {
        let pill = egui::Rect::from_center_size(at, size).shrink(A_GRAZE);
        if !taken
            .iter()
            .any(|written| written.shrink(A_GRAZE).intersects(pill))
        {
            return at;
        }
        at.y += step;
    }
    at
}

/// One number, on a pill of its own, on a line no other number is using.
///
/// The pill is what puts a number *in front* rather than merely last: painted
/// over an ellipse or a filled area, bare text keeps the lines running through
/// its digits whatever order it was drawn in.
fn write(
    ui: &egui::Ui,
    rect: egui::Rect,
    wanted: egui::Pos2,
    number: &str,
    colour: Rgba,
    theme: &Theme,
    taken: &mut Vec<egui::Rect>,
) {
    let painter = ui.painter_at(rect);
    let text = painter.layout_no_wrap(
        number.to_owned(),
        egui::FontId::proportional(TEXT_POINTS),
        tint_to_color(colour),
    );
    let size = text.size() + egui::vec2(8.0, 4.0);
    let pill = egui::Rect::from_center_size(onto_a_free_line(wanted, size, taken), size);
    taken.push(pill);
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
