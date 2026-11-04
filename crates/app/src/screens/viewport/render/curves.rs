//! The curves a sketch is drawn with, turned into the straight steps the line
//! renderer takes.
//!
//! `cao_render` knows one shape, a segment between two places. A circle, an
//! arc and a broken line all come down to a run of them, and the dashes of
//! construction geometry are cut along that run, a dash long on screen.

use cao_sketch::{ArcDraft, EllipseDraft, Sketch, places_along};
use glam::{DVec2, DVec3};

use crate::screens::viewport::ViewScale;

/// How long each dash and the gap after it are on screen, so the pattern
/// stays readable at any zoom instead of vanishing or clumping together.
const DASH_PIXELS: f64 = 6.0;

/// A straight line, plain or, for construction geometry, broken into dashes
/// of a constant size on screen.
pub(super) fn push_line(
    out: &mut Vec<cao_render::Vertex>,
    start: DVec3,
    end: DVec3,
    color: [f32; 4],
    width: f32,
    construction: bool,
    scale: ViewScale,
) {
    let span = end - start;
    let length = span.length();
    let dash = DASH_PIXELS * scale.units_per_pixel;
    if !construction || length < dash {
        out.push(cao_render::Vertex::line(start.as_vec3(), color, width));
        out.push(cao_render::Vertex::line(end.as_vec3(), color, width));
        return;
    }
    let direction = span / length;
    let mut walked = 0.0;
    while walked < length {
        let dash_end = (walked + dash).min(length);
        out.push(cao_render::Vertex::line(
            (start + direction * walked).as_vec3(),
            color,
            width,
        ));
        out.push(cao_render::Vertex::line(
            (start + direction * dash_end).as_vec3(),
            color,
            width,
        ));
        walked += dash * 2.0;
    }
}

/// Circles are drawn as a many-sided polygon: the line renderer only knows
/// about segments, and at this many sides the corners are invisible.
#[allow(clippy::too_many_arguments)]
pub(super) fn push_circle_at(
    out: &mut Vec<cao_render::Vertex>,
    sketch: &Sketch,
    center: DVec2,
    radius: f64,
    color: [f32; 4],
    width: f32,
    construction: bool,
    scale: ViewScale,
) {
    const SIDES: usize = 96;
    if radius <= 0.0 {
        return;
    }
    let places = (0..=SIDES).map(|step| {
        let angle = step as f64 / SIDES as f64 * std::f64::consts::TAU;
        center + DVec2::from_angle(angle) * radius
    });
    push_curve(out, sketch, places, color, width, construction, scale);
}

/// The piece of a circle an arc draws, and nothing of the rest of that circle.
pub(super) fn push_arc_at(
    out: &mut Vec<cao_render::Vertex>,
    sketch: &Sketch,
    drawn: ArcDraft,
    color: [f32; 4],
    width: f32,
    construction: bool,
    scale: ViewScale,
) {
    push_curve(
        out,
        sketch,
        places_along(drawn).into_iter(),
        color,
        width,
        construction,
        scale,
    );
}

/// The curve an ellipse draws, whole or one stretch of it.
pub(super) fn push_ellipse_at(
    out: &mut Vec<cao_render::Vertex>,
    sketch: &Sketch,
    places: Vec<DVec2>,
    color: [f32; 4],
    width: f32,
    construction: bool,
    scale: ViewScale,
) {
    push_curve(
        out,
        sketch,
        places.into_iter(),
        color,
        width,
        construction,
        scale,
    );
}

/// The run of places one stretch of an ellipse is drawn as, from where it
/// opens over how far it goes.
pub(super) fn places_of(drawn: EllipseDraft, from: f64, sweep: f64) -> Vec<DVec2> {
    drawn.places_along(from, sweep, drawn.steps_over(sweep))
}

/// A run of places joined up, plain or, for construction geometry, broken
/// into dashes a dash long on screen with gaps as long — measured along the
/// run, as a line's are, so that a step longer on screen than a dash still
/// holds several, and a dash running past a place goes on into the next step.
///
/// A run of no length at all draws nothing.
fn push_curve(
    out: &mut Vec<cao_render::Vertex>,
    sketch: &Sketch,
    places: impl Iterator<Item = DVec2>,
    color: [f32; 4],
    width: f32,
    construction: bool,
    scale: ViewScale,
) {
    let worlds: Vec<DVec3> = places.map(|place| sketch.plane.to_world(place)).collect();
    let run: f64 = worlds
        .windows(2)
        .map(|pair| pair[0].distance(pair[1]))
        .sum();
    if run < 1e-12 * (worlds.len().max(2) - 1) as f64 {
        return;
    }
    let mut stroke = |from: DVec3, to: DVec3| {
        out.push(cao_render::Vertex::line(from.as_vec3(), color, width));
        out.push(cao_render::Vertex::line(to.as_vec3(), color, width));
    };
    let dash = DASH_PIXELS * scale.units_per_pixel;
    if !construction || dash <= 0.0 {
        for pair in worlds.windows(2) {
            stroke(pair[0], pair[1]);
        }
        return;
    }
    // How far into the dash, or into the gap after it, the walk stands —
    // carried from one step to the next.
    let (mut drawing, mut into) = (true, 0.0);
    for pair in worlds.windows(2) {
        let (from, length) = (pair[0], pair[0].distance(pair[1]));
        if length <= 0.0 {
            continue;
        }
        let along = (pair[1] - from) / length;
        let mut walked = 0.0;
        while walked < length {
            let left = (dash - into).max(0.0);
            let stop = (walked + left).min(length);
            if drawing {
                stroke(from + along * walked, from + along * stop);
            }
            // Past the end of the dash or the gap, the other begins: set
            // outright rather than added up, so that a remainder too small to
            // move the walk cannot hold it in place.
            match walked + left <= length {
                true => (drawing, into) = (!drawing, 0.0),
                false => into += stop - walked,
            }
            walked = stop;
        }
    }
}

#[cfg(test)]
mod tests;
