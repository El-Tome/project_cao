//! The fillets and chamfers of a drawing as they are drawn: what is left of
//! each side, the stretch of it hidden up to the sharp corner, and the curve
//! standing where the corner was.

use cao_prefs::theme::Theme;
use cao_sketch::{
    Chamfer, Constraint, CornerCutId, CutCurve, CutPoint, CutShape, CutSide, DrawnCut, PointId,
    PulledEnds, SegmentId, Selection, Sketch,
};
use glam::{DVec2, DVec3};

use super::super::ViewportState;
use super::curves::{push_arc_at, push_line};
use super::drawing::sketch_colors;
use super::marks::push_point_marker;
use super::overlays::{tint_to_color, to_screen};
use super::{emphasis, tint, tint_at};
use crate::screens::SketchContext;
use crate::screens::viewport::ViewScale;

mod curve_sides;

/// How faint the stretch of a side hidden by a cut is drawn, as a share of
/// the side's own colour: the sharp corner is still there to be read.
const HIDDEN: f32 = 0.45;

/// How strongly a cut laid with the one held or pointed at is lit: enough to
/// read the group, faint enough to tell which one a key acts on.
const LINKED: f32 = 0.35;

/// How far from its curve the value of a cut stands, in pixels.
const LABEL_PIXELS: f64 = 14.0;

/// The cuts of one drawing, read once for the frame that draws them.
pub(super) struct CutCanvas<'a> {
    sketch: &'a Sketch,
    drawn: Vec<DrawnCut>,
    pulled: PulledEnds,
    /// The cuts under the cursor or held, and the cuts laid with them.
    lit: Lit,
    /// The sides of each at its corner, whose hidden stretch lights with it.
    lit_ends: Vec<(PointId, CutSide)>,
    linked_ends: Vec<(PointId, CutSide)>,
}

/// How a line is drawn.
#[derive(Clone, Copy)]
pub(super) struct Stroke {
    pub(super) color: [f32; 4],
    pub(super) width: f32,
    pub(super) construction: bool,
}

impl<'a> CutCanvas<'a> {
    pub(super) fn read(sketch: &'a Sketch, context: &SketchContext<'_>) -> Self {
        let drawn = sketch.drawn_cuts();
        let pulled = sketch.pulled_ends(&drawn);
        let lit = lit_cuts(sketch, context);
        let lit_ends = ends_of(sketch, &lit.held);
        let linked_ends = ends_of(sketch, &lit.linked);
        Self {
            sketch,
            drawn,
            pulled,
            lit,
            lit_ends,
            linked_ends,
        }
    }

    /// One side of the drawing, from `start` to `end` as this frame shows
    /// them: solid where it is still drawn, and the stretch a cut hides up to
    /// its sharp corner thin and dashed — lit with the cut while it is.
    pub(super) fn push_side(
        &self,
        out: &mut Vec<cao_render::Vertex>,
        side: SegmentId,
        (start, end): (DVec2, DVec2),
        stroke: Stroke,
        (theme, scale): (&Theme, ViewScale),
    ) {
        let world = |place: DVec2| self.sketch.plane.to_world(place);
        let corners = self.sketch.segments()[side.0];
        let hidden = |corner: PointId| self.hidden_at(corner, side.into(), stroke, theme);
        let Some(ends) = self.sketch.visible_ends(side, &self.pulled) else {
            push_hidden(out, world(start), world(end), hidden(corners.start), scale);
            return;
        };
        let from = ends[0].unwrap_or(start);
        let to = ends[1].unwrap_or(end);
        let Stroke {
            color,
            width,
            construction,
        } = stroke;
        push_line(
            out,
            world(from),
            world(to),
            color,
            width,
            construction,
            scale,
        );
        if ends[0].is_some() {
            push_hidden(out, world(start), world(from), hidden(corners.start), scale);
        }
        if ends[1].is_some() {
            push_hidden(out, world(to), world(end), hidden(corners.end), scale);
        }
    }

