//! What a refusal named, blinking on the canvas: values on a drawing, faces
//! of the part.

use cao_sketch::DimensionTarget;

use super::ViewportState;

/// What a refusal named that stands on the canvas — values on a drawing,
/// faces of the part — and when it was refused.
pub(super) struct Blinking {
    values: Vec<(usize, DimensionTarget)>,
    faces: Vec<usize>,
    since: f64,
}

impl ViewportState {
    /// Has what a refusal named blink from `now` — these values on the
    /// drawing, these faces of the part — so that it is seen where it is.
    pub fn blink(&mut self, values: Vec<(usize, DimensionTarget)>, faces: Vec<usize>, now: f64) {
        self.blinking = (!values.is_empty() || !faces.is_empty()).then_some(Blinking {
            values,
            faces,
            since: now,
        });
    }

    /// The values of a sketch blinking at `now`, and whether they are lit or
    /// dark at that instant — nothing once the blinking is over.
    pub(crate) fn blinking_on(
        &self,
        sketch: usize,
        now: f64,
    ) -> Option<(Vec<DimensionTarget>, bool)> {
        let blinking = self.blinking.as_ref()?;
        let lit = crate::screens::blinking::lit(blinking.since, now)?;
        let here: Vec<DimensionTarget> = blinking
            .values
            .iter()
            .filter(|(on, _)| *on == sketch)
            .map(|(_, target)| *target)
            .collect();
        (!here.is_empty()).then_some((here, lit))
    }

    /// The faces of the part blinking at `now`, and whether they are lit or
    /// dark at that instant — nothing once the blinking is over.
    pub(crate) fn faces_blinking(&self, now: f64) -> Option<(&[usize], bool)> {
        let blinking = self.blinking.as_ref()?;
        let lit = crate::screens::blinking::lit(blinking.since, now)?;
        (!blinking.faces.is_empty()).then_some((blinking.faces.as_slice(), lit))
    }
}
