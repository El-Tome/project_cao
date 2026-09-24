//! One click of the symmetric line tool: the first remembers the middle, the
//! second draws the segment growing equally on both sides of it. A length
//! typed reaches one edge, the way the plain line tool reads its own anchor.

use cao_part::history::{Operation, PointRef};
use cao_sketch::{ChainAnchor, Constraint, SegmentId, SymmetricClick, ToolState, symmetric_click};
use glam::DVec2;

use super::{born_at, lean_on_an_arm};
use crate::screens::SketchContext;

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
                ChainAnchor::Pending(position) => born_at(sketch, position),
            };
            let opened = context.document.history.mark();
            context.document.apply(Operation::AddSymmetricSegment {
                sketch: index,
                middle: point_ref(middle),
                end: point_ref(end),
                construction: context.editor.construction,
            });

            let sketch = &context.document.sketches()[index];
            let drawn = SegmentId(sketch.segments().len().saturating_sub(1));
            // The arm springs from the middle, which is the point the rule
            // the operation laid holds halfway along the trait.
            let sprung_from = sketch.constraints().iter().find_map(|rule| match rule {
                Constraint::Midpoint { point, segment } if *segment == drawn => Some(*point),
                _ => None,
            });
            if let (Some(sprung_from), true) = (sprung_from, locked.second.is_some()) {
                lean_on_an_arm(context, index, drawn, sprung_from, pixel);
            }
            dimension_the_symmetric_line(context, index, drawn, locked, pixel);
            context.document.history.fold_into_one_gesture(opened);

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

    // What was typed reaches from the middle to one edge; the value laid is
    // the whole trait, twice that.
    let half = context
        .editor
        .live
        .typed_as_written(0)
        .map(|(written, value)| (written.times(2.0), value * 2.0));
    let typed = |target| match target {
        cao_sketch::DimensionTarget::Length(drawn) if drawn == segment => half.clone(),
        _ => None,
    };
    super::super::values::lay_values(context, index, wanted, typed, pixel);
}
