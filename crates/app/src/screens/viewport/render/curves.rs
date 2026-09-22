//! The curves a sketch is drawn with, turned into the straight steps the line
//! renderer takes.
//!
//! `cao_render` knows one shape, a segment between two places. A circle, an
//! arc and a broken line all come down to a run of them, and the dashes of
//! construction geometry are runs left out.

use cao_sketch::{ArcDraft, EllipseDraft, Sketch, places_along, steps_along, sweep_of};
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
    let side = std::f64::consts::TAU * radius / SIDES as f64;
    push_curve(out, sketch, places, side, color, width, construction, scale);
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
    let radius = drawn.centre.distance(drawn.start);
    let step = sweep_of(drawn) * radius / steps_along(drawn) as f64;
    push_curve(
        out,
        sketch,
        places_along(drawn).into_iter(),
        step,
        color,
        width,
        construction,
        scale,
    );
}

/// The whole curve an ellipse draws.
pub(super) fn push_ellipse_at(
    out: &mut Vec<cao_render::Vertex>,
    sketch: &Sketch,
    places: Vec<DVec2>,
    color: [f32; 4],
    width: f32,
    construction: bool,
    scale: ViewScale,
) {
    // The steps of an ellipse are not all of one length, as a circle's are;
    // their mean keeps the dashes about the size a circle's would be.
    let run: f64 = places
        .windows(2)
        .map(|pair| pair[0].distance(pair[1]))
        .sum();
    let step = run / (places.len().max(2) - 1) as f64;
    push_curve(
        out,
        sketch,
        places.into_iter(),
        step,
        color,
        width,
        construction,
        scale,
    );
}

/// One stretch of an ellipse, from where it opens over how far it goes.
#[allow(clippy::too_many_arguments)]
pub(super) fn push_ellipse_run(
    out: &mut Vec<cao_render::Vertex>,
    sketch: &Sketch,
    drawn: EllipseDraft,
    from: f64,
    sweep: f64,
    color: [f32; 4],
    width: f32,
    construction: bool,
    scale: ViewScale,
) {
    let places = drawn.places_along(from, sweep, drawn.steps_over(sweep));
    let run: f64 = places
        .windows(2)
        .map(|pair| pair[0].distance(pair[1]))
        .sum();
    let step = run / (places.len().max(2) - 1) as f64;
    push_curve(
        out,
        sketch,
        places.into_iter(),
        step,
        color,
        width,
        construction,
        scale,
    );
}

/// A run of places joined up, plain or, for construction geometry, with every
/// other run of steps left out — which is what turns the same polyline into a
/// dashed curve.
///
/// `step` is how long one of those steps is, the steps of a circle and of an
/// arc being all of one length. How many of them make a dash is worked out
/// from it once, so that a dash keeps a constant size on screen — the same way
/// a construction line's dashes do — rather than growing with the curve's own
/// radius.
#[allow(clippy::too_many_arguments)]
fn push_curve(
    out: &mut Vec<cao_render::Vertex>,
    sketch: &Sketch,
    places: impl Iterator<Item = DVec2>,
    step: f64,
    color: [f32; 4],
    width: f32,
    construction: bool,
    scale: ViewScale,
) {
    if step < 1e-12 {
        return;
    }
    let dash_group = ((DASH_PIXELS * scale.units_per_pixel / step).round() as usize).max(1);
    let mut previous = None;
    for (rank, place) in places.enumerate() {
        let world = sketch.plane.to_world(place).as_vec3();
        let drawn = !construction || (rank / dash_group).is_multiple_of(2);
        if let (Some(previous), true) = (previous, drawn) {
            out.push(cao_render::Vertex::line(previous, color, width));
            out.push(cao_render::Vertex::line(world, color, width));
        }
        previous = Some(world);
    }
}
