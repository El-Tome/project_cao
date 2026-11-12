//! What the view shows while the circular pattern's panel is open, as P13 of
//! `docs/redesign/part-screen.md` draws it: the circle the copies stand on,
//! the radius out to what is copied, the centre marked « Centre », and the
//! step as an arc turning the way the pattern will, its pitch written beside
//! it. The copies, dashed, and what is copied, lit, are drawn with the rest of
//! the drawing, in `drawing` and `emphasis`.

use cao_prefs::theme::Theme;
use cao_sketch::{Element, Sketch};
use glam::DVec2;

use super::curves::{push_circle_at, push_line};
use super::overlays::to_screen;
use super::token;
use crate::screens::SketchContext;
use crate::screens::sketch::{CopyTarget, Tool};
use crate::screens::viewport::{ViewScale, ViewportState};
use crate::ui::theme::tokens::{
    ACCENT_AMBER, ACCENT_AMBER_BACKGROUND, ACCENT_AMBER_BORDER, ACCENT_AMBER_TEXT,
    BACKGROUND_FLOATING, BORDER_CONTROL, RADIUS_BUTTON, TEXT_TITLE, TYPOGRAPHY_SIZES_DETAIL,
    TYPOGRAPHY_SIZES_MONO_VALUE,
};

/// How far from the centre the step's arc runs, as a share of how far the
/// copies stand: inside their circle, clear of the copies themselves.
const ARC_REACH: f64 = 0.5;

/// How many straight pieces the step's arc is drawn in.
const ARC_PIECES: usize = 32;

/// How long each wing of the arrow is on screen, and how far it opens from
/// the arc, in radians.
const WING_PIXELS: f64 = 9.0;
const WING_OPENS: f64 = 0.45;

/// How far the centre's mark reaches on screen.
const MARK_PIXELS: f64 = 6.0;

/// How the guides stand back from the drawing: thinner than its lines.
const GUIDE_WIDTH: f32 = 0.7;

/// Where the « Centre » label stands from the centre, on screen, and how far
/// the pitch stands out from its arc.
const CENTRE_LABEL_FROM: egui::Vec2 = egui::vec2(12.0, 10.0);
const PITCH_CLEARS_ITS_ARC: f32 = 6.0;

/// A label's room around its text.
const LABEL_PADDING: egui::Vec2 = egui::vec2(8.0, 3.0);

/// What the guides are drawn from, on the sketch.
pub(super) struct Guides {
    pub(super) centre: DVec2,
    /// Where what is copied stands: the middle of what the panel picked.
    pub(super) source: DVec2,
    /// The step the copies turn by, counter-clockwise when positive.
    pub(super) step_degrees: f64,
    /// The step as the panel shows it.
    pub(super) typed_degrees: f64,
}

/// The amber of the circular pattern: what it copies, its copies, its guides.
pub(super) fn amber() -> [f32; 4] {
    token(ACCENT_AMBER)
}

/// Whether the drawing shows the circular pattern's copies dashed: while its
/// panel is open, and for it alone.
pub(super) fn dashes_its_copies(context: &SketchContext<'_>) -> bool {
    context.editor.tool == Tool::CircularPattern && context.editor.copy_panel().is_some()
}

/// The guides of the circular pattern in hand, once its centre is chosen and
/// its step typed, and while what it copies stands apart from that centre.
fn guides(sketch: &Sketch, context: &SketchContext<'_>) -> Option<Guides> {
    if !dashes_its_copies(context) {
        return None;
    }
    let panel = context.editor.copy_panel()?;
    let Some(CopyTarget::Centre(centre)) = panel.target() else {
        return None;
    };
    let centre = *sketch.points().get(centre.0)?;
    let standing: Vec<DVec2> = panel
        .picked()
        .filter_map(|element| stands_at(sketch, element))
        .collect();
    let source = middle(&standing).filter(|source| source.distance(centre) > f64::EPSILON)?;
    let typed_degrees = context.editor.live.typed(0)?;
    Some(Guides {
        centre,
        source,
        step_degrees: panel.rotation().step(typed_degrees),
        typed_degrees,
    })
}

/// Where one element stands: a point where it is, a trait at its middle, a
/// curve at its centre.
fn stands_at(sketch: &Sketch, element: Element) -> Option<DVec2> {
    let at = |point: cao_sketch::PointId| sketch.points().get(point.0).copied();
    match element {
        Element::Point(point) => at(point),
        Element::Segment(id) => {
            let side = sketch.segments().get(id.0)?;
            Some((at(side.start)? + at(side.end)?) / 2.0)
        }
        Element::Circle(id) => at(sketch.circles().get(id.0)?.center),
        Element::Arc(id) => at(sketch.arcs().get(id.0)?.center),
        Element::Ellipse(id) => at(sketch.ellipses().get(id.0)?.center),
    }
}

/// The middle of some places; none for none.
pub(super) fn middle(places: &[DVec2]) -> Option<DVec2> {
    (!places.is_empty()).then(|| places.iter().copied().sum::<DVec2>() / places.len() as f64)
}

