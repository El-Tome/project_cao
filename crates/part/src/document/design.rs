//! How a design goes into the archive, and how it comes back out.
//!
//! `design/history.json` is the ordered index: what each major step is, its
//! number, and what it is raised from. What a step is made of lives in a
//! folder of its own, named after its kind and its number — `design/sketch-1/`
//! — so that changing one step touches one file, and so that the geometry a
//! step rebuilds to has somewhere to be cached beside it.
//!
//! The folder is not earned: a one-operation extrusion gets one too. A short
//! step written straight into the index would be a second layout to read, and
//! the day that extrusion's distance is edited it would have to grow a folder
//! anyway.

use std::io::{Read, Seek, Write};

use crate::errors::PartFileError;
use crate::history::{History, Index, Operation};

pub(super) const INDEX_ENTRY: &str = "design/history.json";

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
    let mut files = vec![Written {
        path: INDEX_ENTRY.to_string(),
        text: serde_json::to_string_pretty(&history.index())?,
    }];
    for (rank, step) in history.steps().iter().enumerate() {
        files.push(Written {
            path: step.folder(),
            text: serde_json::to_string_pretty(history.operations_of(rank))?,
        });
    }
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
    let index: Index = serde_json::from_str(&index_text)?;

    let mut read = vec![index_text];
    let mut operations: Vec<Operation> = Vec::new();
    for step in &index.steps {
        let text = super::read_entry(archive, &step.folder())?;
        operations.extend(serde_json::from_str::<Vec<Operation>>(&text)?);
        read.push(text);
    }

    let history = History::restore(index, operations).ok_or(PartFileError::BrokenDesign)?;
    Ok((history, read))
}
