//! The curves a part's sketches are drawn with — circles, arcs and ellipses —
//! laid down again as the history replays them.

use cao_sketch::{Constraint, PointId, Sketch};

use crate::history::PointRef;
use crate::outcome::Outcome;
use crate::state::{PartState, resolve};

impl PartState {
    /// A circle, with the points clicked on its rim held on it.
    pub(crate) fn add_circle(
        &mut self,
        sketch: usize,
        center: &PointRef,
        radius: f64,
        rim: &[PointRef],
        construction: bool,
    ) -> Option<Outcome> {
        let sketch = self.sketches.get_mut(sketch)?;
        let center = resolve(sketch, center);
        let circle = match construction {
            true => sketch.add_construction_circle(center, radius),
            false => sketch.add_circle(center, radius),
        };
        for place in rim {
            let point = resolve(sketch, place);
            sketch.add_constraint(Constraint::OnCircle { point, circle });
        }
        None
    }

    /// An arc, from its centre, its start and its end.
    pub(crate) fn add_arc(
        &mut self,
        sketch: usize,
        places: [&PointRef; 3],
        construction: bool,
    ) -> Option<Outcome> {
        let sketch = self.sketches.get_mut(sketch)?;
        let [center, start, end] = places.map(|place| resolve(sketch, place));
        match construction {
            true => sketch.add_construction_arc(center, start, end),
            false => sketch.add_arc(center, start, end),
        };
        None
    }

    /// An ellipse, from its centre and the ends of its two axes — and, for an
    /// arc of one, the stretch of it that is drawn.
    ///
    /// **Within one operation the same reference names the same point.** Half
    /// an ellipse names its centre twice — once as the centre, once as the end
    /// its second axis stands on — and is drawn between the ends of its own
    /// first axis. Resolved afresh each time, a `New` reference would lay a
    /// second point on top of the first, and the curve would stand on points
    /// nobody can see beside the ones it was given.
    pub(crate) fn add_ellipse(
        &mut self,
        sketch: usize,
        center: &PointRef,
        first: &[PointRef; 2],
        second: &[PointRef; 2],
        construction: bool,
        drawn: &Option<[PointRef; 2]>,
    ) -> Option<Outcome> {
        let sketch = self.sketches.get_mut(sketch)?;
        let mut laid_here: Vec<(&PointRef, PointId)> = Vec::new();
        let center = resolved_once(sketch, &mut laid_here, center);
        let first =
            [&first[0], &first[1]].map(|place| resolved_once(sketch, &mut laid_here, place));
        let second =
            [&second[0], &second[1]].map(|place| resolved_once(sketch, &mut laid_here, place));
        let laid = match construction {
            true => sketch.add_construction_ellipse(center, first, second),
            false => sketch.add_ellipse(center, first, second),
        };
        if let Some(ends) = drawn {
            let ends =
                [&ends[0], &ends[1]].map(|place| resolved_once(sketch, &mut laid_here, place));
            sketch.draw_the_stretch(laid, ends[0], ends[1]);
        }
        None
    }
}

/// The point a reference names, laid at most once for the operation in hand.
///
/// `resolve` makes a fresh point for every `New` reference it is handed. An
/// operation naming the same one twice — an ellipse whose second axis stands on
/// its own centre — would get two points in the same place, and the drawing
/// would stand on whichever of them each field happened to hold.
fn resolved_once<'a>(
    sketch: &mut Sketch,
    laid: &mut Vec<(&'a PointRef, PointId)>,
    place: &'a PointRef,
) -> PointId {
    if let Some((_, point)) = laid.iter().find(|(known, _)| *known == place) {
        return *point;
    }
    let point = resolve(sketch, place);
    laid.push((place, point));
    point
}
