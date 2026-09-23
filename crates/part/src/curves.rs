//! The curves a part's sketches are drawn with — circles, arcs and ellipses —
//! laid down again as the history replays them.

use cao_sketch::{Constraint, PointId};

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
    /// A stretch named by one of the five references the ellipse was just laid
    /// on comes back as that very point. Half an ellipse is drawn between the
    /// ends of its own first axis, and those ends are made by this operation:
    /// resolved a second time, a `New` reference would lay a second point on
    /// top of each of them, and the curve would run between two points nobody
    /// can see instead of the ones it stands on.
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
        let laid_here: Vec<(&PointRef, PointId)> =
            [center, &first[0], &first[1], &second[0], &second[1]]
                .into_iter()
                .map(|place| (place, resolve(sketch, place)))
                .collect();
        let [center, first_start, first_end, second_start, second_end] =
            [0, 1, 2, 3, 4].map(|rank| laid_here[rank].1);
        let laid = match construction {
            true => sketch.add_construction_ellipse(
                center,
                [first_start, first_end],
                [second_start, second_end],
            ),
            false => {
                sketch.add_ellipse(center, [first_start, first_end], [second_start, second_end])
            }
        };
        if let Some(ends) = drawn {
            let ends = [&ends[0], &ends[1]].map(|place| {
                match laid_here.iter().find(|(known, _)| *known == place) {
                    Some((_, point)) => *point,
                    None => resolve(sketch, place),
                }
            });
            sketch.draw_the_stretch(laid, ends[0], ends[1]);
        }
        None
    }
}
