//! What a part does when a tool lays copies down: the mirror and the two
//! patterns, each reading its lengths in millimetres and settling the drawing
//! around what it left.

use cao_sketch::{ChosenAxis, Element, PointId, Repeats};

use crate::outcome::Outcome;
use crate::state::PartState;

impl PartState {
    pub(crate) fn mirror(
        &mut self,
        index: usize,
        elements: &[Element],
        axis: ChosenAxis,
    ) -> Option<Outcome> {
        let scale = self.scale();
        let sketch = self.sketches.get_mut(index)?;
        sketch.mirror(elements, axis)?;
        sketch.resolve(scale);
        None
    }

    pub(crate) fn pattern_around(
        &mut self,
        index: usize,
        elements: &[Element],
        centre: PointId,
        degrees: f64,
        count: usize,
    ) -> Option<Outcome> {
        let scale = self.scale();
        let sketch = self.sketches.get_mut(index)?;
        sketch.pattern_around(elements, centre, degrees, count)?;
        sketch.resolve(scale);
        None
    }

    pub(crate) fn pattern_along(
        &mut self,
        index: usize,
        elements: &[Element],
        direction: ChosenAxis,
        along: Repeats,
        across: Repeats,
    ) -> Option<Outcome> {
        let scale = self.scale();
        let in_units = |run: Repeats| Repeats {
            step: run.step / scale,
            count: run.count,
        };
        let sketch = self.sketches.get_mut(index)?;
        sketch.pattern_along(elements, direction, in_units(along), in_units(across))?;
        sketch.resolve(scale);
        None
    }
}
