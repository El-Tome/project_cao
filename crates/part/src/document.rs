use std::io::{Cursor, Read, Write};
use std::path::{Path, PathBuf};

use cao_sketch::Sketch;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::PartFileError;
use crate::file_name;
use crate::history::{History, Operation};
use crate::outcome::Outcome;
use crate::picture::Picture;
use crate::ports::Files;
use crate::state::PartState;

mod design;
mod geometry_cache;

/// Bumped whenever the layout of a saved part changes.
///
/// Older versions are refused rather than converted: while the tool is still
/// taking shape, a conversion would be more likely to rebuild a part wrongly
/// than to save anything worth keeping.
pub const SCHEMA_VERSION: u32 = 5;

const METADATA_ENTRY: &str = "part.json";
/// How big the picture is. Kept apart from its bytes so that neither entry has
/// to carry a header the other could contradict.
const PICTURE_SHAPE_ENTRY: &str = "picture.json";
const PICTURE_PIXELS_ENTRY: &str = "picture.rgba";

/// What a write of the archive is for.
#[derive(Clone, Copy, PartialEq)]
enum Writing {
    /// The end of a gesture: the design alone.
    AtRest,
    /// The end of the session with the part: the geometry it is showing goes
    /// with it.
    PuttingAway,
}

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
    picture: Option<Picture>,
}

/// How big a picture is, beside the bytes that fill it.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
struct PictureShape {
    width: u32,
    height: u32,
}

impl PartDocument {
    pub fn new(name: impl Into<String>, now: DateTime<Utc>) -> Self {
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
            picture: None,
        }
    }

    /// What the part looked like when it was last put away, if anybody has
    /// taken its picture. Read by a panel listing a folder, which must not
    /// replay a history to show a row.
    pub fn picture(&self) -> Option<&Picture> {
        self.picture.as_ref()
    }

    pub fn set_picture(&mut self, picture: Picture) {
        self.picture = Some(picture);
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
    pub fn apply(&mut self, operation: Operation) -> Option<Outcome> {
        let outcome = self.state.apply(&operation);
        self.history.push(operation);
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

    /// Rewrites the history down to the steps that still describe the part,
    /// dropping the redo tail with it.
    pub fn compact_history(&mut self) {
        self.history = crate::compaction::compact(&self.history);
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
        now: DateTime<Utc>,
    ) -> Result<(Self, PathBuf), PartFileError> {
        let mut doc = Self::new(name, now);
        let (free, path) = file_name::free_in(files, dir, doc.name())?;
        doc.metadata.name = free;
        doc.save(files, &path, now)?;
        Ok((doc, path))
    }

    /// Writes the part at the end of a gesture, its design alone. `now` goes
    /// into the file and nowhere else: what a part carries in memory is still
    /// the hour it was built with.
    pub fn save(
        &self,
        files: &impl Files,
        path: &Path,
        now: DateTime<Utc>,
    ) -> Result<(), PartFileError> {
        self.write(files, path, now, Writing::AtRest)
    }

    /// Writes the part on the way out of it, with the geometry it is showing.
    ///
    /// Nothing more is coming to make that geometry stale, which is what a
    /// gesture could never say: the cache exists to save the replay at the
    /// next open, and writing it again at every stroke would cost more than
    /// the replay it saves.
    pub fn put_away(
        &self,
        files: &impl Files,
        path: &Path,
        now: DateTime<Utc>,
    ) -> Result<(), PartFileError> {
        self.write(files, path, now, Writing::PuttingAway)
    }

    fn write(
        &self,
        files: &impl Files,
        path: &Path,
        now: DateTime<Utc>,
        writing: Writing,
    ) -> Result<(), PartFileError> {
        let metadata = PartMetadata {
            modified_at: now,
            ..self.metadata.clone()
        };
        let mut archive = zip::ZipWriter::new(Cursor::new(Vec::new()));
        let options: zip::write::SimpleFileOptions = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);

        archive.start_file(METADATA_ENTRY, options)?;
        archive
            .write_all(serde_json::to_string_pretty(&metadata)?.as_bytes())
            .map_err(zip::result::ZipError::from)?;
        let design = design::laid_out(&self.history)?;
        design::write(&mut archive, options, &design)?;
        if writing == Writing::PuttingAway {
            let print: Vec<&str> = design.iter().map(design::Written::text).collect();
            geometry_cache::write(&mut archive, options, &self.state, &print)?;
        }

        if let Some(picture) = &self.picture {
            let shape = PictureShape {
                width: picture.width(),
                height: picture.height(),
            };
            archive.start_file(PICTURE_SHAPE_ENTRY, options)?;
            archive
                .write_all(serde_json::to_string(&shape)?.as_bytes())
                .map_err(zip::result::ZipError::from)?;
            archive.start_file(PICTURE_PIXELS_ENTRY, options)?;
            archive
                .write_all(picture.pixels())
                .map_err(zip::result::ZipError::from)?;
        }

        let bytes = archive.finish()?.into_inner();
        files.write(path, &bytes)?;
        Ok(())
    }

    /// The picture a part carries, pulled out of the archive on its own.
    ///
    /// Opening the part would replay its whole history — the cost the picture
    /// exists to avoid. A panel listing a folder must not pay it once a row.
    pub fn picture_in(files: &impl Files, path: &Path) -> Option<Picture> {
        let bytes = files.read(path).ok()?;
        let mut archive = zip::ZipArchive::new(Cursor::new(bytes)).ok()?;
        read_picture(&mut archive)
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

        let (history, design) = design::read(&mut archive)?;
        let print: Vec<&str> = design.iter().map(String::as_str).collect();
        let state = geometry_cache::read(&mut archive, &print)
            .unwrap_or_else(|| PartState::rebuild(&history));

        Ok(Self {
            metadata,
            history,
            state,
            picture: read_picture(&mut archive),
        })
    }
}

/// The picture, when the archive holds a whole one.
///
/// A part written before pictures existed carries neither entry, and a part
/// whose picture is damaged is a part with no picture — never a part that
/// refuses to open. Nothing of the drawing is in there.
fn read_picture<R: Read + std::io::Seek>(archive: &mut zip::ZipArchive<R>) -> Option<Picture> {
    let shape: PictureShape =
        serde_json::from_str(&read_entry(archive, PICTURE_SHAPE_ENTRY).ok()?).ok()?;
    let mut pixels = Vec::new();
    archive
        .by_name(PICTURE_PIXELS_ENTRY)
        .ok()?
        .read_to_end(&mut pixels)
        .ok()?;
    Picture::new(shape.width, shape.height, pixels)
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
mod tests;
