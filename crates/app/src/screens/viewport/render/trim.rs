//! What the trim tool shows before the click: the stretch that will go, in the
//! colour of what disappears.
//!
//! The other tools' previews say *will appear* and are drawn as a promise; this
//! one says *will disappear*, which is why it has a colour of its own and is
//! drawn over the drawing rather than in place of it. What is aimed at stays
//! visible: hiding it already would take away the very thing being aimed at.

use cao_prefs::theme::Theme;
use cao_sketch::{Going, Sketch, Stretch};

use super::curves::{push_arc_at, push_circle_at, push_ellipse_run, push_line};
use super::tint;
use crate::screens::viewport::input::trim_shows;
use crate::screens::viewport::{PICK_PIXELS, SketchContext, ViewScale};

/// How much thicker than the drawing the doomed stretch is drawn.
pub(crate) const LOUDER: f32 = 1.8;

/// What the tool in hand would take out of the drawing being edited, worked
/// out once a frame so that the stretch, the values and the marks all read off
/// the same answer.
pub(crate) fn what_would_go(context: &SketchContext<'_>, scale: ViewScale) -> Option<Going> {
    let index = context.editor.active_sketch()?;
    let sketch = context.document.sketches().get(index)?;
    let cursor = context.editor.cursor?;
    trim_shows(
        sketch,
        context.editor,
        index,
        cursor,
        scale.world_size_of(PICK_PIXELS),
    )
}

pub(crate) fn push_going(
    out: &mut Vec<cao_render::Vertex>,
    sketch: &Sketch,
    going: &Going,
    theme: &Theme,
    scale: ViewScale,
) {
    let color = tint(theme.going);
    let width = theme.sketch_width * LOUDER;
    match going.stretch {
        Stretch::Straight { from, to } => push_line(
            out,
            sketch.plane.to_world(from),
            sketch.plane.to_world(to),
            color,
            width,
            going.construction,
            scale,
        ),
        Stretch::Curved(drawn) => {
            push_arc_at(out, sketch, drawn, color, width, going.construction, scale)
        }
        Stretch::Round { centre, reach } => push_circle_at(
            out,
            sketch,
            centre,
            reach,
            color,
            width,
            going.construction,
            scale,
        ),
        Stretch::Oval { drawn, from, sweep } => push_ellipse_run(
            out,
            sketch,
            drawn,
            from,
            sweep,
            color,
            width,
            going.construction,
            scale,
        ),
    }
}

#[cfg(test)]
mod tests;
