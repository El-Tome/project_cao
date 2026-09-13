//! One click of the symmetric line tool: the first remembers the middle, the
//! second draws the segment growing equally on both sides of it. A length
//! typed reaches one edge, the way the plain line tool reads its own anchor.

use cao_part::history::{Operation, PointRef};
use cao_sketch::{ChainAnchor, SegmentId, SymmetricClick, ToolState, symmetric_click};
use glam::DVec2;

use super::annotation_position;
use crate::screens::viewport::SketchContext;
use crate::wording::dimension;

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
            let point_ref = |anchor: ChainAnchor| match anchor {
                ChainAnchor::Point(id) => PointRef::Existing(id),
                ChainAnchor::Pending(position) => PointRef::New(position),
            };
            context.document.apply(Operation::AddSymmetricSegment {
                sketch: index,
                middle: point_ref(middle),
                end: point_ref(end),
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
        let outcome = context.document.apply(Operation::SetDimension {
            sketch: index,
            target,
            value,
            placement: annotation_position(context, index, target, pixel)
                .map(|placement| placement.offset),
        });
        if let Some(message) = dimension::outcome_message(context.lang, outcome) {
            context.editor.message = Some(message);
        }
    }
}
