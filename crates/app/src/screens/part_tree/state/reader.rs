//! The step reader, the card above the history in the Part panel: one box per
//! key step the part stands on — a sketch or a volume. A section raises
//! nothing and is no step.

use std::collections::HashMap;

use cao_part::PartDocument;
use cao_part::history::StepKind;

use super::volumes::volume_name;
use super::{PartTree, sketch_name};
use crate::lang::Catalogue;
use crate::screens::viewport::ViewportState;
use crate::ui::badge::Level;

/// One key step, named as its row in the tree is.
#[derive(Clone, Debug, PartialEq)]
pub struct KeyStep {
    pub kind: StepKind,
    pub name: String,
    /// Played: at or before the reading point.
    pub applied: bool,
    /// The level of the gravest alert of its own step, as its row in the
    /// tree wears it, wherever the part is read to.
    pub alert: Option<Level>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct StepReader {
    pub boxes: Vec<KeyStep>,
    /// One between each two boxes, applied when the box it leads to is.
    pub arrows: Vec<bool>,
}

impl StepReader {
    /// The part read to the point the canvas reads it to.
    pub fn seen(document: &PartDocument, viewport: &ViewportState, lang: &Catalogue) -> Self {
        let mut boxes = Self::of(document, lang).boxes;
        for (rank, step) in boxes.iter_mut().enumerate() {
            step.applied = viewport.reads_through(rank);
        }
        Self::arrowed(boxes)
    }

    /// The part read to its end.
    pub fn of(document: &PartDocument, lang: &Catalogue) -> Self {
        let alerts = alerts(&PartTree::of(document, lang));
        let (mut sketches, mut volumes) = (0, 0);
        let boxes = document
            .history
            .applied_operations()
            .iter()
            .enumerate()
            .filter_map(|(step, operation)| {
                let kind = StepKind::opened_by(operation)?;
                let name = match kind {
                    StepKind::Sketch => {
                        sketches += 1;
                        sketch_name(lang, sketches)
                    }
                    StepKind::Extrusion | StepKind::Revolution => {
                        volumes += 1;
                        volume_name(lang, operation, volumes)?
                    }
                    StepKind::Section => return None,
                };
                Some(KeyStep {
                    kind,
                    name,
                    applied: true,
                    alert: alerts.get(&step).copied(),
                })
            })
            .collect();
        Self::arrowed(boxes)
    }

    fn arrowed(boxes: Vec<KeyStep>) -> Self {
        let arrows = boxes.iter().skip(1).map(|step| step.applied).collect();
        Self { boxes, arrows }
    }
}

/// The level of each badge of the tree's sketches and volumes, by the step
/// in the design the row stands for.
fn alerts(tree: &PartTree) -> HashMap<usize, Level> {
    let sketches = tree.sketches.iter().map(|drawn| (drawn.step, drawn.badge));
    let volumes = tree.bodies.iter().map(|body| (body.step, body.badge));
    sketches
        .chain(volumes)
        .filter_map(|(step, badge)| Some((step, badge?.level())))
        .collect()
}
