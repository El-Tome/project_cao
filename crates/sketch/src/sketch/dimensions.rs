//! The values a drawing carries: finding one, setting it, moving where it is
//! written, and taking it away.

use glam::DVec2;

use super::Sketch;
use crate::annotation::AnnotationMetrics;
use crate::constraints::{Dimension, DimensionTarget};

impl Sketch {
    pub fn erase_dimension(&mut self, target: DimensionTarget) {
        self.dimensions
            .retain(|dimension| dimension.target != target);
    }

    /// Moves an annotation away from where it would sit on its own.
    pub fn offset_dimension(&mut self, target: DimensionTarget, offset: DVec2) {
        if let Some(dimension) = self
            .dimensions
            .iter_mut()
            .find(|dimension| dimension.target == target)
        {
            dimension.offset = Some(offset);
        }
    }

    /// The dimension whose annotation sits nearest `position`, within `tolerance`.
    pub fn nearest_dimension(
        &self,
        position: DVec2,
        tolerance: f64,
        metrics: AnnotationMetrics,
    ) -> Option<DimensionTarget> {
        self.anchors(metrics)
            .into_iter()
            .map(|(target, at)| (target, at.distance(position)))
            .filter(|(_, distance)| *distance <= tolerance)
            .min_by(|a, b| a.1.total_cmp(&b.1))
            .map(|(target, _)| target)
    }

    pub fn dimension_of(&self, target: DimensionTarget) -> Option<&Dimension> {
        self.dimensions
            .iter()
            .find(|dimension| dimension.target == target)
    }

    /// Records a value the user typed, replacing any previous one on the same
    /// target. Moving the geometry is a separate step: the very first dimension
    /// of a document sets its scale instead of resizing anything, and a driven
    /// one never moves anything at all.
    pub fn set_dimension(&mut self, target: DimensionTarget, value: f64, driven: bool) {
        match self
            .dimensions
            .iter_mut()
            .find(|dimension| dimension.target == target)
        {
            Some(existing) => {
                existing.value = value;
                existing.driven = driven;
            }
            None => self.dimensions.push(Dimension {
                target,
                value,
                driven,
                offset: None,
                written: None,
            }),
        }
    }

    /// Says what a value was written as, or that it was written as nothing but
    /// itself. The drawing never reads it back.
    pub fn write_dimension_as(&mut self, target: DimensionTarget, written: Option<String>) {
        if let Some(dimension) = self
            .dimensions
            .iter_mut()
            .find(|dimension| dimension.target == target)
        {
            dimension.written = written;
        }
    }

    /// A value carried over from one the drawing had, onto something the
    /// drawing still has: its number, whether it drives, where it is written
    /// and what it was written as all go with it.
    pub(crate) fn carry_dimension(&mut self, onto: DimensionTarget, held: &Dimension) {
        self.set_dimension(onto, held.value, held.driven);
        if let Some(offset) = held.offset {
            self.offset_dimension(onto, offset);
        }
        self.write_dimension_as(onto, held.written.clone());
    }
}
