//! How a design goes into the archive, and how it comes back out.
//!
//! `design/history.json` is the ordered index: what each major step is and
//! **what it stands on**. What a step *does* lives in a folder of its own,
//! named after its kind and its rank in that kind — `design/sketch-0/` — so
//! that changing one step touches one file, and so that the geometry a step
//! rebuilds to has somewhere to be cached beside it.
//!
//! The line between the two is not arbitrary. A field belongs in the index
//! when losing what it points at is something the user has to be told about:
//! the plane a sketch is drawn on, the sketch an extrusion lifts, the areas
//! clicked, the axis a revolution turns around. A distance, an angle or a mode
//! points at nothing and cannot be lost, so it stays in the folder — which is
//! also what keeps the index still while the tools that write those values
//! grow.
//!
//! That line cuts the operation that opens a step in two. A sketch has nothing
//! left once its plane has gone up: `CreateSketch` disappears from the folder
//! and lives on as a line of the index — keeping its number, so that undo can
//! still take back the making of the sketch. An extrusion leaves its distance
//! and its mode behind. Only this module knows about the cut; above it a
//! history is one list of whole operations.

use std::io::{Read, Seek, Write};

use cao_sketch::WorkPlane;
use glam::DVec2;
use serde::{Deserialize, Serialize};

use crate::errors::PartFileError;
use crate::history::{
    ExtrusionMode, FaceAnchor, History, Index, Operation, RevolutionAxis, Step, StepKind,
};

pub(super) const INDEX_ENTRY: &str = "design/history.json";

/// Where a step stands, and what it acts on.
#[derive(Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
enum Stands {
    Sketch {
        plane: WorkPlane,
        #[serde(default)]
        on: Option<FaceAnchor>,
    },
    Extrusion {
        sketch: usize,
        picks: Vec<DVec2>,
    },
    Revolution {
        sketch: usize,
        picks: Vec<DVec2>,
        axis: RevolutionAxis,
    },
}

/// What the operation opening a step sets, once the index has taken what the
/// step stands on. A sketch leaves nothing: its plane was all it was.
#[derive(Serialize, Deserialize)]
#[serde(tag = "opens", rename_all = "lowercase")]
enum Opening {
    Extrusion { distance: f64, mode: ExtrusionMode },
    Revolution { angle: f64, mode: ExtrusionMode },
}

/// One line of the index.
#[derive(Serialize, Deserialize)]
struct Listed {
    #[serde(flatten)]
    stands_on: Stands,
    operations: Vec<u32>,
}

/// The index as the archive carries it.
#[derive(Serialize, Deserialize)]
struct Contents {
    steps: Vec<Listed>,
    applied: usize,
    last_operation_number: u32,
}

/// A step's folder: what its opening operation sets, and everything recorded
/// under it afterwards.
#[derive(Serialize, Deserialize)]
struct Folder {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    opening: Option<Opening>,
    operations: Vec<Operation>,
}

/// One file of the design, and what goes in it.
pub(super) struct Written {
    path: String,
    text: String,
}

impl Written {
    pub(super) fn text(&self) -> &str {
        &self.text
    }
}

/// The design, laid out as the files that carry it: the index first, then one
/// per step in the order the index names them.
pub(super) fn laid_out(history: &History) -> Result<Vec<Written>, PartFileError> {
    let mut listed = Vec::new();
    let mut folders = Vec::new();
    let mut ranks = Ranks::default();

    for (rank, step) in history.steps().iter().enumerate() {
        let operations = history.operations_of(rank);
        let (stands_on, opening) = taken_apart(&operations[0])?;
        listed.push(Listed {
            stands_on,
            operations: step.operations().to_vec(),
        });
        folders.push(Written {
            path: ranks.folder_of(step.kind()),
            text: serde_json::to_string_pretty(&Folder {
                opening,
                operations: operations[1..].to_vec(),
            })?,
        });
    }

    let index = history.index();
    let mut files = vec![Written {
        path: INDEX_ENTRY.to_string(),
        text: serde_json::to_string_pretty(&Contents {
            steps: listed,
            applied: index.applied,
            last_operation_number: index.last_operation_number,
        })?,
    }];
    files.append(&mut folders);
    Ok(files)
}

