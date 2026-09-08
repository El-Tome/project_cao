use std::io::{Cursor, Read, Write};
use std::path::{Path, PathBuf};

use cao_sketch::Sketch;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::PartFileError;
use crate::file_name;
use crate::history::{History, Operation};
use crate::ports::Files;
use crate::state::{DimensionOutcome, PartState};

/// Bumped whenever the layout of a saved part changes.
///
/// Older versions are refused rather than converted: while the tool is still
/// taking shape, a conversion would be more likely to rebuild a part wrongly
/// than to save anything worth keeping.
pub const SCHEMA_VERSION: u32 = 3;

const METADATA_ENTRY: &str = "part.json";
const HISTORY_ENTRY: &str = "history.json";

/// What a part is, as opposed to what has been done to it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartMetadata {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub schema_version: u32,
}

/// A part in memory: what it is, everything done to it, and the geometry that
/// follows from replaying that history.
///
/// A part file is an archive rather than one JSON blob: metadata, history and —
/// later — thumbnails or exported meshes are separate concerns with different
/// lifetimes, and one file per concern keeps a change to one from rewriting the
/// others.
#[derive(Debug, Clone)]
pub struct PartDocument {
    pub metadata: PartMetadata,
    pub history: History,
    state: PartState,
}

impl PartDocument {
    pub fn new(name: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            metadata: PartMetadata {
                id: Uuid::new_v4(),
                name: name.into(),
                created_at: now,
                modified_at: now,
                schema_version: SCHEMA_VERSION,
            },
            history: History::default(),
            state: PartState::default(),
        }
    }

    pub fn name(&self) -> &str {
        &self.metadata.name
    }

    pub fn sketches(&self) -> &[Sketch] {
        &self.state.sketches
    }

    /// The matter of the part, as one surface.
    pub fn body(&self) -> &cao_solid::Mesh {
        &self.state.body
    }

    pub fn scale(&self) -> f64 {
        self.state.scale()
    }

    pub fn has_scale(&self) -> bool {
        self.state.has_scale()
    }

    pub fn to_millimeters(&self, units: f64) -> f64 {
        self.state.to_millimeters(units)
    }

    /// What the geometry measures for a dimension target, right now.
    pub fn measured(&self, sketch: usize, target: cao_sketch::DimensionTarget) -> Option<f64> {
        self.state.measured(sketch, target)
    }

    /// Records an operation and applies it. Recording and applying go together
    /// so the two can never fall out of step.
    pub fn apply(&mut self, operation: Operation) -> Option<DimensionOutcome> {
        let outcome = self.state.apply(&operation);
        self.history.push(operation);
        self.metadata.modified_at = Utc::now();
        outcome
    }

    pub fn undo(&mut self) -> bool {
        self.history.undo() && self.rebuild()
    }

    pub fn redo(&mut self) -> bool {
        self.history.redo() && self.rebuild()
    }

    /// Puts the part back the way it was after `applied` operations.
    pub fn rewind_to(&mut self, applied: usize) {
        self.history.rewind_to(applied);
        self.rebuild();
    }

    fn rebuild(&mut self) -> bool {
        self.state = PartState::rebuild(&self.history);
        true
    }

    /// Creates a new part and writes it to `dir/<name>.caopart`.
    pub fn create_in(
        files: &impl Files,
        dir: &Path,
        name: impl Into<String>,
    ) -> Result<(Self, PathBuf), PartFileError> {
        let mut doc = Self::new(name);
        let (free, path) = file_name::free_in(files, dir, doc.name());
        doc.metadata.name = free;
        doc.save(files, &path)?;
        Ok((doc, path))
    }

    pub fn save(&self, files: &impl Files, path: &Path) -> Result<(), PartFileError> {
        let mut archive = zip::ZipWriter::new(Cursor::new(Vec::new()));
        let options: zip::write::SimpleFileOptions = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);

        archive.start_file(METADATA_ENTRY, options)?;
        write_entry(&mut archive, &serde_json::to_string_pretty(&self.metadata)?)?;
        archive.start_file(HISTORY_ENTRY, options)?;
        write_entry(&mut archive, &serde_json::to_string_pretty(&self.history)?)?;

        let bytes = archive.finish()?.into_inner();
        files.write(path, &bytes)?;
        Ok(())
    }

    pub fn load(files: &impl Files, path: &Path) -> Result<Self, PartFileError> {
        let bytes = files.read(path)?;

        // Files from before the archive format are not read: the tool changed
        // too much for a conversion to be worth trusting, and nothing of value
        // was drawn with those versions.
        if !bytes.starts_with(b"PK") {
            return Err(PartFileError::UnsupportedVersion(1));
        }

        let mut archive = zip::ZipArchive::new(Cursor::new(bytes))?;
        let metadata: PartMetadata =
            serde_json::from_str(&read_entry(&mut archive, METADATA_ENTRY)?)?;
        if metadata.schema_version != SCHEMA_VERSION {
            return Err(PartFileError::UnsupportedVersion(metadata.schema_version));
        }

        let history: History = serde_json::from_str(&read_entry(&mut archive, HISTORY_ENTRY)?)?;
        let state = PartState::rebuild(&history);

        Ok(Self {
            metadata,
            history,
            state,
        })
    }
}

