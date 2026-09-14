//! What one click of the trim tool does.

use cao_part::Operation;
use cao_sketch::Sketch;
use glam::DVec2;

use crate::screens::viewport::SketchContext;
use crate::wording::outcome;

/// One click of the trim tool: takes out the stretch of trait or of curve the
/// click fell in, between the two points sitting on either side of it.
pub(crate) fn trim(
    context: &mut SketchContext<'_>,
    index: usize,
    cursor: DVec2,
    snap: f64,
) -> bool {
    context.editor.message = Some(context.lang.t("sketch.click_a_stretch"));

    let Some(sketch) = context.document.sketches().get(index) else {
        return false;
    };
    let Some(cut) = cut_under(sketch, index, cursor, snap) else {
        return false;
    };

    let applied = context.document.apply(cut);
    if let Some(message) = outcome::message(context.lang, applied) {
        context.editor.message = Some(message);
    }
    true
}

/// Which cut the click is asking for.
///
/// The straight trait first, then the curve — the order `Sketch::pick` and the
/// constraint tool already read a click in. Where a trait runs into a curve
/// both are within reach of the same click, and a tool that answered with
/// whichever came out of the drawing first would cut a different element
/// depending on the order they were drawn in.
fn cut_under(sketch: &Sketch, index: usize, cursor: DVec2, snap: f64) -> Option<Operation> {
    if let Some(segment) = sketch.nearest_segment(cursor, snap)
        && let Some((from, to)) = sketch.stretch_at(segment, cursor)
    {
        return Some(Operation::Trim {
            sketch: index,
            segment,
            from,
            to,
        });
    }
    let arc = sketch.nearest_arc(cursor, snap)?;
    let (from, to) = sketch.arc_stretch_at(arc, cursor)?;
    Some(Operation::TrimArc {
        sketch: index,
        arc,
        from,
        to,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cao_sketch::WorkPlane;

    /// A trait laid across the top of a curve that grazes it, so that one click
    /// is within reach of both.
    fn a_trait_touching_a_curve() -> Sketch {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let left = sketch.add_point(DVec2::ZERO);
        let right = sketch.add_point(DVec2::new(10.0, 0.0));
        sketch.add_segment(left, right);
        let centre = sketch.add_point(DVec2::new(5.0, -5.0));
        let east = sketch.add_point(DVec2::new(10.0, -5.0));
        let west = sketch.add_point(DVec2::new(0.0, -5.0));
        sketch.add_arc(centre, east, west);
        sketch
    }

    #[test]
    fn a_click_within_reach_of_both_a_trait_and_a_curve_cuts_the_trait() {
        let sketch = a_trait_touching_a_curve();

        let cut = cut_under(&sketch, 0, DVec2::new(5.0, 0.0), 0.5);

        assert!(
            matches!(cut, Some(Operation::Trim { .. })),
            "the click cut {cut:?}",
        );
    }

    #[test]
    fn a_click_the_trait_is_out_of_reach_of_cuts_the_curve() {
        let sketch = a_trait_touching_a_curve();

        let cut = cut_under(&sketch, 0, DVec2::new(1.47, -1.47), 0.5);

        assert!(
            matches!(cut, Some(Operation::TrimArc { .. })),
            "the click cut {cut:?}",
        );
    }
}