pub(super) fn write<W: Write + Seek>(
    archive: &mut zip::ZipWriter<W>,
    options: zip::write::SimpleFileOptions,
    files: &[Written],
) -> Result<(), PartFileError> {
    for file in files {
        archive.start_file(&file.path, options)?;
        archive
            .write_all(file.text.as_bytes())
            .map_err(zip::result::ZipError::from)?;
    }
    Ok(())
}

/// The design, and the text every one of its files held.
///
/// The text comes back out with it because the geometry cache is guarded by a
/// print taken over the whole design: reading it again from the archive to
/// take that print would cost a second pass over what was just read.
pub(super) fn read<R: Read + Seek>(
    archive: &mut zip::ZipArchive<R>,
) -> Result<(History, Vec<String>), PartFileError> {
    let index_text = super::read_entry(archive, INDEX_ENTRY)?;
    let contents: Contents = serde_json::from_str(&index_text)?;

    let mut read = vec![index_text];
    let mut steps = Vec::new();
    let mut operations: Vec<Operation> = Vec::new();
    let mut ranks = Ranks::default();

    for listed in contents.steps {
        let kind = kind_of(&listed.stands_on);
        let text = super::read_entry(archive, &ranks.folder_of(kind))?;
        let folder: Folder = serde_json::from_str(&text)?;
        operations.push(put_together(listed.stands_on, folder.opening)?);
        operations.extend(folder.operations);
        steps.push(Step::listed(kind, listed.operations));
        read.push(text);
    }

    let index = Index {
        steps,
        applied: contents.applied,
        last_operation_number: contents.last_operation_number,
    };
    let history = History::restore(index, operations).ok_or(PartFileError::BrokenDesign)?;
    Ok((history, read))
}

/// How many steps of each kind have gone by, which is what names their
/// folders: the rank the operations already speak in, so that `sketch-1` is
/// the sketch those operations call 1.
#[derive(Default)]
struct Ranks {
    sketches: usize,
    extrusions: usize,
    revolutions: usize,
}

impl Ranks {
    fn folder_of(&mut self, kind: StepKind) -> String {
        let seen = match kind {
            StepKind::Sketch => &mut self.sketches,
            StepKind::Extrusion => &mut self.extrusions,
            StepKind::Revolution => &mut self.revolutions,
        };
        *seen += 1;
        format!("design/{}-{}/steps.json", kind.folder(), *seen - 1)
    }
}

fn kind_of(stands_on: &Stands) -> StepKind {
    match stands_on {
        Stands::Sketch { .. } => StepKind::Sketch,
        Stands::Extrusion { .. } => StepKind::Extrusion,
        Stands::Revolution { .. } => StepKind::Revolution,
    }
}

/// The operation opening a step, split into what the index holds and what its
/// folder keeps.
fn taken_apart(opening: &Operation) -> Result<(Stands, Option<Opening>), PartFileError> {
    Ok(match opening {
        Operation::CreateSketch { plane, on } => (
            Stands::Sketch {
                plane: *plane,
                on: *on,
            },
            None,
        ),
        Operation::Extrude {
            sketch,
            picks,
            distance,
            mode,
        } => (
            Stands::Extrusion {
                sketch: *sketch,
                picks: picks.clone(),
            },
            Some(Opening::Extrusion {
                distance: *distance,
                mode: *mode,
            }),
        ),
        Operation::Revolve {
            sketch,
            picks,
            axis,
            angle,
            mode,
        } => (
            Stands::Revolution {
                sketch: *sketch,
                picks: picks.clone(),
                axis: *axis,
            },
            Some(Opening::Revolution {
                angle: *angle,
                mode: *mode,
            }),
        ),
        _ => return Err(PartFileError::BrokenDesign),
    })
}

/// The same operation, back in one piece.
fn put_together(stands_on: Stands, opening: Option<Opening>) -> Result<Operation, PartFileError> {
    Ok(match (stands_on, opening) {
        (Stands::Sketch { plane, on }, None) => Operation::CreateSketch { plane, on },
        (Stands::Extrusion { sketch, picks }, Some(Opening::Extrusion { distance, mode })) => {
            Operation::Extrude {
                sketch,
                picks,
                distance,
                mode,
            }
        }
        (
            Stands::Revolution {
                sketch,
                picks,
                axis,
            },
            Some(Opening::Revolution { angle, mode }),
        ) => Operation::Revolve {
            sketch,
            picks,
            axis,
            angle,
            mode,
        },
        _ => return Err(PartFileError::BrokenDesign),
    })
}

#[cfg(test)]
mod tests;
