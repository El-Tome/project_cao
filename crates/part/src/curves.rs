//! The curves a part's sketches are drawn with — circles, arcs and ellipses —
//! laid down again as the history replays them.

use cao_sketch::Constraint;

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
        let center = resolve(sketch, center);
        let first = [&first[0], &first[1]].map(|place| resolve(sketch, place));
        let second = [&second[0], &second[1]].map(|place| resolve(sketch, place));
        let laid = match construction {
            true => sketch.add_construction_ellipse(center, first, second),
            false => sketch.add_ellipse(center, first, second),
        };
        if let Some(ends) = drawn {
            let ends = [&ends[0], &ends[1]].map(|place| resolve(sketch, place));
            sketch.draw_the_stretch(laid, ends[0], ends[1]);
        }
        None
    }
}
