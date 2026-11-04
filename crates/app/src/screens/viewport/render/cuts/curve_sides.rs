//! A curve side of a cut corner as the canvas draws it: solid where it shows,
//! and in thin dashes over each stretch a cut that fits hides up to its sharp
//! corner — as a trait side is drawn. What shows is the sketch's to say.

use cao_sketch::{ArcDraft, ArcId, EllipseId};
use glam::DVec2;

use super::super::curves::{places_of, push_arc_at, push_ellipse_at};
use super::{CutCanvas, HIDDEN, Stroke};
use crate::screens::viewport::ViewScale;

impl CutCanvas<'_> {
    /// An arc of the drawing.
    pub(in crate::screens::viewport::render) fn push_arc(
        &self,
        out: &mut Vec<cao_render::Vertex>,
        arc: ArcId,
        stroke: Stroke,
        scale: ViewScale,
    ) {
        let drawn = self.sketch.arc_draft(arc);
        let stretches = self.sketch.arc_stretches(arc, &self.pulled);
        let radius = drawn.centre.distance(drawn.start);
        let piece = |(from, sweep): (f64, f64)| ArcDraft {
            centre: drawn.centre,
            start: drawn.centre + DVec2::from_angle(from) * radius,
            end: drawn.centre + DVec2::from_angle(from + sweep) * radius,
        };
        let push = |out: &mut Vec<cao_render::Vertex>, drawn: ArcDraft, stroke: Stroke| {
            let Stroke {
                color,
                width,
                construction,
            } = stroke;
            push_arc_at(out, self.sketch, drawn, color, width, construction, scale);
        };
        if stretches.hidden.is_empty() {
            push(out, drawn, stroke);
            return;
        }
        if let Some(shows) = stretches.shows {
            push(out, piece(shows), stroke);
        }
        for hidden in stretches.hidden {
            push(out, piece(hidden), faint(stroke));
        }
    }

    /// An ellipse of the drawing, or the stretch a cut left of one.
    pub(in crate::screens::viewport::render) fn push_ellipse(
        &self,
        out: &mut Vec<cao_render::Vertex>,
        ellipse: EllipseId,
        stroke: Stroke,
        scale: ViewScale,
    ) {
        let stretches = self.sketch.ellipse_stretches(ellipse, &self.pulled);
        let drawn = self.sketch.ellipse_draft(ellipse);
        let push = |out: &mut Vec<cao_render::Vertex>, places: Vec<DVec2>, stroke: Stroke| {
            let Stroke {
                color,
                width,
                construction,
            } = stroke;
            push_ellipse_at(out, self.sketch, places, color, width, construction, scale);
        };
        if stretches.hidden.is_empty() {
            push(out, self.sketch.ellipse_polyline(ellipse), stroke);
            return;
        }
        if let Some((from, sweep)) = stretches.shows {
            push(out, places_of(drawn, from, sweep), stroke);
        }
        for (from, sweep) in stretches.hidden {
            push(out, places_of(drawn, from, sweep), faint(stroke));
        }
    }
}

/// How the stretch a cut hides up to its sharp corner is drawn: thin, faint
/// and dashed, the corner still there to be read.
pub(super) fn faint(stroke: Stroke) -> Stroke {
    let [red, green, blue, alpha] = stroke.color;
    Stroke {
        color: [red, green, blue, alpha * HIDDEN],
        width: stroke.width * 0.6,
        construction: true,
    }
}
