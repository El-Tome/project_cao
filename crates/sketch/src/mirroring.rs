use crate::axis::ChosenAxis;
use crate::duplicating::Duplicated;
use crate::element::Element;
use crate::sketch::Sketch;

impl Sketch {
    /// Copies the elements given to the other side of an axis.
    ///
    /// Nothing when the axis names no direction.
    pub fn mirror(&mut self, of: &[Element], axis: ChosenAxis) -> Option<Duplicated> {
        let (through, along) = self.axis_line(axis)?;
        Some(self.duplicate(of, |at| {
            let offset = at - through;
            through + along * 2.0 * offset.dot(along) - offset
        }))
    }
}

#[cfg(test)]
mod tests;
