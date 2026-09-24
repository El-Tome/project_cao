//! What one click of the circle tool does, and the circle the picks so far make.

use cao_part::Operation;
use cao_sketch::{CircleId, CircleMode, DimensionTarget, Found, ToolState};
use glam::DVec2;

use super::{annotation_position, point_ref_at};
use crate::screens::SketchContext;
use crate::wording::outcome;

/// One click of the circle tool: takes what was pointed at, and draws the
/// circle as soon as enough of it is known.
pub(crate) fn draw_circle(
    context: &mut SketchContext<'_>,
    index: usize,
    cursor: DVec2,
    snap: f64,
    pixel: f64,
) -> bool {
    let mode = context.editor.circle_mode;
    let (points, segments) = match &context.editor.tool_state {
        ToolState::Circle { points, segments } => (points.clone(), segments.clone()),
        _ => (Vec::new(), Vec::new()),
    };
    let sketch = &context.document.sketches()[index];

    match cao_sketch::circle_progress(mode, &points, &segments, sketch, cursor, snap) {
        cao_sketch::CircleProgress::NeedsSegment => {
            context.editor.message = Some(context.lang.t("sketch.click_a_trait"));
            false
        }
        cao_sketch::CircleProgress::AlreadyPicked => {
            context.editor.message = Some(crate::wording::circle::asks_for(context.lang, mode));
            false
        }
        cao_sketch::CircleProgress::AddPoint(point) => {
            let mut points = points;
            points.push(point);
            context.editor.tool_state = ToolState::Circle { points, segments };
            context.editor.live.open();
            context.editor.message = Some(crate::wording::circle::asks_for(context.lang, mode));
            false
        }
        cao_sketch::CircleProgress::AddSegment(segment) => {
            let mut segments = segments;
            segments.push(segment);
            context.editor.tool_state = ToolState::Circle { points, segments };
            context.editor.live.open();
            context.editor.message = Some(crate::wording::circle::asks_for(context.lang, mode));
            false
        }
        cao_sketch::CircleProgress::Ready => {
            let Some(found) = circle_from(context, index, cursor, snap) else {
                context.editor.message = Some(context.lang.t("sketch.no_circle_from_these"));
                return false;
            };
            context.editor.tool_state = ToolState::None;
            let sketch = &context.document.sketches()[index];
            let mut touched = segments;
            // The last trait of a three-tangent circle is the one under the
            // cursor at the click, and it holds the circle just as much as
            // the other two.
            if mode == CircleMode::ThreeTangents
                && let Some(last) = sketch.nearest_segment(cursor, snap)
                && !touched.contains(&last)
            {
                touched.push(last);
            }

            // The centre reuses a point already drawn when one is under it, as
            // everywhere else, so shapes hang together instead of stacking
            // points.
            let center = point_ref_at(context, index, found.centre, snap);
            let rim: Vec<cao_part::PointRef> =
                cao_sketch::rim_of(mode, &points, cursor, found.centre)
                    .into_iter()
                    .map(|place| point_ref_at(context, index, place, snap))
                    .collect();
            context.document.apply(Operation::AddCircle {
                sketch: index,
                center,
                radius: found.radius,
                rim,
                construction: context.editor.construction,
            });
            let drawn = CircleId(
                context.document.sketches()[index]
                    .circles()
                    .len()
                    .saturating_sub(1),
            );

            // A circle drawn against traits stays against them: the tangency
            // is the whole point of having pointed at them.
            for segment in touched {
                context.document.apply(Operation::Constrain {
                    sketch: index,
                    constraint: cao_sketch::Constraint::Tangent {
                        circle: drawn,
                        segment,
                        at: None,
                    },
                });
            }
            // And a size typed by hand becomes the dimension it deserves.
            if let Some(diameter) = context.editor.live.typed(0) {
                let target = DimensionTarget::Diameter(drawn);
                let scale = context.document.scale();
                if !context.document.sketches()[index].would_be_redundant(target, scale) {
                    let typed = context.editor.live.typed_as_written(0);
                    let applied = context.document.apply(Operation::SetDimension {
                        sketch: index,
                        target,
                        value: super::super::values::as_typed(typed, diameter),
                        placement: annotation_position(context, index, target, pixel)
                            .map(|placement| placement.offset),
                    });
                    if let Some(message) = outcome::message(context.lang, applied) {
                        context.editor.message = Some(message);
                    }
                }
            }

            context.editor.live.clear();
            context.editor.message = Some(crate::wording::circle::asks_for(context.lang, mode));
            true
        }
    }
}

/// The circle the picks so far and the cursor make. The reading is the
/// drawing's; the third trait of an inscribed circle is picked here, since only
/// the canvas knows what the cursor is over.
pub(crate) fn circle_from(
    context: &SketchContext<'_>,
    index: usize,
    cursor: DVec2,
    snap: f64,
) -> Option<Found> {
    let sketch = context.document.sketches().get(index)?;
    let mode = context.editor.circle_mode;
    let (points, mut segments) = match &context.editor.tool_state {
        ToolState::Circle { points, segments } => (points.clone(), segments.clone()),
        _ => (Vec::new(), Vec::new()),
    };
    if mode == CircleMode::ThreeTangents {
        segments.push(sketch.nearest_segment(cursor, snap)?);
    }
    let lines: Vec<_> = segments
        .iter()
        .map(|segment| sketch.endpoints(*segment))
        .collect();

    cao_sketch::circle_from(
        mode,
        &points,
        &lines,
        cursor,
        context.editor.live.typed(0),
        context.document.scale(),
    )
}
