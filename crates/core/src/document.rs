use std::fs;
use std::path::{Path, PathBuf};

use cao_sketch::{LengthOutcome, SegmentId, Sketch};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::storage::StorageError;

/// File extension used for a CAO part document.
pub const PART_EXTENSION: &str = "caopart";

/// Bumped whenever the shape of a saved part changes. Older files still load:
/// fields added since carry `serde` defaults.
pub const SCHEMA_VERSION: u32 = 2;

/// What applying a typed length did to the document.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DimensionOutcome {
    /// The document had no dimension yet, so this one set its scale instead of
    /// moving anything: the drawing keeps its shape and gains a real size.
    ScaleDefined { millimeters_per_unit: f32 },
    /// The scale was already fixed, so the geometry moved to match.
    Geometry(LengthOutcome),
}

/// The on-disk representation of a part: metadata, the scale that turns world
/// units into millimetres, and the sketches drawn so far.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartDocument {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    #[serde(default = "schema_version")]
    pub schema_version: u32,
    /// Millimetres one world unit is worth. Left undefined until the first
    /// dimension is typed, which is what fixes the size of the drawing.
    #[serde(default)]
    pub millimeters_per_unit: Option<f32>,
    #[serde(default)]
    pub sketches: Vec<Sketch>,
}

fn schema_version() -> u32 {
    SCHEMA_VERSION
}

impl PartDocument {
    pub fn new(name: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            created_at: now,
            modified_at: now,
            schema_version: SCHEMA_VERSION,
            millimeters_per_unit: None,
            sketches: Vec::new(),
        }
    }

    /// Millimetres per world unit, falling back to 1 while the scale is still
    /// undefined so lengths can always be displayed.
    pub fn scale(&self) -> f32 {
        self.millimeters_per_unit.unwrap_or(1.0)
    }

    pub fn has_scale(&self) -> bool {
        self.millimeters_per_unit.is_some()
    }

    pub fn to_millimeters(&self, units: f32) -> f32 {
        units * self.scale()
    }

    /// Applies a length typed by the user, in millimetres.
    ///
    /// The very first one defines what the drawing measures: nothing moves, the
    /// document simply learns how many millimetres a world unit is worth. Every
    /// later one is a constraint, and the geometry gives way instead.
    pub fn apply_dimension(
        &mut self,
        sketch: usize,
        segment: SegmentId,
        millimeters: f32,
    ) -> Option<DimensionOutcome> {
        if millimeters <= 0.0 {
            return None;
        }
        let length_in_units = self.sketches.get(sketch)?.segment_length(segment);
        if length_in_units < 1e-6 {
            return None;
        }

        if !self.has_scale() {
            self.sketches[sketch].set_dimension(segment, millimeters);
            let millimeters_per_unit = millimeters / length_in_units;
            self.millimeters_per_unit = Some(millimeters_per_unit);
            return Some(DimensionOutcome::ScaleDefined {
                millimeters_per_unit,
            });
        }

        let target_units = millimeters / self.scale();
        let sketch = &mut self.sketches[sketch];
        sketch.set_dimension(segment, millimeters);
        Some(DimensionOutcome::Geometry(
            sketch.set_segment_length(segment, target_units),
        ))
    }

    /// Creates a new part and writes it to `dir/<name>.caopart`.
    pub fn create_in(dir: &Path, name: impl Into<String>) -> Result<(Self, PathBuf), StorageError> {
        fs::create_dir_all(dir)?;
        let doc = Self::new(name);
        let path = dir.join(format!("{}.{PART_EXTENSION}", sanitize_filename(&doc.name)));
        doc.save(&path)?;
        Ok((doc, path))
    }

    pub fn save(&self, path: &Path) -> Result<(), StorageError> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    pub fn load(path: &Path) -> Result<Self, StorageError> {
        let json = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&json)?)
    }
}