    /// The curve of every cut: in the drawing's colours while it fits, in the
    /// colour of a refusal where it would be while it does not. What a tool
    /// only offers to cut is drawn as a promise.
    pub(super) fn push_cuts(
        &self,
        out: &mut Vec<cao_render::Vertex>,
        offered: &[CornerCutId],
        (active, settled): (bool, &[bool]),
        (theme, scale): (&Theme, ViewScale),
        context: &SketchContext<'_>,
    ) {
        for cut in &self.drawn {
            let held = settled.get(cut.corner_point.0).copied().unwrap_or(false);
            let (color, width) = sketch_colors(theme, active, held);
            let base = match cut.fits {
                true => color,
                false => tint(theme.refused),
            };
            let (color, width) = if self.lit.held.contains(&cut.id) {
                (tint_at(theme.highlight, 1.0), width * 2.0)
            } else if self.lit.linked.contains(&cut.id) {
                (tint_at(theme.highlight, LINKED), width * 1.4)
            } else {
                emphasis::mark(context, theme, Selection::Cut(cut.id), &[], base, width)
            };
            let color = match offered.contains(&cut.id) {
                true => emphasis::ghost(theme),
                false => color,
            };
            match cut.curve {
                CutCurve::Arc(arc) => {
                    push_arc_at(out, self.sketch, arc, color, width, cut.construction, scale)
                }
                CutCurve::Straight { from, to } => push_line(
                    out,
                    self.sketch.plane.to_world(from),
                    self.sketch.plane.to_world(to),
                    color,
                    width,
                    cut.construction,
                    scale,
                ),
            }
        }
    }
}

impl CutCanvas<'_> {
    /// The points every cut offers — a fillet's centre, and where a cut meets
    /// each side — drawn as the drawing's own points are, in the colour of
    /// their cut. One something already hangs on is a point of the drawing,
    /// and is drawn with them.
    pub(super) fn push_points(
        &self,
        out: &mut Vec<cao_render::Vertex>,
        settled: &[bool],
        (theme, scale): (&Theme, ViewScale),
        context: &SketchContext<'_>,
    ) {
        let half = scale.world_size_of(4.0);
        let hung: Vec<(CornerCutId, CutPoint)> = self
            .sketch
            .constraints()
            .iter()
            .filter_map(|rule| match *rule {
                Constraint::AtCutPoint { cut, at, .. } => Some((cut, at)),
                _ => None,
            })
            .collect();
        let reach = scale.world_size_of(crate::screens::viewport::PICK_PIXELS);
        let hovered = context
            .editor
            .cursor
            .filter(|_| context.editor.hovered_point.is_none() && context.editor.band().is_none())
            .and_then(|cursor| {
                self.drawn
                    .iter()
                    .flat_map(|drawn| drawn.points().map(|(at, place)| (drawn.id, at, place)))
                    .map(|(cut, at, place)| (cut, at, place.distance(cursor)))
                    .filter(|(_, _, distance)| *distance <= reach)
                    .min_by(|(_, _, one), (_, _, other)| one.total_cmp(other))
                    .map(|(cut, at, _)| (cut, at))
            });
        for drawn in &self.drawn {
            let held = settled.get(drawn.corner_point.0).copied().unwrap_or(false);
            let (color, _) = sketch_colors(theme, true, held);
            let color = match drawn.fits {
                true => color,
                false => tint(theme.refused),
            };
            for (at, place) in drawn.points() {
                if hung.contains(&(drawn.id, at)) {
                    continue;
                }
                let picked = context
                    .editor
                    .rule_picks()
                    .contains(&cao_sketch::RulePick::CutPoint(drawn.id, at));
                let lit =
                    hovered == Some((drawn.id, at)) || picked || self.lit.held.contains(&drawn.id);
                let (color, width) = match lit {
                    true => (tint_at(theme.highlight, 1.0), 2.5),
                    false => (color, 1.5),
                };
                push_point_marker(out, self.sketch, place, half, color, width);
            }
        }
    }
}

fn push_hidden(
    out: &mut Vec<cao_render::Vertex>,
    from: DVec3,
    to: DVec3,
    stroke: Stroke,
    scale: ViewScale,
) {
    let faint = curve_sides::faint(stroke);
    push_line(out, from, to, faint.color, faint.width, true, scale);
}

/// The cuts under the cursor or held by the selection, apart from the cuts
/// laid with them: a key such as `Suppr` acts on the first alone, and the
/// second only shows the rounding they make together.
#[derive(Debug, Default, PartialEq)]
pub(super) struct Lit {
    pub(super) held: Vec<CornerCutId>,
    pub(super) linked: Vec<CornerCutId>,
}

pub(super) fn lit_cuts(sketch: &Sketch, context: &SketchContext<'_>) -> Lit {
    let pointed_at = |cut: CornerCutId| {
        let what = Selection::Cut(cut);
        context.editor.is_selected(what) || context.editor.hovered == Some(what)
    };
    let held: Vec<CornerCutId> = sketch
        .live_corner_cuts()
        .map(|(id, _)| id)
        .filter(|id| pointed_at(*id))
        .collect();
    let mut linked: Vec<CornerCutId> = Vec::new();
    for cut in held.iter().flat_map(|id| sketch.cuts_grouped_with(*id)) {
        if !held.contains(&cut) && !linked.contains(&cut) {
            linked.push(cut);
        }
    }
    Lit { held, linked }
}

