//! What the arc tool shows before its curve is drawn: a radius or a distance
//! typed while the second place is picked, and — for `ByCenter` only, since a
//! `ByEnds` arc is bent to where the cursor points rather than to an angle —
//! a swept angle typed while the third is.

use cao_sketch::{ArcMode, sweep_of};
use glam::DVec2;

use crate::screens::viewport::SketchContext;
use crate::screens::viewport::input::{arc_aimed, arc_preview};

pub(crate) fn live_fields(
    context: &SketchContext<'_>,
    cursor: DVec2,
) -> Option<([&'static str; 2], [f64; 2])> {
    let places = context.editor.arc_places();
    match (context.editor.arc_mode, places.len()) {
        (ArcMode::ByCenter, 1) | (ArcMode::ByEnds, 1) => {
            let aimed = arc_aimed(context, places, cursor);
            let distance = places[0].distance(aimed) * context.document.scale();
            Some((["mm", ""], [distance, 0.0]))
        }
        (ArcMode::ByCenter, 2) => {
            let drawn = arc_preview(context, cursor)?;
            Some((["°", ""], [sweep_of(drawn).to_degrees(), 0.0]))
        }
        _ => None,
    }
}