fn write_entry<W: Write + std::io::Seek>(
    archive: &mut zip::ZipWriter<W>,
    text: &str,
) -> Result<(), zip::result::ZipError> {
    archive
        .write_all(text.as_bytes())
        .map_err(zip::result::ZipError::from)
}

fn read_entry<R: Read + std::io::Seek>(
    archive: &mut zip::ZipArchive<R>,
    name: &str,
) -> Result<String, PartFileError> {
    let mut entry = archive
        .by_name(name)
        .map_err(|_| PartFileError::MissingEntry(name.to_string()))?;
    let mut text = String::new();
    entry
        .read_to_string(&mut text)
        .map_err(zip::result::ZipError::from)?;
    Ok(text)
}

#[cfg(test)]
mod tests {
    use cao_sketch::{DimensionTarget, SegmentId, WorkPlane};

    use crate::adapters::InMemoryFiles;
    use crate::history::PointRef;
    use glam::DVec2;

    use super::*;

    fn drawn_part() -> PartDocument {
        let mut document = PartDocument::new("Test");
        document.apply(Operation::CreateSketch {
            plane: WorkPlane::XY,
        });
        document.apply(Operation::AddSegment {
            sketch: 0,
            start: PointRef::New(DVec2::ZERO),
            end: PointRef::New(DVec2::new(2.0, 0.0)),
        });
        document.apply(Operation::SetDimension {
            sketch: 0,
            target: DimensionTarget::Length(SegmentId(0)),
            value: 100.0,
            placement: None,
        });
        document
    }

    #[test]
    fn a_part_survives_a_save_and_reload() {
        let files = InMemoryFiles::default();
        let path = Path::new("/parts/piece.caopart");

        let document = drawn_part();
        document.save(&files, path).expect("saves");
        let reloaded = PartDocument::load(&files, path).expect("loads");

        assert!(files.read(path).expect("bytes").starts_with(b"PK"), "zip");
        assert_eq!(reloaded.name(), "Test");
        assert_eq!(reloaded.history, document.history);
        assert_eq!(reloaded.sketches().len(), 1);
        assert_eq!(reloaded.scale(), 50.0);
    }

    /// Undone steps are kept in the file, so redo still works after reopening.
    #[test]
    fn the_redo_tail_survives_a_save_and_reload() {
        let files = InMemoryFiles::default();
        let path = Path::new("/parts/piece.caopart");

        let mut document = drawn_part();
        document.undo();
        document.save(&files, path).expect("saves");

        let mut reloaded = PartDocument::load(&files, path).expect("loads");
        assert!(reloaded.history.can_redo());
        assert!(!reloaded.has_scale(), "the dimension is undone");
        assert!(reloaded.redo());
        assert_eq!(reloaded.scale(), 50.0);
    }

    #[test]
    fn undo_and_redo_rebuild_the_geometry() {
        let mut document = drawn_part();
        assert_eq!(document.sketches()[0].segments().len(), 1);

        assert!(document.undo());
        assert!(!document.has_scale());
        assert!(document.undo());
        assert_eq!(document.sketches()[0].segments().len(), 0);

        assert!(document.redo());
        assert_eq!(document.sketches()[0].segments().len(), 1);
    }

    #[test]
    fn rewinding_puts_the_part_back_to_a_step() {
        let mut document = drawn_part();
        document.rewind_to(1);
        assert_eq!(document.sketches().len(), 1);
        assert!(document.sketches()[0].segments().is_empty());

        document.rewind_to(3);
        assert_eq!(document.sketches()[0].segments().len(), 1);
        assert!(document.has_scale());
    }

    #[test]
    fn a_part_from_an_older_version_is_refused() {
        let files = InMemoryFiles::default();
        let path = Path::new("/parts/piece.caopart");

        PartDocument::new("Ancienne")
            .save(&files, path)
            .expect("saves");
        let mut document = PartDocument::load(&files, path).expect("loads");
        document.metadata.schema_version = 2;
        document.save(&files, path).expect("saves again");

        let error = PartDocument::load(&files, path).expect_err("must be refused");
        assert!(matches!(error, PartFileError::UnsupportedVersion(2)));
    }

    #[test]
    fn a_second_part_of_the_same_name_does_not_destroy_the_first() {
        let files = InMemoryFiles::default();
        let directory = Path::new("/parts");

        let (first, first_path) =
            PartDocument::create_in(&files, directory, "Support").expect("creates");
        let (second, second_path) =
            PartDocument::create_in(&files, directory, "Support").expect("creates a second");
        let reopened =
            PartDocument::load(&files, &first_path).expect("the first one is still there");

        assert_ne!(first_path, second_path);
        assert_eq!(second.name(), "Support 2");
        assert_eq!(reopened.metadata.id, first.metadata.id);
        assert_eq!(reopened.name(), "Support");
    }

    #[test]
    fn a_plain_json_file_is_refused() {
        let files = InMemoryFiles::default();
        let path = Path::new("/parts/piece.caopart");
        files
            .write(path, br#"{"name":"ancienne"}"#)
            .expect("writes");

        let error = PartDocument::load(&files, path).expect_err("must be refused");
        assert!(matches!(error, PartFileError::UnsupportedVersion(1)));
    }
}
