//! One click of the symmetric line tool: the first remembers the middle, the
//! second draws the segment growing equally on both sides of it. A length
//! typed reaches one edge, the way the plain line tool reads its own anchor.

use cao_part::history::Operation;
use cao_sketch::{SegmentId, SymmetricClick, ToolState, symmetric_click};
use glam::DVec2;

use super::annotation_position;
use crate::screens::viewport::SketchContext;
use crate::wording::outcome;

pub(crate) fn draw_symmetric_line_point(
    context: &mut SketchContext<'_>,
    index: usize,
    cursor: DVec2,
    snap: f64,
    pixel: f64,
) -> bool {
    let sketch = &context.document.sketches()[index];
    let middle = match context.editor.tool_state {
        ToolState::SymmetricLine { middle } => Some(middle),
        _ => None,
    };
    let locked = context.editor.live.locked();
    let scale = context.document.scale();

    match symmetric_click(sketch, middle, cursor, snap, locked, scale) {
        SymmetricClick::Started(middle) => {
            context.editor.tool_state = ToolState::SymmetricLine { middle };
            context.editor.live.open();
            false
        }
        SymmetricClick::Ignored => false,
        SymmetricClick::Drew { middle, end } => {
            context.document.apply(Operation::AddSymmetricSegment {
                sketch: index,
                middle: super::point_ref_of(context, index, middle, snap),
                end: super::point_ref_of(context, index, end, snap),
                construction: context.editor.construction,
            });

            let sketch = &context.document.sketches()[index];
            let drawn = SegmentId(sketch.segments().len().saturating_sub(1));
            dimension_the_symmetric_line(context, index, drawn, locked, pixel);

            context.editor.tool_state = ToolState::None;
            context.editor.live.clear();
            true
        }
    }
}

/// Places on the segment just drawn its own length and the angle it was drawn
/// at.
fn dimension_the_symmetric_line(
    context: &mut SketchContext<'_>,
    index: usize,
    segment: SegmentId,
    locked: cao_sketch::LockedInput,
    pixel: f64,
) {
    let scale = context.document.scale();
    let wanted = cao_sketch::symmetric_segment_dimensions(
        &context.document.sketches()[index],
        segment,
        locked,
        scale,
    );

    for (target, value) in wanted {
        let applied = context.document.apply(Operation::SetDimension {
            sketch: index,
            target,
            value,
            placement: annotation_position(context, index, target, pixel)
                .map(|placement| placement.offset),
        });
        if let Some(message) = outcome::message(context.lang, applied) {
            context.editor.message = Some(message);
        }
    }
}
