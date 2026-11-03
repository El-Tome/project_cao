use cao_prefs::theme::Theme;
use cao_sketch::{CornerCutId, CutCurve, Sketch};
use glam::DVec2;

use super::super::curves::push_line;
use super::super::tint;
use super::{push_dashed, push_triangle};
use crate::screens::viewport::ViewScale;

/// A chamfer is read as a run, the way a trait is. A fillet's radius runs from
/// its centre to the middle of its arc, where « Rayon » is written.
pub(super) fn push_cut(
    out: &mut Vec<cao_render::Vertex>,
    sketch: &Sketch,
    cut: CornerCutId,
    theme: &Theme,
    scale: ViewScale,
) {
    let (Some(drawn), Some(middle)) = (sketch.drawn_cut(cut), sketch.cut_middle(cut)) else {
        return;
    };
    match drawn.curve {
        CutCurve::Straight { from, to } => push_triangle(out, sketch, from, to, theme, scale),
        CutCurve::Arc(arc) => {
            let arm = scale.world_size_of(5.0);
            for across in [DVec2::X * arm, DVec2::Y * arm] {
                push_line(
                    out,
                    sketch.plane.to_world(arc.centre - across),
                    sketch.plane.to_world(arc.centre + across),
                    tint(theme.measure),
                    theme.sketch_width,
                    false,
                    scale,
                );
            }
            push_dashed(out, sketch, arc.centre, middle, theme.measure, theme, scale);
        }
    }
}