fn sanitize_filename(name: &str) -> String {
    let cleaned: String = name
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' || c == ' ' {
                c
            } else {
                '_'
            }
        })
        .collect();
    let trimmed = cleaned.trim();
    if trimmed.is_empty() {
        "Sans titre".to_string()
    } else {
        trimmed.to_string()
    }
}

#[cfg(test)]
mod tests {
    use cao_sketch::{LengthOutcome, Sketch, WorkPlane};
    use glam::Vec2;

    use super::*;

    fn part_with_chain() -> PartDocument {
        let mut document = PartDocument::new("Test");
        let mut sketch = Sketch::new(WorkPlane::XY);
        let a = sketch.add_point(Vec2::ZERO);
        let b = sketch.add_point(Vec2::new(2.0, 0.0));
        let c = sketch.add_point(Vec2::new(2.0, 1.0));
        sketch.add_segment(a, b);
        sketch.add_segment(b, c);
        document.sketches.push(sketch);
        document
    }

    /// The first dimension must not move anything: saying a 2-unit line is
    /// 100 mm simply decides that a unit is 50 mm.
    #[test]
    fn the_first_dimension_sets_the_scale_without_moving_anything() {
        let mut document = part_with_chain();
        let segment = SegmentId(0);
        let before: Vec<_> = document.sketches[0].points().to_vec();

        let outcome = document.apply_dimension(0, segment, 100.0);

        assert_eq!(
            outcome,
            Some(DimensionOutcome::ScaleDefined {
                millimeters_per_unit: 50.0
            })
        );
        assert_eq!(document.sketches[0].points(), before.as_slice());
        assert!((document.to_millimeters(2.0) - 100.0).abs() < 1e-3);
    }

    /// Once the scale is fixed, a dimension is a constraint: the geometry moves.
    #[test]
    fn later_dimensions_move_the_geometry() {
        let mut document = part_with_chain();
        document.apply_dimension(0, SegmentId(0), 100.0);

        let outcome = document.apply_dimension(0, SegmentId(1), 100.0);

        assert_eq!(
            outcome,
            Some(DimensionOutcome::Geometry(LengthOutcome::Exact))
        );
        // The second segment was 1 unit = 50 mm; asking for 100 mm doubles it.
        let length_mm = document.to_millimeters(document.sketches[0].segment_length(SegmentId(1)));
        assert!((length_mm - 100.0).abs() < 1e-2, "got {length_mm} mm");
    }

    #[test]
    fn a_dimension_needs_a_positive_length_and_a_real_segment() {
        let mut document = part_with_chain();
        assert_eq!(document.apply_dimension(0, SegmentId(0), 0.0), None);
        assert_eq!(document.apply_dimension(9, SegmentId(0), 10.0), None);
        assert!(!document.has_scale());
    }

    /// Parts saved before sketches existed must still open.
    #[test]
    fn a_part_file_from_an_older_version_still_loads() {
        let legacy = r#"{
            "id": "6f1b2c34-5d6e-4f80-9a1b-2c3d4e5f6071",
            "name": "Ancienne pièce",
            "created_at": "2026-01-01T10:00:00Z",
            "modified_at": "2026-01-01T10:00:00Z"
        }"#;

        let document: PartDocument = serde_json::from_str(legacy).expect("legacy part loads");
        assert_eq!(document.name, "Ancienne pièce");
        assert_eq!(document.schema_version, SCHEMA_VERSION);
        assert!(document.sketches.is_empty());
        assert!(!document.has_scale());
    }

    #[test]
    fn a_part_survives_a_save_and_reload() {
        let mut document = part_with_chain();
        document.apply_dimension(0, SegmentId(0), 100.0);

        let json = serde_json::to_string(&document).expect("serializes");
        let reloaded: PartDocument = serde_json::from_str(&json).expect("deserializes");

        assert_eq!(reloaded.sketches.len(), 1);
        assert_eq!(reloaded.sketches[0].segments().len(), 2);
        assert_eq!(reloaded.millimeters_per_unit, Some(50.0));
        assert_eq!(reloaded.sketches[0].dimensions().len(), 1);
    }
}
