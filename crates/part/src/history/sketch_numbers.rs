//! The number each sketch bears, the « n » it is named by: the highest ever
//! given plus one when it is made, never given again — not to a sketch made
//! after it was dropped, nor once the part is reopened — and its own again
//! when it is redone. A sketch put in among the steps is numbered as any.

use super::{History, Step, StepKind};

impl History {
    /// A step of that kind opened now, a sketch numbered.
    pub(super) fn open(&mut self, kind: StepKind) -> Step {
        let mut step = Step::opened(kind);
        if kind == StepKind::Sketch {
            self.last_sketch_number += 1;
            step.number_as(self.last_sketch_number);
        }
        step
    }

    /// The number the sketch of that rank bears, by the rank the operations
    /// name it by; nothing when there is no such sketch.
    pub fn sketch_number(&self, sketch: usize) -> Option<u32> {
        let (_, place) = *StepKind::Sketch
            .in_the_order_made(&self.steps)
            .get(sketch)?;
        self.steps[place].number()
    }

    /// Numbers, in the order they were made, the sketches of a design written
    /// before they bore one.
    pub(super) fn number_the_unnumbered(&mut self) {
        for (_, place) in StepKind::Sketch.in_the_order_made(&self.steps) {
            if self.steps[place].number().is_none() {
                self.last_sketch_number += 1;
                self.steps[place].number_as(self.last_sketch_number);
            }
        }
    }

    /// Gives each sketch the number the sketch of the same rank bears in
    /// `old`, and goes on numbering from where `old` left off: what compaction
    /// writes keeps the names the part had.
    pub(crate) fn numbered_as(&mut self, old: &History) {
        let made = StepKind::Sketch.in_the_order_made(&self.steps);
        for (rank, (_, place)) in made.into_iter().enumerate() {
            if let Some(number) = old.sketch_number(rank) {
                self.steps[place].number_as(number);
            }
        }
        self.last_sketch_number = old.last_sketch_number;
    }
}
