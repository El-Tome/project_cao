//! The step reader, the card above the history in the Part panel: one box per
//! key step the part stands on — a sketch or a volume. A section raises
//! nothing and is no step.

use cao_part::PartDocument;
use cao_part::history::StepKind;

use super::sketch_name;
use super::volumes::volume_name;
use crate::lang::Catalogue;

/// One key step, named as its row in the tree is.
#[derive(Clone, Debug, PartialEq)]
pub struct KeyStep {
    pub kind: StepKind,
    pub name: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct StepReader {
    pub boxes: Vec<KeyStep>,
}

impl StepReader {
    pub fn of(document: &PartDocument, lang: &Catalogue) -> Self {
        let (mut sketches, mut volumes) = (0, 0);
        let boxes = document
            .history
            .applied_operations()
            .iter()
            .filter_map(|operation| {
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
                Some(KeyStep { kind, name })
            })
            .collect();
        Self { boxes }
    }
}
