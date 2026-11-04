//! A curve side of a cut corner as the canvas draws it: solid where it shows,
//! and in thin dashes over each stretch a cut that fits hides up to its sharp
//! corner — lit with the cut while it is, as a trait side is drawn. What shows
//! is the sketch's to say.

use cao_prefs::theme::Theme;
use cao_sketch::{ArcDraft, ArcId, CutSide, EllipseId, PointId, Stretches};
use glam::DVec2;

use super::super::curves::{places_of, push_arc_at, push_ellipse_at};
use super::hidden::faint;
use super::{CutCanvas, Stroke};
use crate::screens::viewport::ViewScale;

impl CutCanvas<'_> {
    /// An arc of the drawing.
    pub(in crate::screens::viewport::render) fn push_arc(
        &self,
        out: &mut Vec<cao_render::Vertex>,
        arc: ArcId,
        stroke: Stroke,
        (theme, scale): (&Theme, ViewScale),
    ) {
        let drawn = self.sketch.arc_draft(arc);
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
        let stretches = self.sketch.arc_stretches(arc, &self.pulled);
        if stretches.hidden == [None, None] {
            push(out, drawn, stroke);
            return;
        }
        let ends = self.sketch.arc(arc);
        let hidden = self.hidden_strokes(arc.into(), [ends.start, ends.end], stroke, theme);
        self.push_stretches(out, stretches, hidden, stroke, |out, run, stroke| {
            push(out, piece(run), stroke)
        });
    }

    /// An ellipse of the drawing, or the stretch a cut left of one.
    pub(in crate::screens::viewport::render) fn push_ellipse(
        &self,
        out: &mut Vec<cao_render::Vertex>,
        ellipse: EllipseId,
        stroke: Stroke,
        (theme, scale): (&Theme, ViewScale),
    ) {
        let push = |out: &mut Vec<cao_render::Vertex>, places: Vec<DVec2>, stroke: Stroke| {
            let Stroke {
                color,
                width,
                construction,
            } = stroke;
            push_ellipse_at(out, self.sketch, places, color, width, construction, scale);
        };
        let stretches = self.sketch.ellipse_stretches(ellipse, &self.pulled);
        let (start, end) = match (stretches.hidden, self.sketch.ellipse_ends(ellipse)) {
            ([None, None], _) | (_, None) => {
                push(out, self.sketch.ellipse_polyline(ellipse), stroke);
                return;
            }
            (_, Some(ends)) => ends,
        };
        let drawn = self.sketch.ellipse_draft(ellipse);
        let hidden = self.hidden_strokes(ellipse.into(), [start, end], stroke, theme);
        self.push_stretches(
            out,
            stretches,
            hidden,
            stroke,
            |out, (from, sweep), stroke| push(out, places_of(drawn, from, sweep), stroke),
        );
    }

    /// What shows of a curve in its own stroke, then each stretch a cut hides
    /// in the stroke of the corner it hides it up to.
    fn push_stretches(
        &self,
        out: &mut Vec<cao_render::Vertex>,
        stretches: Stretches,
        hidden: [Stroke; 2],
        stroke: Stroke,
        push: impl Fn(&mut Vec<cao_render::Vertex>, (f64, f64), Stroke),
    ) {
        if let Some(shows) = stretches.shows {
            push(out, shows, stroke);
        }
        for (run, stroke) in stretches.hidden.into_iter().zip(hidden) {
            if let Some(run) = run {
                push(out, run, faint(stroke));
            }
        }
    }

    /// The strokes of the stretches a side hides up to its start and its end.
    fn hidden_strokes(
        &self,
        side: CutSide,
        ends: [PointId; 2],
        stroke: Stroke,
        theme: &Theme,
    ) -> [Stroke; 2] {
        ends.map(|corner| self.hidden_at(corner, side, stroke, theme))
    }
}