/// The places the step's arc runs through: round `centre`, `reach` away from
/// it, from the side `source` stands on, over the signed step.
pub(super) fn step_arc(centre: DVec2, source: DVec2, step_degrees: f64, reach: f64) -> Vec<DVec2> {
    let from = (source - centre).to_angle();
    let sweep = step_degrees.to_radians();
    (0..=ARC_PIECES)
        .map(|piece| {
            let angle = from + sweep * piece as f64 / ARC_PIECES as f64;
            centre + DVec2::from_angle(angle) * reach
        })
        .collect()
}

/// The two wings of an arrow at the end of `run`, `length` long, pointing the
/// way the run goes there.
pub(super) fn arrow_wings(run: &[DVec2], length: f64) -> Option<[DVec2; 2]> {
    let [.., before, tip] = run else {
        return None;
    };
    let back = (*before - *tip).try_normalize()?;
    let wing = |opens: f64| *tip + DVec2::from_angle(opens).rotate(back) * length;
    Some([wing(WING_OPENS), wing(-WING_OPENS)])
}

/// The guides, in the scene the GPU draws.
pub(super) fn push_guides(
    out: &mut Vec<cao_render::Vertex>,
    sketch: &Sketch,
    context: &SketchContext<'_>,
    theme: &Theme,
    scale: ViewScale,
) {
    let Some(guides) = guides(sketch, context) else {
        return;
    };
    let (amber, width) = (amber(), theme.sketch_width * GUIDE_WIDTH);
    let world = |place: DVec2| sketch.plane.to_world(place);
    let mut line = |from: DVec2, to: DVec2| {
        push_line(out, world(from), world(to), amber, width, false, scale);
    };
    let reach = guides.centre.distance(guides.source);
    let arc = step_arc(
        guides.centre,
        guides.source,
        guides.step_degrees,
        reach * ARC_REACH,
    );
    line(guides.centre, guides.source);
    for pair in arc.windows(2) {
        line(pair[0], pair[1]);
    }
    if let (Some(wings), Some(tip)) = (
        arrow_wings(&arc, WING_PIXELS * scale.units_per_pixel),
        arc.last(),
    ) {
        for wing in wings {
            line(*tip, wing);
        }
    }
    let mark = MARK_PIXELS * scale.units_per_pixel;
    for across in [DVec2::X, DVec2::Y] {
        line(
            guides.centre - across * mark * 1.6,
            guides.centre + across * mark * 1.6,
        );
    }
    let circle = |out: &mut Vec<_>, radius, dashed| {
        push_circle_at(
            out,
            sketch,
            guides.centre,
            radius,
            amber,
            width,
            dashed,
            scale,
        );
    };
    circle(out, reach, true);
    circle(out, mark, false);
}

/// « Centre » beside the centre, and the pitch beside its arc.
pub(crate) fn paint_pattern_labels(
    ui: &egui::Ui,
    state: &ViewportState,
    rect: egui::Rect,
    context: &SketchContext<'_>,
) {
    let Some(index) = context.editor.active_sketch() else {
        return;
    };
    let Some(sketch) = context.document.sketches().get(index) else {
        return;
    };
    let Some(guides) = guides(sketch, context) else {
        return;
    };
    let view_projection = state
        .camera
        .view_projection(rect.width() / rect.height().max(1.0));
    let screen = |place: DVec2| to_screen(sketch.plane.to_world(place), view_projection, rect);
    let painter = ui.painter_at(rect);

    if let Some(at) = screen(guides.centre) {
        let said = context.lang.t("copy_panel.centre");
        let font = egui::FontId::proportional(TYPOGRAPHY_SIZES_DETAIL);
        let text = painter.layout_no_wrap(said, font, ACCENT_AMBER_TEXT);
        let pill =
            egui::Rect::from_min_size(at + CENTRE_LABEL_FROM, text.size() + 2.0 * LABEL_PADDING);
        painter.rect_filled(pill, RADIUS_BUTTON, BACKGROUND_FLOATING);
        let border = egui::Stroke::new(1.0, ACCENT_AMBER_BORDER);
        painter.rect(
            pill,
            RADIUS_BUTTON,
            ACCENT_AMBER_BACKGROUND,
            border,
            egui::StrokeKind::Inside,
        );
        painter.galley(pill.min + LABEL_PADDING, text, ACCENT_AMBER_TEXT);
    }

    let reach = guides.centre.distance(guides.source) * ARC_REACH;
    let arc = step_arc(guides.centre, guides.source, guides.step_degrees, reach);
    let Some(halfway) = arc.get(arc.len() / 2).copied() else {
        return;
    };
    if let (Some(at), Some(centre)) = (screen(halfway), screen(guides.centre)) {
        let said = crate::wording::dimension::degrees(guides.typed_degrees);
        let font = egui::FontId::monospace(TYPOGRAPHY_SIZES_MONO_VALUE);
        let text = painter.layout_no_wrap(said, font, TEXT_TITLE);
        let size = text.size() + 2.0 * LABEL_PADDING;
        let outward = (at - centre).normalized();
        let stands = at + outward * (size.length() / 2.0 + PITCH_CLEARS_ITS_ARC);
        let pill = egui::Rect::from_center_size(stands, size);
        let border = egui::Stroke::new(1.0, BORDER_CONTROL);
        painter.rect(
            pill,
            RADIUS_BUTTON,
            BACKGROUND_FLOATING,
            border,
            egui::StrokeKind::Inside,
        );
        painter.galley(pill.min + LABEL_PADDING, text, TEXT_TITLE);
    }
}

#[cfg(test)]
mod tests;
