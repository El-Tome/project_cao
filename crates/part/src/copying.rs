//! What a part does when a tool lays copies down: the mirror and the two
//! patterns, each reading its lengths in millimetres and settling the drawing
//! around what it left.

use cao_sketch::{ChosenAxis, Element, PointId, Repeats};

use crate::broken::Broken;
use crate::formula::Formula;
use crate::history::RepeatsAsked;
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
        degrees: &Formula,
        count: &Formula,
    ) -> Option<Outcome> {
        let degrees = self.size(degrees, |_| true)?;
        let Some(count) = count.whole(&self.values) else {
            self.broke(Broken::Operation(self.replaying));
            return None;
        };
        let scale = self.scale();
        let laid = self
            .sketches
            .get_mut(index)?
            .pattern_around(elements, centre, degrees, count);
        self.settle_the_copies(index, laid, scale)
    }

    pub(crate) fn pattern_along(
        &mut self,
        index: usize,
        elements: &[Element],
        direction: ChosenAxis,
        along: &RepeatsAsked,
        across: &RepeatsAsked,
    ) -> Option<Outcome> {
        let (Some(along), Some(across)) = (
            along.worked_out(&self.values),
            across.worked_out(&self.values),
        ) else {
            self.broke(Broken::Operation(self.replaying));
            return None;
        };
        let scale = self.scale();
        let in_units = |run: Repeats| Repeats {
            step: run.step / scale,
            count: run.count,
        };
        let laid = self.sketches.get_mut(index)?.pattern_along(
            elements,
            direction,
            in_units(along),
            in_units(across),
        );
        self.settle_the_copies(index, laid, scale)
    }

    /// Settles the drawing round what a pattern laid, or notes the pattern as
    /// broken when the drawing laid nothing — a count too small, or copies
    /// piling up on one another.
    fn settle_the_copies<T>(
        &mut self,
        index: usize,
        laid: Option<T>,
        scale: f64,
    ) -> Option<Outcome> {
        if laid.is_none() {
            self.broke(Broken::Operation(self.replaying));
            return None;
        }
        self.sketches.get_mut(index)?.resolve(scale);
        None
    }
}
