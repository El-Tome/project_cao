use glam::DVec2;
use serde::{Deserialize, Serialize};

use crate::constraints::SketchAxis;
use crate::duplicating::Duplicated;
use crate::element::Element;
use crate::sketch::{SegmentId, Sketch};

/// Below this a trait runs nowhere, and names no direction to mirror across.
const NO_DIRECTION: f64 = 1e-9;

/// What a selection is mirrored across.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MirrorAxis {
    /// A trait of the drawing, which follows it when it moves.
    Trait(SegmentId),
    /// One of the sketch's own two axes.
    Sketch(SketchAxis),
}

impl Sketch {
    /// Copies the elements given to the other side of an axis.
    ///
    /// Nothing when the axis names no direction.
    pub fn mirror(&mut self, of: &[Element], axis: MirrorAxis) -> Option<Duplicated> {
        let (through, along) = self.axis_line(axis)?;
        let kept: Vec<Element> = match axis {
            MirrorAxis::Trait(id) => of
                .iter()
                .copied()
                .filter(|held| *held != Element::Segment(id))
                .collect(),
            MirrorAxis::Sketch(_) => of.to_vec(),
        };
        Some(self.duplicate(&kept, |at| {
            let offset = at - through;
            through + along * 2.0 * offset.dot(along) - offset
        }))
    }

    /// A point the axis runs through, and the direction it runs in.
    fn axis_line(&self, axis: MirrorAxis) -> Option<(DVec2, DVec2)> {
        match axis {
            MirrorAxis::Sketch(SketchAxis::U) => Some((DVec2::ZERO, DVec2::X)),
            MirrorAxis::Sketch(SketchAxis::V) => Some((DVec2::ZERO, DVec2::Y)),
            MirrorAxis::Trait(id) => {
                if self.is_erased_segment(id) || id.0 >= self.segments().len() {
                    return None;
                }
                let (start, end) = self.endpoints(id);
                let along = end - start;
                (along.length() > NO_DIRECTION).then(|| (start, along.normalize()))
            }
        }
    }
}

#[cfg(test)]
mod tests;