fn corners_of(sketch: &Sketch, cuts: &[CornerCutId]) -> Vec<PointId> {
    cuts.iter()
        .filter_map(|cut| sketch.corner_cuts().get(cut.0).map(|cut| cut.corner))
        .collect()
}

/// Each side of these cuts at its corner: a point carrying several cuts
/// lights only the sides of the one lit.
fn ends_of(sketch: &Sketch, cuts: &[CornerCutId]) -> Vec<(PointId, CutSide)> {
    cuts.iter()
        .filter_map(|cut| sketch.corner_cuts().get(cut.0))
        .flat_map(|cut| cut.sides.map(|side| (cut.corner, side)))
        .collect()
}

/// The corners of the cuts under the cursor or held, which light with them.
pub(super) fn lit_corners(sketch: &Sketch, context: &SketchContext<'_>) -> Vec<PointId> {
    corners_of(sketch, &lit_cuts(sketch, context).held)
}

/// The cut whose value is written under the cursor. The text stands near the
/// sharp corner, whose point a press would otherwise take first — except on
/// the dot of a point itself, which stays the point's.
pub(crate) fn cut_label_under(sketch: &Sketch, cursor: DVec2, pixel: f64) -> Option<CornerCutId> {
    if sketch.nearest_point(cursor, ON_A_POINT * pixel).is_some() {
        return None;
    }
    sketch
        .drawn_cuts()
        .iter()
        .map(|cut| (cut.id, label_place(cut, pixel) - cursor))
        .filter(|(_, off)| {
            off.x.abs() <= LABEL_HALF_WIDTH * pixel && off.y.abs() <= LABEL_HALF_HEIGHT * pixel
        })
        .min_by(|(_, one), (_, other)| one.length().total_cmp(&other.length()))
        .map(|(id, _)| id)
}

/// How far either way of its place the value of a cut reads as under the
/// cursor, in pixels: about the size « R10 mm » is written at.
const LABEL_HALF_WIDTH: f64 = 26.0;
const LABEL_HALF_HEIGHT: f64 = 8.0;

/// How near a point a press has to land, in pixels, to take the point rather
/// than the value written over it.
const ON_A_POINT: f64 = 6.0;

/// Where the value of a cut is written: beside the middle of its curve, on
/// the side away from what it rounds off.
fn label_place(cut: &DrawnCut, pixel: f64) -> DVec2 {
    let (middle, outwards) = match cut.curve {
        CutCurve::Arc(arc) => {
            let from = (arc.start - arc.centre).to_angle();
            let half = cao_sketch::sweep_of(arc) / 2.0;
            let middle =
                arc.centre + DVec2::from_angle(from + half) * arc.centre.distance(arc.start);
            (middle, (middle - arc.centre).normalize_or_zero())
        }
        CutCurve::Straight { from, to } => {
            let middle = (from + to) / 2.0;
            (middle, (cut.corner - middle).normalize_or_zero())
        }
    };
    middle + outwards * pixel * LABEL_PIXELS
}

/// Writes the value of every cut beside its curve: « R10 mm » for a fillet,
/// its two distances or its distance and angle for a chamfer — in the colour
/// of a refusal while it does not fit.
pub(super) fn paint_cut_labels(
    painter: &egui::Painter,
    sketch: &Sketch,
    state: &ViewportState,
    millimeters_per_unit: f64,
    view_projection: glam::Mat4,
    (rect, pixel): (egui::Rect, f64),
) {
    for cut in sketch.drawn_cuts() {
        let mm = |units: f64| state.config.unit.format(units * millimeters_per_unit);
        let text = match sketch.corner_cuts()[cut.id.0].shape {
            CutShape::Fillet { radius } => format!("R{}", mm(radius)),
            CutShape::Chamfer(Chamfer::Equal(reach)) => format!("{} × {}", mm(reach), mm(reach)),
            CutShape::Chamfer(Chamfer::Sided { first, second }) => {
                format!("{} × {}", mm(first), mm(second))
            }
            CutShape::Chamfer(Chamfer::Angled { along, degrees }) => {
                format!("{} × {degrees:.1}°", mm(along))
            }
        };
        let at = sketch.plane.to_world(label_place(&cut, pixel));
        let Some(position) = to_screen(at, view_projection, rect) else {
            continue;
        };
        let color = match cut.fits {
            true => egui::Color32::from_rgb(250, 220, 120),
            false => tint_to_color(state.theme.refused),
        };
        painter.text(
            position,
            egui::Align2::CENTER_CENTER,
            text,
            egui::FontId::proportional(13.0),
            color,
        );
    }
}

#[cfg(test)]
mod tests;
